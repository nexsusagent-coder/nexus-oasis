//! ─── Cloudflare Workers AI Provider ───
//!
//! Cloudflare Workers AI - Edge inference, serverless AI
//! https://developers.cloudflare.com/workers-ai

use async_trait::async_trait;
use futures::{Stream, StreamExt};
use reqwest::Client;
use std::pin::Pin;

use crate::error::{LlmError, LlmResult};
use crate::types::{ChatRequest, ChatResponse, ModelInfo, StreamChunk};
use crate::provider::LlmProvider;

use super::build_client;

/// Cloudflare Workers AI provider
pub struct CloudflareAIProvider {
    client: Client,
    api_token: String,
    account_id: String,
    base_url: String,
}

impl CloudflareAIProvider {
    pub fn new(api_token: impl Into<String>, account_id: impl Into<String>) -> LlmResult<Self> {
        Ok(Self {
            client: build_client()?,
            api_token: api_token.into(),
            account_id: account_id.into(),
            base_url: "https://api.cloudflare.com/client/v4".into(),
        })
    }

    pub fn from_env() -> LlmResult<Self> {
        let api_token = std::env::var("CLOUDFLARE_API_TOKEN")
            .map_err(|_| LlmError::Authentication("CLOUDFLARE_API_TOKEN not set".into()))?;
        let account_id = std::env::var("CLOUDFLARE_ACCOUNT_ID")
            .map_err(|_| LlmError::Authentication("CLOUDFLARE_ACCOUNT_ID not set".into()))?;
        Self::new(api_token, account_id)
    }
}

#[async_trait]
impl LlmProvider for CloudflareAIProvider {
    fn name(&self) -> &str { "Cloudflare AI" }
    fn id(&self) -> &str { "cloudflare" }

    fn models(&self) -> Vec<ModelInfo> {
        vec![
            ModelInfo {
                id: "@cf/meta/llama-3.3-70b-instruct-fp8-fast".into(),
                name: "Llama 3.3 70B (CF)".into(),
                provider: "Cloudflare".into(),
                context_window: 65_536,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.00075,
                output_cost_per_1k: 0.00075,
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: Some("2024-12".into()),
                quality_rating: 5,
                speed_rating: 5,
                is_reasoning: false,
                free_tier: true,
            },
            ModelInfo {
                id: "@cf/meta/llama-3.1-8b-instruct".into(),
                name: "Llama 3.1 8B (CF)".into(),
                provider: "Cloudflare".into(),
                context_window: 32_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.0001,
                output_cost_per_1k: 0.0001,
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: Some("2024-07".into()),
                quality_rating: 3,
                speed_rating: 5,
                is_reasoning: false,
                free_tier: true,
            },
            ModelInfo {
                id: "@cf/mistralai/mistral-small-3.1-24b-instruct".into(),
                name: "Mistral Small 3.1 24B (CF)".into(),
                provider: "Cloudflare".into(),
                context_window: 32_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.0002,
                output_cost_per_1k: 0.0002,
                supports_vision: true,
                supports_tools: true,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: Some("2025-02".into()),
                quality_rating: 4,
                speed_rating: 5,
                is_reasoning: false,
                free_tier: true,
            },
            ModelInfo {
                id: "@cf/qwen/qwen1.5-14b-chat-awq".into(),
                name: "Qwen 1.5 14B (CF)".into(),
                provider: "Cloudflare".into(),
                context_window: 32_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.0001,
                output_cost_per_1k: 0.0001,
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: Some("2024-02".into()),
                quality_rating: 3,
                speed_rating: 5,
                is_reasoning: false,
                free_tier: true,
            },
            ModelInfo {
                id: "@cf/deepseek-ai/deepseek-r1-distill-qwen-32b".into(),
                name: "DeepSeek R1 Distill 32B (CF)".into(),
                provider: "Cloudflare".into(),
                context_window: 32_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.0003,
                output_cost_per_1k: 0.0003,
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: Some("2024-11".into()),
                quality_rating: 4,
                speed_rating: 4,
                is_reasoning: true,
                free_tier: true,
            },
        ]
    }

    fn is_configured(&self) -> bool { !self.api_token.is_empty() && !self.account_id.is_empty() }

    async fn chat(&self, request: ChatRequest) -> LlmResult<ChatResponse> {
        let model = &request.model;
        let response = self.client
            .post(format!("{}/accounts/{}/ai/run/{}", self.base_url, self.account_id, model))
            .header("Authorization", format!("Bearer {}", self.api_token))
            .json(&serde_json::json!({
                "messages": request.messages.iter().map(|m| serde_json::json!({
                    "role": m.role.to_string(),
                    "content": m.content.as_text().unwrap_or("")
                })).collect::<Vec<_>>(),
                "max_tokens": request.max_tokens,
                "temperature": request.temperature,
                "stream": false,
            }))
            .send()
            .await
            .map_err(|e| LlmError::Network(e.to_string()))?;

        if !response.status().is_success() {
            let err = response.text().await.unwrap_or_default();
            return Err(LlmError::ApiError(format!("Cloudflare error: {}", err)));
        }
        response.json().await.map_err(|e| LlmError::ParseError(e.to_string()))
    }

    async fn chat_stream(
        &self,
        _request: ChatRequest,
    ) -> LlmResult<Pin<Box<dyn Stream<Item = LlmResult<StreamChunk>> + Send>>> {
        Err(LlmError::ApiError("Cloudflare streaming via SSE not yet implemented".into()))
    }

    fn count_tokens(&self, text: &str, _model: &str) -> LlmResult<usize> {
        Ok(text.len() / 4)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_cloudflare_provider() { assert!(CloudflareAIProvider::new("test-token", "test-account").is_ok()); }
    #[test]
    fn test_cloudflare_models() { let p = CloudflareAIProvider::new("t", "a").unwrap(); assert!(p.models().len() >= 5); }
}
