// ═══════════════════════════════════════════════════════════════════════════════
//  SENTIENT OS - Sandbox Errors
// ═══════════════════════════════════════════════════════════════════════════════

use thiserror::Error;

pub type Result<T> = std::result::Result<T, SandboxError>;

/// Sandbox errors
#[derive(Debug, Error)]
pub enum SandboxError {
    #[error("API key is missing. Set E2B_API_KEY environment variable.")]
    MissingApiKey,

    #[error("Invalid API key")]
    InvalidApiKey,

    #[error("Sandbox '{0}' not found")]
    SandboxNotFound(String),

    #[error("Template '{0}' not found")]
    TemplateNotFound(String),

    #[error("Sandbox timeout after {0} seconds")]
    Timeout(u64),

    #[error("Execution failed: {0}")]
    ExecutionFailed(String),

    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    #[error("Insufficient credits")]
    InsufficientCredits,

    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("JSON parsing failed: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("API error: {0}")]
    ApiError(String),

    #[error("IO error: {0}")]
    IoError(String),

    #[error("Invalid response: {0}")]
    InvalidResponse(String),
}

impl SandboxError {
    /// Check if error is retryable
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            SandboxError::RateLimitExceeded |
            SandboxError::Timeout(_) |
            SandboxError::HttpError(_)
        )
    }
}
