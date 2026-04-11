// ═══════════════════════════════════════════════════════════════════════════════
//  SENTIENT OS - OpenAI DALL-E Provider
// ═══════════════════════════════════════════════════════════════════════════════

use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::{
    ImageRequest, GeneratedImage, ImageData, Result, ImageError,
    ImageGenerator, ImageProviderType, ImageResponse, ImageSize,
};

/// OpenAI DALL-E provider
pub struct OpenAIProvider {
    api_key: String,
    base_url: String,
    http: Client,
}

impl OpenAIProvider {
    pub fn new(api_key: &str) -> Self {
        let http = Client::builder()
            .timeout(Duration::from_secs(60))
            .build()
            .unwrap();

        Self {
            api_key: api_key.to_string(),
            base_url: "https://api.openai.com/v1".to_string(),
            http,
        }
    }

    pub fn with_base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into();
        self
    }

    /// Edit image (DALL-E 2)
    pub async fn edit(
        &self,
        image: &[u8],
        prompt: &str,
        mask: Option<&[u8]>,
    ) -> Result<GeneratedImage> {
        let mut form = reqwest::multipart::Form::new()
            .text("prompt", prompt.to_string())
            .part("image", reqwest::multipart::Part::bytes(image.to_vec()).file_name("image.png"));

        if let Some(mask_data) = mask {
            form = form.part("mask", reqwest::multipart::Part::bytes(mask_data.to_vec()).file_name("mask.png"));
        }

        let response = self.http
            .post(format!("{}/images/edits", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .multipart(form)
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// Create variations (DALL-E 2)
    pub async fn create_variation(&self, image: &[u8]) -> Result<GeneratedImage> {
        let form = reqwest::multipart::Form::new()
            .part("image", reqwest::multipart::Part::bytes(image.to_vec()).file_name("image.png"));

        let response = self.http
            .post(format!("{}/images/variations", self.base_url))
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
                error: Option<ErrorDetail>,
            }
            #[derive(Deserialize)]
            struct ErrorDetail {
                message: Option<String>,
                code: Option<String>,
            }

            let error_msg = if let Ok(err) = serde_json::from_str::<ErrorResponse>(&body) {
                err.error.map(|e| e.message.unwrap_or_default())
                    .unwrap_or(body.clone())
            } else {
                body
            };

            return Err(match status.as_u16() {
                401 => ImageError::InvalidApiKey,
                400 if error_msg.contains("content_policy") => 
                    ImageError::ContentPolicyViolation(error_msg),
                429 => ImageError::RateLimitExceeded,
                _ => ImageError::ApiError(format!("HTTP {}: {}", status, error_msg)),
            });
        }

        let api_response: ImageResponse = response.json().await?;

        let first_image = api_response.data.first()
            .ok_or_else(|| ImageError::GenerationFailed("No image returned".to_string()))?;

        let data = if let Some(url) = &first_image.url {
            ImageData::Url(url.clone())
        } else if let Some(b64) = &first_image.b64_json {
            ImageData::Base64(b64.clone())
        } else {
            return Err(ImageError::GenerationFailed("No image data".to_string()));
        };

        Ok(GeneratedImage {
            data,
            revised_prompt: first_image.revised_prompt.clone(),
            model: "dall-e-3".to_string(),
            size: ImageSize::Square1024,
            seed: None,
        })
    }
}

#[async_trait]
impl ImageGenerator for OpenAIProvider {
    async fn generate(&self, request: &ImageRequest) -> Result<GeneratedImage> {
        let size_str = request.size.to_string_api();

        #[derive(Serialize)]
        struct ApiRequest {
            prompt: String,
            model: String,
            n: u8,
            size: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            quality: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            style: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            response_format: Option<String>,
        }

        let api_request = ApiRequest {
            prompt: request.prompt.clone(),
            model: request.model.clone(),
            n: request.n.unwrap_or(1),
            size: size_str,
            quality: request.quality.map(|q| match q {
                crate::ImageQuality::Standard => "standard",
                crate::ImageQuality::HD => "hd",
            }.to_string()),
            style: request.style.map(|s| match s {
                crate::ImageStyle::Vivid => "vivid",
                crate::ImageStyle::Natural => "natural",
            }.to_string()),
            response_format: request.response_format.clone(),
        };

        let response = self.http
            .post(format!("{}/images/generations", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&api_request)
            .send()
            .await?;

        self.handle_response(response).await
    }

    fn provider_type(&self) -> ImageProviderType {
        ImageProviderType::OpenAI
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_creation() {
        let provider = OpenAIProvider::new("test-key");
        assert_eq!(provider.api_key, "test-key");
    }
}
