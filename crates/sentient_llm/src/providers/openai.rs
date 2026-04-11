//! ─── OpenAI Provider ───
//!
//! Implementation for OpenAI API (GPT-4, GPT-4o, GPT-3.5, o1)

use async_trait::async_trait;
use futures::{Stream, StreamExt};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::pin::Pin;

use crate::error::{LlmError, LlmResult};
use crate::types::{ChatRequest, ChatResponse, ModelInfo, StreamChunk};
use crate::provider::LlmProvider;
use crate::models;

use super::{build_client, parse_api_error};

// ═══════════════════════════════════════════════════════════════════════════════
//  OPENAI PROVIDER
// ═══════════════════════════════════════════════════════════════════════════════

/// OpenAI API provider
pub struct OpenAIProvider {
    client: Client,
    api_key: String,
    base_url: String,
}

impl OpenAIProvider {
    /// Create new OpenAI provider
    pub fn new(api_key: impl Into<String>) -> LlmResult<Self> {
        Ok(Self {
            client: build_client()?,
            api_key: api_key.into(),
            base_url: "https://api.openai.com/v1".into(),
        })
    }

    /// Create with custom base URL (for Azure, proxies)
    pub fn with_base_url(api_key: impl Into<String>, base_url: impl Into<String>) -> LlmResult<Self> {
        Ok(Self {
            client: build_client()?,
            api_key: api_key.into(),
            base_url: base_url.into(),
        })
    }

    /// From environment variable
    pub fn from_env() -> LlmResult<Self> {
        let api_key = std::env::var("OPENAI_API_KEY")
            .map_err(|_| LlmError::Authentication("OPENAI_API_KEY not set".into()))?;
        Self::new(api_key)
    }

    /// Convert request to OpenAI format
    fn convert_request(&self, request: ChatRequest) -> OpenAIRequest {
        OpenAIRequest {
            model: request.model,
            messages: request.messages.into_iter().map(|m| {
                OpenAIMessage {
                    role: match m.role {
                        crate::types::Role::System => "system".into(),
                        crate::types::Role::User => "user".into(),
                        crate::types::Role::Assistant => "assistant".into(),
                        crate::types::Role::Tool => "tool".into(),
                        crate::types::Role::Function => "function".into(),
                    },
                    content: Some(m.content),
                    name: m.name,
                    tool_calls: m.tool_calls.map(|calls| {
                        calls.into_iter().map(|tc| OpenAIToolCall {
                            id: tc.id,
                            type_: "function".into(),
                            function: OpenAIFunctionCall {
                                name: tc.function.name,
                                arguments: tc.function.arguments,
                            },
                        }).collect()
                    }),
                    tool_call_id: m.tool_call_id,
                }
            }).collect(),
            max_tokens: request.max_tokens,
            temperature: request.temperature,
            top_p: request.top_p,
            n: request.n,
            stop: request.stop,
            presence_penalty: request.presence_penalty,
            frequency_penalty: request.frequency_penalty,
            user: request.user,
            tools: request.tools.map(|tools| {
                tools.into_iter().map(|t| OpenAITool {
                    type_: t.tool_type,
                    function: t.function,
                }).collect()
            }),
            tool_choice: request.tool_choice.map(|tc| {
                match tc {
                    crate::types::ToolChoice::String(s) => s,
                    crate::types::ToolChoice::Object { type_, function } => {
                        serde_json::to_string(&serde_json::json!({
                            "type": type_,
                            "function": {"name": function.name}
                        })).unwrap()
                    }
                }
            }),
            response_format: request.response_format,
            seed: request.seed,
            stream: request.stream,
            logprobs: request.logprobs,
            top_logprobs: request.top_logprobs,
        }
    }
}

#[async_trait]
impl LlmProvider for OpenAIProvider {
    fn name(&self) -> &str {
        "OpenAI"
    }

    fn id(&self) -> &str {
        "openai"
    }

    fn models(&self) -> Vec<ModelInfo> {
        models::openai_models()
    }

    fn is_configured(&self) -> bool {
        !self.api_key.is_empty()
    }

    async fn chat(&self, request: ChatRequest) -> LlmResult<ChatResponse> {
        let openai_request = self.convert_request(request);

        let response = self.client
            .post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&openai_request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(LlmError::ApiError(parse_api_error(response).await));
        }

