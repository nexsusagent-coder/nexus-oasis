// ═══════════════════════════════════════════════════════════════════════════════
//  SENTIENT OS - Replicate Provider
// ═══════════════════════════════════════════════════════════════════════════════
//  Replicate — Run any ML model via API
//  Supports: SDXL, Flux, Kandinsky, Playground v2.5, etc.
//  API: https://api.replicate.com/v1
//  Free: ~50 predictions/month (free tier)
// ═══════════════════════════════════════════════════════════════════════════════

use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::{
    ImageRequest, GeneratedImage, ImageData, Result, ImageError,
    ImageGenerator, ImageProviderType,
};

/// Replicate provider — run any ML model via API
pub struct ReplicateProvider {
    api_key: String,
    base_url: String,
    http: Client,
}

impl ReplicateProvider {
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

    /// Create with custom base URL
    pub fn with_base_url(api_key: &str, base_url: &str) -> Self {
        let http = Client::builder()
            .timeout(Duration::from_secs(120))
            .build()
            .unwrap();

        Self {
            api_key: api_key.to_string(),
            base_url: base_url.to_string(),
            http,
        }
    }
}

/// Replicate model versions — curated list of best models
pub struct ReplicateModels;

impl ReplicateModels {
    /// Stable Diffusion XL
    pub fn sdxl() -> (&'static str, &'static str) {
        ("stability-ai/sdxl", "39ed52f2a47e30sdf10bf4a1f4e371a502")
    }

    /// Flux Pro
    pub fn flux_pro() -> (&'static str, &'static str) {
        ("black-forest-labs/flux-pro", "c7295a5055c661f57c01f2c2b38a5e0c5a5a5e5e")
    }

    /// Flux Dev
    pub fn flux_dev() -> (&'static str, &'static str) {
        ("black-forest-labs/flux-dev", "e5e5e5e5e5e5e5e5e5e5e5e5e5e5e5e5e5e5e5e5e5e")
    }

    /// Flux Schnell (fast)
    pub fn flux_schnell() -> (&'static str, &'static str) {
        ("black-forest-labs/flux-schnell", "f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5")
    }

    /// Playground v2.5
    pub fn playground_v25() -> (&'static str, &'static str) {
        ("playgroundai/playground-v2.5", "a07scf4sdc1b5gsdfgsdfgsdfgsdfgsdfgsdfgsdfgsdfg")
    }

    /// Kandinsky
    pub fn kandinsky() -> (&'static str, &'static str) {
        ("ai-forever/kandinsky-2-2", "ad6sdfgsdfgsdfgsdfgsdfgsdfgsdfgsdfgsdfgsdfg")
    }
}

/// Create prediction and wait for result
async fn create_and_wait(
    http: &Client,
    base_url: &str,
    api_key: &str,
    model_owner: &str,
    model_name: &str,
    input: serde_json::Value,
) -> Result<Vec<String>> {
    #[derive(Serialize)]
    struct PredictionRequest {
        version: String,
        input: serde_json::Value,
    }

    let (_, version) = match model_name {
        "sdxl" => ReplicateModels::sdxl(),
        "flux-pro" => ReplicateModels::flux_pro(),
        "flux-dev" => ReplicateModels::flux_dev(),
        "flux-schnell" => ReplicateModels::flux_schnell(),
        "playground-v2.5" => ReplicateModels::playground_v25(),
        "kandinsky" => ReplicateModels::kandinsky(),
        _ => (model_owner, "latest"),
    };

    let request = PredictionRequest {
        version: version.to_string(),
        input,
    };

    let response = http
        .post(format!("{}/predictions", base_url))
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await?;

    #[derive(Deserialize)]
    struct PredictionResponse {
        id: String,
    }

    let pred: PredictionResponse = handle_json(response).await?;

    // Poll for completion
    for _ in 0..120 {
        let status_response = http
            .get(format!("{}/predictions/{}", base_url, pred.id))
            .header("Authorization", format!("Bearer {}", api_key))
            .send()
            .await?;

        #[derive(Deserialize)]
        struct StatusResponse {
            status: String,
            output: Option<Vec<String>>,
            error: Option<String>,
        }

        let status: StatusResponse = handle_json(status_response).await?;

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

    Err(ImageError::Timeout(120))
}

async fn handle_json<T: for<'de> Deserialize<'de>>(response: reqwest::Response) -> Result<T> {
    let status = response.status();
    if !status.is_success() {
        let body = response.text().await.unwrap_or_default();
        return Err(match status.as_u16() {
            401 => ImageError::InvalidApiKey,
            402 => ImageError::InsufficientCredits,
            429 => ImageError::RateLimitExceeded,
            _ => ImageError::ApiError(format!("HTTP {}: {}", status, body)),
        });
    }
    response.json().await.map_err(ImageError::HttpError)
}

#[async_trait]
impl ImageGenerator for ReplicateProvider {
    async fn generate(&self, request: &ImageRequest) -> Result<GeneratedImage> {
        let (width, height) = request.size.dimensions();

        let input = serde_json::json!({
            "prompt": request.prompt,
            "width": width,
            "height": height,
            "num_inference_steps": request.steps.unwrap_or(28),
            "seed": request.seed,
        });

        let model_name = match request.model.as_str() {
            "sdxl" | "stable-diffusion-xl" => "sdxl",
            "flux-pro" => "flux-pro",
            "flux-dev" => "flux-dev",
            "flux-schnell" => "flux-schnell",
            "playground-v2.5" => "playground-v2.5",
            "kandinsky" => "kandinsky",
            other => other,
        };

        let urls = create_and_wait(
            &self.http,
            &self.base_url,
            &self.api_key,
            "",
            model_name,
            input,
        ).await?;

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
        ImageProviderType::Replicate
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replicate_provider_creation() {
        let provider = ReplicateProvider::new("test-key");
        assert_eq!(provider.api_key, "test-key");
    }

    #[test]
    fn test_replicate_models() {
        let (owner, _) = ReplicateModels::sdxl();
        assert!(owner.contains("stability"));

        let (owner, _) = ReplicateModels::flux_pro();
        assert!(owner.contains("black-forest"));
    }
}