//! ─── Error Module ───

use serde::{Deserialize, Serialize};
use thiserror::Error;
use crate::dataset::DatasetError;

// ═══════════════════════════════════════════════════════════════════════════════
//  FINETUNE ERROR
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Error, Clone, Serialize, Deserialize)]
pub enum FinetuneError {
    #[error("API error: {0}")]
    ApiError(String),

    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),

    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    #[error("Dataset not found: {0}")]
    DatasetNotFound(String),

    #[error("Model not found: {0}")]
    ModelNotFound(String),

    #[error("Job not found: {0}")]
    JobNotFound(String),

    #[error("Training failed: {0}")]
    TrainingFailed(String),

    #[error("Validation failed: {0}")]
    ValidationFailed(String),

    #[error("Insufficient quota: {0}")]
    InsufficientQuota(String),

    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    #[error("Timeout waiting for training")]
    Timeout,

    #[error("Invalid dataset format: {0}")]
    InvalidDatasetFormat(String),

    #[error("Dataset too large: {0} tokens (max: {1})")]
    DatasetTooLarge(usize, usize),

    #[error("Unsupported model: {0}")]
    UnsupportedModel(String),

    #[error("Unsupported method: {0} for model {1}")]
    UnsupportedMethod(String, String),

    #[error("GPU not available")]
    GpuNotAvailable,

    #[error("Insufficient GPU memory: {0}GB required, {1}GB available")]
    InsufficientGpuMemory(u32, u32),

    #[error("Checkpoint error: {0}")]
    CheckpointError(String),

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

pub type FinetuneResult<T> = Result<T, FinetuneError>;

// ═══════════════════════════════════════════════════════════════════════════════
//  IMPLEMENTATIONS
// ═══════════════════════════════════════════════════════════════════════════════

impl From<serde_json::Error> for FinetuneError {
    fn from(e: serde_json::Error) -> Self {
        FinetuneError::SerializationError(e.to_string())
    }
}

impl From<DatasetError> for FinetuneError {
    fn from(e: DatasetError) -> Self {
        match e {
            DatasetError::ParseError(msg) => FinetuneError::InvalidDatasetFormat(msg),
            DatasetError::SerializeError(msg) => FinetuneError::SerializationError(msg),
            DatasetError::EmptyDataset => FinetuneError::InvalidRequest("Empty dataset".into()),
            DatasetError::EmptySample(idx) => FinetuneError::InvalidRequest(format!("Empty sample at index {}", idx)),
            DatasetError::InvalidFormat(msg) => FinetuneError::InvalidDatasetFormat(msg),
        }
    }
}

impl From<std::io::Error> for FinetuneError {
    fn from(e: std::io::Error) -> Self {
        FinetuneError::IoError(e.to_string())
    }
}

impl From<reqwest::Error> for FinetuneError {
    fn from(e: reqwest::Error) -> Self {
        if e.is_timeout() {
            FinetuneError::Timeout
        } else if e.is_connect() {
            FinetuneError::NetworkError(e.to_string())
        } else {
            FinetuneError::ApiError(e.to_string())
        }
    }
}

impl FinetuneError {
    /// Check if error is retryable
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            FinetuneError::RateLimitExceeded |
            FinetuneError::Timeout |
            FinetuneError::NetworkError(_)
        )
    }

    /// Get error code
    pub fn error_code(&self) -> &'static str {
        match self {
            FinetuneError::ApiError(_) => "API_ERROR",
            FinetuneError::AuthenticationFailed(_) => "AUTH_FAILED",
            FinetuneError::InvalidRequest(_) => "INVALID_REQUEST",
            FinetuneError::DatasetNotFound(_) => "DATASET_NOT_FOUND",
            FinetuneError::ModelNotFound(_) => "MODEL_NOT_FOUND",
            FinetuneError::JobNotFound(_) => "JOB_NOT_FOUND",
            FinetuneError::TrainingFailed(_) => "TRAINING_FAILED",
            FinetuneError::ValidationFailed(_) => "VALIDATION_FAILED",
            FinetuneError::InsufficientQuota(_) => "INSUFFICIENT_QUOTA",
            FinetuneError::RateLimitExceeded => "RATE_LIMIT",
            FinetuneError::Timeout => "TIMEOUT",
            FinetuneError::InvalidDatasetFormat(_) => "INVALID_DATASET",
            FinetuneError::DatasetTooLarge(_, _) => "DATASET_TOO_LARGE",
            FinetuneError::UnsupportedModel(_) => "UNSUPPORTED_MODEL",
            FinetuneError::UnsupportedMethod(_, _) => "UNSUPPORTED_METHOD",
            FinetuneError::GpuNotAvailable => "GPU_UNAVAILABLE",
            FinetuneError::InsufficientGpuMemory(_, _) => "INSUFFICIENT_GPU_MEMORY",
            FinetuneError::CheckpointError(_) => "CHECKPOINT_ERROR",
            FinetuneError::IoError(_) => "IO_ERROR",
            FinetuneError::SerializationError(_) => "SERIALIZE_ERROR",
            FinetuneError::NetworkError(_) => "NETWORK_ERROR",
            FinetuneError::InternalError(_) => "INTERNAL_ERROR",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_codes() {
        assert_eq!(FinetuneError::Timeout.error_code(), "TIMEOUT");
        assert_eq!(FinetuneError::RateLimitExceeded.error_code(), "RATE_LIMIT");
    }

    #[test]
    fn test_is_retryable() {
        assert!(FinetuneError::RateLimitExceeded.is_retryable());
        assert!(FinetuneError::Timeout.is_retryable());
        assert!(!FinetuneError::InvalidRequest("test".into()).is_retryable());
    }
}
