//! ─── Video Types ───
//!
//! Core types for video generation

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// ═══════════════════════════════════════════════════════════════════════════════
//  VIDEO REQUEST
// ═══════════════════════════════════════════════════════════════════════════════

/// Video generation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoRequest {
    /// Text prompt describing the video
    pub prompt: String,
    
    /// Negative prompt (what to avoid)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    
    /// Image URL for image-to-video generation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    
    /// Video duration in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<f32>,
    
    /// Aspect ratio
    #[serde(default)]
    pub aspect_ratio: AspectRatio,
    
    /// Resolution
    #[serde(default)]
    pub resolution: VideoResolution,
    
    /// Generation parameters
    #[serde(default)]
    pub params: GenerationParams,
    
    /// Model identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    
    /// Random seed for reproducibility
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    
    /// Style preset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<VideoStyle>,
    
    /// Camera motion
    #[serde(skip_serializing_if = "Option::is_none")]
    pub camera_motion: Option<CameraMotion>,
    
    /// Loop the video
    #[serde(default)]
    pub loop_video: bool,
}

impl VideoRequest {
    /// Create text-to-video request
    pub fn text_to_video(prompt: impl Into<String>) -> Self {
        Self {
            prompt: prompt.into(),
            negative_prompt: None,
            image_url: None,
            duration: Some(4.0),
            aspect_ratio: AspectRatio::Landscape16x9,
            resolution: VideoResolution::HD,
            params: GenerationParams::default(),
            model: None,
            seed: None,
            style: None,
            camera_motion: None,
            loop_video: false,
        }
    }

    /// Create image-to-video request
    pub fn image_to_video(prompt: impl Into<String>, image_url: impl Into<String>) -> Self {
        Self {
            prompt: prompt.into(),
            negative_prompt: None,
            image_url: Some(image_url.into()),
            duration: Some(4.0),
            aspect_ratio: AspectRatio::Landscape16x9,
            resolution: VideoResolution::HD,
            params: GenerationParams::default(),
            model: None,
            seed: None,
            style: None,
            camera_motion: None,
            loop_video: false,
        }
    }

    /// Get generation type
    pub fn generation_type(&self) -> GenerationType {
        if self.image_url.is_some() {
            GenerationType::ImageToVideo
        } else {
            GenerationType::TextToVideo
        }
    }
}

/// Generation type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GenerationType {
    TextToVideo,
    ImageToVideo,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  VIDEO RESPONSE
// ═══════════════════════════════════════════════════════════════════════════════

/// Video generation response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoResponse {
    /// Unique video ID
    pub id: String,
    
    /// Video URL (MP4)
    pub url: String,
    
    /// Duration in seconds
    pub duration: f32,
    
    /// Video format
    pub format: VideoFormat,
    
    /// Resolution
    pub resolution: VideoResolution,
    
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    
    /// Thumbnail URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_url: Option<String>,
    
    /// File size in bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<u64>,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  VIDEO JOB
// ═══════════════════════════════════════════════════════════════════════════════

/// Video generation job status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoJob {
    /// Job ID
    pub id: String,
    
    /// Current status
    pub status: VideoStatus,
    
    /// Video URL (when complete)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    
    /// Duration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<f32>,
    
    /// Resolution
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<VideoResolution>,
    
    /// Progress percentage (0-100)
    pub progress: u8,
    
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    
    /// Estimated completion time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_completion: Option<DateTime<Utc>>,
}

/// Video generation status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VideoStatus {
    Pending,
    Queued,
    Processing,
    Completed,
    Failed(String),
    Cancelled,
}

impl VideoStatus {
    pub fn is_terminal(&self) -> bool {
        matches!(self, VideoStatus::Completed | VideoStatus::Failed(_) | VideoStatus::Cancelled)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  VIDEO MODEL
// ═══════════════════════════════════════════════════════════════════════════════

/// Video model information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoModel {
    /// Model ID
    pub id: String,
    
    /// Display name
    pub name: String,
    
    /// Provider name
    pub provider: String,
    
    /// Supports text-to-video
    pub text_to_video: bool,
    
    /// Supports image-to-video
    pub image_to_video: bool,
    
    /// Maximum duration in seconds
    pub max_duration: f32,
    
    /// Supported aspect ratios
    pub aspect_ratios: Vec<AspectRatio>,
    
    /// Cost per second in USD
    pub cost_per_second: f32,
    
    /// Free tier available
    pub free_tier: bool,
    
    /// Free tier limit (videos per month)
    pub free_tier_limit: Option<u32>,
    
    /// Average generation time in seconds
    pub avg_generation_time: f32,
    
    /// Quality rating (1-5)
    pub quality_rating: u8,
    
    /// Speed rating (1-5)
    pub speed_rating: u8,
}

impl VideoModel {
    // ═══════════════════════════════════════════════════════════════════════════
    //  RUNWAY MODELS
    // ═══════════════════════════════════════════════════════════════════════════
    
