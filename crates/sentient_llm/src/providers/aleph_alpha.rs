//! ─── Aleph Alpha Provider ───
//!
//! Aleph Alpha - European sovereign AI (Luminous series)
//! https://aleph-alpha.com

use async_trait::async_trait;
use futures::{Stream, StreamExt};
use reqwest::Client;
use std::pin::Pin;

use crate::error::{LlmError, LlmResult};
use crate::types::{ChatRequest, ChatResponse, ModelInfo, StreamChunk};
use crate::provider::LlmProvider;

use super::{build_client, parse_api_error};

/// Aleph Alpha provider - European sovereign AI
pub struct AlephAlphaProvider {
    client: Client,
    api_key: String,
    base_url: String,
}

impl AlephAlphaProvider {
    pub fn new(api_key: impl Into<String>) -> LlmResult<Self> {
        Ok(Self {
            client: build_client()?,
            api_key: api_key.into(),
            base_url: "https://api.aleph-alpha.com/v1".into(),
        })
    }

    pub fn from_env() -> LlmResult<Self> {
        let api_key = std::env::var("ALEPH_ALPHA_API_KEY")
            .map_err(|_| LlmError::Authentication("ALEPH_ALPHA_API_KEY not set".into()))?;
        Self::new(api_key)
    }
}

#[async_trait]
impl LlmProvider for AlephAlphaProvider {
    fn name(&self) -> &str { "Aleph Alpha" }
    fn id(&self) -> &str { "aleph_alpha" }

    fn models(&self) -> Vec<ModelInfo> {
        vec![
            ModelInfo {
                id: "luminous-supreme-control".into(),
                name: "Luminous Supreme Control".into(),
                provider: "Aleph Alpha".into(),
                context_window: 32_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.03,
                output_cost_per_1k: 0.03,
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: Some("2024-01".into()),
                quality_rating: 4,
                speed_rating: 3,
                is_reasoning: false,
                free_tier: false,
            },
            ModelInfo {
                id: "luminous-base-control".into(),
                name: "Luminous Base Control".into(),
                provider: "Aleph Alpha".into(),
                context_window: 8_192,
                max_output_tokens: 2_048,
                input_cost_per_1k: 0.006,
                output_cost_per_1k: 0.006,
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: Some("2023-06".into()),
                quality_rating: 3,
                speed_rating: 5,
                is_reasoning: false,
                free_tier: true,
            },
            ModelInfo {
                id: "pharia-1-llm-7b-control".into(),
                name: "Pharia 1 LLM 7B Control".into(),
                provider: "Aleph Alpha".into(),
                context_window: 32_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.0002,
                output_cost_per_1k: 0.0002,
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: Some("2024-06".into()),
                quality_rating: 3,
                speed_rating: 5,
                is_reasoning: false,
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
    fn test_aleph_provider() { assert!(AlephAlphaProvider::new("test-key").is_ok()); }
    #[test]
    fn test_aleph_models() { let p = AlephAlphaProvider::new("test-key").unwrap(); assert!(p.models().len() >= 3); }
}
