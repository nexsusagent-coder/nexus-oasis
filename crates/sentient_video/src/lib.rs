//! ─── SENTIENT OS Video Generation ───
//!
//! AI-powered video generation with multiple providers
//! - **Runway** (Gen-2, Gen-3 Alpha) - Industry leader
//! - **Pika** (1.5, 2.0) - Fast and affordable
//! - **Luma AI** (Dream Machine) - Near Sora quality
//! - **Kling AI** (v1, v1.5) - High quality, longer videos
//! - **Haiper AI** (v2) - Fast generation
//! - **Stability AI** (SVD) - Image-to-video only
//!
//! # Quick Start
//!
//! ```ignore
//! use sentient_video::{VideoClient, VideoRequest, VideoBuilder};
//!
//! // Create client with your preferred provider
//! let client = VideoClient::runway("your-api-key");
//!
//! // Simple text-to-video
//! let request = VideoRequest::text_to_video("A cat playing piano");
//! let video = client.generate(request).await?;
//!
//! // Or use the builder for more control
//! let request = VideoBuilder::new("A sunset over mountains")
//!     .duration(5.0)
//!     .aspect_ratio(AspectRatio::Landscape16x9)
//!     .style(VideoStyle::Cinematic)
//!     .seed(42)
//!     .build();
//!
//! let video = client.generate(request).await?;
//! println!("Video URL: {}", video.url);
//! ```
//!
//! # Provider Comparison
//!
//! | Provider | Free Tier | Text-to-Video | Image-to-Video | Max Duration |
//! |----------|-----------|---------------|----------------|--------------|
//! | Runway | 125 credits | ✅ | ✅ | 18s |
//! | Pika | 250/month | ✅ | ✅ | 10s |
//! | Luma | 30/month | ✅ | ✅ | 5s |
//! | Kling | 66/day | ✅ | ✅ | 10s |
//! | Haiper | 150/month | ✅ | ✅ | 6s |
//! | Stability | 150 total | ❌ | ✅ | 6s |
//!
//! # Pricing (2025)
//!
//! ## Runway
//! - Free: 125 credits (new users)
//! - Standard: $12/month (625 credits)
//! - Pro: $28/month (1750 credits)
//! - Ultra: $76/month (5000 credits)
//! - ~$0.05-0.20/second
//!
//! ## Pika
//! - Free: 250 credits/month
//! - Standard: $8/month (500 credits)
//! - Pro: $28/month (2000 credits)
//! - ~$0.02-0.03/second
//!
//! ## Luma AI
//! - Free: 30 videos/month
//! - Pro: $29/month (500 videos)
//! - Premier: $99/month (2000 videos)
//! - ~$0.04/second
//!
//! ## Kling AI
//! - Free: 66 daily credits
//! - Standard: $10/month (300 credits)
//! - Pro: $30/month (1050 credits)
//! - Premium: $95/month (3600 credits)
//! - ~$0.02-0.025/second
//!
//! ## Haiper
//! - Free: 150 credits/month
//! - Creator: $15/month (500 credits)
//! - Pro: $50/month (2000 credits)
//! - ~$0.02/second
//!
//! ## Stability AI (SVD)
//! - Free: 150 credits
//! - Pro: $10/month
//! - ~$0.02-0.03/second
//!
//! # Choosing a Provider
//!
//! ## Best Quality: Kling AI, Luma AI
//! - Kling: Highest realism, good for people/faces
//! - Luma: Near Sora quality, excellent motion
//!
//! ## Best Speed: Pika, Haiper
//! - Pika: ~45s generation
//! - Haiper: ~60s generation
//!
//! ## Best Free Tier: Kling AI
//! - 66 daily credits = ~22 videos/day free
//! - Good quality
//!
//! ## Best Value: Pika
//! - Cheapest paid plans
//! - Good quality/speed balance
//!
//! ## Image-to-Video Only: Stability AI SVD
//! - Fastest for animating images
//! - No text-to-video
//!
//! # Examples
//!
//! ## Text-to-Video
//!
//! ```ignore
//! use sentient_video::{VideoClient, VideoBuilder, VideoStyle, CameraMotion};
//!
//! let client = VideoClient::kling("api-key");
//!
//! let request = VideoBuilder::new("A woman walking through Tokyo streets at night")
//!     .duration(5.0)
//!     .style(VideoStyle::Cinematic)
//!     .camera_motion(CameraMotion::DollyIn)
//!     .build();
//!
//! let video = client.generate(request).await?;
//! ```
//!
//! ## Image-to-Video
//!
//! ```ignore
//! use sentient_video::{VideoClient, VideoRequest, AspectRatio};
//!
//! let client = VideoClient::luma("api-key");
//!
//! let request = VideoRequest::image_to_video(
//!     "Animate this image with gentle motion",
//!     "https://example.com/your-image.jpg"
//! );
//!
//! let video = client.generate(request).await?;
//! ```
//!
//! ## Social Media Content (TikTok/Reels)
//!
//! ```ignore
//! use sentient_video::{VideoClient, VideoBuilder, AspectRatio, VideoStyle};
//!
//! let client = VideoClient::pika("api-key");
//!
//! let request = VideoBuilder::new("A product showcase with dramatic lighting")
//!     .duration(5.0)
//!     .aspect_ratio(AspectRatio::Portrait9x16) // 9:16 for TikTok/Reels
//!     .style(VideoStyle::Commercial)
//!     .build();
//!
//! let video = client.generate(request).await?;
//! ```
//!
//! ## Cost Estimation
//!
//! ```ignore
//! use sentient_video::VideoModel;
//!
//! let model = VideoModel::runway_gen3();
//! let cost = model.calculate_cost(5.0); // 5 seconds
//! println!("Cost: ${:.2}", cost); // $0.50
//!
//! // Compare all models
//! for model in VideoModel::by_cost() {
//!     println!("{}: ${:.3}/s", model.name, model.cost_per_second);
//! }
//! ```
//!
//! ## Status Checking
//!
//! ```ignore
//! use sentient_video::{VideoClient, VideoRequest};
//!
//! let client = VideoClient::runway("api-key");
//! let request = VideoRequest::text_to_video("A sunset");
//!
//! // Start generation
//! let video = client.generate(request).await?;
//!
//! // Or check status manually
//! let job = client.status("job-id").await?;
//! println!("Status: {:?}", job.status);
//! ```

