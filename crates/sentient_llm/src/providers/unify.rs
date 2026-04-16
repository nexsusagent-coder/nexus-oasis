//! ─── Unify AI Provider ───
//!
//! Unify - Intelligent model router, automatically picks best model
//! https://unify.ai
//!
//! Unify enables routing across LLM providers based on:
//! - Cost optimization (cheapest model that meets quality)
//! - Quality optimization (best model for task)
//! - Latency optimization (fastest model)
//! - Custom routing with @-syntax: "router@q>0.9&c<0.001"

use async_trait::async_trait;
use futures::{Stream, StreamExt};
use reqwest::Client;
use std::pin::Pin;

use crate::error::{LlmError, LlmResult};
use crate::types::{ChatRequest, ChatResponse, ModelInfo, StreamChunk};
use crate::provider::LlmProvider;

use super::{build_client, parse_api_error};

/// Unify AI provider - Intelligent model routing
pub struct UnifyProvider {
    client: Client,
    api_key: String,
    base_url: String,
}

impl UnifyProvider {
    pub fn new(api_key: impl Into<String>) -> LlmResult<Self> {
        Ok(Self {
            client: build_client()?,
            api_key: api_key.into(),
            base_url: "https://api.unify.ai/v1".into(),
        })
    }

    pub fn from_env() -> LlmResult<Self> {
        let api_key = std::env::var("UNIFY_API_KEY")
            .map_err(|_| LlmError::Authentication("UNIFY_API_KEY not set".into()))?;
        Self::new(api_key)
    }
}

#[async_trait]
impl LlmProvider for UnifyProvider {
    fn name(&self) -> &str { "Unify AI" }
    fn id(&self) -> &str { "unify" }

    fn models(&self) -> Vec<ModelInfo> {
        vec![
            // Unify Router - akıllı yönlendirme
            ModelInfo {
                id: "router@q>0.9&c<0.001".into(),
                name: "Unify Smart Router (Quality>0.9, Cheap)".into(),
                provider: "Unify".into(),
                context_window: 128_000, max_output_tokens: 8_192,
                input_cost_per_1k: 0.001, output_cost_per_1k: 0.004,
                supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
                training_cutoff: None, quality_rating: 5, speed_rating: 4,
                is_reasoning: false, free_tier: true,
            },
            ModelInfo {
                id: "router@speed".into(),
                name: "Unify Speed Router (Fastest)".into(),
                provider: "Unify".into(),
                context_window: 128_000, max_output_tokens: 8_192,
                input_cost_per_1k: 0.0005, output_cost_per_1k: 0.002,
                supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
                training_cutoff: None, quality_rating: 4, speed_rating: 5,
                is_reasoning: false, free_tier: true,
            },
            ModelInfo {
                id: "router@cost".into(),
                name: "Unify Cost Router (Cheapest)".into(),
                provider: "Unify".into(),
                context_window: 128_000, max_output_tokens: 4_096,
                input_cost_per_1k: 0.0001, output_cost_per_1k: 0.0003,
                supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
                training_cutoff: None, quality_rating: 3, speed_rating: 5,
                is_reasoning: false, free_tier: true,
            },
            // Doğrudan model erişimi
            ModelInfo {
                id: "anthropic@claude-4-sonnet".into(),
                name: "Claude Sonnet 4 (Unify)".into(),
                provider: "Unify".into(),
                context_window: 200_000, max_output_tokens: 16_000,
                input_cost_per_1k: 0.003, output_cost_per_1k: 0.015,
                supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
                training_cutoff: Some("2025-02".into()), quality_rating: 5, speed_rating: 4,
                is_reasoning: false, free_tier: false,
            },
            ModelInfo {
                id: "openai@gpt-4o".into(),
                name: "GPT-4o (Unify)".into(),
                provider: "Unify".into(),
                context_window: 128_000, max_output_tokens: 16_384,
                input_cost_per_1k: 0.0025, output_cost_per_1k: 0.01,
                supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
                training_cutoff: Some("2024-04".into()), quality_rating: 5, speed_rating: 4,
                is_reasoning: false, free_tier: false,
            },
            ModelInfo {
                id: "deepseek@deepseek-r1".into(),
                name: "DeepSeek R1 (Unify)".into(),
                provider: "Unify".into(),
                context_window: 64_000, max_output_tokens: 8_192,
                input_cost_per_1k: 0.00055, output_cost_per_1k: 0.00219,
                supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
                training_cutoff: Some("2024-11".into()), quality_rating: 5, speed_rating: 3,
                is_reasoning: true, free_tier: true,
            },
            ModelInfo {
                id: "google@gemini-2.5-flash".into(),
                name: "Gemini 2.5 Flash (Unify)".into(),
                provider: "Unify".into(),
                context_window: 1_048_576, max_output_tokens: 65_536,
                input_cost_per_1k: 0.00015, output_cost_per_1k: 0.0006,
                supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
                training_cutoff: Some("2025-01".into()), quality_rating: 5, speed_rating: 5,
                is_reasoning: true, free_tier: true,
            },
            ModelInfo {
                id: "xai@grok-3".into(),
                name: "Grok 3 (Unify)".into(),
                provider: "Unify".into(),
                context_window: 131_072, max_output_tokens: 16_384,
                input_cost_per_1k: 0.003, output_cost_per_1k: 0.015,
                supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
                training_cutoff: Some("2025-02".into()), quality_rating: 5, speed_rating: 4,
                is_reasoning: false, free_tier: false,
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
    fn test_unify_provider() { assert!(UnifyProvider::new("test-key").is_ok()); }
    #[test]
    fn test_unify_models() { let p = UnifyProvider::new("test-key").unwrap(); assert!(p.models().len() >= 8); }
}
