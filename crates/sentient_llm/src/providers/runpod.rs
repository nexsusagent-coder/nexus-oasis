//! ─── RunPod Serverless Provider ───
//!
//! Implementation for RunPod Serverless API (GPU Inference)
//! API Docs: https://docs.runpod.io/serverless

use async_trait::async_trait;
use futures::{Stream, StreamExt};
use reqwest::Client;
use std::pin::Pin;

use crate::error::{LlmError, LlmResult};
use crate::types::{ChatRequest, ChatResponse, ModelInfo, StreamChunk};
use crate::provider::LlmProvider;

use super::{build_client, parse_api_error};

/// RunPod Serverless provider
pub struct RunPodProvider {
    client: Client,
    api_key: String,
    base_url: String,
}

impl RunPodProvider {
    pub fn new(api_key: impl Into<String>) -> LlmResult<Self> {
        Ok(Self {
            client: build_client()?,
            api_key: api_key.into(),
            base_url: "https://api.runpod.ai/v2".into(),
        })
    }

    pub fn from_env() -> LlmResult<Self> {
        let api_key = std::env::var("RUNPOD_API_KEY")
            .map_err(|_| LlmError::Authentication("RUNPOD_API_KEY not set".into()))?;
        Self::new(api_key)
    }

    /// Get endpoint ID for a model (RunPod uses endpoints)
    fn get_endpoint_id(&self, model: &str) -> &str {
        match model {
            "llama-3-70b" => "llama3-70b",
            "llama-3-8b" => "llama3-8b",
            "mixtral-8x7b" => "mixtral-8x7b",
            "qwen-2.5-72b" => "qwen2-5-72b",
            _ => "llama3-70b", // Default
        }
    }
}

#[async_trait]
impl LlmProvider for RunPodProvider {
    fn name(&self) -> &str { "RunPod Serverless" }
    fn id(&self) -> &str { "runpod" }

    fn models(&self) -> Vec<ModelInfo> {
        vec![
            // ═══════════════════════════════════════════════════════════
            // LLAMA 3 MODELS
            // ═══════════════════════════════════════════════════════════
            ModelInfo {
                id: "llama-3-70b".into(),
                name: "Llama 3 70B (RunPod)".into(),
                provider: "RunPod Serverless".into(),
                context_window: 8_192,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.0008,
                output_cost_per_1k: 0.001,
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
                id: "llama-3-8b".into(),
                name: "Llama 3 8B (RunPod)".into(),
                provider: "RunPod Serverless".into(),
                context_window: 8_192,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.0001,
                output_cost_per_1k: 0.0001,
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: None,
                quality_rating: 3,
                speed_rating: 5,
                is_reasoning: false,
                free_tier: true,
            },
            // ═══════════════════════════════════════════════════════════
            // MIXTRAL
            // ═══════════════════════════════════════════════════════════
            ModelInfo {
                id: "mixtral-8x7b".into(),
                name: "Mixtral 8x7B (RunPod)".into(),
                provider: "RunPod Serverless".into(),
                context_window: 32_768,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.0006,
                output_cost_per_1k: 0.0008,
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
            // QWEN 2.5
            // ═══════════════════════════════════════════════════════════
            ModelInfo {
                id: "qwen-2.5-72b".into(),
                name: "Qwen 2.5 72B (RunPod)".into(),
                provider: "RunPod Serverless".into(),
                context_window: 32_768,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.0007,
                output_cost_per_1k: 0.0009,
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
        ]
    }

    fn is_configured(&self) -> bool { !self.api_key.is_empty() }

    async fn chat(&self, request: ChatRequest) -> LlmResult<ChatResponse> {
        let endpoint_id = self.get_endpoint_id(&request.model);
        let url = format!("{}/{}/runsync", self.base_url, endpoint_id);

        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({
                "input": {
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
                }
            }))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(LlmError::ApiError(parse_api_error(response).await));
        }

        let json: serde_json::Value = response.json().await?;
        Ok(parse_runpod_response(json, &request.model))
    }

