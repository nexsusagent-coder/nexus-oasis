// ═══════════════════════════════════════════════════════════════════════════════
//  SENTIENT OS - Groq Errors
// ═══════════════════════════════════════════════════════════════════════════════

use thiserror::Error;

pub type Result<T> = std::result::Result<T, GroqError>;

/// Groq API errors
#[derive(Debug, Error)]
pub enum GroqError {
    #[error("API key is missing. Set GROQ_API_KEY environment variable.")]
    MissingApiKey,

    #[error("Invalid API key")]
    InvalidApiKey,

    #[error("Rate limit exceeded. Wait before retrying.")]
    RateLimitExceeded,

    #[error("Model '{0}' not found or not available")]
    ModelNotFound(String),

    #[error("Context length exceeded. Max: {max}, Requested: {requested}")]
    ContextLengthExceeded { max: usize, requested: usize },

    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("JSON parsing failed: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("API error: {0}")]
    ApiError(String),

    #[error("Streaming error: {0}")]
    StreamError(String),

    #[error("Timeout after {0} seconds")]
    Timeout(u64),

    #[error("Max retries ({0}) exceeded")]
    MaxRetriesExceeded(u32),

    #[error("Invalid response: {0}")]
    InvalidResponse(String),
}

impl GroqError {
    /// Check if error is retryable
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            GroqError::RateLimitExceeded |
            GroqError::Timeout(_) |
            GroqError::HttpError(_) |
            GroqError::StreamError(_)
        )
    }

    /// Get suggested wait time for retry (in milliseconds)
    pub fn retry_after_ms(&self) -> Option<u64> {
        match self {
            GroqError::RateLimitExceeded => Some(60000), // 1 minute
            GroqError::Timeout(_) => Some(5000),         // 5 seconds
            _ => None,
        }
    }
}
