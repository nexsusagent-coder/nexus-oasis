//! ─── Fine-tuning Providers ───

pub mod openai;
pub mod together;
pub mod local;

pub use openai::OpenAIProvider;
pub use together::TogetherProvider;
pub use local::LocalProvider;

use async_trait::async_trait;
use crate::{
    TrainingConfig, TrainingJob, ModelAdapter, BaseModel,
    Dataset, FinetuneResult,
};

// ═══════════════════════════════════════════════════════════════════════════════
//  FINETUNE PROVIDER TRAIT
// ═══════════════════════════════════════════════════════════════════════════════

/// Trait for fine-tuning providers
#[async_trait]
pub trait FineTuneProvider {
    /// Create a fine-tuning job
    async fn create_job(&self, config: TrainingConfig) -> FinetuneResult<TrainingJob>;

    /// Get job status
    async fn get_job(&self, job_id: &str) -> FinetuneResult<TrainingJob>;

    /// Cancel a job
    async fn cancel_job(&self, job_id: &str) -> FinetuneResult<()>;

    /// List all jobs
    async fn list_jobs(&self) -> FinetuneResult<Vec<TrainingJob>>;

    /// Upload dataset
    async fn upload_dataset(&self, dataset: Dataset) -> FinetuneResult<String>;

    /// Get fine-tuned model
    async fn get_model(&self, job_id: &str) -> FinetuneResult<ModelAdapter>;

    /// List supported base models
    fn supported_models(&self) -> Vec<BaseModel>;
}

// ═══════════════════════════════════════════════════════════════════════════════
//  FINETUNER FACADE
// ═══════════════════════════════════════════════════════════════════════════════

/// High-level fine-tuner
pub struct FineTuner;

impl FineTuner {
    /// Create OpenAI provider
    pub fn openai(api_key: impl Into<String>) -> OpenAIProvider {
        OpenAIProvider::new(api_key.into())
    }

    /// Create Together AI provider
    pub fn together(api_key: impl Into<String>) -> TogetherProvider {
        TogetherProvider::new(api_key.into())
    }

    /// Create local provider
    pub fn local() -> LocalProvider {
        LocalProvider::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  COMMON HELPERS
// ═══════════════════════════════════════════════════════════════════════════════

fn build_client() -> reqwest::Client {
    reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(300))
        .build()
        .expect("Failed to create HTTP client")
}

fn parse_api_error(response: reqwest::Response) -> String {
    format!("HTTP {} - {}", response.status(), response.url())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_finetuner_constructors() {
        let _openai = FineTuner::openai("test-key");
        let _together = FineTuner::together("test-key");
        let _local = FineTuner::local();
    }
}
