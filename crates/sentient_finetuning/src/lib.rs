//! # Sentient Fine-tuning
//!
//! Fine-tuning support for SENTIENT OS - LoRA, QLoRA, dataset preparation.
//!
//! ## Features
//!
//! - **Dataset Handling**: Load/save datasets in multiple formats (JSON, JSONL, CSV, Parquet)
//! - **Training Engine**: Async training with progress monitoring
//! - **Fine-tuning Methods**: LoRA, QLoRA, Full fine-tuning, Prefix/Prompt tuning
//! - **Checkpointing**: Save/resume training from checkpoints

// Suppress warnings
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
//! - **Hyperparameters**: Comprehensive hyperparameter configuration
//!
//! ## Example
//!
//! ```rust
//! use sentient_finetuning::{
//!     TrainingEngine, TrainingConfig, Dataset, DatasetLoader,
//!     TrainingSample, Hyperparameters, FinetuningMethod,
//! };
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Load dataset
//! let dataset = DatasetLoader::new("data.jsonl")
//!     .with_input_field("prompt")
//!     .with_output_field("response")
//!     .load()?;
//!
//! // Configure training
//! let config = TrainingConfig::new("gemma-4-4b")
//!     .with_output_dir("./output")
//!     .with_method(FinetuningMethod::Lora)
//!     .with_hyperparameters(
//!         Hyperparameters::new()
//!             .with_learning_rate(1e-4)
//!             .with_batch_size(8)
//!             .with_epochs(3)
//!     );
//!
//! // Create and run training
//! let mut engine = TrainingEngine::new(config, dataset);
//! let handle = engine.handle();
//!
//! // Monitor training
//! tokio::spawn(async move {
//!     let mut events = handle.subscribe();
//!     while let Ok(event) = events.recv().await {
//!         println!("Event: {:?}", event);
//!     }
//! });
//!
//! // Train
//! let model = engine.train().await?;
//! println!("Trained model: {:?}", model.id);
//!
//! # Ok(())
//! # }
//! ```

pub mod error;
pub mod types;
pub mod dataset;
pub mod training;

pub use error::{FinetuningError, Result};
pub use types::*;
pub use dataset::{DatasetLoader, DatasetSaver, DatasetFormat, DatasetUtils, DatasetStats};
pub use training::{TrainingEngine, TrainingHandle, TrainingStatus, TrainingEvent};

/// Fine-tuning version
pub const FINETUNING_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!FINETUNING_VERSION.is_empty());
    }

    #[tokio::test]
    async fn test_end_to_end_workflow() {
        use std::io::Write;
        use tempfile::tempdir;

        // Create temp dataset
        let dir = tempdir().unwrap();
        let dataset_path = dir.path().join("test.jsonl");

        let mut file = std::fs::File::create(&dataset_path).unwrap();
        for i in 0..5 {
            writeln!(file, r#"{{"input": "Q{}", "output": "A{}"}}"#, i, i).unwrap();
        }

        // Load dataset
        let dataset = DatasetLoader::new(&dataset_path)
            .load()
            .unwrap();

        assert_eq!(dataset.len(), 5);

        // Configure
        let config = TrainingConfig::new("test-model")
            .with_output_dir(dir.path().join("output").to_str().unwrap())
            .with_method(FinetuningMethod::Lora)
            .with_hyperparameters(
                Hyperparameters::new()
                    .with_epochs(1)
                    .with_batch_size(2)
            );

        // Train
        let mut engine = TrainingEngine::new(config, dataset);
        let result = engine.train().await;

        assert!(result.is_ok());
        let model = result.unwrap();
        assert!(model.id.contains("finetuned"));
    }
}
