// ═══════════════════════════════════════════════════════════════════════════════
//  SENTIENT OS - Stability AI Provider
// ═══════════════════════════════════════════════════════════════════════════════

use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::{
    ImageRequest, GeneratedImage, ImageData, Result, ImageError,
    ImageGenerator, ImageProviderType, ImageSize,
};

/// Stability AI provider (Stable Diffusion)
pub struct StabilityProvider {
    api_key: String,
    base_url: String,
    http: Client,
}

impl StabilityProvider {
    pub fn new(api_key: &str) -> Self {
        let http = Client::builder()
            .timeout(Duration::from_secs(120)) // SD takes longer
            .build()
            .unwrap();

        Self {
            api_key: api_key.to_string(),
            base_url: "https://api.stability.ai/v1".to_string(),
            http,
        }
    }

    /// Upscale image
    pub async fn upscale(&self, image: &[u8], scale: f32) -> Result<Vec<u8>> {
        let response = self.http
            .post(format!("{}/generation/{}/upscale", self.base_url, "stable-diffusion-xl-1024-v1-0"))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "image/png")
            .header("Accept", "image/png")
            .query(&[("scale", scale.to_string())])
            .body(image.to_vec())
            .send()
            .await?;

        let bytes = response.bytes().await?;
        Ok(bytes.to_vec())
    }

    /// Image-to-image
    pub async fn img2img(
        &self,
        init_image: &[u8],
        prompt: &str,
        strength: f32,
    ) -> Result<GeneratedImage> {
        #[derive(Serialize)]
        struct Img2ImgRequest {
            text_prompts: Vec<TextPrompt>,
            cfg_scale: f32,
            image_strength: f32,
            steps: u32,
        }

        #[derive(Serialize)]
        struct TextPrompt {
            text: String,
            weight: f32,
        }

        let request = Img2ImgRequest {
            text_prompts: vec![TextPrompt {
                text: prompt.to_string(),
                weight: 1.0,
            }],
            cfg_scale: 7.0,
            image_strength: strength,
            steps: 30,
        };

        let form = reqwest::multipart::Form::new()
            .part("init_image", reqwest::multipart::Part::bytes(init_image.to_vec()).file_name("init.png"))
            .part("json", reqwest::multipart::Part::text(serde_json::to_string(&request)?));

        let response = self.http
            .post(format!("{}/generation/{}/image-to-image", self.base_url, "stable-diffusion-xl-1024-v1-0"))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .multipart(form)
            .send()
            .await?;

        self.handle_response(response).await
    }

    async fn handle_response(&self, response: reqwest::Response) -> Result<GeneratedImage> {
        let status = response.status();

        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();

            #[derive(Deserialize)]
            struct ErrorResponse {
                message: Option<String>,
            }

            let error_msg = if let Ok(err) = serde_json::from_str::<ErrorResponse>(&body) {
                err.message.unwrap_or(body.clone())
            } else {
                body
            };

            return Err(match status.as_u16() {
                401 => ImageError::InvalidApiKey,
                402 => ImageError::InsufficientCredits,
                400 if error_msg.contains("content") => 
                    ImageError::ContentPolicyViolation(error_msg),
                _ => ImageError::ApiError(format!("HTTP {}: {}", status, error_msg)),
            });
        }

        #[derive(Deserialize)]
        struct StabilityResponse {
            artifacts: Vec<Artifact>,
        }

        #[derive(Deserialize)]
        struct Artifact {
            base64: String,
            seed: u64,
        }

        let api_response: StabilityResponse = response.json().await?;

        let first = api_response.artifacts.first()
            .ok_or_else(|| ImageError::GenerationFailed("No image returned".to_string()))?;

        Ok(GeneratedImage {
            data: ImageData::Base64(first.base64.clone()),
            revised_prompt: None,
            model: "stable-diffusion-xl".to_string(),
            size: ImageSize::Square1024,
            seed: Some(first.seed),
        })
    }
}

#[async_trait]
impl ImageGenerator for StabilityProvider {
    async fn generate(&self, request: &ImageRequest) -> Result<GeneratedImage> {
        #[derive(Serialize)]
        struct TextPrompt {
            text: String,
            weight: f32,
        }

        #[derive(Serialize)]
        struct StabilityRequest {
            text_prompts: Vec<TextPrompt>,
            cfg_scale: f32,
            height: u32,
            width: u32,
            steps: u32,
            #[serde(skip_serializing_if = "Option::is_none")]
            seed: Option<u64>,
        }

        let (width, height) = request.size.dimensions();

        let mut text_prompts = vec![TextPrompt {
            text: request.prompt.clone(),
            weight: 1.0,
        }];

        if let Some(neg) = &request.negative_prompt {
            text_prompts.push(TextPrompt {
                text: neg.clone(),
                weight: -1.0,
            });
        }

        let api_request = StabilityRequest {
            text_prompts,
            cfg_scale: request.cfg_scale.unwrap_or(7.0),
            height,
            width,
            steps: request.steps.unwrap_or(30),
            seed: request.seed,
        };

        let response = self.http
            .post(format!("{}/generation/{}/text-to-image", self.base_url, request.model))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .json(&api_request)
            .send()
            .await?;

        self.handle_response(response).await
    }

    fn provider_type(&self) -> ImageProviderType {
        ImageProviderType::StabilityAI
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stability_provider() {
        let provider = StabilityProvider::new("test-key");
        assert_eq!(provider.api_key, "test-key");
    }
}
