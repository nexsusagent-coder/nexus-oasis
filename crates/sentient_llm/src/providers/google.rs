//! ─── Google Provider ───
//!
//! Implementation for Google AI API (Gemini 2.0, Gemini 1.5, Gemma)

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

/// Google AI provider
pub struct GoogleProvider {
    client: Client,
    api_key: String,
    base_url: String,
}

impl GoogleProvider {
    pub fn new(api_key: impl Into<String>) -> LlmResult<Self> {
        Ok(Self {
            client: build_client()?,
            api_key: api_key.into(),
            base_url: "https://generativelanguage.googleapis.com/v1beta".into(),
        })
    }

    pub fn from_env() -> LlmResult<Self> {
        let api_key = std::env::var("GOOGLE_API_KEY")
            .map_err(|_| LlmError::Authentication("GOOGLE_API_KEY not set".into()))?;
        Self::new(api_key)
    }
}

#[async_trait]
impl LlmProvider for GoogleProvider {
    fn name(&self) -> &str { "Google" }
    fn id(&self) -> &str { "google" }
    fn models(&self) -> Vec<ModelInfo> { models::google_models() }
    fn is_configured(&self) -> bool { !self.api_key.is_empty() }

    async fn chat(&self, request: ChatRequest) -> LlmResult<ChatResponse> {
        let google_request = convert_to_google(request);

        let response = self.client
            .post(format!(
                "{}/models/{}:generateContent?key={}",
                self.base_url, google_request.model, self.api_key
            ))
            .json(&google_request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(LlmError::ApiError(parse_api_error(response).await));
        }

        let google_response: GoogleResponse = response.json().await?;
        Ok(convert_from_google(google_response))
    }

    async fn chat_stream(
        &self,
        request: ChatRequest,
    ) -> LlmResult<Pin<Box<dyn Stream<Item = LlmResult<StreamChunk>> + Send>>> {
        let google_request = convert_to_google(request);

        let response = self.client
            .post(format!(
                "{}/models/{}:streamGenerateContent?key={}&alt=sse",
                self.base_url, google_request.model, self.api_key
            ))
            .json(&google_request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(LlmError::ApiError(parse_api_error(response).await));
        }

        let model = google_request.model.clone();
        let stream = response.bytes_stream()
            .map(move |result| {
                match result {
                    Ok(bytes) => {
                        let text = String::from_utf8_lossy(&bytes);
                        for line in text.lines() {
                            if line.starts_with("data: ") {
                                let data = &line[6..];
                                if let Ok(chunk) = serde_json::from_str::<GoogleResponse>(data) {
                                    return Ok(Some(convert_to_stream_chunk(chunk, &model)));
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

fn convert_to_google(request: ChatRequest) -> GoogleRequest {
    let contents: Vec<GoogleContent> = request.messages.into_iter()
        .filter_map(|m| {
            let text = m.content.as_text()?;
            Some(GoogleContent {
                role: match m.role {
                    Role::User => "user".into(),
                    Role::Assistant => "model".into(),
                    _ => "user".into(),
                },
                parts: vec![GooglePart { text: text.to_string() }],
            })
        })
        .collect();

    GoogleRequest {
        model: request.model,
        contents,
        generation_config: Some(GoogleGenerationConfig {
            max_output_tokens: request.max_tokens,
            temperature: request.temperature,
            top_p: request.top_p,
            stop_sequences: request.stop,
        }),
    }
}

fn convert_from_google(response: GoogleResponse) -> ChatResponse {
    let text = response.candidates.iter()
        .filter_map(|c| c.content.parts.iter().next())
        .map(|p| p.text.clone())
        .collect::<Vec<_>>()
        .join("");

    ChatResponse {
        id: uuid::Uuid::new_v4().to_string(),
        object: "chat.completion".into(),
        created: chrono::Utc::now().timestamp(),
        model: response.model.unwrap_or_default(),
        choices: vec![crate::types::Choice {
            index: 0,
            message: crate::types::Message::assistant(&text),
            finish_reason: Some("stop".into()),
            logprobs: None,
        }],
        usage: crate::types::Usage {
            prompt_tokens: response.usage_metadata.as_ref().map(|u| u.prompt_token_count).unwrap_or(0),
            completion_tokens: response.usage_metadata.as_ref().map(|u| u.candidates_token_count).unwrap_or(0),
            total_tokens: response.usage_metadata.as_ref().map(|u| u.total_token_count).unwrap_or(0),
        },
        system_fingerprint: None,
    }
}

fn convert_to_stream_chunk(response: GoogleResponse, model: &str) -> StreamChunk {
    let text = response.candidates.iter()
        .filter_map(|c| c.content.parts.iter().next())
        .map(|p| p.text.clone())
        .collect::<Vec<_>>()
        .join("");

    StreamChunk {
        id: uuid::Uuid::new_v4().to_string(),
        object: "chat.completion.chunk".into(),
        created: chrono::Utc::now().timestamp(),
        model: model.to_string(),
        system_fingerprint: None,
        choices: vec![crate::types::StreamChoice {
            index: 0,
            delta: crate::types::Delta {
                role: Some(Role::Assistant),
                content: Some(text),
                tool_calls: None,
            },
            finish_reason: None,
        }],
    }
}

#[derive(Debug, Serialize)]
struct GoogleRequest {
    #[serde(skip)]
    model: String,
    contents: Vec<GoogleContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    generation_config: Option<GoogleGenerationConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
struct GoogleContent {
    role: String,
    parts: Vec<GooglePart>,
}

#[derive(Debug, Serialize, Deserialize)]
struct GooglePart {
    text: String,
}

#[derive(Debug, Serialize)]
struct GoogleGenerationConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_output_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stop_sequences: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct GoogleResponse {
    #[serde(default)]
    model: Option<String>,
    candidates: Vec<GoogleCandidate>,
    usage_metadata: Option<GoogleUsageMetadata>,
}

#[derive(Debug, Deserialize)]
struct GoogleCandidate {
    content: GoogleContent,
    #[serde(default)]
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GoogleUsageMetadata {
    prompt_token_count: u32,
    candidates_token_count: u32,
    total_token_count: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_google_provider() {
        let provider = GoogleProvider::new("test-key");
        assert!(provider.is_ok());
    }
}