pub mod error;
pub mod types;
pub mod providers;
pub mod template;

pub use error::{VideoError, VideoResult};
pub use types::{
    VideoRequest, VideoResponse, VideoStatus, VideoFormat,
    VideoResolution, AspectRatio, GenerationParams, VideoJob,
    VideoModel, VideoStyle, CameraMotion, GenerationType,
    GenerationParams as VideoParams, VideoBuilder,
};
pub use providers::{
    VideoProvider, VideoGenerator,
    RunwayProvider, PikaProvider, SVDProvider,
    LumaProvider, KlingProvider, HaiperProvider,
    SoraProvider, HailuoProvider,
    ProviderInfo,
};
pub use template::{VideoTemplate, TemplateLibrary, TemplateCategory};

use serde::{Deserialize, Serialize};
use std::sync::Arc;

// ═══════════════════════════════════════════════════════════════════════════════
//  VIDEO CLIENT
// ═══════════════════════════════════════════════════════════════════════════════

/// Main video generation client with multiple provider support
/// 
/// # Example
/// 
/// ```ignore
/// // Create client with Runway
/// let client = VideoClient::runway("api-key");
/// 
/// // Or with Pika
/// let client = VideoClient::pika("api-key");
/// 
/// // Or with Luma
/// let client = VideoClient::luma("api-key");
/// 
/// // Or with Kling
/// let client = VideoClient::kling("api-key");
/// 
/// // Or with Haiper
/// let client = VideoClient::haiper("api-key");
/// 
/// // Or with Stability AI (image-to-video only)
/// let client = VideoClient::stability("api-key");
/// ```
pub struct VideoClient {
    provider: Arc<dyn VideoProvider + Send + Sync>,
}

