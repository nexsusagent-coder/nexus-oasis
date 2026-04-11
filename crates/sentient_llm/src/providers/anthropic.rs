//! ─── Anthropic Provider ───
//!
//! Implementation for Anthropic API (Claude 4, Claude 3.5, Claude 3)

use async_trait::async_trait;
use futures::{Stream, StreamExt};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::pin::Pin;

use crate::error::{LlmError, LlmResult};
use crate::types::{ChatRequest, ChatResponse, ModelInfo, StreamChunk, Role, Message, Content};
use crate::provider::LlmProvider;
use crate::models;

use super::{build_client, parse_api_error};

// ═══════════════════════════════════════════════════════════════════════════════
//  ANTHROPIC PROVIDER
// ═══════════════════════════════════════════════════════════════════════════════

/// Anthropic API provider
pub struct AnthropicProvider {
    client: Client,
    api_key: String,
    base_url: String,
}

impl AnthropicProvider {
    /// Create new Anthropic provider
    pub fn new(api_key: impl Into<String>) -> LlmResult<Self> {
        Ok(Self {
            client: build_client()?,
            api_key: api_key.into(),
            base_url: "https://api.anthropic.com/v1".into(),
        })
    }

    /// From environment variable
    pub fn from_env() -> LlmResult<Self> {
        let api_key = std::env::var("ANTHROPIC_API_KEY")
            .map_err(|_| LlmError::Authentication("ANTHROPIC_API_KEY not set".into()))?;
        Self::new(api_key)
    }

    /// Convert request to Anthropic format
    fn convert_request(&self, request: ChatRequest) -> AnthropicRequest {
        // Anthropic separates system message from messages
        let mut system = None;
        let mut messages: Vec<AnthropicMessage> = Vec::new();

        for msg in request.messages {
            match msg.role {
                Role::System => {
                    system = msg.content.as_text().map(|s| s.to_string());
                }
                Role::User => {
                    messages.push(AnthropicMessage {
                        role: "user".into(),
                        content: convert_content(msg.content),
                    });
                }
                Role::Assistant => {
                    messages.push(AnthropicMessage {
                        role: "assistant".into(),
                        content: convert_content(msg.content),
                    });
                }
                Role::Tool | Role::Function => {
                    // Tool messages as user
                    messages.push(AnthropicMessage {
                        role: "user".into(),
                        content: vec![AnthropicContent::Text {
                            text: msg.content.as_text().unwrap_or("").to_string(),
                        }],
                    });
                }
            }
        }

        AnthropicRequest {
            model: request.model,
            messages,
            system,
            max_tokens: request.max_tokens.unwrap_or(4096),
            temperature: request.temperature,
            top_p: request.top_p,
            stop_sequences: request.stop,
            stream: request.stream,
            tools: request.tools.map(|tools| {
                tools.into_iter().map(|t| AnthropicTool {
                    name: t.function.name,
                    description: Some(t.function.description),
                    input_schema: t.function.parameters,
                }).collect()
            }),
        }
    }
}

/// Convert content to Anthropic format
fn convert_content(content: Content) -> Vec<AnthropicContent> {
    match content {
        Content::Text(text) => vec![AnthropicContent::Text { text }],
        Content::Parts(parts) => parts.into_iter().map(|p| {
            match p {
                crate::types::ContentPart::Text { text } => AnthropicContent::Text { text },
                crate::types::ContentPart::Image { image_url } => {
                    // Parse base64 or URL
                    if image_url.url.starts_with("data:") {
                        let parts: Vec<&str> = image_url.url.splitn(2, ',').collect();
                        if parts.len() == 2 {
                            let media_type = parts[0].trim_start_matches("data:").trim_end_matches(";base64");
                            AnthropicContent::Image {
                                source: AnthropicImageSource {
                                    type_: "base64".into(),
                                    media_type: media_type.into(),
                                    data: parts[1].into(),
                                },
                            }
                        } else {
                            AnthropicContent::Text { text: image_url.url }
                        }
                    } else {
                        AnthropicContent::Text { text: image_url.url }
                    }
                }
            }
        }).collect(),
    }
}

#[async_trait]
impl LlmProvider for AnthropicProvider {
    fn name(&self) -> &str {
        "Anthropic"
    }

    fn id(&self) -> &str {
        "anthropic"
    }

    fn models(&self) -> Vec<ModelInfo> {
        models::anthropic_models()
    }

    fn is_configured(&self) -> bool {
        !self.api_key.is_empty()
    }

    async fn chat(&self, request: ChatRequest) -> LlmResult<ChatResponse> {
        let anthropic_request = self.convert_request(request);

        let response = self.client
            .post(format!("{}/messages", self.base_url))
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .json(&anthropic_request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(LlmError::ApiError(parse_api_error(response).await));
        }

        let anthropic_response: AnthropicResponse = response.json().await?;
        Ok(anthropic_response.into())
    }

