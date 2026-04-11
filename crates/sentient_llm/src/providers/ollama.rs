//! ─── Ollama Provider ───
//!
//! Implementation for Ollama API (local LLM inference)

use async_trait::async_trait;
use futures::{Stream, StreamExt};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::pin::Pin;

use crate::error::{LlmError, LlmResult};
use crate::types::{ChatRequest, ChatResponse, ModelInfo, StreamChunk, Role};
use crate::provider::LlmProvider;

use super::build_client;

/// Ollama provider - local LLM inference
pub struct OllamaProvider {
    client: Client,
    base_url: String,
}

impl OllamaProvider {
    pub fn new() -> LlmResult<Self> {
        Ok(Self {
            client: build_client()?,
            base_url: std::env::var("OLLAMA_HOST")
                .unwrap_or_else(|_| "http://localhost:11434".into()),
        })
    }

    pub fn with_base_url(base_url: impl Into<String>) -> LlmResult<Self> {
        Ok(Self {
            client: build_client()?,
            base_url: base_url.into(),
        })
    }

    fn convert_request(&self, request: ChatRequest) -> OllamaRequest {
        OllamaRequest {
            model: request.model,
            messages: request.messages.into_iter().map(|m| OllamaMessage {
                role: match m.role {
                    Role::System => "system".into(),
                    Role::User => "user".into(),
                    Role::Assistant => "assistant".into(),
                    _ => "user".into(),
                },
                content: m.content.as_text().map(|s| s.to_string()).unwrap_or_default(),
            }).collect(),
            stream: request.stream,
            options: Some(OllamaOptions {
                temperature: request.temperature,
                num_predict: request.max_tokens,
                top_p: request.top_p,
            }),
        }
    }

    /// Check if Ollama is running
    pub async fn is_running(&self) -> bool {
        self.client
            .get(format!("{}/api/tags", self.base_url))
            .send()
            .await
            .map(|r| r.status().is_success())
            .unwrap_or(false)
    }

    /// List available models
    pub async fn list_models(&self) -> LlmResult<Vec<String>> {
        let response = self.client
            .get(format!("{}/api/tags", self.base_url))
            .send()
            .await
            .map_err(|e| LlmError::Network(e.to_string()))?;

        if !response.status().is_success() {
            return Err(LlmError::ApiError(format!("Ollama not running at {}", self.base_url)));
        }

        let models: OllamaModels = response.json().await
            .map_err(|e| LlmError::ParseError(e.to_string()))?;

        Ok(models.models.into_iter().map(|m| m.name).collect())
    }

    /// Pull a model
    pub async fn pull_model(&self, model: &str) -> LlmResult<()> {
        let response = self.client
            .post(format!("{}/api/pull", self.base_url))
            .json(&serde_json::json!({ "name": model }))
            .send()
            .await
            .map_err(|e| LlmError::Network(e.to_string()))?;

        if !response.status().is_success() {
            return Err(LlmError::ApiError("Failed to pull model".into()));
        }

        Ok(())
    }
}

#[async_trait]
impl LlmProvider for OllamaProvider {
    fn name(&self) -> &str { "Ollama" }
    fn id(&self) -> &str { "ollama" }
    
    fn models(&self) -> Vec<ModelInfo> {
        // Return popular Ollama models
        vec![
            ModelInfo {
                id: "llama3.2".into(),
                name: "Llama 3.2".into(),
                provider: "Ollama".into(),
                context_window: 128_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.0,
                output_cost_per_1k: 0.0,
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: None,
                quality_rating: 4,
                speed_rating: 5,
                is_reasoning: false,
                free_tier: true,
            },
            ModelInfo {
                id: "llama3.3".into(),
                name: "Llama 3.3 70B".into(),
                provider: "Ollama".into(),
                context_window: 128_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.0,
                output_cost_per_1k: 0.0,
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: None,
                quality_rating: 5,
                speed_rating: 3,
                is_reasoning: false,
                free_tier: true,
            },
            ModelInfo {
                id: "deepseek-r1".into(),
                name: "DeepSeek R1".into(),
                provider: "Ollama".into(),
                context_window: 128_000,
                max_output_tokens: 8_192,
                input_cost_per_1k: 0.0,
                output_cost_per_1k: 0.0,
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: None,
                quality_rating: 5,
                speed_rating: 3,
                is_reasoning: true,
                free_tier: true,
            },
            ModelInfo {
                id: "qwen2.5".into(),
                name: "Qwen 2.5".into(),
                provider: "Ollama".into(),
                context_window: 128_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.0,
                output_cost_per_1k: 0.0,
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: None,
                quality_rating: 4,
                speed_rating: 4,
                is_reasoning: false,
                free_tier: true,
            },
            ModelInfo {
                id: "mistral".into(),
                name: "Mistral".into(),
                provider: "Ollama".into(),
                context_window: 32_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.0,
                output_cost_per_1k: 0.0,
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: None,
                quality_rating: 4,
                speed_rating: 5,
                is_reasoning: false,
                free_tier: true,
            },
            ModelInfo {
                id: "codellama".into(),
                name: "Code Llama".into(),
                provider: "Ollama".into(),
                context_window: 16_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.0,
                output_cost_per_1k: 0.0,
                supports_vision: false,
                supports_tools: false,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: None,
                quality_rating: 4,
                speed_rating: 4,
                is_reasoning: false,
                free_tier: true,
            },
        ]
    }

