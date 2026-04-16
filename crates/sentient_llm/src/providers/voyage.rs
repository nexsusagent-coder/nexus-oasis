//! ─── Voyage AI Provider ───
//!
//! Voyage AI - Specialized embedding & reranking models
//! https://voyageai.com

use async_trait::async_trait;
use futures::{Stream, StreamExt};
use reqwest::Client;
use std::pin::Pin;

use crate::error::{LlmError, LlmResult};
use crate::types::{ChatRequest, ChatResponse, ModelInfo, StreamChunk};
use crate::provider::LlmProvider;

use super::{build_client, parse_api_error};

/// Voyage AI provider
pub struct VoyageProvider {
    client: Client,
    api_key: String,
    base_url: String,
}

impl VoyageProvider {
    pub fn new(api_key: impl Into<String>) -> LlmResult<Self> {
        Ok(Self {
            client: build_client()?,
            api_key: api_key.into(),
            base_url: "https://api.voyageai.com/v1".into(),
        })
    }

    pub fn from_env() -> LlmResult<Self> {
        let api_key = std::env::var("VOYAGE_API_KEY")
            .map_err(|_| LlmError::Authentication("VOYAGE_API_KEY not set".into()))?;
        Self::new(api_key)
    }
}

#[async_trait]
impl LlmProvider for VoyageProvider {
    fn name(&self) -> &str { "Voyage AI" }
    fn id(&self) -> &str { "voyage" }

    fn models(&self) -> Vec<ModelInfo> {
        vec![
            ModelInfo {
                id: "voyage-3".into(),
                name: "Voyage 3".into(),
                provider: "Voyage".into(),
                context_window: 32_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.00006,
                output_cost_per_1k: 0.00006,
                supports_vision: false,
                supports_tools: false,
                supports_streaming: false,
                supports_json: true,
                training_cutoff: Some("2024-06".into()),
                quality_rating: 5,
                speed_rating: 5,
                is_reasoning: false,
                free_tier: true,
            },
            ModelInfo {
                id: "voyage-3-lite".into(),
                name: "Voyage 3 Lite".into(),
                provider: "Voyage".into(),
                context_window: 32_000,
                max_output_tokens: 2_048,
                input_cost_per_1k: 0.00002,
                output_cost_per_1k: 0.00002,
                supports_vision: false,
                supports_tools: false,
                supports_streaming: false,
                supports_json: true,
                training_cutoff: Some("2024-06".into()),
                quality_rating: 4,
                speed_rating: 5,
                is_reasoning: false,
                free_tier: true,
            },
            ModelInfo {
                id: "voyage-code-3".into(),
                name: "Voyage Code 3".into(),
                provider: "Voyage".into(),
                context_window: 32_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.00006,
                output_cost_per_1k: 0.00006,
                supports_vision: false,
                supports_tools: false,
                supports_streaming: false,
                supports_json: true,
                training_cutoff: Some("2024-08".into()),
                quality_rating: 5,
                speed_rating: 5,
                is_reasoning: false,
                free_tier: true,
            },
        ]
    }

    fn is_configured(&self) -> bool { !self.api_key.is_empty() }

    async fn chat(&self, _request: ChatRequest) -> LlmResult<ChatResponse> {
        Err(LlmError::ApiError("Voyage AI is embedding-only, not a chat provider".into()))
    }

    async fn chat_stream(
        &self,
        _request: ChatRequest,
    ) -> LlmResult<Pin<Box<dyn Stream<Item = LlmResult<StreamChunk>> + Send>>> {
        Err(LlmError::ApiError("Voyage AI is embedding-only, not a chat provider".into()))
    }

    fn count_tokens(&self, text: &str, _model: &str) -> LlmResult<usize> {
        Ok(text.len() / 4)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_voyage_provider() { assert!(VoyageProvider::new("test-key").is_ok()); }
}
