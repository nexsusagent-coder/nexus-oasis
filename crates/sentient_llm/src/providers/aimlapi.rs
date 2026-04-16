//! ─── AI/ML API Provider ───
//!
//! AI/ML API - 100+ models through single API, GPT-4 level at lower cost
//! https://aimlapi.com
//!
//! Features:
//! - 100+ models from 30+ providers
//! - OpenAI-compatible API
//! - Significantly cheaper than direct providers
//! - Serverless inference
//! - Image generation (DALL-E, Stable Diffusion, FLUX)

use async_trait::async_trait;
use futures::{Stream, StreamExt};
use reqwest::Client;
use std::pin::Pin;

use crate::error::{LlmError, LlmResult};
use crate::types::{ChatRequest, ChatResponse, ModelInfo, StreamChunk};
use crate::provider::LlmProvider;

use super::{build_client, parse_api_error};

/// AI/ML API provider - 100+ models at lower cost
pub struct AiMlApiProvider {
    client: Client,
    api_key: String,
    base_url: String,
}

impl AiMlApiProvider {
    pub fn new(api_key: impl Into<String>) -> LlmResult<Self> {
        Ok(Self {
            client: build_client()?,
            api_key: api_key.into(),
            base_url: "https://api.aimlapi.com/v1".into(),
        })
    }

    pub fn from_env() -> LlmResult<Self> {
        let api_key = std::env::var("AIMLAPI_API_KEY")
            .map_err(|_| LlmError::Authentication("AIMLAPI_API_KEY not set".into()))?;
        Self::new(api_key)
    }
}

#[async_trait]
impl LlmProvider for AiMlApiProvider {
    fn name(&self) -> &str { "AI/ML API" }
    fn id(&self) -> &str { "aimlapi" }

    fn models(&self) -> Vec<ModelInfo> {
        vec![
            ModelInfo {
                id: "gpt-4o".into(), name: "GPT-4o (AI/ML API)".into(), provider: "AI/ML API".into(),
                context_window: 128_000, max_output_tokens: 16_384,
                input_cost_per_1k: 0.0015, output_cost_per_1k: 0.006,
                supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
                training_cutoff: Some("2024-04".into()), quality_rating: 5, speed_rating: 4,
                is_reasoning: false, free_tier: false,
            },
            ModelInfo {
                id: "claude-4-sonnet".into(), name: "Claude Sonnet 4 (AI/ML API)".into(), provider: "AI/ML API".into(),
                context_window: 200_000, max_output_tokens: 16_384,
                input_cost_per_1k: 0.002, output_cost_per_1k: 0.01,
                supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
                training_cutoff: Some("2025-02".into()), quality_rating: 5, speed_rating: 4,
                is_reasoning: false, free_tier: false,
            },
            ModelInfo {
                id: "gemini-2.5-flash".into(), name: "Gemini 2.5 Flash (AI/ML API)".into(), provider: "AI/ML API".into(),
                context_window: 1_048_576, max_output_tokens: 65_536,
                input_cost_per_1k: 0.0001, output_cost_per_1k: 0.0004,
                supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
                training_cutoff: Some("2025-01".into()), quality_rating: 5, speed_rating: 5,
                is_reasoning: true, free_tier: true,
            },
            ModelInfo {
                id: "deepseek-r1".into(), name: "DeepSeek R1 (AI/ML API)".into(), provider: "AI/ML API".into(),
                context_window: 64_000, max_output_tokens: 8_192,
                input_cost_per_1k: 0.00035, output_cost_per_1k: 0.0014,
                supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
                training_cutoff: Some("2024-11".into()), quality_rating: 5, speed_rating: 3,
                is_reasoning: true, free_tier: true,
            },
            ModelInfo {
                id: "llama-4-maverick-17b-128e".into(), name: "Llama 4 Maverick (AI/ML API)".into(), provider: "AI/ML API".into(),
                context_window: 1_048_576, max_output_tokens: 16_384,
                input_cost_per_1k: 0.001, output_cost_per_1k: 0.0015,
                supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
                training_cutoff: Some("2025-04".into()), quality_rating: 5, speed_rating: 4,
                is_reasoning: false, free_tier: false,
            },
            ModelInfo {
                id: "qwen3-235b-a22b".into(), name: "Qwen3 235B (AI/ML API)".into(), provider: "AI/ML API".into(),
                context_window: 131_072, max_output_tokens: 16_384,
                input_cost_per_1k: 0.001, output_cost_per_1k: 0.003,
                supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
                training_cutoff: Some("2025-03".into()), quality_rating: 5, speed_rating: 4,
                is_reasoning: true, free_tier: false,
            },
            ModelInfo {
                id: "grok-3".into(), name: "Grok 3 (AI/ML API)".into(), provider: "AI/ML API".into(),
                context_window: 131_072, max_output_tokens: 16_384,
                input_cost_per_1k: 0.002, output_cost_per_1k: 0.01,
                supports_vision: true, supports_tools: true, supports_streaming: true, supports_json: true,
                training_cutoff: Some("2025-02".into()), quality_rating: 5, speed_rating: 4,
                is_reasoning: false, free_tier: false,
            },
            ModelInfo {
                id: "mistral-large-2".into(), name: "Mistral Large 2 (AI/ML API)".into(), provider: "AI/ML API".into(),
                context_window: 128_000, max_output_tokens: 4_096,
                input_cost_per_1k: 0.0015, output_cost_per_1k: 0.0045,
                supports_vision: false, supports_tools: true, supports_streaming: true, supports_json: true,
                training_cutoff: Some("2024-07".into()), quality_rating: 5, speed_rating: 4,
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
    fn test_aimlapi_provider() { assert!(AiMlApiProvider::new("test-key").is_ok()); }
    #[test]
    fn test_aimlapi_models() { let p = AiMlApiProvider::new("test-key").unwrap(); assert!(p.models().len() >= 8); }
}
