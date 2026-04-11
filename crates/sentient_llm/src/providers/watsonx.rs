//! ─── IBM WatsonX Provider ───
//!
//! Implementation for IBM WatsonX API (Granite models)

use async_trait::async_trait;
use futures::{Stream, StreamExt};
use reqwest::Client;
use std::pin::Pin;

use crate::error::{LlmError, LlmResult};
use crate::types::{ChatRequest, ChatResponse, ModelInfo, StreamChunk};
use crate::provider::LlmProvider;

use super::{build_client, parse_api_error};

/// IBM WatsonX provider
pub struct WatsonXProvider {
    client: Client,
    api_key: String,
    project_id: String,
    base_url: String,
}

impl WatsonXProvider {
    pub fn new(api_key: impl Into<String>, project_id: impl Into<String>) -> LlmResult<Self> {
        Ok(Self {
            client: build_client()?,
            api_key: api_key.into(),
            project_id: project_id.into(),
            base_url: "https://us-south.ml.cloud.ibm.com/ml/v1".into(),
        })
    }

    pub fn from_env() -> LlmResult<Self> {
        let api_key = std::env::var("WATSONX_API_KEY")
            .map_err(|_| LlmError::Authentication("WATSONX_API_KEY not set".into()))?;
        let project_id = std::env::var("WATSONX_PROJECT_ID")
            .map_err(|_| LlmError::Authentication("WATSONX_PROJECT_ID not set".into()))?;
        Self::new(api_key, project_id)
    }
    
    async fn get_iam_token(&self) -> LlmResult<String> {
        let response = self.client
            .post("https://iam.cloud.ibm.com/identity/token")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body("grant_type=urn:ibm:params:oauth:grant-type:apikey&apikey=".to_string() + &self.api_key)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(LlmError::Authentication("Failed to get IAM token".into()));
        }

        let json: serde_json::Value = response.json().await?;
        Ok(json["access_token"].as_str().unwrap_or("").to_string())
    }
}

#[async_trait]
impl LlmProvider for WatsonXProvider {
    fn name(&self) -> &str { "IBM WatsonX" }
    fn id(&self) -> &str { "watsonx" }
    
    fn models(&self) -> Vec<ModelInfo> {
        vec![
            // ═══════════════════════════════════════════════════════════
            // IBM GRANITE MODELS
            // ═══════════════════════════════════════════════════════════
            ModelInfo {
                id: "ibm/granite-3.2-8b-instruct".into(),
                name: "Granite 3.2 8B Instruct".into(),
                provider: "IBM WatsonX".into(),
                context_window: 128_000,
                max_output_tokens: 8_192,
                input_cost_per_1k: 0.0001,
                output_cost_per_1k: 0.0001,
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: None,
                quality_rating: 3,
                speed_rating: 5,
                is_reasoning: false,
                free_tier: false,
            },
            ModelInfo {
                id: "ibm/granite-3.1-8b-instruct".into(),
                name: "Granite 3.1 8B Instruct".into(),
                provider: "IBM WatsonX".into(),
                context_window: 128_000,
                max_output_tokens: 8_192,
                input_cost_per_1k: 0.0001,
                output_cost_per_1k: 0.0001,
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: None,
                quality_rating: 3,
                speed_rating: 5,
                is_reasoning: false,
                free_tier: false,
            },
            ModelInfo {
                id: "ibm/granite-13b-chat-v2".into(),
                name: "Granite 13B Chat v2".into(),
                provider: "IBM WatsonX".into(),
                context_window: 8_192,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.0002,
                output_cost_per_1k: 0.0002,
                supports_vision: false,
                supports_tools: false,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: None,
                quality_rating: 3,
                speed_rating: 4,
                is_reasoning: false,
                free_tier: false,
            },
            ModelInfo {
                id: "ibm/granite-20b-code-instruct".into(),
                name: "Granite 20B Code Instruct".into(),
                provider: "IBM WatsonX".into(),
                context_window: 8_192,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.0003,
                output_cost_per_1k: 0.0003,
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: None,
                quality_rating: 4,
                speed_rating: 4,
                is_reasoning: false,
                free_tier: false,
            },
            ModelInfo {
                id: "ibm/granite-34b-code-instruct".into(),
                name: "Granite 34B Code Instruct".into(),
                provider: "IBM WatsonX".into(),
                context_window: 8_192,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.0005,
                output_cost_per_1k: 0.0005,
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: None,
                quality_rating: 4,
                speed_rating: 3,
                is_reasoning: false,
                free_tier: false,
            },
            ModelInfo {
                id: "ibm/granite-3b-code-instruct".into(),
                name: "Granite 3B Code Instruct".into(),
                provider: "IBM WatsonX".into(),
                context_window: 8_192,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.00005,
                output_cost_per_1k: 0.00005,
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: None,
                quality_rating: 3,
                speed_rating: 5,
                is_reasoning: false,
                free_tier: false,
            },
            ModelInfo {
                id: "meta-llama/llama-3-70b-instruct".into(),
                name: "Llama 3 70B (WatsonX)".into(),
                provider: "IBM WatsonX".into(),
                context_window: 8_192,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.0007,
                output_cost_per_1k: 0.0007,
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: None,
                quality_rating: 4,
                speed_rating: 3,
                is_reasoning: false,
                free_tier: false,
            },
            ModelInfo {
                id: "mistralai/mixtral-8x7b-instruct-v0.1".into(),
                name: "Mixtral 8x7B (WatsonX)".into(),
                provider: "IBM WatsonX".into(),
                context_window: 32_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.0006,
                output_cost_per_1k: 0.0006,
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: None,
                quality_rating: 4,
                speed_rating: 3,
                is_reasoning: false,
                free_tier: false,
            },
        ]
    }
    
