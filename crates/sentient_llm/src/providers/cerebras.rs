//! ─── Cerebras Provider ───
//!
//! Cerebras - Fastest inference (CS-3 wafer-scale engine)
//! https://cerebras.ai

use async_trait::async_trait;
use futures::{Stream, StreamExt};
use reqwest::Client;
use std::pin::Pin;

use crate::error::{LlmError, LlmResult};
use crate::types::{ChatRequest, ChatResponse, ModelInfo, StreamChunk};
use crate::provider::LlmProvider;

use super::build_client;

/// Cerebras provider - Fastest inference
pub struct CerebrasProvider {
    client: Client,
    api_key: String,
    base_url: String,
}

impl CerebrasProvider {
    pub fn new(api_key: impl Into<String>) -> LlmResult<Self> {
        Ok(Self {
            client: build_client()?,
            api_key: api_key.into(),
            base_url: "https://api.cerebras.ai/v1".into(),
        })
    }

    pub fn from_env() -> LlmResult<Self> {
        let api_key = std::env::var("CEREBRAS_API_KEY")
            .map_err(|_| LlmError::Authentication("CEREBRAS_API_KEY not set".into()))?;
        Self::new(api_key)
    }
}

#[async_trait]
impl LlmProvider for CerebrasProvider {
    fn name(&self) -> &str { "Cerebras" }
    fn id(&self) -> &str { "cerebras" }

    fn models(&self) -> Vec<ModelInfo> {
        vec![
            ModelInfo {
                id: "llama3.1-8b".into(),
                name: "Llama 3.1 8B".into(),
                provider: "Cerebras".into(),
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
                free_tier: true,
            },
            ModelInfo {
                id: "llama-3.3-70b".into(),
                name: "Llama 3.3 70B".into(),
                provider: "Cerebras".into(),
                context_window: 128_000,
                max_output_tokens: 8_192,
                input_cost_per_1k: 0.0006,
                output_cost_per_1k: 0.0006,
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
                id: "llama-3.3-70b".into(),
                name: "Llama 3.3 70B".into(),
                provider: "Cerebras".into(),
                context_window: 128_000,
                max_output_tokens: 8_192,
                input_cost_per_1k: 0.0006,
                output_cost_per_1k: 0.0006,
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
    fn test_cerebras_provider() {
        let provider = CerebrasProvider::new("test-key");
        assert!(provider.is_ok());
    }

    #[test]
    fn test_cerebras_models() {
        let provider = CerebrasProvider::new("test-key").unwrap();
        let models = provider.models();
        assert!(models.len() >= 2);
    }
}
