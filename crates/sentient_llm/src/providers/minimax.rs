//! ─── MiniMax Provider ───
//!
//! Implementation for MiniMax API (Chinese LLM)
//! API Docs: https://www.minimaxi.com/document/

use async_trait::async_trait;
use futures::{Stream, StreamExt};
use reqwest::Client;
use std::pin::Pin;

use crate::error::{LlmError, LlmResult};
use crate::types::{ChatRequest, ChatResponse, ModelInfo, StreamChunk};
use crate::provider::LlmProvider;

use super::{build_client, parse_api_error};

/// MiniMax provider
pub struct MiniMaxProvider {
    client: Client,
    api_key: String,
    group_id: String,
    base_url: String,
}

impl MiniMaxProvider {
    pub fn new(api_key: impl Into<String>, group_id: impl Into<String>) -> LlmResult<Self> {
        Ok(Self {
            client: build_client()?,
            api_key: api_key.into(),
            group_id: group_id.into(),
            base_url: "https://api.minimax.chat/v1".into(),
        })
    }

    pub fn from_env() -> LlmResult<Self> {
        let api_key = std::env::var("MINIMAX_API_KEY")
            .map_err(|_| LlmError::Authentication("MINIMAX_API_KEY not set".into()))?;
        let group_id = std::env::var("MINIMAX_GROUP_ID")
            .map_err(|_| LlmError::Authentication("MINIMAX_GROUP_ID not set".into()))?;
        Self::new(api_key, group_id)
    }

    /// Map model ID to MiniMax API model name
    fn get_model_name(&self, model: &str) -> &str {
        match model {
            "abab6.5-chat" => "abab6.5-chat",
            "abab6.5s-chat" => "abab6.5s-chat",
            "abab5.5-chat" => "abab5.5-chat",
            "abab5.5s-chat" => "abab5.5s-chat",
            _ => "abab6.5-chat", // Default to flagship
        }
    }
}

#[async_trait]
impl LlmProvider for MiniMaxProvider {
    fn name(&self) -> &str { "MiniMax" }
    fn id(&self) -> &str { "minimax" }

    fn models(&self) -> Vec<ModelInfo> {
        vec![
            // ═══════════════════════════════════════════════════════════
            // ABAB 6.5 - Latest Flagship Models
            // ═══════════════════════════════════════════════════════════
            ModelInfo {
                id: "abab6.5-chat".into(),
                name: "ABAB 6.5 Chat".into(),
                provider: "MiniMax".into(),
                context_window: 245_000, // 245K context
                max_output_tokens: 16_384,
                input_cost_per_1k: 0.03,
                output_cost_per_1k: 0.03,
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: None,
                quality_rating: 4,
                speed_rating: 3,
                is_reasoning: false,
                free_tier: false,
            },
            ModelInfo {
                id: "abab6.5s-chat".into(),
                name: "ABAB 6.5S Chat".into(),
                provider: "MiniMax".into(),
                context_window: 245_000,
                max_output_tokens: 16_384,
                input_cost_per_1k: 0.015,
                output_cost_per_1k: 0.015,
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: None,
                quality_rating: 4,
                speed_rating: 4,
                is_reasoning: false,
                free_tier: false,
            },
            // ═══════════════════════════════════════════════════════════
            // ABAB 5.5 - Previous Generation
            // ═══════════════════════════════════════════════════════════
            ModelInfo {
                id: "abab5.5-chat".into(),
                name: "ABAB 5.5 Chat".into(),
                provider: "MiniMax".into(),
                context_window: 16_384,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.005,
                output_cost_per_1k: 0.005,
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: None,
                quality_rating: 3,
                speed_rating: 4,
                is_reasoning: false,
                free_tier: false,
            },
            ModelInfo {
                id: "abab5.5s-chat".into(),
                name: "ABAB 5.5S Chat".into(),
                provider: "MiniMax".into(),
                context_window: 16_384,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.003,
                output_cost_per_1k: 0.003,
                supports_vision: false,
                supports_tools: false,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: None,
                quality_rating: 3,
                speed_rating: 5,
                is_reasoning: false,
                free_tier: true,
            },
        ]
    }

    fn is_configured(&self) -> bool { !self.api_key.is_empty() && !self.group_id.is_empty() }

    async fn chat(&self, request: ChatRequest) -> LlmResult<ChatResponse> {
        let model_name = self.get_model_name(&request.model);
        let url = format!("{}/text/chat", self.base_url);

        // Build messages for MiniMax format
        let messages: Vec<serde_json::Value> = request.messages.iter().map(|m| {
            serde_json::json!({
                "sender_type": match m.role {
                    crate::types::Role::System => "SYSTEM",
                    crate::types::Role::User => "USER",
                    crate::types::Role::Assistant => "BOT",
                    _ => "USER",
                },
                "sender_id": "user",
                "text": m.content.as_text().unwrap_or(""),
            })
        }).collect();

        let mut body = serde_json::json!({
            "model": model_name,
            "messages": messages,
            "group_id": self.group_id,
        });

        if let Some(temp) = request.temperature {
            body["temperature"] = serde_json::json!(temp);
        }
        if let Some(max_tokens) = request.max_tokens {
            body["tokens_to_generate"] = serde_json::json!(max_tokens);
        }

        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(LlmError::ApiError(parse_api_error(response).await));
        }

