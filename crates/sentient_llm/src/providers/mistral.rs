//! ─── Mistral Provider ───
//!
//! Implementation for Mistral AI API

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

/// Mistral AI provider
pub struct MistralProvider {
    client: Client,
    api_key: String,
    base_url: String,
}

impl MistralProvider {
    pub fn new(api_key: impl Into<String>) -> LlmResult<Self> {
        Ok(Self {
            client: build_client()?,
            api_key: api_key.into(),
            base_url: "https://api.mistral.ai/v1".into(),
        })
    }

    pub fn from_env() -> LlmResult<Self> {
        let api_key = std::env::var("MISTRAL_API_KEY")
            .map_err(|_| LlmError::Authentication("MISTRAL_API_KEY not set".into()))?;
        Self::new(api_key)
    }

    fn convert_request(&self, request: ChatRequest) -> MistralRequest {
        MistralRequest {
            model: request.model,
            messages: request.messages.into_iter().map(|m| MistralMessage {
                role: match m.role {
                    Role::System => "system".into(),
                    Role::User => "user".into(),
                    Role::Assistant => "assistant".into(),
                    Role::Tool => "tool".into(),
                    Role::Function => "function".into(),
                },
                content: m.content.as_text().map(|s| s.to_string()).unwrap_or_default(),
            }).collect(),
            max_tokens: request.max_tokens,
            temperature: request.temperature,
            top_p: request.top_p,
            stream: request.stream,
            random_seed: request.seed,
        }
    }
}

#[async_trait]
impl LlmProvider for MistralProvider {
    fn name(&self) -> &str { "Mistral" }
    fn id(&self) -> &str { "mistral" }
    fn models(&self) -> Vec<ModelInfo> { models::mistral_models() }
    fn is_configured(&self) -> bool { !self.api_key.is_empty() }

    async fn chat(&self, request: ChatRequest) -> LlmResult<ChatResponse> {
        let mistral_request = self.convert_request(request);

        let response = self.client
            .post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&mistral_request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(LlmError::ApiError(parse_api_error(response).await));
        }

        let mistral_response: MistralResponse = response.json().await?;
        Ok(mistral_response.into())
    }

    async fn chat_stream(
        &self,
        request: ChatRequest,
    ) -> LlmResult<Pin<Box<dyn Stream<Item = LlmResult<StreamChunk>> + Send>>> {
        let mut mistral_request = self.convert_request(request);
        mistral_request.stream = true;

        let response = self.client
            .post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&mistral_request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(LlmError::ApiError(parse_api_error(response).await));
        }

        let stream = response.bytes_stream()
            .map(|result| {
                match result {
                    Ok(bytes) => {
                        let text = String::from_utf8_lossy(&bytes);
                        for line in text.lines() {
                            if line.starts_with("data: ") {
                                let data = &line[6..];
                                if data == "[DONE]" { return Ok(None); }
                                if let Ok(chunk) = serde_json::from_str::<MistralStreamChunk>(data) {
                                    return Ok(Some(chunk.into()));
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
struct MistralRequest {
    model: String,
    messages: Vec<MistralMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
    stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    random_seed: Option<i64>,
}

#[derive(Debug, Serialize)]
struct MistralMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct MistralResponse {
    id: String,
    object: String,
    created: i64,
    model: String,
    choices: Vec<MistralChoice>,
    usage: MistralUsage,
}

impl From<MistralResponse> for ChatResponse {
    fn from(r: MistralResponse) -> Self {
        Self {
            id: r.id,
            object: r.object,
            created: r.created,
            model: r.model,
            choices: r.choices.into_iter().map(|c| c.into()).collect(),
            usage: r.usage.into(),
            system_fingerprint: None,
        }
    }
}

#[derive(Debug, Deserialize)]
struct MistralChoice {
    index: u32,
    message: MistralMessageResponse,
    finish_reason: Option<String>,
}

impl From<MistralChoice> for crate::types::Choice {
    fn from(c: MistralChoice) -> Self {
        Self {
            index: c.index,
            message: crate::types::Message {
                role: Role::Assistant,
                content: Content::text(c.message.content),
                name: None,
                tool_calls: None,
                tool_call_id: None,
            },
            finish_reason: c.finish_reason,
            logprobs: None,
        }
    }
}

#[derive(Debug, Deserialize)]
struct MistralMessageResponse {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct MistralUsage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

impl From<MistralUsage> for crate::types::Usage {
    fn from(u: MistralUsage) -> Self {
        Self {
            prompt_tokens: u.prompt_tokens,
            completion_tokens: u.completion_tokens,
            total_tokens: u.total_tokens,
        }
    }
}

#[derive(Debug, Deserialize)]
struct MistralStreamChunk {
    id: String,
    object: String,
    created: i64,
    model: String,
    choices: Vec<MistralStreamChoice>,
}

impl From<MistralStreamChunk> for StreamChunk {
    fn from(c: MistralStreamChunk) -> Self {
        Self {
            id: c.id,
            object: c.object,
            created: c.created,
            model: c.model,
            system_fingerprint: None,
            choices: c.choices.into_iter().map(|ch| crate::types::StreamChoice {
                index: ch.index,
                delta: crate::types::Delta {
                    role: Some(Role::Assistant),
                    content: ch.delta.content,
                    tool_calls: None,
                },
                finish_reason: ch.finish_reason,
            }).collect(),
        }
    }
}

#[derive(Debug, Deserialize)]
struct MistralStreamChoice {
    index: u32,
    delta: MistralDelta,
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct MistralDelta {
    content: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mistral_provider() {
        let provider = MistralProvider::new("test-key");
        assert!(provider.is_ok());
    }
}
