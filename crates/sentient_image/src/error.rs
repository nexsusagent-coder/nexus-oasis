// ═══════════════════════════════════════════════════════════════════════════════
//  SENTIENT OS - Image Generation Errors
// ═══════════════════════════════════════════════════════════════════════════════

use thiserror::Error;

pub type Result<T> = std::result::Result<T, ImageError>;

/// Image generation errors
#[derive(Debug, Error)]
pub enum ImageError {
    #[error("API key is missing")]
    MissingApiKey,

    #[error("Invalid API key")]
    InvalidApiKey,

    #[error("Provider '{0}' not available")]
    ProviderNotAvailable(String),

    #[error("Model '{0}' not found")]
    ModelNotFound(String),

    #[error("Content policy violation: {0}")]
    ContentPolicyViolation(String),

    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    #[error("Insufficient credits")]
    InsufficientCredits,

    #[error("Invalid prompt: {0}")]
    InvalidPrompt(String),

    #[error("Invalid image size: {0}")]
    InvalidSize(String),

    #[error("Image generation failed: {0}")]
    GenerationFailed(String),

    #[error("Timeout after {0} seconds")]
    Timeout(u64),

    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("JSON parsing failed: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("API error: {0}")]
    ApiError(String),

    #[error("Invalid base64: {0}")]
    InvalidBase64(String),

    #[error("IO error: {0}")]
    IoError(String),
}

impl ImageError {
    /// Check if error is retryable
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            ImageError::RateLimitExceeded |
            ImageError::Timeout(_) |
            ImageError::HttpError(_)
        )
    }
}
