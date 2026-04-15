// ═══════════════════════════════════════════════════════════════════════════════
//  SENTIENT OS - Image Generation Integration
// ═══════════════════════════════════════════════════════════════════════════════
//  Multi-provider image generation
//  - DALL-E 3 (OpenAI)
//  - Stable Diffusion (Stability AI)
//  - Flux (Black Forest Labs)
//  - Ideogram
// ═══════════════════════════════════════════════════════════════════════════════

pub mod providers;
pub mod types;
pub mod error;
pub mod edit;

pub use providers::{ImageProvider, ImageGenerator};
pub use types::{ImageRequest, ImageResponse, ImageSize, ImageStyle, ImageQuality};
pub use error::{ImageError, Result};
pub use edit::{ImageEditor, EditRequest, EditResponse, EditOperation, ImageSource, RgbaColor};

use serde::{Deserialize, Serialize};

/// Image generation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageConfig {
    /// Default provider
    pub default_provider: ImageProviderType,
    /// Default model
    pub default_model: String,
    /// Default size
    pub default_size: ImageSize,
    /// Request timeout in seconds
    pub timeout_secs: u64,
}

impl ImageConfig {
    pub fn new() -> Self {
        Self {
            default_provider: ImageProviderType::OpenAI,
            default_model: "dall-e-3".to_string(),
            default_size: ImageSize::Square1024,
            timeout_secs: 60,
        }
    }

    pub fn with_provider(mut self, provider: ImageProviderType) -> Self {
        self.default_provider = provider;
        self
    }

    pub fn with_model(mut self, model: impl Into<String>) -> Self {
        self.default_model = model.into();
        self
    }
}

impl Default for ImageConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Image provider type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ImageProviderType {
    #[serde(rename = "openai")]
    OpenAI,
    #[serde(rename = "stability")]
    StabilityAI,
    #[serde(rename = "flux")]
    Flux,
    #[serde(rename = "ideogram")]
    Ideogram,
    #[serde(rename = "replicate")]
    Replicate,
}

/// Generated image data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedImage {
    /// Image URL or base64 data
    pub data: ImageData,
    /// Revised prompt (DALL-E 3 returns improved prompts)
    pub revised_prompt: Option<String>,
    /// Model used
    pub model: String,
    /// Size
    pub size: ImageSize,
    /// Seed used for generation
    pub seed: Option<u64>,
}

impl GeneratedImage {
    /// Check if image is URL
    pub fn is_url(&self) -> bool {
        matches!(&self.data, ImageData::Url(_))
    }

    /// Check if image is base64
    pub fn is_base64(&self) -> bool {
        matches!(&self.data, ImageData::Base64(_))
    }

    /// Get URL if available
    pub fn url(&self) -> Option<&str> {
        match &self.data {
            ImageData::Url(url) => Some(url),
            _ => None,
        }
    }

    /// Get base64 data if available
    pub fn base64(&self) -> Option<&str> {
        match &self.data {
            ImageData::Base64(data) => Some(data),
            _ => None,
        }
    }

    /// Save to file
    pub async fn save_to_file(&self, path: &str) -> Result<()> {
        let bytes = self.to_bytes().await?;
        tokio::fs::write(path, bytes).await
            .map_err(|e| ImageError::IoError(e.to_string()))?;
        Ok(())
    }

    /// Convert to bytes
    pub async fn to_bytes(&self) -> Result<Vec<u8>> {
        match &self.data {
            ImageData::Url(url) => {
                let response = reqwest::get(url).await?;
                let bytes = response.bytes().await?;
                Ok(bytes.to_vec())
            }
            ImageData::Base64(data) => {
                use base64::Engine;
                let bytes = base64::engine::general_purpose::STANDARD
                    .decode(data)
                    .map_err(|e| ImageError::InvalidBase64(e.to_string()))?;
                Ok(bytes)
            }
        }
    }
}

/// Image data (URL or base64)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ImageData {
    Url(String),
    Base64(String),
}

// Re-export for convenience
pub mod prelude {
    pub use crate::{ImageConfig, ImageProviderType, GeneratedImage};
    pub use crate::types::{ImageRequest, ImageResponse, ImageSize};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_creation() {
        let config = ImageConfig::new();
        assert_eq!(config.default_provider, ImageProviderType::OpenAI);
    }

    #[test]
    fn test_config_builder() {
        let config = ImageConfig::new()
            .with_provider(ImageProviderType::StabilityAI)
            .with_model("stable-diffusion-xl");
        
        assert_eq!(config.default_provider, ImageProviderType::StabilityAI);
        assert_eq!(config.default_model, "stable-diffusion-xl");
    }

    #[test]
    fn test_generated_image() {
        let img = GeneratedImage {
            data: ImageData::Url("https://example.com/image.png".to_string()),
            revised_prompt: Some("A beautiful sunset".to_string()),
            model: "dall-e-3".to_string(),
            size: ImageSize::Square1024,
            seed: Some(12345),
        };

        assert!(img.is_url());
        assert!(!img.is_base64());
        assert_eq!(img.url(), Some("https://example.com/image.png"));
    }
}
