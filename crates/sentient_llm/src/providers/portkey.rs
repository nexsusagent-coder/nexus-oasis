//! ─── Portkey AI Provider ───
//!
//! Portkey - AI Gateway, Router & Observability Platform
//! https://portkey.ai
//!
//! Features:
//! - 250+ model support through unified API
//! - Automatic failover & load balancing
//! - Caching, retries, rate limiting
//! - Full observability (logs, metrics, traces)
//! - Prompt templates & virtual keys

use async_trait::async_trait;
use futures::{Stream, StreamExt};
use reqwest::Client;
use std::pin::Pin;

use crate::error::{LlmError, LlmResult};
use crate::types::{ChatRequest, ChatResponse, ModelInfo, StreamChunk};
use crate::provider::LlmProvider;

use super::{build_client, parse_api_error};

/// Portkey AI provider - Gateway + Router + Observability
pub struct PortkeyProvider {
    client: Client,
    api_key: String,
    base_url: String,
    virtual_key: Option<String>,
}

impl PortkeyProvider {
    pub fn new(api_key: impl Into<String>) -> LlmResult<Self> {
        Ok(Self {
            client: build_client()?,
            api_key: api_key.into(),
            base_url: "https://api.portkey.ai/v1".into(),
            virtual_key: None,
        })
    }

    pub fn with_virtual_key(mut self, vk: impl Into<String>) -> Self {
        self.virtual_key = Some(vk.into());
        self
    }

    pub fn from_env() -> LlmResult<Self> {
        let api_key = std::env::var("PORTKEY_API_KEY")
            .map_err(|_| LlmError::Authentication("PORTKEY_API_KEY not set".into()))?;
        let mut provider = Self::new(api_key)?;
        provider.virtual_key = std::env::var("PORTKEY_VIRTUAL_KEY").ok();
        Ok(provider)
    }
}

#[async_trait]
impl LlmProvider for PortkeyProvider {
    fn name(&self) -> &str { "Portkey" }
    fn id(&self) -> &str { "portkey" }

    fn models(&self) -> Vec<ModelInfo> {
        vec![
            // Portkey Router configs
            ModelInfo {
                id: "portkey/gpt-4o".into(), name: "GPT-4o (Portkey)".into(), provider: "Portkey".into(),
                context_window: 128_000, max_output_tokens: 16_384,
                input_cost_per_1k: 0.0025, output_cost_per_1k: 0.01,
                supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
                training_cutoff: Some("2024-04".into()), quality_rating: 5, speed_rating: 4,
                is_reasoning: false, free_tier: false,
            },
            ModelInfo {
                id: "portkey/claude-4-sonnet".into(), name: "Claude Sonnet 4 (Portkey)".into(), provider: "Portkey".into(),
                context_window: 200_000, max_output_tokens: 16_384,
                input_cost_per_1k: 0.003, output_cost_per_1k: 0.015,
                supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
                training_cutoff: Some("2025-02".into()), quality_rating: 5, speed_rating: 4,
                is_reasoning: false, free_tier: false,
            },
            ModelInfo {
                id: "portkey/gemini-2.5-pro".into(), name: "Gemini 2.5 Pro (Portkey)".into(), provider: "Portkey".into(),
                context_window: 1_048_576, max_output_tokens: 65_536,
                input_cost_per_1k: 0.00125, output_cost_per_1k: 0.01,
                supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
                training_cutoff: Some("2025-01".into()), quality_rating: 5, speed_rating: 3,
                is_reasoning: true, free_tier: false,
            },
            ModelInfo {
                id: "portkey/deepseek-r1".into(), name: "DeepSeek R1 (Portkey)".into(), provider: "Portkey".into(),
                context_window: 64_000, max_output_tokens: 8_192,
                input_cost_per_1k: 0.00055, output_cost_per_1k: 0.00219,
                supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
                training_cutoff: Some("2024-11".into()), quality_rating: 5, speed_rating: 3,
                is_reasoning: true, free_tier: true,
            },
            ModelInfo {
                id: "portkey/llama-4-maverick".into(), name: "Llama 4 Maverick (Portkey)".into(), provider: "Portkey".into(),
                context_window: 1_048_576, max_output_tokens: 16_384,
                input_cost_per_1k: 0.0015, output_cost_per_1k: 0.002,
                supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
                training_cutoff: Some("2025-04".into()), quality_rating: 5, speed_rating: 4,
                is_reasoning: false, free_tier: false,
            },
            ModelInfo {
                id: "portkey/grok-3".into(), name: "Grok 3 (Portkey)".into(), provider: "Portkey".into(),
                context_window: 131_072, max_output_tokens: 16_384,
                input_cost_per_1k: 0.003, output_cost_per_1k: 0.015,
                supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
                training_cutoff: Some("2025-02".into()), quality_rating: 5, speed_rating: 4,
                is_reasoning: false, free_tier: false,
            },
            ModelInfo {
                id: "portkey/mistral-large-2".into(), name: "Mistral Large 2 (Portkey)".into(), provider: "Portkey".into(),
                context_window: 128_000, max_output_tokens: 4_096,
                input_cost_per_1k: 0.002, output_cost_per_1k: 0.006,
                supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
                training_cutoff: Some("2024-07".into()), quality_rating: 5, speed_rating: 4,
                is_reasoning: false, free_tier: false,
            },
            ModelInfo {
                id: "portkey/command-a".into(), name: "Command A (Portkey)".into(), provider: "Portkey".into(),
                context_window: 256_000, max_output_tokens: 4_096,
                input_cost_per_1k: 0.002, output_cost_per_1k: 0.008,
                supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
                training_cutoff: Some("2025-02".into()), quality_rating: 5, speed_rating: 4,
                is_reasoning: false, free_tier: false,
            },
        ]
    }

    fn is_configured(&self) -> bool { !self.api_key.is_empty() }

    async fn chat(&self, request: ChatRequest) -> LlmResult<ChatResponse> {
        let mut headers = vec![
            ("Authorization", format!("Bearer {}", self.api_key)),
            ("x-portkey-api-key", self.api_key.clone()),
        ];
        if let Some(ref vk) = self.virtual_key {
            headers.push(("x-portkey-virtual-key", vk.clone()));
        }

        let mut req = self.client
            .post(format!("{}/chat/completions", self.base_url));
        for (k, v) in headers { req = req.header(k, v); }

        let response = req.json(&request).send().await
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

        let mut req = self.client
            .post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("x-portkey-api-key", &self.api_key);
        if let Some(ref vk) = self.virtual_key {
            req = req.header("x-portkey-virtual-key", vk);
        }

        let response = req.json(&request).send().await
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
    fn test_portkey_provider() { assert!(PortkeyProvider::new("test-key").is_ok()); }
    #[test]
    fn test_portkey_models() { let p = PortkeyProvider::new("test-key").unwrap(); assert!(p.models().len() >= 8); }
}
