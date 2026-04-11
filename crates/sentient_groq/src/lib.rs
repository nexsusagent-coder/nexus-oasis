// ═══════════════════════════════════════════════════════════════════════════════
//  SENTIENT OS - Groq LPU Integration
// ═══════════════════════════════════════════════════════════════════════════════
//  Ultra-fast inference using Groq's LPU (Language Processing Unit)
//  - 500+ tokens/second
//  - OpenAI-compatible API
//  - Models: Llama 3.3, Mixtral, Gemma
//  - Cheaper than OpenAI
// ═══════════════════════════════════════════════════════════════════════════════

pub mod client;
pub mod models;
pub mod chat;
pub mod streaming;
pub mod error;

pub use client::GroqClient;
pub use models::{GroqModel, GroqModelFamily};
pub use chat::{ChatRequest, ChatResponse, ChatMessage, MessageRole};
pub use streaming::{StreamConfig, StreamEvent};
pub use error::{GroqError, Result};

use serde::{Deserialize, Serialize};

/// Groq API configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroqConfig {
    /// API key (get from https://console.groq.com)
    pub api_key: String,
    /// Base URL (default: https://api.groq.com/openai/v1)
    pub base_url: String,
    /// Default model
    pub default_model: GroqModel,
    /// Request timeout in seconds
    pub timeout_secs: u64,
    /// Max retries
    pub max_retries: u32,
}

impl GroqConfig {
    /// Create new config with API key
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            base_url: "https://api.groq.com/openai/v1".to_string(),
            default_model: GroqModel::Llama33_70B,
            timeout_secs: 30,
            max_retries: 3,
        }
    }

    /// Set base URL (for proxies)
    pub fn with_base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into();
        self
    }

    /// Set default model
    pub fn with_model(mut self, model: GroqModel) -> Self {
        self.default_model = model;
        self
    }

    /// Load from environment variable GROQ_API_KEY
    pub fn from_env() -> Result<Self> {
        let api_key = std::env::var("GROQ_API_KEY")
            .map_err(|_| GroqError::MissingApiKey)?;
        Ok(Self::new(api_key))
    }
}

impl Default for GroqConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            base_url: "https://api.groq.com/openai/v1".to_string(),
            default_model: GroqModel::Llama33_70B,
            timeout_secs: 30,
            max_retries: 3,
        }
    }
}

// Re-export for convenience
pub mod prelude {
    pub use crate::{GroqClient, GroqConfig, GroqModel};
    pub use crate::chat::{ChatRequest, ChatMessage, MessageRole};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_groq_config_creation() {
        let config = GroqConfig::new("test-key");
        assert_eq!(config.api_key, "test-key");
        assert_eq!(config.default_model, GroqModel::Llama33_70B);
    }

    #[test]
    fn test_groq_config_with_model() {
        let config = GroqConfig::new("test-key")
            .with_model(GroqModel::Mixtral_8x7B);
        assert_eq!(config.default_model, GroqModel::Mixtral_8x7B);
    }
}
