//! ─── AI21 Provider ───
//!
//! Implementation for AI21 API (Jamba)

use async_trait::async_trait;
use futures::{Stream, StreamExt};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::pin::Pin;

use crate::error::{LlmError, LlmResult};
use crate::types::{ChatRequest, ChatResponse, ModelInfo, StreamChunk, Role};
use crate::provider::LlmProvider;
use crate::models;

use super::{build_client, parse_api_error};

/// AI21 provider
pub struct AI21Provider {
    client: Client,
    api_key: String,
    base_url: String,
}

impl AI21Provider {
    pub fn new(api_key: impl Into<String>) -> LlmResult<Self> {
        Ok(Self {
            client: build_client()?,
            api_key: api_key.into(),
            base_url: "https://api.ai21.com/v1".into(),
        })
    }

    pub fn from_env() -> LlmResult<Self> {
        let api_key = std::env::var("AI21_API_KEY")
            .map_err(|_| LlmError::Authentication("AI21_API_KEY not set".into()))?;
        Self::new(api_key)
    }

    fn convert_request(&self, request: ChatRequest) -> AI21Request {
        AI21Request {
            model: request.model,
            messages: request.messages.into_iter().map(|m| AI21Message {
                role: match m.role {
                    Role::System => "system".into(),
                    Role::User => "user".into(),
                    Role::Assistant => "assistant".into(),
                    _ => "user".into(),
                },
                content: m.content.as_text().map(|s| s.to_string()).unwrap_or_default(),
            }).collect(),
            max_tokens: request.max_tokens,
            temperature: request.temperature,
            top_p: request.top_p,
            stream: request.stream,
        }
    }
}

#[async_trait]
impl LlmProvider for AI21Provider {
    fn name(&self) -> &str { "AI21" }
    fn id(&self) -> &str { "ai21" }
    fn models(&self) -> Vec<ModelInfo> { models::ai21_models() }
    fn is_configured(&self) -> bool { !self.api_key.is_empty() }

    async fn chat(&self, request: ChatRequest) -> LlmResult<ChatResponse> {
        let ai21_request = self.convert_request(request);

        let response = self.client
            .post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&ai21_request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(LlmError::ApiError(parse_api_error(response).await));
        }

        let ai21_response: AI21Response = response.json().await?;
        Ok(ai21_response.into())
    }

    async fn chat_stream(
        &self,
        request: ChatRequest,
    ) -> LlmResult<Pin<Box<dyn Stream<Item = LlmResult<StreamChunk>> + Send>>> {
        let mut ai21_request = self.convert_request(request);
        ai21_request.stream = true;

        let response = self.client
            .post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&ai21_request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(LlmError::ApiError(parse_api_error(response).await));
        }

        let model = ai21_request.model.clone();
        let stream = response.bytes_stream()
            .map(move |result| {
                match result {
                    Ok(bytes) => {
                        let text = String::from_utf8_lossy(&bytes);
                        for line in text.lines() {
                            if line.starts_with("data: ") {
                                let data = &line[6..];
                                if data == "[DONE]" { return Ok(None); }
                                if let Ok(chunk) = serde_json::from_str::<AI21StreamChunk>(data) {
                                    return Ok(Some(StreamChunk {
                                        id: chunk.id,
                                        object: chunk.object,
                                        created: chunk.created,
                                        model: model.clone(),
                                        system_fingerprint: None,
                                        choices: chunk.choices.into_iter().map(|c| crate::types::StreamChoice {
                                            index: c.index,
                                            delta: crate::types::Delta {
                                                role: Some(Role::Assistant),
                                                content: c.delta.content,
                                                tool_calls: None,
                                            },
                                            finish_reason: c.finish_reason,
                                        }).collect(),
                                    }));
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
struct AI21Request {
    model: String,
    messages: Vec<AI21Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
    stream: bool,
}

#[derive(Debug, Serialize)]
struct AI21Message {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct AI21Response {
    id: String,
    object: String,
    created: i64,
    model: String,
    choices: Vec<AI21Choice>,
    usage: AI21Usage,
}

impl From<AI21Response> for ChatResponse {
    fn from(r: AI21Response) -> Self {
        Self {
            id: r.id,
            object: r.object,
            created: r.created,
            model: r.model,
            choices: r.choices.into_iter().map(|c| crate::types::Choice {
                index: c.index,
                message: crate::types::Message::assistant(&c.message.content),
                finish_reason: c.finish_reason,
                logprobs: None,
            }).collect(),
            usage: r.usage.into(),
            system_fingerprint: None,
        }
    }
}

#[derive(Debug, Deserialize)]
struct AI21Choice {
    index: u32,
    message: AI21MessageResponse,
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct AI21MessageResponse {
    content: String,
}

#[derive(Debug, Deserialize)]
struct AI21Usage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

impl From<AI21Usage> for crate::types::Usage {
    fn from(u: AI21Usage) -> Self {
        Self {
            prompt_tokens: u.prompt_tokens,
            completion_tokens: u.completion_tokens,
            total_tokens: u.total_tokens,
        }
    }
}

#[derive(Debug, Deserialize)]
struct AI21StreamChunk {
    id: String,
    object: String,
    created: i64,
    choices: Vec<AI21StreamChoice>,
}

#[derive(Debug, Deserialize)]
struct AI21StreamChoice {
    index: u32,
    delta: AI21Delta,
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct AI21Delta {
    content: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ai21_provider() {
        let provider = AI21Provider::new("test-key");
        assert!(provider.is_ok());
    }
}
