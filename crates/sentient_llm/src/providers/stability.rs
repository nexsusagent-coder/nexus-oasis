//! ─── Stability AI Provider ───
//!
//! Implementation for Stability AI API (StableLM, Stable Diffusion)

use async_trait::async_trait;
use futures::{Stream, StreamExt};
use reqwest::Client;
use std::pin::Pin;

use crate::error::{LlmError, LlmResult};
use crate::types::{ChatRequest, ChatResponse, ModelInfo, StreamChunk};
use crate::provider::LlmProvider;

use super::{build_client, parse_api_error};

/// Stability AI provider
pub struct StabilityProvider {
    client: Client,
    api_key: String,
    base_url: String,
}

impl StabilityProvider {
    pub fn new(api_key: impl Into<String>) -> LlmResult<Self> {
        Ok(Self {
            client: build_client()?,
            api_key: api_key.into(),
            base_url: "https://api.stability.ai/v1".into(),
        })
    }

    pub fn from_env() -> LlmResult<Self> {
        let api_key = std::env::var("STABILITY_API_KEY")
            .map_err(|_| LlmError::Authentication("STABILITY_API_KEY not set".into()))?;
        Self::new(api_key)
    }
}

#[async_trait]
impl LlmProvider for StabilityProvider {
    fn name(&self) -> &str { "Stability AI" }
    fn id(&self) -> &str { "stability" }
    
    fn models(&self) -> Vec<ModelInfo> {
        vec![
            // ═══════════════════════════════════════════════════════════
            // STABLELM LANGUAGE MODELS
            // ═══════════════════════════════════════════════════════════
            ModelInfo {
                id: "stablelm-2-12b-chat".into(),
                name: "StableLM 2 12B Chat".into(),
                provider: "Stability AI".into(),
                context_window: 4_096,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.0001,
                output_cost_per_1k: 0.0001,
                supports_vision: false,
                supports_tools: false,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: None,
                quality_rating: 3,
                speed_rating: 5,
                is_reasoning: false,
                free_tier: false,
            },
            ModelInfo {
                id: "stablelm-2-7b-chat".into(),
                name: "StableLM 2 7B Chat".into(),
                provider: "Stability AI".into(),
                context_window: 4_096,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.00005,
                output_cost_per_1k: 0.00005,
                supports_vision: false,
                supports_tools: false,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: None,
                quality_rating: 3,
                speed_rating: 5,
                is_reasoning: false,
                free_tier: false,
            },
            ModelInfo {
                id: "stablelm-2-1-6b-chat".into(),
                name: "StableLM 2 1.6B Chat".into(),
                provider: "Stability AI".into(),
                context_window: 4_096,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.00002,
                output_cost_per_1k: 0.00002,
                supports_vision: false,
                supports_tools: false,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: None,
                quality_rating: 2,
                speed_rating: 5,
                is_reasoning: false,
                free_tier: true,
            },
            ModelInfo {
                id: "stablelm-zephyr-3b".into(),
                name: "StableLM Zephyr 3B".into(),
                provider: "Stability AI".into(),
                context_window: 4_096,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.00003,
                output_cost_per_1k: 0.00003,
                supports_vision: false,
                supports_tools: false,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: None,
                quality_rating: 3,
                speed_rating: 5,
                is_reasoning: false,
                free_tier: false,
            },
            ModelInfo {
                id: "stable-code-3b".into(),
                name: "Stable Code 3B".into(),
                provider: "Stability AI".into(),
                context_window: 16_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.00003,
                output_cost_per_1k: 0.00003,
                supports_vision: false,
                supports_tools: false,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: None,
                quality_rating: 3,
                speed_rating: 5,
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
        Ok(parse_stability_response(json))
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

fn parse_stability_response(json: serde_json::Value) -> ChatResponse {
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

fn parse_stream_chunk(json: serde_json::Value, model: &str) -> StreamChunk {
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
    fn test_stability_provider() {
        let provider = StabilityProvider::new("test-key");
        assert!(provider.is_ok());
    }

    #[test]
    fn test_stability_models() {
        let provider = StabilityProvider::new("test-key").unwrap();
        let models = provider.models();
        assert!(models.len() >= 5, "Stability should have at least 5 models");
        assert!(models.iter().any(|m| m.id.contains("stablelm")));
        assert!(models.iter().any(|m| m.id.contains("zephyr")));
    }
}