impl VideoClient {
    // ═══════════════════════════════════════════════════════════════════════════
    //  PROVIDER CONSTRUCTORS
    // ═══════════════════════════════════════════════════════════════════════════

    /// Create client with Runway provider
    /// 
    /// Best for: High quality, professional video production
    /// - Gen-3 Alpha: Highest quality
    /// - Gen-3 Turbo: Faster, slightly lower quality
    /// - Gen-2: Legacy, still good
    pub fn runway(api_key: impl Into<String>) -> Self {
        Self {
            provider: Arc::new(RunwayProvider::new(api_key.into())),
        }
    }

    /// Create client with Pika provider
    /// 
    /// Best for: Fast, affordable videos
    /// - Pika 2.0: Latest model
    /// - Pika 1.5: Stable, reliable
    pub fn pika(api_key: impl Into<String>) -> Self {
        Self {
            provider: Arc::new(PikaProvider::new(api_key.into())),
        }
    }

    /// Create client with Luma AI provider
    /// 
    /// Best for: Near Sora quality, smooth motion
    /// - Dream Machine: Main model
    pub fn luma(api_key: impl Into<String>) -> Self {
        Self {
            provider: Arc::new(LumaProvider::new(api_key.into())),
        }
    }

    /// Create client with Kling AI provider
    /// 
    /// Best for: Highest quality realistic videos, longest free tier
    /// - Kling v1.5: Latest, best quality
    /// - Kling v1: Still excellent
    pub fn kling(api_key: impl Into<String>) -> Self {
        Self {
            provider: Arc::new(KlingProvider::new(api_key.into())),
        }
    }

    /// Create client with Haiper AI provider
    /// 
    /// Best for: Fast generation, good value
    /// - Haiper v2: Current model
    pub fn haiper(api_key: impl Into<String>) -> Self {
        Self {
            provider: Arc::new(HaiperProvider::new(api_key.into())),
        }
    }

    /// Create client with Stability AI provider (SVD)
    /// 
    /// Best for: Image-to-video only (no text-to-video)
    /// - SVD XT: Extended, 6 seconds
    /// - SVD: Standard, 4 seconds
    pub fn stability(api_key: impl Into<String>) -> Self {
        Self {
            provider: Arc::new(SVDProvider::new(api_key.into())),
        }
    }

    /// Alias for stability()
    pub fn svd(api_key: impl Into<String>) -> Self {
        Self::stability(api_key)
    }

    /// Create client with OpenAI Sora provider
    /// 
    /// Best for: High quality, natural motion, OpenAI ecosystem
    /// - Sora 1.0: High quality, up to 20 seconds
    /// - Sora 1.0 Turbo: Faster, shorter videos
    pub fn sora(api_key: impl Into<String>) -> Self {
        Self {
            provider: Arc::new(SoraProvider::new(api_key)),
        }
    }

    /// Create client with Hailuo AI (MiniMax) provider
    /// 
    /// Best for: Asian content, high realism, Chinese faces
    /// - Hailuo 01: Main model, 6s videos
    /// - Hailuo 01 Live: Higher quality, 10s videos
    pub fn hailuo(api_key: impl Into<String>) -> Self {
        Self {
            provider: Arc::new(HailuoProvider::new(api_key)),
        }
    }

