//! ─── AWS Bedrock Provider ───
//!
//! AWS Bedrock - Enterprise LLM service on AWS
//! https://aws.amazon.com/bedrock/

use async_trait::async_trait;
use futures::{Stream, StreamExt};
use reqwest::Client;
use std::pin::Pin;

use crate::error::{LlmError, LlmResult};
use crate::types::{ChatRequest, ChatResponse, ModelInfo, StreamChunk};
use crate::provider::LlmProvider;

use super::build_client;

/// AWS Bedrock provider - Enterprise LLM service on AWS
pub struct BedrockProvider {
    client: Client,
    access_key: String,
    secret_key: String,
    region: String,
}

impl BedrockProvider {
    pub fn new(
        access_key: impl Into<String>,
        secret_key: impl Into<String>,
        region: impl Into<String>,
    ) -> LlmResult<Self> {
        Ok(Self {
            client: build_client()?,
            access_key: access_key.into(),
            secret_key: secret_key.into(),
            region: region.into(),
        })
    }

    pub fn from_env() -> LlmResult<Self> {
        let access_key = std::env::var("AWS_ACCESS_KEY_ID")
            .map_err(|_| LlmError::Authentication("AWS_ACCESS_KEY_ID not set".into()))?;
        let secret_key = std::env::var("AWS_SECRET_ACCESS_KEY")
            .map_err(|_| LlmError::Authentication("AWS_SECRET_ACCESS_KEY not set".into()))?;
        let region = std::env::var("AWS_REGION")
            .unwrap_or_else(|_| "us-east-1".into());
        Self::new(access_key, secret_key, region)
    }
}

#[async_trait]
impl LlmProvider for BedrockProvider {
    fn name(&self) -> &str { "AWS Bedrock" }
    fn id(&self) -> &str { "bedrock" }

