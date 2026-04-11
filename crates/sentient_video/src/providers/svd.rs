//! ─── Stable Video Diffusion Provider ───
//!
//! Stability AI's SVD model for image-to-video
//! Works via Replicate or Stability API
//! API Docs: https://platform.stability.ai/

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
//  SVD PROVIDER
// ═══════════════════════════════════════════════════════════════════════════════

pub struct SVDProvider {
    client: Client,
    api_key: String,
    base_url: String,
}

impl SVDProvider {
    pub fn new(api_key: String) -> Self {
        Self {
            client: build_client(),
            api_key,
            base_url: "https://api.stability.ai/v2beta".into(),
        }
    }

    /// Set custom base URL (e.g., for Replicate)
    pub fn with_base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into();
        self
    }

    /// Generate video from image
    async fn generate_from_image(&self, request: VideoRequest) -> VideoResult<VideoResponse> {
        let image_url = request.image_url.as_ref()
            .ok_or_else(|| VideoError::InvalidRequest("Image URL required for SVD".into()))?;

        let svd_request = SVDRequest::from(request.clone());
        
        let response = self.client
            .post(format!("{}/image-to-video", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&svd_request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(VideoError::ApiError(parse_api_error(response)));
        }

        let result = response.json::<SVDGenerateResponse>().await?;
        
        // Poll for completion
        let max_attempts = 60;
        for _ in 0..max_attempts {
            let status = self.get_result(&result.id).await?;
            
            match status.status.as_str() {
                "complete" | "succeeded" => {
                    let video = status.video
                        .ok_or(VideoError::NoVideoUrl)?;
                    
                    return Ok(VideoResponse {
                        id: result.id,
                        url: video,
                        duration: 4.0, // SVD generates 4 second videos
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
                _ => {
                    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                }
            }
        }

        Err(VideoError::Timeout)
    }

    /// Get generation result
    async fn get_result(&self, generation_id: &str) -> VideoResult<SVDStatusResponse> {
        let response = self.client
            .get(format!("{}/image-to-video/result/{}", self.base_url, generation_id))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(VideoError::ApiError(parse_api_error(response)));
        }

        response.json::<SVDStatusResponse>().await.map_err(Into::into)
    }
}

#[async_trait]
impl VideoProvider for SVDProvider {
    async fn generate(&self, request: VideoRequest) -> VideoResult<VideoResponse> {
        // SVD is image-to-video only
        if request.image_url.is_none() {
            return Err(VideoError::InvalidRequest(
                "Stable Video Diffusion requires an input image. Use image_to_video() instead.".into()
            ));
        }

        self.generate_from_image(request).await
    }

    async fn status(&self, job_id: &str) -> VideoResult<VideoJob> {
        let status = self.get_result(job_id).await?;
        
        let video_status = match status.status.as_str() {
            "in-progress" | "processing" => VideoStatus::Processing,
            "complete" | "succeeded" => VideoStatus::Completed,
            "failed" => VideoStatus::Failed(
                status.error.unwrap_or_else(|| "Unknown error".into())
            ),
            "cancelled" => VideoStatus::Cancelled,
            _ => VideoStatus::Pending,
        };

        Ok(VideoJob {
            id: job_id.into(),
            status: video_status,
            url: status.video,
            duration: Some(4.0),
            resolution: Some(VideoResolution::HD),
            progress: 0,
            created_at: Utc::now(),
            estimated_completion: None,
        })
    }

    async fn cancel(&self, _job_id: &str) -> VideoResult<()> {
        // Stability API doesn't support cancellation
        Err(VideoError::InvalidRequest("Cancellation not supported by SVD".into()))
    }

    fn models(&self) -> Vec<VideoModel> {
        vec![VideoModel::svd()]
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  SVD API TYPES
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Serialize)]
struct SVDRequest {
    /// Image URL or base64
    image: String,
    /// Motion bucket ID (1-255)
    #[serde(default = "default_motion_bucket")]
    motion_bucket_id: u8,
    /// Condensation augmentation (0-1)
    #[serde(default)]
    condensation_augmentation: f32,
    /// Seed for reproducibility
    #[serde(skip_serializing_if = "Option::is_none")]
    seed: Option<i64>,
}

fn default_motion_bucket() -> u8 { 127 }

impl From<VideoRequest> for SVDRequest {
    fn from(req: VideoRequest) -> Self {
        let image = req.image_url.unwrap_or_default();
        Self {
            image,
            motion_bucket_id: req.params.motion_bucket_id,
            condensation_augmentation: req.params.noise_augmentation,
            seed: req.seed,
        }
    }
}

#[derive(Debug, Deserialize)]
struct SVDGenerateResponse {
    id: String,
}

#[derive(Debug, Deserialize)]
struct SVDStatusResponse {
    status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    video: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  REPLICATE SVD (Alternative)
// ═══════════════════════════════════════════════════════════════════════════════

impl SVDProvider {
    /// Create provider for Replicate's SVD endpoint
    pub fn replicate(api_key: String) -> Self {
        Self {
            client: build_client(),
            api_key,
            base_url: "https://api.replicate.com/v1".into(),
        }
    }

    /// Generate via Replicate API
    pub async fn generate_replicate(&self, request: VideoRequest) -> VideoResult<VideoResponse> {
        let image_url = request.image_url.as_ref()
            .ok_or_else(|| VideoError::InvalidRequest("Image URL required".into()))?;

        let rep_request = ReplicateRequest {
            version: "stability-ai/stable-video-diffusion:3f0457e4619daac51203dedb472816fd4af51f3149fa7a9e3b4c9eead3c7d15".into(),
            input: ReplicateInput {
                input_image: image_url.clone(),
                motion_bucket_id: request.params.motion_bucket_id as i32,
                fps: request.params.fps as i32,
                condensation_augmentation: request.params.noise_augmentation,
            },
        };

        let response = self.client
            .post(format!("{}/predictions", self.base_url))
            .header("Authorization", format!("Token {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&rep_request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(VideoError::ApiError(parse_api_error(response)));
        }

        let result = response.json::<ReplicateResponse>().await?;
        
        // Poll for completion
        let max_attempts = 60;
        for _ in 0..max_attempts {
            let status = self.get_replicate_result(&result.id).await?;
            
            match status.status.as_str() {
                "succeeded" => {
                    let output = status.output.first()
                        .ok_or(VideoError::NoVideoUrl)?;
                    
                    return Ok(VideoResponse {
                        id: result.id,
                        url: output.clone(),
                        duration: 4.0,
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
                "canceled" => {
                    return Err(VideoError::GenerationFailed("Cancelled".into()));
                }
                _ => {
                    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                }
            }
        }

        Err(VideoError::Timeout)
    }

    async fn get_replicate_result(&self, prediction_id: &str) -> VideoResult<ReplicateResponse> {
        let response = self.client
            .get(format!("{}/predictions/{}", self.base_url, prediction_id))
            .header("Authorization", format!("Token {}", self.api_key))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(VideoError::ApiError(parse_api_error(response)));
        }

        response.json::<ReplicateResponse>().await.map_err(Into::into)
    }
}

#[derive(Debug, Serialize)]
struct ReplicateRequest {
    version: String,
    input: ReplicateInput,
}

#[derive(Debug, Serialize)]
struct ReplicateInput {
    input_image: String,
    motion_bucket_id: i32,
    fps: i32,
    condensation_augmentation: f32,
}

#[derive(Debug, Deserialize)]
struct ReplicateResponse {
    id: String,
    status: String,
    #[serde(default)]
    output: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_svd_requires_image() {
        let provider = SVDProvider::new("test-key".into());
        let models = provider.models();
        
        assert_eq!(models.len(), 1);
        assert!(!models[0].text_to_video); // SVD is image-to-video only
        assert!(models[0].image_to_video);
    }

    #[test]
    fn test_svd_request_conversion() {
        let video_req = VideoRequest::image_to_video(
            "Animate",
            "https://example.com/image.jpg"
        );
        let svd_req: SVDRequest = video_req.into();
        
        assert_eq!(svd_req.image, "https://example.com/image.jpg");
    }
}
