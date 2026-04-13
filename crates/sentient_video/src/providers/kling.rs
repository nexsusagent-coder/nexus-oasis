//! ─── Kling AI Provider ───
//!
//! Kling AI video generation (by Kuaishou)
//! Website: https://klingai.com/
//! API Docs: https://klingai.com/docs/api

use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use chrono::Utc;

use crate::{
    VideoRequest, VideoResponse, VideoJob, VideoModel, VideoResult,
    VideoError, VideoStatus, VideoResolution, AspectRatio,
};
use super::{VideoProvider, build_client, parse_api_error};

// ═══════════════════════════════════════════════════════════════════════════════
//  KLING PROVIDER
// ═══════════════════════════════════════════════════════════════════════════════

/// Kling AI video generation provider
/// 
/// # Pricing (2025)
/// - Free tier: 66 daily credits (~22 videos/day)
/// - Standard: $10/month (300 credits)
/// - Pro: $30/month (1050 credits)
/// - Premium: $95/month (3600 credits)
/// 
/// # Features
/// - Text-to-video and image-to-video
/// - Up to 10 seconds per video
/// - Extremely high quality (Sora competitor)
/// - Slower generation (2-3 minutes)
/// - Chinese AI, strong realistic output
/// 
/// # Example
/// ```ignore
/// use sentient_video::{VideoClient, VideoRequest};
/// 
/// let client = VideoClient::kling("your-api-key");
/// let request = VideoRequest::text_to_video("A woman walking in Tokyo streets");
/// let video = client.generate(request).await?;
/// ```
pub struct KlingProvider {
    client: Client,
    api_key: String,
    base_url: String,
}

impl KlingProvider {
    /// Create new Kling provider
    pub fn new(api_key: String) -> Self {
        Self {
            client: build_client(),
            api_key,
            base_url: "https://api.klingai.com/v1".into(),
        }
    }

    /// Set custom base URL
    pub fn with_base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into();
        self
    }

    /// Create text-to-video generation
    async fn create_text_to_video(&self, request: VideoRequest) -> VideoResult<KlingTask> {
        let kling_request = KlingTextRequest::from(request);
        
        let response = self.client
            .post(format!("{}/videos/text2video", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&kling_request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(VideoError::ApiError(parse_api_error(response)));
        }

        let result = response.json::<KlingResponse>().await?;
        Ok(KlingTask { 
            id: result.data.task_id,
            task_status: "pending".into(),
            task_result: KlingTaskResult::default(),
        })
    }

    /// Create image-to-video generation
    async fn create_image_to_video(&self, request: VideoRequest) -> VideoResult<KlingTask> {
        let image_url = request.image_url.as_ref()
            .ok_or_else(|| VideoError::InvalidRequest("Image URL required".into()))?;

        let kling_request = KlingImageRequest {
            image: image_url.clone(),
            prompt: request.prompt.clone(),
            negative_prompt: request.negative_prompt.clone(),
            duration: request.duration.unwrap_or(5.0) as f32,
            cfg_scale: request.params.cfg_scale,
            mode: "std".into(), // std or pro
            aspect_ratio: self.aspect_ratio_to_kling(&request.aspect_ratio),
            seed: request.seed,
        };
        
        let response = self.client
            .post(format!("{}/videos/image2video", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&kling_request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(VideoError::ApiError(parse_api_error(response)));
        }

        let result = response.json::<KlingResponse>().await?;
        Ok(KlingTask { 
            id: result.data.task_id,
            task_status: "pending".into(),
            task_result: KlingTaskResult::default(),
        })
    }

    /// Get task status
    async fn get_task(&self, task_id: &str) -> VideoResult<KlingTask> {
        let response = self.client
            .get(format!("{}/videos/text2video/{}", self.base_url, task_id))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(VideoError::ApiError(parse_api_error(response)));
        }

        let result = response.json::<KlingTaskResponse>().await?;
        Ok(result.data)
    }

    /// Convert aspect ratio to Kling format
    fn aspect_ratio_to_kling(&self, ratio: &AspectRatio) -> String {
        match ratio {
            AspectRatio::Landscape16x9 => "16:9",
            AspectRatio::Portrait9x16 => "9:16",
            AspectRatio::Square => "1:1",
            _ => "16:9",
        }.into()
    }
}

