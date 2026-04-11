//! ─── LLM Provider Trait ───
//!
//! Core trait for LLM providers

use async_trait::async_trait;
use futures::Stream;
use std::pin::Pin;

use crate::types::{ChatRequest, ChatResponse, ModelInfo, StreamChunk};
use crate::error::LlmResult;

// ═══════════════════════════════════════════════════════════════════════════════
//  LLM PROVIDER TRAIT
// ═══════════════════════════════════════════════════════════════════════════════

/// LLM Provider trait
#[async_trait]
pub trait LlmProvider: Send + Sync {
    /// Get provider name
    fn name(&self) -> &str;

    /// Get provider ID
    fn id(&self) -> &str;

    /// Get available models
    fn models(&self) -> Vec<ModelInfo>;

    /// Check if API key is configured
    fn is_configured(&self) -> bool;

    /// Chat completion
    async fn chat(&self, request: ChatRequest) -> LlmResult<ChatResponse>;

    /// Streaming chat completion
    async fn chat_stream(
        &self,
        request: ChatRequest,
    ) -> LlmResult<Pin<Box<dyn Stream<Item = LlmResult<StreamChunk>> + Send>>>;

    /// Count tokens in a text
    fn count_tokens(&self, text: &str, model: &str) -> LlmResult<usize>;

    /// Get model info
    fn get_model(&self, model_id: &str) -> Option<ModelInfo> {
        self.models().into_iter().find(|m| m.id == model_id)
    }

