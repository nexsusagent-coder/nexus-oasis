//! ─── Video Providers ───
//!
//! Video generation provider implementations

mod runway;
mod pika;
mod svd;
mod luma;
mod kling;
mod haiper;
mod sora;
mod hailuo;

pub use runway::RunwayProvider;
pub use pika::PikaProvider;
pub use svd::SVDProvider;
pub use luma::LumaProvider;
pub use kling::KlingProvider;
pub use haiper::HaiperProvider;
pub use sora::SoraProvider;
pub use hailuo::HailuoProvider;

use async_trait::async_trait;
use reqwest::{Client, Response};
use std::time::Duration;

use crate::{VideoRequest, VideoResponse, VideoJob, VideoModel, VideoResult};

// ═══════════════════════════════════════════════════════════════════════════════
//  VIDEO PROVIDER TRAIT
// ═══════════════════════════════════════════════════════════════════════════════

/// Trait for video generation providers
#[async_trait]
pub trait VideoProvider: Send + Sync {
    /// Generate a video from a request
    async fn generate(&self, request: VideoRequest) -> VideoResult<VideoResponse>;
    
    /// Check the status of a generation job
    async fn status(&self, job_id: &str) -> VideoResult<VideoJob>;
    
    /// Cancel a generation job
    async fn cancel(&self, job_id: &str) -> VideoResult<()>;
    
    /// List available models
    fn models(&self) -> Vec<VideoModel>;
}

// ═══════════════════════════════════════════════════════════════════════════════
//  VIDEO GENERATOR (DYNAMIC DISPATCH)
// ═══════════════════════════════════════════════════════════════════════════════

/// Video generator with dynamic provider
pub struct VideoGenerator {
    provider: Box<dyn VideoProvider>,
}

impl VideoGenerator {
    /// Create with a specific provider
    pub fn new(provider: impl VideoProvider + 'static) -> Self {
        Self {
            provider: Box::new(provider),
        }
    }

    /// Generate video
    pub async fn generate(&self, request: VideoRequest) -> VideoResult<VideoResponse> {
        self.provider.generate(request).await
    }

    /// Check status
    pub async fn status(&self, job_id: &str) -> VideoResult<VideoJob> {
        self.provider.status(job_id).await
    }

    /// Cancel generation
    pub async fn cancel(&self, job_id: &str) -> VideoResult<()> {
        self.provider.cancel(job_id).await
    }

