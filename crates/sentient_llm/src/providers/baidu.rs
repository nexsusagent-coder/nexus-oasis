//! ─── Baidu ERNIE Provider ───
//!
//! Implementation for Baidu ERNIE API (Chinese LLM)
//! API Docs: https://cloud.baidu.com/doc/WENXINWORKSHOP/index.html

use async_trait::async_trait;
use futures::{Stream, StreamExt};
use reqwest::Client;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::error::{LlmError, LlmResult};
use crate::types::{ChatRequest, ChatResponse, ModelInfo, StreamChunk};
use crate::provider::LlmProvider;

use super::{build_client, parse_api_error};

/// Baidu ERNIE provider
pub struct BaiduErnieProvider {
    client: Client,
    api_key: String,
    secret_key: String,
    access_token: Arc<RwLock<Option<String>>>,
}

impl BaiduErnieProvider {
    pub fn new(api_key: impl Into<String>, secret_key: impl Into<String>) -> LlmResult<Self> {
        Ok(Self {
            client: build_client()?,
            api_key: api_key.into(),
            secret_key: secret_key.into(),
            access_token: Arc::new(RwLock::new(None)),
        })
    }

    pub fn from_env() -> LlmResult<Self> {
        let api_key = std::env::var("BAIDU_API_KEY")
            .map_err(|_| LlmError::Authentication("BAIDU_API_KEY not set".into()))?;
        let secret_key = std::env::var("BAIDU_SECRET_KEY")
            .map_err(|_| LlmError::Authentication("BAIDU_SECRET_KEY not set".into()))?;
        Self::new(api_key, secret_key)
    }

    /// Get or refresh access token
    async fn get_access_token(&self) -> LlmResult<String> {
        // Check if we have a cached token
        {
            let token = self.access_token.read().await;
            if let Some(ref t) = *token {
                return Ok(t.clone());
            }
        }

        // Get new token from Baidu OAuth
        let url = format!(
            "https://aip.baidubce.com/oauth/2.0/token?grant_type=client_credentials&client_id={}&client_secret={}",
            self.api_key, self.secret_key
        );

        let response = self.client
            .post(&url)
            .send()
            .await
            .map_err(|e| LlmError::Network(e.to_string()))?;

        if !response.status().is_success() {
            return Err(LlmError::Authentication("Failed to get Baidu access token".into()));
        }

        let json: serde_json::Value = response.json().await
            .map_err(|e| LlmError::ParseError(e.to_string()))?;

        let token = json["access_token"].as_str()
            .ok_or_else(|| LlmError::Authentication("No access_token in response".into()))?
            .to_string();

        // Cache the token
        {
            let mut cached = self.access_token.write().await;
            *cached = Some(token.clone());
        }

        Ok(token)
    }

    /// Map model ID to Baidu API endpoint
    fn get_endpoint(&self, model: &str) -> String {
        match model {
            "ernie-4.0-8k" => "completions_pro".to_string(),
            "ernie-4.0-turbo-8k" => "ernie-4.0-turbo-8k".to_string(),
            "ernie-3.5-8k" => "completions".to_string(),
            "ernie-speed-8k" => "ernie_speed".to_string(),
            "ernie-speed-128k" => "ernie-speed-128k".to_string(),
            // Default to ERNIE 4.0
            _ => "completions_pro".to_string(),
        }
    }
}

#[async_trait]
impl LlmProvider for BaiduErnieProvider {
    fn name(&self) -> &str { "Baidu ERNIE" }
    fn id(&self) -> &str { "baidu" }

