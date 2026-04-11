//! ─── Training Types ───

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

use crate::FineTuneMethod;

// ═══════════════════════════════════════════════════════════════════════════════
//  TRAINING CONFIG
// ═══════════════════════════════════════════════════════════════════════════════

/// Training configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingConfig {
    /// Base model to fine-tune
    pub base_model: String,
    /// Dataset ID
    pub dataset_id: String,
    /// Fine-tuning method
    #[serde(default)]
    pub method: FineTuneMethod,
    /// Hyperparameters
    #[serde(default)]
    pub hyperparameters: Hyperparameters,
    /// LoRA configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lora_config: Option<crate::LoraConfig>,
    /// QLoRA configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qlora_config: Option<crate::QloraConfig>,
    /// Output model name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_name: Option<String>,
    /// Validation split ratio (0.0 - 1.0)
    #[serde(default = "default_validation_split")]
    pub validation_split: f32,
    /// Random seed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<u64>,
    /// Suffix for model name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
    /// Checkpoint every N steps
    #[serde(default)]
    pub checkpoint_steps: Option<u32>,
    /// Evaluation every N steps
    #[serde(default)]
    pub eval_steps: Option<u32>,
    /// Early stopping patience
    #[serde(default)]
    pub early_stopping_patience: Option<u32>,
}

fn default_validation_split() -> f32 { 0.1 }

impl Default for TrainingConfig {
    fn default() -> Self {
        Self {
            base_model: String::new(),
            dataset_id: String::new(),
            method: FineTuneMethod::default(),
            hyperparameters: Hyperparameters::default(),
            lora_config: None,
            qlora_config: None,
            output_name: None,
            validation_split: default_validation_split(),
            seed: None,
            suffix: None,
            checkpoint_steps: None,
            eval_steps: None,
            early_stopping_patience: None,
        }
    }
}

impl TrainingConfig {
    /// Set number of epochs
    pub fn with_epochs(mut self, epochs: u32) -> Self {
        self.hyperparameters.num_epochs = epochs;
        self
    }

    /// Set batch size
    pub fn with_batch_size(mut self, size: u32) -> Self {
        self.hyperparameters.batch_size = size;
        self
    }

    /// Set learning rate
    pub fn with_learning_rate(mut self, lr: f32) -> Self {
        self.hyperparameters.learning_rate = lr;
        self
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  HYPERPARAMETERS
// ═══════════════════════════════════════════════════════════════════════════════

/// Training hyperparameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hyperparameters {
    /// Number of training epochs
    #[serde(default = "default_epochs")]
    pub num_epochs: u32,
    
    /// Batch size
    #[serde(default = "default_batch_size")]
    pub batch_size: u32,
    
    /// Learning rate
    #[serde(default = "default_learning_rate")]
    pub learning_rate: f32,
    
    /// Weight decay
    #[serde(default)]
    pub weight_decay: f32,
    
    /// Warmup ratio
    #[serde(default = "default_warmup_ratio")]
    pub warmup_ratio: f32,
    
    /// Gradient accumulation steps
    #[serde(default)]
    pub gradient_accumulation_steps: u32,
    
    /// Maximum gradient norm
    #[serde(default = "default_max_grad_norm")]
    pub max_grad_norm: f32,
    
    /// Learning rate scheduler
    #[serde(default)]
    pub lr_scheduler: LrScheduler,
    
    /// Optimizer
    #[serde(default)]
    pub optimizer: Optimizer,
}

fn default_epochs() -> u32 { 3 }
fn default_batch_size() -> u32 { 8 }
fn default_learning_rate() -> f32 { 5e-5 }
fn default_warmup_ratio() -> f32 { 0.03 }
fn default_max_grad_norm() -> f32 { 1.0 }

impl Default for Hyperparameters {
    fn default() -> Self {
        Self {
            num_epochs: default_epochs(),
            batch_size: default_batch_size(),
            learning_rate: default_learning_rate(),
            weight_decay: 0.0,
            warmup_ratio: default_warmup_ratio(),
            gradient_accumulation_steps: 1,
            max_grad_norm: default_max_grad_norm(),
            lr_scheduler: LrScheduler::default(),
            optimizer: Optimizer::default(),
        }
    }
}

/// Learning rate scheduler
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LrScheduler {
    #[serde(rename = "constant")]
    Constant,
    #[serde(rename = "linear")]
    Linear,
    #[serde(rename = "cosine")]
    Cosine,
    #[serde(rename = "polynomial")]
    Polynomial,
}

impl Default for LrScheduler {
    fn default() -> Self {
        Self::Cosine
    }
}

/// Optimizer type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Optimizer {
    #[serde(rename = "adam")]
    Adam,
    #[serde(rename = "adamw")]
    AdamW,
    #[serde(rename = "sgd")]
    SGD,
    #[serde(rename = "adafactor")]
    Adafactor,
}

