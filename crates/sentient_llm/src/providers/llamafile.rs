//! ─── Llamafile Provider ───
//!
//! Mozilla Llamafile - Single-file LLM distribution
//! https://llamafile.ai

use async_trait::async_trait;
use futures::{Stream, StreamExt};
use reqwest::Client;
use std::pin::Pin;

use crate::error::{LlmError, LlmResult};
use crate::types::{ChatRequest, ChatResponse, ModelInfo, StreamChunk, Role};
use crate::provider::LlmProvider;

use super::build_client;

/// Llamafile provider - Local single-file LLM
pub struct LlamafileProvider {
    client: Client,
    base_url: String,
}

impl LlamafileProvider {
    pub fn new() -> LlmResult<Self> {
        Ok(Self {
            client: build_client()?,
            base_url: std::env::var("LLAMAFILE_HOST")
                .unwrap_or_else(|_| "http://localhost:8080".into()),
        })
    }

    pub fn with_base_url(base_url: impl Into<String>) -> LlmResult<Self> {
        Ok(Self {
            client: build_client()?,
            base_url: base_url.into(),
        })
    }

    pub async fn is_running(&self) -> bool {
        self.client
            .get(format!("{}/health", self.base_url))
            .send()
            .await
            .map(|r| r.status().is_success())
            .unwrap_or(false)
    }
}

#[async_trait]
impl LlmProvider for LlamafileProvider {
    fn name(&self) -> &str { "Llamafile" }
    fn id(&self) -> &str { "llamafile" }

    fn models(&self) -> Vec<ModelInfo> {
        vec![
            ModelInfo {
                id: "llamafile-default".into(),
                name: "Llamafile Default".into(),
                provider: "Llamafile".into(),
                context_window: 32_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.0,
                output_cost_per_1k: 0.0,
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: None,
                quality_rating: 4,
                speed_rating: 4,
                is_reasoning: false,
                free_tier: true,
            },
        ]
    }

    fn is_configured(&self) -> bool { true }

    async fn chat(&self, request: ChatRequest) -> LlmResult<ChatResponse> {
        let response = self.client
            .post(format!("{}/v1/chat/completions", self.base_url))
            .json(&request)
            .send()
            .await
            .map_err(|e| LlmError::Network(e.to_string()))?;

        if !response.status().is_success() {
            let err = response.text().await.unwrap_or_default();
            return Err(LlmError::ApiError(format!("Llamafile error: {}", err)));
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
            .post(format!("{}/v1/chat/completions", self.base_url))
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
    fn test_llamafile_provider() { assert!(LlamafileProvider::new().is_ok()); }
}
