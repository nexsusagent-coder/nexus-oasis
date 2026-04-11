//! ─── SENTIENT LLM ───
//!
//! Comprehensive LLM Model Hub - All providers, all models, unified API
//!
//! # Supported Providers
//! - **OpenAI**: GPT-4o, GPT-4 Turbo, GPT-3.5, o1, o3
//! - **Anthropic**: Claude 4, Claude 3.5, Claude 3
//! - **Google**: Gemini 2.0, Gemini 1.5, Gemma
//! - **Mistral**: Mistral Large, Medium, Small, Codestral, Mixtral
//! - **DeepSeek**: DeepSeek V3, R1, Coder (cheapest!)
//! - **xAI**: Grok 2, Grok Vision
//! - **Cohere**: Command R+, Command R, Aya
//! - **Perplexity**: Sonar (online with web search)
//! - **Groq**: LPU inference - fastest!
//! - **Together**: 100+ open source models
//! - **Fireworks**: Fast open source inference
//! - **Replicate**: Run any model
//! - **AI21**: Jamba 1.5
//! - **Ollama**: Local LLM inference (FREE!)
//!
//! # Example
//! ```rust,ignore
//! use sentient_llm::{LlmHub, ChatRequest, Message};
//!
//! #[tokio::main]
//! async fn main() {
//!     // Create hub with all providers
//!     let hub = LlmHub::from_env().unwrap();
//!
//!     // Chat with any model
//!     let response = hub.chat(ChatRequest {
//!         model: "gpt-4o".into(),
//!         messages: vec![Message::user("Hello!")],
//!         ..Default::default()
//!     }).await.unwrap();
//!
//!     println!("{}", response.choices[0].message.content.as_text().unwrap());
//! }
//! ```

pub mod error;
pub mod types;
pub mod models;
pub mod provider;
pub mod providers;
pub mod registry;

pub use error::{LlmError, LlmResult};
pub use types::{
    ChatRequest, ChatResponse, Message, Role, Content, ContentPart,
    ToolCall, FunctionCall, Tool, FunctionDefinition,
    Usage, Choice, StreamChunk, StreamChoice, Delta,
    ModelInfo, ResponseFormat, ToolChoice,
};
pub use provider::{LlmProvider, ProviderInfo};
pub use registry::{LlmHub, LlmHubBuilder, RoutingStrategy};

// Re-export providers
pub use providers::{
    OpenAIProvider, AnthropicProvider, GoogleProvider, MistralProvider,
    DeepSeekProvider, XAIProvider, CohereProvider, PerplexityProvider,
    GroqProvider, TogetherProvider, FireworksProvider, ReplicateProvider,
    AI21Provider, OllamaProvider,
};

/// Prelude for common imports
pub mod prelude {
    pub use crate::{LlmHub, ChatRequest, ChatResponse, Message, Role, LlmProvider};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_models_count() {
        let models = models::all_models();
        assert!(models.len() >= 50, "Should have 50+ models, got {}", models.len());
    }

    #[test]
    fn test_providers_count() {
        let providers = provider::ProviderInfo::all();
        assert_eq!(providers.len(), 13, "Should have 13 providers");
    }

    #[test]
    fn test_free_tier_models() {
        let free = models::free_tier();
        assert!(!free.is_empty());
        assert!(free.iter().all(|m| m.free_tier));
    }

    #[test]
    fn test_cheapest_provider() {
        let by_cost = models::by_cost();
        let cheapest = &by_cost[0];
        // DeepSeek or free models should be at the top
        assert!(cheapest.input_cost_per_1k <= 0.0001 || cheapest.provider == "Ollama");
    }

    #[test]
    fn test_reasoning_models() {
        let reasoning = models::reasoning_models();
        assert!(reasoning.iter().any(|m| m.id.contains("o1") || m.id.contains("r1") || m.id.contains("reasoning")));
    }

    #[test]
    fn test_vision_models() {
        let vision = models::vision_models();
        assert!(!vision.is_empty());
        assert!(vision.iter().all(|m| m.supports_vision));
    }
}
