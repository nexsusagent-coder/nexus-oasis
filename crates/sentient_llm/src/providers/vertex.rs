//! ─── Google Vertex AI Provider ───
//!
//! Google Vertex AI - Enterprise AI on Google Cloud
//! https://cloud.google.com/vertex-ai

use async_trait::async_trait;
use futures::{Stream, StreamExt};
use reqwest::Client;
use std::pin::Pin;

use crate::error::{LlmError, LlmResult};
use crate::types::{ChatRequest, ChatResponse, ModelInfo, StreamChunk};
use crate::provider::LlmProvider;

use super::build_client;

/// Google Vertex AI provider - Enterprise AI on Google Cloud
pub struct VertexAIProvider {
    client: Client,
    access_token: String,
    project_id: String,
    location: String,
}

impl VertexAIProvider {
    pub fn new(
        access_token: impl Into<String>,
        project_id: impl Into<String>,
        location: impl Into<String>,
    ) -> LlmResult<Self> {
        Ok(Self {
            client: build_client()?,
            access_token: access_token.into(),
            project_id: project_id.into(),
            location: location.into(),
        })
    }

    pub fn from_env() -> LlmResult<Self> {
        let access_token = std::env::var("GOOGLE_ACCESS_TOKEN")
            .map_err(|_| LlmError::Authentication("GOOGLE_ACCESS_TOKEN not set".into()))?;
        let project_id = std::env::var("GOOGLE_PROJECT_ID")
            .map_err(|_| LlmError::Authentication("GOOGLE_PROJECT_ID not set".into()))?;
        let location = std::env::var("GOOGLE_LOCATION")
            .unwrap_or_else(|_| "us-central1".into());
        Self::new(access_token, project_id, location)
    }
}

#[async_trait]
impl LlmProvider for VertexAIProvider {
    fn name(&self) -> &str { "Google Vertex AI" }
    fn id(&self) -> &str { "vertex" }

