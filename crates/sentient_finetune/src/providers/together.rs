//! ─── Together AI Fine-tuning Provider ───
//!
//! Together AI fine-tuning API
//! Supports LoRA and QLoRA
//! API Docs: https://docs.together.ai/docs/fine-tuning

use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use chrono::Utc;

use crate::{
    TrainingConfig, TrainingJob, TrainingStatus, TrainingMetrics,
    ModelAdapter, BaseModel, Dataset, FinetuneResult, FinetuneError,
    FineTuneMethod,
};
use super::{FineTuneProvider, build_client, parse_api_error};

// ═══════════════════════════════════════════════════════════════════════════════
//  TOGETHER PROVIDER
// ═══════════════════════════════════════════════════════════════════════════════

pub struct TogetherProvider {
    client: Client,
    api_key: String,
    base_url: String,
}

impl TogetherProvider {
    pub fn new(api_key: String) -> Self {
        Self {
            client: build_client(),
            api_key,
            base_url: "https://api.together.xyz/v1".into(),
        }
    }

    async fn upload_file(&self, content: &str, filename: &str) -> FinetuneResult<String> {
        let response = self.client
            .post(format!("{}/files/upload", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .multipart(
                reqwest::multipart::Form::new()
                    .text("purpose", "fine-tune")
                    .part("file", reqwest::multipart::Part::text(content.to_string())
                        .file_name(filename.to_string()))
            )
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(FinetuneError::ApiError(parse_api_error(response)));
        }

        let result = response.json::<FileResponse>().await?;
        Ok(result.id)
    }
}

#[async_trait]
impl FineTuneProvider for TogetherProvider {
    async fn create_job(&self, config: TrainingConfig) -> FinetuneResult<TrainingJob> {
        let method_str = match config.method {
            FineTuneMethod::Lora => "lora",
            FineTuneMethod::Qlora => "qlora",
            FineTuneMethod::Full => "full",
            _ => "lora",
        };

        let request = TogetherRequest {
            training_file: config.dataset_id.clone(),
            model: config.base_model.clone(),
            method: method_str.into(),
            hyperparameters: TogetherHyperparams {
                n_epochs: config.hyperparameters.num_epochs,
                batch_size: config.hyperparameters.batch_size,
                learning_rate: config.hyperparameters.learning_rate,
            },
            lora_config: config.lora_config.as_ref().map(|l| TogetherLoraConfig {
                r: l.r,
                alpha: l.alpha,
                dropout: l.dropout,
                target_modules: l.target_modules.clone(),
            }),
            suffix: config.suffix.clone(),
        };

        let response = self.client
            .post(format!("{}/fine-tunes", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(FinetuneError::ApiError(parse_api_error(response)));
        }

        let result = response.json::<TogetherJobResponse>().await?;
        
        Ok(TrainingJob {
            id: result.id,
            base_model: result.model,
            dataset_id: config.dataset_id,
            status: map_together_status(&result.status),
            created_at: chrono::DateTime::from_timestamp(result.created_at, 0)
                .unwrap_or_else(Utc::now),
            updated_at: Utc::now(),
            fine_tuned_model: result.output_model,
            metrics: None,
            error: result.error,
            estimated_completion: None,
            checkpoints: vec![],
        })
    }

    async fn get_job(&self, job_id: &str) -> FinetuneResult<TrainingJob> {
        let response = self.client
            .get(format!("{}/fine-tunes/{}", self.base_url, job_id))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(FinetuneError::ApiError(parse_api_error(response)));
        }

        let result = response.json::<TogetherJobResponse>().await?;
        
        Ok(TrainingJob {
            id: result.id.clone(),
            base_model: result.model,
            dataset_id: String::new(),
            status: map_together_status(&result.status),
            created_at: chrono::DateTime::from_timestamp(result.created_at, 0)
                .unwrap_or_else(Utc::now),
            updated_at: Utc::now(),
            fine_tuned_model: result.output_model,
            metrics: result.train_loss.map(|loss| TrainingMetrics {
                epoch: result.epoch.unwrap_or(0.0),
                step: result.step.unwrap_or(0),
                total_steps: 0,
                train_loss: loss,
                val_loss: None,
                learning_rate: 0.0,
                tokens_processed: 0,
                tokens_per_second: None,
                gpu_memory_gb: None,
                estimated_time_remaining: None,
            }),
            error: result.error,
            estimated_completion: None,
            checkpoints: vec![],
        })
    }

