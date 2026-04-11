//! ─── OpenRouter Provider ───
//!
//! OpenRouter - 200+ models through single API
//! https://openrouter.ai

use async_trait::async_trait;
use futures::{Stream, StreamExt};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::pin::Pin;

use crate::error::{LlmError, LlmResult};
use crate::types::{ChatRequest, ChatResponse, ModelInfo, StreamChunk, Role};
use crate::provider::LlmProvider;

use super::build_client;

/// OpenRouter provider - 200+ models through single API
pub struct OpenRouterProvider {
    client: Client,
    api_key: String,
    base_url: String,
}

impl OpenRouterProvider {
    pub fn new(api_key: impl Into<String>) -> LlmResult<Self> {
        Ok(Self {
            client: build_client()?,
            api_key: api_key.into(),
            base_url: "https://openrouter.ai/api/v1".into(),
        })
    }

    pub fn from_env() -> LlmResult<Self> {
        let api_key = std::env::var("OPENROUTER_API_KEY")
            .map_err(|_| LlmError::Authentication("OPENROUTER_API_KEY not set".into()))?;
        Self::new(api_key)
    }

    fn convert_request(&self, request: ChatRequest) -> OpenRouterRequest {
        OpenRouterRequest {
            model: request.model,
            messages: request.messages.into_iter().map(|m| OpenRouterMessage {
                role: m.role.to_string(),
                content: m.content.as_text().map(|s| s.to_string()).unwrap_or_default(),
            }).collect(),
            max_tokens: request.max_tokens,
            temperature: request.temperature,
            top_p: request.top_p,
            stream: request.stream,
        }
    }
}

#[async_trait]
impl LlmProvider for OpenRouterProvider {
    fn name(&self) -> &str { "OpenRouter" }
    fn id(&self) -> &str { "openrouter" }

