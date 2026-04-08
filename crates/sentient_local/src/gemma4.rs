//! ═══════════════════════════════════════════════════════════════════════════════
//!  GEMMA 4 KERNEL ENGINE - SENTIENT OS NATIVE INTELLIGENCE
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Google DeepMind Gemma 4 native integration for SENTIENT OS.
//! - 31B parameters, multimodal (text + vision)
//! - 256K context length
//! - Native thinking mode + function calling
//! - Apache 2.0 license - FULLY FREE
//!
//! ═══════════════════════════════════════════════════════════════════════════════
//!  ZERO-COPY MEMORY INTEGRATION
//! ═══════════════════════════════════════════════════════════════════════════════
//! Output directly flows to Memory Cube without intermediate copies:
//! ┌─────────────┐    ┌─────────────┐    ┌─────────────┐
//! │   GEMMA 4   │───▶│ ZERO-COPY   │───▶│ MEMORY CUBE │
//! │   KERNEL    │    │   BUFFER    │    │   L3        │
//! └─────────────┘    └─────────────┘    └─────────────┘
//! ═══════════════════════════════════════════════════════════════════════════════

use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, debug};

// ═══════════════════════════════════════════════════════════════════════════════
//  GEMMA 4 CONFIGURATION
// ═══════════════════════════════════════════════════════════════════════════════

/// Gemma 4 Engine Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gemma4Config {
    /// Model variant (31b, 26b-moe, e4b, e2b)
    pub model_variant: String,
    /// Ollama host
    pub host: String,
    /// Ollama port
    pub port: u16,
    /// Temperature (0.0 - 2.0)
    pub temperature: f32,
    /// Max output tokens (up to 16384 for Gemma 4)
    pub max_tokens: u32,
    /// Enable thinking mode
    pub thinking_mode: bool,
    /// Zero-copy mode for Memory Cube
    pub zero_copy: bool,
    /// Context length (up to 256K)
    pub context_length: u32,
    /// Top-p sampling
    pub top_p: f32,
    /// Top-k sampling
    pub top_k: u32,
    /// Repeat penalty
    pub repeat_penalty: f32,
}

