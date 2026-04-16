// ═══════════════════════════════════════════════════════════════════════════════
//  SENTIENT OS - OpenAI Sora Provider
// ═══════════════════════════════════════════════════════════════════════════════
//  Sora — OpenAI's video generation model
//  API: https://api.openai.com/v1/video/generations
//  Models: sora-1.0-turbo, sora-1.0
//  Free: Limited (ChatGPT Pro includes some)
// ═══════════════════════════════════════════════════════════════════════════════

use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use chrono::Utc;

use crate::{
    VideoRequest, VideoResponse, VideoJob, VideoResult,
    VideoStatus, VideoFormat, VideoResolution, VideoModel,
    VideoError,
    providers::VideoProvider,
};

/// OpenAI Sora provider
pub struct SoraProvider {
    api_key: String,
    base_url: String,
    http: Client,
}

impl SoraProvider {
    pub fn new(api_key: impl Into<String>) -> Self {
        let http = Client::builder()
            .timeout(Duration::from_secs(300))
            .build()
            .expect("Failed to build HTTP client");

        Self {
            api_key: api_key.into(),
            base_url: "https://api.openai.com/v1".to_string(),
            http,
        }
    }

    /// Create with custom base URL (for V-GATE proxy)
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

/// Sora API request
#[derive(Debug, Clone, Serialize)]
struct SoraRequest {
    model: String,
    prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    seed: Option<u64>,
}

/// Sora API response
#[derive(Debug, Clone, Deserialize)]
struct SoraResponse {
    id: Option<String>,
    model: Option<String>,
    status: Option<String>,
    url: Option<String>,
    error: Option<SoraErrorBody>,
}

#[derive(Debug, Clone, Deserialize)]
struct SoraErrorBody {
    message: String,
    code: Option<String>,
}

#[async_trait]
impl VideoProvider for SoraProvider {
    async fn generate(&self, request: VideoRequest) -> VideoResult<VideoResponse> {
        let sora_request = SoraRequest {
            model: "sora-1.0-turbo".to_string(),
            prompt: request.prompt.clone(),
            size: None,
            duration: request.duration,
            seed: request.seed.map(|s| s as u64),
        };

        let response = self.http
            .post(format!("{}/video/generations", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&sora_request)
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

        let api_response: SoraResponse = response.json().await
            .map_err(|e| VideoError::ApiError(e.to_string()))?;

        // Poll for video URL
        if let Some(job_id) = api_response.id {
            for _ in 0..60 {
                tokio::time::sleep(Duration::from_secs(5)).await;

                let job = self.status(&job_id).await?;
                if let Some(url) = job.url {
                    return Ok(VideoResponse {
                        id: job_id,
                        url,
                        duration: request.duration.unwrap_or(5.0),
                        format: VideoFormat::Mp4,
                        resolution: VideoResolution::FullHD,
                        created_at: Utc::now(),
                        thumbnail_url: None,
                        file_size: None,
                    });
                }
                if matches!(job.status, VideoStatus::Failed(_)) {
                    return Err(VideoError::GenerationFailed("Sora generation failed".to_string()));
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

        let data: SoraResponse = response.json().await
            .map_err(|e| VideoError::ApiError(e.to_string()))?;

        let status = if data.url.is_some() {
            VideoStatus::Completed
        } else {
            VideoStatus::Processing
        };

        Ok(VideoJob {
            id: job_id.to_string(),
            status,
            url: data.url,
            duration: None,
            resolution: None,
            progress: 0,
            created_at: Utc::now(),
            estimated_completion: None,
        })
    }

    async fn cancel(&self, _job_id: &str) -> VideoResult<()> {
        // Sora doesn't have a cancel endpoint yet
        Ok(())
    }

    fn models(&self) -> Vec<VideoModel> {
        vec![VideoModel::sora()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sora_provider_creation() {
        let provider = SoraProvider::new("test-key");
        assert_eq!(provider.api_key, "test-key");
    }

    #[test]
    fn test_sora_with_base_url() {
        let provider = SoraProvider::with_base_url("key", "https://custom.api.com");
        assert_eq!(provider.base_url, "https://custom.api.com");
    }

    #[test]
    fn test_sora_models() {
        let provider = SoraProvider::new("test-key");
        let models = provider.models();
        assert_eq!(models.len(), 1);
        assert_eq!(models[0].id, "sora");
    }
}
