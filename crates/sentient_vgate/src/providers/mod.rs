//! ─── SENTIENT V-GATE PROVIDERS (LLM SAĞLAYICILARI) ───
//!
//! Farklı LLM sağlayıcıları ile iletişimi yöneten modül.
//! OpenRouter, OpenAI, Anthropic, Groq ve yerel modeller desteklenir.
//!
//! ════════════════════════════════════════════════════════════════
//!  VARSAYILAN MODEL: google/gemma-4-31b-it:free
//!  - 31B parametre, multimodal (text + image)
//!  - 256K context length
//!  - Native thinking mode + function calling
//!  - Apache 2.0 lisansı
//! ════════════════════════════════════════════════════════════════

pub mod models;      // Model registry (Gemma 4 öncelikli)
pub mod openrouter;
pub mod openai;
pub mod anthropic;
pub mod base;


pub use base::{LlmProvider, LlmRequest, LlmResponse, ChatMessage};
pub use openrouter::OpenRouterProvider;
pub use openai::OpenAIProvider;
pub use anthropic::AnthropicProvider;

// Model registry exports
pub use models::{
    ModelDefinition, DEFAULT_MODEL, GEMMA4_MODELS, 
    find_model, get_default_model, all_models, free_models,
    thinking_capable_models, vision_capable_models, audio_capable_models,
};

/// ─── Sağlayıcı Factory ───

pub struct ProviderFactory;

impl ProviderFactory {
    /// Sağlayıcı oluştur
    pub fn create(
        provider: crate::auth::Provider,
        base_url: String,
        api_key: String,
    ) -> Box<dyn LlmProvider + Send + Sync> {
        match provider {
            crate::auth::Provider::OpenRouter => {
                Box::new(OpenRouterProvider::new(base_url, api_key))
            }
            crate::auth::Provider::OpenAI => {
                Box::new(OpenAIProvider::new(base_url, api_key))
            }
            crate::auth::Provider::Anthropic => {
                Box::new(AnthropicProvider::new(base_url, api_key))
            }
            crate::auth::Provider::Groq => {
                // Groq, OpenAI uyumlu API kullanır
                Box::new(OpenAIProvider::new(base_url, api_key))
            }
            crate::auth::Provider::Local => {
                // Yerel modeller için OpenAI uyumlu API
                Box::new(OpenAIProvider::new(base_url, api_key))
            }
        }
    }
}