impl Default for Optimizer {
    fn default() -> Self {
        Self::AdamW
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  TRAINING JOB
// ═══════════════════════════════════════════════════════════════════════════════

/// Training job
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingJob {
    /// Job ID
    pub id: String,
    /// Base model
    pub base_model: String,
    /// Dataset ID
    pub dataset_id: String,
    /// Status
    pub status: TrainingStatus,
    /// Created at
    pub created_at: DateTime<Utc>,
    /// Updated at
    pub updated_at: DateTime<Utc>,
    /// Fine-tuned model ID (when complete)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fine_tuned_model: Option<String>,
    /// Training metrics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<TrainingMetrics>,
    /// Error message (if failed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Estimated completion time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_completion: Option<DateTime<Utc>>,
    /// Checkpoints
    #[serde(default)]
    pub checkpoints: Vec<Checkpoint>,
}

/// Training status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrainingStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "validating")]
    Validating,
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "succeeded")]
    Succeeded,
    #[serde(rename = "failed")]
    Failed(String),
    #[serde(rename = "cancelled")]
    Cancelled,
}

impl Default for TrainingStatus {
    fn default() -> Self {
        Self::Pending
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  TRAINING METRICS
// ═══════════════════════════════════════════════════════════════════════════════

/// Training metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingMetrics {
    /// Current epoch
    pub epoch: f32,
    /// Current step
    pub step: u32,
    /// Total steps
    pub total_steps: u32,
    /// Training loss
    pub train_loss: f32,
    /// Validation loss
    #[serde(skip_serializing_if = "Option::is_none")]
    pub val_loss: Option<f32>,
    /// Learning rate
    pub learning_rate: f32,
    /// Tokens processed
    pub tokens_processed: u64,
    /// Tokens per second
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tokens_per_second: Option<f32>,
    /// GPU memory usage (GB)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gpu_memory_gb: Option<f32>,
    /// Estimated time remaining (seconds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_time_remaining: Option<u64>,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  CHECKPOINT
// ═══════════════════════════════════════════════════════════════════════════════

/// Training checkpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Checkpoint {
    /// Checkpoint ID
    pub id: String,
    /// Step number
    pub step: u32,
    /// Epoch
    pub epoch: f32,
    /// Training loss
    pub train_loss: f32,
    /// Validation loss
    #[serde(skip_serializing_if = "Option::is_none")]
    pub val_loss: Option<f32>,
    /// Created at
    pub created_at: DateTime<Utc>,
    /// File size in bytes
    pub size_bytes: u64,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  MODEL ADAPTER
// ═══════════════════════════════════════════════════════════════════════════════

/// Fine-tuned model adapter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelAdapter {
    /// Model ID
    pub id: String,
    /// Base model
    pub base_model: String,
    /// Adapter name
    pub name: String,
    /// Fine-tuning method
    pub method: FineTuneMethod,
    /// Created at
    pub created_at: DateTime<Utc>,
    /// Model size in bytes
    pub size_bytes: u64,
    /// Download URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_url: Option<String>,
    /// HuggingFace repo ID (if pushed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hf_repo_id: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_training_config_default() {
        let config = TrainingConfig::default();
        assert!(config.base_model.is_empty());
        assert_eq!(config.validation_split, 0.1);
    }

    #[test]
    fn test_hyperparameters_default() {
        let hp = Hyperparameters::default();
        assert_eq!(hp.num_epochs, 3);
        assert_eq!(hp.batch_size, 8);
    }

    #[test]
    fn test_training_status_serialization() {
        let status = TrainingStatus::Running;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, "\"running\"");
    }

    #[test]
    fn test_lr_scheduler_default() {
        assert_eq!(LrScheduler::default(), LrScheduler::Cosine);
    }

    #[test]
    fn test_optimizer_default() {
        assert_eq!(Optimizer::default(), Optimizer::AdamW);
    }
}
