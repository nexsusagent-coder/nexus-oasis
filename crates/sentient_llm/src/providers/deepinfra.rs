//! ─── DeepInfra Provider ───
//!
//! DeepInfra - Cheap LLM inference
//! https://deepinfra.com

use async_trait::async_trait;
use futures::{Stream, StreamExt};
use reqwest::Client;
use std::pin::Pin;

use crate::error::{LlmError, LlmResult};
use crate::types::{ChatRequest, ChatResponse, ModelInfo, StreamChunk};
use crate::provider::LlmProvider;

use super::build_client;

/// DeepInfra provider - Cheap LLM inference
pub struct DeepInfraProvider {
    client: Client,
    api_key: String,
    base_url: String,
}

impl DeepInfraProvider {
    pub fn new(api_key: impl Into<String>) -> LlmResult<Self> {
        Ok(Self {
            client: build_client()?,
            api_key: api_key.into(),
            base_url: "https://api.deepinfra.com/v1/openai".into(),
        })
    }

    pub fn from_env() -> LlmResult<Self> {
        let api_key = std::env::var("DEEPINFRA_API_KEY")
            .map_err(|_| LlmError::Authentication("DEEPINFRA_API_KEY not set".into()))?;
        Self::new(api_key)
    }
}

#[async_trait]
impl LlmProvider for DeepInfraProvider {
    fn name(&self) -> &str { "DeepInfra" }
    fn id(&self) -> &str { "deepinfra" }

    fn models(&self) -> Vec<ModelInfo> {
        vec![
            ModelInfo {
                id: "meta-llama/Meta-Llama-3.1-405B-Instruct".into(),
                name: "Llama 3.1 405B".into(),
                provider: "DeepInfra".into(),
                context_window: 128_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.0008,
                output_cost_per_1k: 0.0008,
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
                provider: "DeepInfra".into(),
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
                id: "meta-llama/Meta-Llama-3.1-8B-Instruct".into(),
                name: "Llama 3.1 8B".into(),
                provider: "DeepInfra".into(),
                context_window: 128_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.00002,
                output_cost_per_1k: 0.00002,
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
                id: "meta-llama/Llama-3.3-70B-Instruct".into(),
                name: "Llama 3.3 70B".into(),
                provider: "DeepInfra".into(),
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
                id: "Qwen/Qwen2.5-72B-Instruct".into(),
                name: "Qwen 2.5 72B".into(),
                provider: "DeepInfra".into(),
                context_window: 128_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.00025,
                output_cost_per_1k: 0.00025,
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
                provider: "DeepInfra".into(),
                context_window: 128_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.00012,
                output_cost_per_1k: 0.00012,
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
                provider: "DeepInfra".into(),
                context_window: 128_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.00002,
                output_cost_per_1k: 0.00002,
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
                id: "mistralai/Mixtral-8x22B-Instruct-v0.1".into(),
                name: "Mixtral 8x22B".into(),
                provider: "DeepInfra".into(),
                context_window: 64_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.0004,
                output_cost_per_1k: 0.0004,
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
                provider: "DeepInfra".into(),
                context_window: 32_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.00012,
                output_cost_per_1k: 0.00012,
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
                id: "mistralai/Mistral-7B-Instruct-v0.3".into(),
                name: "Mistral 7B".into(),
                provider: "DeepInfra".into(),
                context_window: 32_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.000015,
                output_cost_per_1k: 0.000015,
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
                id: "deepseek-ai/DeepSeek-V3".into(),
                name: "DeepSeek V3".into(),
                provider: "DeepInfra".into(),
                context_window: 64_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.00005,
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
                id: "deepseek-ai/DeepSeek-R1".into(),
                name: "DeepSeek R1".into(),
                provider: "DeepInfra".into(),
                context_window: 128_000,
                max_output_tokens: 8_192,
                input_cost_per_1k: 0.0004,
                output_cost_per_1k: 0.001,
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
    fn test_deepinfra_provider() {
        let provider = DeepInfraProvider::new("test-key");
        assert!(provider.is_ok());
    }

    #[test]
    fn test_deepinfra_models() {
        let provider = DeepInfraProvider::new("test-key").unwrap();
        let models = provider.models();
        assert!(models.len() >= 10);
    }
}
