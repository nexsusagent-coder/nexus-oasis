//! ─── SENTIENT OS Model Fine-tuning ───
//!
//! Fine-tuning support for LLMs with multiple methods:
//! - LoRA (Low-Rank Adaptation)
//! - QLoRA (Quantized LoRA)
//! - Full Fine-tuning
//!
//! # Example
//! ```ignore
//! use sentient_finetune::{FineTuner, TrainingConfig, Method};

// Suppress warnings
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_mut)]
//!
//! let trainer = FineTuner::openai("api-key");
//! let config = TrainingConfig::new("model-id", "dataset-id")
//!     .with_method(Method::Lora)
//!     .with_epochs(3);
//! let job = trainer.train(config).await?;
//! ```

pub mod dataset;
pub mod error;
pub mod types;
pub mod method;
pub mod providers;
pub mod monitor;
pub mod multimodal;

pub use error::{FinetuneError, FinetuneResult};
pub use types::{
    TrainingConfig, TrainingJob, TrainingStatus, TrainingMetrics,
    Hyperparameters, Checkpoint, ModelAdapter,
};
pub use method::{FineTuneMethod, LoraConfig, QloraConfig, FullConfig};
pub use dataset::{Dataset, DatasetFormat, DatasetSplitter};
pub use providers::{FineTuneProvider, FineTuner};
pub use monitor::{TrainingMonitor, LogEntry};
pub use multimodal::{
    MultiModalTrainer, MultiModalTrainingConfig, MultiModalDataset,
    MultiModalSample, Modality, ImageData, AudioData, VideoData,
    VisionEncoderConfig, AudioEncoderConfig, AlignmentMethod,
};

use serde::{Deserialize, Serialize};
use std::sync::Arc;

// ═══════════════════════════════════════════════════════════════════════════════
//  FINE-TUNE CLIENT
// ═══════════════════════════════════════════════════════════════════════════════

/// Main fine-tuning client
pub struct FinetuneClient {
    provider: Arc<dyn FineTuneProvider + Send + Sync>,
}

impl FinetuneClient {
    /// Create client with OpenAI provider
    pub fn openai(api_key: impl Into<String>) -> Self {
        Self {
            provider: Arc::new(providers::OpenAIProvider::new(api_key.into())),
        }
    }

    /// Create client with Together AI provider
    pub fn together(api_key: impl Into<String>) -> Self {
        Self {
            provider: Arc::new(providers::TogetherProvider::new(api_key.into())),
        }
    }

    /// Create client with local training (requires GPU)
    pub fn local() -> Self {
        Self {
            provider: Arc::new(providers::LocalProvider::new()),
        }
    }

    /// Create with custom provider
    pub fn with_provider(provider: Arc<dyn FineTuneProvider + Send + Sync>) -> Self {
        Self { provider }
    }

    /// Start a fine-tuning job
    pub async fn train(&self, config: TrainingConfig) -> FinetuneResult<TrainingJob> {
        log::info!("🎯 FINETUNE: Starting training for {}", config.base_model);
        self.provider.create_job(config).await
    }

    /// Get job status
    pub async fn status(&self, job_id: &str) -> FinetuneResult<TrainingJob> {
        self.provider.get_job(job_id).await
    }

    /// Cancel a running job
    pub async fn cancel(&self, job_id: &str) -> FinetuneResult<()> {
        self.provider.cancel_job(job_id).await
    }

    /// List all jobs
    pub async fn list_jobs(&self) -> FinetuneResult<Vec<TrainingJob>> {
        self.provider.list_jobs().await
    }

    /// Upload dataset
    pub async fn upload_dataset(&self, dataset: Dataset) -> FinetuneResult<String> {
        self.provider.upload_dataset(dataset).await
    }

    /// Get fine-tuned model
    pub async fn get_model(&self, job_id: &str) -> FinetuneResult<ModelAdapter> {
        self.provider.get_model(job_id).await
    }

    /// Wait for completion
    pub async fn wait_for_completion(
        &self,
        job_id: &str,
        poll_interval_secs: u64,
        timeout_secs: u64,
    ) -> FinetuneResult<TrainingJob> {
        let start = std::time::Instant::now();
        let timeout = std::time::Duration::from_secs(timeout_secs);
        let poll_interval = std::time::Duration::from_secs(poll_interval_secs);

        loop {
            let job = self.status(job_id).await?;

            match job.status {
                TrainingStatus::Succeeded => {
                    log::info!("✅ FINETUNE: Training completed for {}", job.id);
                    return Ok(job);
                }
                TrainingStatus::Failed(ref error) => {
                    return Err(FinetuneError::TrainingFailed(error.clone()));
                }
                TrainingStatus::Cancelled => {
                    return Err(FinetuneError::TrainingFailed("Cancelled".into()));
                }
                _ => {
                    if start.elapsed() > timeout {
                        return Err(FinetuneError::Timeout);
                    }
                    tokio::time::sleep(poll_interval).await;
                }
            }
        }
    }

