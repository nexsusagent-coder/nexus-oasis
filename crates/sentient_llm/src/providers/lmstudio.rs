//! ─── LM Studio Provider ───
//!
//! LM Studio - Local LLM inference
//! https://lmstudio.ai

use async_trait::async_trait;
use futures::{Stream, StreamExt};
use reqwest::Client;
use std::pin::Pin;

use crate::error::{LlmError, LlmResult};
use crate::types::{ChatRequest, ChatResponse, ModelInfo, StreamChunk};
use crate::provider::LlmProvider;

use super::build_client;

/// LM Studio provider - Local LLM inference
pub struct LmStudioProvider {
    client: Client,
    base_url: String,
}

impl LmStudioProvider {
    pub fn new() -> LlmResult<Self> {
        Ok(Self {
            client: build_client()?,
            base_url: "http://localhost:1234/v1".into(),
        })
    }

    pub fn with_port(port: u16) -> LlmResult<Self> {
        Ok(Self {
            client: build_client()?,
            base_url: format!("http://localhost:{}/v1", port),
        })
    }

    pub fn from_env() -> LlmResult<Self> {
        let base_url = std::env::var("LMSTUDIO_BASE_URL")
            .unwrap_or_else(|_| "http://localhost:1234/v1".into());
        Ok(Self {
            client: build_client()?,
            base_url,
        })
    }
}

#[async_trait]
impl LlmProvider for LmStudioProvider {
    fn name(&self) -> &str { "LM Studio" }
    fn id(&self) -> &str { "lmstudio" }

    fn models(&self) -> Vec<ModelInfo> {
        // LM Studio runs any GGUF model locally
        vec![
            ModelInfo {
                id: "llama-3.2-3b-instruct".into(),
                name: "Llama 3.2 3B (Local)".into(),
                provider: "LM Studio".into(),
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
                id: "llama-3.1-8b-instruct".into(),
                name: "Llama 3.1 8B (Local)".into(),
                provider: "LM Studio".into(),
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
                id: "mistral-7b-instruct-v0.3".into(),
                name: "Mistral 7B (Local)".into(),
                provider: "LM Studio".into(),
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
                id: "qwen2.5-7b-instruct".into(),
                name: "Qwen 2.5 7B (Local)".into(),
                provider: "LM Studio".into(),
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
                id: "gemma-2-9b-it".into(),
                name: "Gemma 2 9B (Local)".into(),
                provider: "LM Studio".into(),
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
                id: "phi-3.5-mini-instruct".into(),
                name: "Phi 3.5 Mini (Local)".into(),
                provider: "LM Studio".into(),
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
                id: "deepseek-coder-6.7b-instruct".into(),
                name: "DeepSeek Coder 6.7B (Local)".into(),
                provider: "LM Studio".into(),
                context_window: 16_000,
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
        true // LM Studio is always available locally
    }

    async fn chat(&self, request: ChatRequest) -> LlmResult<ChatResponse> {
        let response = self.client
            .post(format!("{}/chat/completions", self.base_url))
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

impl Default for LmStudioProvider {
    fn default() -> Self {
        Self::new().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lmstudio_provider() {
        let provider = LmStudioProvider::new();
        assert!(provider.is_ok());
    }

    #[test]
    fn test_lmstudio_models() {
        let provider = LmStudioProvider::new().unwrap();
        let models = provider.models();
        assert!(models.len() >= 7);
        // All LM Studio models are free (local)
        assert!(models.iter().all(|m| m.free_tier));
    }
}
