//! ─── Modal Provider ───
//!
//! Implementation for Modal Serverless API
//! API Docs: https://modal.com/docs

use async_trait::async_trait;
use futures::{Stream, StreamExt};
use reqwest::Client;
use std::pin::Pin;

use crate::error::{LlmError, LlmResult};
use crate::types::{ChatRequest, ChatResponse, ModelInfo, StreamChunk};
use crate::provider::LlmProvider;

use super::{build_client, parse_api_error};

/// Modal provider
pub struct ModalProvider {
    client: Client,
    api_key: String,
    base_url: String,
}

impl ModalProvider {
    pub fn new(api_key: impl Into<String>) -> LlmResult<Self> {
        Ok(Self {
            client: build_client()?,
            api_key: api_key.into(),
            base_url: "https://api.modal.ai/v1".into(),
        })
    }

    pub fn from_env() -> LlmResult<Self> {
        let api_key = std::env::var("MODAL_API_KEY")
            .map_err(|_| LlmError::Authentication("MODAL_API_KEY not set".into()))?;
        Self::new(api_key)
    }
}

#[async_trait]
impl LlmProvider for ModalProvider {
    fn name(&self) -> &str { "Modal" }
    fn id(&self) -> &str { "modal" }

    fn models(&self) -> Vec<ModelInfo> {
        vec![
            // ═══════════════════════════════════════════════════════════
            // LLAMA 3.3 - Latest Llama
            // ═══════════════════════════════════════════════════════════
            ModelInfo {
                id: "llama-3.3-70b".into(),
                name: "Llama 3.3 70B (Modal)".into(),
                provider: "Modal".into(),
                context_window: 128_000,
                max_output_tokens: 8_192,
                input_cost_per_1k: 0.0006,
                output_cost_per_1k: 0.0008,
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: None,
                quality_rating: 5,
                speed_rating: 3,
                is_reasoning: false,
                free_tier: false,
            },
            // ═══════════════════════════════════════════════════════════
            // LLAMA 3.1 405B - Largest Open Model
            // ═══════════════════════════════════════════════════════════
            ModelInfo {
                id: "llama-3.1-405b".into(),
                name: "Llama 3.1 405B (Modal)".into(),
                provider: "Modal".into(),
                context_window: 128_000,
                max_output_tokens: 16_384,
                input_cost_per_1k: 0.002,
                output_cost_per_1k: 0.003,
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: None,
                quality_rating: 5,
                speed_rating: 2,
                is_reasoning: false,
                free_tier: false,
            },
            // ═══════════════════════════════════════════════════════════
            // MIXTRAL 8x22B - Large MoE
            // ═══════════════════════════════════════════════════════════
            ModelInfo {
                id: "mixtral-8x22b".into(),
                name: "Mixtral 8x22B (Modal)".into(),
                provider: "Modal".into(),
                context_window: 65_536,
                max_output_tokens: 8_192,
                input_cost_per_1k: 0.0009,
                output_cost_per_1k: 0.0012,
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: None,
                quality_rating: 5,
                speed_rating: 3,
                is_reasoning: false,
                free_tier: false,
            },
        ]
    }

    fn is_configured(&self) -> bool { !self.api_key.is_empty() }

    async fn chat(&self, request: ChatRequest) -> LlmResult<ChatResponse> {
        let response = self.client
            .post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({
                "model": request.model,
                "messages": request.messages.iter().map(|m| serde_json::json!({
                    "role": match m.role {
                        crate::types::Role::System => "system",
                        crate::types::Role::User => "user",
                        crate::types::Role::Assistant => "assistant",
                        _ => "user",
                    },
                    "content": m.content.as_text().unwrap_or(""),
                })).collect::<Vec<_>>(),
                "max_tokens": request.max_tokens,
                "temperature": request.temperature,
            }))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(LlmError::ApiError(parse_api_error(response).await));
        }

        let json: serde_json::Value = response.json().await?;
        Ok(parse_modal_response(json))
    }

    async fn chat_stream(
        &self,
        request: ChatRequest,
    ) -> LlmResult<Pin<Box<dyn Stream<Item = LlmResult<StreamChunk>> + Send>>> {
        let response = self.client
            .post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({
                "model": request.model,
                "messages": request.messages.iter().map(|m| serde_json::json!({
                    "role": match m.role {
                        crate::types::Role::System => "system",
                        crate::types::Role::User => "user",
                        crate::types::Role::Assistant => "assistant",
                        _ => "user",
                    },
                    "content": m.content.as_text().unwrap_or(""),
                })).collect::<Vec<_>>(),
                "stream": true,
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
                                    return Ok(Some(parse_modal_stream_chunk(json, &model)));
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

fn parse_modal_response(json: serde_json::Value) -> ChatResponse {
    ChatResponse {
        id: json["id"].as_str().unwrap_or("").to_string(),
        object: json["object"].as_str().unwrap_or("chat.completion").to_string(),
        created: json["created"].as_i64().unwrap_or(0),
        model: json["model"].as_str().unwrap_or("").to_string(),
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

fn parse_modal_stream_chunk(json: serde_json::Value, model: &str) -> StreamChunk {
    StreamChunk {
        id: json["id"].as_str().unwrap_or("").to_string(),
        object: json["object"].as_str().unwrap_or("chat.completion.chunk").to_string(),
        created: json["created"].as_i64().unwrap_or(0),
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
    fn test_modal_provider() {
        let provider = ModalProvider::new("test-api-key");
        assert!(provider.is_ok());
    }

    #[test]
    fn test_modal_models() {
        let provider = ModalProvider::new("test-key").unwrap();
        let models = provider.models();
        assert_eq!(models.len(), 3, "Modal should have 3 models");
        assert!(models.iter().any(|m| m.id == "llama-3.3-70b"));
        assert!(models.iter().any(|m| m.id == "llama-3.1-405b"));
        assert!(models.iter().any(|m| m.id == "mixtral-8x22b"));
    }

    #[test]
    fn test_modal_context_windows() {
        let provider = ModalProvider::new("test-key").unwrap();
        let models = provider.models();

        let llama33 = models.iter().find(|m| m.id == "llama-3.3-70b").unwrap();
        assert_eq!(llama33.context_window, 128_000);

        let llama405 = models.iter().find(|m| m.id == "llama-3.1-405b").unwrap();
        assert_eq!(llama405.context_window, 128_000);

        let mixtral = models.iter().find(|m| m.id == "mixtral-8x22b").unwrap();
        assert_eq!(mixtral.context_window, 65_536);
    }

    #[test]
    fn test_modal_quality_ratings() {
        let provider = ModalProvider::new("test-key").unwrap();
        let models = provider.models();

        // All Modal models should have high quality ratings
        for model in &models {
            assert!(model.quality_rating >= 4);
        }
    }

    #[test]
    fn test_modal_405b_is_largest() {
        let provider = ModalProvider::new("test-key").unwrap();
        let models = provider.models();

        let llama405 = models.iter().find(|m| m.id == "llama-3.1-405b").unwrap();
        // 405B should have highest cost
        assert!(llama405.input_cost_per_1k > 0.001);
    }
}
