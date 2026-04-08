//! ═══════════════════════════════════════════════════════════════════════════════
//!  SENTIENT LOCAL - LOCAL LLM INTEGRATION
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Run LLMs locally without API dependencies:
//! - **Gemma 4**: DEFAULT KERNEL MODEL (Google DeepMind)
//! - **Ollama**: Simple local LLM runner
//! - **GPT4All**: CPU-optimized inference
//! - **Text Generation WebUI**: Full-featured UI
//!
//! ═══════════════════════════════════════════════════════════════════════════════
//!  GEMMA 4 - NATIVE KERNEL MODEL
//!  ─────────────────────────────────────────────────────────────────────────────
//!  • 31B parameters, multimodal (text + vision)
//!  • 256K context length
//!  • Native thinking mode + function calling
//!  • Apache 2.0 license - FULLY FREE
//!  • Zero-copy memory integration
//!  ═══════════════════════════════════════════════════════════════════════════════

use serde::{Deserialize, Serialize};
use tracing::info;
use std::sync::Arc;
use tokio::sync::RwLock;

pub mod ollama;
pub mod gpt4all;
pub mod gemma4;

// Re-exports
pub use gemma4::{Gemma4Engine, Gemma4Config, Gemma4Response, ThinkingMode};

// ═══════════════════════════════════════════════════════════════════════════════
//  CONSTANTS - GEMMA 4 AS DEFAULT
// ═══════════════════════════════════════════════════════════════════════════════

/// Default Gemma 4 model ID
pub const GEMMA4_DEFAULT_MODEL: &str = "gemma4:31b";

/// Gemma 4 model family
pub const GEMMA4_MODELS: &[&str] = &[
    "gemma4:31b",      // Full model - 31B parameters
    "gemma4:26b-moe",  // Mixture of Experts - faster
    "gemma4:e4b",      // Edge - laptop optimized
    "gemma4:e2b",      // Mobile - ultra light
];

/// Local LLM Provider
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LocalProvider {
    /// Gemma 4 - SENTIENT KERNEL (Default)
    Gemma4,
    Ollama,
    GPT4All,
    TextGenWebUI,
}

impl Default for LocalProvider {
    fn default() -> Self {
        LocalProvider::Gemma4
    }
}

/// Local Model Info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalModel {
    pub name: String,
    pub provider: LocalProvider,
    pub size_gb: f32,
    pub parameters: String,
    pub context_length: usize,
}

/// Local LLM Configuration - Gemma 4 Default
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalConfig {
    pub provider: LocalProvider,
    pub model: String,
    pub host: String,
    pub port: u16,
    pub temperature: f32,
    pub max_tokens: usize,
    /// Zero-copy mode for Memory Cube integration
    pub zero_copy: bool,
    /// Thinking mode (Gemma 4 native)
    pub thinking_mode: bool,
}

impl Default for LocalConfig {
    fn default() -> Self {
        Self {
            provider: LocalProvider::Gemma4,
            model: GEMMA4_DEFAULT_MODEL.to_string(),
            host: "localhost".to_string(),
            port: 11434,
            temperature: 0.7,
            max_tokens: 16384,  // Gemma 4 supports up to 16K output
            zero_copy: true,
            thinking_mode: true,
        }
    }
}

/// Chat Message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

/// Available Local Models - Gemma 4 First
pub fn available_models() -> Vec<LocalModelInfo> {
    vec![
        // ═══════════════════════════════════════════════════════════════
        // GEMMA 4 - SENTIENT OS KERNEL (PRIORITY)
        // ═══════════════════════════════════════════════════════════════
        LocalModelInfo {
            provider: LocalProvider::Gemma4,
            name: "Gemma 4 (KERNEL)".to_string(),
            source: "native://gemma4".to_string(),
            status: "KERNEL".to_string(),
            models: vec![
                "gemma4:31b".to_string(),
                "gemma4:26b-moe".to_string(),
                "gemma4:e4b".to_string(),
                "gemma4:e2b".to_string(),
            ],
        },
        LocalModelInfo {
            provider: LocalProvider::Ollama,
            name: "Ollama".to_string(),
            source: "integrations/framework/ollama".to_string(),
            status: "READY".to_string(),
            models: vec!["llama3.2".to_string(), "mistral".to_string(), "qwen2.5".to_string(), "deepseek-r1".to_string()],
        },
        LocalModelInfo {
            provider: LocalProvider::GPT4All,
            name: "GPT4All".to_string(),
            source: "integrations/framework/gpt4all".to_string(),
            status: "READY".to_string(),
            models: vec!["Llama 3".to_string(), "Mistral".to_string(), "Phi-3".to_string()],
        },
        LocalModelInfo {
            provider: LocalProvider::TextGenWebUI,
            name: "Text Generation WebUI".to_string(),
            source: "integrations/framework/text-generation-webui".to_string(),
            status: "READY".to_string(),
            models: vec!["Any HuggingFace model".to_string()],
        },
    ]
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalModelInfo {
    pub provider: LocalProvider,
    pub name: String,
    pub source: String,
    pub status: String,
    pub models: Vec<String>,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  LOCAL ENGINE - UNIFIED INTERFACE
// ═══════════════════════════════════════════════════════════════════════════════

/// Unified local LLM engine
pub struct LocalEngine {
    config: LocalConfig,
    gemma4: Arc<RwLock<Option<Gemma4Engine>>>,
}

impl LocalEngine {
    /// Create new local engine with Gemma 4 as default
    pub fn new() -> Self {
        Self::with_config(LocalConfig::default())
    }
    
    /// Create with custom config
    pub fn with_config(config: LocalConfig) -> Self {
        info!("🔧  LOCAL ENGINE: Initializing with {:?}", config.provider);
        Self {
            config,
            gemma4: Arc::new(RwLock::new(None)),
        }
    }
    
    /// Initialize Gemma 4 kernel
    pub async fn init_gemma4(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut engine = self.gemma4.write().await;
        *engine = Some(Gemma4Engine::new(Gemma4Config::default())?);
        info!("✅  GEMMA 4: Kernel initialized");
        Ok(())
    }
    
    /// Generate with Gemma 4 (zero-copy)
    pub async fn generate(&self, prompt: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let engine = self.gemma4.read().await;
        if let Some(ref gemma) = *engine {
            let response = gemma.generate(prompt).await?;
            Ok(response.content)
        } else {
            // Fallback to Ollama
            let client = ollama::OllamaClient::new(&self.config.host, self.config.port);
            client.generate(&self.config.model, prompt).await
        }
    }
    
    /// Chat with Gemma 4
    pub async fn chat(&self, messages: Vec<ChatMessage>) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let engine = self.gemma4.read().await;
        if let Some(ref gemma) = *engine {
            let response = gemma.chat(messages).await?;
            Ok(response.content)
        } else {
            let client = ollama::OllamaClient::new(&self.config.host, self.config.port);
            client.chat(&self.config.model, messages).await
        }
    }
}

impl Default for LocalEngine {
    fn default() -> Self {
        Self::new()
    }
}
