//! Core types for fine-tuning

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Model identifier
pub type ModelId = String;

/// Dataset identifier
pub type DatasetId = String;

/// Checkpoint identifier
pub type CheckpointId = String;

/// Training step
pub type Step = usize;

/// Epoch number
pub type Epoch = usize;

/// Training loss value
pub type Loss = f32;

/// Learning rate
pub type LearningRate = f32;

/// Batch size
pub type BatchSize = usize;

/// Training sample
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingSample {
    /// Unique identifier
    pub id: String,
    /// Input text (prompt)
    pub input: String,
    /// Output text (completion)
    pub output: String,
    /// Optional metadata
    #[serde(default)]
    pub metadata: HashMap<String, String>,
}

impl TrainingSample {
    /// Create new training sample
    pub fn new(input: impl Into<String>, output: impl Into<String>) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            input: input.into(),
            output: output.into(),
            metadata: HashMap::new(),
        }
    }

    /// With custom ID
    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = id.into();
        self
    }

    /// Add metadata
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Format as conversation
    pub fn as_conversation(&self) -> String {
        format!("User: {}\nAssistant: {}", self.input, self.output)
    }

    /// Format as completion
    pub fn as_completion(&self) -> String {
        format!("{}{}", self.input, self.output)
    }
}

/// Dataset for fine-tuning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dataset {
    /// Dataset ID
    pub id: DatasetId,
    /// Dataset name
    pub name: String,
    /// Description
    pub description: Option<String>,
    /// Training samples
    pub samples: Vec<TrainingSample>,
    /// Validation samples (optional)
    pub validation_samples: Option<Vec<TrainingSample>>,
    /// Dataset metadata
    #[serde(default)]
    pub metadata: HashMap<String, String>,
}

impl Dataset {
    /// Create new dataset
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.into(),
            description: None,
            samples: Vec::new(),
            validation_samples: None,
            metadata: HashMap::new(),
        }
    }

    /// Add sample
    pub fn add_sample(&mut self, sample: TrainingSample) {
        self.samples.push(sample);
    }

    /// Add samples
    pub fn add_samples(&mut self, samples: Vec<TrainingSample>) {
        self.samples.extend(samples);
    }

    /// Split into train/validation
    pub fn split_validation(&mut self, ratio: f32) {
        if ratio <= 0.0 || ratio >= 1.0 {
            return;
        }

        let split_idx = ((self.samples.len() as f32) * (1.0 - ratio)) as usize;
        let validation = self.samples.split_off(split_idx);
        self.validation_samples = Some(validation);
    }

    /// Get sample count
    pub fn len(&self) -> usize {
        self.samples.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.samples.is_empty()
    }

    /// Get validation sample count
    pub fn validation_len(&self) -> usize {
        self.validation_samples.as_ref().map(|v| v.len()).unwrap_or(0)
    }
}

/// Hyperparameters for training
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hyperparameters {
    /// Learning rate
    pub learning_rate: LearningRate,
    /// Batch size
    pub batch_size: BatchSize,
    /// Number of epochs
    pub epochs: Epoch,
    /// Warmup steps
    pub warmup_steps: usize,
    /// Weight decay
    pub weight_decay: f32,
    /// Gradient accumulation steps
    pub gradient_accumulation_steps: usize,
    /// Max gradient norm (for clipping)
    pub max_grad_norm: f32,
    /// Learning rate scheduler
    pub lr_scheduler: LrScheduler,
    /// Optimizer
    pub optimizer: Optimizer,
    /// Seed for reproducibility
    pub seed: Option<u64>,
}

impl Default for Hyperparameters {
    fn default() -> Self {
        Self {
            learning_rate: 1e-4,
            batch_size: 8,
            epochs: 3,
            warmup_steps: 100,
            weight_decay: 0.01,
            gradient_accumulation_steps: 1,
            max_grad_norm: 1.0,
            lr_scheduler: LrScheduler::Linear,
            optimizer: Optimizer::AdamW,
            seed: Some(42),
        }
    }
}

impl Hyperparameters {
    /// Create new hyperparameters
    pub fn new() -> Self {
        Self::default()
    }

    /// With learning rate
    pub fn with_learning_rate(mut self, lr: LearningRate) -> Self {
        self.learning_rate = lr;
        self
    }

    /// With batch size
    pub fn with_batch_size(mut self, size: BatchSize) -> Self {
        self.batch_size = size;
        self
    }

    /// With epochs
    pub fn with_epochs(mut self, epochs: Epoch) -> Self {
        self.epochs = epochs;
        self
    }