    fn models(&self) -> Vec<ModelInfo> {
        vec![
            // ═══════════════════════════════════════════════════════════
            // ANTHROPIC CLAUDE (Bedrock)
            // ═══════════════════════════════════════════════════════════
            ModelInfo {
                id: "anthropic.claude-3-5-sonnet-20241022-v2:0".into(),
                name: "Claude 3.5 Sonnet v2 (Bedrock)".into(),
                provider: "AWS Bedrock".into(),
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
                id: "anthropic.claude-3-5-haiku-20241022-v1:0".into(),
                name: "Claude 3.5 Haiku (Bedrock)".into(),
                provider: "AWS Bedrock".into(),
                context_window: 200_000,
                max_output_tokens: 8_192,
                input_cost_per_1k: 0.0008,
                output_cost_per_1k: 0.004,
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
                id: "anthropic.claude-3-opus-20240229-v1:0".into(),
                name: "Claude 3 Opus (Bedrock)".into(),
                provider: "AWS Bedrock".into(),
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
            ModelInfo {
                id: "anthropic.claude-3-sonnet-20240229-v1:0".into(),
                name: "Claude 3 Sonnet (Bedrock)".into(),
                provider: "AWS Bedrock".into(),
                context_window: 200_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.003,
                output_cost_per_1k: 0.015,
                supports_vision: true,
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
                id: "anthropic.claude-3-haiku-20240307-v1:0".into(),
                name: "Claude 3 Haiku (Bedrock)".into(),
                provider: "AWS Bedrock".into(),
                context_window: 200_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.00025,
                output_cost_per_1k: 0.00125,
                supports_vision: true,
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
            // META LLAMA (Bedrock)
            // ═══════════════════════════════════════════════════════════
            ModelInfo {
                id: "meta.llama3-3-70b-instruct-v1:0".into(),
                name: "Llama 3.3 70B (Bedrock)".into(),
                provider: "AWS Bedrock".into(),
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
                speed_rating: 4,
                is_reasoning: false,
                free_tier: false,
            },
            ModelInfo {
                id: "meta.llama3-1-405b-instruct-v1:0".into(),
                name: "Llama 3.1 405B (Bedrock)".into(),
                provider: "AWS Bedrock".into(),
                context_window: 128_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.00256,
                output_cost_per_1k: 0.00256,
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
                id: "meta.llama3-1-70b-instruct-v1:0".into(),
                name: "Llama 3.1 70B (Bedrock)".into(),
                provider: "AWS Bedrock".into(),
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
                speed_rating: 4,
                is_reasoning: false,
                free_tier: false,
            },
            ModelInfo {
                id: "meta.llama3-1-8b-instruct-v1:0".into(),
                name: "Llama 3.1 8B (Bedrock)".into(),
                provider: "AWS Bedrock".into(),
                context_window: 128_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.00022,
                output_cost_per_1k: 0.00022,
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
            // MISTRAL (Bedrock)
            // ═══════════════════════════════════════════════════════════
            ModelInfo {
                id: "mistral.mistral-large-2402-v1:0".into(),
                name: "Mistral Large (Bedrock)".into(),
                provider: "AWS Bedrock".into(),
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
                id: "mistral.mistral-small-2402-v1:0".into(),
                name: "Mistral Small (Bedrock)".into(),
                provider: "AWS Bedrock".into(),
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
            ModelInfo {
                id: "mistral.mixtral-8x7b-instruct-v0:1".into(),
                name: "Mixtral 8x7B (Bedrock)".into(),
                provider: "AWS Bedrock".into(),
                context_window: 32_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.00045,
                output_cost_per_1k: 0.0007,
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
            // AMAZON TITAN
            // ═══════════════════════════════════════════════════════════
            ModelInfo {
                id: "amazon.titan-text-premier-v1:0".into(),
                name: "Amazon Titan Premier".into(),
                provider: "AWS Bedrock".into(),
                context_window: 32_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.0013,
                output_cost_per_1k: 0.0017,
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
                id: "amazon.titan-text-lite-v1".into(),
                name: "Amazon Titan Lite".into(),
                provider: "AWS Bedrock".into(),
                context_window: 4_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.0003,
                output_cost_per_1k: 0.0004,
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: None,
                quality_rating: 2,
                speed_rating: 5,
                is_reasoning: false,
                free_tier: false,
            },

            // ═══════════════════════════════════════════════════════════
            // COHERE (Bedrock)
            // ═══════════════════════════════════════════════════════════
            ModelInfo {
                id: "cohere.command-r-plus-v1:0".into(),
                name: "Command R+ (Bedrock)".into(),
                provider: "AWS Bedrock".into(),
                context_window: 128_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.003,
                output_cost_per_1k: 0.015,
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
                id: "cohere.command-r-v1:0".into(),
                name: "Command R (Bedrock)".into(),
                provider: "AWS Bedrock".into(),
                context_window: 128_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.0005,
                output_cost_per_1k: 0.0015,
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
            // AI21 (Bedrock)
            // ═══════════════════════════════════════════════════════════
            ModelInfo {
                id: "ai21.jamba-1-5-large-v1:0".into(),
                name: "Jamba 1.5 Large (Bedrock)".into(),
                provider: "AWS Bedrock".into(),
                context_window: 256_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.002,
                output_cost_per_1k: 0.008,
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
                id: "ai21.jamba-1-5-mini-v1:0".into(),
                name: "Jamba 1.5 Mini (Bedrock)".into(),
                provider: "AWS Bedrock".into(),
                context_window: 256_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.0002,
                output_cost_per_1k: 0.0004,
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
        ]
    }

    fn is_configured(&self) -> bool {
        !self.access_key.is_empty() && !self.secret_key.is_empty()
    }

    async fn chat(&self, request: ChatRequest) -> LlmResult<ChatResponse> {
        // Simplified - in production, use AWS SDK for proper signing
        let response = self.client
            .post(format!(
                "https://bedrock-runtime.{}.amazonaws.com/model/{}/invoke",
                self.region, request.model
            ))
            .header("X-Amz-Access-Key", &self.access_key)
            .header("X-Amz-Secret-Key", &self.secret_key)
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
                "https://bedrock-runtime.{}.amazonaws.com/model/{}/invoke-with-response-stream",
                self.region, request.model
            ))
            .header("X-Amz-Access-Key", &self.access_key)
            .header("X-Amz-Secret-Key", &self.secret_key)
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
    fn test_bedrock_provider() {
        let provider = BedrockProvider::new("access", "secret", "us-east-1");
        assert!(provider.is_ok());
    }

    #[test]
    fn test_bedrock_models() {
        let provider = BedrockProvider::new("access", "secret", "us-east-1").unwrap();
        let models = provider.models();
        assert!(models.len() >= 18);
    }
}
