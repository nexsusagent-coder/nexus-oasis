//! ─── Qwen Direct Provider ───
//!
//! Alibaba Cloud Qwen models - Direct API access
//! https://help.aliyun.com/product/2400365.html

use async_trait::async_trait;
use futures::{Stream, StreamExt};
use reqwest::Client;
use std::pin::Pin;

use crate::error::{LlmError, LlmResult};
use crate::types::{ChatRequest, ChatResponse, ModelInfo, StreamChunk};
use crate::provider::LlmProvider;

use super::{build_client, parse_api_error};

/// Qwen (Alibaba Cloud) provider
pub struct QwenProvider {
    client: Client,
    api_key: String,
    base_url: String,
}

impl QwenProvider {
    pub fn new(api_key: impl Into<String>) -> LlmResult<Self> {
        Ok(Self {
            client: build_client()?,
            api_key: api_key.into(),
            base_url: "https://dashscope.aliyuncs.com/compatible-mode/v1".into(),
        })
    }

    pub fn from_env() -> LlmResult<Self> {
        let api_key = std::env::var("DASHSCOPE_API_KEY")
            .map_err(|_| LlmError::Authentication("DASHSCOPE_API_KEY not set".into()))?;
        Self::new(api_key)
    }
}

#[async_trait]
impl LlmProvider for QwenProvider {
    fn name(&self) -> &str { "Qwen (Alibaba)" }
    fn id(&self) -> &str { "qwen" }

    fn models(&self) -> Vec<ModelInfo> {
        vec![
            // Qwen3 series (2025)
            ModelInfo {
                id: "qwen3-235b-a22b".into(),
                name: "Qwen3 235B-A22B (MoE)".into(),
                provider: "Qwen".into(),
                context_window: 128_000,
                max_output_tokens: 16_384,
                input_cost_per_1k: 0.0014,
                output_cost_per_1k: 0.0042,
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: Some("2025-03".into()),
                quality_rating: 5,
                speed_rating: 4,
                is_reasoning: true,
                free_tier: true,
            },
            ModelInfo {
                id: "qwen3-32b".into(),
                name: "Qwen3 32B".into(),
                provider: "Qwen".into(),
                context_window: 128_000,
                max_output_tokens: 16_384,
                input_cost_per_1k: 0.0005,
                output_cost_per_1k: 0.0015,
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: Some("2025-03".into()),
                quality_rating: 5,
                speed_rating: 4,
                is_reasoning: true,
                free_tier: true,
            },
            ModelInfo {
                id: "qwen3-30b-a3b".into(),
                name: "Qwen3 30B-A3B (MoE)".into(),
                provider: "Qwen".into(),
                context_window: 128_000,
                max_output_tokens: 8_192,
                input_cost_per_1k: 0.0003,
                output_cost_per_1k: 0.0009,
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: Some("2025-03".into()),
                quality_rating: 4,
                speed_rating: 5,
                is_reasoning: true,
                free_tier: true,
            },
            // Qwen 2.5 series
            ModelInfo {
                id: "qwen2.5-72b-instruct".into(),
                name: "Qwen 2.5 72B".into(),
                provider: "Qwen".into(),
                context_window: 131_072,
                max_output_tokens: 8_192,
                input_cost_per_1k: 0.0004,
                output_cost_per_1k: 0.0012,
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: Some("2024-09".into()),
                quality_rating: 5,
                speed_rating: 4,
                is_reasoning: false,
                free_tier: true,
            },
            ModelInfo {
                id: "qwen2.5-coder-32b-instruct".into(),
                name: "Qwen 2.5 Coder 32B".into(),
                provider: "Qwen".into(),
                context_window: 131_072,
                max_output_tokens: 8_192,
                input_cost_per_1k: 0.0002,
                output_cost_per_1k: 0.0006,
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: Some("2024-11".into()),
                quality_rating: 5,
                speed_rating: 4,
                is_reasoning: false,
                free_tier: true,
            },
            // QwQ (reasoning)
            ModelInfo {
                id: "qwq-32b".into(),
                name: "QwQ 32B (Reasoning)".into(),
                provider: "Qwen".into(),
                context_window: 131_072,
                max_output_tokens: 16_384,
                input_cost_per_1k: 0.0005,
                output_cost_per_1k: 0.0015,
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: Some("2025-02".into()),
                quality_rating: 5,
                speed_rating: 3,
                is_reasoning: true,
                free_tier: true,
            },
            // QVQ (vision reasoning)
            ModelInfo {
                id: "qvq-72b-preview".into(),
                name: "QVQ 72B (Vision Reasoning)".into(),
                provider: "Qwen".into(),
                context_window: 131_072,
                max_output_tokens: 16_384,
                input_cost_per_1k: 0.0008,
                output_cost_per_1k: 0.0024,
                supports_vision: true,
                supports_tools: true,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: Some("2024-12".into()),
                quality_rating: 5,
                speed_rating: 3,
                is_reasoning: true,
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
        Ok(text.len() / 3) // Chinese characters are ~1.5 tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_qwen_provider() { assert!(QwenProvider::new("test-key").is_ok()); }
    #[test]
    fn test_qwen_models() { let p = QwenProvider::new("test-key").unwrap(); assert!(p.models().len() >= 7); }
}
