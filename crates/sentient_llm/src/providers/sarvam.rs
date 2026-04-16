//! ─── Sarvam AI Provider ───
//!
//! Sarvam AI - Indian multilingual AI models
//! https://sarvam.ai

use async_trait::async_trait;
use futures::{Stream, StreamExt};
use reqwest::Client;
use std::pin::Pin;

use crate::error::{LlmError, LlmResult};
use crate::types::{ChatRequest, ChatResponse, ModelInfo, StreamChunk};
use crate::provider::LlmProvider;

use super::{build_client, parse_api_error};

/// Sarvam AI provider - Indian multilingual AI
pub struct SarvamProvider {
    client: Client,
    api_key: String,
    base_url: String,
}

impl SarvamProvider {
    pub fn new(api_key: impl Into<String>) -> LlmResult<Self> {
        Ok(Self {
            client: build_client()?,
            api_key: api_key.into(),
            base_url: "https://api.sarvam.ai/v1".into(),
        })
    }

    pub fn from_env() -> LlmResult<Self> {
        let api_key = std::env::var("SARVAM_API_KEY")
            .map_err(|_| LlmError::Authentication("SARVAM_API_KEY not set".into()))?;
        Self::new(api_key)
    }
}

#[async_trait]
impl LlmProvider for SarvamProvider {
    fn name(&self) -> &str { "Sarvam AI" }
    fn id(&self) -> &str { "sarvam" }

    fn models(&self) -> Vec<ModelInfo> {
        vec![
            ModelInfo {
                id: "sarvam-m".into(),
                name: "Sarvam-M".into(),
                provider: "Sarvam".into(),
                context_window: 32_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.0002,
                output_cost_per_1k: 0.0006,
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: Some("2024-10".into()),
                quality_rating: 4,
                speed_rating: 4,
                is_reasoning: false,
                free_tier: true,
            },
            ModelInfo {
                id: "sarvam-2b".into(),
                name: "Sarvam 2B".into(),
                provider: "Sarvam".into(),
                context_window: 8_192,
                max_output_tokens: 2_048,
                input_cost_per_1k: 0.00002,
                output_cost_per_1k: 0.00006,
                supports_vision: false,
                supports_tools: false,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: Some("2024-06".into()),
                quality_rating: 2,
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
        Ok(text.len() / 3) // Multilingual token ratio
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sarvam_provider() { assert!(SarvamProvider::new("test-key").is_ok()); }
}