    fn models(&self) -> Vec<ModelInfo> {
        vec![
            // ═══════════════════════════════════════════════════════════
            // ERNIE 4.0 - Flagship Model
            // ═══════════════════════════════════════════════════════════
            ModelInfo {
                id: "ernie-4.0-8k".into(),
                name: "ERNIE 4.0 8K".into(),
                provider: "Baidu ERNIE".into(),
                context_window: 8_192,
                max_output_tokens: 2_048,
                input_cost_per_1k: 0.12,
                output_cost_per_1k: 0.12,
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
                id: "ernie-4.0-turbo-8k".into(),
                name: "ERNIE 4.0 Turbo 8K".into(),
                provider: "Baidu ERNIE".into(),
                context_window: 8_192,
                max_output_tokens: 2_048,
                input_cost_per_1k: 0.03,
                output_cost_per_1k: 0.06,
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
            // ERNIE 3.5 - Balanced Model
            // ═══════════════════════════════════════════════════════════
            ModelInfo {
                id: "ernie-3.5-8k".into(),
                name: "ERNIE 3.5 8K".into(),
                provider: "Baidu ERNIE".into(),
                context_window: 8_192,
                max_output_tokens: 2_048,
                input_cost_per_1k: 0.004,
                output_cost_per_1k: 0.008,
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
            // ═══════════════════════════════════════════════════════════
            // ERNIE Speed - Fast & Cheap
            // ═══════════════════════════════════════════════════════════
            ModelInfo {
                id: "ernie-speed-8k".into(),
                name: "ERNIE Speed 8K".into(),
                provider: "Baidu ERNIE".into(),
                context_window: 8_192,
                max_output_tokens: 2_048,
                input_cost_per_1k: 0.001,
                output_cost_per_1k: 0.002,
                supports_vision: false,
                supports_tools: false,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: None,
                quality_rating: 2,
                speed_rating: 5,
                is_reasoning: false,
                free_tier: true,
            },
            ModelInfo {
                id: "ernie-speed-128k".into(),
                name: "ERNIE Speed 128K".into(),
                provider: "Baidu ERNIE".into(),
                context_window: 131_072,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.001,
                output_cost_per_1k: 0.002,
                supports_vision: false,
                supports_tools: false,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: None,
                quality_rating: 2,
                speed_rating: 5,
                is_reasoning: false,
                free_tier: false,
            },
        ]
    }

    fn is_configured(&self) -> bool { !self.api_key.is_empty() && !self.secret_key.is_empty() }

    async fn chat(&self, request: ChatRequest) -> LlmResult<ChatResponse> {
        let access_token = self.get_access_token().await?;
        let endpoint = self.get_endpoint(&request.model);
        let url = format!(
            "https://aip.baidubce.com/rpc/2.0/ai_custom/v1/wenxinworkshop/chat/{}?access_token={}",
            endpoint, access_token
        );

        // Build messages for Baidu format
        let messages: Vec<serde_json::Value> = request.messages.iter().map(|m| {
            serde_json::json!({
                "role": match m.role {
                    crate::types::Role::System => "system",
                    crate::types::Role::User => "user",
                    crate::types::Role::Assistant => "assistant",
                    _ => "user",
                },
                "content": m.content.as_text().unwrap_or(""),
            })
        }).collect();

        let mut body = serde_json::json!({
            "messages": messages,
        });

        if let Some(temp) = request.temperature {
            body["temperature"] = serde_json::json!(temp);
        }
        if let Some(max_tokens) = request.max_tokens {
            body["max_output_tokens"] = serde_json::json!(max_tokens);
        }

        let response = self.client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(LlmError::ApiError(parse_api_error(response).await));
        }

        let json: serde_json::Value = response.json().await?;
        Ok(parse_baidu_response(json, &request.model))
    }

    async fn chat_stream(
        &self,
        request: ChatRequest,
    ) -> LlmResult<Pin<Box<dyn Stream<Item = LlmResult<StreamChunk>> + Send>>> {
        let access_token = self.get_access_token().await?;
        let endpoint = self.get_endpoint(&request.model);
        let url = format!(
            "https://aip.baidubce.com/rpc/2.0/ai_custom/v1/wenxinworkshop/chat/{}?access_token={}&stream=true",
            endpoint, access_token
        );

        let messages: Vec<serde_json::Value> = request.messages.iter().map(|m| {
            serde_json::json!({
                "role": match m.role {
                    crate::types::Role::System => "system",
                    crate::types::Role::User => "user",
                    crate::types::Role::Assistant => "assistant",
                    _ => "user",
                },
                "content": m.content.as_text().unwrap_or(""),
            })
        }).collect();

        let response = self.client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({
                "messages": messages,
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
                                    return Ok(Some(parse_baidu_stream_chunk(json, &model)));
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
        // Baidu uses their own tokenizer, approximate with Chinese chars
        Ok(text.chars().count())
    }
}

fn parse_baidu_response(json: serde_json::Value, model: &str) -> ChatResponse {
    ChatResponse {
        id: json["id"].as_str().unwrap_or("").to_string(),
        object: "chat.completion".to_string(),
        created: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64,
        model: model.to_string(),
        choices: vec![crate::types::Choice {
            index: 0,
            message: crate::types::Message::assistant(
                json["result"].as_str().unwrap_or("")
            ),
            finish_reason: json["finish_reason"].as_str().map(|s| s.to_string()),
            logprobs: None,
        }],
        usage: crate::types::Usage {
            prompt_tokens: json["usage"]["prompt_tokens"].as_u64().unwrap_or(0) as u32,
            completion_tokens: json["usage"]["completion_tokens"].as_u64().unwrap_or(0) as u32,
            total_tokens: json["usage"]["total_tokens"].as_u64().unwrap_or(0) as u32,
        },
        system_fingerprint: None,
    }
}

fn parse_baidu_stream_chunk(json: serde_json::Value, model: &str) -> StreamChunk {
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
                content: json["result"].as_str().map(|s| s.to_string()),
                tool_calls: None,
            },
            finish_reason: json["is_end"].as_bool().and_then(|is_end| {
                if is_end { Some("stop".to_string()) } else { None }
            }),
        }],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_baidu_provider() {
        let provider = BaiduErnieProvider::new("test-api-key", "test-secret-key");
        assert!(provider.is_ok());
    }

    #[test]
    fn test_baidu_models() {
        let provider = BaiduErnieProvider::new("test-key", "test-secret").unwrap();
        let models = provider.models();
        assert_eq!(models.len(), 5, "Baidu ERNIE should have 5 models");
        assert!(models.iter().any(|m| m.id == "ernie-4.0-8k"));
        assert!(models.iter().any(|m| m.id == "ernie-3.5-8k"));
        assert!(models.iter().any(|m| m.id == "ernie-speed-128k"));
    }

    #[test]
    fn test_baidu_endpoint_mapping() {
        let provider = BaiduErnieProvider::new("test-key", "test-secret").unwrap();
        assert_eq!(provider.get_endpoint("ernie-4.0-8k"), "completions_pro");
        assert_eq!(provider.get_endpoint("ernie-3.5-8k"), "completions");
        assert_eq!(provider.get_endpoint("ernie-speed-8k"), "ernie_speed");
    }

    #[test]
    fn test_baidu_context_windows() {
        let provider = BaiduErnieProvider::new("test-key", "test-secret").unwrap();
        let models = provider.models();

        let speed_128k = models.iter().find(|m| m.id == "ernie-speed-128k").unwrap();
        assert_eq!(speed_128k.context_window, 131_072);
    }
}