    fn is_configured(&self) -> bool { !self.api_key.is_empty() && !self.project_id.is_empty() }

    async fn chat(&self, request: ChatRequest) -> LlmResult<ChatResponse> {
        let token = self.get_iam_token().await?;
        
        let response = self.client
            .post(format!("{}/text/chat?version=2024-05-01", self.base_url))
            .header("Authorization", format!("Bearer {}", token))
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({
                "model_id": request.model,
                "project_id": self.project_id,
                "messages": request.messages.iter().map(|m| serde_json::json!({
                    "role": match m.role {
                        crate::types::Role::System => "system",
                        crate::types::Role::User => "user",
                        crate::types::Role::Assistant => "assistant",
                        _ => "user",
                    },
                    "content": m.content.as_text().unwrap_or(""),
                })).collect::<Vec<_>>(),
                "parameters": {
                    "max_new_tokens": request.max_tokens,
                    "temperature": request.temperature.unwrap_or(0.7),
                },
            }))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(LlmError::ApiError(parse_api_error(response).await));
        }

        let json: serde_json::Value = response.json().await?;
        Ok(parse_watsonx_response(json, &request.model))
    }

    async fn chat_stream(
        &self,
        request: ChatRequest,
    ) -> LlmResult<Pin<Box<dyn Stream<Item = LlmResult<StreamChunk>> + Send>>> {
        let token = self.get_iam_token().await?;
        
        let response = self.client
            .post(format!("{}/text/chat_stream?version=2024-05-01", self.base_url))
            .header("Authorization", format!("Bearer {}", token))
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({
                "model_id": request.model,
                "project_id": self.project_id,
                "messages": request.messages.iter().map(|m| serde_json::json!({
                    "role": match m.role {
                        crate::types::Role::System => "system",
                        crate::types::Role::User => "user",
                        crate::types::Role::Assistant => "assistant",
                        _ => "user",
                    },
                    "content": m.content.as_text().unwrap_or(""),
                })).collect::<Vec<_>>(),
            }))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(LlmError::ApiError(parse_api_error(response).await));
        }

        let model = request.model.clone();
        let stream = response.bytes_stream()
            .map(move |result| {
                match result {
                    Ok(bytes) => {
                        let text = String::from_utf8_lossy(&bytes);
                        for line in text.lines() {
                            if line.starts_with("data: ") {
                                let data = &line[6..];
                                if data == "[DONE]" { return Ok(None); }
                                if let Ok(json) = serde_json::from_str::<serde_json::Value>(data) {
                                    return Ok(Some(parse_stream_chunk(json, &model)));
                                }
                            }
                        }
                        Ok(None)
                    }
                    Err(e) => Err(LlmError::StreamError(e.to_string())),
                }
            })
            .filter_map(|result| async move { result.transpose() });

        Ok(Box::pin(stream))
    }

    fn count_tokens(&self, text: &str, _model: &str) -> LlmResult<usize> {
        Ok(text.len() / 4)
    }
}

fn parse_watsonx_response(json: serde_json::Value, model: &str) -> ChatResponse {
    ChatResponse {
        id: json["id"].as_str().unwrap_or("").to_string(),
        object: "chat.completion".to_string(),
        created: chrono::Utc::now().timestamp(),
        model: model.to_string(),
        choices: json["choices"].as_array().map(|arr| {
            arr.iter().map(|c| crate::types::Choice {
                index: c["index"].as_u64().unwrap_or(0) as u32,
                message: crate::types::Message::assistant(
                    c["message"]["content"].as_str().unwrap_or("")
                ),
                finish_reason: c["finish_reason"].as_str().map(|s| s.to_string()),
                logprobs: None,
            }).collect()
        }).unwrap_or_default(),
        usage: crate::types::Usage {
            prompt_tokens: json["usage"]["prompt_tokens"].as_u64().unwrap_or(0) as u32,
            completion_tokens: json["usage"]["completion_tokens"].as_u64().unwrap_or(0) as u32,
            total_tokens: json["usage"]["total_tokens"].as_u64().unwrap_or(0) as u32,
        },
        system_fingerprint: None,
    }
}

fn parse_stream_chunk(json: serde_json::Value, model: &str) -> StreamChunk {
    StreamChunk {
        id: json["id"].as_str().unwrap_or("").to_string(),
        object: "chat.completion.chunk".to_string(),
        created: chrono::Utc::now().timestamp(),
        model: model.to_string(),
        system_fingerprint: None,
        choices: json["choices"].as_array().map(|arr| {
            arr.iter().map(|c| crate::types::StreamChoice {
                index: c["index"].as_u64().unwrap_or(0) as u32,
                delta: crate::types::Delta {
                    role: Some(crate::types::Role::Assistant),
                    content: c["delta"]["content"].as_str().map(|s| s.to_string()),
                    tool_calls: None,
                },
                finish_reason: c["finish_reason"].as_str().map(|s| s.to_string()),
            }).collect()
        }).unwrap_or_default(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_watsonx_provider() {
        let provider = WatsonXProvider::new("test-key", "test-project");
        assert!(provider.is_ok());
    }

    #[test]
    fn test_watsonx_models() {
        let provider = WatsonXProvider::new("test-key", "test-project").unwrap();
        let models = provider.models();
        assert!(models.len() >= 8, "WatsonX should have at least 8 models");
        assert!(models.iter().any(|m| m.id.contains("granite")));
        assert!(models.iter().any(|m| m.id.contains("llama")));
    }
}
