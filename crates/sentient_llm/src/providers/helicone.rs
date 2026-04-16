//! ─── Helicone AI Provider ───
//!
//! Helicone - AI Observability, Proxy & Cost Management
//! https://helicone.ai
//!
//! Features:
//! - LLM Proxy (OpenAI-compatible gateway)
//! - Cost tracking per request/model/user
//! - Logging & observability
//! - Rate limiting & caching
//! - Prompt management

use async_trait::async_trait;
use futures::{Stream, StreamExt};
use reqwest::Client;
use std::pin::Pin;

use crate::error::{LlmError, LlmResult};
use crate::types::{ChatRequest, ChatResponse, ModelInfo, StreamChunk};
use crate::provider::LlmProvider;

use super::{build_client, parse_api_error};

/// Helicone AI provider - Observability Proxy
pub struct HeliconeProvider {
    client: Client,
    api_key: String,
    base_url: String,
}

impl HeliconeProvider {
    pub fn new(api_key: impl Into<String>) -> LlmResult<Self> {
        Ok(Self {
            client: build_client()?,
            api_key: api_key.into(),
            base_url: "https://gateway.helicone.ai/v1".into(),
        })
    }

    pub fn from_env() -> LlmResult<Self> {
        let api_key = std::env::var("HELICONE_API_KEY")
            .map_err(|_| LlmError::Authentication("HELICONE_API_KEY not set".into()))?;
        Self::new(api_key)
    }
}

#[async_trait]
impl LlmProvider for HeliconeProvider {
    fn name(&self) -> &str { "Helicone" }
    fn id(&self) -> &str { "helicone" }

    fn models(&self) -> Vec<ModelInfo> {
        vec![
            ModelInfo {
                id: "helicone/gpt-4o".into(), name: "GPT-4o (Helicone)".into(), provider: "Helicone".into(),
                context_window: 128_000, max_output_tokens: 16_384,
                input_cost_per_1k: 0.0025, output_cost_per_1k: 0.01,
                supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
                training_cutoff: Some("2024-04".into()), quality_rating: 5, speed_rating: 4,
                is_reasoning: false, free_tier: false,
            },
            ModelInfo {
                id: "helicone/claude-4-sonnet".into(), name: "Claude Sonnet 4 (Helicone)".into(), provider: "Helicone".into(),
                context_window: 200_000, max_output_tokens: 16_384,
                input_cost_per_1k: 0.003, output_cost_per_1k: 0.015,
                supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
                training_cutoff: Some("2025-02".into()), quality_rating: 5, speed_rating: 4,
                is_reasoning: false, free_tier: false,
            },
            ModelInfo {
                id: "helicone/gemini-2.5-flash".into(), name: "Gemini 2.5 Flash (Helicone)".into(), provider: "Helicone".into(),
                context_window: 1_048_576, max_output_tokens: 65_536,
                input_cost_per_1k: 0.00015, output_cost_per_1k: 0.0006,
                supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
                training_cutoff: Some("2025-01".into()), quality_rating: 5, speed_rating: 5,
                is_reasoning: true, free_tier: true,
            },
            ModelInfo {
                id: "helicone/deepseek-r1".into(), name: "DeepSeek R1 (Helicone)".into(), provider: "Helicone".into(),
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
            .header("Helicone-Auth", format!("Bearer {}", self.api_key))
            .json(&request)
            .send().await
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
            .header("Helicone-Auth", format!("Bearer {}", self.api_key))
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
    fn test_helicone_provider() { assert!(HeliconeProvider::new("test-key").is_ok()); }
    #[test]
    fn test_helicone_models() { let p = HeliconeProvider::new("test-key").unwrap(); assert!(p.models().len() >= 4); }
}
