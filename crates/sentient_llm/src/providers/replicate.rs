//! ─── Replicate Provider ───
//!
//! Implementation for Replicate API

use async_trait::async_trait;
use futures::{Stream, StreamExt};
use reqwest::Client;
use serde_json::json;
use std::pin::Pin;

use crate::error::{LlmError, LlmResult};
use crate::types::{ChatRequest, ChatResponse, ModelInfo, StreamChunk};
use crate::provider::LlmProvider;
use crate::models;

use super::{build_client, parse_api_error};

/// Replicate provider
pub struct ReplicateProvider {
    client: Client,
    api_key: String,
    base_url: String,
}

impl ReplicateProvider {
    pub fn new(api_key: impl Into<String>) -> LlmResult<Self> {
        Ok(Self {
            client: build_client()?,
            api_key: api_key.into(),
            base_url: "https://api.replicate.com/v1".into(),
        })
    }

    pub fn from_env() -> LlmResult<Self> {
        let api_key = std::env::var("REPLICATE_API_TOKEN")
            .map_err(|_| LlmError::Authentication("REPLICATE_API_TOKEN not set".into()))?;
        Self::new(api_key)
    }
}

#[async_trait]
impl LlmProvider for ReplicateProvider {
    fn name(&self) -> &str { "Replicate" }
    fn id(&self) -> &str { "replicate" }
    fn models(&self) -> Vec<ModelInfo> { models::replicate_models() }
    fn is_configured(&self) -> bool { !self.api_key.is_empty() }

    async fn chat(&self, request: ChatRequest) -> LlmResult<ChatResponse> {
        // Replicate uses a prediction API
        let response = self.client
            .post(format!("{}/predictions", self.base_url))
            .header("Authorization", format!("Token {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&json!({
                "version": request.model,
                "input": {
                    "prompt": request.messages.iter()
                        .filter_map(|m| m.content.as_text())
                        .collect::<Vec<_>>()
                        .join("\n"),
                    "max_tokens": request.max_tokens.unwrap_or(4096),
                    "temperature": request.temperature.unwrap_or(0.7),
                }
            }))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(LlmError::ApiError(parse_api_error(response).await));
        }

        let prediction: serde_json::Value = response.json().await?;
        
        // Poll for result
        let id = prediction["id"].as_str().ok_or_else(|| LlmError::ApiError("No prediction ID".into()))?;
        let mut status = prediction["status"].as_str().unwrap_or("starting").to_string();
        
        while status != "succeeded" && status != "failed" && status != "canceled" {
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
            
            let poll_response = self.client
                .get(format!("{}/predictions/{}", self.base_url, id))
                .header("Authorization", format!("Token {}", self.api_key))
                .send()
                .await?;
            
            let poll_json: serde_json::Value = poll_response.json().await?;
            status = poll_json["status"].as_str().unwrap_or("failed").to_string();
            
            if status == "succeeded" {
                let output = poll_json["output"].as_array();
                let text = output
                    .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect::<Vec<_>>().join(""))
                    .unwrap_or_default();
                
                return Ok(ChatResponse {
                    id: id.to_string(),
                    object: "chat.completion".into(),
                    created: chrono::Utc::now().timestamp(),
                    model: request.model,
                    choices: vec![crate::types::Choice {
                        index: 0,
                        message: crate::types::Message::assistant(&text),
                        finish_reason: Some("stop".into()),
                        logprobs: None,
                    }],
                    usage: crate::types::Usage::default(),
                    system_fingerprint: None,
                });
            }
        }
        
        Err(LlmError::ApiError(format!("Prediction {} with status: {}", id, status)))
    }

    async fn chat_stream(
        &self,
        _request: ChatRequest,
    ) -> LlmResult<Pin<Box<dyn Stream<Item = LlmResult<StreamChunk>> + Send>>> {
        // Replicate streaming is complex, return empty stream for now
        Err(LlmError::Unsupported("Streaming not supported for Replicate".into()))
    }

    fn count_tokens(&self, text: &str, _model: &str) -> LlmResult<usize> {
        Ok(text.len() / 4)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replicate_provider() {
        let provider = ReplicateProvider::new("test-key");
        assert!(provider.is_ok());
    }
}
