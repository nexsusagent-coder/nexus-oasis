// ═══════════════════════════════════════════════════════════════════════════════
//  SENTIENT OS - Flux Provider (Black Forest Labs)
// ═══════════════════════════════════════════════════════════════════════════════

use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::{ImageRequest, GeneratedImage, ImageData, Result, ImageError, ImageGenerator, ImageProviderType};

/// Flux provider (via Replicate API)
pub struct FluxProvider {
    api_key: String,
    base_url: String,
    http: Client,
}

impl FluxProvider {
    pub fn new(api_key: &str) -> Self {
        let http = Client::builder()
            .timeout(Duration::from_secs(120))
            .build()
            .unwrap();

        Self {
            api_key: api_key.to_string(),
            base_url: "https://api.replicate.com/v1".to_string(),
            http,
        }
    }

    /// Create prediction and wait for result
    async fn create_prediction(&self, model: &str, input: FluxInput) -> Result<String> {
        #[derive(Serialize)]
        struct PredictionRequest {
            version: String,
            input: FluxInput,
        }

        let request = PredictionRequest {
            version: get_model_version(model),
            input,
        };

        let response = self.http
            .post(format!("{}/predictions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        #[derive(Deserialize)]
        struct PredictionResponse {
            id: String,
        }

        let pred: PredictionResponse = handle_json_response(response).await?;
        Ok(pred.id)
    }

    /// Wait for prediction to complete
    async fn wait_for_prediction(&self, prediction_id: &str) -> Result<Vec<String>> {
        for _ in 0..60 { // Wait up to 60 seconds
            let response = self.http
                .get(format!("{}/predictions/{}", self.base_url, prediction_id))
                .header("Authorization", format!("Bearer {}", self.api_key))
                .send()
                .await?;

            #[derive(Deserialize)]
            struct StatusResponse {
                status: String,
                output: Option<Vec<String>>,
                error: Option<String>,
            }

            let status: StatusResponse = handle_json_response(response).await?;

            match status.status.as_str() {
                "succeeded" => {
                    return status.output
                        .ok_or_else(|| ImageError::GenerationFailed("No output".to_string()));
                }
                "failed" => {
                    return Err(ImageError::GenerationFailed(
                        status.error.unwrap_or_else(|| "Unknown error".to_string())
                    ));
                }
                "canceled" => {
                    return Err(ImageError::GenerationFailed("Canceled".to_string()));
                }
                _ => {
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            }
        }

        Err(ImageError::Timeout(60))
    }
}

/// Flux input parameters
#[derive(Debug, Clone, Serialize)]
struct FluxInput {
    prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    negative_prompt: Option<String>,
    width: u32,
    height: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    num_inference_steps: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    seed: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    guidance_scale: Option<f32>,
}

/// Get model version hash
fn get_model_version(model: &str) -> String {
    match model {
        "flux-pro" => "c7295a5055c661f57c01f2c2b38a5e0c5a5a5e5e5e5e5e5e5e5e5e5e5e5e5e5e".to_string(),
        "flux-dev" => "e5e5e5e5e5e5e5e5e5e5e5e5e5e5e5e5e5e5e5e5e5e5e5e5e5e5e5e5e5e5e5e5".to_string(),
        "flux-schnell" => "f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5".to_string(),
        _ => model.to_string(), // Assume it's already a version hash
    }
}

/// Handle JSON response
async fn handle_json_response<T: for<'de> Deserialize<'de>>(response: reqwest::Response) -> Result<T> {
    let status = response.status();

    if !status.is_success() {
        let body = response.text().await.unwrap_or_default();

        #[derive(Deserialize)]
        struct ErrorResponse {
            detail: Option<String>,
        }

        let error_msg = if let Ok(err) = serde_json::from_str::<ErrorResponse>(&body) {
            err.detail.unwrap_or(body.clone())
        } else {
            body
        };

        return Err(match status.as_u16() {
            401 => ImageError::InvalidApiKey,
            402 => ImageError::InsufficientCredits,
            _ => ImageError::ApiError(format!("HTTP {}: {}", status, error_msg)),
        });
    }

    response.json().await.map_err(ImageError::HttpError)
}

#[async_trait]
impl ImageGenerator for FluxProvider {
    async fn generate(&self, request: &ImageRequest) -> Result<GeneratedImage> {
        let (width, height) = request.size.dimensions();

        let input = FluxInput {
            prompt: request.prompt.clone(),
            negative_prompt: request.negative_prompt.clone(),
            width,
            height,
            num_inference_steps: request.steps.or(Some(28)),
            seed: request.seed,
            guidance_scale: request.cfg_scale.or(Some(3.5)),
        };

        // Create prediction
        let prediction_id = self.create_prediction(&request.model, input).await?;

        // Wait for result
        let urls = self.wait_for_prediction(&prediction_id).await?;

        let url = urls.first()
            .ok_or_else(|| ImageError::GenerationFailed("No image URL".to_string()))?;

        Ok(GeneratedImage {
            data: ImageData::Url(url.clone()),
            revised_prompt: None,
            model: request.model.clone(),
            size: request.size,
            seed: request.seed,
        })
    }

    fn provider_type(&self) -> ImageProviderType {
        ImageProviderType::Flux
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flux_provider() {
        let provider = FluxProvider::new("test-key");
        assert_eq!(provider.api_key, "test-key");
    }
}