#[async_trait]
impl VideoProvider for KlingProvider {
    async fn generate(&self, request: VideoRequest) -> VideoResult<VideoResponse> {
        // Create task based on generation type
        let task = if request.image_url.is_some() {
            self.create_image_to_video(request.clone()).await?
        } else {
            self.create_text_to_video(request.clone()).await?
        };
        
        // Poll for completion
        let max_attempts = 180; // 15 minutes at 5s intervals (Kling is slower)
        for _ in 0..max_attempts {
            let status = self.get_task(&task.id).await?;
            
            match status.task_status.as_str() {
                "succeed" => {
                    let video = status.task_result.videos.first()
                        .ok_or(VideoError::NoVideoUrl)?;
                    
                    return Ok(VideoResponse {
                        id: task.id,
                        url: video.url.clone(),
                        duration: video.duration,
                        format: crate::VideoFormat::Mp4,
                        resolution: VideoResolution::HD,
                        created_at: Utc::now(),
                        thumbnail_url: None,
                        file_size: None,
                    });
                }
                "failed" => {
                    return Err(VideoError::GenerationFailed(
                        status.task_result.fail_reason.unwrap_or_else(|| "Unknown error".into())
                    ));
                }
                _ => {
                    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                }
            }
        }

        Err(VideoError::Timeout)
    }

    async fn status(&self, job_id: &str) -> VideoResult<VideoJob> {
        let task = self.get_task(job_id).await?;
        
        let video_status = match task.task_status.as_str() {
            "pending" | "submitted" => VideoStatus::Queued,
            "processing" | "running" => VideoStatus::Processing,
            "succeed" => VideoStatus::Completed,
            "failed" => VideoStatus::Failed(
                task.task_result.fail_reason.unwrap_or_else(|| "Unknown error".into())
            ),
            _ => VideoStatus::Pending,
        };

        let video_url = task.task_result.videos.first().map(|v| v.url.clone());

        Ok(VideoJob {
            id: job_id.into(),
            status: video_status,
            url: video_url,
            duration: task.task_result.videos.first().map(|v| v.duration),
            resolution: Some(VideoResolution::HD),
            progress: 0,
            created_at: Utc::now(),
            estimated_completion: None,
        })
    }

    async fn cancel(&self, _job_id: &str) -> VideoResult<()> {
        // Kling doesn't support cancellation
        Err(VideoError::InvalidRequest("Cancellation not supported by Kling".into()))
    }

    fn models(&self) -> Vec<VideoModel> {
        vec![VideoModel::kling_v1(), VideoModel::kling_v1_5()]
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  KLING API TYPES
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Serialize)]
struct KlingTextRequest {
    prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    negative_prompt: Option<String>,
    duration: f32,
    cfg_scale: f32,
    mode: String,
    aspect_ratio: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    seed: Option<i64>,
}

impl From<VideoRequest> for KlingTextRequest {
    fn from(req: VideoRequest) -> Self {
        Self {
            prompt: req.prompt,
            negative_prompt: req.negative_prompt,
            duration: req.duration.unwrap_or(5.0),
            cfg_scale: req.params.cfg_scale,
            mode: "std".into(),
            aspect_ratio: "16:9".into(),
            seed: req.seed,
        }
    }
}

#[derive(Debug, Serialize)]
struct KlingImageRequest {
    image: String,
    prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    negative_prompt: Option<String>,
    duration: f32,
    cfg_scale: f32,
    mode: String,
    aspect_ratio: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    seed: Option<i64>,
}

#[derive(Debug, Deserialize)]
struct KlingResponse {
    data: KlingResponseData,
}

#[derive(Debug, Deserialize)]
struct KlingResponseData {
    task_id: String,
}

#[derive(Debug, Deserialize)]
struct KlingTaskResponse {
    data: KlingTask,
}

#[derive(Debug, Deserialize)]
struct KlingTask {
    id: String,
    task_status: String,
    #[serde(default)]
    task_result: KlingTaskResult,
}

impl KlingTask {
    #[allow(dead_code)]
    fn id(&self) -> &str {
        &self.id
    }
}

#[derive(Debug, Deserialize, Default)]
struct KlingTaskResult {
    #[serde(default)]
    videos: Vec<KlingVideo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fail_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct KlingVideo {
    url: String,
    duration: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kling_text_request() {
        let video_req = VideoRequest::text_to_video("A robot dancing");
        let kling_req: KlingTextRequest = video_req.into();
        
        assert_eq!(kling_req.prompt, "A robot dancing");
    }

    #[test]
    fn test_kling_models() {
        let provider = KlingProvider::new("test-key".into());
        let models = provider.models();
        
        assert_eq!(models.len(), 2);
    }

    #[test]
    fn test_kling_free_tier() {
        let model = VideoModel::kling_v1_5();
        assert!(model.free_tier);
        assert_eq!(model.free_tier_limit, Some(66));
    }

    #[test]
    fn test_kling_max_duration() {
        let model = VideoModel::kling_v1_5();
        assert_eq!(model.max_duration, 10.0);
    }
}