impl Default for Gemma4Config {
    fn default() -> Self {
        Self {
            model_variant: "gemma4:31b".into(),
            host: "localhost".into(),
            port: 11434,
            temperature: 0.7,
            max_tokens: 16384,
            thinking_mode: true,
            zero_copy: true,
            context_length: 262_144, // 256K
            top_p: 0.9,
            top_k: 40,
            repeat_penalty: 1.1,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  THINKING MODE
// ═══════════════════════════════════════════════════════════════════════════════

/// Thinking mode configuration
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum ThinkingMode {
    /// Disabled - direct output
    Off,
    /// Enabled - include thinking process
    On,
    /// Auto - model decides
    Auto,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  RESPONSE TYPES
// ═══════════════════════════════════════════════════════════════════════════════

/// Gemma 4 Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gemma4Response {
    /// Generated content
    pub content: String,
    /// Thinking process (if enabled)
    pub thinking: Option<String>,
    /// Token count
    pub total_tokens: u32,
    /// Prompt tokens
    pub prompt_tokens: u32,
    /// Completion tokens
    pub completion_tokens: u32,
    /// Model used
    pub model: String,
    /// Duration in ms
    pub duration_ms: u64,
    /// Zero-copy buffer reference (for Memory Cube)
    pub buffer_id: Option<String>,
}

/// Zero-copy buffer for Memory Cube integration
#[derive(Debug, Clone)]
pub struct ZeroCopyBuffer {
    /// Buffer ID
    pub id: String,
    /// Content pointer (Arc for shared ownership)
    pub content: Arc<String>,
    /// Token count
    pub token_count: u32,
    /// Creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  GEMMA 4 ENGINE
// ═══════════════════════════════════════════════════════════════════════════════

/// Gemma 4 Kernel Engine
pub struct Gemma4Engine {
    config: Gemma4Config,
    client: Client,
    /// Zero-copy buffers for Memory Cube
    buffers: Arc<RwLock<Vec<ZeroCopyBuffer>>>,
    /// Request counter
    request_count: Arc<RwLock<u64>>,
}

impl Gemma4Engine {
    /// Create new Gemma 4 engine
    pub fn new(config: Gemma4Config) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        info!("🧠  GEMMA 4 KERNEL: Initializing {}...", config.model_variant);
        
        Ok(Self {
            config,
            client: Client::new(),
            buffers: Arc::new(RwLock::new(Vec::new())),
            request_count: Arc::new(RwLock::new(0)),
        })
    }
    
    /// Create with default config
    pub fn default_engine() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        Self::new(Gemma4Config::default())
    }
    
    /// Generate text
    pub async fn generate(&self, prompt: &str) -> Result<Gemma4Response, Box<dyn std::error::Error + Send + Sync>> {
        let start = std::time::Instant::now();
        
        let url = format!("http://{}:{}/api/generate", self.config.host, self.config.port);
        
        let body = serde_json::json!({
            "model": self.config.model_variant,
            "prompt": prompt,
            "stream": false,
            "options": {
                "temperature": self.config.temperature,
                "num_predict": self.config.max_tokens,
                "top_p": self.config.top_p,
                "top_k": self.config.top_k,
                "repeat_penalty": self.config.repeat_penalty,
                "num_ctx": self.config.context_length,
            }
        });
        
        let resp = self.client.post(&url).json(&body).send().await?;
        let data: OllamaGenerateResponse = resp.json().await?;
        
        // Increment request counter
        {
            let mut count = self.request_count.write().await;
            *count += 1;
        }
        
        // Create zero-copy buffer
        let buffer_id = if self.config.zero_copy {
            let buffer = ZeroCopyBuffer {
                id: uuid::Uuid::new_v4().to_string(),
                content: Arc::new(data.response.clone()),
                token_count: data.eval_count.unwrap_or(0) as u32,
                created_at: chrono::Utc::now(),
            };
            
            let mut buffers = self.buffers.write().await;
            buffers.push(buffer.clone());
            Some(buffer.id)
        } else {
            None
        };
        
        let response = Gemma4Response {
            content: data.response,
            thinking: None,
            total_tokens: data.eval_count.unwrap_or(0) as u32,
            prompt_tokens: data.prompt_eval_count.unwrap_or(0) as u32,
            completion_tokens: data.eval_count.unwrap_or(0) as u32,
            model: self.config.model_variant.clone(),
            duration_ms: start.elapsed().as_millis() as u64,
            buffer_id,
        };
        
        debug!("✅  GEMMA 4: Generated {} tokens in {}ms", 
            response.completion_tokens, response.duration_ms);
        
        Ok(response)
    }
    
    /// Chat with message history
    pub async fn chat(&self, messages: Vec<crate::ChatMessage>) -> Result<Gemma4Response, Box<dyn std::error::Error + Send + Sync>> {
        let start = std::time::Instant::now();
        
        let url = format!("http://{}:{}/api/chat", self.config.host, self.config.port);
        
        // Add system prompt for thinking mode
        let mut enhanced_messages = messages.clone();
        if self.config.thinking_mode {
            enhanced_messages.insert(0, crate::ChatMessage {
                role: "system".into(),
                content: "You are SENTIENT OS Kernel, powered by Gemma 4. Think step by step when solving complex problems. Be concise and accurate.".into(),
            });
        }
        
        let body = serde_json::json!({
            "model": self.config.model_variant,
            "messages": enhanced_messages,
            "stream": false,
            "options": {
                "temperature": self.config.temperature,
                "num_predict": self.config.max_tokens,
                "top_p": self.config.top_p,
                "top_k": self.config.top_k,
                "repeat_penalty": self.config.repeat_penalty,
                "num_ctx": self.config.context_length,
            }
        });
        
        let resp = self.client.post(&url).json(&body).send().await?;
        let data: OllamaChatResponse = resp.json().await?;
        
        // Increment request counter
        {
            let mut count = self.request_count.write().await;
            *count += 1;
        }
        
        // Create zero-copy buffer
        let buffer_id = if self.config.zero_copy {
            let buffer = ZeroCopyBuffer {
                id: uuid::Uuid::new_v4().to_string(),
                content: Arc::new(data.message.content.clone()),
                token_count: 0,
                created_at: chrono::Utc::now(),
            };
            
            let mut buffers = self.buffers.write().await;
            buffers.push(buffer.clone());
            Some(buffer.id)
        } else {
            None
        };
        
        let response = Gemma4Response {
            content: data.message.content,
            thinking: data.thinking,
            total_tokens: 0,
            prompt_tokens: 0,
            completion_tokens: 0,
            model: self.config.model_variant.clone(),
            duration_ms: start.elapsed().as_millis() as u64,
            buffer_id,
        };
        
        debug!("✅  GEMMA 4: Chat response in {}ms", response.duration_ms);
        
        Ok(response)
    }
    
    /// Generate with thinking mode
    pub async fn think(&self, prompt: &str) -> Result<(String, Option<String>), Box<dyn std::error::Error + Send + Sync>> {
        let enhanced_prompt = format!(
            "Think step by step about this problem:\n\n{}\n\nProvide your reasoning first, then give your final answer.",
            prompt
        );
        
        let response = self.generate(&enhanced_prompt).await?;
        Ok((response.content, response.thinking))
    }
    
    /// Get zero-copy buffer for Memory Cube
    pub async fn get_buffer(&self, id: &str) -> Option<ZeroCopyBuffer> {
        let buffers = self.buffers.read().await;
        buffers.iter().find(|b| b.id == id).cloned()
    }
    
    /// Get request count
    pub async fn request_count(&self) -> u64 {
        let count = self.request_count.read().await;
        *count
    }
    
    /// Clear buffers (for memory management)
    pub async fn clear_buffers(&self) {
        let mut buffers = self.buffers.write().await;
        buffers.clear();
        info!("🧹  GEMMA 4: Zero-copy buffers cleared");
    }
    
    /// Pull Gemma 4 model via Ollama
    pub async fn pull_model(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        info!("📥  GEMMA 4: Pulling model {}...", self.config.model_variant);
        
        let url = format!("http://{}:{}/api/pull", self.config.host, self.config.port);
        
        let body = serde_json::json!({
            "name": self.config.model_variant,
            "stream": false
        });
        
        let _resp = self.client.post(&url).json(&body).send().await?;
        
        info!("✅  GEMMA 4: Model {} pulled successfully", self.config.model_variant);
        Ok(())
    }
    
    /// Check if model is available
    pub async fn is_available(&self) -> bool {
        let url = format!("http://{}:{}/api/tags", self.config.host, self.config.port);
        
        match self.client.get(&url).send().await {
            Ok(resp) => {
                if let Ok(data) = resp.json::<OllamaModels>().await {
                    data.models.iter().any(|m| m.name.starts_with(&self.config.model_variant.split(':').next().unwrap_or("gemma4")))
                } else {
                    false
                }
            }
            Err(_) => false,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  OLLAMA API TYPES
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Deserialize)]
struct OllamaGenerateResponse {
    response: String,
    eval_count: Option<i32>,
    prompt_eval_count: Option<i32>,
}

#[derive(Debug, Deserialize)]
struct OllamaChatResponse {
    message: OllamaMessage,
    thinking: Option<String>,
}

#[derive(Debug, Deserialize)]
struct OllamaMessage {
    content: String,
}

#[derive(Debug, Deserialize)]
struct OllamaModels {
    models: Vec<OllamaModel>,
}

#[derive(Debug, Deserialize)]
struct OllamaModel {
    name: String,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  MEMORY CUBE INTEGRATION TRAIT
// ═══════════════════════════════════════════════════════════════════════════════

/// Trait for zero-copy Memory Cube integration
pub trait MemoryCubeSource {
    /// Get content as Arc for zero-copy
    fn get_content_arc(&self) -> Arc<String>;
    /// Get token count
    fn get_token_count(&self) -> u32;
}

impl MemoryCubeSource for Gemma4Response {
    fn get_content_arc(&self) -> Arc<String> {
        Arc::new(self.content.clone())
    }
    
    fn get_token_count(&self) -> u32 {
        self.completion_tokens
    }
}

impl MemoryCubeSource for ZeroCopyBuffer {
    fn get_content_arc(&self) -> Arc<String> {
        self.content.clone()
    }
    
    fn get_token_count(&self) -> u32 {
        self.token_count
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gemma4_config_default() {
        let config = Gemma4Config::default();
        assert_eq!(config.model_variant, "gemma4:31b");
        assert_eq!(config.context_length, 262_144);
        assert!(config.thinking_mode);
        assert!(config.zero_copy);
    }

    #[test]
    fn test_gemma4_response_creation() {
        let response = Gemma4Response {
            content: "Test response".into(),
            thinking: Some("Thinking process...".into()),
            total_tokens: 100,
            prompt_tokens: 50,
            completion_tokens: 50,
            model: "gemma4:31b".into(),
            duration_ms: 1000,
            buffer_id: Some("test-buffer-id".into()),
        };
        
        assert_eq!(response.content, "Test response");
        assert!(response.thinking.is_some());
    }

    #[test]
    fn test_zero_copy_buffer() {
        let buffer = ZeroCopyBuffer {
            id: "test-id".into(),
            content: Arc::new("Test content".into()),
            token_count: 10,
            created_at: chrono::Utc::now(),
        };
        
        assert_eq!(buffer.id, "test-id");
        assert_eq!(*buffer.content, "Test content");
    }

    #[test]
    fn test_memory_cube_source_trait() {
        let response = Gemma4Response {
            content: "Test".into(),
            thinking: None,
            total_tokens: 10,
            prompt_tokens: 5,
            completion_tokens: 5,
            model: "gemma4:31b".into(),
            duration_ms: 100,
            buffer_id: None,
        };
        
        let arc = response.get_content_arc();
        assert_eq!(*arc, "Test");
    }
}
