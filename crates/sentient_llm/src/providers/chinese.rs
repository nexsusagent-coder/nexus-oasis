//! ─── Chinese AI Providers ───
//!
//! Zhipu AI (GLM), Moonshot (Kimi), Yi (01.AI), Baichuan, Alibaba Qwen

use async_trait::async_trait;
use futures::{Stream, StreamExt};
use reqwest::Client;
use std::pin::Pin;

use crate::error::{LlmError, LlmResult};
use crate::types::{ChatRequest, ChatResponse, ModelInfo, StreamChunk};
use crate::provider::LlmProvider;

use super::build_client;

// ═══════════════════════════════════════════════════════════════════════════════
//  ZHIPU AI (GLM)
// ═══════════════════════════════════════════════════════════════════════════════

/// Zhipu AI provider - GLM models
pub struct ZhipuProvider {
    client: Client,
    api_key: String,
    base_url: String,
}

impl ZhipuProvider {
    pub fn new(api_key: impl Into<String>) -> LlmResult<Self> {
        Ok(Self {
            client: build_client()?,
            api_key: api_key.into(),
            base_url: "https://open.bigmodel.cn/api/paas/v4".into(),
        })
    }

    pub fn from_env() -> LlmResult<Self> {
        let api_key = std::env::var("ZHIPU_API_KEY")
            .map_err(|_| LlmError::Authentication("ZHIPU_API_KEY not set".into()))?;
        Self::new(api_key)
    }
}

#[async_trait]
impl LlmProvider for ZhipuProvider {
    fn name(&self) -> &str { "Zhipu AI" }
    fn id(&self) -> &str { "zhipu" }