    /// Validate hyperparameters
    pub fn validate(&self) -> crate::Result<()> {
        if self.learning_rate <= 0.0 {
            return Err(crate::FinetuningError::InvalidHyperparameters(
                "Learning rate must be positive".to_string(),
            ));
        }
        if self.batch_size == 0 {
            return Err(crate::FinetuningError::InvalidHyperparameters(
                "Batch size must be at least 1".to_string(),
            ));
        }
        if self.epochs == 0 {
            return Err(crate::FinetuningError::InvalidHyperparameters(
                "Epochs must be at least 1".to_string(),
            ));
        }
        Ok(())
    }
}

/// Learning rate scheduler
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum LrScheduler {
    /// Constant learning rate
    Constant,
    /// Linear decay
    #[default]
    Linear,
    /// Cosine decay
    Cosine,
    /// Cosine with warm restarts
    CosineWithRestarts,
    /// Polynomial decay
    Polynomial,
    /// Inverse square root decay
    InverseSqrt,
}

/// Optimizer type
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Optimizer {
    /// Adam
    Adam,
    /// AdamW (Adam with weight decay)
    #[default]
    AdamW,
    /// SGD
    Sgd,
    /// AdaFactor
    AdaFactor,
    /// LAMB
    Lamb,
}

/// Training configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingConfig {
    /// Base model ID or path
    pub model_id: ModelId,
    /// Output directory for checkpoints
    pub output_dir: String,
    /// Hyperparameters
    pub hyperparameters: Hyperparameters,
    /// Fine-tuning method
    pub method: FinetuningMethod,
    /// Maximum sequence length
    pub max_seq_length: usize,
    /// Save checkpoint every N steps
    pub save_steps: usize,
    /// Evaluate every N steps
    pub eval_steps: usize,
    /// Log every N steps
    pub log_steps: usize,
    /// Use mixed precision (FP16)
    pub fp16: bool,
    /// Use BF16 (if supported)
    pub bf16: bool,
    /// Use gradient checkpointing
    pub gradient_checkpointing: bool,
    /// Number of dataloader workers
    pub dataloader_workers: usize,
}

impl Default for TrainingConfig {
    fn default() -> Self {
        Self {
            model_id: "gemma-4-4b".to_string(),
            output_dir: "./output".to_string(),
            hyperparameters: Hyperparameters::default(),
            method: FinetuningMethod::Lora,
            max_seq_length: 2048,
            save_steps: 500,
            eval_steps: 100,
            log_steps: 10,
            fp16: true,
            bf16: false,
            gradient_checkpointing: true,
            dataloader_workers: 4,
        }
    }
}

impl TrainingConfig {
    /// Create new config
    pub fn new(model_id: impl Into<String>) -> Self {
        Self {
            model_id: model_id.into(),
            ..Default::default()
        }
    }

    /// With output directory
    pub fn with_output_dir(mut self, dir: impl Into<String>) -> Self {
        self.output_dir = dir.into();
        self
    }

    /// With hyperparameters
    pub fn with_hyperparameters(mut self, hp: Hyperparameters) -> Self {
        self.hyperparameters = hp;
        self
    }

    /// With fine-tuning method
    pub fn with_method(mut self, method: FinetuningMethod) -> Self {
        self.method = method;
        self
    }

    /// Validate configuration
    pub fn validate(&self) -> crate::Result<()> {
        self.hyperparameters.validate()?;

        if self.max_seq_length == 0 {
            return Err(crate::FinetuningError::Config(
                "max_seq_length must be at least 1".to_string(),
            ));
        }

        if self.model_id.is_empty() {
            return Err(crate::FinetuningError::Config(
                "model_id cannot be empty".to_string(),
            ));
        }

        Ok(())
    }
}

/// Fine-tuning method
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum FinetuningMethod {
    /// Full fine-tuning (all parameters)
    Full,
    /// LoRA (Low-Rank Adaptation)
    Lora,
    /// QLoRA (Quantized LoRA)
    Qlora,
    /// Prefix tuning
    PrefixTuning,
    /// Prompt tuning
    PromptTuning,
    /// IA³ (Infused Adapter by Inhibiting and Amplifying Inner Activations)
    Ia3,
}

impl Default for FinetuningMethod {
    fn default() -> Self {
        Self::Lora
    }
}

impl FinetuningMethod {
    /// Check if method is parameter-efficient
    pub fn is_peft(&self) -> bool {
        matches!(
            self,
            Self::Lora | Self::Qlora | Self::PrefixTuning | Self::PromptTuning | Self::Ia3
        )
    }

    /// Check if method uses quantization
    pub fn is_quantized(&self) -> bool {
        matches!(self, Self::Qlora)
    }
}

