//! ─── NVIDIA NIM Provider ───
//!
//! NVIDIA NIM - Enterprise AI inference
//! https://build.nvidia.com

use async_trait::async_trait;
use futures::{Stream, StreamExt};
use reqwest::Client;
use std::pin::Pin;

use crate::error::{LlmError, LlmResult};
use crate::types::{ChatRequest, ChatResponse, ModelInfo, StreamChunk};
use crate::provider::LlmProvider;

use super::build_client;

/// NVIDIA NIM provider - Enterprise AI inference
pub struct NvidiaProvider {
    client: Client,
    api_key: String,
    base_url: String,
}

impl NvidiaProvider {
    pub fn new(api_key: impl Into<String>) -> LlmResult<Self> {
        Ok(Self {
            client: build_client()?,
            api_key: api_key.into(),
            base_url: "https://integrate.api.nvidia.com/v1".into(),
        })
    }

    pub fn from_env() -> LlmResult<Self> {
        let api_key = std::env::var("NVIDIA_API_KEY")
            .map_err(|_| LlmError::Authentication("NVIDIA_API_KEY not set".into()))?;
        Self::new(api_key)
    }
}

#[async_trait]
impl LlmProvider for NvidiaProvider {
    fn name(&self) -> &str { "NVIDIA NIM" }
    fn id(&self) -> &str { "nvidia" }

    fn models(&self) -> Vec<ModelInfo> {
        vec![
            // ═══════════════════════════════════════════════════════════
            // NVIDIA NEMOTRON
            // ═══════════════════════════════════════════════════════════
            ModelInfo {
                id: "nvidia/llama-3.1-nemotron-70b-instruct".into(),
                name: "Nemotron 70B".into(),
                provider: "NVIDIA".into(),
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
                speed_rating: 4,
                is_reasoning: false,
                free_tier: true,
            },
            ModelInfo {
                id: "nvidia/llama-3.1-nemotron-ultra-253b-v1".into(),
                name: "Nemotron Ultra 253B".into(),
                provider: "NVIDIA".into(),
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
                speed_rating: 3,
                is_reasoning: false,
                free_tier: false,
            },

            // ═══════════════════════════════════════════════════════════
            // LLAMA ON NIM
            // ═══════════════════════════════════════════════════════════
            ModelInfo {
                id: "meta/llama-3.1-405b-instruct".into(),
                name: "Llama 3.1 405B".into(),
                provider: "NVIDIA".into(),
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
                speed_rating: 3,
                is_reasoning: false,
                free_tier: false,
            },
            ModelInfo {
                id: "meta/llama-3.1-70b-instruct".into(),
                name: "Llama 3.1 70B".into(),
                provider: "NVIDIA".into(),
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
                speed_rating: 4,
                is_reasoning: false,
                free_tier: true,
            },
            ModelInfo {
                id: "meta/llama-3.1-8b-instruct".into(),
                name: "Llama 3.1 8B".into(),
                provider: "NVIDIA".into(),
                context_window: 128_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.00002,
                output_cost_per_1k: 0.00002,
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
                id: "meta/llama-3.2-3b-instruct".into(),
                name: "Llama 3.2 3B".into(),
                provider: "NVIDIA".into(),
                context_window: 128_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.00001,
                output_cost_per_1k: 0.00001,
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
                id: "meta/llama-3.2-1b-instruct".into(),
                name: "Llama 3.2 1B".into(),
                provider: "NVIDIA".into(),
                context_window: 128_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.000005,
                output_cost_per_1k: 0.000005,
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: None,
                quality_rating: 2,
                speed_rating: 5,
                is_reasoning: false,
                free_tier: true,
            },

            // ═══════════════════════════════════════════════════════════
            // OTHER MODELS
            // ═══════════════════════════════════════════════════════════
            ModelInfo {
                id: "mistralai/mixtral-8x22b-instruct-v0.1".into(),
                name: "Mixtral 8x22B".into(),
                provider: "NVIDIA".into(),
                context_window: 64_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.0004,
                output_cost_per_1k: 0.0004,
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
                id: "mistralai/mistral-large-2-instruct".into(),
                name: "Mistral Large 2".into(),
                provider: "NVIDIA".into(),
                context_window: 128_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.0012,
                output_cost_per_1k: 0.0012,
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
                id: "google/gemma-2-27b-it".into(),
                name: "Gemma 2 27B".into(),
                provider: "NVIDIA".into(),
                context_window: 8_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.00008,
                output_cost_per_1k: 0.00008,
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
                id: "google/gemma-2-9b-it".into(),
                name: "Gemma 2 9B".into(),
                provider: "NVIDIA".into(),
                context_window: 8_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.00002,
                output_cost_per_1k: 0.00002,
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
                id: "qwen/qwen2.5-72b-instruct".into(),
                name: "Qwen 2.5 72B".into(),
                provider: "NVIDIA".into(),
                context_window: 128_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.00035,
                output_cost_per_1k: 0.00035,
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
                id: "deepseek-ai/deepseek-v3".into(),
                name: "DeepSeek V3".into(),
                provider: "NVIDIA".into(),
                context_window: 64_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.0001,
                output_cost_per_1k: 0.0001,
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
        ]
    }

    fn is_configured(&self) -> bool {
        !self.api_key.is_empty()
    }

    async fn chat(&self, request: ChatRequest) -> LlmResult<ChatResponse> {
        let response = self.client
            .post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await
            .map_err(|e| LlmError::Network(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(LlmError::ServerError(status.as_u16(), body));
        }

        response.json().await
            .map_err(|e| LlmError::ParseError(e.to_string()))
    }

    async fn chat_stream(
        &self,
        request: ChatRequest,
    ) -> LlmResult<Pin<Box<dyn Stream<Item = LlmResult<StreamChunk>> + Send>>> {
        let mut request = request;
        request.stream = true;

        let response = self.client
            .post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await
            .map_err(|e| LlmError::Network(e.to_string()))?;

        let stream = response.bytes_stream()
            .filter_map(|result| async move {
                match result {
                    Ok(bytes) => {
                        let text = String::from_utf8_lossy(&bytes);
                        for line in text.lines() {
                            if line.starts_with("data: ") {
                                let data = &line[6..];
                                if data == "[DONE]" {
                                    return None;
                                }
                                if let Ok(chunk) = serde_json::from_str::<StreamChunk>(data) {
                                    return Some(Ok(chunk));
                                }
                            }
                        }
                        None
                    }
                    Err(e) => Some(Err(LlmError::StreamError(e.to_string()))),
                }
            });

        Ok(Box::pin(stream))
    }

    fn count_tokens(&self, text: &str, _model: &str) -> LlmResult<usize> {
        Ok(text.len() / 4)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nvidia_provider() {
        let provider = NvidiaProvider::new("test-key");
        assert!(provider.is_ok());
    }

    #[test]
    fn test_nvidia_models() {
        let provider = NvidiaProvider::new("test-key").unwrap();
        let models = provider.models();
        assert!(models.len() >= 12);
    }
}
