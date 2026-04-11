//! ─── Azure OpenAI Provider ───
//!
//! Azure OpenAI Service - Enterprise OpenAI on Azure
//! https://azure.microsoft.com/en-us/products/ai-services/openai-service

use async_trait::async_trait;
use futures::{Stream, StreamExt};
use reqwest::Client;
use std::pin::Pin;

use crate::error::{LlmError, LlmResult};
use crate::types::{ChatRequest, ChatResponse, ModelInfo, StreamChunk};
use crate::provider::LlmProvider;

use super::build_client;

/// Azure OpenAI provider - Enterprise OpenAI on Azure
pub struct AzureOpenAIProvider {
    client: Client,
    api_key: String,
    endpoint: String,
    deployment_name: String,
    api_version: String,
}

impl AzureOpenAIProvider {
    pub fn new(
        api_key: impl Into<String>,
        endpoint: impl Into<String>,
        deployment_name: impl Into<String>,
    ) -> LlmResult<Self> {
        Ok(Self {
            client: build_client()?,
            api_key: api_key.into(),
            endpoint: endpoint.into(),
            deployment_name: deployment_name.into(),
            api_version: "2024-02-15-preview".into(),
        })
    }

    pub fn from_env() -> LlmResult<Self> {
        let api_key = std::env::var("AZURE_OPENAI_API_KEY")
            .map_err(|_| LlmError::Authentication("AZURE_OPENAI_API_KEY not set".into()))?;
        let endpoint = std::env::var("AZURE_OPENAI_ENDPOINT")
            .map_err(|_| LlmError::Authentication("AZURE_OPENAI_ENDPOINT not set".into()))?;
        let deployment = std::env::var("AZURE_OPENAI_DEPLOYMENT_NAME")
            .unwrap_or_else(|_| "gpt-4o".into());
        Self::new(api_key, endpoint, deployment)
    }

    fn build_url(&self) -> String {
        format!(
            "{}/openai/deployments/{}/chat/completions?api-version={}",
            self.endpoint.trim_end_matches('/'),
            self.deployment_name,
            self.api_version
        )
    }
}

#[async_trait]
impl LlmProvider for AzureOpenAIProvider {
    fn name(&self) -> &str { "Azure OpenAI" }
    fn id(&self) -> &str { "azure" }

    fn models(&self) -> Vec<ModelInfo> {
        vec![
            // ═══════════════════════════════════════════════════════════
            // GPT-4 FAMILY (Azure deployments)
            // ═══════════════════════════════════════════════════════════
            ModelInfo {
                id: "gpt-4o".into(),
                name: "GPT-4o (Azure)".into(),
                provider: "Azure OpenAI".into(),
                context_window: 128_000,
                max_output_tokens: 16_384,
                input_cost_per_1k: 0.005,
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
                id: "gpt-4o-mini".into(),
                name: "GPT-4o Mini (Azure)".into(),
                provider: "Azure OpenAI".into(),
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
            ModelInfo {
                id: "gpt-4-turbo".into(),
                name: "GPT-4 Turbo (Azure)".into(),
                provider: "Azure OpenAI".into(),
                context_window: 128_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.01,
                output_cost_per_1k: 0.03,
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
                id: "gpt-4".into(),
                name: "GPT-4 (Azure)".into(),
                provider: "Azure OpenAI".into(),
                context_window: 8_192,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.03,
                output_cost_per_1k: 0.06,
                supports_vision: false,
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
                id: "gpt-4-32k".into(),
                name: "GPT-4 32K (Azure)".into(),
                provider: "Azure OpenAI".into(),
                context_window: 32_768,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.06,
                output_cost_per_1k: 0.12,
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: None,
                quality_rating: 5,
                speed_rating: 2,
                is_reasoning: false,
                free_tier: false,
            },

            // ═══════════════════════════════════════════════════════════
            // GPT-3.5 FAMILY
            // ═══════════════════════════════════════════════════════════
            ModelInfo {
                id: "gpt-35-turbo".into(),
                name: "GPT-3.5 Turbo (Azure)".into(),
                provider: "Azure OpenAI".into(),
                context_window: 16_385,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.0005,
                output_cost_per_1k: 0.0015,
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
                id: "gpt-35-turbo-16k".into(),
                name: "GPT-3.5 Turbo 16K (Azure)".into(),
                provider: "Azure OpenAI".into(),
                context_window: 16_385,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.003,
                output_cost_per_1k: 0.004,
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
            // O SERIES - Reasoning Models
            // ═══════════════════════════════════════════════════════════
            ModelInfo {
                id: "o1-preview".into(),
                name: "o1-preview (Azure)".into(),
                provider: "Azure OpenAI".into(),
                context_window: 128_000,
                max_output_tokens: 32_768,
                input_cost_per_1k: 0.015,
                output_cost_per_1k: 0.06,
                supports_vision: true,
                supports_tools: false,
                supports_streaming: false,
                supports_json: true,
                training_cutoff: None,
                quality_rating: 5,
                speed_rating: 2,
                is_reasoning: true,
                free_tier: false,
            },
            ModelInfo {
                id: "o1-mini".into(),
                name: "o1-mini (Azure)".into(),
                provider: "Azure OpenAI".into(),
                context_window: 128_000,
                max_output_tokens: 65_536,
                input_cost_per_1k: 0.003,
                output_cost_per_1k: 0.012,
                supports_vision: true,
                supports_tools: false,
                supports_streaming: false,
                supports_json: true,
                training_cutoff: None,
                quality_rating: 4,
                speed_rating: 3,
                is_reasoning: true,
                free_tier: false,
            },
        ]
    }

    fn is_configured(&self) -> bool {
        !self.api_key.is_empty() && !self.endpoint.is_empty()
    }

    async fn chat(&self, request: ChatRequest) -> LlmResult<ChatResponse> {
        let response = self.client
            .post(self.build_url())
            .header("api-key", &self.api_key)
            .header("Content-Type", "application/json")
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
            .post(self.build_url())
            .header("api-key", &self.api_key)
            .header("Content-Type", "application/json")
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
    fn test_azure_provider() {
        let provider = AzureOpenAIProvider::new("key", "https://test.openai.azure.com", "gpt-4o");
        assert!(provider.is_ok());
    }

    #[test]
    fn test_azure_models() {
        let provider = AzureOpenAIProvider::new("key", "https://test.openai.azure.com", "gpt-4o").unwrap();
        let models = provider.models();
        assert!(models.len() >= 9);
    }
}
