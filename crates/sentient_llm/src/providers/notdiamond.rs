//! ─── NotDiamond AI Provider ───
//!
//! NotDiamond - AI Model Router that learns which model works best
//! https://notdiamond.ai
//!
//! Features:
//! - ML-based model routing (learns from your usage patterns)
//! - Automatic model selection per prompt
//! - Fallback chains
//! - Cost optimization with quality guarantees
//! - Supports 50+ underlying models

use async_trait::async_trait;
use futures::{Stream, StreamExt};
use reqwest::Client;
use std::pin::Pin;

use crate::error::{LlmError, LlmResult};
use crate::types::{ChatRequest, ChatResponse, ModelInfo, StreamChunk};
use crate::provider::LlmProvider;

use super::{build_client, parse_api_error};

/// NotDiamond provider - ML-based intelligent model routing
pub struct NotDiamondProvider {
    client: Client,
    api_key: String,
    base_url: String,
}

impl NotDiamondProvider {
    pub fn new(api_key: impl Into<String>) -> LlmResult<Self> {
        Ok(Self {
            client: build_client()?,
            api_key: api_key.into(),
            base_url: "https://api.notdiamond.ai/v1".into(),
        })
    }

    pub fn from_env() -> LlmResult<Self> {
        let api_key = std::env::var("NOTDIAMOND_API_KEY")
            .map_err(|_| LlmError::Authentication("NOTDIAMOND_API_KEY not set".into()))?;
        Self::new(api_key)
    }
}

#[async_trait]
impl LlmProvider for NotDiamondProvider {
    fn name(&self) -> &str { "NotDiamond" }
    fn id(&self) -> &str { "notdiamond" }

    fn models(&self) -> Vec<ModelInfo> {
        vec![
            // NotDiamond router - ML ile otomatik model seçimi
            ModelInfo {
                id: "notdiamond/auto".into(), name: "NotDiamond Auto Router".into(), provider: "NotDiamond".into(),
                context_window: 128_000, max_output_tokens: 8_192,
                input_cost_per_1k: 0.002, output_cost_per_1k: 0.008,
                supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
                training_cutoff: None, quality_rating: 5, speed_rating: 4,
                is_reasoning: false, free_tier: false,
            },
            ModelInfo {
                id: "notdiamond/code".into(), name: "NotDiamond Code Router".into(), provider: "NotDiamond".into(),
                context_window: 128_000, max_output_tokens: 8_192,
                input_cost_per_1k: 0.0015, output_cost_per_1k: 0.006,
                supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
                training_cutoff: None, quality_rating: 5, speed_rating: 4,
                is_reasoning: false, free_tier: false,
            },
            ModelInfo {
                id: "notdiamond/reasoning".into(), name: "NotDiamond Reasoning Router".into(), provider: "NotDiamond".into(),
                context_window: 128_000, max_output_tokens: 16_384,
                input_cost_per_1k: 0.003, output_cost_per_1k: 0.012,
                supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
                training_cutoff: None, quality_rating: 5, speed_rating: 3,
                is_reasoning: true, free_tier: false,
            },
            ModelInfo {
                id: "notdiamond/cheap".into(), name: "NotDiamond Cost-Optimized Router".into(), provider: "NotDiamond".into(),
                context_window: 64_000, max_output_tokens: 4_096,
                input_cost_per_1k: 0.0003, output_cost_per_1k: 0.0012,
                supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
                training_cutoff: None, quality_rating: 4, speed_rating: 5,
                is_reasoning: false, free_tier: true,
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
    fn test_notdiamond_provider() { assert!(NotDiamondProvider::new("test-key").is_ok()); }
    #[test]
    fn test_notdiamond_models() { let p = NotDiamondProvider::new("test-key").unwrap(); assert!(p.models().len() >= 4); }
}
