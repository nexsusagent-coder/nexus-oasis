//! ─── OpenAI Fine-tuning Provider ───
//!
//! OpenAI fine-tuning API
//! API Docs: https://platform.openai.com/docs/api-reference/fine-tuning

use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use chrono::Utc;

use crate::{
    TrainingConfig, TrainingJob, TrainingStatus, ModelAdapter,
    BaseModel, Dataset, FinetuneResult, FinetuneError,
    FineTuneMethod,
};
use super::{FineTuneProvider, build_client, parse_api_error};

// ═══════════════════════════════════════════════════════════════════════════════
//  OPENAI PROVIDER
// ═══════════════════════════════════════════════════════════════════════════════

pub struct OpenAIProvider {
    client: Client,
    api_key: String,
    base_url: String,
}

impl OpenAIProvider {
    pub fn new(api_key: String) -> Self {
        Self {
            client: build_client(),
            api_key,
            base_url: "https://api.openai.com/v1".into(),
        }
    }

    async fn upload_file(&self, content: &str, filename: &str) -> FinetuneResult<String> {
        let response = self.client
            .post(format!("{}/files", self.base_url))
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

    async fn get_job_status(&self, job_id: &str) -> FinetuneResult<OpenAIJobResponse> {
        let response = self.client
            .get(format!("{}/fine_tuning/jobs/{}", self.base_url, job_id))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(FinetuneError::ApiError(parse_api_error(response)));
        }

        response.json::<OpenAIJobResponse>().await.map_err(Into::into)
    }
}

#[async_trait]
impl FineTuneProvider for OpenAIProvider {
    async fn create_job(&self, config: TrainingConfig) -> FinetuneResult<TrainingJob> {
        // OpenAI only supports Full fine-tuning
        if config.method != FineTuneMethod::Full {
            return Err(FinetuneError::UnsupportedMethod(
                config.method.display_name().into(),
                config.base_model.clone()
            ));
        }

        let request = OpenAIRequest {
            model: config.base_model.clone(),
            training_file: config.dataset_id.clone(),
            hyperparameters: OpenAIHyperparams {
                n_epochs: config.hyperparameters.num_epochs,
                batch_size: Some(config.hyperparameters.batch_size),
                learning_rate_multiplier: Some(config.hyperparameters.learning_rate),
            },
            suffix: config.suffix.clone(),
        };

        let response = self.client
            .post(format!("{}/fine_tuning/jobs", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(FinetuneError::ApiError(parse_api_error(response)));
        }

        let result = response.json::<OpenAIJobResponse>().await?;
        
        Ok(TrainingJob {
            id: result.id,
            base_model: result.model,
            dataset_id: config.dataset_id,
            status: map_openai_status(&result.status),
            created_at: chrono::DateTime::from_timestamp(result.created_at, 0)
                .unwrap_or_else(Utc::now),
            updated_at: Utc::now(),
            fine_tuned_model: result.fine_tuned_model,
            metrics: None,
            error: result.error.map(|e| e.message),
            estimated_completion: None,
            checkpoints: vec![],
        })
    }

    async fn get_job(&self, job_id: &str) -> FinetuneResult<TrainingJob> {
        let result = self.get_job_status(job_id).await?;
        
        Ok(TrainingJob {
            id: result.id.clone(),
            base_model: result.model,
            dataset_id: String::new(),
            status: map_openai_status(&result.status),
            created_at: chrono::DateTime::from_timestamp(result.created_at, 0)
                .unwrap_or_else(Utc::now),
            updated_at: Utc::now(),
            fine_tuned_model: result.fine_tuned_model,
            metrics: None,
            error: result.error.map(|e| e.message),
            estimated_completion: None,
            checkpoints: vec![],
        })
    }

    async fn cancel_job(&self, job_id: &str) -> FinetuneResult<()> {
        let response = self.client
            .post(format!("{}/fine_tuning/jobs/{}/cancel", self.base_url, job_id))
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
            .get(format!("{}/fine_tuning/jobs", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(FinetuneError::ApiError(parse_api_error(response)));
        }

        let result = response.json::<OpenAIListResponse>().await?;
        
        Ok(result.data.into_iter().map(|j| TrainingJob {
            id: j.id,
            base_model: j.model,
            dataset_id: String::new(),
            status: map_openai_status(&j.status),
            created_at: chrono::DateTime::from_timestamp(j.created_at, 0)
                .unwrap_or_else(Utc::now),
            updated_at: Utc::now(),
            fine_tuned_model: j.fine_tuned_model,
            metrics: None,
            error: j.error.map(|e| e.message),
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
                method: FineTuneMethod::Full,
                created_at: job.created_at,
                size_bytes: 0,
                download_url: None,
                hf_repo_id: None,
            }),
            None => Err(FinetuneError::TrainingFailed("Model not ready".into())),
        }
    }

    fn supported_models(&self) -> Vec<BaseModel> {
        vec![BaseModel::gpt35_turbo(), BaseModel::gpt4()]
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  OPENAI API TYPES
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Serialize)]
struct OpenAIRequest {
    model: String,
    training_file: String,
    hyperparameters: OpenAIHyperparams,
    #[serde(skip_serializing_if = "Option::is_none")]
    suffix: Option<String>,
}

#[derive(Debug, Serialize)]
struct OpenAIHyperparams {
    n_epochs: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    batch_size: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    learning_rate_multiplier: Option<f32>,
}

#[derive(Debug, Deserialize)]
struct FileResponse {
    id: String,
}

#[derive(Debug, Deserialize)]
struct OpenAIJobResponse {
    id: String,
    model: String,
    status: String,
    created_at: i64,
    #[serde(default)]
    fine_tuned_model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<OpenAIError>,
}

#[derive(Debug, Deserialize)]
struct OpenAIError {
    message: String,
}

#[derive(Debug, Deserialize)]
struct OpenAIListResponse {
    data: Vec<OpenAIJobResponse>,
}

fn map_openai_status(status: &str) -> TrainingStatus {
    match status {
        "validating_files" => TrainingStatus::Validating,
        "queued" => TrainingStatus::Queued,
        "running" => TrainingStatus::Running,
        "succeeded" => TrainingStatus::Succeeded,
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
        let provider = OpenAIProvider::new("test-key".to_string());
        let models = provider.supported_models();
        
        assert_eq!(models.len(), 2);
    }

    #[test]
    fn test_status_mapping() {
        assert_eq!(map_openai_status("running"), TrainingStatus::Running);
        assert_eq!(map_openai_status("succeeded"), TrainingStatus::Succeeded);
        assert_eq!(map_openai_status("failed"), TrainingStatus::Failed("Unknown error".into()));
    }
}