    /// List supported base models
    pub fn supported_models(&self) -> Vec<BaseModel> {
        self.provider.supported_models()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  BASE MODEL
// ═══════════════════════════════════════════════════════════════════════════════

/// Base model for fine-tuning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseModel {
    /// Model ID
    pub id: String,
    /// Display name
    pub name: String,
    /// Parameter count (e.g., "7B", "70B")
    pub params: String,
    /// Context length
    pub context_length: usize,
    /// Supported fine-tuning methods
    pub methods: Vec<FineTuneMethod>,
    /// Provider
    pub provider: String,
    /// Cost per 1K tokens (training)
    pub cost_per_1k_tokens: f32,
}

impl BaseModel {
    /// OpenAI GPT-3.5 Turbo
    pub fn gpt35_turbo() -> Self {
        Self {
            id: "gpt-3.5-turbo".into(),
            name: "GPT-3.5 Turbo".into(),
            params: "175B".into(),
            context_length: 4096,
            methods: vec![FineTuneMethod::Full],
            provider: "openai".into(),
            cost_per_1k_tokens: 0.008,
        }
    }

    /// OpenAI GPT-4
    pub fn gpt4() -> Self {
        Self {
            id: "gpt-4".into(),
            name: "GPT-4".into(),
            params: "1.7T".into(),
            context_length: 8192,
            methods: vec![FineTuneMethod::Full],
            provider: "openai".into(),
            cost_per_1k_tokens: 0.03,
        }
    }

    /// Llama 2 7B
    pub fn llama2_7b() -> Self {
        Self {
            id: "meta-llama/Llama-2-7b-hf".into(),
            name: "Llama 2 7B".into(),
            params: "7B".into(),
            context_length: 4096,
            methods: vec![FineTuneMethod::Lora, FineTuneMethod::Qlora, FineTuneMethod::Full],
            provider: "together".into(),
            cost_per_1k_tokens: 0.0002,
        }
    }

    /// Llama 2 70B
    pub fn llama2_70b() -> Self {
        Self {
            id: "meta-llama/Llama-2-70b-hf".into(),
            name: "Llama 2 70B".into(),
            params: "70B".into(),
            context_length: 4096,
            methods: vec![FineTuneMethod::Lora, FineTuneMethod::Qlora],
            provider: "together".into(),
            cost_per_1k_tokens: 0.0008,
        }
    }

    /// Mistral 7B
    pub fn mistral_7b() -> Self {
        Self {
            id: "mistralai/Mistral-7B-v0.1".into(),
            name: "Mistral 7B".into(),
            params: "7B".into(),
            context_length: 8192,
            methods: vec![FineTuneMethod::Lora, FineTuneMethod::Qlora, FineTuneMethod::Full],
            provider: "together".into(),
            cost_per_1k_tokens: 0.0002,
        }
    }