    fn is_configured(&self) -> bool {
        true // Ollama doesn't need API key
    }

    async fn chat(&self, request: ChatRequest) -> LlmResult<ChatResponse> {
        let ollama_request = self.convert_request(request);

        let response = self.client
            .post(format!("{}/api/chat", self.base_url))
            .json(&ollama_request)
            .send()
            .await
            .map_err(|e| LlmError::Network(e.to_string()))?;

        if !response.status().is_success() {
            return Err(LlmError::ApiError("Ollama request failed".into()));
        }

        let ollama_response: OllamaResponse = response.json().await
            .map_err(|e| LlmError::ParseError(e.to_string()))?;

        Ok(ChatResponse {
            id: uuid::Uuid::new_v4().to_string(),
            object: "chat.completion".into(),
            created: chrono::Utc::now().timestamp(),
            model: ollama_response.model,
            choices: vec![crate::types::Choice {
                index: 0,
                message: crate::types::Message::assistant(&ollama_response.message.content),
                finish_reason: Some("stop".into()),
                logprobs: None,
            }],
            usage: crate::types::Usage {
                prompt_tokens: ollama_response.prompt_eval_count.unwrap_or(0),
                completion_tokens: ollama_response.eval_count.unwrap_or(0),
                total_tokens: ollama_response.prompt_eval_count.unwrap_or(0) + ollama_response.eval_count.unwrap_or(0),
            },
            system_fingerprint: None,
        })
    }

    async fn chat_stream(
        &self,
        request: ChatRequest,
    ) -> LlmResult<Pin<Box<dyn Stream<Item = LlmResult<StreamChunk>> + Send>>> {
        let mut ollama_request = self.convert_request(request);
        ollama_request.stream = true;

        let response = self.client
            .post(format!("{}/api/chat", self.base_url))
            .json(&ollama_request)
            .send()
            .await
            .map_err(|e| LlmError::Network(e.to_string()))?;

        let model = ollama_request.model.clone();
        let stream = response.bytes_stream()
            .map(move |result| {
                match result {
                    Ok(bytes) => {
                        let text = String::from_utf8_lossy(&bytes);
                        // Ollama streams JSON objects, one per line
                        for line in text.lines() {
                            if let Ok(chunk) = serde_json::from_str::<OllamaStreamResponse>(line) {
                                return Ok(Some(StreamChunk {
                                    id: uuid::Uuid::new_v4().to_string(),
                                    object: "chat.completion.chunk".into(),
                                    created: chrono::Utc::now().timestamp(),
                                    model: model.clone(),
                                    system_fingerprint: None,
                                    choices: vec![crate::types::StreamChoice {
                                        index: 0,
                                        delta: crate::types::Delta {
                                            role: Some(Role::Assistant),
                                            content: Some(chunk.message.content),
                                            tool_calls: None,
                                        },
                                        finish_reason: if chunk.done { Some("stop".into()) } else { None },
                                    }],
                                }));
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

// ═══════════════════════════════════════════════════════════════════════════════
//  OLLAMA API TYPES
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Serialize)]
struct OllamaRequest {
    model: String,
    messages: Vec<OllamaMessage>,
    stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<OllamaOptions>,
}

#[derive(Debug, Serialize)]
struct OllamaMessage {
    role: String,
    content: String,
}

#[derive(Debug, Serialize)]
struct OllamaOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    num_predict: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
}

#[derive(Debug, Deserialize)]
struct OllamaResponse {
    model: String,
    message: OllamaMessageResponse,
    prompt_eval_count: Option<u32>,
    eval_count: Option<u32>,
}

#[derive(Debug, Deserialize)]
struct OllamaMessageResponse {
    content: String,
}

#[derive(Debug, Deserialize)]
struct OllamaStreamResponse {
    message: OllamaMessageResponse,
    done: bool,
}

#[derive(Debug, Deserialize)]
struct OllamaModels {
    models: Vec<OllamaModel>,
}

#[derive(Debug, Deserialize)]
struct OllamaModel {
    name: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ollama_provider() {
        let provider = OllamaProvider::new();
        assert!(provider.is_ok());
    }

    #[test]
    fn test_ollama_models() {
        let provider = OllamaProvider::new().unwrap();
        let models = provider.models();
        assert!(!models.is_empty());
        assert!(models.iter().all(|m| m.free_tier));
        assert!(models.iter().all(|m| m.input_cost_per_1k == 0.0));
    }
}