/// LoRA configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoraConfig {
    /// LoRA rank (r)
    pub r: usize,
    /// LoRA alpha scaling
    pub alpha: f32,
    /// Dropout probability
    pub dropout: f32,
    /// Target modules (e.g., ["q_proj", "v_proj"])
    pub target_modules: Vec<String>,
    /// Bias type: none, all, or lora_only
    pub bias: LoraBias,
    /// Task type
    pub task_type: Option<TaskType>,
}

impl Default for LoraConfig {
    fn default() -> Self {
        Self {
            r: 8,
            alpha: 16.0,
            dropout: 0.05,
            target_modules: vec!["q_proj".to_string(), "v_proj".to_string()],
            bias: LoraBias::None,
            task_type: None,
        }
    }
}

impl LoraConfig {
    /// Create new LoRA config
    pub fn new(r: usize) -> Self {
        Self {
            r,
            alpha: (r * 2) as f32,
            ..Default::default()
        }
    }

    /// With target modules
    pub fn with_target_modules(mut self, modules: Vec<String>) -> Self {
        self.target_modules = modules;
        self
    }

    /// With dropout
    pub fn with_dropout(mut self, dropout: f32) -> Self {
        self.dropout = dropout;
        self
    }

    /// Calculate scaling factor
    pub fn scaling(&self) -> f32 {
        self.alpha / (self.r as f32)
    }
}

/// LoRA bias configuration
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum LoraBias {
    #[default]
    None,
    All,
    LoraOnly,
}

/// Task type for fine-tuning
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TaskType {
    CausalLm,
    Seq2SeqLm,
    TokenClassification,
    SeqClassification,
    QuestionAnswering,
}

/// Training metrics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TrainingMetrics {
    /// Current epoch
    pub epoch: Epoch,
    /// Current step
    pub step: Step,
    /// Total steps
    pub total_steps: usize,
    /// Training loss
    pub train_loss: Loss,
    /// Validation loss
    pub val_loss: Option<Loss>,
    /// Learning rate
    pub learning_rate: LearningRate,
    /// Gradient norm
    pub grad_norm: Option<f32>,
    /// Tokens per second
    pub tokens_per_second: Option<f32>,
    /// Estimated time remaining (seconds)
    pub eta_seconds: Option<u64>,
    /// Memory usage (MB)
    pub memory_mb: Option<usize>,
    /// GPU memory usage (MB)
    pub gpu_memory_mb: Option<usize>,
}

impl TrainingMetrics {
    /// Progress percentage (0-100)
    pub fn progress_percent(&self) -> f32 {
        if self.total_steps == 0 {
            return 0.0;
        }
        (self.step as f32 / self.total_steps as f32) * 100.0
    }

    /// Format ETA as human readable
    pub fn format_eta(&self) -> String {
        if let Some(secs) = self.eta_seconds {
            let hours = secs / 3600;
            let mins = (secs % 3600) / 60;
            let secs = secs % 60;

            if hours > 0 {
                format!("{}h {}m {}s", hours, mins, secs)
            } else if mins > 0 {
                format!("{}m {}s", mins, secs)
            } else {
                format!("{}s", secs)
            }
        } else {
            "N/A".to_string()
        }
    }
}

/// Training state for checkpointing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingState {
    /// Current step
    pub step: Step,
    /// Current epoch
    pub epoch: Epoch,
    /// Best validation loss
    pub best_val_loss: Option<Loss>,
    /// Random state for reproducibility
    pub random_state: Option<Vec<u8>>,
    /// Optimizer state (serialized)
    pub optimizer_state: Option<Vec<u8>>,
    /// Scheduler state (serialized)
    pub scheduler_state: Option<Vec<u8>>,
}

impl TrainingState {
    /// Create new training state
    pub fn new() -> Self {
        Self {
            step: 0,
            epoch: 0,
            best_val_loss: None,
            random_state: None,
            optimizer_state: None,
            scheduler_state: None,
        }
    }
}

impl Default for TrainingState {
    fn default() -> Self {
        Self::new()
    }
}

/// Checkpoint information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Checkpoint {
    /// Checkpoint ID
    pub id: CheckpointId,
    /// Checkpoint path
    pub path: String,
    /// Step when saved
    pub step: Step,
    /// Epoch when saved
    pub epoch: Epoch,
    /// Training loss
    pub train_loss: Loss,
    /// Validation loss
    pub val_loss: Option<Loss>,
    /// Timestamp
    pub timestamp: i64,
    /// File size in bytes
    pub size_bytes: u64,
}

impl Checkpoint {
    /// Format size as human readable
    pub fn format_size(&self) -> String {
        let mb = self.size_bytes as f64 / (1024.0 * 1024.0);
        if mb >= 1024.0 {
            format!("{:.2} GB", mb / 1024.0)
        } else {
            format!("{:.2} MB", mb)
        }
    }
}

