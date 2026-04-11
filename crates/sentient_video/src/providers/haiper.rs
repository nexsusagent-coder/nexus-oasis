//! ─── Haiper AI Provider ───
//!
//! Haiper AI video generation
//! Website: https://haiper.ai/
//! API Docs: https://docs.haiper.ai/

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
//  HAIPER PROVIDER
// ═══════════════════════════════════════════════════════════════════════════════

/// Haiper AI video generation provider
/// 
/// # Pricing (2025)
/// - Free tier: 150 credits/month (~50 videos)
/// - Creator: $15/month (500 credits)
/// - Pro: $50/month (2000 credits)
/// - Enterprise: Custom
/// 
/// # Features
/// - Text-to-video and image-to-video
/// - Up to 6 seconds per video
/// - Good quality, fast generation
/// - Strong motion understanding
/// 
/// # Example
/// ```ignore
/// use sentient_video::{VideoClient, VideoRequest};
/// 
/// let client = VideoClient::haiper("your-api-key");
/// let request = VideoRequest::text_to_video("A car driving through a city");
/// let video = client.generate(request).await?;
/// ```
pub struct HaiperProvider {
    client: Client,
    api_key: String,
    base_url: String,
}

impl HaiperProvider {
    /// Create new Haiper provider
    pub fn new(api_key: String) -> Self {
        Self {
            client: build_client(),
            api_key,
            base_url: "https://api.haiper.ai/v1".into(),
        }
    }

    /// Set custom base URL
    pub fn with_base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into();
        self
    }

    /// Create a generation task
    async fn create_task(&self, request: VideoRequest) -> VideoResult<HaiperTask> {
        let haiper_request = HaiperRequest::from(request);
        
        let response = self.client
            .post(format!("{}/generate", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&haiper_request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(VideoError::ApiError(parse_api_error(response)));
        }

        response.json::<HaiperTaskResponse>().await
            .map(|r| HaiperTask { id: r.id })
            .map_err(Into::into)
    }

    /// Get task status
    async fn get_task(&self, task_id: &str) -> VideoResult<HaiperTaskStatus> {
        let response = self.client
            .get(format!("{}/generate/{}", self.base_url, task_id))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(VideoError::ApiError(parse_api_error(response)));
        }

        response.json::<HaiperTaskStatus>().await.map_err(Into::into)
    }
}

#[async_trait]
impl VideoProvider for HaiperProvider {
    async fn generate(&self, request: VideoRequest) -> VideoResult<VideoResponse> {
        // Create task
        let task = self.create_task(request.clone()).await?;
        
        // Poll for completion
        let max_attempts = 90; // 7.5 minutes at 5s intervals
        for _ in 0..max_attempts {
            let status = self.get_task(&task.id).await?;
            
            match status.status.as_str() {
                "completed" | "succeeded" => {
                    let url = status.output.url
                        .ok_or(VideoError::NoVideoUrl)?;
                    
                    return Ok(VideoResponse {
                        id: task.id,
                        url,
                        duration: status.output.duration.unwrap_or(4.0),
                        format: crate::VideoFormat::Mp4,
                        resolution: VideoResolution::HD,
                        created_at: Utc::now(),
                        thumbnail_url: status.output.thumbnail,
                        file_size: None,
                    });
                }
                "failed" => {
                    return Err(VideoError::GenerationFailed(
                        status.error.unwrap_or_else(|| "Unknown error".into())
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
        let status = self.get_task(job_id).await?;
        
        let video_status = match status.status.as_str() {
            "pending" | "queued" => VideoStatus::Queued,
            "processing" | "generating" => VideoStatus::Processing,
            "completed" | "succeeded" => VideoStatus::Completed,
            "failed" => VideoStatus::Failed(
                status.error.unwrap_or_else(|| "Unknown error".into())
            ),
            "cancelled" => VideoStatus::Cancelled,
            _ => VideoStatus::Pending,
        };

        Ok(VideoJob {
            id: job_id.into(),
            status: video_status,
            url: status.output.url,
            duration: status.output.duration,
            resolution: Some(VideoResolution::HD),
            progress: status.progress.unwrap_or(0),
            created_at: status.created_at.unwrap_or_else(Utc::now),
            estimated_completion: None,
        })
    }

    async fn cancel(&self, job_id: &str) -> VideoResult<()> {
        let response = self.client
            .post(format!("{}/generate/{}/cancel", self.base_url, job_id))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(VideoError::ApiError(parse_api_error(response)));
        }

        Ok(())
    }

    fn models(&self) -> Vec<VideoModel> {
        vec![VideoModel::haiper_v2()]
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  HAIPER API TYPES
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Serialize)]
struct HaiperRequest {
    prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    negative_prompt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    aspect_ratio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    seed: Option<i64>,
}

impl From<VideoRequest> for HaiperRequest {
    fn from(req: VideoRequest) -> Self {
        Self {
            prompt: req.prompt,
            negative_prompt: req.negative_prompt,
            image: req.image_url,
            duration: req.duration,
            aspect_ratio: Some("16:9".into()),
            seed: req.seed,
        }
    }
}

#[derive(Debug, Deserialize)]
struct HaiperTaskResponse {
    id: String,
}

#[derive(Debug)]
struct HaiperTask {
    id: String,
}

#[derive(Debug, Deserialize)]
struct HaiperTaskStatus {
    status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    progress: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created_at: Option<chrono::DateTime<Utc>>,
    #[serde(default)]
    output: HaiperOutput,
}

#[derive(Debug, Deserialize, Default)]
struct HaiperOutput {
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thumbnail: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_haiper_request_conversion() {
        let video_req = VideoRequest::text_to_video("A space shuttle launching");
        let haiper_req: HaiperRequest = video_req.into();
        
        assert_eq!(haiper_req.prompt, "A space shuttle launching");
    }

    #[test]
    fn test_haiper_models() {
        let provider = HaiperProvider::new("test-key".into());
        let models = provider.models();
        
        assert_eq!(models.len(), 1);
    }

    #[test]
    fn test_haiper_free_tier() {
        let model = VideoModel::haiper_v2();
        assert!(model.free_tier);
        assert_eq!(model.free_tier_limit, Some(150));
    }
}
