//! ─── Character.AI Provider ───
//!
//! Implementation for Character.AI API (Unofficial)
//! Note: This uses the unofficial API. Use at your own risk.
//! API Docs: https://character.ai (Reverse-engineered)

use async_trait::async_trait;
use futures::{Stream, StreamExt};
use reqwest::Client;
use std::pin::Pin;

use crate::error::{LlmError, LlmResult};
use crate::types::{ChatRequest, ChatResponse, ModelInfo, StreamChunk};
use crate::provider::LlmProvider;

use super::{build_client, parse_api_error};

/// Character.AI provider
pub struct CharacterAIProvider {
    client: Client,
    token: String,
    base_url: String,
}

impl CharacterAIProvider {
    pub fn new(token: impl Into<String>) -> LlmResult<Self> {
        Ok(Self {
            client: build_client()?,
            token: token.into(),
            base_url: "https://plus.character.ai".into(),
        })
    }

    pub fn from_env() -> LlmResult<Self> {
        let token = std::env::var("CHARACTER_AI_TOKEN")
            .map_err(|_| LlmError::Authentication("CHARACTER_AI_TOKEN not set".into()))?;
        Self::new(token)
    }

    /// Get character ID from model name
    fn get_character_id<'a>(&self, model: &'a str) -> &'a str {
        match model {
            "character-default" => "default",
            "character-assistant" => "assistant",
            "character-creative" => "creative",
            _ => model, // Use model as character ID
        }
    }
}

#[async_trait]
impl LlmProvider for CharacterAIProvider {
    fn name(&self) -> &str { "Character.AI" }
    fn id(&self) -> &str { "character-ai" }

    fn models(&self) -> Vec<ModelInfo> {
        vec![
            // ═══════════════════════════════════════════════════════════
            // DEFAULT CHARACTER
            // ═══════════════════════════════════════════════════════════
            ModelInfo {
                id: "character-default".into(),
                name: "Character.AI Default".into(),
                provider: "Character.AI".into(),
                context_window: 4_096,
                max_output_tokens: 2_048,
                input_cost_per_1k: 0.0,
                output_cost_per_1k: 0.0,
                supports_vision: false,
                supports_tools: false,
                supports_streaming: true,
                supports_json: false,
                training_cutoff: None,
                quality_rating: 3,
                speed_rating: 4,
                is_reasoning: false,
                free_tier: true,
            },
            // ═══════════════════════════════════════════════════════════
            // ASSISTANT CHARACTER
            // ═══════════════════════════════════════════════════════════
            ModelInfo {
                id: "character-assistant".into(),
                name: "Character.AI Assistant".into(),
                provider: "Character.AI".into(),
                context_window: 4_096,
                max_output_tokens: 2_048,
                input_cost_per_1k: 0.0,
                output_cost_per_1k: 0.0,
                supports_vision: false,
                supports_tools: false,
                supports_streaming: true,
                supports_json: false,
                training_cutoff: None,
                quality_rating: 3,
                speed_rating: 4,
                is_reasoning: false,
                free_tier: true,
            },
            // ═══════════════════════════════════════════════════════════
            // CREATIVE CHARACTER
            // ═══════════════════════════════════════════════════════════
            ModelInfo {
                id: "character-creative".into(),
                name: "Character.AI Creative".into(),
                provider: "Character.AI".into(),
                context_window: 4_096,
                max_output_tokens: 2_048,
                input_cost_per_1k: 0.0,
                output_cost_per_1k: 0.0,
                supports_vision: false,
                supports_tools: false,
                supports_streaming: true,
                supports_json: false,
                training_cutoff: None,
                quality_rating: 3,
                speed_rating: 4,
                is_reasoning: false,
                free_tier: true,
            },
        ]
    }

    fn is_configured(&self) -> bool { !self.token.is_empty() }