    /// Runway Gen-2 (Legacy, but still good)
    pub fn runway_gen2() -> Self {
        Self {
            id: "gen2".into(),
            name: "Runway Gen-2".into(),
            provider: "Runway".into(),
            text_to_video: true,
            image_to_video: true,
            max_duration: 18.0,
            aspect_ratios: vec![AspectRatio::Landscape16x9, AspectRatio::Portrait9x16, AspectRatio::Square],
            cost_per_second: 0.05,
            free_tier: true,
            free_tier_limit: Some(125), // 125 credits for new users
            avg_generation_time: 90.0,
            quality_rating: 4,
            speed_rating: 3,
        }
    }

    /// Runway Gen-3 Alpha (Latest, highest quality)
    pub fn runway_gen3() -> Self {
        Self {
            id: "gen3a_turbo".into(),
            name: "Runway Gen-3 Alpha Turbo".into(),
            provider: "Runway".into(),
            text_to_video: true,
            image_to_video: true,
            max_duration: 10.0,
            aspect_ratios: vec![AspectRatio::Landscape16x9, AspectRatio::Portrait9x16],
            cost_per_second: 0.10, // $0.10/second
            free_tier: true,
            free_tier_limit: Some(125),
            avg_generation_time: 60.0,
            quality_rating: 5,
            speed_rating: 4,
        }
    }