    async fn cancel_job(&self, job_id: &str) -> FinetuneResult<()> {
        let response = self.client
            .post(format!("{}/fine-tunes/{}/cancel", self.base_url, job_id))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(FinetuneError::ApiError(parse_api_error(response)));
        }

        Ok(())
    }

    async fn list_jobs(&self) -> FinetuneResult<Vec<TrainingJob>> {
        let response = self.client
            .get(format!("{}/fine-tunes", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(FinetuneError::ApiError(parse_api_error(response)));
        }

        let result = response.json::<TogetherListResponse>().await?;
        
        Ok(result.data.into_iter().map(|j| TrainingJob {
            id: j.id,
            base_model: j.model,
            dataset_id: String::new(),
            status: map_together_status(&j.status),
            created_at: chrono::DateTime::from_timestamp(j.created_at, 0)
                .unwrap_or_else(Utc::now),
            updated_at: Utc::now(),
            fine_tuned_model: j.output_model,
            metrics: None,
            error: j.error,
            estimated_completion: None,
            checkpoints: vec![],
        }).collect())
    }

    async fn upload_dataset(&self, dataset: Dataset) -> FinetuneResult<String> {
        dataset.validate()?;
        let jsonl = dataset.to_jsonl()?;
        self.upload_file(&jsonl, &format!("{}.jsonl", dataset.name)).await
    }

    async fn get_model(&self, job_id: &str) -> FinetuneResult<ModelAdapter> {
        let job = self.get_job(job_id).await?;
        
        match job.fine_tuned_model {
            Some(model_id) => Ok(ModelAdapter {
                id: model_id.clone(),
                base_model: job.base_model,
                name: model_id,
                method: FineTuneMethod::Lora,
                created_at: job.created_at,
                size_bytes: 0,
                download_url: None,
                hf_repo_id: None,
            }),
            None => Err(FinetuneError::TrainingFailed("Model not ready".into())),
        }
    }

    fn supported_models(&self) -> Vec<BaseModel> {
        vec![
            BaseModel::llama2_7b(),
            BaseModel::llama2_70b(),
            BaseModel::mistral_7b(),
            BaseModel::codellama_34b(),
        ]
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  TOGETHER API TYPES
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Serialize)]
struct TogetherRequest {
    training_file: String,
    model: String,
    method: String,
    hyperparameters: TogetherHyperparams,
    #[serde(skip_serializing_if = "Option::is_none")]
    lora_config: Option<TogetherLoraConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    suffix: Option<String>,
}

#[derive(Debug, Serialize)]
struct TogetherHyperparams {
    n_epochs: u32,
    batch_size: u32,
    learning_rate: f32,
}

#[derive(Debug, Serialize)]
struct TogetherLoraConfig {
    r: u8,
    alpha: u16,
    dropout: f32,
    target_modules: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct FileResponse {
    id: String,
}

#[derive(Debug, Deserialize)]
struct TogetherJobResponse {
    id: String,
    model: String,
    status: String,
    created_at: i64,
    #[serde(default)]
    output_model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    train_loss: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    epoch: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    step: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

#[derive(Debug, Deserialize)]
struct TogetherListResponse {
    data: Vec<TogetherJobResponse>,
}

fn map_together_status(status: &str) -> TrainingStatus {
    match status {
        "pending" => TrainingStatus::Pending,
        "validating" => TrainingStatus::Validating,
        "queued" => TrainingStatus::Queued,
        "running" => TrainingStatus::Running,
        "completed" => TrainingStatus::Succeeded,
        "failed" => TrainingStatus::Failed("Unknown error".into()),
        "cancelled" => TrainingStatus::Cancelled,
        _ => TrainingStatus::Pending,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_supported_models() {
        let provider = TogetherProvider::new("test-key".to_string());
        let models = provider.supported_models();
        
        assert_eq!(models.len(), 4);
        assert!(models.iter().any(|m| m.id.contains("llama")));
    }

    #[test]
    fn test_status_mapping() {
        assert_eq!(map_together_status("running"), TrainingStatus::Running);
        assert_eq!(map_together_status("completed"), TrainingStatus::Succeeded);
    }
}