    /// Get available models
    pub fn models(&self) -> Vec<VideoModel> {
        self.provider.models()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  HELPER FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════════

/// Build a configured HTTP client
pub fn build_client() -> Client {
    Client::builder()
        .timeout(Duration::from_secs(300))
        .connect_timeout(Duration::from_secs(30))
        .user_agent("SENTIENT-OS/4.0 VideoGenerator")
        .build()
        .expect("Failed to build HTTP client")
}

/// Parse API error from response
pub fn parse_api_error(response: Response) -> String {
    let status = response.status();
    format!("API error: {} ({})", status.as_str(), status.canonical_reason().unwrap_or("Unknown"))
}

// ═══════════════════════════════════════════════════════════════════════════════
//  PROVIDER INFO
// ═══════════════════════════════════════════════════════════════════════════════

/// Provider information
#[derive(Debug, Clone)]
pub struct ProviderInfo {
    pub name: String,
    pub website: String,
    pub docs: String,
    pub free_tier: bool,
    pub free_tier_limit: Option<String>,
    pub pricing_url: String,
    pub supports_text_to_video: bool,
    pub supports_image_to_video: bool,
    pub max_duration_seconds: f32,
    pub avg_generation_time_seconds: f32,
}

impl ProviderInfo {
    /// Get Runway info
    pub fn runway() -> Self {
        Self {
            name: "Runway".into(),
            website: "https://runwayml.com/".into(),
            docs: "https://docs.runwayml.com/".into(),
            free_tier: true,
            free_tier_limit: Some("125 credits (new users)".into()),
            pricing_url: "https://runwayml.com/pricing/".into(),
            supports_text_to_video: true,
            supports_image_to_video: true,
            max_duration_seconds: 18.0,
            avg_generation_time_seconds: 90.0,
        }
    }

    /// Get Pika info
    pub fn pika() -> Self {
        Self {
            name: "Pika".into(),
            website: "https://pika.art/".into(),
            docs: "https://pika.art/docs".into(),
            free_tier: true,
            free_tier_limit: Some("250 credits/month".into()),
            pricing_url: "https://pika.art/pricing".into(),
            supports_text_to_video: true,
            supports_image_to_video: true,
            max_duration_seconds: 10.0,
            avg_generation_time_seconds: 45.0,
        }
    }

    /// Get Luma AI info
    pub fn luma() -> Self {
        Self {
            name: "Luma AI".into(),
            website: "https://lumalabs.ai/".into(),
            docs: "https://lumalabs.ai/api/docs".into(),
            free_tier: true,
            free_tier_limit: Some("30 videos/month".into()),
            pricing_url: "https://lumalabs.ai/pricing".into(),
            supports_text_to_video: true,
            supports_image_to_video: true,
            max_duration_seconds: 5.0,
            avg_generation_time_seconds: 120.0,
        }
    }

    /// Get Kling AI info
    pub fn kling() -> Self {
        Self {
            name: "Kling AI".into(),
            website: "https://klingai.com/".into(),
            docs: "https://klingai.com/docs".into(),
            free_tier: true,
            free_tier_limit: Some("66 daily credits".into()),
            pricing_url: "https://klingai.com/pricing".into(),
            supports_text_to_video: true,
            supports_image_to_video: true,
            max_duration_seconds: 10.0,
            avg_generation_time_seconds: 180.0,
        }
    }

    /// Get Haiper AI info
    pub fn haiper() -> Self {
        Self {
            name: "Haiper".into(),
            website: "https://haiper.ai/".into(),
            docs: "https://docs.haiper.ai/".into(),
            free_tier: true,
            free_tier_limit: Some("150 credits/month".into()),
            pricing_url: "https://haiper.ai/pricing".into(),
            supports_text_to_video: true,
            supports_image_to_video: true,
            max_duration_seconds: 6.0,
            avg_generation_time_seconds: 60.0,
        }
    }

    /// Get Stability AI info
    pub fn stability() -> Self {
        Self {
            name: "Stability AI".into(),
            website: "https://stability.ai/".into(),
            docs: "https://platform.stability.ai/docs".into(),
            free_tier: true,
            free_tier_limit: Some("150 credits".into()),
            pricing_url: "https://platform.stability.ai/pricing".into(),
            supports_text_to_video: false,
            supports_image_to_video: true,
            max_duration_seconds: 6.0,
            avg_generation_time_seconds: 30.0,
        }
    }

    /// Get Sora (OpenAI) info
    pub fn sora() -> Self {
        Self {
            name: "OpenAI Sora".into(),
            website: "https://openai.com/sora".into(),
            docs: "https://platform.openai.com/docs/guides/video".into(),
            free_tier: true,
            free_tier_limit: Some("Included with ChatGPT Plus/Pro".into()),
            pricing_url: "https://openai.com/pricing".into(),
            supports_text_to_video: true,
            supports_image_to_video: true,
            max_duration_seconds: 20.0,
            avg_generation_time_seconds: 120.0,
        }
    }

    /// Get Hailuo AI (MiniMax) info
    pub fn hailuo() -> Self {
        Self {
            name: "Hailuo AI".into(),
            website: "https://hailuoai.com/".into(),
            docs: "https://docs.hailuo.ai/".into(),
            free_tier: true,
            free_tier_limit: Some("20 videos/day".into()),
            pricing_url: "https://hailuoai.com/pricing".into(),
            supports_text_to_video: true,
            supports_image_to_video: true,
            max_duration_seconds: 6.0,
            avg_generation_time_seconds: 90.0,
        }
    }

    /// Get all providers
    pub fn all() -> Vec<Self> {
        vec![
            Self::runway(),
            Self::pika(),
            Self::luma(),
            Self::kling(),
            Self::haiper(),
            Self::stability(),
            Self::sora(),
            Self::hailuo(),
        ]
    }

    /// Get free tier providers
    pub fn free_tier() -> Vec<Self> {
        Self::all().into_iter().filter(|p| p.free_tier).collect()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_providers_exist() {
        let providers = ProviderInfo::all();
        assert_eq!(providers.len(), 8);
    }

    #[test]
    fn test_all_have_free_tier() {
        let providers = ProviderInfo::all();
        assert!(providers.iter().all(|p| p.free_tier));
    }

    #[test]
    fn test_provider_info() {
        let runway = ProviderInfo::runway();
        assert!(runway.supports_text_to_video);
        assert!(runway.supports_image_to_video);
    }

    #[test]
    fn test_svd_no_text_to_video() {
        let stability = ProviderInfo::stability();
        assert!(!stability.supports_text_to_video);
        assert!(stability.supports_image_to_video);
    }

    #[test]
    fn test_build_client() {
        let client = build_client();
        // Client should be usable
        drop(client);
    }
}
