//! ─── GLHF.chat Provider ───
//!
//! GLHF.chat - Free & cheap LLM access
//! https://glhf.chat

use async_trait::async_trait;
use futures::{Stream, StreamExt};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::pin::Pin;

use crate::error::{LlmError, LlmResult};
use crate::types::{ChatRequest, ChatResponse, ModelInfo, StreamChunk, Role};
use crate::provider::LlmProvider;

use super::build_client;

/// GLHF.chat provider - Free & cheap LLM access
pub struct GlhfProvider {
    client: Client,
    api_key: String,
    base_url: String,
}

impl GlhfProvider {
    pub fn new(api_key: impl Into<String>) -> LlmResult<Self> {
        Ok(Self {
            client: build_client()?,
            api_key: api_key.into(),
            base_url: "https://glhf.chat/api/openai/v1".into(),
        })
    }

    pub fn from_env() -> LlmResult<Self> {
        let api_key = std::env::var("GLHF_API_KEY")
            .map_err(|_| LlmError::Authentication("GLHF_API_KEY not set".into()))?;
        Self::new(api_key)
    }

    fn convert_request(&self, request: ChatRequest) -> GlhfRequest {
        GlhfRequest {
            model: request.model,
            messages: request.messages.into_iter().map(|m| GlhfMessage {
                role: m.role.to_string(),
                content: m.content.as_text().map(|s| s.to_string()).unwrap_or_default(),
            }).collect(),
            max_tokens: request.max_tokens,
            temperature: request.temperature,
            stream: request.stream,
        }
    }
}

#[async_trait]
impl LlmProvider for GlhfProvider {
    fn name(&self) -> &str { "GLHF.chat" }
    fn id(&self) -> &str { "glhf" }