        let json: serde_json::Value = response.json().await?;
        Ok(parse_minimax_response(json, &request.model))
    }

    async fn chat_stream(
        &self,
        request: ChatRequest,
    ) -> LlmResult<Pin<Box<dyn Stream<Item = LlmResult<StreamChunk>> + Send>>> {
        let model_name = self.get_model_name(&request.model);
        let url = format!("{}/text/chat?stream=true", self.base_url);

        let messages: Vec<serde_json::Value> = request.messages.iter().map(|m| {
            serde_json::json!({
                "sender_type": match m.role {
                    crate::types::Role::System => "SYSTEM",
                    crate::types::Role::User => "USER",
                    crate::types::Role::Assistant => "BOT",
                    _ => "USER",
                },
                "sender_id": "user",
                "text": m.content.as_text().unwrap_or(""),
            })
        }).collect();

        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({
                "model": model_name,
                "messages": messages,
                "group_id": self.group_id,
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
                                    return Ok(Some(parse_minimax_stream_chunk(json, &model)));
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
        // MiniMax uses their own tokenizer, approximate
        Ok(text.chars().count())
    }
}

fn parse_minimax_response(json: serde_json::Value, model: &str) -> ChatResponse {
    // MiniMax response format
    let base_resp = &json["base_resp"];
    let choices = json["choices"].as_array();

    ChatResponse {
        id: json["id"].as_str().unwrap_or("").to_string(),
        object: "chat.completion".to_string(),
        created: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64,
        model: model.to_string(),
        choices: choices.map(|arr| {
            arr.iter().enumerate().map(|(i, c)| crate::types::Choice {
                index: i as u32,
                message: crate::types::Message::assistant(
                    c["messages"].as_array()
                        .and_then(|msgs| msgs.first())
                        .and_then(|m| m["text"].as_str())
                        .unwrap_or("")
                ),
                finish_reason: c["finish_reason"].as_str().map(|s| s.to_string()),
                logprobs: None,
            }).collect()
        }).unwrap_or_else(|| {
            // Fallback: try reply field
            vec![crate::types::Choice {
                index: 0,
                message: crate::types::Message::assistant(
                    json["reply"].as_str().unwrap_or("")
                ),
                finish_reason: Some("stop".to_string()),
                logprobs: None,
            }]
        }),
        usage: crate::types::Usage {
            prompt_tokens: json["usage"]["prompt_tokens"].as_u64().unwrap_or(0) as u32,
            completion_tokens: json["usage"]["completion_tokens"].as_u64().unwrap_or(0) as u32,
            total_tokens: json["usage"]["total_tokens"].as_u64().unwrap_or(0) as u32,
        },
        system_fingerprint: None,
    }
}

fn parse_minimax_stream_chunk(json: serde_json::Value, model: &str) -> StreamChunk {
    StreamChunk {
        id: json["id"].as_str().unwrap_or("").to_string(),
        object: "chat.completion.chunk".to_string(),
        created: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64,
        model: model.to_string(),
        system_fingerprint: None,
        choices: vec![crate::types::StreamChoice {
            index: 0,
            delta: crate::types::Delta {
                role: Some(crate::types::Role::Assistant),
                content: json["choices"][0]["messages"][0]["text"]
                    .as_str()
                    .or_else(|| json["content"].as_str())
                    .map(|s| s.to_string()),
                tool_calls: None,
            },
            finish_reason: json["finish_reason"].as_str().map(|s| s.to_string()),
        }],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimax_provider() {
        let provider = MiniMaxProvider::new("test-api-key", "test-group-id");
        assert!(provider.is_ok());
    }

    #[test]
    fn test_minimax_models() {
        let provider = MiniMaxProvider::new("test-key", "test-group").unwrap();
        let models = provider.models();
        assert_eq!(models.len(), 4, "MiniMax should have 4 models");
        assert!(models.iter().any(|m| m.id == "abab6.5-chat"));
        assert!(models.iter().any(|m| m.id == "abab5.5-chat"));
    }

    #[test]
    fn test_minimax_model_names() {
        let provider = MiniMaxProvider::new("test-key", "test-group").unwrap();
        assert_eq!(provider.get_model_name("abab6.5-chat"), "abab6.5-chat");
        assert_eq!(provider.get_model_name("abab6.5s-chat"), "abab6.5s-chat");
        assert_eq!(provider.get_model_name("unknown"), "abab6.5-chat"); // Default
    }

    #[test]
    fn test_minimax_context_windows() {
        let provider = MiniMaxProvider::new("test-key", "test-group").unwrap();
        let models = provider.models();

        let abab65 = models.iter().find(|m| m.id == "abab6.5-chat").unwrap();
        assert_eq!(abab65.context_window, 245_000);

        let abab55 = models.iter().find(|m| m.id == "abab5.5-chat").unwrap();
        assert_eq!(abab55.context_window, 16_384);
    }

    #[test]
    fn test_minimax_free_tier() {
        let provider = MiniMaxProvider::new("test-key", "test-group").unwrap();
        let models = provider.models();

        let abab55s = models.iter().find(|m| m.id == "abab5.5s-chat").unwrap();
        assert!(abab55s.free_tier);
    }
}
