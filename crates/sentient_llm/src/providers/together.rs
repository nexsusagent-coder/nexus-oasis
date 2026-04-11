//! ─── Together Provider ───
//!
//! Implementation for Together AI API (100+ open source models)

use async_trait::async_trait;
use futures::{Stream, StreamExt};
use reqwest::Client;
use std::pin::Pin;

use crate::error::{LlmError, LlmResult};
use crate::types::{ChatRequest, ChatResponse, ModelInfo, StreamChunk};
use crate::provider::LlmProvider;
use crate::models;

use super::{build_client, parse_api_error};

/// Together AI provider - 100+ open source models
pub struct TogetherProvider {
    client: Client,
    api_key: String,
    base_url: String,
}

impl TogetherProvider {
    pub fn new(api_key: impl Into<String>) -> LlmResult<Self> {
        Ok(Self {
            client: build_client()?,
            api_key: api_key.into(),
            base_url: "https://api.together.xyz/v1".into(),
        })
    }

    pub fn from_env() -> LlmResult<Self> {
        let api_key = std::env::var("TOGETHER_API_KEY")
            .map_err(|_| LlmError::Authentication("TOGETHER_API_KEY not set".into()))?;
        Self::new(api_key)
    }
}

#[async_trait]
impl LlmProvider for TogetherProvider {
    fn name(&self) -> &str { "Together" }
    fn id(&self) -> &str { "together" }
    fn models(&self) -> Vec<ModelInfo> {
        vec![
            // ═══════════════════════════════════════════════════════════
            // META LLAMA MODELS
            // ═══════════════════════════════════════════════════════════
            ModelInfo {
                id: "meta-llama/Llama-3.3-70B-Instruct-Turbo".into(),
                name: "Llama 3.3 70B Turbo".into(),
                provider: "Together".into(),
                context_window: 128_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.00088,
                output_cost_per_1k: 0.00088,
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: None,
                quality_rating: 5,
                speed_rating: 5,
                is_reasoning: false,
                free_tier: false,
            },
            ModelInfo {
                id: "meta-llama/Llama-3.3-70B-Instruct".into(),
                name: "Llama 3.3 70B".into(),
                provider: "Together".into(),
                context_window: 128_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.0006,
                output_cost_per_1k: 0.0006,
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: None,
                quality_rating: 5,
                speed_rating: 4,
                is_reasoning: false,
                free_tier: false,
            },
            ModelInfo {
                id: "meta-llama/Llama-3.2-90B-Vision-Instruct-Turbo".into(),
                name: "Llama 3.2 90B Vision".into(),
                provider: "Together".into(),
                context_window: 128_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.00088,
                output_cost_per_1k: 0.00088,
                supports_vision: true,
                supports_tools: true,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: None,
                quality_rating: 5,
                speed_rating: 5,
                is_reasoning: false,
                free_tier: false,
            },
            ModelInfo {
                id: "meta-llama/Llama-3.2-11B-Vision-Instruct-Turbo".into(),
                name: "Llama 3.2 11B Vision".into(),
                provider: "Together".into(),
                context_window: 128_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.00018,
                output_cost_per_1k: 0.00018,
                supports_vision: true,
                supports_tools: true,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: None,
                quality_rating: 4,
                speed_rating: 5,
                is_reasoning: false,
                free_tier: false,
            },
            ModelInfo {
                id: "meta-llama/Llama-3.1-405B-Instruct-Turbo".into(),
                name: "Llama 3.1 405B Turbo".into(),
                provider: "Together".into(),
                context_window: 128_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.002,
                output_cost_per_1k: 0.002,
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: None,
                quality_rating: 5,
                speed_rating: 4,
                is_reasoning: false,
                free_tier: false,
            },
            ModelInfo {
                id: "meta-llama/Llama-3.1-70B-Instruct-Turbo".into(),
                name: "Llama 3.1 70B Turbo".into(),
                provider: "Together".into(),
                context_window: 128_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.00054,
                output_cost_per_1k: 0.00054,
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: None,
                quality_rating: 4,
                speed_rating: 5,
                is_reasoning: false,
                free_tier: false,
            },
            ModelInfo {
                id: "meta-llama/Llama-3.1-8B-Instruct-Turbo".into(),
                name: "Llama 3.1 8B Turbo".into(),
                provider: "Together".into(),
                context_window: 128_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.00003,
                output_cost_per_1k: 0.00003,
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

            // ═══════════════════════════════════════════════════════════
            // MISTRAL MODELS
            // ═══════════════════════════════════════════════════════════
            ModelInfo {
                id: "mistralai/Mixtral-8x22B-Instruct-v0.1".into(),
                name: "Mixtral 8x22B".into(),
                provider: "Together".into(),
                context_window: 64_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.0009,
                output_cost_per_1k: 0.0009,
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
                id: "mistralai/Mixtral-8x7B-Instruct-v0.1".into(),
                name: "Mixtral 8x7B".into(),
                provider: "Together".into(),
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
                speed_rating: 4,
                is_reasoning: false,
                free_tier: false,
            },
            ModelInfo {
                id: "mistralai/Mistral-7B-Instruct-v0.3".into(),
                name: "Mistral 7B v0.3".into(),
                provider: "Together".into(),
                context_window: 32_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.0002,
                output_cost_per_1k: 0.0002,
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
                id: "mistralai/Mistral-7B-Instruct-v0.2".into(),
                name: "Mistral 7B v0.2".into(),
                provider: "Together".into(),
                context_window: 32_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.0002,
                output_cost_per_1k: 0.0002,
                supports_vision: false,
                supports_tools: false,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: None,
                quality_rating: 3,
                speed_rating: 5,
                is_reasoning: false,
                free_tier: true,
            },

            // ═══════════════════════════════════════════════════════════
            // QWEN MODELS
            // ═══════════════════════════════════════════════════════════
            ModelInfo {
                id: "Qwen/Qwen2.5-72B-Instruct-Turbo".into(),
                name: "Qwen 2.5 72B Turbo".into(),
                provider: "Together".into(),
                context_window: 128_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.0007,
                output_cost_per_1k: 0.0007,
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: None,
                quality_rating: 4,
                speed_rating: 5,
                is_reasoning: false,
                free_tier: false,
            },
            ModelInfo {
                id: "Qwen/Qwen2.5-32B-Instruct".into(),
                name: "Qwen 2.5 32B".into(),
                provider: "Together".into(),
                context_window: 128_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.0004,
                output_cost_per_1k: 0.0004,
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: None,
                quality_rating: 4,
                speed_rating: 5,
                is_reasoning: false,
                free_tier: false,
            },
            ModelInfo {
                id: "Qwen/Qwen2.5-14B-Instruct".into(),
                name: "Qwen 2.5 14B".into(),
                provider: "Together".into(),
                context_window: 128_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.0002,
                output_cost_per_1k: 0.0002,
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
                id: "Qwen/Qwen2.5-Coder-32B-Instruct".into(),
                name: "Qwen 2.5 Coder 32B".into(),
                provider: "Together".into(),
                context_window: 128_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.0004,
                output_cost_per_1k: 0.0004,
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: None,
                quality_rating: 4,
                speed_rating: 5,
                is_reasoning: false,
                free_tier: false,
            },

            // ═══════════════════════════════════════════════════════════
            // DEEPSEEK MODELS
            // ═══════════════════════════════════════════════════════════
            ModelInfo {
                id: "deepseek-ai/deepseek-v3".into(),
                name: "DeepSeek V3".into(),
                provider: "Together".into(),
                context_window: 64_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.00028,
                output_cost_per_1k: 0.00028,
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
                id: "deepseek-ai/DeepSeek-R1".into(),
                name: "DeepSeek R1".into(),
                provider: "Together".into(),
                context_window: 128_000,
                max_output_tokens: 8_192,
                input_cost_per_1k: 0.0012,
                output_cost_per_1k: 0.0012,
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: None,
                quality_rating: 5,
                speed_rating: 3,
                is_reasoning: true,
                free_tier: false,
            },
            ModelInfo {
                id: "deepseek-ai/deepseek-coder-33b-instruct".into(),
                name: "DeepSeek Coder 33B".into(),
                provider: "Together".into(),
                context_window: 16_000,
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

            // ═══════════════════════════════════════════════════════════
            // GOOGLE GEMMA MODELS
            // ═══════════════════════════════════════════════════════════
            ModelInfo {
                id: "google/gemma-2-27b-it".into(),
                name: "Gemma 2 27B".into(),
                provider: "Together".into(),
                context_window: 8_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.0003,
                output_cost_per_1k: 0.0003,
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
                id: "google/gemma-2-9b-it".into(),
                name: "Gemma 2 9B".into(),
                provider: "Together".into(),
                context_window: 8_000,
                max_output_tokens: 4_096,
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
                free_tier: true,
            },

            // ═══════════════════════════════════════════════════════════
            // PHI MODELS
            // ═══════════════════════════════════════════════════════════
            ModelInfo {
                id: "microsoft/Phi-4".into(),
                name: "Phi-4".into(),
                provider: "Together".into(),
                context_window: 16_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.00014,
                output_cost_per_1k: 0.00014,
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: None,
                quality_rating: 4,
                speed_rating: 5,
                is_reasoning: false,
                free_tier: false,
            },
            ModelInfo {
                id: "microsoft/Phi-3.5-mini-instruct".into(),
                name: "Phi 3.5 Mini".into(),
                provider: "Together".into(),
                context_window: 128_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.00014,
                output_cost_per_1k: 0.00014,
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
        Ok(parse_openai_response(json))
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

fn parse_openai_response(json: serde_json::Value) -> ChatResponse {
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
    fn test_together_provider() {
        let provider = TogetherProvider::new("test-key");
        assert!(provider.is_ok());
    }

    #[test]
    fn test_together_models() {
        let provider = TogetherProvider::new("test-key").unwrap();
        let models = provider.models();
        assert!(models.len() >= 20, "Together should have at least 20 models");
        // Check for key models
        assert!(models.iter().any(|m| m.id.contains("Llama-3.3")));
        assert!(models.iter().any(|m| m.id.contains("Qwen")));
        assert!(models.iter().any(|m| m.id.contains("DeepSeek")));
        assert!(models.iter().any(|m| m.id.contains("Phi")));
    }
}