    fn models(&self) -> Vec<ModelInfo> {
        vec![
            // ═══════════════════════════════════════════════════════════
            // FREE TIER
            // ═══════════════════════════════════════════════════════════
            ModelInfo {
                id: "hf:meta-llama/Meta-Llama-3.1-8B-Instruct".into(),
                name: "Llama 3.1 8B (Free)".into(),
                provider: "GLHF".into(),
                context_window: 128_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.0,
                output_cost_per_1k: 0.0,
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
            ModelInfo {
                id: "hf:mistralai/Mistral-7B-Instruct-v0.3".into(),
                name: "Mistral 7B (Free)".into(),
                provider: "GLHF".into(),
                context_window: 32_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.0,
                output_cost_per_1k: 0.0,
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
            ModelInfo {
                id: "hf:Qwen/Qwen2.5-7B-Instruct".into(),
                name: "Qwen 2.5 7B (Free)".into(),
                provider: "GLHF".into(),
                context_window: 128_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.0,
                output_cost_per_1k: 0.0,
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
            ModelInfo {
                id: "hf:google/gemma-2-9b-it".into(),
                name: "Gemma 2 9B (Free)".into(),
                provider: "GLHF".into(),
                context_window: 8_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.0,
                output_cost_per_1k: 0.0,
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
            ModelInfo {
                id: "hf:cognitivecomputations/dolphin-2.9.4-llama3.1-8b".into(),
                name: "Dolphin Llama 3.1 8B (Free)".into(),
                provider: "GLHF".into(),
                context_window: 128_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.0,
                output_cost_per_1k: 0.0,
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
            // PAID - HIGH QUALITY
            // ═══════════════════════════════════════════════════════════
            ModelInfo {
                id: "hf:meta-llama/Meta-Llama-3.1-70B-Instruct".into(),
                name: "Llama 3.1 70B".into(),
                provider: "GLHF".into(),
                context_window: 128_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.0002,
                output_cost_per_1k: 0.0002,
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
                id: "hf:meta-llama/Meta-Llama-3.1-405B-Instruct-FP8".into(),
                name: "Llama 3.1 405B FP8".into(),
                provider: "GLHF".into(),
                context_window: 128_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.0008,
                output_cost_per_1k: 0.0008,
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
            ModelInfo {
                id: "hf:mistralai/Mixtral-8x7B-Instruct-v0.1".into(),
                name: "Mixtral 8x7B".into(),
                provider: "GLHF".into(),
                context_window: 32_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.00015,
                output_cost_per_1k: 0.00015,
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: None,
                quality_rating: 3,
                speed_rating: 4,
                is_reasoning: false,
                free_tier: false,
            },
            ModelInfo {
                id: "hf:Qwen/Qwen2.5-72B-Instruct".into(),
                name: "Qwen 2.5 72B".into(),
                provider: "GLHF".into(),
                context_window: 128_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.0003,
                output_cost_per_1k: 0.0003,
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
                id: "hf:deepseek-ai/DeepSeek-V3".into(),
                name: "DeepSeek V3".into(),
                provider: "GLHF".into(),
                context_window: 64_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.00005,
                output_cost_per_1k: 0.0002,
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
                id: "hf:nvidia/Llama-3.1-Nemotron-70B-Instruct".into(),
                name: "Nemotron 70B".into(),
                provider: "GLHF".into(),
                context_window: 128_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.0002,
                output_cost_per_1k: 0.0002,
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

            // ═══════════════════════════════════════════════════════════
            // SPECIAL MODELS
            // ═══════════════════════════════════════════════════════════
            ModelInfo {
                id: "hf:deepseek-ai/DeepSeek-R1".into(),
                name: "DeepSeek R1 (Reasoning)".into(),
                provider: "GLHF".into(),
                context_window: 128_000,
                max_output_tokens: 8_192,
                input_cost_per_1k: 0.0005,
                output_cost_per_1k: 0.002,
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
                id: "hf:Qwen/QwQ-32B-Preview".into(),
                name: "QwQ 32B (Reasoning)".into(),
                provider: "GLHF".into(),
                context_window: 32_000,
                max_output_tokens: 8_192,
                input_cost_per_1k: 0.00015,
                output_cost_per_1k: 0.00015,
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: None,
                quality_rating: 4,
                speed_rating: 4,
                is_reasoning: true,
                free_tier: false,
            },
        ]
    }

    fn is_configured(&self) -> bool {
        !self.api_key.is_empty()
    }

    async fn chat(&self, request: ChatRequest) -> LlmResult<ChatResponse> {
        let glhf_request = self.convert_request(request);

        let response = self.client
            .post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&glhf_request)
            .send()
            .await
            .map_err(|e| LlmError::Network(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(LlmError::ServerError(status.as_u16(), body));
        }

        let glhf_response: GlhfResponse = response.json().await
            .map_err(|e| LlmError::ParseError(e.to_string()))?;

        Ok(glhf_response.into())
    }

    async fn chat_stream(
        &self,
        request: ChatRequest,
    ) -> LlmResult<Pin<Box<dyn Stream<Item = LlmResult<StreamChunk>> + Send>>> {
        let mut glhf_request = self.convert_request(request);
        glhf_request.stream = true;

        let model = glhf_request.model.clone();
        let response = self.client
            .post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&glhf_request)
            .send()
            .await
            .map_err(|e| LlmError::Network(e.to_string()))?;

        let stream = response.bytes_stream()
            .map(move |result| {
                match result {
                    Ok(bytes) => {
                        let text = String::from_utf8_lossy(&bytes);
                        for line in text.lines() {
                            if line.starts_with("data: ") {
                                let data = &line[6..];
                                if data == "[DONE]" {
                                    return Ok(None);
                                }
                                if let Ok(chunk) = serde_json::from_str::<GlhfStreamChunk>(data) {
                                    return Ok(Some(StreamChunk {
                                        id: chunk.id,
                                        object: chunk.object,
                                        created: chunk.created,
                                        model: model.clone(),
                                        system_fingerprint: None,
                                        choices: chunk.choices.into_iter().map(|c| {
                                            crate::types::StreamChoice {
                                                index: c.index,
                                                delta: crate::types::Delta {
                                                    role: c.delta.role.map(|r| match r.as_str() {
                                                        "assistant" => Role::Assistant,
                                                        "user" => Role::User,
                                                        "system" => Role::System,
                                                        _ => Role::Assistant,
                                                    }),
                                                    content: c.delta.content,
                                                    tool_calls: None,
                                                },
                                                finish_reason: c.finish_reason,
                                            }
                                        }).collect(),
                                    }));
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

// ═══════════════════════════════════════════════════════════════════════════════
//  GLHF API TYPES
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Serialize)]
struct GlhfRequest {
    model: String,
    messages: Vec<GlhfMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    stream: bool,
}

#[derive(Debug, Serialize)]
struct GlhfMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct GlhfResponse {
    id: String,
    object: String,
    created: i64,
    model: String,
    choices: Vec<GlhfChoice>,
    usage: GlhfUsage,
}

impl From<GlhfResponse> for ChatResponse {
    fn from(resp: GlhfResponse) -> Self {
        ChatResponse {
            id: resp.id,
            object: resp.object,
            created: resp.created,
            model: resp.model,
            choices: resp.choices.into_iter().map(|c| crate::types::Choice {
                index: c.index,
                message: crate::types::Message::assistant(&c.message.content),
                finish_reason: c.finish_reason,
                logprobs: None,
            }).collect(),
            usage: crate::types::Usage {
                prompt_tokens: resp.usage.prompt_tokens,
                completion_tokens: resp.usage.completion_tokens,
                total_tokens: resp.usage.total_tokens,
            },
            system_fingerprint: None,
        }
    }
}

#[derive(Debug, Deserialize)]
struct GlhfChoice {
    index: u32,
    message: GlhfMessageResponse,
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GlhfMessageResponse {
    content: String,
}

#[derive(Debug, Deserialize)]
struct GlhfUsage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

#[derive(Debug, Deserialize)]
struct GlhfStreamChunk {
    id: String,
    object: String,
    created: i64,
    choices: Vec<GlhfStreamChoice>,
}

#[derive(Debug, Deserialize)]
struct GlhfStreamChoice {
    index: u32,
    delta: GlhfDelta,
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GlhfDelta {
    #[serde(default)]
    role: Option<String>,
    #[serde(default)]
    content: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_glhf_provider() {
        let provider = GlhfProvider::new("test-key");
        assert!(provider.is_ok());
    }

    #[test]
    fn test_glhf_models() {
        let provider = GlhfProvider::new("test-key").unwrap();
        let models = provider.models();
        assert!(models.len() > 10);
        assert!(models.iter().any(|m| m.free_tier));
    }
}
