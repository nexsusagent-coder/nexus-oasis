//! ─── vLLM Provider ───
//!
//! vLLM - High-performance LLM inference server
//! https://vllm.ai

use async_trait::async_trait;
use futures::{Stream, StreamExt};
use reqwest::Client;
use std::pin::Pin;

use crate::error::{LlmError, LlmResult};
use crate::types::{ChatRequest, ChatResponse, ModelInfo, StreamChunk};
use crate::provider::LlmProvider;

use super::build_client;

/// vLLM provider - High-performance LLM inference
pub struct VLLMProvider {
    client: Client,
    api_key: String,
    base_url: String,
}

impl VLLMProvider {
    pub fn new(base_url: impl Into<String>) -> LlmResult<Self> {
        Ok(Self {
            client: build_client()?,
            api_key: "".into(),
            base_url: base_url.into(),
        })
    }

    pub fn with_key(api_key: impl Into<String>, base_url: impl Into<String>) -> LlmResult<Self> {
        Ok(Self {
            client: build_client()?,
            api_key: api_key.into(),
            base_url: base_url.into(),
        })
    }

    pub fn from_env() -> LlmResult<Self> {
        let base_url = std::env::var("VLLM_BASE_URL")
            .unwrap_or_else(|_| "http://localhost:8000/v1".into());
        let api_key = std::env::var("VLLM_API_KEY").unwrap_or_default();
        Self::with_key(api_key, base_url)
    }
}

#[async_trait]
impl LlmProvider for VLLMProvider {
    fn name(&self) -> &str { "vLLM" }
    fn id(&self) -> &str { "vllm" }

    fn models(&self) -> Vec<ModelInfo> {
        // vLLM can run any open-source model
        vec![
            ModelInfo {
                id: "meta-llama/Meta-Llama-3.1-405B-Instruct".into(),
                name: "Llama 3.1 405B".into(),
                provider: "vLLM".into(),
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
                speed_rating: 4,
                is_reasoning: false,
                free_tier: true,
            },
            ModelInfo {
                id: "meta-llama/Meta-Llama-3.1-70B-Instruct".into(),
                name: "Llama 3.1 70B".into(),
                provider: "vLLM".into(),
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
                id: "meta-llama/Meta-Llama-3.1-8B-Instruct".into(),
                name: "Llama 3.1 8B".into(),
                provider: "vLLM".into(),
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
                id: "Qwen/Qwen2.5-72B-Instruct".into(),
                name: "Qwen 2.5 72B".into(),
                provider: "vLLM".into(),
                context_window: 128_000,
                max_output_tokens: 8_192,
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
                id: "mistralai/Mixtral-8x22B-Instruct-v0.1".into(),
                name: "Mixtral 8x22B".into(),
                provider: "vLLM".into(),
                context_window: 64_000,
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
                id: "deepseek-ai/DeepSeek-V3".into(),
                name: "DeepSeek V3".into(),
                provider: "vLLM".into(),
                context_window: 64_000,
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
                id: "deepseek-ai/DeepSeek-R1".into(),
                name: "DeepSeek R1".into(),
                provider: "vLLM".into(),
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
                id: "google/gemma-2-27b-it".into(),
                name: "Gemma 2 27B".into(),
                provider: "vLLM".into(),
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
        ]
    }

    fn is_configured(&self) -> bool {
        !self.base_url.is_empty()
    }

    async fn chat(&self, request: ChatRequest) -> LlmResult<ChatResponse> {
        let mut req = self.client
            .post(format!("{}/chat/completions", self.base_url))
            .json(&request);

        if !self.api_key.is_empty() {
            req = req.header("Authorization", format!("Bearer {}", self.api_key));
        }

        let response = req.send().await
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

        let mut req = self.client
            .post(format!("{}/chat/completions", self.base_url))
            .json(&request);

        if !self.api_key.is_empty() {
            req = req.header("Authorization", format!("Bearer {}", self.api_key));
        }

        let response = req.send().await
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
    fn test_vllm_provider() {
        let provider = VLLMProvider::new("http://localhost:8000/v1");
        assert!(provider.is_ok());
    }

    #[test]
    fn test_vllm_models() {
        let provider = VLLMProvider::new("http://localhost:8000/v1").unwrap();
        let models = provider.models();
        assert!(models.len() >= 8);
        // All vLLM models are free (self-hosted)
        assert!(models.iter().all(|m| m.free_tier));
    }
}
