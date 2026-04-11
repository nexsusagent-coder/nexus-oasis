//! ─── Luma AI Provider ───
//!
//! Luma Dream Machine video generation
//! Website: https://lumalabs.ai/
//! API Docs: https://lumalabs.ai/api

use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use chrono::Utc;

use crate::{
    VideoRequest, VideoResponse, VideoJob, VideoModel, VideoResult,
    VideoError, VideoStatus, VideoResolution,
};
use super::{VideoProvider, build_client, parse_api_error};

// ═══════════════════════════════════════════════════════════════════════════════
//  LUMA PROVIDER
// ═══════════════════════════════════════════════════════════════════════════════

/// Luma AI video generation provider
/// 
/// # Pricing (2025)
/// - Free tier: 30 videos/month
/// - Pro: $29/month (500 videos)
/// - Premier: $99/month (2,000 videos)
/// - Pay-as-you-go: $0.04/second
/// 
/// # Features
/// - Text-to-video and image-to-video
/// - Up to 5 seconds per video
/// - High quality output (near Sora quality)
/// - Fast generation (~2 minutes)
/// 
/// # Example
/// ```ignore
/// use sentient_video::{VideoClient, VideoRequest};
/// 
/// let client = VideoClient::luma("your-api-key");
/// let request = VideoRequest::text_to_video("A dragon flying over mountains");
/// let video = client.generate(request).await?;
/// ```
pub struct LumaProvider {
    client: Client,
    api_key: String,
    base_url: String,
}

impl LumaProvider {
    /// Create new Luma provider
    pub fn new(api_key: String) -> Self {
        Self {
            client: build_client(),
            api_key,
            base_url: "https://api.lumalabs.ai/dream-machine/v1".into(),
        }
    }

    /// Set custom base URL
    pub fn with_base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into();
        self
    }

    /// Create a generation task
    async fn create_generation(&self, request: VideoRequest) -> VideoResult<LumaGeneration> {
        let luma_request = LumaRequest::from(request);
        
        let response = self.client
            .post(format!("{}/generations", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&luma_request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(VideoError::ApiError(error_text));
        }

        response.json::<LumaGeneration>().await.map_err(Into::into)
    }

    /// Get generation status
    async fn get_generation(&self, generation_id: &str) -> VideoResult<LumaGeneration> {
        let response = self.client
            .get(format!("{}/generations/{}", self.base_url, generation_id))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(VideoError::ApiError(error_text));
        }

        response.json::<LumaGeneration>().await.map_err(Into::into)
    }
}

#[async_trait]
impl VideoProvider for LumaProvider {
    async fn generate(&self, request: VideoRequest) -> VideoResult<VideoResponse> {
        // Create generation
        let generation = self.create_generation(request.clone()).await?;
        
        // Poll for completion
        let max_attempts = 120; // 10 minutes at 5s intervals
        for _ in 0..max_attempts {
            let status = self.get_generation(&generation.id).await?;
            
            match status.state.as_str() {
                "completed" => {
                    let video = status.assets.video
                        .ok_or(VideoError::NoVideoUrl)?;
                    
                    return Ok(VideoResponse {
                        id: status.id,
                        url: video,
                        duration: 5.0, // Luma generates 5s videos
                        format: crate::VideoFormat::Mp4,
                        resolution: VideoResolution::HD,
                        created_at: Utc::now(),
                        thumbnail_url: status.assets.thumbnail,
                        file_size: None,
                    });
                }
                "failed" => {
                    return Err(VideoError::GenerationFailed(
                        status.failure_reason.unwrap_or_else(|| "Unknown error".into())
                    ));
                }
                "cancelled" => {
                    return Err(VideoError::GenerationFailed("Cancelled".into()));
                }
                _ => {
                    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                }
            }
        }

        Err(VideoError::Timeout)
    }

    async fn status(&self, job_id: &str) -> VideoResult<VideoJob> {
        let generation = self.get_generation(job_id).await?;
        
        let video_status = match generation.state.as_str() {
            "pending" | "queued" => VideoStatus::Queued,
            "dreaming" | "processing" => VideoStatus::Processing,
            "completed" => VideoStatus::Completed,
            "failed" => VideoStatus::Failed(
                generation.failure_reason.unwrap_or_else(|| "Unknown error".into())
            ),
            "cancelled" => VideoStatus::Cancelled,
            _ => VideoStatus::Pending,
        };

        Ok(VideoJob {
            id: job_id.into(),
            status: video_status,
            url: generation.assets.video,
            duration: Some(5.0),
            resolution: Some(VideoResolution::HD),
            progress: 0,
            created_at: generation.created_at.unwrap_or_else(Utc::now),
            estimated_completion: None,
        })
    }

    async fn cancel(&self, job_id: &str) -> VideoResult<()> {
        let response = self.client
            .post(format!("{}/generations/{}/cancel", self.base_url, job_id))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(VideoError::ApiError(parse_api_error(response)));
        }

        Ok(())
    }

    fn models(&self) -> Vec<VideoModel> {
        vec![VideoModel::luma_dream_machine(), VideoModel::luma_ray()]
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  LUMA API TYPES
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Serialize)]
struct LumaRequest {
    /// Text prompt
    prompt: String,
    /// Image URL for image-to-video
    #[serde(skip_serializing_if = "Option::is_none")]
    image_url: Option<String>,
    /// Aspect ratio
    #[serde(skip_serializing_if = "Option::is_none")]
    aspect_ratio: Option<String>,
    /// Loop video
    #[serde(skip_serializing_if = "Option::is_none")]
    loop_: Option<bool>,
    /// Seed
    #[serde(skip_serializing_if = "Option::is_none")]
    seed: Option<i64>,
}

impl From<VideoRequest> for LumaRequest {
    fn from(req: VideoRequest) -> Self {
        Self {
            prompt: req.prompt,
            image_url: req.image_url,
            aspect_ratio: Some("16:9".into()),
            loop_: if req.loop_video { Some(true) } else { None },
            seed: req.seed,
        }
    }
}

#[derive(Debug, Deserialize)]
struct LumaGeneration {
    id: String,
    state: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    failure_reason: Option<String>,
    assets: LumaAssets,
    #[serde(skip_serializing_if = "Option::is_none")]
    created_at: Option<chrono::DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
struct LumaAssets {
    #[serde(skip_serializing_if = "Option::is_none")]
    video: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thumbnail: Option<String>,
}

impl Default for LumaAssets {
    fn default() -> Self {
        Self {
            video: None,
            thumbnail: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_luma_request_conversion() {
        let video_req = VideoRequest::text_to_video("A dragon flying");
        let luma_req: LumaRequest = video_req.into();
        
        assert_eq!(luma_req.prompt, "A dragon flying");
    }

    #[test]
    fn test_luma_models() {
        let provider = LumaProvider::new("test-key".into());
        let models = provider.models();
        
        assert_eq!(models.len(), 2);
    }

    #[test]
    fn test_luma_free_tier() {
        let model = VideoModel::luma_dream_machine();
        assert!(model.free_tier);
        assert_eq!(model.free_tier_limit, Some(30));
    }
}
