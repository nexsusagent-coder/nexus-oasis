//! ─── XAI Grok 3 Provider ───
//!
//! Updated xAI with Grok 3, Grok 3 Mini models (2025)
//! https://x.ai

use async_trait::async_trait;
use futures::{Stream, StreamExt};
use reqwest::Client;
use std::pin::Pin;

use crate::error::{LlmError, LlmResult};
use crate::types::{ChatRequest, ChatResponse, ModelInfo, StreamChunk};
use crate::provider::LlmProvider;

use super::{build_client, parse_api_error};

/// xAI (Grok) provider - Updated with Grok 3
pub struct XAIProvider {
    client: Client,
    api_key: String,
    base_url: String,
}

impl XAIProvider {
    pub fn new(api_key: impl Into<String>) -> LlmResult<Self> {
        Ok(Self {
            client: build_client()?,
            api_key: api_key.into(),
            base_url: "https://api.x.ai/v1".into(),
        })
    }

    pub fn from_env() -> LlmResult<Self> {
        let api_key = std::env::var("XAI_API_KEY")
            .map_err(|_| LlmError::Authentication("XAI_API_KEY not set".into()))?;
        Self::new(api_key)
    }
}

#[async_trait]
impl LlmProvider for XAIProvider {
    fn name(&self) -> &str { "xAI" }
    fn id(&self) -> &str { "xai" }

    fn models(&self) -> Vec<ModelInfo> {
        vec![
            // Grok 3 series (2025)
            ModelInfo {
                id: "grok-3".into(),
                name: "Grok 3".into(),
                provider: "xAI".into(),
                context_window: 131_072,
                max_output_tokens: 16_384,
                input_cost_per_1k: 0.003,
                output_cost_per_1k: 0.015,
                supports_vision: true,
                supports_tools: true,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: Some("2025-02".into()),
                quality_rating: 5,
                speed_rating: 4,
                is_reasoning: false,
                free_tier: false,
            },
            ModelInfo {
                id: "grok-3-mini".into(),
                name: "Grok 3 Mini (Reasoning)".into(),
                provider: "xAI".into(),
                context_window: 131_072,
                max_output_tokens: 16_384,
                input_cost_per_1k: 0.0003,
                output_cost_per_1k: 0.0005,
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: Some("2025-02".into()),
                quality_rating: 4,
                speed_rating: 5,
                is_reasoning: true,
                free_tier: true,
            },
            ModelInfo {
                id: "grok-3-fast".into(),
                name: "Grok 3 Fast".into(),
                provider: "xAI".into(),
                context_window: 131_072,
                max_output_tokens: 16_384,
                input_cost_per_1k: 0.0005,
                output_cost_per_1k: 0.003,
                supports_vision: true,
                supports_tools: true,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: Some("2025-02".into()),
                quality_rating: 5,
                speed_rating: 5,
                is_reasoning: false,
                free_tier: false,
            },
            // Grok 2 series (legacy)
            ModelInfo {
                id: "grok-2-latest".into(),
                name: "Grok 2".into(),
                provider: "xAI".into(),
                context_window: 131_072,
                max_output_tokens: 8_192,
                input_cost_per_1k: 0.002,
                output_cost_per_1k: 0.01,
                supports_vision: true,
                supports_tools: true,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: Some("2024-08".into()),
                quality_rating: 5,
                speed_rating: 4,
                is_reasoning: false,
                free_tier: false,
            },
            ModelInfo {
                id: "grok-2-vision-latest".into(),
                name: "Grok 2 Vision".into(),
                provider: "xAI".into(),
                context_window: 32_768,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.002,
                output_cost_per_1k: 0.01,
                supports_vision: true,
                supports_tools: true,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: Some("2024-08".into()),
                quality_rating: 5,
                speed_rating: 4,
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
    fn test_xai_provider() { assert!(XAIProvider::new("test-key").is_ok()); }
    #[test]
    fn test_xai_models() { let p = XAIProvider::new("test-key").unwrap(); assert!(p.models().len() >= 5); }
}
