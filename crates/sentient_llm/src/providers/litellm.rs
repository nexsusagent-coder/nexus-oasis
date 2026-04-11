//! ─── LiteLLM Provider ───
//!
//! LiteLLM - Unified API for 100+ LLM providers
//! https://litellm.ai

use async_trait::async_trait;
use futures::{Stream, StreamExt};
use reqwest::Client;
use std::pin::Pin;

use crate::error::{LlmError, LlmResult};
use crate::types::{ChatRequest, ChatResponse, ModelInfo, StreamChunk};
use crate::provider::LlmProvider;

use super::build_client;

/// LiteLLM provider - Unified API for 100+ LLM providers
pub struct LiteLLMProvider {
    client: Client,
    api_key: String,
    base_url: String,
}

impl LiteLLMProvider {
    pub fn new(api_key: impl Into<String>) -> LlmResult<Self> {
        Ok(Self {
            client: build_client()?,
            api_key: api_key.into(),
            base_url: "https://api.litellm.ai/v1".into(),
        })
    }

    pub fn from_env() -> LlmResult<Self> {
        let api_key = std::env::var("LITELLM_API_KEY")
            .map_err(|_| LlmError::Authentication("LITELLM_API_KEY not set".into()))?;
        Self::new(api_key)
    }

    /// Create with custom base URL (for self-hosted LiteLLM)
    pub fn with_base_url(api_key: impl Into<String>, base_url: impl Into<String>) -> LlmResult<Self> {
        Ok(Self {
            client: build_client()?,
            api_key: api_key.into(),
            base_url: base_url.into(),
        })
    }
}

#[async_trait]
impl LlmProvider for LiteLLMProvider {
    fn name(&self) -> &str { "LiteLLM" }
    fn id(&self) -> &str { "litellm" }

    fn models(&self) -> Vec<ModelInfo> {
        // LiteLLM supports 100+ providers, showing key ones
        vec![
            // ═══════════════════════════════════════════════════════════
            // OPENAI FAMILY
            // ═══════════════════════════════════════════════════════════
            ModelInfo {
                id: "openai/gpt-4o".into(),
                name: "GPT-4o (via LiteLLM)".into(),
                provider: "LiteLLM".into(),
                context_window: 128_000,
                max_output_tokens: 16_384,
                input_cost_per_1k: 0.0025,
                output_cost_per_1k: 0.01,
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
                id: "openai/gpt-4o-mini".into(),
                name: "GPT-4o Mini (via LiteLLM)".into(),
                provider: "LiteLLM".into(),
                context_window: 128_000,
                max_output_tokens: 16_384,
                input_cost_per_1k: 0.00015,
                output_cost_per_1k: 0.0006,
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
            // ANTHROPIC FAMILY
            // ═══════════════════════════════════════════════════════════
            ModelInfo {
                id: "anthropic/claude-4-opus-20250514".into(),
                name: "Claude 4 Opus (via LiteLLM)".into(),
                provider: "LiteLLM".into(),
                context_window: 200_000,
                max_output_tokens: 16_384,
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
            ModelInfo {
                id: "anthropic/claude-3.5-sonnet".into(),
                name: "Claude 3.5 Sonnet (via LiteLLM)".into(),
                provider: "LiteLLM".into(),
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

            // ═══════════════════════════════════════════════════════════
            // VERTEX AI
            // ═══════════════════════════════════════════════════════════
            ModelInfo {
                id: "vertex_ai/gemini-2.0-flash-exp".into(),
                name: "Gemini 2.0 Flash (via LiteLLM)".into(),
                provider: "LiteLLM".into(),
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
                id: "vertex_ai/gemini-1.5-pro".into(),
                name: "Gemini 1.5 Pro (via LiteLLM)".into(),
                provider: "LiteLLM".into(),
                context_window: 1_000_000,
                max_output_tokens: 8_192,
                input_cost_per_1k: 0.00125,
                output_cost_per_1k: 0.005,
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

            // ═══════════════════════════════════════════════════════════
            // AWS BEDROCK
            // ═══════════════════════════════════════════════════════════
            ModelInfo {
                id: "bedrock/anthropic.claude-3-5-sonnet-20241022-v2:0".into(),
                name: "Claude 3.5 Sonnet (Bedrock)".into(),
                provider: "LiteLLM".into(),
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
                id: "bedrock/meta.llama3-3-70b-instruct-v1:0".into(),
                name: "Llama 3.3 70B (Bedrock)".into(),
                provider: "LiteLLM".into(),
                context_window: 128_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.00072,
                output_cost_per_1k: 0.00072,
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

            // ═══════════════════════════════════════════════════════════
            // AZURE OPENAI
            // ═══════════════════════════════════════════════════════════
            ModelInfo {
                id: "azure/gpt-4o".into(),
                name: "GPT-4o (Azure)".into(),
                provider: "LiteLLM".into(),
                context_window: 128_000,
                max_output_tokens: 16_384,
                input_cost_per_1k: 0.0025,
                output_cost_per_1k: 0.01,
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

            // ═══════════════════════════════════════════════════════════
            // MORE PROVIDERS (via LiteLLM unified format)
            // ═══════════════════════════════════════════════════════════
            ModelInfo {
                id: "huggingface/meta-llama/Meta-Llama-3.1-70B-Instruct".into(),
                name: "Llama 3.1 70B (HF)".into(),
                provider: "LiteLLM".into(),
                context_window: 128_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.0002,
                output_cost_per_1k: 0.0002,
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
                id: "nvidia/nvidia/llama-3.1-nemotron-70b-instruct".into(),
                name: "Nemotron 70B (NVIDIA)".into(),
                provider: "LiteLLM".into(),
                context_window: 128_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.0002,
                output_cost_per_1k: 0.0002,
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
                id: "vllm/meta-llama/Meta-Llama-3.1-70B-Instruct".into(),
                name: "Llama 3.1 70B (vLLM)".into(),
                provider: "LiteLLM".into(),
                context_window: 128_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.0001,
                output_cost_per_1k: 0.0001,
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
                id: "sambanova/Meta-Llama-3.1-70B-Instruct".into(),
                name: "Llama 3.1 70B (SambaNova)".into(),
                provider: "LiteLLM".into(),
                context_window: 128_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.0003,
                output_cost_per_1k: 0.0003,
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
                id: "deepinfra/meta-llama/Meta-Llama-3.1-70B-Instruct".into(),
                name: "Llama 3.1 70B (DeepInfra)".into(),
                provider: "LiteLLM".into(),
                context_window: 128_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.00018,
                output_cost_per_1k: 0.00018,
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
                id: "nebius/meta-llama/Meta-Llama-3.1-70B-Instruct".into(),
                name: "Llama 3.1 70B (Nebius)".into(),
                provider: "LiteLLM".into(),
                context_window: 128_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.0001,
                output_cost_per_1k: 0.0001,
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
        ]
    }

    fn is_configured(&self) -> bool {
        !self.api_key.is_empty()
    }

    async fn chat(&self, request: ChatRequest) -> LlmResult<ChatResponse> {
        let response = self.client
            .post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
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
            .post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
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
    fn test_litellm_provider() {
        let provider = LiteLLMProvider::new("test-key");
        assert!(provider.is_ok());
    }

    #[test]
    fn test_litellm_models() {
        let provider = LiteLLMProvider::new("test-key").unwrap();
        let models = provider.models();
        assert!(models.len() >= 15);
    }
}