    fn models(&self) -> Vec<ModelInfo> {
        vec![
            // ═══════════════════════════════════════════════════════════
            // TOP TIER - Best quality
            // ═══════════════════════════════════════════════════════════
            ModelInfo {
                id: "anthropic/claude-4-opus".into(),
                name: "Claude 4 Opus".into(),
                provider: "OpenRouter".into(),
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
                name: "Claude 3.5 Sonnet".into(),
                provider: "OpenRouter".into(),
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
                id: "openai/gpt-4o".into(),
                name: "GPT-4o".into(),
                provider: "OpenRouter".into(),
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
                id: "google/gemini-pro-1.5".into(),
                name: "Gemini 1.5 Pro".into(),
                provider: "OpenRouter".into(),
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
            // REASONING MODELS
            // ═══════════════════════════════════════════════════════════
            ModelInfo {
                id: "openai/o1-preview".into(),
                name: "o1 Preview".into(),
                provider: "OpenRouter".into(),
                context_window: 128_000,
                max_output_tokens: 32_768,
                input_cost_per_1k: 0.015,
                output_cost_per_1k: 0.06,
                supports_vision: false,
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
                id: "openai/o1-mini".into(),
                name: "o1 Mini".into(),
                provider: "OpenRouter".into(),
                context_window: 128_000,
                max_output_tokens: 65_536,
                input_cost_per_1k: 0.003,
                output_cost_per_1k: 0.012,
                supports_vision: false,
                supports_tools: false,
                supports_streaming: false,
                supports_json: true,
                training_cutoff: None,
                quality_rating: 4,
                speed_rating: 3,
                is_reasoning: true,
                free_tier: false,
            },
            ModelInfo {
                id: "deepseek/deepseek-r1".into(),
                name: "DeepSeek R1".into(),
                provider: "OpenRouter".into(),
                context_window: 128_000,
                max_output_tokens: 8_192,
                input_cost_per_1k: 0.00055,
                output_cost_per_1k: 0.00219,
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

            // ═══════════════════════════════════════════════════════════
            // OPEN SOURCE - LLAMA
            // ═══════════════════════════════════════════════════════════
            ModelInfo {
                id: "meta-llama/llama-3.3-70b-instruct".into(),
                name: "Llama 3.3 70B".into(),
                provider: "OpenRouter".into(),
                context_window: 128_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.00035,
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
                id: "meta-llama/llama-3.2-90b-vision-instruct".into(),
                name: "Llama 3.2 90B Vision".into(),
                provider: "OpenRouter".into(),
                context_window: 128_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.0009,
                output_cost_per_1k: 0.0009,
                supports_vision: true,
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
                id: "meta-llama/llama-3.2-11b-vision-instruct".into(),
                name: "Llama 3.2 11B Vision".into(),
                provider: "OpenRouter".into(),
                context_window: 128_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.000055,
                output_cost_per_1k: 0.000055,
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
            ModelInfo {
                id: "meta-llama/llama-3.2-3b-instruct".into(),
                name: "Llama 3.2 3B".into(),
                provider: "OpenRouter".into(),
                context_window: 128_000,
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
                id: "meta-llama/llama-3.1-405b-instruct".into(),
                name: "Llama 3.1 405B".into(),
                provider: "OpenRouter".into(),
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
                speed_rating: 2,
                is_reasoning: false,
                free_tier: false,
            },
            ModelInfo {
                id: "meta-llama/llama-3.1-8b-instruct".into(),
                name: "Llama 3.1 8B".into(),
                provider: "OpenRouter".into(),
                context_window: 128_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.00001,
                output_cost_per_1k: 0.00001,
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
            // OPEN SOURCE - MISTRAL
            // ═══════════════════════════════════════════════════════════
            ModelInfo {
                id: "mistralai/mistral-large-2411".into(),
                name: "Mistral Large 2".into(),
                provider: "OpenRouter".into(),
                context_window: 128_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.002,
                output_cost_per_1k: 0.006,
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
                id: "mistralai/mixtral-8x22b-instruct".into(),
                name: "Mixtral 8x22B".into(),
                provider: "OpenRouter".into(),
                context_window: 64_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.00065,
                output_cost_per_1k: 0.00065,
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
                id: "mistralai/mixtral-8x7b-instruct".into(),
                name: "Mixtral 8x7B".into(),
                provider: "OpenRouter".into(),
                context_window: 32_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.00024,
                output_cost_per_1k: 0.00024,
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
                id: "mistralai/ministral-8b".into(),
                name: "Ministral 8B".into(),
                provider: "OpenRouter".into(),
                context_window: 128_000,
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
                free_tier: false,
            },

            // ═══════════════════════════════════════════════════════════
            // OPEN SOURCE - QWEN
            // ═══════════════════════════════════════════════════════════
            ModelInfo {
                id: "qwen/qwen-2.5-72b-instruct".into(),
                name: "Qwen 2.5 72B".into(),
                provider: "OpenRouter".into(),
                context_window: 128_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.00035,
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
                id: "qwen/qwen-2.5-32b-instruct".into(),
                name: "Qwen 2.5 32B".into(),
                provider: "OpenRouter".into(),
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
                id: "qwen/qwen-2.5-7b-instruct".into(),
                name: "Qwen 2.5 7B".into(),
                provider: "OpenRouter".into(),
                context_window: 128_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.000035,
                output_cost_per_1k: 0.000035,
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
                id: "qwen/qwen-2-vl-7b-instruct".into(),
                name: "Qwen 2 VL 7B".into(),
                provider: "OpenRouter".into(),
                context_window: 32_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.0001,
                output_cost_per_1k: 0.0001,
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
            // OPEN SOURCE - DEEPSEEK
            // ═══════════════════════════════════════════════════════════
            ModelInfo {
                id: "deepseek/deepseek-chat".into(),
                name: "DeepSeek V3".into(),
                provider: "OpenRouter".into(),
                context_window: 64_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.00007,
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
                id: "deepseek/deepseek-coder".into(),
                name: "DeepSeek Coder".into(),
                provider: "OpenRouter".into(),
                context_window: 64_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.00007,
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
            // OTHER POPULAR MODELS
            // ═══════════════════════════════════════════════════════════
            ModelInfo {
                id: "google/gemma-2-27b-it".into(),
                name: "Gemma 2 27B".into(),
                provider: "OpenRouter".into(),
                context_window: 8_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.00008,
                output_cost_per_1k: 0.00008,
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
                id: "google/gemma-2-9b-it".into(),
                name: "Gemma 2 9B".into(),
                provider: "OpenRouter".into(),
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
                free_tier: false,
            },
            ModelInfo {
                id: "cohere/command-r-plus".into(),
                name: "Command R+".into(),
                provider: "OpenRouter".into(),
                context_window: 128_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.00025,
                output_cost_per_1k: 0.001,
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
                id: "cohere/command-r".into(),
                name: "Command R".into(),
                provider: "OpenRouter".into(),
                context_window: 128_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.00005,
                output_cost_per_1k: 0.00015,
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
                id: "x-ai/grok-beta".into(),
                name: "Grok Beta".into(),
                provider: "OpenRouter".into(),
                context_window: 128_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.005,
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
                id: "nousresearch/hermes-3-llama-3.1-405b".into(),
                name: "Hermes 3 405B".into(),
                provider: "OpenRouter".into(),
                context_window: 128_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.002,
                output_cost_per_1k: 0.002,
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: None,
                quality_rating: 4,
                speed_rating: 2,
                is_reasoning: false,
                free_tier: false,
            },
            ModelInfo {
                id: "cognitivecomputations/dolphin-mixtral-8x22b".into(),
                name: "Dolphin Mixtral 8x22B".into(),
                provider: "OpenRouter".into(),
                context_window: 64_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.0009,
                output_cost_per_1k: 0.0009,
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                supports_json: true,
                training_cutoff: None,
                quality_rating: 3,
                speed_rating: 3,
                is_reasoning: false,
                free_tier: false,
            },
            ModelInfo {
                id: "teknium/openhermes-2.5-mistral-7b".into(),
                name: "OpenHermes 2.5".into(),
                provider: "OpenRouter".into(),
                context_window: 4_096,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.000058,
                output_cost_per_1k: 0.000058,
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
            // FREE TIER MODELS (OpenRouter Free)
            // ═══════════════════════════════════════════════════════════
            ModelInfo {
                id: "meta-llama/llama-3.2-3b-instruct:free".into(),
                name: "Llama 3.2 3B (Free)".into(),
                provider: "OpenRouter".into(),
                context_window: 128_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.0,
                output_cost_per_1k: 0.0,
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
                id: "google/gemma-2-9b-it:free".into(),
                name: "Gemma 2 9B (Free)".into(),
                provider: "OpenRouter".into(),
                context_window: 8_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.0,
                output_cost_per_1k: 0.0,
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
                id: "qwen/qwen-2-7b-instruct:free".into(),
                name: "Qwen 2 7B (Free)".into(),
                provider: "OpenRouter".into(),
                context_window: 32_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.0,
                output_cost_per_1k: 0.0,
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
                id: "mistralai/mistral-7b-instruct:free".into(),
                name: "Mistral 7B (Free)".into(),
                provider: "OpenRouter".into(),
                context_window: 32_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.0,
                output_cost_per_1k: 0.0,
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
        ]
    }

    fn is_configured(&self) -> bool {
        !self.api_key.is_empty()
    }

    async fn chat(&self, request: ChatRequest) -> LlmResult<ChatResponse> {
        let openrouter_request = self.convert_request(request);

        let response = self.client
            .post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("HTTP-Referer", "https://sentient-os.ai")
            .header("X-Title", "SENTIENT OS")
            .json(&openrouter_request)
            .send()
            .await
            .map_err(|e| LlmError::Network(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(LlmError::ServerError(status.as_u16(), body));
        }

        let openrouter_response: OpenRouterResponse = response.json().await
            .map_err(|e| LlmError::ParseError(e.to_string()))?;

        Ok(openrouter_response.into())
    }

    async fn chat_stream(
        &self,
        request: ChatRequest,
    ) -> LlmResult<Pin<Box<dyn Stream<Item = LlmResult<StreamChunk>> + Send>>> {
        let mut openrouter_request = self.convert_request(request);
        openrouter_request.stream = true;

        let model = openrouter_request.model.clone();
        let response = self.client
            .post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("HTTP-Referer", "https://sentient-os.ai")
            .header("X-Title", "SENTIENT OS")
            .json(&openrouter_request)
            .send()
            .await
            .map_err(|e| LlmError::Network(e.to_string()))?;

        let stream = response.bytes_stream()
            .map(move |result| {
                match result {
                    Ok(bytes) => {
                        let text = String::from_utf8_lossy(&bytes);
                        for line in text.lines() {
                            if line.starts_with("data: ") {
                                let data = &line[6..];
                                if data == "[DONE]" {
                                    return Ok(None);
                                }
                                if let Ok(chunk) = serde_json::from_str::<OpenRouterStreamChunk>(data) {
                                    return Ok(Some(StreamChunk {
                                        id: chunk.id,
                                        object: chunk.object,
                                        created: chunk.created,
                                        model: model.clone(),
                                        system_fingerprint: None,
                                        choices: chunk.choices.into_iter().map(|c| {
                                            crate::types::StreamChoice {
                                                index: c.index,
                                                delta: crate::types::Delta {
                                                    role: c.delta.role.map(|r| match r.as_str() {
                                                        "assistant" => Role::Assistant,
                                                        "user" => Role::User,
                                                        "system" => Role::System,
                                                        _ => Role::Assistant,
                                                    }),
                                                    content: c.delta.content,
                                                    tool_calls: None,
                                                },
                                                finish_reason: c.finish_reason,
                                            }
                                        }).collect(),
                                    }));
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

// ═══════════════════════════════════════════════════════════════════════════════
//  OPENROUTER API TYPES
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Serialize)]
struct OpenRouterRequest {
    model: String,
    messages: Vec<OpenRouterMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
    stream: bool,
}

#[derive(Debug, Serialize)]
struct OpenRouterMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct OpenRouterResponse {
    id: String,
    object: String,
    created: i64,
    model: String,
    choices: Vec<OpenRouterChoice>,
    usage: OpenRouterUsage,
}

impl From<OpenRouterResponse> for ChatResponse {
    fn from(resp: OpenRouterResponse) -> Self {
        ChatResponse {
            id: resp.id,
            object: resp.object,
            created: resp.created,
            model: resp.model,
            choices: resp.choices.into_iter().map(|c| crate::types::Choice {
                index: c.index,
                message: crate::types::Message::assistant(&c.message.content),
                finish_reason: c.finish_reason,
                logprobs: None,
            }).collect(),
            usage: crate::types::Usage {
                prompt_tokens: resp.usage.prompt_tokens,
                completion_tokens: resp.usage.completion_tokens,
                total_tokens: resp.usage.total_tokens,
            },
            system_fingerprint: None,
        }
    }
}

#[derive(Debug, Deserialize)]
struct OpenRouterChoice {
    index: u32,
    message: OpenRouterMessageResponse,
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct OpenRouterMessageResponse {
    content: String,
}

#[derive(Debug, Deserialize)]
struct OpenRouterUsage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

#[derive(Debug, Deserialize)]
struct OpenRouterStreamChunk {
    id: String,
    object: String,
    created: i64,
    choices: Vec<OpenRouterStreamChoice>,
}

#[derive(Debug, Deserialize)]
struct OpenRouterStreamChoice {
    index: u32,
    delta: OpenRouterDelta,
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct OpenRouterDelta {
    #[serde(default)]
    role: Option<String>,
    #[serde(default)]
    content: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_openrouter_provider() {
        let provider = OpenRouterProvider::new("test-key");
        assert!(provider.is_ok());
    }

    #[test]
    fn test_openrouter_models() {
        let provider = OpenRouterProvider::new("test-key").unwrap();
        let models = provider.models();
        assert!(models.len() > 30);
        assert!(models.iter().any(|m| m.free_tier));
    }
}