    fn models(&self) -> Vec<ModelInfo> {
        vec![
            ModelInfo {
                id: "glm-4-plus".into(),
                name: "GLM-4 Plus".into(),
                provider: "Zhipu AI".into(),
                context_window: 128_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.006,
                output_cost_per_1k: 0.006,
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
                id: "glm-4-0520".into(),
                name: "GLM-4 0520".into(),
                provider: "Zhipu AI".into(),
                context_window: 128_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.014,
                output_cost_per_1k: 0.014,
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
                id: "glm-4-air".into(),
                name: "GLM-4 Air".into(),
                provider: "Zhipu AI".into(),
                context_window: 128_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.001,
                output_cost_per_1k: 0.001,
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
                id: "glm-4-flash".into(),
                name: "GLM-4 Flash".into(),
                provider: "Zhipu AI".into(),
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
                free_tier: true,
            },
            ModelInfo {
                id: "glm-4v-plus".into(),
                name: "GLM-4V Plus (Vision)".into(),
                provider: "Zhipu AI".into(),
                context_window: 8_192,
                max_output_tokens: 1_024,
                input_cost_per_1k: 0.01,
                output_cost_per_1k: 0.01,
                supports_vision: true,
                supports_tools: false,
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

    fn is_configured(&self) -> bool { !self.api_key.is_empty() }

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

        response.json().await.map_err(|e| LlmError::ParseError(e.to_string()))
    }

    async fn chat_stream(&self, request: ChatRequest) -> LlmResult<Pin<Box<dyn Stream<Item = LlmResult<StreamChunk>> + Send>>> {
        let mut request = request;
        request.stream = true;

        let response = self.client
            .post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await
            .map_err(|e| LlmError::Network(e.to_string()))?;

        Ok(Box::pin(response.bytes_stream().filter_map(|result| async move {
            match result {
                Ok(bytes) => {
                    let text = String::from_utf8_lossy(&bytes);
                    for line in text.lines() {
                        if line.starts_with("data: ") {
                            let data = &line[6..];
                            if data == "[DONE]" { return None; }
                            if let Ok(chunk) = serde_json::from_str::<StreamChunk>(data) {
                                return Some(Ok(chunk));
                            }
                        }
                    }
                    None
                }
                Err(e) => Some(Err(LlmError::StreamError(e.to_string()))),
            }
        })))
    }

    fn count_tokens(&self, text: &str, _model: &str) -> LlmResult<usize> {
        Ok(text.chars().count() / 2) // Chinese chars
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  MOONSHOT (Kimi)
// ═══════════════════════════════════════════════════════════════════════════════

/// Moonshot AI provider - Kimi models
pub struct MoonshotProvider {
    client: Client,
    api_key: String,
}

impl MoonshotProvider {
    pub fn new(api_key: impl Into<String>) -> LlmResult<Self> {
        Ok(Self {
            client: build_client()?,
            api_key: api_key.into(),
        })
    }

    pub fn from_env() -> LlmResult<Self> {
        let api_key = std::env::var("MOONSHOT_API_KEY")
            .map_err(|_| LlmError::Authentication("MOONSHOT_API_KEY not set".into()))?;
        Self::new(api_key)
    }
}

#[async_trait]
impl LlmProvider for MoonshotProvider {
    fn name(&self) -> &str { "Moonshot AI" }
    fn id(&self) -> &str { "moonshot" }

    fn models(&self) -> Vec<ModelInfo> {
        vec![
            ModelInfo {
                id: "moonshot-v1-8k".into(),
                name: "Moonshot V1 8K".into(),
                provider: "Moonshot".into(),
                context_window: 8_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.012,
                output_cost_per_1k: 0.012,
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
                id: "moonshot-v1-32k".into(),
                name: "Moonshot V1 32K".into(),
                provider: "Moonshot".into(),
                context_window: 32_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.024,
                output_cost_per_1k: 0.024,
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
                id: "moonshot-v1-128k".into(),
                name: "Moonshot V1 128K".into(),
                provider: "Moonshot".into(),
                context_window: 128_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.06,
                output_cost_per_1k: 0.06,
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
        ]
    }

    fn is_configured(&self) -> bool { !self.api_key.is_empty() }

    async fn chat(&self, request: ChatRequest) -> LlmResult<ChatResponse> {
        let response = self.client
            .post("https://api.moonshot.cn/v1/chat/completions")
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

        response.json().await.map_err(|e| LlmError::ParseError(e.to_string()))
    }

    async fn chat_stream(&self, request: ChatRequest) -> LlmResult<Pin<Box<dyn Stream<Item = LlmResult<StreamChunk>> + Send>>> {
        let mut request = request;
        request.stream = true;

        let response = self.client
            .post("https://api.moonshot.cn/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await
            .map_err(|e| LlmError::Network(e.to_string()))?;

        Ok(Box::pin(response.bytes_stream().filter_map(|result| async move {
            match result {
                Ok(bytes) => {
                    let text = String::from_utf8_lossy(&bytes);
                    for line in text.lines() {
                        if line.starts_with("data: ") {
                            let data = &line[6..];
                            if data == "[DONE]" { return None; }
                            if let Ok(chunk) = serde_json::from_str::<StreamChunk>(data) {
                                return Some(Ok(chunk));
                            }
                        }
                    }
                    None
                }
                Err(e) => Some(Err(LlmError::StreamError(e.to_string()))),
            }
        })))
    }

    fn count_tokens(&self, text: &str, _model: &str) -> LlmResult<usize> {
        Ok(text.chars().count() / 2)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  YI (01.AI)
// ═══════════════════════════════════════════════════════════════════════════════

/// Yi (01.AI) provider
pub struct YiProvider {
    client: Client,
    api_key: String,
}

impl YiProvider {
    pub fn new(api_key: impl Into<String>) -> LlmResult<Self> {
        Ok(Self {
            client: build_client()?,
            api_key: api_key.into(),
        })
    }

    pub fn from_env() -> LlmResult<Self> {
        let api_key = std::env::var("YI_API_KEY")
            .map_err(|_| LlmError::Authentication("YI_API_KEY not set".into()))?;
        Self::new(api_key)
    }
}

#[async_trait]
impl LlmProvider for YiProvider {
    fn name(&self) -> &str { "Yi (01.AI)" }
    fn id(&self) -> &str { "yi" }

    fn models(&self) -> Vec<ModelInfo> {
        vec![
            ModelInfo {
                id: "yi-lightning".into(),
                name: "Yi Lightning".into(),
                provider: "Yi".into(),
                context_window: 16_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.0005,
                output_cost_per_1k: 0.0005,
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
                id: "yi-large".into(),
                name: "Yi Large".into(),
                provider: "Yi".into(),
                context_window: 32_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.02,
                output_cost_per_1k: 0.02,
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
                id: "yi-medium".into(),
                name: "Yi Medium".into(),
                provider: "Yi".into(),
                context_window: 16_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.0025,
                output_cost_per_1k: 0.0025,
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
                id: "yi-spark".into(),
                name: "Yi Spark".into(),
                provider: "Yi".into(),
                context_window: 16_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.0002,
                output_cost_per_1k: 0.0002,
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
                id: "yi-vision".into(),
                name: "Yi Vision".into(),
                provider: "Yi".into(),
                context_window: 16_000,
                max_output_tokens: 4_096,
                input_cost_per_1k: 0.006,
                output_cost_per_1k: 0.006,
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
        ]
    }

    fn is_configured(&self) -> bool { !self.api_key.is_empty() }

    async fn chat(&self, request: ChatRequest) -> LlmResult<ChatResponse> {
        let response = self.client
            .post("https://api.lingyiwanwu.com/v1/chat/completions")
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

        response.json().await.map_err(|e| LlmError::ParseError(e.to_string()))
    }

    async fn chat_stream(&self, request: ChatRequest) -> LlmResult<Pin<Box<dyn Stream<Item = LlmResult<StreamChunk>> + Send>>> {
        let mut request = request;
        request.stream = true;

        let response = self.client
            .post("https://api.lingyiwanwu.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await
            .map_err(|e| LlmError::Network(e.to_string()))?;

        Ok(Box::pin(response.bytes_stream().filter_map(|result| async move {
            match result {
                Ok(bytes) => {
                    let text = String::from_utf8_lossy(&bytes);
                    for line in text.lines() {
                        if line.starts_with("data: ") {
                            let data = &line[6..];
                            if data == "[DONE]" { return None; }
                            if let Ok(chunk) = serde_json::from_str::<StreamChunk>(data) {
                                return Some(Ok(chunk));
                            }
                        }
                    }
                    None
                }
                Err(e) => Some(Err(LlmError::StreamError(e.to_string()))),
            }
        })))
    }

    fn count_tokens(&self, text: &str, _model: &str) -> LlmResult<usize> {
        Ok(text.chars().count() / 2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zhipu_provider() {
        let provider = ZhipuProvider::new("test-key");
        assert!(provider.is_ok());
    }

    #[test]
    fn test_zhipu_models() {
        let provider = ZhipuProvider::new("test-key").unwrap();
        assert!(provider.models().len() >= 5);
    }

    #[test]
    fn test_moonshot_provider() {
        let provider = MoonshotProvider::new("test-key");
        assert!(provider.is_ok());
    }

    #[test]
    fn test_moonshot_models() {
        let provider = MoonshotProvider::new("test-key").unwrap();
        assert!(provider.models().len() >= 3);
    }

    #[test]
    fn test_yi_provider() {
        let provider = YiProvider::new("test-key");
        assert!(provider.is_ok());
    }

    #[test]
    fn test_yi_models() {
        let provider = YiProvider::new("test-key").unwrap();
        assert!(provider.models().len() >= 5);
    }
}