        let openai_response: OpenAIResponse = response.json().await?;
        Ok(openai_response.into())
    }

    async fn chat_stream(
        &self,
        request: ChatRequest,
    ) -> LlmResult<Pin<Box<dyn Stream<Item = LlmResult<StreamChunk>> + Send>>> {
        let mut openai_request = self.convert_request(request);
        openai_request.stream = true;

        let response = self.client
            .post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&openai_request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(LlmError::ApiError(parse_api_error(response).await));
        }

        // Create stream
        let stream = response.bytes_stream()
            .map(|result| {
                match result {
                    Ok(bytes) => {
                        let text = String::from_utf8_lossy(&bytes);
                        // Parse SSE format
                        for line in text.lines() {
                            if line.starts_with("data: ") {
                                let data = &line[6..];
                                if data == "[DONE]" {
                                    return Ok(None);
                                }
                                if let Ok(chunk) = serde_json::from_str::<OpenAIStreamChunk>(data) {
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
        // Use tiktoken for accurate token counting
        use tiktoken_rs::cl100k_base;
        let bpe = cl100k_base().map_err(|e| LlmError::TokenCounting(e.to_string()))?;
        Ok(bpe.encode_with_special_tokens(text).len())
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  OPENAI API TYPES
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Serialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<OpenAIMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    n: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stop: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    presence_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    frequency_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<OpenAITool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_choice: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_format: Option<crate::types::ResponseFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    seed: Option<i64>,
    stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    logprobs: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_logprobs: Option<u32>,
}

#[derive(Debug, Serialize)]
struct OpenAIMessage {
    role: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<crate::types::Content>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_calls: Option<Vec<OpenAIToolCall>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_call_id: Option<String>,
}

#[derive(Debug, Serialize)]
struct OpenAITool {
    #[serde(rename = "type")]
    type_: String,
    function: crate::types::FunctionDefinition,
}

#[derive(Debug, Serialize)]
struct OpenAIToolCall {
    id: String,
    #[serde(rename = "type")]
    type_: String,
    function: OpenAIFunctionCall,
}

#[derive(Debug, Serialize)]
struct OpenAIFunctionCall {
    name: String,
    arguments: String,
}

#[derive(Debug, Deserialize)]
struct OpenAIResponse {
    id: String,
    object: String,
    created: i64,
    model: String,
    choices: Vec<OpenAIChoice>,
    usage: OpenAIUsage,
    system_fingerprint: Option<String>,
}

impl From<OpenAIResponse> for ChatResponse {
    fn from(r: OpenAIResponse) -> Self {
        Self {
            id: r.id,
            object: r.object,
            created: r.created,
            model: r.model,
            choices: r.choices.into_iter().map(|c| c.into()).collect(),
            usage: r.usage.into(),
            system_fingerprint: r.system_fingerprint,
        }
    }
}

#[derive(Debug, Deserialize)]
struct OpenAIChoice {
    index: u32,
    message: OpenAIMessageResponse,
    finish_reason: Option<String>,
    logprobs: Option<serde_json::Value>,
}

impl From<OpenAIChoice> for crate::types::Choice {
    fn from(c: OpenAIChoice) -> Self {
        Self {
            index: c.index,
            message: crate::types::Message {
                role: match c.message.role.as_str() {
                    "system" => crate::types::Role::System,
                    "user" => crate::types::Role::User,
                    "assistant" => crate::types::Role::Assistant,
                    "tool" => crate::types::Role::Tool,
                    _ => crate::types::Role::Assistant,
                },
                content: c.message.content.unwrap_or(crate::types::Content::text("")),
                name: c.message.name,
                tool_calls: c.message.tool_calls.map(|calls| {
                    calls.into_iter().map(|tc| crate::types::ToolCall {
                        id: tc.id,
                        tool_type: tc.type_,
                        function: crate::types::FunctionCall {
                            name: tc.function.name,
                            arguments: tc.function.arguments,
                        },
                    }).collect()
                }),
                tool_call_id: c.message.tool_call_id,
            },
            finish_reason: c.finish_reason,
            logprobs: None,
        }
    }
}

#[derive(Debug, Deserialize)]
struct OpenAIMessageResponse {
    role: String,
    content: Option<crate::types::Content>,
    name: Option<String>,
    tool_calls: Option<Vec<OpenAIToolCallResponse>>,
    tool_call_id: Option<String>,
}

#[derive(Debug, Deserialize)]
struct OpenAIToolCallResponse {
    id: String,
    #[serde(rename = "type")]
    type_: String,
    function: OpenAIFunctionCallResponse,
}

#[derive(Debug, Deserialize)]
struct OpenAIFunctionCallResponse {
    name: String,
    arguments: String,
}

#[derive(Debug, Deserialize)]
struct OpenAIUsage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

impl From<OpenAIUsage> for crate::types::Usage {
    fn from(u: OpenAIUsage) -> Self {
        Self {
            prompt_tokens: u.prompt_tokens,
            completion_tokens: u.completion_tokens,
            total_tokens: u.total_tokens,
        }
    }
}

#[derive(Debug, Deserialize)]
struct OpenAIStreamChunk {
    id: String,
    object: String,
    created: i64,
    model: String,
    system_fingerprint: Option<String>,
    choices: Vec<OpenAIStreamChoice>,
}

impl From<OpenAIStreamChunk> for StreamChunk {
    fn from(c: OpenAIStreamChunk) -> Self {
        Self {
            id: c.id,
            object: c.object,
            created: c.created,
            model: c.model,
            system_fingerprint: c.system_fingerprint,
            choices: c.choices.into_iter().map(|ch| ch.into()).collect(),
        }
    }
}

#[derive(Debug, Deserialize)]
struct OpenAIStreamChoice {
    index: u32,
    delta: OpenAIDelta,
    finish_reason: Option<String>,
}

impl From<OpenAIStreamChoice> for crate::types::StreamChoice {
    fn from(c: OpenAIStreamChoice) -> Self {
        Self {
            index: c.index,
            delta: crate::types::Delta {
                role: c.delta.role.map(|r| match r.as_str() {
                    "system" => crate::types::Role::System,
                    "user" => crate::types::Role::User,
                    "assistant" => crate::types::Role::Assistant,
                    _ => crate::types::Role::Assistant,
                }),
                content: c.delta.content,
                tool_calls: None,
            },
            finish_reason: c.finish_reason,
        }
    }
}

#[derive(Debug, Deserialize)]
struct OpenAIDelta {
    role: Option<String>,
    content: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_openai_provider_creation() {
        let provider = OpenAIProvider::new("test-key");
        assert!(provider.is_ok());
    }

    #[test]
    fn test_openai_models() {
        let provider = OpenAIProvider::new("test-key").unwrap();
        let models = provider.models();
        assert!(!models.is_empty());
        assert!(models.iter().any(|m| m.id == "gpt-4o"));
    }

    #[test]
    fn test_token_counting() {
        let provider = OpenAIProvider::new("test-key").unwrap();
        let count = provider.count_tokens("Hello, world!", "gpt-4o");
        assert!(count.is_ok());
        assert!(count.unwrap() > 0);
    }
}