    /// Validate request
    fn validate_request(&self, request: &ChatRequest) -> LlmResult<()> {
        let model = self.get_model(&request.model)
            .ok_or_else(|| crate::error::LlmError::ModelNotFound(request.model.clone()))?;

        // Check if messages fit in context
        let total_tokens: usize = request.messages.iter()
            .filter_map(|m| m.content.as_text())
            .map(|t| t.len() / 4) // Rough estimate
            .sum();

        if total_tokens > model.context_window as usize {
            return Err(crate::error::LlmError::ContextLengthExceeded {
                max: model.context_window as usize,
                provided: total_tokens,
            });
        }

        Ok(())
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  PROVIDER INFO
// ═══════════════════════════════════════════════════════════════════════════════

/// Provider information for display
#[derive(Debug, Clone)]
pub struct ProviderInfo {
    pub name: String,
    pub id: String,
    pub description: String,
    pub base_url: String,
    pub docs_url: String,
    pub pricing_url: String,
    pub free_tier: bool,
    pub free_tier_limits: Option<String>,
    pub model_count: usize,
    pub supports_streaming: bool,
    pub supports_tools: bool,
    pub supports_vision: bool,
    pub api_key_env: String,
}

impl ProviderInfo {
    /// OpenAI provider info
    pub fn openai() -> Self {
        Self {
            name: "OpenAI".into(),
            id: "openai".into(),
            description: "GPT-4, GPT-4o, GPT-3.5, o1 reasoning models".into(),
            base_url: "https://api.openai.com/v1".into(),
            docs_url: "https://platform.openai.com/docs".into(),
            pricing_url: "https://openai.com/pricing".into(),
            free_tier: true,
            free_tier_limits: Some("$5 free credits for new users".into()),
            model_count: 9,
            supports_streaming: true,
            supports_tools: true,
            supports_vision: true,
            api_key_env: "OPENAI_API_KEY".into(),
        }
    }

    /// Anthropic provider info
    pub fn anthropic() -> Self {
        Self {
            name: "Anthropic".into(),
            id: "anthropic".into(),
            description: "Claude 4, Claude 3.5, Claude 3 models".into(),
            base_url: "https://api.anthropic.com/v1".into(),
            docs_url: "https://docs.anthropic.com".into(),
            pricing_url: "https://www.anthropic.com/pricing".into(),
            free_tier: false,
            free_tier_limits: None,
            model_count: 5,
            supports_streaming: true,
            supports_tools: true,
            supports_vision: true,
            api_key_env: "ANTHROPIC_API_KEY".into(),
        }
    }

    /// Google provider info
    pub fn google() -> Self {
        Self {
            name: "Google".into(),
            id: "google".into(),
            description: "Gemini 2.0, Gemini 1.5, Gemma models".into(),
            base_url: "https://generativelanguage.googleapis.com/v1beta".into(),
            docs_url: "https://ai.google.dev/docs".into(),
            pricing_url: "https://ai.google.dev/pricing".into(),
            free_tier: true,
            free_tier_limits: Some("Free tier: 15 RPM, 1M tokens/day".into()),
            model_count: 6,
            supports_streaming: true,
            supports_tools: true,
            supports_vision: true,
            api_key_env: "GOOGLE_API_KEY".into(),
        }
    }

    /// Mistral provider info
    pub fn mistral() -> Self {
        Self {
            name: "Mistral".into(),
            id: "mistral".into(),
            description: "Mistral Large, Medium, Small, Codestral".into(),
            base_url: "https://api.mistral.ai/v1".into(),
            docs_url: "https://docs.mistral.ai".into(),
            pricing_url: "https://mistral.ai/pricing".into(),
            free_tier: true,
            free_tier_limits: Some("Free tier available".into()),
            model_count: 6,
            supports_streaming: true,
            supports_tools: true,
            supports_vision: true,
            api_key_env: "MISTRAL_API_KEY".into(),
        }
    }

    /// DeepSeek provider info
    pub fn deepseek() -> Self {
        Self {
            name: "DeepSeek".into(),
            id: "deepseek".into(),
            description: "DeepSeek V3, R1 (reasoning), Coder - cheapest quality models".into(),
            base_url: "https://api.deepseek.com/v1".into(),
            docs_url: "https://platform.deepseek.com/docs".into(),
            pricing_url: "https://platform.deepseek.com/pricing".into(),
            free_tier: true,
            free_tier_limits: Some("Very cheap pricing".into()),
            model_count: 3,
            supports_streaming: true,
            supports_tools: true,
            supports_vision: false,
            api_key_env: "DEEPSEEK_API_KEY".into(),
        }
    }

    /// xAI provider info
    pub fn xai() -> Self {
        Self {
            name: "xAI".into(),
            id: "xai".into(),
            description: "Grok 2, Grok Vision models".into(),
            base_url: "https://api.x.ai/v1".into(),
            docs_url: "https://docs.x.ai/docs".into(),
            pricing_url: "https://x.ai/pricing".into(),
            free_tier: false,
            free_tier_limits: None,
            model_count: 3,
            supports_streaming: true,
            supports_tools: true,
            supports_vision: true,
            api_key_env: "XAI_API_KEY".into(),
        }
    }

    /// Cohere provider info
    pub fn cohere() -> Self {
        Self {
            name: "Cohere".into(),
            id: "cohere".into(),
            description: "Command R+, Command R, Aya multilingual".into(),
            base_url: "https://api.cohere.ai/v1".into(),
            docs_url: "https://docs.cohere.com".into(),
            pricing_url: "https://cohere.com/pricing".into(),
            free_tier: true,
            free_tier_limits: Some("Free tier: 1000 calls/month".into()),
            model_count: 5,
            supports_streaming: true,
            supports_tools: true,
            supports_vision: false,
            api_key_env: "COHERE_API_KEY".into(),
        }
    }

    /// Perplexity provider info
    pub fn perplexity() -> Self {
        Self {
            name: "Perplexity".into(),
            id: "perplexity".into(),
            description: "Sonar - online LLMs with web search built-in".into(),
            base_url: "https://api.perplexity.ai".into(),
            docs_url: "https://docs.perplexity.ai".into(),
            pricing_url: "https://perplexity.ai/pricing".into(),
            free_tier: false,
            free_tier_limits: None,
            model_count: 3,
            supports_streaming: true,
            supports_tools: false,
            supports_vision: false,
            api_key_env: "PERPLEXITY_API_KEY".into(),
        }
    }

    /// Groq provider info
    pub fn groq() -> Self {
        Self {
            name: "Groq".into(),
            id: "groq".into(),
            description: "LPU inference - fastest LLM inference available".into(),
            base_url: "https://api.groq.com/openai/v1".into(),
            docs_url: "https://console.groq.com/docs".into(),
            pricing_url: "https://groq.com/pricing".into(),
            free_tier: true,
            free_tier_limits: Some("Free tier with rate limits".into()),
            model_count: 5,
            supports_streaming: true,
            supports_tools: true,
            supports_vision: false,
            api_key_env: "GROQ_API_KEY".into(),
        }
    }

    /// Together AI provider info
    pub fn together() -> Self {
        Self {
            name: "Together".into(),
            id: "together".into(),
            description: "100+ open source models, Llama, Mistral, etc.".into(),
            base_url: "https://api.together.xyz/v1".into(),
            docs_url: "https://docs.together.ai".into(),
            pricing_url: "https://together.ai/pricing".into(),
            free_tier: true,
            free_tier_limits: Some("$1 free credits".into()),
            model_count: 3,
            supports_streaming: true,
            supports_tools: true,
            supports_vision: false,
            api_key_env: "TOGETHER_API_KEY".into(),
        }
    }

    /// Fireworks AI provider info
    pub fn fireworks() -> Self {
        Self {
            name: "Fireworks".into(),
            id: "fireworks".into(),
            description: "Fast inference for open source models".into(),
            base_url: "https://api.fireworks.ai/inference/v1".into(),
            docs_url: "https://docs.fireworks.ai".into(),
            pricing_url: "https://fireworks.ai/pricing".into(),
            free_tier: true,
            free_tier_limits: Some("Free tier available".into()),
            model_count: 2,
            supports_streaming: true,
            supports_tools: true,
            supports_vision: false,
            api_key_env: "FIREWORKS_API_KEY".into(),
        }
    }

    /// Replicate provider info
    pub fn replicate() -> Self {
        Self {
            name: "Replicate".into(),
            id: "replicate".into(),
            description: "Run any open source model via API".into(),
            base_url: "https://api.replicate.com/v1".into(),
            docs_url: "https://replicate.com/docs".into(),
            pricing_url: "https://replicate.com/pricing".into(),
            free_tier: false,
            free_tier_limits: None,
            model_count: 2,
            supports_streaming: true,
            supports_tools: true,
            supports_vision: false,
            api_key_env: "REPLICATE_API_TOKEN".into(),
        }
    }

    /// AI21 provider info
    pub fn ai21() -> Self {
        Self {
            name: "AI21".into(),
            id: "ai21".into(),
            description: "Jamba 1.5 - SSM-Transformer hybrid".into(),
            base_url: "https://api.ai21.com/v1".into(),
            docs_url: "https://docs.ai21.com".into(),
            pricing_url: "https://ai21.com/pricing".into(),
            free_tier: true,
            free_tier_limits: Some("Free tier available".into()),
            model_count: 2,
            supports_streaming: true,
            supports_tools: true,
            supports_vision: false,
            api_key_env: "AI21_API_KEY".into(),
        }
    }

    /// Get all providers
    pub fn all() -> Vec<ProviderInfo> {
        vec![
            Self::openai(),
            Self::anthropic(),
            Self::google(),
            Self::mistral(),
            Self::deepseek(),
            Self::xai(),
            Self::cohere(),
            Self::perplexity(),
            Self::groq(),
            Self::together(),
            Self::fireworks(),
            Self::replicate(),
            Self::ai21(),
        ]
    }

    /// Get providers with free tier
    pub fn free_tier() -> Vec<ProviderInfo> {
        Self::all().into_iter().filter(|p| p.free_tier).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_info_count() {
        let providers = ProviderInfo::all();
        assert_eq!(providers.len(), 13);
    }

    #[test]
    fn test_free_tier_providers() {
        let free = ProviderInfo::free_tier();
        assert!(!free.is_empty());
        assert!(free.iter().all(|p| p.free_tier));
    }
}
