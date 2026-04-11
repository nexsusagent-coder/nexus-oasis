//! ─── Pika Labs Provider ───
//!
//! Pika 1.0 video generation
//! Website: https://pika.art/

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
//  PIKA PROVIDER
// ═══════════════════════════════════════════════════════════════════════════════

pub struct PikaProvider {
    client: Client,
    api_key: String,
    base_url: String,
}

impl PikaProvider {
    pub fn new(api_key: String) -> Self {
        Self {
            client: build_client(),
            api_key,
            base_url: "https://api.pika.art/v1".into(),
        }
    }

    /// Set custom base URL
    pub fn with_base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into();
        self
    }

    /// Create a generation task
    async fn create_task(&self, request: VideoRequest) -> VideoResult<PikaTask> {
        let pika_request = PikaRequest::from(request);
        
        let response = self.client
            .post(format!("{}/generate", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&pika_request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(VideoError::ApiError(parse_api_error(response)));
        }

        response.json::<PikaTaskResponse>().await
            .map(|r| PikaTask { id: r.id })
            .map_err(Into::into)
    }

    /// Get task status
    async fn get_task(&self, task_id: &str) -> VideoResult<PikaTaskStatus> {
        let response = self.client
            .get(format!("{}/generate/{}", self.base_url, task_id))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(VideoError::ApiError(parse_api_error(response)));
        }

        response.json::<PikaTaskStatus>().await.map_err(Into::into)
    }
}

#[async_trait]
impl VideoProvider for PikaProvider {
    async fn generate(&self, request: VideoRequest) -> VideoResult<VideoResponse> {
        // Create task
        let task = self.create_task(request.clone()).await?;
        
        // Poll for completion
        let max_attempts = 60;
        for _ in 0..max_attempts {
            let status = self.get_task(&task.id).await?;
            
            match status.status.as_str() {
                "completed" | "succeeded" => {
                    let url = status.video_url
                        .ok_or(VideoError::NoVideoUrl)?;
                    
                    return Ok(VideoResponse {
                        id: task.id,
                        url,
                        duration: status.duration.unwrap_or(4.0),
                        format: crate::VideoFormat::Mp4,
                        resolution: VideoResolution::HD,
                        created_at: Utc::now(),
                        thumbnail_url: None,
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
            "pending" | "queued" => VideoStatus::Pending,
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
            url: status.video_url,
            duration: status.duration,
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
        vec![VideoModel::pika1()]
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  PIKA API TYPES
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Serialize)]
struct PikaRequest {
    prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    negative_prompt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    aspect_ratio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    seed: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    guidance_scale: Option<f32>,
}

impl From<VideoRequest> for PikaRequest {
    fn from(req: VideoRequest) -> Self {
        Self {
            prompt: req.prompt,
            negative_prompt: req.negative_prompt,
            image: req.image_url,
            aspect_ratio: Some("16:9".into()),
            seed: req.seed,
            guidance_scale: Some(req.params.guidance_scale),
        }
    }
}

#[derive(Debug, Deserialize)]
struct PikaTaskResponse {
    id: String,
}

#[derive(Debug)]
struct PikaTask {
    id: String,
}

#[derive(Debug, Deserialize)]
struct PikaTaskStatus {
    status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    video_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<f32>,
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
    fn test_pika_request_conversion() {
        let video_req = VideoRequest::text_to_video("A dancing robot");
        let pika_req: PikaRequest = video_req.into();
        
        assert_eq!(pika_req.prompt, "A dancing robot");
    }

    #[test]
    fn test_pika_models() {
        let provider = PikaProvider::new("test-key".to_string());
        let models = provider.models();
        
        assert_eq!(models.len(), 1);
        assert_eq!(models[0].id, "pika1.0");
    }
}
