//! LLM Settings - AI model ayarları

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmSettings {
    /// Provider (openai, anthropic, google, ollama)
    pub provider: String,
    
    /// Model adı
    pub model: String,
    
    /// Temperature (0.0 - 2.0)
    pub temperature: f32,
    
    /// Max tokens
    pub max_tokens: usize,
    
    /// Top P
    pub top_p: f32,
    
    /// Frequency penalty
    pub frequency_penalty: f32,
    
    /// Presence penalty
    pub presence_penalty: f32,
    
    /// System prompt
    pub system_prompt: String,
    
    /// Streaming
    pub streaming: bool,
    
    /// Timeout (saniye)
    pub timeout: u64,
    
    /// Retry count
    pub retry_count: u8,
    
    /// Provider'a özel ayarlar
    pub provider_config: HashMap<String, String>,
}

impl Default for LlmSettings {
    fn default() -> Self {
        Self {
            provider: "ollama".to_string(),
            model: "qwen2.5-coder:7b".to_string(),
            temperature: 0.7,
            max_tokens: 4096,
            top_p: 1.0,
            frequency_penalty: 0.0,
            presence_penalty: 0.0,
            system_prompt: "Sen SENTIENT NEXUS OS asistanısın. Yardımcı, bilgili ve profesyonelsin.".to_string(),
            streaming: true,
            timeout: 120,
            retry_count: 3,
            provider_config: HashMap::new(),
        }
    }
}

impl LlmSettings {
    pub fn openai_default() -> Self {
        Self {
            provider: "openai".to_string(),
            model: "gpt-4-turbo".to_string(),
            ..Default::default()
        }
    }
    
    pub fn anthropic_default() -> Self {
        Self {
            provider: "anthropic".to_string(),
            model: "claude-3-opus-20240229".to_string(),
            ..Default::default()
        }
    }
    
    pub fn ollama_default() -> Self {
        Self {
            provider: "ollama".to_string(),
            model: "qwen2.5-coder:7b".to_string(),
            ..Default::default()
        }
    }
}
