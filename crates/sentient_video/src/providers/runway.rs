//! ─── Runway ML Provider ───
//!
//! Gen-2 and Gen-3 Alpha video generation
//! API Docs: https://docs.runwayml.com/

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
//  RUNWAY PROVIDER
// ═══════════════════════════════════════════════════════════════════════════════

pub struct RunwayProvider {
    client: Client,
    api_key: String,
    base_url: String,
}

impl RunwayProvider {
    pub fn new(api_key: String) -> Self {
        Self {
            client: build_client(),
            api_key,
            base_url: "https://api.runwayml.com/v1".into(),
        }
    }

    /// Set custom base URL
    pub fn with_base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into();
        self
    }

    /// Create a generation task
    async fn create_task(&self, request: VideoRequest) -> VideoResult<RunwayTask> {
        let runway_request = RunwayRequest::from(request);
        
        let response = self.client
            .post(format!("{}/generate", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&runway_request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(VideoError::ApiError(parse_api_error(response)));
        }

        response.json::<RunwayTaskResponse>().await
            .map(|r| RunwayTask { id: r.id })
            .map_err(Into::into)
    }

    /// Get task status
    async fn get_task(&self, task_id: &str) -> VideoResult<RunwayTaskStatus> {
        let response = self.client
            .get(format!("{}/generate/{}", self.base_url, task_id))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(VideoError::ApiError(parse_api_error(response)));
        }

        response.json::<RunwayTaskStatus>().await.map_err(Into::into)
    }
}

#[async_trait]
impl VideoProvider for RunwayProvider {
    async fn generate(&self, request: VideoRequest) -> VideoResult<VideoResponse> {
        // Create task
        let task = self.create_task(request.clone()).await?;
        
        // Poll for completion
        let max_attempts = 60; // 5 minutes at 5s intervals
        for _ in 0..max_attempts {
            let status = self.get_task(&task.id).await?;
            
            match status.status.as_str() {
                "SUCCEEDED" | "succeeded" => {
                    let url = status.output.first()
                        .ok_or(VideoError::NoVideoUrl)?;
                    
                    return Ok(VideoResponse {
                        id: task.id,
                        url: url.clone(),
                        duration: request.duration.unwrap_or(4.0),
                        format: crate::VideoFormat::Mp4,
                        resolution: request.resolution,
                        created_at: Utc::now(),
                        thumbnail_url: None,
                        file_size: None,
                    });
                }
                "FAILED" | "failed" => {
                    return Err(VideoError::GenerationFailed(
                        status.error.unwrap_or_else(|| "Unknown error".into())
                    ));
                }
                "CANCELLED" | "cancelled" => {
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
            "PENDING" | "pending" => VideoStatus::Pending,
            "RUNNING" | "running" | "THROTTLED" => VideoStatus::Processing,
            "SUCCEEDED" | "succeeded" => VideoStatus::Completed,
            "FAILED" | "failed" => VideoStatus::Failed(
                status.error.unwrap_or_else(|| "Unknown error".into())
            ),
            "CANCELLED" | "cancelled" => VideoStatus::Cancelled,
            _ => VideoStatus::Pending,
        };

        Ok(VideoJob {
            id: job_id.into(),
            status: video_status,
            url: status.output.first().cloned(),
            duration: None,
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
        vec![VideoModel::runway_gen2(), VideoModel::runway_gen3()]
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  RUNWAY API TYPES
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Serialize)]
struct RunwayRequest {
    model: String,
    prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resolution: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    seed: Option<i64>,
}

impl From<VideoRequest> for RunwayRequest {
    fn from(req: VideoRequest) -> Self {
        Self {
            model: req.model.unwrap_or_else(|| "gen2".into()),
            prompt: req.prompt,
            image: req.image_url,
            duration: req.duration,
            resolution: Some("720p".into()),
            seed: req.seed,
        }
    }
}

#[derive(Debug, Deserialize)]
struct RunwayTaskResponse {
    id: String,
}

#[derive(Debug)]
struct RunwayTask {
    id: String,
}

#[derive(Debug, Deserialize)]
struct RunwayTaskStatus {
    status: String,
    #[serde(default)]
    output: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    progress: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created_at: Option<chrono::DateTime<Utc>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runway_request_conversion() {
        let video_req = VideoRequest::text_to_video("A sunset");
        let runway_req: RunwayRequest = video_req.into();
        
        assert_eq!(runway_req.prompt, "A sunset");
        assert_eq!(runway_req.model, "gen2");
    }

    #[test]
    fn test_runway_models() {
        let provider = RunwayProvider::new("test-key".to_string());
        let models = provider.models();
        
        assert_eq!(models.len(), 2);
        assert_eq!(models[0].id, "gen2");
        assert_eq!(models[1].id, "gen3a_turbo");
    }
}
