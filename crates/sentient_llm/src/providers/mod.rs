//! ─── LLM Providers ───
//!
//! Provider implementations for various LLM APIs

// ═══════════════════════════════════════════════════════════════════════════════
//  DIRECT PROVIDERS
// ═══════════════════════════════════════════════════════════════════════════════

mod openai;
mod anthropic;
mod google;
mod mistral;
mod deepseek;
mod xai;
mod cohere;
mod perplexity;
mod groq;
mod together;
mod fireworks;
mod replicate;
mod ai21;
mod ollama;

pub use openai::OpenAIProvider;
pub use anthropic::AnthropicProvider;
pub use google::GoogleProvider;
pub use mistral::MistralProvider;
pub use deepseek::DeepSeekProvider;
pub use xai::XAIProvider;
pub use cohere::CohereProvider;
pub use perplexity::PerplexityProvider;
pub use groq::GroqProvider;
pub use together::TogetherProvider;
pub use fireworks::FireworksProvider;
pub use replicate::ReplicateProvider;
pub use ai21::AI21Provider;
pub use ollama::OllamaProvider;

// ═══════════════════════════════════════════════════════════════════════════════
//  AGGREGATOR PROVIDERS (200+ models each)
// ═══════════════════════════════════════════════════════════════════════════════

mod openrouter;
mod glhf;
mod novita;
mod hyperbolic;
mod siliconflow;
mod cerebras;
mod litellm;
mod huggingface;

pub use openrouter::OpenRouterProvider;
pub use glhf::GlhfProvider;
pub use novita::NovitaProvider;
pub use hyperbolic::HyperbolicProvider;
pub use siliconflow::SiliconFlowProvider;
pub use cerebras::CerebrasProvider;
pub use litellm::LiteLLMProvider;
pub use huggingface::HuggingFaceProvider;

// ═══════════════════════════════════════════════════════════════════════════════
//  ENTERPRISE PROVIDERS
// ═══════════════════════════════════════════════════════════════════════════════

mod nvidia;
mod sambanova;
mod deepinfra;
mod azure;
mod bedrock;
mod vertex;

pub use nvidia::NvidiaProvider;
pub use sambanova::SambaNovaProvider;
pub use deepinfra::DeepInfraProvider;
pub use azure::AzureOpenAIProvider;
pub use bedrock::BedrockProvider;
pub use vertex::VertexAIProvider;

// ═══════════════════════════════════════════════════════════════════════════════
//  LOCAL PROVIDERS
// ═══════════════════════════════════════════════════════════════════════════════

mod vllm;
mod lmstudio;

pub use vllm::VLLMProvider;
pub use lmstudio::LmStudioProvider;

// ═══════════════════════════════════════════════════════════════════════════════
//  CHINESE AI PROVIDERS
// ═══════════════════════════════════════════════════════════════════════════════

mod chinese;

pub use chinese::ZhipuProvider;
pub use chinese::MoonshotProvider;
pub use chinese::YiProvider;

use crate::error::{LlmError, LlmResult};
use reqwest::{Client, Response};
use serde::Deserialize;

// ═══════════════════════════════════════════════════════════════════════════════
//  HTTP CLIENT UTILITIES
// ═══════════════════════════════════════════════════════════════════════════════

/// Build HTTP client with defaults
pub fn build_client() -> LlmResult<Client> {
    Client::builder()
        .timeout(std::time::Duration::from_secs(120))
        .connect_timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(|e| LlmError::Network(e.to_string()))
}

/// Parse API error from response
pub async fn parse_api_error(response: Response) -> String {
    let status = response.status();
    match response.text().await {
        Ok(body) => format!("HTTP {}: {}", status, body),
        Err(_) => format!("HTTP {} (no body)", status),
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  COMMON RESPONSE TYPES
// ═══════════════════════════════════════════════════════════════════════════════

/// Generic API error response
#[derive(Debug, Clone, Deserialize)]
pub struct ApiErrorResponse {
    pub error: Option<ApiError>,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ApiError {
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub message: String,
    pub code: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_client() {
        let client = build_client();
        assert!(client.is_ok());
    }
}
