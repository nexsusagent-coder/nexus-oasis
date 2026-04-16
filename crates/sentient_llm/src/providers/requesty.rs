//! ─── Requesty AI Provider ───
//!
//! Requesty - LLM Router & Gateway
//! https://requesty.ai
//!
//! Features:
//! - Intelligent model routing
//! - 50+ model providers
//! - Cost tracking & analytics
//! - Prompt management
//! - A/B testing across models

use async_trait::async_trait;
use futures::{Stream, StreamExt};
use reqwest::Client;
use std::pin::Pin;

use crate::error::{LlmError, LlmResult};
use crate::types::{ChatRequest, ChatResponse, ModelInfo, StreamChunk};
use crate::provider::LlmProvider;

use super::{build_client, parse_api_error};

/// Requesty AI provider - LLM Router & Gateway
pub struct RequestyProvider {
    client: Client,
    api_key: String,
    base_url: String,
}

impl RequestyProvider {
    pub fn new(api_key: impl Into<String>) -> LlmResult<Self> {
        Ok(Self {
            client: build_client()?,
            api_key: api_key.into(),
            base_url: "https://api.requesty.ai/v1".into(),
        })
    }

    pub fn from_env() -> LlmResult<Self> {
        let api_key = std::env::var("REQUESTY_API_KEY")
            .map_err(|_| LlmError::Authentication("REQUESTY_API_KEY not set".into()))?;
        Self::new(api_key)
    }
}

#[async_trait]
impl LlmProvider for RequestyProvider {
    fn name(&self) -> &str { "Requesty" }
    fn id(&self) -> &str { "requesty" }

    fn models(&self) -> Vec<ModelInfo> {
        vec![
            ModelInfo {
                id: "requesty/auto".into(), name: "Requesty Auto Router".into(), provider: "Requesty".into(),
                context_window: 128_000, max_output_tokens: 8_192,
                input_cost_per_1k: 0.002, output_cost_per_1k: 0.008,
                supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
                training_cutoff: None, quality_rating: 5, speed_rating: 4,
                is_reasoning: false, free_tier: false,
            },
            ModelInfo {
                id: "requesty/gpt-4o".into(), name: "GPT-4o (Requesty)".into(), provider: "Requesty".into(),
                context_window: 128_000, max_output_tokens: 16_384,
                input_cost_per_1k: 0.0025, output_cost_per_1k: 0.01,
                supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
                training_cutoff: Some("2024-04".into()), quality_rating: 5, speed_rating: 4,
                is_reasoning: false, free_tier: false,
            },
            ModelInfo {
                id: "requesty/claude-4-sonnet".into(), name: "Claude Sonnet 4 (Requesty)".into(), provider: "Requesty".into(),
                context_window: 200_000, max_output_tokens: 16_384,
                input_cost_per_1k: 0.003, output_cost_per_1k: 0.015,
                supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
                training_cutoff: Some("2025-02".into()), quality_rating: 5, speed_rating: 4,
                is_reasoning: false, free_tier: false,
            },
            ModelInfo {
                id: "requesty/deepseek-r1".into(), name: "DeepSeek R1 (Requesty)".into(), provider: "Requesty".into(),
                context_window: 64_000, max_output_tokens: 8_192,
                input_cost_per_1k: 0.00055, output_cost_per_1k: 0.00219,
                supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
                training_cutoff: Some("2024-11".into()), quality_rating: 5, speed_rating: 3,
                is_reasoning: true, free_tier: true,
            },
        ]
    }

    fn is_configured(&self) -> bool { !self.api_key.is_empty() }

    async fn chat(&self, request: ChatRequest) -> LlmResult<ChatResponse> {
        let response = self.client
            .post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request).send().await
            .map_err(|e| LlmError::Network(e.to_string()))?;

        if !response.status().is_success() {
            return Err(LlmError::ApiError(parse_api_error(response).await));
        }
        response.json().await.map_err(|e| LlmError::ParseError(e.to_string()))
    }

    async fn chat_stream(
        &self, request: ChatRequest,
    ) -> LlmResult<Pin<Box<dyn Stream<Item = LlmResult<StreamChunk>> + Send>>> {
        let mut request = request;
        request.stream = true;
        let response = self.client
            .post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request).send().await
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

    fn count_tokens(&self, text: &str, _model: &str) -> LlmResult<usize> { Ok(text.len() / 4) }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_requesty_provider() { assert!(RequestyProvider::new("test-key").is_ok()); }
    #[test]
    fn test_requesty_models() { let p = RequestyProvider::new("test-key").unwrap(); assert!(p.models().len() >= 4); }
}
