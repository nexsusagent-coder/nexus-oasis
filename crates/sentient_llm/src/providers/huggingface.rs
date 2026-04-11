//! ─── Hugging Face Inference Provider ───
//!
//! Hugging Face Inference Endpoints - Serverless & Dedicated
//! https://huggingface.co/docs/inference-endpoints

use async_trait::async_trait;
use futures::{Stream, stream};
use reqwest::Client;
use std::pin::Pin;

use crate::error::{LlmError, LlmResult};
use crate::types::{ChatRequest, ChatResponse, ModelInfo, StreamChunk};
use crate::provider::LlmProvider;

use super::build_client;

/// Hugging Face Inference provider
pub struct HuggingFaceProvider {
    client: Client,
    api_key: String,
    base_url: String,
}

impl HuggingFaceProvider {
    pub fn new(api_key: impl Into<String>) -> LlmResult<Self> {
        Ok(Self {
            client: build_client()?,
            api_key: api_key.into(),
            base_url: "https://api-inference.huggingface.co/models".into(),
        })
    }

    pub fn from_env() -> LlmResult<Self> {
        let api_key = std::env::var("HF_API_KEY")
            .map_err(|_| LlmError::Authentication("HF_API_KEY not set".into()))?;
        Self::new(api_key)
    }

    /// Create with custom endpoint URL (for dedicated endpoints)
    pub fn with_endpoint(api_key: impl Into<String>, endpoint_url: impl Into<String>) -> LlmResult<Self> {
        Ok(Self {
            client: build_client()?,
            api_key: api_key.into(),
            base_url: endpoint_url.into(),
        })
    }
}

#[async_trait]
impl LlmProvider for HuggingFaceProvider {
    fn name(&self) -> &str { "Hugging Face" }
    fn id(&self) -> &str { "huggingface" }