    /// Runway Gen-3 Alpha (Full quality)
    pub fn runway_gen3_alpha() -> Self {
        Self {
            id: "gen3a".into(),
            name: "Runway Gen-3 Alpha".into(),
            provider: "Runway".into(),
            text_to_video: true,
            image_to_video: true,
            max_duration: 10.0,
            aspect_ratios: vec![AspectRatio::Landscape16x9, AspectRatio::Portrait9x16],
            cost_per_second: 0.20, // $0.20/second
            free_tier: true,
            free_tier_limit: Some(125),
            avg_generation_time: 120.0,
            quality_rating: 5,
            speed_rating: 3,
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    //  PIKA MODELS
    // ═══════════════════════════════════════════════════════════════════════════

    /// Pika 1.5 (Latest)
    pub fn pika1_5() -> Self {
        Self {
            id: "pika1.5".into(),
            name: "Pika 1.5".into(),
            provider: "Pika".into(),
            text_to_video: true,
            image_to_video: true,
            max_duration: 5.0,
            aspect_ratios: vec![AspectRatio::Landscape16x9, AspectRatio::Portrait9x16, AspectRatio::Square],
            cost_per_second: 0.02,
            free_tier: true,
            free_tier_limit: Some(250), // 250 credits/month
            avg_generation_time: 45.0,
            quality_rating: 4,
            speed_rating: 4,
        }
    }

    /// Pika 2.0 (Newest)
    pub fn pika2() -> Self {
        Self {
            id: "pika2.0".into(),
            name: "Pika 2.0".into(),
            provider: "Pika".into(),
            text_to_video: true,
            image_to_video: true,
            max_duration: 10.0,
            aspect_ratios: vec![AspectRatio::Landscape16x9, AspectRatio::Portrait9x16, AspectRatio::Square],
            cost_per_second: 0.03,
            free_tier: true,
            free_tier_limit: Some(150),
            avg_generation_time: 50.0,
            quality_rating: 5,
            speed_rating: 4,
        }
    }

    /// Pika 1.0 (Legacy)
    pub fn pika1() -> Self {
        Self {
            id: "pika1.0".into(),
            name: "Pika 1.0".into(),
            provider: "Pika".into(),
            text_to_video: true,
            image_to_video: true,
            max_duration: 4.0,
            aspect_ratios: vec![AspectRatio::Landscape16x9, AspectRatio::Portrait9x16],
            cost_per_second: 0.015,
            free_tier: true,
            free_tier_limit: Some(250),
            avg_generation_time: 40.0,
            quality_rating: 3,
            speed_rating: 4,
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    //  LUMA AI MODELS
    // ═══════════════════════════════════════════════════════════════════════════

    /// Luma Dream Machine
    pub fn luma_dream_machine() -> Self {
        Self {
            id: "dream-machine".into(),
            name: "Luma Dream Machine".into(),
            provider: "Luma AI".into(),
            text_to_video: true,
            image_to_video: true,
            max_duration: 5.0,
            aspect_ratios: vec![AspectRatio::Landscape16x9, AspectRatio::Portrait9x16, AspectRatio::Square],
            cost_per_second: 0.04,
            free_tier: true,
            free_tier_limit: Some(30), // 30 videos/month free
            avg_generation_time: 120.0,
            quality_rating: 5,
            speed_rating: 3,
        }
    }

    /// Luma Ray
    pub fn luma_ray() -> Self {
        Self {
            id: "ray".into(),
            name: "Luma Ray".into(),
            provider: "Luma AI".into(),
            text_to_video: true,
            image_to_video: true,
            max_duration: 5.0,
            aspect_ratios: vec![AspectRatio::Landscape16x9, AspectRatio::Portrait9x16],
            cost_per_second: 0.03,
            free_tier: true,
            free_tier_limit: Some(30),
            avg_generation_time: 90.0,
            quality_rating: 4,
            speed_rating: 3,
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    //  KLING AI MODELS
    // ═══════════════════════════════════════════════════════════════════════════

    /// Kling AI v1
    pub fn kling_v1() -> Self {
        Self {
            id: "kling-v1".into(),
            name: "Kling AI v1".into(),
            provider: "Kling".into(),
            text_to_video: true,
            image_to_video: true,
            max_duration: 10.0,
            aspect_ratios: vec![AspectRatio::Landscape16x9, AspectRatio::Portrait9x16],
            cost_per_second: 0.02,
            free_tier: true,
            free_tier_limit: Some(66), // 66 daily credits
            avg_generation_time: 180.0, // Slower but high quality
            quality_rating: 5,
            speed_rating: 2,
        }
    }

    /// Kling AI v1.5 (Latest)
    pub fn kling_v1_5() -> Self {
        Self {
            id: "kling-v1.5".into(),
            name: "Kling AI v1.5".into(),
            provider: "Kling".into(),
            text_to_video: true,
            image_to_video: true,
            max_duration: 10.0,
            aspect_ratios: vec![AspectRatio::Landscape16x9, AspectRatio::Portrait9x16],
            cost_per_second: 0.025,
            free_tier: true,
            free_tier_limit: Some(66),
            avg_generation_time: 150.0,
            quality_rating: 5,
            speed_rating: 2,
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    //  STABILITY AI MODELS
    // ═══════════════════════════════════════════════════════════════════════════

    /// Stable Video Diffusion (Image-to-Video only)
    pub fn svd() -> Self {
        Self {
            id: "stable-video-diffusion".into(),
            name: "Stable Video Diffusion".into(),
            provider: "Stability AI".into(),
            text_to_video: false,
            image_to_video: true,
            max_duration: 4.0,
            aspect_ratios: vec![AspectRatio::Landscape16x9, AspectRatio::Portrait9x16, AspectRatio::Square],
            cost_per_second: 0.02,
            free_tier: true,
            free_tier_limit: Some(150), // Stability free tier
            avg_generation_time: 30.0,
            quality_rating: 3,
            speed_rating: 4,
        }
    }

    /// Stable Video Diffusion XT (Extended)
    pub fn svd_xt() -> Self {
        Self {
            id: "svd-xt".into(),
            name: "SVD XT (Extended)".into(),
            provider: "Stability AI".into(),
            text_to_video: false,
            image_to_video: true,
            max_duration: 6.0,
            aspect_ratios: vec![AspectRatio::Landscape16x9, AspectRatio::Portrait9x16],
            cost_per_second: 0.03,
            free_tier: true,
            free_tier_limit: Some(150),
            avg_generation_time: 45.0,
            quality_rating: 4,
            speed_rating: 3,
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    //  HAIPER AI MODELS
    // ═══════════════════════════════════════════════════════════════════════════

    /// Haiper AI v2
    pub fn haiper_v2() -> Self {
        Self {
            id: "haiper-v2".into(),
            name: "Haiper AI v2".into(),
            provider: "Haiper".into(),
            text_to_video: true,
            image_to_video: true,
            max_duration: 6.0,
            aspect_ratios: vec![AspectRatio::Landscape16x9, AspectRatio::Portrait9x16, AspectRatio::Square],
            cost_per_second: 0.02,
            free_tier: true,
            free_tier_limit: Some(150),
            avg_generation_time: 60.0,
            quality_rating: 4,
            speed_rating: 4,
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    //  OPENAI SORA (Preview)
    // ═══════════════════════════════════════════════════════════════════════════

    /// OpenAI Sora (when available)
    pub fn sora() -> Self {
        Self {
            id: "sora".into(),
            name: "OpenAI Sora".into(),
            provider: "OpenAI".into(),
            text_to_video: true,
            image_to_video: true,
            max_duration: 60.0, // Sora supports up to 60 seconds
            aspect_ratios: vec![AspectRatio::Landscape16x9, AspectRatio::Portrait9x16, AspectRatio::Square],
            cost_per_second: 0.20, // Estimated
            free_tier: false,
            free_tier_limit: None,
            avg_generation_time: 300.0, // Longer generations
            quality_rating: 5,
            speed_rating: 2,
        }
    }

    /// OpenAI Sora 2.0 (2026)
    pub fn sora2() -> Self {
        Self {
            id: "sora-2.0".into(),
            name: "OpenAI Sora 2.0".into(),
            provider: "OpenAI".into(),
            text_to_video: true,
            image_to_video: true,
            max_duration: 120.0, // 2 minute videos!
            aspect_ratios: vec![AspectRatio::Landscape16x9, AspectRatio::Portrait9x16, AspectRatio::Square, AspectRatio::Cinematic2_39_1],
            cost_per_second: 0.30,
            free_tier: false,
            free_tier_limit: None,
            avg_generation_time: 180.0,
            quality_rating: 5,
            speed_rating: 3,
        }
    }

    /// Runway Gen-4 Alpha (2026)
    pub fn runway_gen4() -> Self {
        Self {
            id: "gen4_alpha".into(),
            name: "Runway Gen-4 Alpha".into(),
            provider: "Runway".into(),
            text_to_video: true,
            image_to_video: true,
            max_duration: 30.0,
            aspect_ratios: vec![AspectRatio::Landscape16x9, AspectRatio::Portrait9x16, AspectRatio::Square],
            cost_per_second: 0.15,
            free_tier: true,
            free_tier_limit: Some(50),
            avg_generation_time: 60.0,
            quality_rating: 5,
            speed_rating: 4,
        }
    }

    /// Kling AI v2.0 (2026)
    pub fn kling_v2() -> Self {
        Self {
            id: "kling-v2".into(),
            name: "Kling AI v2.0".into(),
            provider: "Kling AI".into(),
            text_to_video: true,
            image_to_video: true,
            max_duration: 15.0,
            aspect_ratios: vec![AspectRatio::Landscape16x9, AspectRatio::Portrait9x16, AspectRatio::Square],
            cost_per_second: 0.04,
            free_tier: true,
            free_tier_limit: Some(66),
            avg_generation_time: 120.0,
            quality_rating: 5,
            speed_rating: 3,
        }
    }

    /// Kling AI v2.1 Master (2026)
    pub fn kling_v2_master() -> Self {
        Self {
            id: "kling-v2-master".into(),
            name: "Kling AI v2.1 Master".into(),
            provider: "Kling AI".into(),
            text_to_video: true,
            image_to_video: true,
            max_duration: 20.0,
            aspect_ratios: vec![AspectRatio::Landscape16x9, AspectRatio::Portrait9x16, AspectRatio::Square, AspectRatio::Cinematic2_39_1],
            cost_per_second: 0.08,
            free_tier: true,
            free_tier_limit: Some(30),
            avg_generation_time: 180.0,
            quality_rating: 5,
            speed_rating: 2,
        }
    }

    /// Luma Dream Machine 2.0 (2026)
    pub fn luma_dream_machine_2() -> Self {
        Self {
            id: "dream-machine-2".into(),
            name: "Luma Dream Machine 2.0".into(),
            provider: "Luma AI".into(),
            text_to_video: true,
            image_to_video: true,
            max_duration: 10.0,
            aspect_ratios: vec![AspectRatio::Landscape16x9, AspectRatio::Portrait9x16, AspectRatio::Square],
            cost_per_second: 0.06,
            free_tier: true,
            free_tier_limit: Some(30),
            avg_generation_time: 60.0,
            quality_rating: 5,
            speed_rating: 4,
        }
    }

    /// Luma Ray2 (2026)
    pub fn luma_ray2() -> Self {
        Self {
            id: "ray2".into(),
            name: "Luma Ray2".into(),
            provider: "Luma AI".into(),
            text_to_video: true,
            image_to_video: true,
            max_duration: 15.0,
            aspect_ratios: vec![AspectRatio::Landscape16x9, AspectRatio::Portrait9x16, AspectRatio::Square, AspectRatio::Cinematic2_39_1],
            cost_per_second: 0.10,
            free_tier: true,
            free_tier_limit: Some(10),
            avg_generation_time: 90.0,
            quality_rating: 5,
            speed_rating: 3,
        }
    }

    /// Pika 3.0 (2026)
    pub fn pika3() -> Self {
        Self {
            id: "pika-3".into(),
            name: "Pika 3.0".into(),
            provider: "Pika".into(),
            text_to_video: true,
            image_to_video: true,
            max_duration: 15.0,
            aspect_ratios: vec![AspectRatio::Landscape16x9, AspectRatio::Portrait9x16, AspectRatio::Square],
            cost_per_second: 0.05,
            free_tier: true,
            free_tier_limit: Some(250),
            avg_generation_time: 30.0,
            quality_rating: 4,
            speed_rating: 5,
        }
    }

    /// Hailuo 02 (2026)
    pub fn hailuo_02() -> Self {
        Self {
            id: "hailuo-02".into(),
            name: "Hailuo 02".into(),
            provider: "Hailuo AI".into(),
            text_to_video: true,
            image_to_video: true,
            max_duration: 10.0,
            aspect_ratios: vec![AspectRatio::Landscape16x9, AspectRatio::Portrait9x16, AspectRatio::Square],
            cost_per_second: 0.05,
            free_tier: true,
            free_tier_limit: Some(30),
            avg_generation_time: 75.0,
            quality_rating: 5,
            speed_rating: 3,
        }
    }

    /// Veo 2 (Google DeepMind, 2026)
    pub fn veo2() -> Self {
        Self {
            id: "veo-2".into(),
            name: "Google Veo 2".into(),
            provider: "Google".into(),
            text_to_video: true,
            image_to_video: true,
            max_duration: 60.0,
            aspect_ratios: vec![AspectRatio::Landscape16x9, AspectRatio::Portrait9x16, AspectRatio::Square],
            cost_per_second: 0.15,
            free_tier: true,
            free_tier_limit: Some(20), // Via Google AI Studio
            avg_generation_time: 120.0,
            quality_rating: 5,
            speed_rating: 3,
        }
    }

    /// Hailuo 01 Live
    pub fn hailuo_01_live() -> Self {
        Self {
            id: "hailuo-01-live".into(),
            name: "Hailuo 01 Live".into(),
            provider: "Hailuo AI".into(),
            text_to_video: true,
            image_to_video: true,
            max_duration: 10.0,
            aspect_ratios: vec![AspectRatio::Landscape16x9, AspectRatio::Portrait9x16],
            cost_per_second: 0.06,
            free_tier: true,
            free_tier_limit: Some(10),
            avg_generation_time: 120.0,
            quality_rating: 5,
            speed_rating: 3,
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    //  COST CALCULATION
    // ═══════════════════════════════════════════════════════════════════════════

    /// Calculate cost for a generation
    pub fn calculate_cost(&self, duration_seconds: f32) -> f32 {
        self.cost_per_second * duration_seconds
    }

    /// Get all available models
    pub fn all() -> Vec<Self> {
        vec![
            // 2026 latest
            Self::sora2(),
            Self::runway_gen4(),
            Self::kling_v2_master(),
            Self::kling_v2(),
            Self::luma_ray2(),
            Self::luma_dream_machine_2(),
            Self::pika3(),
            Self::hailuo_02(),
            Self::veo2(),
            // 2025 established
            Self::runway_gen3_alpha(),
            Self::runway_gen3(),
            Self::runway_gen2(),
            Self::pika2(),
            Self::pika1_5(),
            Self::luma_dream_machine(),
            Self::luma_ray(),
            Self::kling_v1_5(),
            Self::kling_v1(),
            Self::haiper_v2(),
            Self::svd_xt(),
            Self::svd(),
            Self::sora(),
            Self::hailuo_01_live(),
        ]
    }

    /// Get free tier models
    pub fn free_tier_models() -> Vec<Self> {
        Self::all().into_iter().filter(|m| m.free_tier).collect()
    }

    /// Compare models by quality
    pub fn by_quality() -> Vec<Self> {
        let mut models = Self::all();
        models.sort_by(|a, b| b.quality_rating.cmp(&a.quality_rating));
        models
    }

    /// Compare models by speed
    pub fn by_speed() -> Vec<Self> {
        let mut models = Self::all();
        models.sort_by(|a, b| b.speed_rating.cmp(&a.speed_rating));
        models
    }

    /// Compare models by cost (cheapest first)
    pub fn by_cost() -> Vec<Self> {
        let mut models = Self::all();
        models.sort_by(|a, b| a.cost_per_second.partial_cmp(&b.cost_per_second).unwrap());
        models
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  ENUMS
// ═══════════════════════════════════════════════════════════════════════════════

/// Aspect ratio options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum AspectRatio {
    #[default]
    Landscape16x9,
    Landscape4x3,
    Portrait9x16,
    Portrait3x4,
    Square,
    UltraWide21x9,
    Cinematic2_39_1,
}

impl AspectRatio {
    pub fn dimensions(&self, base_height: u32) -> (u32, u32) {
        match self {
            AspectRatio::Landscape16x9 => (base_height * 16 / 9, base_height),
            AspectRatio::Landscape4x3 => (base_height * 4 / 3, base_height),
            AspectRatio::Portrait9x16 => (base_height * 9 / 16, base_height),
            AspectRatio::Portrait3x4 => (base_height * 3 / 4, base_height),
            AspectRatio::Square => (base_height, base_height),
            AspectRatio::UltraWide21x9 => (base_height * 21 / 9, base_height),
            AspectRatio::Cinematic2_39_1 => (base_height * 239 / 100, base_height),
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            AspectRatio::Landscape16x9 => "16:9 (YouTube, TV)",
            AspectRatio::Landscape4x3 => "4:3 (Traditional)",
            AspectRatio::Portrait9x16 => "9:16 (TikTok, Reels)",
            AspectRatio::Portrait3x4 => "3:4 (Instagram)",
            AspectRatio::Square => "1:1 (Square)",
            AspectRatio::UltraWide21x9 => "21:9 (Ultra-wide)",
            AspectRatio::Cinematic2_39_1 => "2.39:1 (Cinematic)",
        }
    }
}

/// Video resolution
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum VideoResolution {
    #[default]
    HD,      // 1280x720
    FullHD,  // 1920x1080
    QHD,     // 2560x1440
    FourK,   // 3840x2160
    Custom(u32, u32),
}

impl VideoResolution {
    pub fn dimensions(&self) -> (u32, u32) {
        match self {
            VideoResolution::HD => (1280, 720),
            VideoResolution::FullHD => (1920, 1080),
            VideoResolution::QHD => (2560, 1440),
            VideoResolution::FourK => (3840, 2160),
            VideoResolution::Custom(w, h) => (*w, *h),
        }
    }

    pub fn width(&self) -> u32 {
        self.dimensions().0
    }

    pub fn height(&self) -> u32 {
        self.dimensions().1
    }

    pub fn name(&self) -> String {
        match self {
            VideoResolution::HD => "720p HD".into(),
            VideoResolution::FullHD => "1080p Full HD".into(),
            VideoResolution::QHD => "1440p QHD".into(),
            VideoResolution::FourK => "4K UHD".into(),
            VideoResolution::Custom(w, h) => format!("{}x{}", w, h),
        }
    }
}

/// Video format
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VideoFormat {
    Mp4,
    WebM,
    Mov,
    Gif,
}

/// Video style presets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VideoStyle {
    Cinematic,
    Anime,
    ThreeDAnimation,
    Realistic,
    Cartoon,
    Watercolor,
    Sketch,
    Noir,
    Vintage,
    Cyberpunk,
    Fantasy,
    SciFi,
    Documentary,
    Commercial,
    MusicVideo,
    Animation,
    Artistic,
}

impl VideoStyle {
    pub fn prompt_suffix(&self) -> &'static str {
        match self {
            VideoStyle::Cinematic => "cinematic lighting, film grain, 35mm film",
            VideoStyle::Anime => "anime style, vibrant colors, Japanese animation",
            VideoStyle::ThreeDAnimation => "3D animation, Pixar style, smooth rendering",
            VideoStyle::Realistic => "photorealistic, high detail, natural lighting",
            VideoStyle::Cartoon => "cartoon style, bold outlines, colorful",
            VideoStyle::Watercolor => "watercolor painting style, soft edges, artistic",
            VideoStyle::Sketch => "pencil sketch style, hand drawn, artistic",
            VideoStyle::Noir => "film noir, black and white, dramatic shadows",
            VideoStyle::Vintage => "vintage film, 1970s aesthetic, warm colors",
            VideoStyle::Cyberpunk => "cyberpunk aesthetic, neon lights, futuristic",
            VideoStyle::Fantasy => "fantasy style, magical, ethereal lighting",
            VideoStyle::SciFi => "science fiction, futuristic, space age",
            VideoStyle::Documentary => "documentary style, natural, authentic",
            VideoStyle::Commercial => "commercial quality, polished, professional",
            VideoStyle::MusicVideo => "music video style, dynamic, energetic",
            VideoStyle::Animation => "animation style, smooth motion, animated",
            VideoStyle::Artistic => "artistic style, creative, expressive",
        }
    }
}

/// Camera motion options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CameraMotion {
    Static,
    PanLeft,
    PanRight,
    TiltUp,
    TiltDown,
    ZoomIn,
    ZoomOut,
    OrbitLeft,
    OrbitRight,
    DollyIn,
    DollyOut,
    CraneUp,
    CraneDown,
    Handheld,
    Drone,
}

impl CameraMotion {
    pub fn prompt_suffix(&self) -> &'static str {
        match self {
            CameraMotion::Static => "static camera, no movement",
            CameraMotion::PanLeft => "camera panning left",
            CameraMotion::PanRight => "camera panning right",
            CameraMotion::TiltUp => "camera tilting up",
            CameraMotion::TiltDown => "camera tilting down",
            CameraMotion::ZoomIn => "slow zoom in",
            CameraMotion::ZoomOut => "slow zoom out",
            CameraMotion::OrbitLeft => "camera orbiting left",
            CameraMotion::OrbitRight => "camera orbiting right",
            CameraMotion::DollyIn => "dolly shot moving forward",
            CameraMotion::DollyOut => "dolly shot moving backward",
            CameraMotion::CraneUp => "crane shot rising",
            CameraMotion::CraneDown => "crane shot descending",
            CameraMotion::Handheld => "handheld camera movement, natural shake",
            CameraMotion::Drone => "aerial drone shot, sweeping movement",
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  GENERATION PARAMS
// ═══════════════════════════════════════════════════════════════════════════════

/// Generation parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationParams {
    /// Guidance scale (how closely to follow prompt)
    #[serde(default = "default_guidance_scale")]
    pub guidance_scale: f32,

    /// FPS for output video
    #[serde(default = "default_fps")]
    pub fps: u8,

    /// Motion intensity (0-255 for SVD)
    #[serde(default = "default_motion_bucket")]
    pub motion_bucket_id: u8,

    /// Noise augmentation
    #[serde(default)]
    pub noise_augmentation: f32,

    /// Number of inference steps
    #[serde(default = "default_steps")]
    pub num_inference_steps: u32,

    /// CFG scale
    #[serde(default = "default_cfg_scale")]
    pub cfg_scale: f32,

    /// scheduler
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduler: Option<String>,
}

fn default_guidance_scale() -> f32 { 7.5 }
fn default_fps() -> u8 { 24 }
fn default_motion_bucket() -> u8 { 127 }
fn default_steps() -> u32 { 25 }
fn default_cfg_scale() -> f32 { 7.0 }

impl Default for GenerationParams {
    fn default() -> Self {
        Self {
            guidance_scale: default_guidance_scale(),
            fps: default_fps(),
            motion_bucket_id: default_motion_bucket(),
            noise_augmentation: 0.0,
            num_inference_steps: default_steps(),
            cfg_scale: default_cfg_scale(),
            scheduler: None,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  VIDEO BUILDER
// ═══════════════════════════════════════════════════════════════════════════════

/// Builder for video requests
pub struct VideoBuilder {
    request: VideoRequest,
}

impl VideoBuilder {
    pub fn new(prompt: impl Into<String>) -> Self {
        Self {
            request: VideoRequest::text_to_video(prompt),
        }
    }

    pub fn duration(mut self, seconds: f32) -> Self {
        self.request.duration = Some(seconds);
        self
    }

    pub fn aspect_ratio(mut self, ratio: AspectRatio) -> Self {
        self.request.aspect_ratio = ratio;
        self
    }

    pub fn resolution(mut self, resolution: VideoResolution) -> Self {
        self.request.resolution = resolution;
        self
    }

    pub fn model(mut self, model: impl Into<String>) -> Self {
        self.request.model = Some(model.into());
        self
    }

    pub fn seed(mut self, seed: i64) -> Self {
        self.request.seed = Some(seed);
        self
    }

    pub fn style(mut self, style: VideoStyle) -> Self {
        self.request.style = Some(style);
        self
    }

    pub fn camera_motion(mut self, motion: CameraMotion) -> Self {
        self.request.camera_motion = Some(motion);
        self
    }

    pub fn negative_prompt(mut self, prompt: impl Into<String>) -> Self {
        self.request.negative_prompt = Some(prompt.into());
        self
    }

    pub fn image_url(mut self, url: impl Into<String>) -> Self {
        self.request.image_url = Some(url.into());
        self
    }

    pub fn loop_video(mut self, should_loop: bool) -> Self {
        self.request.loop_video = should_loop;
        self
    }

    pub fn guidance_scale(mut self, scale: f32) -> Self {
        self.request.params.guidance_scale = scale;
        self
    }

    pub fn fps(mut self, fps: u8) -> Self {
        self.request.params.fps = fps;
        self
    }

    pub fn motion_intensity(mut self, intensity: u8) -> Self {
        self.request.params.motion_bucket_id = intensity;
        self
    }

    pub fn build(self) -> VideoRequest {
        self.request
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_to_video_request() {
        let req = VideoRequest::text_to_video("A cat playing piano");
        assert_eq!(req.prompt, "A cat playing piano");
        assert!(req.image_url.is_none());
        assert_eq!(req.generation_type(), GenerationType::TextToVideo);
    }

    #[test]
    fn test_image_to_video_request() {
        let req = VideoRequest::image_to_video("Animate", "https://example.com/img.jpg");
        assert_eq!(req.generation_type(), GenerationType::ImageToVideo);
    }

    #[test]
    fn test_video_builder() {
        let req = VideoBuilder::new("A sunset")
            .duration(5.0)
            .aspect_ratio(AspectRatio::Portrait9x16)
            .seed(42)
            .build();

        assert_eq!(req.prompt, "A sunset");
        assert_eq!(req.duration, Some(5.0));
        assert_eq!(req.seed, Some(42));
    }

    #[test]
    fn test_model_cost_calculation() {
        let model = VideoModel::runway_gen3();
        let cost = model.calculate_cost(5.0);
        assert!((cost - 0.50).abs() < 0.01); // $0.10/s * 5s = $0.50
    }

    #[test]
    fn test_all_models() {
        let models = VideoModel::all();
        assert!(!models.is_empty());
        assert!(models.iter().all(|m| m.free_tier || !m.free_tier));
    }

    #[test]
    fn test_free_tier_models() {
        let free = VideoModel::free_tier_models();
        assert!(free.iter().all(|m| m.free_tier));
    }

    #[test]
    fn test_aspect_ratio_dimensions() {
        let (w, h) = AspectRatio::Landscape16x9.dimensions(720);
        assert_eq!(w, 1280);
        assert_eq!(h, 720);
    }

    #[test]
    fn test_resolution_dimensions() {
        let (w, h) = VideoResolution::FullHD.dimensions();
        assert_eq!(w, 1920);
        assert_eq!(h, 1080);
    }

    #[test]
    fn test_style_prompt_suffix() {
        let suffix = VideoStyle::Cinematic.prompt_suffix();
        assert!(suffix.contains("cinematic"));
    }

    #[test]
    fn test_camera_motion_prompt_suffix() {
        let suffix = CameraMotion::Drone.prompt_suffix();
        assert!(suffix.contains("aerial"));
    }

    #[test]
    fn test_video_status_terminal() {
        assert!(VideoStatus::Completed.is_terminal());
        assert!(VideoStatus::Failed("error".into()).is_terminal());
        assert!(!VideoStatus::Processing.is_terminal());
    }

    #[test]
    fn test_models_by_quality() {
        let models = VideoModel::by_quality();
        assert!(models[0].quality_rating >= models[models.len()-1].quality_rating);
    }

    #[test]
    fn test_models_by_cost() {
        let models = VideoModel::by_cost();
        assert!(models[0].cost_per_second <= models[models.len()-1].cost_per_second);
    }
}