    async fn chat_stream(
        &self,
        request: ChatRequest,
    ) -> LlmResult<Pin<Box<dyn Stream<Item = LlmResult<StreamChunk>> + Send>>> {
        let mut anthropic_request = self.convert_request(request);
        anthropic_request.stream = true;

        let response = self.client
            .post(format!("{}/messages", self.base_url))
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .json(&anthropic_request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(LlmError::ApiError(parse_api_error(response).await));
        }

        let model = anthropic_request.model.clone();
        let stream = response.bytes_stream()
            .map(move |result| {
                match result {
                    Ok(bytes) => {
                        let text = String::from_utf8_lossy(&bytes);
                        // Parse SSE format for Anthropic
                        for line in text.lines() {
                            if line.starts_with("data: ") {
                                let data = &line[6..];
                                if let Ok(event) = serde_json::from_str::<AnthropicStreamEvent>(data) {
                                    if event.type_ == "content_block_delta" {
                                        if let Some(delta) = event.delta {
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
                                                        content: Some(delta.text),
                                                        tool_calls: None,
                                                    },
                                                    finish_reason: None,
                                                }],
                                            }));
                                        }
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
        // Rough estimate for Claude models
        Ok(text.len() / 4)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  ANTHROPIC API TYPES
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Serialize)]
struct AnthropicRequest {
    model: String,
    messages: Vec<AnthropicMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    system: Option<String>,
    max_tokens: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stop_sequences: Option<Vec<String>>,
    stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<AnthropicTool>>,
}

#[derive(Debug, Serialize)]
struct AnthropicMessage {
    role: String,
    content: Vec<AnthropicContent>,
}

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
enum AnthropicContent {
    Text { text: String },
    Image { source: AnthropicImageSource },
}

#[derive(Debug, Serialize)]
struct AnthropicImageSource {
    #[serde(rename = "type")]
    type_: String,
    media_type: String,
    data: String,
}

#[derive(Debug, Serialize)]
struct AnthropicTool {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    input_schema: serde_json::Value,
}

#[derive(Debug, Deserialize)]
struct AnthropicResponse {
    id: String,
    model: String,
    content: Vec<AnthropicResponseContent>,
    usage: AnthropicUsage,
}

impl From<AnthropicResponse> for ChatResponse {
    fn from(r: AnthropicResponse) -> Self {
        let text = r.content.iter()
            .filter_map(|c| match c {
                AnthropicResponseContent::Text { text } => Some(text.clone()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join("");

        let tool_calls = r.content.iter()
            .filter_map(|c| match c {
                AnthropicResponseContent::ToolUse { id, name, input } => {
                    Some(crate::types::ToolCall {
                        id: id.clone(),
                        tool_type: "function".into(),
                        function: crate::types::FunctionCall {
                            name: name.clone(),
                            arguments: serde_json::to_string(input).unwrap_or_default(),
                        },
                    })
                }
                _ => None,
            })
            .collect::<Vec<_>>();

        Self {
            id: r.id,
            object: "chat.completion".into(),
            created: chrono::Utc::now().timestamp(),
            model: r.model,
            choices: vec![crate::types::Choice {
                index: 0,
                message: Message {
                    role: Role::Assistant,
                    content: Content::text(text),
                    name: None,
                    tool_calls: if tool_calls.is_empty() { None } else { Some(tool_calls) },
                    tool_call_id: None,
                },
                finish_reason: Some("stop".into()),
                logprobs: None,
            }],
            usage: r.usage.into(),
            system_fingerprint: None,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum AnthropicResponseContent {
    Text { text: String },
    ToolUse { id: String, name: String, input: serde_json::Value },
}

#[derive(Debug, Deserialize)]
struct AnthropicUsage {
    input_tokens: u32,
    output_tokens: u32,
}

impl From<AnthropicUsage> for crate::types::Usage {
    fn from(u: AnthropicUsage) -> Self {
        Self {
            prompt_tokens: u.input_tokens,
            completion_tokens: u.output_tokens,
            total_tokens: u.input_tokens + u.output_tokens,
        }
    }
}

#[derive(Debug, Deserialize)]
struct AnthropicStreamEvent {
    #[serde(rename = "type")]
    type_: String,
    delta: Option<AnthropicStreamDelta>,
}

#[derive(Debug, Deserialize)]
struct AnthropicStreamDelta {
    #[serde(rename = "type")]
    type_: String,
    text: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anthropic_provider_creation() {
        let provider = AnthropicProvider::new("test-key");
        assert!(provider.is_ok());
    }

    #[test]
    fn test_anthropic_models() {
        let provider = AnthropicProvider::new("test-key").unwrap();
        let models = provider.models();
        assert!(!models.is_empty());
        assert!(models.iter().any(|m| m.id.contains("claude")));
    }
}