    /// CodeLlama 34B
    pub fn codellama_34b() -> Self {
        Self {
            id: "codellama/CodeLlama-34b-hf".into(),
            name: "CodeLlama 34B".into(),
            params: "34B".into(),
            context_length: 16384,
            methods: vec![FineTuneMethod::Lora, FineTuneMethod::Qlora],
            provider: "together".into(),
            cost_per_1k_tokens: 0.0006,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  TRAINING CONFIG BUILDER
// ═══════════════════════════════════════════════════════════════════════════════

/// Builder for training configurations
pub struct TrainingBuilder {
    config: TrainingConfig,
}

impl TrainingBuilder {
    /// Create new builder
    pub fn new(base_model: impl Into<String>, dataset_id: impl Into<String>) -> Self {
        Self {
            config: TrainingConfig {
                base_model: base_model.into(),
                dataset_id: dataset_id.into(),
                ..Default::default()
            },
        }
    }

    /// Set fine-tuning method
    pub fn method(mut self, method: FineTuneMethod) -> Self {
        self.config.method = method;
        self
    }

    /// Set number of epochs
    pub fn epochs(mut self, epochs: u32) -> Self {
        self.config.hyperparameters.num_epochs = epochs;
        self
    }

    /// Set batch size
    pub fn batch_size(mut self, size: u32) -> Self {
        self.config.hyperparameters.batch_size = size;
        self
    }

    /// Set learning rate
    pub fn learning_rate(mut self, lr: f32) -> Self {
        self.config.hyperparameters.learning_rate = lr;
        self
    }

    /// Set LoRA config
    pub fn lora_config(mut self, config: LoraConfig) -> Self {
        self.config.lora_config = Some(config);
        self.config.method = FineTuneMethod::Lora;
        self
    }

    /// Set QLoRA config
    pub fn qlora_config(mut self, config: QloraConfig) -> Self {
        self.config.qlora_config = Some(config);
        self.config.method = FineTuneMethod::Qlora;
        self
    }

    /// Set output model name
    pub fn output_name(mut self, name: impl Into<String>) -> Self {
        self.config.output_name = Some(name.into());
        self
    }

    /// Set validation split
    pub fn validation_split(mut self, split: f32) -> Self {
        self.config.validation_split = split;
        self
    }

    /// Set seed
    pub fn seed(mut self, seed: u64) -> Self {
        self.config.seed = Some(seed);
        self
    }

    /// Set suffix for model name
    pub fn suffix(mut self, suffix: impl Into<String>) -> Self {
        self.config.suffix = Some(suffix.into());
        self
    }

    /// Build the configuration
    pub fn build(self) -> TrainingConfig {
        self.config
    }
}

impl TrainingConfig {
    /// Create a new training configuration
    pub fn new(base_model: impl Into<String>, dataset_id: impl Into<String>) -> Self {
        Self {
            base_model: base_model.into(),
            dataset_id: dataset_id.into(),
            ..Default::default()
        }
    }

    /// Create a builder
    pub fn builder(base_model: impl Into<String>, dataset_id: impl Into<String>) -> TrainingBuilder {
        TrainingBuilder::new(base_model, dataset_id)
    }

    /// Estimate training cost
    pub fn estimate_cost(&self, num_tokens: usize, base_model: &BaseModel) -> f32 {
        let tokens_per_epoch = num_tokens as f32;
        let epochs = self.hyperparameters.num_epochs as f32;
        let total_tokens = tokens_per_epoch * epochs;
        
        (total_tokens / 1000.0) * base_model.cost_per_1k_tokens
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_training_builder() {
        let config = TrainingConfig::builder("llama-2-7b", "dataset-123")
            .method(FineTuneMethod::Lora)
            .epochs(3)
            .batch_size(4)
            .learning_rate(0.0001)
            .output_name("my-model")
            .build();

        assert_eq!(config.base_model, "llama-2-7b");
        assert_eq!(config.dataset_id, "dataset-123");
        assert_eq!(config.method, FineTuneMethod::Lora);
        assert_eq!(config.hyperparameters.num_epochs, 3);
        assert_eq!(config.hyperparameters.batch_size, 4);
    }

    #[test]
    fn test_lora_config_default() {
        let config = LoraConfig::default();
        assert_eq!(config.r, 8);
        assert_eq!(config.alpha, 16);
        assert_eq!(config.dropout, 0.05);
    }

    #[test]
    fn test_qlora_config() {
        let config = QloraConfig {
            bits: 4,
            ..Default::default()
        };
        assert_eq!(config.bits, 4);
        assert!(config.double_quant);
    }

    #[test]
    fn test_base_models() {
        let gpt35 = BaseModel::gpt35_turbo();
        assert_eq!(gpt35.provider, "openai");
        assert!(gpt35.methods.contains(&FineTuneMethod::Full));

        let llama = BaseModel::llama2_7b();
        assert!(llama.methods.contains(&FineTuneMethod::Lora));
        assert!(llama.methods.contains(&FineTuneMethod::Qlora));
    }

    #[test]
    fn test_cost_estimation() {
        let config = TrainingConfig::new("llama-2-7b", "dataset")
            .with_epochs(3);
        
        let model = BaseModel::llama2_7b();
        let cost = config.estimate_cost(100_000, &model);
        
        assert!(cost > 0.0);
    }

    #[test]
    fn test_hyperparameters_default() {
        let hp = Hyperparameters::default();
        assert_eq!(hp.num_epochs, 3);
        assert_eq!(hp.batch_size, 8);
        assert!((hp.learning_rate - 0.00005).abs() < 1e-10);
    }

    #[test]
    fn test_training_status_serialization() {
        let status = TrainingStatus::Running;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, "\"running\"");
    }
}
