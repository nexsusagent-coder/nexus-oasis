// ═══════════════════════════════════════════════════════════════════════════════
//  SENTIENT OS - Hailuo/MiniMax Provider
// ═══════════════════════════════════════════════════════════════════════════════
//  Hailuo AI (MiniMax) — High quality video generation
//  Website: https://hailuoai.com/
//  Free: 20 videos/day
// ═══════════════════════════════════════════════════════════════════════════════

use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use chrono::Utc;

use crate::{
    VideoRequest, VideoResponse, VideoJob, VideoResult,
    VideoStatus, VideoFormat, VideoResolution, VideoModel,
    VideoError, AspectRatio,
    providers::VideoProvider,
};

/// Hailuo AI (MiniMax) provider
pub struct HailuoProvider {
    api_key: String,
    base_url: String,
    http: Client,
}

impl HailuoProvider {
    pub fn new(api_key: impl Into<String>) -> Self {
        let http = Client::builder()
            .timeout(Duration::from_secs(300))
            .build()
            .expect("Failed to build HTTP client");

        Self {
            api_key: api_key.into(),
            base_url: "https://api.hailuo.ai/v1".to_string(),
            http,
        }
    }

    pub fn with_base_url(api_key: impl Into<String>, base_url: impl Into<String>) -> Self {
        let http = Client::builder()
            .timeout(Duration::from_secs(300))
            .build()
            .expect("Failed to build HTTP client");

        Self {
            api_key: api_key.into(),
            base_url: base_url.into(),
            http,
        }
    }
}

/// Hailuo API request
#[derive(Debug, Clone, Serialize)]
struct HailuoRequest {
    model: String,
    prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    seed: Option<u64>,
}

/// Hailuo API response
#[derive(Debug, Clone, Deserialize)]
struct HailuoResponse {
    id: Option<String>,
    video_url: Option<String>,
    status: Option<String>,
    error: Option<HailuoErrorBody>,
}

#[derive(Debug, Clone, Deserialize)]
struct HailuoErrorBody {
    message: String,
}

#[async_trait]
impl VideoProvider for HailuoProvider {
    async fn generate(&self, request: VideoRequest) -> VideoResult<VideoResponse> {
        let hailuo_request = HailuoRequest {
            model: "hailuo-01".to_string(),
            prompt: request.prompt.clone(),
            duration: request.duration,
            seed: request.seed.map(|s| s as u64),
        };

        let response = self.http
            .post(format!("{}/video/generations", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&hailuo_request)
            .send()
            .await
            .map_err(|e| VideoError::ApiError(e.to_string()))?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(match status.as_u16() {
                401 => VideoError::AuthenticationFailed(body),
                429 => VideoError::RateLimitExceeded,
                400 => VideoError::InvalidRequest(body),
                _ => VideoError::ApiError(format!("HTTP {}: {}", status, body)),
            });
        }

        let api_response: HailuoResponse = response.json().await
            .map_err(|e| VideoError::ApiError(e.to_string()))?;

        if let Some(url) = api_response.video_url {
            return Ok(VideoResponse {
                id: api_response.id.unwrap_or_default(),
                url,
                duration: request.duration.unwrap_or(6.0),
                format: VideoFormat::Mp4,
                resolution: VideoResolution::FullHD,
                created_at: Utc::now(),
                thumbnail_url: None,
                file_size: None,
            });
        }

        if let Some(job_id) = api_response.id {
            for _ in 0..60 {
                tokio::time::sleep(Duration::from_secs(5)).await;
                let job = self.status(&job_id).await?;
                if let Some(url) = job.url {
                    return Ok(VideoResponse {
                        id: job_id,
                        url,
                        duration: request.duration.unwrap_or(6.0),
                        format: VideoFormat::Mp4,
                        resolution: VideoResolution::FullHD,
                        created_at: Utc::now(),
                        thumbnail_url: None,
                        file_size: None,
                    });
                }
                if matches!(job.status, VideoStatus::Failed(_)) {
                    return Err(VideoError::GenerationFailed("Hailuo generation failed".to_string()));
                }
            }
            return Err(VideoError::Timeout);
        }

        Err(VideoError::NoVideoUrl)
    }

    async fn status(&self, job_id: &str) -> VideoResult<VideoJob> {
        let response = self.http
            .get(format!("{}/video/generations/{}", self.base_url, job_id))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await
            .map_err(|e| VideoError::ApiError(e.to_string()))?;

        let data: HailuoResponse = response.json().await
            .map_err(|e| VideoError::ApiError(e.to_string()))?;

        let status = match data.status.as_deref() {
            Some("completed" | "success") => VideoStatus::Completed,
            Some("failed" | "error") => VideoStatus::Failed("Hailuo error".to_string()),
            Some("cancelled") => VideoStatus::Cancelled,
            _ => VideoStatus::Processing,
        };

        Ok(VideoJob {
            id: job_id.to_string(),
            status,
            url: data.video_url,
            duration: None,
            resolution: None,
            progress: 0,
            created_at: Utc::now(),
            estimated_completion: None,
        })
    }

    async fn cancel(&self, _job_id: &str) -> VideoResult<()> {
        Ok(())
    }

    fn models(&self) -> Vec<VideoModel> {
        vec![
            VideoModel {
                id: "hailuo-01".to_string(),
                name: "Hailuo 01".to_string(),
                provider: "Hailuo AI".to_string(),
                text_to_video: true,
                image_to_video: true,
                max_duration: 6.0,
                aspect_ratios: vec![AspectRatio::Landscape16x9, AspectRatio::Portrait9x16, AspectRatio::Square],
                cost_per_second: 0.04,
                free_tier: true,
                free_tier_limit: Some(20),
                avg_generation_time: 90.0,
                quality_rating: 4,
                speed_rating: 3,
            },
            VideoModel {
                id: "hailuo-01-live".to_string(),
                name: "Hailuo 01 Live".to_string(),
                provider: "Hailuo AI".to_string(),
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
            },
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hailuo_provider_creation() {
        let provider = HailuoProvider::new("test-key");
        assert_eq!(provider.api_key, "test-key");
    }

    #[test]
    fn test_hailuo_models() {
        let provider = HailuoProvider::new("test-key");
        let models = provider.models();
        assert_eq!(models.len(), 2);
        assert!(models.iter().any(|m| m.id == "hailuo-01"));
    }
}
