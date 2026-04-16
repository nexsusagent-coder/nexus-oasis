//! ─── Chutes AI Provider ───
//!
//! Chutes - Serverless AI inference platform
//! https://chutes.ai

use async_trait::async_trait;
use futures::{Stream, StreamExt};
use reqwest::Client;
use std::pin::Pin;

use crate::error::{LlmError, LlmResult};
use crate::types::{ChatRequest, ChatResponse, ModelInfo, StreamChunk};
use crate::provider::LlmProvider;

use super::{build_client, parse_api_error};

/// Chutes AI provider - Serverless AI inference
pub struct ChutesProvider {
    client: Client,
    api_key: String,
    base_url: String,
}

impl ChutesProvider {
    pub fn new(api_key: impl Into<String>) -> LlmResult<Self> {
        Ok(Self {
            client: build_client()?,
            api_key: api_key.into(),
            base_url: "https://llm.chutes.ai/v1".into(),
        })
    }

    pub fn from_env() -> LlmResult<Self> {
        let api_key = std::env::var("CHUTES_API_KEY")
            .map_err(|_| LlmError::Authentication("CHUTES_API_KEY not set".into()))?;
        Self::new(api_key)
    }
}

#[async_trait]
impl LlmProvider for ChutesProvider {
    fn name(&self) -> &str { "Chutes" }
    fn id(&self) -> &str { "chutes" }

    fn models(&self) -> Vec<ModelInfo> {
        vec![
            ModelInfo {
                id: "chutesai/Llama-4-Maverick-17B-128E-Instruct".into(),
                name: "Llama 4 Maverick 400B (MoE)".into(),
                provider: "Chutes".into(),
                context_window: 1_048_576,
                max_output_tokens: 16_384,
                input_cost_per_1k: 0.0,
                output_cost_per_1k: 0.0,
                supports_vision: true,
                supports_tools: true,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: Some("2025-04".into()),
                quality_rating: 5,
                speed_rating: 4,
                is_reasoning: false,
                free_tier: true,
            },
            ModelInfo {
                id: "deepseek-ai/DeepSeek-R1".into(),
                name: "DeepSeek R1 (Chutes)".into(),
                provider: "Chutes".into(),
                context_window: 131_072,
                max_output_tokens: 16_384,
                input_cost_per_1k: 0.0,
                output_cost_per_1k: 0.0,
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: Some("2024-11".into()),
                quality_rating: 5,
                speed_rating: 3,
                is_reasoning: true,
                free_tier: true,
            },
            ModelInfo {
                id: "Qwen/Qwen3-235B-A22B".into(),
                name: "Qwen3 235B (Chutes)".into(),
                provider: "Chutes".into(),
                context_window: 131_072,
                max_output_tokens: 16_384,
                input_cost_per_1k: 0.0,
                output_cost_per_1k: 0.0,
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: Some("2025-03".into()),
                quality_rating: 5,
                speed_rating: 4,
                is_reasoning: true,
                free_tier: true,
            },
        ]
    }

    fn is_configured(&self) -> bool { !self.api_key.is_empty() }

    async fn chat(&self, request: ChatRequest) -> LlmResult<ChatResponse> {
        let response = self.client
            .post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await
            .map_err(|e| LlmError::Network(e.to_string()))?;

        if !response.status().is_success() {
            return Err(LlmError::ApiError(parse_api_error(response).await));
        }
        response.json().await.map_err(|e| LlmError::ParseError(e.to_string()))
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
                                if data == "[DONE]" { return None; }
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
    fn test_chutes_provider() { assert!(ChutesProvider::new("test-key").is_ok()); }
}
