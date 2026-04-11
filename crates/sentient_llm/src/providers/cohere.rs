//! ─── Cohere Provider ───
//!
//! Implementation for Cohere API

use async_trait::async_trait;
use futures::{Stream, StreamExt};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::pin::Pin;

use crate::error::{LlmError, LlmResult};
use crate::types::{ChatRequest, ChatResponse, ModelInfo, StreamChunk, Role, Content};
use crate::provider::LlmProvider;
use crate::models;

use super::{build_client, parse_api_error};

/// Cohere provider
pub struct CohereProvider {
    client: Client,
    api_key: String,
    base_url: String,
}

impl CohereProvider {
    pub fn new(api_key: impl Into<String>) -> LlmResult<Self> {
        Ok(Self {
            client: build_client()?,
            api_key: api_key.into(),
            base_url: "https://api.cohere.ai/v1".into(),
        })
    }

    pub fn from_env() -> LlmResult<Self> {
        let api_key = std::env::var("COHERE_API_KEY")
            .map_err(|_| LlmError::Authentication("COHERE_API_KEY not set".into()))?;
        Self::new(api_key)
    }

    fn convert_request(&self, request: ChatRequest) -> CohereRequest {
        let mut preamble = None;
        let mut messages: Vec<CohereMessage> = Vec::new();

        for msg in request.messages {
            match msg.role {
                Role::System => {
                    preamble = msg.content.as_text().map(|s| s.to_string());
                }
                Role::User => {
                    messages.push(CohereMessage {
                        role: "user".into(),
                        content: msg.content.as_text().map(|s| s.to_string()).unwrap_or_default(),
                    });
                }
                Role::Assistant => {
                    messages.push(CohereMessage {
                        role: "assistant".into(),
                        content: msg.content.as_text().map(|s| s.to_string()).unwrap_or_default(),
                    });
                }
                _ => {}
            }
        }

        CohereRequest {
            model: request.model,
            messages,
            preamble,
            max_tokens: request.max_tokens,
            temperature: request.temperature,
            p: request.top_p,
            stream: request.stream,
        }
    }
}

#[async_trait]
impl LlmProvider for CohereProvider {
    fn name(&self) -> &str { "Cohere" }
    fn id(&self) -> &str { "cohere" }
    fn models(&self) -> Vec<ModelInfo> { models::cohere_models() }
    fn is_configured(&self) -> bool { !self.api_key.is_empty() }

    async fn chat(&self, request: ChatRequest) -> LlmResult<ChatResponse> {
        let cohere_request = self.convert_request(request);

        let response = self.client
            .post(format!("{}/chat", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&cohere_request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(LlmError::ApiError(parse_api_error(response).await));
        }

        let cohere_response: CohereResponse = response.json().await?;
        Ok(cohere_response.into())
    }

    async fn chat_stream(
        &self,
        request: ChatRequest,
    ) -> LlmResult<Pin<Box<dyn Stream<Item = LlmResult<StreamChunk>> + Send>>> {
        let mut cohere_request = self.convert_request(request);
        cohere_request.stream = true;

        let response = self.client
            .post(format!("{}/chat", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&cohere_request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(LlmError::ApiError(parse_api_error(response).await));
        }

        let model = cohere_request.model.clone();
        let stream = response.bytes_stream()
            .map(move |result| {
                match result {
                    Ok(bytes) => {
                        let text = String::from_utf8_lossy(&bytes);
                        for line in text.lines() {
                            if line.starts_with("data: ") {
                                let data = &line[6..];
                                if let Ok(event) = serde_json::from_str::<CohereStreamEvent>(data) {
                                    if event.event_type == "text-generation" {
                                        return Ok(Some(StreamChunk {
                                            id: uuid::Uuid::new_v4().to_string(),
                                            object: "chat.completion.chunk".into(),
                                            created: chrono::Utc::now().timestamp(),
                                            model: model.clone(),
                                            system_fingerprint: None,
                                            choices: vec![crate::types::StreamChoice {
                                                index: 0,
                                                delta: crate::types::Delta {
                                                    role: Some(Role::Assistant),
                                                    content: Some(event.text),
                                                    tool_calls: None,
                                                },
                                                finish_reason: None,
                                            }],
                                        }));
                                    }
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

#[derive(Debug, Serialize)]
struct CohereRequest {
    model: String,
    messages: Vec<CohereMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preamble: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    p: Option<f32>,
    stream: bool,
}

#[derive(Debug, Serialize)]
struct CohereMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct CohereResponse {
    text: String,
    #[serde(default)]
    meta: Option<CohereMeta>,
}

impl From<CohereResponse> for ChatResponse {
    fn from(r: CohereResponse) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            object: "chat.completion".into(),
            created: chrono::Utc::now().timestamp(),
            model: "command".into(),
            choices: vec![crate::types::Choice {
                index: 0,
                message: crate::types::Message::assistant(&r.text),
                finish_reason: Some("complete".into()),
                logprobs: None,
            }],
            usage: r.meta.map(|m| crate::types::Usage {
                prompt_tokens: m.tokens.input_tokens,
                completion_tokens: m.tokens.output_tokens,
                total_tokens: m.tokens.input_tokens + m.tokens.output_tokens,
            }).unwrap_or_default(),
            system_fingerprint: None,
        }
    }
}

#[derive(Debug, Deserialize)]
struct CohereMeta {
    tokens: CohereTokens,
}

#[derive(Debug, Deserialize)]
struct CohereTokens {
    input_tokens: u32,
    output_tokens: u32,
}

#[derive(Debug, Deserialize)]
struct CohereStreamEvent {
    #[serde(rename = "type")]
    event_type: String,
    text: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cohere_provider() {
        let provider = CohereProvider::new("test-key");
        assert!(provider.is_ok());
    }
}
