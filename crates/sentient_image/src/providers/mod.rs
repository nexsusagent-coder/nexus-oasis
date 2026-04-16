// ═══════════════════════════════════════════════════════════════════════════════
//  SENTIENT OS - Image Generation Providers
// ═══════════════════════════════════════════════════════════════════════════════

mod openai;
mod stability;
mod flux;
mod ideogram;
mod replicate;

pub use openai::OpenAIProvider;
pub use stability::StabilityProvider;
pub use flux::FluxProvider;
pub use ideogram::IdeogramProvider;
pub use replicate::ReplicateProvider;

use async_trait::async_trait;
use crate::{ImageRequest, GeneratedImage, Result, ImageError, ImageProviderType};

/// Image generator trait
#[async_trait]
pub trait ImageGenerator: Send + Sync {
    /// Generate image from prompt
    async fn generate(&self, request: &ImageRequest) -> Result<GeneratedImage>;
    
    /// Generate multiple images
    async fn generate_batch(&self, request: &ImageRequest, n: u8) -> Result<Vec<GeneratedImage>> {
        let mut images = Vec::with_capacity(n as usize);
        for _ in 0..n {
            images.push(self.generate(request).await?);
        }
        Ok(images)
    }
    
    /// Get provider type
    fn provider_type(&self) -> ImageProviderType;
}

/// Create image provider
pub fn create_provider(
    provider_type: ImageProviderType,
    api_key: &str,
) -> Result<Box<dyn ImageGenerator>> {
    match provider_type {
        ImageProviderType::OpenAI => Ok(Box::new(OpenAIProvider::new(api_key))),
        ImageProviderType::StabilityAI => Ok(Box::new(StabilityProvider::new(api_key))),
        ImageProviderType::Flux => Ok(Box::new(FluxProvider::new(api_key))),
        ImageProviderType::Ideogram => Ok(Box::new(IdeogramProvider::new(api_key))),
        ImageProviderType::Replicate => Ok(Box::new(ReplicateProvider::new(api_key))),
        _ => Err(ImageError::ProviderNotAvailable(format!("{:?}", provider_type))),
    }
}

/// Multi-provider image generator
pub struct ImageProvider {
    providers: std::collections::HashMap<ImageProviderType, Box<dyn ImageGenerator>>,
    default: ImageProviderType,
}

impl ImageProvider {
    /// Create new multi-provider
    pub fn new() -> Self {
        Self {
            providers: std::collections::HashMap::new(),
            default: ImageProviderType::OpenAI,
        }
    }

    /// Add provider
    pub fn add_provider(mut self, provider_type: ImageProviderType, api_key: &str) -> Result<Self> {
        let provider = create_provider(provider_type, api_key)?;
        self.providers.insert(provider_type, provider);
        Ok(self)
    }

    /// Set default provider
    pub fn with_default(mut self, provider: ImageProviderType) -> Self {
        self.default = provider;
        self
    }

    /// Generate image with default provider
    pub async fn generate(&self, request: &ImageRequest) -> Result<GeneratedImage> {
        let provider = self.providers.get(&self.default)
            .ok_or_else(|| ImageError::ProviderNotAvailable(format!("{:?}", self.default)))?;
        provider.generate(request).await
    }

    /// Generate with specific provider
    pub async fn generate_with(
        &self,
        provider_type: ImageProviderType,
        request: &ImageRequest,
    ) -> Result<GeneratedImage> {
        let provider = self.providers.get(&provider_type)
            .ok_or_else(|| ImageError::ProviderNotAvailable(format!("{:?}", provider_type)))?;
        provider.generate(request).await
    }

    /// Quick generate with DALL-E 3
    pub async fn dalle(&self, prompt: &str, api_key: &str) -> Result<GeneratedImage> {
        let provider = OpenAIProvider::new(api_key);
        provider.generate(&ImageRequest::dalle3(prompt)).await
    }

    /// Quick generate with Stable Diffusion
    pub async fn stable_diffusion(&self, prompt: &str, api_key: &str) -> Result<GeneratedImage> {
        let provider = StabilityProvider::new(api_key);
        provider.generate(&ImageRequest::sdxl(prompt)).await
    }
}

impl Default for ImageProvider {
    fn default() -> Self {
        Self::new()
    }
}