    fn models(&self) -> Vec<ModelInfo> {
        // Hugging Face hosts 200,000+ models - showing top ones
        vec![
            // ═══════════════════════════════════════════════════════════
            // META LLAMA
            // ═══════════════════════════════════════════════════════════
            ModelInfo {
                id: "meta-llama/Meta-Llama-3.1-405B-Instruct".into(),
                name: "Llama 3.1 405B".into(),
                provider: "Hugging Face".into(),
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
                id: "meta-llama/Meta-Llama-3.1-70B-Instruct".into(),
                name: "Llama 3.1 70B".into(),
                provider: "Hugging Face".into(),
                context_window: 128_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.0006,
                output_cost_per_1k: 0.0006,
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: None,
                quality_rating: 4,
                speed_rating: 4,
                is_reasoning: false,
                free_tier: true,
            },
            ModelInfo {
                id: "meta-llama/Meta-Llama-3.1-8B-Instruct".into(),
                name: "Llama 3.1 8B".into(),
                provider: "Hugging Face".into(),
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
                free_tier: true,
            },
            ModelInfo {
                id: "meta-llama/Llama-3.2-3B-Instruct".into(),
                name: "Llama 3.2 3B".into(),
                provider: "Hugging Face".into(),
                context_window: 128_000,
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
                free_tier: true,
            },
            ModelInfo {
                id: "meta-llama/Llama-3.2-1B-Instruct".into(),
                name: "Llama 3.2 1B".into(),
                provider: "Hugging Face".into(),
                context_window: 128_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.00001,
                output_cost_per_1k: 0.00001,
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: None,
                quality_rating: 2,
                speed_rating: 5,
                is_reasoning: false,
                free_tier: true,
            },

            // ═══════════════════════════════════════════════════════════
            // QWEN
            // ═══════════════════════════════════════════════════════════
            ModelInfo {
                id: "Qwen/Qwen2.5-72B-Instruct".into(),
                name: "Qwen 2.5 72B".into(),
                provider: "Hugging Face".into(),
                context_window: 128_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.0006,
                output_cost_per_1k: 0.0006,
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
                id: "Qwen/Qwen2.5-32B-Instruct".into(),
                name: "Qwen 2.5 32B".into(),
                provider: "Hugging Face".into(),
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
                speed_rating: 5,
                is_reasoning: false,
                free_tier: false,
            },
            ModelInfo {
                id: "Qwen/Qwen2.5-7B-Instruct".into(),
                name: "Qwen 2.5 7B".into(),
                provider: "Hugging Face".into(),
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
                free_tier: true,
            },
            ModelInfo {
                id: "Qwen/Qwen2.5-Coder-32B-Instruct".into(),
                name: "Qwen 2.5 Coder 32B".into(),
                provider: "Hugging Face".into(),
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
                speed_rating: 5,
                is_reasoning: false,
                free_tier: false,
            },

            // ═══════════════════════════════════════════════════════════
            // MISTRAL
            // ═══════════════════════════════════════════════════════════
            ModelInfo {
                id: "mistralai/Mixtral-8x22B-Instruct-v0.1".into(),
                name: "Mixtral 8x22B".into(),
                provider: "Hugging Face".into(),
                context_window: 64_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.0009,
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
            ModelInfo {
                id: "mistralai/Mixtral-8x7B-Instruct-v0.1".into(),
                name: "Mixtral 8x7B".into(),
                provider: "Hugging Face".into(),
                context_window: 32_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.0003,
                output_cost_per_1k: 0.0003,
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: None,
                quality_rating: 3,
                speed_rating: 4,
                is_reasoning: false,
                free_tier: true,
            },
            ModelInfo {
                id: "mistralai/Mistral-7B-Instruct-v0.3".into(),
                name: "Mistral 7B v0.3".into(),
                provider: "Hugging Face".into(),
                context_window: 32_000,
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
                free_tier: true,
            },

            // ═══════════════════════════════════════════════════════════
            // DEEPSEEK
            // ═══════════════════════════════════════════════════════════
            ModelInfo {
                id: "deepseek-ai/DeepSeek-V3".into(),
                name: "DeepSeek V3".into(),
                provider: "Hugging Face".into(),
                context_window: 64_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.00014,
                output_cost_per_1k: 0.00028,
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
                id: "deepseek-ai/DeepSeek-R1".into(),
                name: "DeepSeek R1".into(),
                provider: "Hugging Face".into(),
                context_window: 128_000,
                max_output_tokens: 8_192,
                input_cost_per_1k: 0.00055,
                output_cost_per_1k: 0.0022,
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: None,
                quality_rating: 5,
                speed_rating: 3,
                is_reasoning: true,
                free_tier: false,
            },
            ModelInfo {
                id: "deepseek-ai/DeepSeek-Coder-V2-Instruct".into(),
                name: "DeepSeek Coder V2".into(),
                provider: "Hugging Face".into(),
                context_window: 128_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.00014,
                output_cost_per_1k: 0.00028,
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
            // GOOGLE GEMMA
            // ═══════════════════════════════════════════════════════════
            ModelInfo {
                id: "google/gemma-2-27b-it".into(),
                name: "Gemma 2 27B".into(),
                provider: "Hugging Face".into(),
                context_window: 8_000,
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
            ModelInfo {
                id: "google/gemma-2-9b-it".into(),
                name: "Gemma 2 9B".into(),
                provider: "Hugging Face".into(),
                context_window: 8_000,
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
                free_tier: true,
            },

            // ═══════════════════════════════════════════════════════════
            // PHI
            // ═══════════════════════════════════════════════════════════
            ModelInfo {
                id: "microsoft/Phi-4".into(),
                name: "Phi-4".into(),
                provider: "Hugging Face".into(),
                context_window: 16_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.0001,
                output_cost_per_1k: 0.0001,
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: None,
                quality_rating: 4,
                speed_rating: 5,
                is_reasoning: false,
                free_tier: false,
            },
            ModelInfo {
                id: "microsoft/Phi-3.5-mini-instruct".into(),
                name: "Phi 3.5 Mini".into(),
                provider: "Hugging Face".into(),
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
                free_tier: true,
            },

            // ═══════════════════════════════════════════════════════════
            // SPECIALIZED MODELS
            // ═══════════════════════════════════════════════════════════
            ModelInfo {
                id: "codellama/CodeLlama-70b-Instruct-hf".into(),
                name: "Code Llama 70B".into(),
                provider: "Hugging Face".into(),
                context_window: 4_096,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.0003,
                output_cost_per_1k: 0.0003,
                supports_vision: false,
                supports_tools: false,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: None,
                quality_rating: 3,
                speed_rating: 4,
                is_reasoning: false,
                free_tier: false,
            },
            ModelInfo {
                id: "bigcode/starcoder2-15b".into(),
                name: "StarCoder2 15B".into(),
                provider: "Hugging Face".into(),
                context_window: 16_384,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.00005,
                output_cost_per_1k: 0.00005,
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
            ModelInfo {
                id: "tiiuae/falcon-180B-chat".into(),
                name: "Falcon 180B".into(),
                provider: "Hugging Face".into(),
                context_window: 2_048,
                max_output_tokens: 2_048,
                input_cost_per_1k: 0.0007,
                output_cost_per_1k: 0.0007,
                supports_vision: false,
                supports_tools: false,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: None,
                quality_rating: 3,
                speed_rating: 3,
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
            .post(format!("{}/{}", self.base_url, request.model))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&serde_json::json!({
                "inputs": request.messages.last()
                    .map(|m| m.content.as_text().unwrap_or_default())
                    .unwrap_or_default(),
                "parameters": {
                    "max_new_tokens": request.max_tokens,
                    "temperature": request.temperature,
                }
            }))
            .send()
            .await
            .map_err(|e| LlmError::Network(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(LlmError::ServerError(status.as_u16(), body));
        }

        // HF returns different format, convert to standard
        let hf_response: serde_json::Value = response.json().await
            .map_err(|e| LlmError::ParseError(e.to_string()))?;

        let text = hf_response.as_array()
            .and_then(|arr| arr.first())
            .and_then(|first| first.get("generated_text"))
            .and_then(|t| t.as_str())
            .unwrap_or("");

        Ok(ChatResponse {
            id: uuid::Uuid::new_v4().to_string(),
            object: "chat.completion".into(),
            created: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs() as i64)
                .unwrap_or(0),
            model: request.model.clone(),
            choices: vec![crate::types::Choice {
                index: 0,
                message: crate::types::Message {
                    role: crate::types::Role::Assistant,
                    content: crate::types::Content::Text(text.to_string()),
                    name: None,
                    tool_calls: None,
                    tool_call_id: None,
                },
                finish_reason: Some("stop".into()),
                logprobs: None,
            }],
            usage: crate::types::Usage {
                prompt_tokens: 0,
                completion_tokens: 0,
                total_tokens: 0,
            },
            system_fingerprint: None,
        })
    }

    async fn chat_stream(
        &self,
        _request: ChatRequest,
    ) -> LlmResult<Pin<Box<dyn Stream<Item = LlmResult<StreamChunk>> + Send>>> {
        // HF serverless doesn't support streaming well, return empty stream
        Ok(Box::pin(stream::empty()))
    }

    fn count_tokens(&self, text: &str, _model: &str) -> LlmResult<usize> {
        Ok(text.len() / 4)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_huggingface_provider() {
        let provider = HuggingFaceProvider::new("test-key");
        assert!(provider.is_ok());
    }

    #[test]
    fn test_huggingface_models() {
        let provider = HuggingFaceProvider::new("test-key").unwrap();
        let models = provider.models();
        assert!(models.len() >= 20);
    }
}