    async fn chat_stream(
        &self,
        request: ChatRequest,
    ) -> LlmResult<Pin<Box<dyn Stream<Item = LlmResult<StreamChunk>> + Send>>> {
        let endpoint_id = self.get_endpoint_id(&request.model);
        let url = format!("{}/{}/run", self.base_url, endpoint_id);

        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({
                "input": {
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
                }
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
                                    return Ok(Some(parse_runpod_stream_chunk(json, &model)));
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

fn parse_runpod_response(json: serde_json::Value, model: &str) -> ChatResponse {
    // RunPod response format: { "output": { "choices": [...] } }
    let output = &json["output"];

    ChatResponse {
        id: json["id"].as_str().unwrap_or("").to_string(),
        object: "chat.completion".to_string(),
        created: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64,
        model: model.to_string(),
        choices: output["choices"].as_array().map(|arr| {
            arr.iter().map(|c| crate::types::Choice {
                index: c["index"].as_u64().unwrap_or(0) as u32,
                message: crate::types::Message::assistant(
                    c["message"]["content"].as_str().unwrap_or("")
                ),
                finish_reason: c["finish_reason"].as_str().map(|s| s.to_string()),
                logprobs: None,
            }).collect()
        }).unwrap_or_else(|| {
            // Fallback: try text field
            vec![crate::types::Choice {
                index: 0,
                message: crate::types::Message::assistant(
                    output["text"].as_str().unwrap_or("")
                ),
                finish_reason: Some("stop".to_string()),
                logprobs: None,
            }]
        }),
        usage: crate::types::Usage {
            prompt_tokens: output["usage"]["prompt_tokens"].as_u64().unwrap_or(0) as u32,
            completion_tokens: output["usage"]["completion_tokens"].as_u64().unwrap_or(0) as u32,
            total_tokens: output["usage"]["total_tokens"].as_u64().unwrap_or(0) as u32,
        },
        system_fingerprint: None,
    }
}

fn parse_runpod_stream_chunk(json: serde_json::Value, model: &str) -> StreamChunk {
    let output = &json["output"];

    StreamChunk {
        id: json["id"].as_str().unwrap_or("").to_string(),
        object: "chat.completion.chunk".to_string(),
        created: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64,
        model: model.to_string(),
        system_fingerprint: None,
        choices: output["choices"].as_array().map(|arr| {
            arr.iter().map(|c| crate::types::StreamChoice {
                index: c["index"].as_u64().unwrap_or(0) as u32,
                delta: crate::types::Delta {
                    role: Some(crate::types::Role::Assistant),
                    content: c["delta"]["content"].as_str().map(|s| s.to_string()),
                    tool_calls: None,
                },
                finish_reason: c["finish_reason"].as_str().map(|s| s.to_string()),
            }).collect()
        }).unwrap_or_else(|| {
            vec![crate::types::StreamChoice {
                index: 0,
                delta: crate::types::Delta {
                    role: Some(crate::types::Role::Assistant),
                    content: output["token"].as_str().map(|s| s.to_string()),
                    tool_calls: None,
                },
                finish_reason: None,
            }]
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runpod_provider() {
        let provider = RunPodProvider::new("test-api-key");
        assert!(provider.is_ok());
    }

    #[test]
    fn test_runpod_models() {
        let provider = RunPodProvider::new("test-key").unwrap();
        let models = provider.models();
        assert_eq!(models.len(), 4, "RunPod should have 4 models");
        assert!(models.iter().any(|m| m.id == "llama-3-70b"));
        assert!(models.iter().any(|m| m.id == "llama-3-8b"));
        assert!(models.iter().any(|m| m.id == "mixtral-8x7b"));
        assert!(models.iter().any(|m| m.id == "qwen-2.5-72b"));
    }

    #[test]
    fn test_runpod_endpoint_mapping() {
        let provider = RunPodProvider::new("test-key").unwrap();
        assert_eq!(provider.get_endpoint_id("llama-3-70b"), "llama3-70b");
        assert_eq!(provider.get_endpoint_id("mixtral-8x7b"), "mixtral-8x7b");
        assert_eq!(provider.get_endpoint_id("unknown"), "llama3-70b"); // Default
    }

    #[test]
    fn test_runpod_context_windows() {
        let provider = RunPodProvider::new("test-key").unwrap();
        let models = provider.models();

        let mixtral = models.iter().find(|m| m.id == "mixtral-8x7b").unwrap();
        assert_eq!(mixtral.context_window, 32_768);

        let qwen = models.iter().find(|m| m.id == "qwen-2.5-72b").unwrap();
        assert_eq!(qwen.context_window, 32_768);
    }

    #[test]
    fn test_runpod_free_tier() {
        let provider = RunPodProvider::new("test-key").unwrap();
        let models = provider.models();

        let llama3_8b = models.iter().find(|m| m.id == "llama-3-8b").unwrap();
        assert!(llama3_8b.free_tier);
    }
}
