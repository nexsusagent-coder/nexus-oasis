//! ─── Error Module ───

use serde::{Deserialize, Serialize};
use thiserror::Error;

// ═══════════════════════════════════════════════════════════════════════════════
//  VIDEO ERROR
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Error, Clone, Serialize, Deserialize)]
pub enum VideoError {
    #[error("API error: {0}")]
    ApiError(String),

    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),

    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    #[error("Insufficient credits")]
    InsufficientCredits,

    #[error("Generation failed: {0}")]
    GenerationFailed(String),

    #[error("Job not found: {0}")]
    JobNotFound(String),

    #[error("Timeout waiting for video")]
    Timeout,

    #[error("No video URL in response")]
    NoVideoUrl,

    #[error("Invalid image URL: {0}")]
    InvalidImageUrl(String),

    #[error("Invalid prompt: {0}")]
    InvalidPrompt(String),

    #[error("Duration exceeds maximum: {0}s > {1}s")]
    DurationExceedsMax(f32, f32),

    #[error("Provider not available: {0}")]
    ProviderNotAvailable(String),

    #[error("IO error: {0}")]
    IoError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Internal error: {0}")]
    InternalError(String),
}

// ═══════════════════════════════════════════════════════════════════════════════
//  RESULT TYPE
// ═══════════════════════════════════════════════════════════════════════════════

pub type VideoResult<T> = Result<T, VideoError>;

// ═══════════════════════════════════════════════════════════════════════════════
//  IMPLEMENTATIONS
// ═══════════════════════════════════════════════════════════════════════════════

impl From<serde_json::Error> for VideoError {
    fn from(e: serde_json::Error) -> Self {
        VideoError::SerializationError(e.to_string())
    }
}

impl From<std::io::Error> for VideoError {
    fn from(e: std::io::Error) -> Self {
        VideoError::IoError(e.to_string())
    }
}

impl From<reqwest::Error> for VideoError {
    fn from(e: reqwest::Error) -> Self {
        if e.is_timeout() {
            VideoError::Timeout
        } else if e.is_connect() {
            VideoError::NetworkError(e.to_string())
        } else {
            VideoError::ApiError(e.to_string())
        }
    }
}

impl VideoError {
    /// Check if error is retryable
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            VideoError::RateLimitExceeded |
            VideoError::Timeout |
            VideoError::NetworkError(_)
        )
    }

    /// Get error code
    pub fn error_code(&self) -> &'static str {
        match self {
            VideoError::ApiError(_) => "API_ERROR",
            VideoError::AuthenticationFailed(_) => "AUTH_FAILED",
            VideoError::InvalidRequest(_) => "INVALID_REQUEST",
            VideoError::RateLimitExceeded => "RATE_LIMIT",
            VideoError::InsufficientCredits => "NO_CREDITS",
            VideoError::GenerationFailed(_) => "GEN_FAILED",
            VideoError::JobNotFound(_) => "NOT_FOUND",
            VideoError::Timeout => "TIMEOUT",
            VideoError::NoVideoUrl => "NO_URL",
            VideoError::InvalidImageUrl(_) => "INVALID_IMAGE",
            VideoError::InvalidPrompt(_) => "INVALID_PROMPT",
            VideoError::DurationExceedsMax(_, _) => "DURATION_EXCEEDED",
            VideoError::ProviderNotAvailable(_) => "PROVIDER_UNAVAILABLE",
            VideoError::IoError(_) => "IO_ERROR",
            VideoError::SerializationError(_) => "SERIALIZE_ERROR",
            VideoError::NetworkError(_) => "NETWORK_ERROR",
            VideoError::InternalError(_) => "INTERNAL_ERROR",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_codes() {
        let err = VideoError::RateLimitExceeded;
        assert_eq!(err.error_code(), "RATE_LIMIT");
        
        let err = VideoError::Timeout;
        assert_eq!(err.error_code(), "TIMEOUT");
    }

    #[test]
    fn test_is_retryable() {
        assert!(VideoError::RateLimitExceeded.is_retryable());
        assert!(VideoError::Timeout.is_retryable());
        assert!(!VideoError::InvalidRequest("test".into()).is_retryable());
    }
}
