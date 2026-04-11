//! ─── Error Module ───

use serde::{Deserialize, Serialize};
use thiserror::Error;

// ═══════════════════════════════════════════════════════════════════════════════
//  QUANTIZE ERROR
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Error, Clone, Serialize, Deserialize)]
pub enum QuantizeError {
    #[error("Model not found: {0}")]
    ModelNotFound(String),

    #[error("Invalid model format: {0}")]
    InvalidFormat(String),

    #[error("Unsupported quantization method: {0}")]
    UnsupportedMethod(String),

    #[error("Calibration failed: {0}")]
    CalibrationFailed(String),

    #[error("Insufficient memory: required {required}GB, available {available}GB")]
    InsufficientMemory {
        required: u32,
        available: u32,
    },

    #[error("Conversion error: {0}")]
    ConversionError(String),

    #[error("IO error: {0}")]
    IoError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("GPU not available: {0}")]
    GpuNotAvailable(String),

    #[error("Backend not available: {0}")]
    BackendNotAvailable(String),

    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    #[error("Process error: {0}")]
    ProcessError(String),

    #[error("API error: {0}")]
    ApiError(String),

    #[error("Timeout: {0}")]
    Timeout(String),
}

pub type QuantizeResult<T> = Result<T, QuantizeError>;

// ═══════════════════════════════════════════════════════════════════════════════
//  FROM IMPLEMENTATIONS
// ═══════════════════════════════════════════════════════════════════════════════

impl From<std::io::Error> for QuantizeError {
    fn from(e: std::io::Error) -> Self {
        QuantizeError::IoError(e.to_string())
    }
}

impl From<serde_json::Error> for QuantizeError {
    fn from(e: serde_json::Error) -> Self {
        QuantizeError::SerializationError(e.to_string())
    }
}

impl From<reqwest::Error> for QuantizeError {
    fn from(e: reqwest::Error) -> Self {
        QuantizeError::ApiError(e.to_string())
    }
}
