//! ─── Fireworks Provider ───
//!
//! Implementation for Fireworks AI API

use async_trait::async_trait;
use futures::{Stream, StreamExt};
use reqwest::Client;
use std::pin::Pin;

use crate::error::{LlmError, LlmResult};
use crate::types::{ChatRequest, ChatResponse, ModelInfo, StreamChunk};
use crate::provider::LlmProvider;
use crate::models;

use super::{build_client, parse_api_error};

/// Fireworks AI provider
pub struct FireworksProvider {
    client: Client,
    api_key: String,
    base_url: String,
}

impl FireworksProvider {
    pub fn new(api_key: impl Into<String>) -> LlmResult<Self> {
        Ok(Self {
            client: build_client()?,
            api_key: api_key.into(),
            base_url: "https://api.fireworks.ai/inference/v1".into(),
        })
    }

    pub fn from_env() -> LlmResult<Self> {
        let api_key = std::env::var("FIREWORKS_API_KEY")
            .map_err(|_| LlmError::Authentication("FIREWORKS_API_KEY not set".into()))?;
        Self::new(api_key)
    }
}

#[async_trait]
impl LlmProvider for FireworksProvider {
    fn name(&self) -> &str { "Fireworks" }
    fn id(&self) -> &str { "fireworks" }
    fn models(&self) -> Vec<ModelInfo> { models::fireworks_models() }
    fn is_configured(&self) -> bool { !self.api_key.is_empty() }

    async fn chat(&self, request: ChatRequest) -> LlmResult<ChatResponse> {
        let response = self.client
            .post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({
                "model": request.model,
                "messages": request.messages.iter().map(|m| serde_json::json!({
                    "role": match m.role {
                        crate::types::Role::System => "system",
                        crate::types::Role::User => "user",
                        crate::types::Role::Assistant => "assistant",
                        _ => "user",
                    },
                    "content": m.content.as_text().unwrap_or(""),
                })).collect::<Vec<_>>(),
                "max_tokens": request.max_tokens,
                "temperature": request.temperature,
            }))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(LlmError::ApiError(parse_api_error(response).await));
        }

        let json: serde_json::Value = response.json().await?;
        Ok(parse_openai_response(json))
    }

    async fn chat_stream(
        &self,
        request: ChatRequest,
    ) -> LlmResult<Pin<Box<dyn Stream<Item = LlmResult<StreamChunk>> + Send>>> {
        let response = self.client
            .post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({
                "model": request.model,
                "messages": request.messages.iter().map(|m| serde_json::json!({
                    "role": match m.role {
                        crate::types::Role::System => "system",
                        crate::types::Role::User => "user",
                        crate::types::Role::Assistant => "assistant",
                        _ => "user",
                    },
                    "content": m.content.as_text().unwrap_or(""),
                })).collect::<Vec<_>>(),
                "stream": true,
            }))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(LlmError::ApiError(parse_api_error(response).await));
        }

        let model = request.model.clone();
        let stream = response.bytes_stream()
            .map(move |result| {
                match result {
                    Ok(bytes) => {
                        let text = String::from_utf8_lossy(&bytes);
                        for line in text.lines() {
                            if line.starts_with("data: ") {
                                let data = &line[6..];
                                if data == "[DONE]" { return Ok(None); }
                                if let Ok(json) = serde_json::from_str::<serde_json::Value>(data) {
                                    return Ok(Some(parse_stream_chunk(json, &model)));
                                }
                            }
                        }
                        Ok(None)
                    }
                    Err(e) => Err(LlmError::StreamError(e.to_string())),
                }
            })
            .filter_map(|result| async move { result.transpose() });

        Ok(Box::pin(stream))
    }

    fn count_tokens(&self, text: &str, _model: &str) -> LlmResult<usize> {
        Ok(text.len() / 4)
    }
}

fn parse_openai_response(json: serde_json::Value) -> ChatResponse {
    ChatResponse {
        id: json["id"].as_str().unwrap_or("").to_string(),
        object: json["object"].as_str().unwrap_or("chat.completion").to_string(),
        created: json["created"].as_i64().unwrap_or(0),
        model: json["model"].as_str().unwrap_or("").to_string(),
        choices: json["choices"].as_array().map(|arr| {
            arr.iter().map(|c| crate::types::Choice {
                index: c["index"].as_u64().unwrap_or(0) as u32,
                message: crate::types::Message::assistant(
                    c["message"]["content"].as_str().unwrap_or("")
                ),
                finish_reason: c["finish_reason"].as_str().map(|s| s.to_string()),
                logprobs: None,
            }).collect()
        }).unwrap_or_default(),
        usage: crate::types::Usage {
            prompt_tokens: json["usage"]["prompt_tokens"].as_u64().unwrap_or(0) as u32,
            completion_tokens: json["usage"]["completion_tokens"].as_u64().unwrap_or(0) as u32,
            total_tokens: json["usage"]["total_tokens"].as_u64().unwrap_or(0) as u32,
        },
        system_fingerprint: None,
    }
}

fn parse_stream_chunk(json: serde_json::Value, model: &str) -> StreamChunk {
    StreamChunk {
        id: json["id"].as_str().unwrap_or("").to_string(),
        object: json["object"].as_str().unwrap_or("chat.completion.chunk").to_string(),
        created: json["created"].as_i64().unwrap_or(0),
        model: model.to_string(),
        system_fingerprint: None,
        choices: json["choices"].as_array().map(|arr| {
            arr.iter().map(|c| crate::types::StreamChoice {
                index: c["index"].as_u64().unwrap_or(0) as u32,
                delta: crate::types::Delta {
                    role: Some(crate::types::Role::Assistant),
                    content: c["delta"]["content"].as_str().map(|s| s.to_string()),
                    tool_calls: None,
                },
                finish_reason: c["finish_reason"].as_str().map(|s| s.to_string()),
            }).collect()
        }).unwrap_or_default(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fireworks_provider() {
        let provider = FireworksProvider::new("test-key");
        assert!(provider.is_ok());
    }
}