    fn models(&self) -> Vec<ModelInfo> {
        vec![
            // ═══════════════════════════════════════════════════════════
            // GEMINI 2.0 FAMILY
            // ═══════════════════════════════════════════════════════════
            ModelInfo {
                id: "gemini-2.0-flash-exp".into(),
                name: "Gemini 2.0 Flash (Vertex)".into(),
                provider: "Vertex AI".into(),
                context_window: 1_000_000,
                max_output_tokens: 8_192,
                input_cost_per_1k: 0.0,
                output_cost_per_1k: 0.0,
                supports_vision: true,
                supports_tools: true,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: None,
                quality_rating: 4,
                speed_rating: 5,
                is_reasoning: false,
                free_tier: true,
            },
            ModelInfo {
                id: "gemini-2.0-flash-thinking-exp".into(),
                name: "Gemini 2.0 Flash Thinking (Vertex)".into(),
                provider: "Vertex AI".into(),
                context_window: 1_000_000,
                max_output_tokens: 8_192,
                input_cost_per_1k: 0.0,
                output_cost_per_1k: 0.0,
                supports_vision: true,
                supports_tools: true,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: None,
                quality_rating: 5,
                speed_rating: 4,
                is_reasoning: true,
                free_tier: true,
            },

            // ═══════════════════════════════════════════════════════════
            // GEMINI 1.5 FAMILY
            // ═══════════════════════════════════════════════════════════
            ModelInfo {
                id: "gemini-1.5-pro".into(),
                name: "Gemini 1.5 Pro (Vertex)".into(),
                provider: "Vertex AI".into(),
                context_window: 2_000_000,
                max_output_tokens: 8_192,
                input_cost_per_1k: 0.00125,
                output_cost_per_1k: 0.005,
                supports_vision: true,
                supports_tools: true,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: None,
                quality_rating: 5,
                speed_rating: 3,
                is_reasoning: false,
                free_tier: false,
            },
            ModelInfo {
                id: "gemini-1.5-flash".into(),
                name: "Gemini 1.5 Flash (Vertex)".into(),
                provider: "Vertex AI".into(),
                context_window: 1_000_000,
                max_output_tokens: 8_192,
                input_cost_per_1k: 0.00001875,
                output_cost_per_1k: 0.000075,
                supports_vision: true,
                supports_tools: true,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: None,
                quality_rating: 4,
                speed_rating: 5,
                is_reasoning: false,
                free_tier: false,
            },

            // ═══════════════════════════════════════════════════════════
            // GEMINI 1.0 FAMILY
            // ═══════════════════════════════════════════════════════════
            ModelInfo {
                id: "gemini-1.0-pro".into(),
                name: "Gemini 1.0 Pro (Vertex)".into(),
                provider: "Vertex AI".into(),
                context_window: 32_000,
                max_output_tokens: 2_048,
                input_cost_per_1k: 0.00025,
                output_cost_per_1k: 0.0005,
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: None,
                quality_rating: 3,
                speed_rating: 5,
                is_reasoning: false,
                free_tier: false,
            },
            ModelInfo {
                id: "gemini-1.0-pro-vision".into(),
                name: "Gemini 1.0 Pro Vision (Vertex)".into(),
                provider: "Vertex AI".into(),
                context_window: 16_000,
                max_output_tokens: 1_024,
                input_cost_per_1k: 0.00025,
                output_cost_per_1k: 0.0005,
                supports_vision: true,
                supports_tools: false,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: None,
                quality_rating: 3,
                speed_rating: 4,
                is_reasoning: false,
                free_tier: false,
            },

            // ═══════════════════════════════════════════════════════════
            // GEMMA (Vertex)
            // ═══════════════════════════════════════════════════════════
            ModelInfo {
                id: "gemma-2-27b-it".into(),
                name: "Gemma 2 27B (Vertex)".into(),
                provider: "Vertex AI".into(),
                context_window: 8_192,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.00008,
                output_cost_per_1k: 0.00008,
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: None,
                quality_rating: 3,
                speed_rating: 5,
                is_reasoning: false,
                free_tier: false,
            },
            ModelInfo {
                id: "gemma-2-9b-it".into(),
                name: "Gemma 2 9B (Vertex)".into(),
                provider: "Vertex AI".into(),
                context_window: 8_192,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.00003,
                output_cost_per_1k: 0.00003,
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: None,
                quality_rating: 3,
                speed_rating: 5,
                is_reasoning: false,
                free_tier: false,
            },

            // ═══════════════════════════════════════════════════════════
            // LLAMA (Vertex)
            // ═══════════════════════════════════════════════════════════
            ModelInfo {
                id: "meta/llama-3.1-405b-instruct-maas".into(),
                name: "Llama 3.1 405B (Vertex)".into(),
                provider: "Vertex AI".into(),
                context_window: 128_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.002,
                output_cost_per_1k: 0.002,
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: None,
                quality_rating: 5,
                speed_rating: 3,
                is_reasoning: false,
                free_tier: false,
            },
            ModelInfo {
                id: "meta/llama-3.1-70b-instruct-maas".into(),
                name: "Llama 3.1 70B (Vertex)".into(),
                provider: "Vertex AI".into(),
                context_window: 128_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.00035,
                output_cost_per_1k: 0.00035,
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
            ModelInfo {
                id: "meta/llama-3.1-8b-instruct-maas".into(),
                name: "Llama 3.1 8B (Vertex)".into(),
                provider: "Vertex AI".into(),
                context_window: 128_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.00005,
                output_cost_per_1k: 0.00005,
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: None,
                quality_rating: 3,
                speed_rating: 5,
                is_reasoning: false,
                free_tier: false,
            },

            // ═══════════════════════════════════════════════════════════
            // MISTRAL (Vertex)
            // ═══════════════════════════════════════════════════════════
            ModelInfo {
                id: "mistral-large-2402".into(),
                name: "Mistral Large (Vertex)".into(),
                provider: "Vertex AI".into(),
                context_window: 32_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.004,
                output_cost_per_1k: 0.012,
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
                id: "mistral-small-2402".into(),
                name: "Mistral Small (Vertex)".into(),
                provider: "Vertex AI".into(),
                context_window: 32_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.0002,
                output_cost_per_1k: 0.0006,
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: None,
                quality_rating: 3,
                speed_rating: 5,
                is_reasoning: false,
                free_tier: false,
            },

            // ═══════════════════════════════════════════════════════════
            // CLAUDE (Vertex)
            // ═══════════════════════════════════════════════════════════
            ModelInfo {
                id: "claude-3-5-sonnet@20241022".into(),
                name: "Claude 3.5 Sonnet (Vertex)".into(),
                provider: "Vertex AI".into(),
                context_window: 200_000,
                max_output_tokens: 8_192,
                input_cost_per_1k: 0.003,
                output_cost_per_1k: 0.015,
                supports_vision: true,
                supports_tools: true,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: None,
                quality_rating: 5,
                speed_rating: 4,
                is_reasoning: false,
                free_tier: false,
            },
            ModelInfo {
                id: "claude-3-opus@20240229".into(),
                name: "Claude 3 Opus (Vertex)".into(),
                provider: "Vertex AI".into(),
                context_window: 200_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.015,
                output_cost_per_1k: 0.075,
                supports_vision: true,
                supports_tools: true,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: None,
                quality_rating: 5,
                speed_rating: 2,
                is_reasoning: false,
                free_tier: false,
            },
        ]
    }

    fn is_configured(&self) -> bool {
        !self.access_token.is_empty() && !self.project_id.is_empty()
    }

    async fn chat(&self, request: ChatRequest) -> LlmResult<ChatResponse> {
        let response = self.client
            .post(format!(
                "https://{}-aiplatform.googleapis.com/v1/projects/{}/locations/{}/publishers/google/models/{}:generateContent",
                self.location, self.project_id, self.location, request.model
            ))
            .header("Authorization", format!("Bearer {}", self.access_token))
            .json(&request)
            .send()
            .await
            .map_err(|e| LlmError::Network(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(LlmError::ServerError(status.as_u16(), body));
        }

        response.json().await
            .map_err(|e| LlmError::ParseError(e.to_string()))
    }

    async fn chat_stream(
        &self,
        request: ChatRequest,
    ) -> LlmResult<Pin<Box<dyn Stream<Item = LlmResult<StreamChunk>> + Send>>> {
        let mut request = request;
        request.stream = true;

        let response = self.client
            .post(format!(
                "https://{}-aiplatform.googleapis.com/v1/projects/{}/locations/{}/publishers/google/models/{}:streamGenerateContent",
                self.location, self.project_id, self.location, request.model
            ))
            .header("Authorization", format!("Bearer {}", self.access_token))
            .json(&request)
            .send()
            .await
            .map_err(|e| LlmError::Network(e.to_string()))?;

        let stream = response.bytes_stream()
            .filter_map(|result| async move {
                match result {
                    Ok(bytes) => {
                        let text = String::from_utf8_lossy(&bytes);
                        for line in text.lines() {
                            if line.starts_with("data: ") {
                                let data = &line[6..];
                                if data == "[DONE]" {
                                    return None;
                                }
                                if let Ok(chunk) = serde_json::from_str::<StreamChunk>(data) {
                                    return Some(Ok(chunk));
                                }
                            }
                        }
                        None
                    }
                    Err(e) => Some(Err(LlmError::StreamError(e.to_string()))),
                }
            });

        Ok(Box::pin(stream))
    }

    fn count_tokens(&self, text: &str, _model: &str) -> LlmResult<usize> {
        Ok(text.len() / 4)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertex_provider() {
        let provider = VertexAIProvider::new("token", "project-id", "us-central1");
        assert!(provider.is_ok());
    }

    #[test]
    fn test_vertex_models() {
        let provider = VertexAIProvider::new("token", "project-id", "us-central1").unwrap();
        let models = provider.models();
        assert!(models.len() >= 15);
    }
}
