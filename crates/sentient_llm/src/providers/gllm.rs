//! ─── Glama AI Provider ───
//!
//! Glama - Multi-model AI gateway
//! https://glama.ai
//!
//! Features:
//! - Unified API for 100+ models
//! - MCP (Model Context Protocol) server support
//! - Chat, vision, embeddings
//! - Cost optimization

use async_trait::async_trait;
use futures::{Stream, StreamExt};
use reqwest::Client;
use std::pin::Pin;

use crate::error::{LlmError, LlmResult};
use crate::types::{ChatRequest, ChatResponse, ModelInfo, StreamChunk};
use crate::provider::LlmProvider;

use super::{build_client, parse_api_error};

/// Glama AI provider - Multi-model gateway with MCP
pub struct GlamaProvider {
    client: Client,
    api_key: String,
    base_url: String,
}

impl GlamaProvider {
    pub fn new(api_key: impl Into<String>) -> LlmResult<Self> {
        Ok(Self {
            client: build_client()?,
            api_key: api_key.into(),
            base_url: "https://api.glama.ai/v1".into(),
        })
    }

    pub fn from_env() -> LlmResult<Self> {
        let api_key = std::env::var("GLAMA_API_KEY")
            .map_err(|_| LlmError::Authentication("GLAMA_API_KEY not set".into()))?;
        Self::new(api_key)
    }
}

#[async_trait]
impl LlmProvider for GlamaProvider {
    fn name(&self) -> &str { "Glama" }
    fn id(&self) -> &str { "glama" }

    fn models(&self) -> Vec<ModelInfo> {
        vec![
            ModelInfo {
                id: "glama/gpt-4o".into(), name: "GPT-4o (Glama)".into(), provider: "Glama".into(),
                context_window: 128_000, max_output_tokens: 16_384,
                input_cost_per_1k: 0.0025, output_cost_per_1k: 0.01,
                supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
                training_cutoff: Some("2024-04".into()), quality_rating: 5, speed_rating: 4,
                is_reasoning: false, free_tier: false,
            },
            ModelInfo {
                id: "glama/claude-4-sonnet".into(), name: "Claude Sonnet 4 (Glama)".into(), provider: "Glama".into(),
                context_window: 200_000, max_output_tokens: 16_384,
                input_cost_per_1k: 0.003, output_cost_per_1k: 0.015,
                supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
                training_cutoff: Some("2025-02".into()), quality_rating: 5, speed_rating: 4,
                is_reasoning: false, free_tier: false,
            },
            ModelInfo {
                id: "glama/deepseek-r1".into(), name: "DeepSeek R1 (Glama)".into(), provider: "Glama".into(),
                context_window: 64_000, max_output_tokens: 8_192,
                input_cost_per_1k: 0.00055, output_cost_per_1k: 0.00219,
                supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
                training_cutoff: Some("2024-11".into()), quality_rating: 5, speed_rating: 3,
                is_reasoning: true, free_tier: true,
            },
            ModelInfo {
                id: "glama/llama-4-maverick".into(), name: "Llama 4 Maverick (Glama)".into(), provider: "Glama".into(),
                context_window: 1_048_576, max_output_tokens: 16_384,
                input_cost_per_1k: 0.0015, output_cost_per_1k: 0.002,
                supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
                training_cutoff: Some("2025-04".into()), quality_rating: 5, speed_rating: 4,
                is_reasoning: false, free_tier: false,
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
    fn test_glama_provider() { assert!(GlamaProvider::new("test-key").is_ok()); }
    #[test]
    fn test_glama_models() { let p = GlamaProvider::new("test-key").unwrap(); assert!(p.models().len() >= 4); }
}