    async fn chat(&self, request: ChatRequest) -> LlmResult<ChatResponse> {
        let character_id = self.get_character_id(&request.model);
        let url = format!("{}/chat/{}", self.base_url, character_id);

        // Build message history
        let history: Vec<serde_json::Value> = request.messages.iter().map(|m| {
            serde_json::json!({
                "author": match m.role {
                    crate::types::Role::User => "user",
                    crate::types::Role::Assistant => "character",
                    _ => "user",
                },
                "text": m.content.as_text().unwrap_or(""),
            })
        }).collect();

        let response = self.client
            .post(&url)
            .header("Authorization", format!("Token {}", self.token))
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({
                "history": history,
            }))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(LlmError::ApiError(parse_api_error(response).await));
        }

        let json: serde_json::Value = response.json().await?;
        Ok(parse_character_response(json, &request.model))
    }

    async fn chat_stream(
        &self,
        request: ChatRequest,
    ) -> LlmResult<Pin<Box<dyn Stream<Item = LlmResult<StreamChunk>> + Send>>> {
        let character_id = self.get_character_id(&request.model);
        let url = format!("{}/chat/streaming/{}", self.base_url, character_id);

        let history: Vec<serde_json::Value> = request.messages.iter().map(|m| {
            serde_json::json!({
                "author": match m.role {
                    crate::types::Role::User => "user",
                    crate::types::Role::Assistant => "character",
                    _ => "user",
                },
                "text": m.content.as_text().unwrap_or(""),
            })
        }).collect();

        let response = self.client
            .post(&url)
            .header("Authorization", format!("Token {}", self.token))
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({
                "history": history,
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
                                    return Ok(Some(parse_character_stream_chunk(json, &model)));
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

fn parse_character_response(json: serde_json::Value, model: &str) -> ChatResponse {
    ChatResponse {
        id: json["external_id"].as_str().unwrap_or("").to_string(),
        object: "chat.completion".to_string(),
        created: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64,
        model: model.to_string(),
        choices: vec![crate::types::Choice {
            index: 0,
            message: crate::types::Message::assistant(
                json["replies"][0]["text"].as_str().unwrap_or("")
            ),
            finish_reason: Some("stop".to_string()),
            logprobs: None,
        }],
        usage: crate::types::Usage {
            prompt_tokens: 0,
            completion_tokens: 0,
            total_tokens: 0,
        },
        system_fingerprint: None,
    }
}

fn parse_character_stream_chunk(json: serde_json::Value, model: &str) -> StreamChunk {
    StreamChunk {
        id: json["external_id"].as_str().unwrap_or("").to_string(),
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
                content: json["text"].as_str().map(|s| s.to_string()),
                tool_calls: None,
            },
            finish_reason: if json["is_final"].as_bool().unwrap_or(false) {
                Some("stop".to_string())
            } else {
                None
            },
        }],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_character_ai_provider() {
        let provider = CharacterAIProvider::new("test-token");
        assert!(provider.is_ok());
    }

    #[test]
    fn test_character_ai_models() {
        let provider = CharacterAIProvider::new("test-token").unwrap();
        let models = provider.models();
        assert_eq!(models.len(), 3, "Character.AI should have 3 models");
        assert!(models.iter().any(|m| m.id == "character-default"));
        assert!(models.iter().any(|m| m.id == "character-assistant"));
        assert!(models.iter().any(|m| m.id == "character-creative"));
    }

    #[test]
    fn test_character_ai_free_tier() {
        let provider = CharacterAIProvider::new("test-token").unwrap();
        let models = provider.models();

        // All Character.AI models are free
        for model in &models {
            assert!(model.free_tier);
        }
    }

    #[test]
    fn test_character_ai_no_cost() {
        let provider = CharacterAIProvider::new("test-token").unwrap();
        let models = provider.models();

        // All Character.AI models are free (zero cost)
        for model in &models {
            assert_eq!(model.input_cost_per_1k, 0.0);
            assert_eq!(model.output_cost_per_1k, 0.0);
        }
    }

    #[test]
    fn test_character_ai_character_id() {
        let provider = CharacterAIProvider::new("test-token").unwrap();
        assert_eq!(provider.get_character_id("character-default"), "default");
        assert_eq!(provider.get_character_id("character-assistant"), "assistant");
        assert_eq!(provider.get_character_id("custom-char"), "custom-char");
    }
}
