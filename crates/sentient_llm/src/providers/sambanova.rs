//! ─── SambaNova Provider ───
//!
//! SambaNova - Enterprise AI platform
//! https://sambanova.ai

use async_trait::async_trait;
use futures::{Stream, StreamExt};
use reqwest::Client;
use std::pin::Pin;

use crate::error::{LlmError, LlmResult};
use crate::types::{ChatRequest, ChatResponse, ModelInfo, StreamChunk};
use crate::provider::LlmProvider;

use super::build_client;

/// SambaNova provider - Enterprise AI platform
pub struct SambaNovaProvider {
    client: Client,
    api_key: String,
    base_url: String,
}

impl SambaNovaProvider {
    pub fn new(api_key: impl Into<String>) -> LlmResult<Self> {
        Ok(Self {
            client: build_client()?,
            api_key: api_key.into(),
            base_url: "https://api.sambanova.ai/v1".into(),
        })
    }

    pub fn from_env() -> LlmResult<Self> {
        let api_key = std::env::var("SAMBANOVA_API_KEY")
            .map_err(|_| LlmError::Authentication("SAMBANOVA_API_KEY not set".into()))?;
        Self::new(api_key)
    }
}

#[async_trait]
impl LlmProvider for SambaNovaProvider {
    fn name(&self) -> &str { "SambaNova" }
    fn id(&self) -> &str { "sambanova" }

    fn models(&self) -> Vec<ModelInfo> {
        vec![
            ModelInfo {
                id: "Meta-Llama-3.1-405B-Instruct".into(),
                name: "Llama 3.1 405B".into(),
                provider: "SambaNova".into(),
                context_window: 128_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.0015,
                output_cost_per_1k: 0.0015,
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
                id: "Meta-Llama-3.1-70B-Instruct".into(),
                name: "Llama 3.1 70B".into(),
                provider: "SambaNova".into(),
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
                speed_rating: 5,
                is_reasoning: false,
                free_tier: true,
            },
            ModelInfo {
                id: "Meta-Llama-3.1-8B-Instruct".into(),
                name: "Llama 3.1 8B".into(),
                provider: "SambaNova".into(),
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
                free_tier: true,
            },
            ModelInfo {
                id: "Qwen2.5-72B-Instruct".into(),
                name: "Qwen 2.5 72B".into(),
                provider: "SambaNova".into(),
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
                speed_rating: 4,
                is_reasoning: false,
                free_tier: false,
            },
            ModelInfo {
                id: "Qwen2.5-32B-Instruct".into(),
                name: "Qwen 2.5 32B".into(),
                provider: "SambaNova".into(),
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
                speed_rating: 5,
                is_reasoning: false,
                free_tier: false,
            },
            ModelInfo {
                id: "Qwen2.5-7B-Instruct".into(),
                name: "Qwen 2.5 7B".into(),
                provider: "SambaNova".into(),
                context_window: 128_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.00004,
                output_cost_per_1k: 0.00004,
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
    fn test_sambanova_provider() {
        let provider = SambaNovaProvider::new("test-key");
        assert!(provider.is_ok());
    }

    #[test]
    fn test_sambanova_models() {
        let provider = SambaNovaProvider::new("test-key").unwrap();
        let models = provider.models();
        assert!(models.len() >= 6);
    }
}
