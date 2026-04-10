//! Fine-tuning error types

use std::path::PathBuf;
use thiserror::Error;

/// Fine-tuning error type
#[derive(Debug, Error)]
pub enum FinetuningError {
    /// Dataset error
    #[error("Dataset error: {0}")]
    Dataset(String),

    /// Dataset file not found
    #[error("Dataset file not found: {0}")]
    DatasetNotFound(PathBuf),

    /// Invalid dataset format
    #[error("Invalid dataset format: {0}. Expected: {1}")]
    InvalidDatasetFormat(String, String),

    /// Training error
    #[error("Training error: {0}")]
    Training(String),

    /// Model not found
    #[error("Model not found: {0}")]
    ModelNotFound(String),

    /// Checkpoint error
    #[error("Checkpoint error: {0}")]
    Checkpoint(String),

    /// Configuration error
    #[error("Configuration error: {0}")]
    Config(String),

    /// Invalid hyperparameters
    #[error("Invalid hyperparameters: {0}")]
    InvalidHyperparameters(String),

    /// Tokenizer error
    #[error("Tokenizer error: {0}")]
    Tokenizer(String),

    /// GPU/CUDA error
    #[error("GPU error: {0}")]
    Gpu(String),

    /// Memory error
    #[error("Out of memory: requested {requested}MB, available {available}MB")]
    OutOfMemory {
        requested: usize,
        available: usize,
    },

    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// JSON error
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Invalid input
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    /// Training interrupted
    #[error("Training interrupted at step {step}")]
    Interrupted { step: usize },

    /// LoRA error
    #[error("LoRA error: {0}")]
    Lora(String),

    /// Unsupported operation
    #[error("Unsupported operation: {0}")]
    Unsupported(String),

    /// Validation error
    #[error("Validation failed: {0}")]
    Validation(String),
}

/// Result type for fine-tuning operations
pub type Result<T> = std::result::Result<T, FinetuningError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let err = FinetuningError::Dataset("test error".to_string());
        assert!(err.to_string().contains("Dataset error"));
    }

    #[test]
    fn test_dataset_not_found() {
        let path = PathBuf::from("/nonexistent/file.json");
        let err = FinetuningError::DatasetNotFound(path.clone());
        assert!(err.to_string().contains("Dataset file not found"));
    }

    #[test]
    fn test_out_of_memory() {
        let err = FinetuningError::OutOfMemory {
            requested: 8192,
            available: 4096,
        };
        assert!(err.to_string().contains("8192"));
        assert!(err.to_string().contains("4096"));
    }

    #[test]
    fn test_interrupted() {
        let err = FinetuningError::Interrupted { step: 100 };
        assert!(err.to_string().contains("step 100"));
    }
}
