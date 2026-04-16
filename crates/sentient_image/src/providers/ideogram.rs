// ═══════════════════════════════════════════════════════════════════════════════
//  SENTIENT OS - Ideogram Provider
// ═══════════════════════════════════════════════════════════════════════════════
//  Ideogram AI — Best for text-in-image generation
//  API: https://api.ideogram.ai/api/generate
//  Models: ideogram-v2, ideogram-v2-turbo, ideogram-v1
//  Free: ~80 images/month
// ═══════════════════════════════════════════════════════════════════════════════

use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::{
    ImageRequest, GeneratedImage, ImageData, Result, ImageError,
    ImageGenerator, ImageProviderType, ImageSize,
};

/// Ideogram provider — best for text-in-image generation
pub struct IdeogramProvider {
    api_key: String,
    base_url: String,
    http: Client,
}

impl IdeogramProvider {
    pub fn new(api_key: &str) -> Self {
        let http = Client::builder()
            .timeout(Duration::from_secs(60))
            .build()
            .unwrap();

        Self {
            api_key: api_key.to_string(),
            base_url: "https://api.ideogram.ai/api".to_string(),
            http,
        }
    }

    /// Create with custom base URL (for proxies)
    pub fn with_base_url(api_key: &str, base_url: &str) -> Self {
        let http = Client::builder()
            .timeout(Duration::from_secs(60))
            .build()
            .unwrap();

        Self {
            api_key: api_key.to_string(),
            base_url: base_url.to_string(),
            http,
        }
    }
}

/// Ideogram API request
#[derive(Debug, Clone, Serialize)]
struct IdeogramRequest {
    /// Text prompt
    prompt: String,
    /// Model name
    model: String,
    /// Aspect ratio (e.g. "ASPECT_1_1", "ASPECT_16_9")
    aspect_ratio: Option<String>,
    /// Negative prompt
    negative_prompt: Option<String>,
    /// Seed for reproducibility
    seed: Option<u64>,
    /// Style type
    style_type: Option<String>,
    /// Number of images (1-4)
    #[serde(skip_serializing_if = "Option::is_none")]
    num_images: Option<u8>,
}

/// Ideogram API response
#[derive(Debug, Clone, Deserialize)]
struct IdeogramResponse {
    created: Option<i64>,
    data: Vec<IdeogramImage>,
}

/// Single image in response
#[derive(Debug, Clone, Deserialize)]
struct IdeogramImage {
    url: Option<String>,
    resolution: Option<String>,
    seed: Option<u64>,
    prompt: Option<String>,
}

/// Convert ImageSize to Ideogram aspect ratio
fn size_to_aspect_ratio(size: &ImageSize) -> String {
    match size {
        ImageSize::Square1024 | ImageSize::Small256 | ImageSize::Medium512 => "ASPECT_1_1".to_string(),
        ImageSize::Landscape1792 => "ASPECT_16_9".to_string(),
        ImageSize::Portrait1024 => "ASPECT_9_16".to_string(),
        ImageSize::HD1280 => "ASPECT_16_9".to_string(),
    }
}

/// Convert model name
fn resolve_model(model: &str) -> String {
    match model {
        "ideogram-v2" | "ideogram_v2" => "ideogram-v2".to_string(),
        "ideogram-v2-turbo" | "ideogram_v2_turbo" => "ideogram-v2-turbo".to_string(),
        "ideogram-v1" | "ideogram_v1" => "ideogram-v1".to_string(),
        _ => model.to_string(),
    }
}

#[async_trait]
impl ImageGenerator for IdeogramProvider {
    async fn generate(&self, request: &ImageRequest) -> Result<GeneratedImage> {
        let model = resolve_model(&request.model);

        let api_request = IdeogramRequest {
            prompt: request.prompt.clone(),
            model,
            aspect_ratio: Some(size_to_aspect_ratio(&request.size)),
            negative_prompt: request.negative_prompt.clone(),
            seed: request.seed,
            style_type: None,
            num_images: Some(1),
        };

        let response = self.http
            .post(format!("{}/generate", self.base_url))
            .header("Api-Key", &self.api_key)
            .header("Content-Type", "application/json")
            .json(&api_request)
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(match status.as_u16() {
                401 => ImageError::InvalidApiKey,
                402 => ImageError::InsufficientCredits,
                400 if body.contains("content") => ImageError::ContentPolicyViolation(body),
                429 => ImageError::RateLimitExceeded,
                _ => ImageError::ApiError(format!("HTTP {}: {}", status, body)),
            });
        }

        let api_response: IdeogramResponse = response.json().await
            .map_err(|e| ImageError::ApiError(e.to_string()))?;

        let first = api_response.data.first()
            .ok_or_else(|| ImageError::GenerationFailed("No image returned".to_string()))?;

        let url = first.url.clone()
            .ok_or_else(|| ImageError::GenerationFailed("No image URL".to_string()))?;

        Ok(GeneratedImage {
            data: ImageData::Url(url),
            revised_prompt: first.prompt.clone(),
            model: request.model.clone(),
            size: request.size,
            seed: first.seed,
        })
    }

    fn provider_type(&self) -> ImageProviderType {
        ImageProviderType::Ideogram
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ideogram_provider_creation() {
        let provider = IdeogramProvider::new("test-key");
        assert_eq!(provider.api_key, "test-key");
    }

    #[test]
    fn test_resolve_model() {
        assert_eq!(resolve_model("ideogram-v2"), "ideogram-v2");
        assert_eq!(resolve_model("ideogram_v2_turbo"), "ideogram-v2-turbo");
        assert_eq!(resolve_model("ideogram-v1"), "ideogram-v1");
    }

    #[test]
    fn test_aspect_ratio_conversion() {
        assert_eq!(size_to_aspect_ratio(&ImageSize::Square1024), "ASPECT_1_1");
        assert_eq!(size_to_aspect_ratio(&ImageSize::Landscape1792), "ASPECT_16_9");
        assert_eq!(size_to_aspect_ratio(&ImageSize::Portrait1024), "ASPECT_9_16");
    }
}