    /// Create client with custom provider
    pub fn with_provider(provider: impl VideoProvider + Send + Sync + 'static) -> Self {
        Self {
            provider: Arc::new(provider),
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    //  GENERATION METHODS
    // ═══════════════════════════════════════════════════════════════════════════

    /// Generate a video from a request
    /// 
    /// This blocks until generation is complete.
    /// For async status checking, use `start()` and `status()`.
    pub async fn generate(&self, request: VideoRequest) -> VideoResult<VideoResponse> {
        log::info!("🎬 Starting video generation...");
        self.provider.generate(request).await
    }

    /// Check the status of a generation job
    pub async fn status(&self, job_id: &str) -> VideoResult<VideoJob> {
        self.provider.status(job_id).await
    }

    /// Cancel a generation job
    pub async fn cancel(&self, job_id: &str) -> VideoResult<()> {
        self.provider.cancel(job_id).await
    }

    /// Get available models for this provider
    pub fn models(&self) -> Vec<VideoModel> {
        self.provider.models()
    }

    /// Get provider info
    pub fn provider_info(&self) -> ProviderInfo {
        // Determine which provider we're using
        let models = self.models();
        if models.is_empty() {
            return ProviderInfo::runway(); // Default
        }

        let provider_name = &models[0].provider;
        match provider_name.as_str() {
            "Runway" => ProviderInfo::runway(),
            "Pika" => ProviderInfo::pika(),
            "Luma AI" => ProviderInfo::luma(),
            "Kling" => ProviderInfo::kling(),
            "Haiper" => ProviderInfo::haiper(),
            "Stability AI" => ProviderInfo::stability(),
            _ => ProviderInfo::runway(),
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    //  UTILITY METHODS
    // ═══════════════════════════════════════════════════════════════════════════

    /// Get all available providers
    pub fn all_providers() -> Vec<ProviderInfo> {
        ProviderInfo::all()
    }

    /// Get free tier providers
    pub fn free_tier_providers() -> Vec<ProviderInfo> {
        ProviderInfo::free_tier()
    }

    /// Get all available models across all providers
    pub fn all_models() -> Vec<VideoModel> {
        VideoModel::all()
    }

    /// Get models sorted by quality (best first)
    pub fn models_by_quality() -> Vec<VideoModel> {
        VideoModel::by_quality()
    }

    /// Get models sorted by speed (fastest first)
    pub fn models_by_speed() -> Vec<VideoModel> {
        VideoModel::by_speed()
    }

    /// Get models sorted by cost (cheapest first)
    pub fn models_by_cost() -> Vec<VideoModel> {
        VideoModel::by_cost()
    }

    /// Get free tier models
    pub fn free_tier_models() -> Vec<VideoModel> {
        VideoModel::free_tier_models()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  VIDEO STATISTICS
// ═══════════════════════════════════════════════════════════════════════════════

/// Statistics for video generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoStats {
    /// Total videos generated
    pub total_generated: u64,
    /// Total duration in seconds
    pub total_duration_seconds: f64,
    /// Total cost in USD
    pub total_cost: f64,
    /// Average generation time in seconds
    pub avg_generation_time: f64,
    /// Success rate (0.0 - 1.0)
    pub success_rate: f64,
}

impl Default for VideoStats {
    fn default() -> Self {
        Self {
            total_generated: 0,
            total_duration_seconds: 0.0,
            total_cost: 0.0,
            avg_generation_time: 0.0,
            success_rate: 1.0,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_constructors() {
        // Just verify they compile
        let _runway = VideoClient::runway("key");
        let _pika = VideoClient::pika("key");
        let _luma = VideoClient::luma("key");
        let _kling = VideoClient::kling("key");
        let _haiper = VideoClient::haiper("key");
        let _stability = VideoClient::stability("key");
    }

    #[test]
    fn test_all_models() {
        let models = VideoClient::all_models();
        assert!(!models.is_empty());
    }

    #[test]
    fn test_models_by_quality() {
        let models = VideoClient::models_by_quality();
        assert!(!models.is_empty());
        assert!(models[0].quality_rating >= models[models.len()-1].quality_rating);
    }

    #[test]
    fn test_models_by_cost() {
        let models = VideoClient::models_by_cost();
        assert!(!models.is_empty());
        assert!(models[0].cost_per_second <= models[models.len()-1].cost_per_second);
    }

    #[test]
    fn test_free_tier_models() {
        let models = VideoClient::free_tier_models();
        assert!(models.iter().all(|m| m.free_tier));
    }

    #[test]
    fn test_all_providers() {
        let providers = VideoClient::all_providers();
        assert_eq!(providers.len(), 8);
    }

    #[test]
    fn test_provider_info() {
        let runway = ProviderInfo::runway();
        assert!(runway.supports_text_to_video);
        assert!(runway.free_tier);
    }

    #[test]
    fn test_video_stats_default() {
        let stats = VideoStats::default();
        assert_eq!(stats.total_generated, 0);
        assert_eq!(stats.success_rate, 1.0);
    }
}