/// Fine-tuned model info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinetunedModel {
    /// Model ID
    pub id: ModelId,
    /// Base model ID
    pub base_model: ModelId,
    /// Fine-tuning method
    pub method: FinetuningMethod,
    /// Training config used
    pub config: TrainingConfig,
    /// Final training loss
    pub final_train_loss: Loss,
    /// Final validation loss
    pub final_val_loss: Option<Loss>,
    /// Training duration in seconds
    pub training_duration_secs: u64,
    /// Number of training samples
    pub num_samples: usize,
    /// Checkpoint path
    pub checkpoint_path: String,
    /// Created timestamp
    pub created_at: i64,
}

// Add uuid dependency
mod uuid {
    pub struct Uuid;

    impl Uuid {
        pub fn new_v4() -> Self {
            Uuid
        }

        pub fn to_string(&self) -> String {
            // Simple UUID v4 generation
            use std::time::{SystemTime, UNIX_EPOCH};
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or(0);
            format!("{:016x}-{:016x}", now, now.wrapping_mul(0x5851F42D4C957F2D))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_training_sample() {
        let sample = TrainingSample::new("What is Rust?", "Rust is a systems programming language.");

        assert!(!sample.id.is_empty());
        assert_eq!(sample.input, "What is Rust?");
        assert_eq!(sample.output, "Rust is a systems programming language.");
    }

    #[test]
    fn test_training_sample_metadata() {
        let sample = TrainingSample::new("Q", "A")
            .with_metadata("source", "test")
            .with_metadata("quality", "high");

        assert_eq!(sample.metadata.get("source"), Some(&"test".to_string()));
    }

    #[test]
    fn test_dataset() {
        let mut dataset = Dataset::new("test_dataset");
        dataset.add_sample(TrainingSample::new("Q1", "A1"));
        dataset.add_sample(TrainingSample::new("Q2", "A2"));

        assert_eq!(dataset.len(), 2);
    }

    #[test]
    fn test_dataset_split() {
        let mut dataset = Dataset::new("test");
        for i in 0..100 {
            dataset.add_sample(TrainingSample::new(format!("Q{}", i), format!("A{}", i)));
        }

        dataset.split_validation(0.2);
        assert_eq!(dataset.len(), 80);
        assert_eq!(dataset.validation_len(), 20);
    }

    #[test]
    fn test_hyperparameters() {
        let hp = Hyperparameters::new()
            .with_learning_rate(1e-5)
            .with_batch_size(16)
            .with_epochs(5);

        assert!((hp.learning_rate - 1e-5).abs() < 1e-10);
        assert_eq!(hp.batch_size, 16);
        assert_eq!(hp.epochs, 5);
    }

    #[test]
    fn test_hyperparameters_validate() {
        let hp = Hyperparameters::new()
            .with_learning_rate(-1.0);

        assert!(hp.validate().is_err());
    }

    #[test]
    fn test_training_config() {
        let config = TrainingConfig::new("gemma-4-4b")
            .with_output_dir("./output")
            .with_method(FinetuningMethod::Lora);

        assert_eq!(config.model_id, "gemma-4-4b");
        assert!(config.method.is_peft());
    }

    #[test]
    fn test_lora_config() {
        let config = LoraConfig::new(16);

        assert_eq!(config.r, 16);
        assert_eq!(config.alpha, 32.0);
        assert!((config.scaling() - 2.0).abs() < 1e-6);
    }

    #[test]
    fn test_training_metrics() {
        let mut metrics = TrainingMetrics::default();
        metrics.step = 500;
        metrics.total_steps = 1000;
        metrics.train_loss = 0.5;

        assert!((metrics.progress_percent() - 50.0).abs() < 1e-6);
    }

    #[test]
    fn test_training_metrics_eta() {
        let mut metrics = TrainingMetrics::default();
        metrics.eta_seconds = Some(3661);

        assert_eq!(metrics.format_eta(), "1h 1m 1s");
    }

    #[test]
    fn test_finetuning_method() {
        assert!(FinetuningMethod::Lora.is_peft());
        assert!(!FinetuningMethod::Full.is_peft());
        assert!(FinetuningMethod::Qlora.is_quantized());
        assert!(!FinetuningMethod::Lora.is_quantized());
    }

    #[test]
    fn test_checkpoint_size() {
        let checkpoint = Checkpoint {
            id: "ckpt-1".to_string(),
            path: "/models/ckpt-1".to_string(),
            step: 100,
            epoch: 1,
            train_loss: 0.5,
            val_loss: None,
            timestamp: 0,
            size_bytes: 1024 * 1024 * 512, // 512 MB
        };

        assert_eq!(checkpoint.format_size(), "512.00 MB");
    }
}
