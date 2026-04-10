//! GPT4All Local LLM Integration
//!
//! GPT4All is a locally running AI assistant that runs on consumer grade hardware.
//! Supports models like Llama 3, Mistral, Falcon without requiring a GPU.
//!
//! Integration via HTTP API (GPT4All server mode).

use crate::ChatMessage;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::{info, debug, warn};

// ═══════════════════════════════════════════════════════════════════════════════
//  GPT4ALL CONFIGURATION
// ═══════════════════════════════════════════════════════════════════════════════

/// GPT4All Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GPT4AllConfig {
    /// Server host (default: localhost)
    pub host: String,
    /// Server port (default: 4891 for GPT4All server)
    pub port: u16,
    /// Model name (e.g., "llama-3-8b-instruct", "mistral-7b-openorca")
    pub model: String,
    /// Temperature (0.0 - 2.0)
    pub temperature: f32,
    /// Max output tokens
    pub max_tokens: u32,
    /// Top-p sampling
    pub top_p: f32,
    /// Top-k sampling
    pub top_k: u32,
    /// Repeat penalty
    pub repeat_penalty: f32,
    /// Context length
    pub n_ctx: u32,
}

impl Default for GPT4AllConfig {
    fn default() -> Self {
        Self {
            host: "localhost".into(),
            port: 4891, // Default GPT4All server port
            model: "Llama-3-8B".into(),
            temperature: 0.7,
            max_tokens: 2048,
            top_p: 0.9,
            top_k: 40,
            repeat_penalty: 1.1,
            n_ctx: 8192,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  ERROR TYPES
// ═══════════════════════════════════════════════════════════════════════════════

/// GPT4All Error
#[derive(Debug, thiserror::Error)]
pub enum GPT4AllError {
    #[error("Connection error: {0}")]
    ConnectionError(String),

    #[error("Request error: {0}")]
    RequestError(String),

    #[error("Response error: {0}")]
    ResponseError(String),

    #[error("Parse error: {0}")]
    ParseError(String),
}

pub type GPT4AllResult<T> = Result<T, GPT4AllError>;

// ═══════════════════════════════════════════════════════════════════════════════
//  GPT4ALL CLIENT
// ═══════════════════════════════════════════════════════════════════════════════

/// GPT4All Client
pub struct GPT4AllClient {
    config: GPT4AllConfig,
    client: Client,
}

impl GPT4AllClient {
    /// Create new GPT4All client
    pub fn new(config: GPT4AllConfig) -> Self {
        Self {
            config,
            client: Client::new(),
        }
    }

    /// Create with default configuration
    pub fn default_client() -> Self {
        Self::new(GPT4AllConfig::default())
    }

    /// Get base URL
    fn base_url(&self) -> String {
        format!("http://{}:{}", self.config.host, self.config.port)
    }

    /// Check if server is running
    pub async fn health_check(&self) -> GPT4AllResult<bool> {
        let url = format!("{}/health", self.base_url());

        match self.client.get(&url).timeout(std::time::Duration::from_secs(5)).send().await {
            Ok(resp) if resp.status().is_success() => Ok(true),
            Ok(_) => Ok(false),
            Err(e) => {
                debug!("GPT4All health check failed: {}", e);
                Ok(false)
            }
        }
    }

    /// Load model (connect to server)
    pub async fn load_model(&self) -> GPT4AllResult<()> {
        let healthy = self.health_check().await?;
        if healthy {
            info!("✅ GPT4All server connected at {}", self.base_url());
            Ok(())
        } else {
            warn!("⚠️ GPT4All server not available at {}", self.base_url());
            Err(GPT4AllError::ConnectionError(format!(
                "GPT4All server not available at {}",
                self.base_url()
            )))
        }
    }

    /// Generate completion
    pub async fn generate(&self, prompt: &str) -> GPT4AllResult<String> {
        let url = format!("{}/v1/completions", self.base_url());

        let body = GPT4AllRequest {
            model: self.config.model.clone(),
            prompt: prompt.to_string(),
            max_tokens: self.config.max_tokens,
            temperature: self.config.temperature,
            top_p: self.config.top_p,
            stream: false,
        };

        let resp = self.client
            .post(&url)
            .json(&body)
            .timeout(std::time::Duration::from_secs(120))
            .send()
            .await
            .map_err(|e| GPT4AllError::RequestError(format!("Request failed: {}", e)))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            return Err(GPT4AllError::ResponseError(format!(
                "GPT4All error ({}): {}",
                status, text
            )));
        }

        let data: GPT4AllResponse = resp
            .json()
            .await
            .map_err(|e| GPT4AllError::ParseError(format!("JSON parse error: {}", e)))?;

        // Extract text from choices
        if let Some(choice) = data.choices.first() {
            Ok(choice.text.clone())
        } else {
            Ok(String::new())
        }
    }

    /// Chat completion
    pub async fn chat(&self, messages: Vec<ChatMessage>) -> GPT4AllResult<String> {
        let url = format!("{}/v1/chat/completions", self.base_url());

        let body = GPT4AllChatRequest {
            model: self.config.model.clone(),
            messages: messages.into_iter().map(|m| GPT4AllMessage {
                role: m.role,
                content: m.content,
            }).collect(),
            max_tokens: self.config.max_tokens,
            temperature: self.config.temperature,
            top_p: self.config.top_p,
            stream: false,
        };

        let resp = self.client
            .post(&url)
            .json(&body)
            .timeout(std::time::Duration::from_secs(120))
            .send()
            .await
            .map_err(|e| GPT4AllError::RequestError(format!("Request failed: {}", e)))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            return Err(GPT4AllError::ResponseError(format!(
                "GPT4All error ({}): {}",
                status, text
            )));
        }

        let data: GPT4AllChatResponse = resp
            .json()
            .await
            .map_err(|e| GPT4AllError::ParseError(format!("JSON parse error: {}", e)))?;

        Ok(data.choices
            .first()
            .map(|c| c.message.content.clone())
            .unwrap_or_default())
    }

    /// List available models
    pub async fn list_models(&self) -> GPT4AllResult<Vec<String>> {
        let url = format!("{}/v1/models", self.base_url());

        let resp = self.client
            .get(&url)
            .timeout(std::time::Duration::from_secs(10))
            .send()
            .await
            .map_err(|e| GPT4AllError::RequestError(format!("Request failed: {}", e)))?;

        if !resp.status().is_success() {
            return Ok(vec![]);
        }

        let data: GPT4AllModelsResponse = resp
            .json()
            .await
            .map_err(|e| GPT4AllError::ParseError(format!("JSON parse error: {}", e)))?;

        Ok(data.data.into_iter().map(|m| m.id).collect())
    }

    /// Check if client is available
    pub async fn is_available(&self) -> bool {
        self.health_check().await.unwrap_or(false)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  REQUEST/RESPONSE TYPES
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Serialize)]
struct GPT4AllRequest {
    model: String,
    prompt: String,
    max_tokens: u32,
    temperature: f32,
    top_p: f32,
    stream: bool,
}

#[derive(Debug, Serialize)]
struct GPT4AllChatRequest {
    model: String,
    messages: Vec<GPT4AllMessage>,
    max_tokens: u32,
    temperature: f32,
    top_p: f32,
    stream: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct GPT4AllMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct GPT4AllResponse {
    #[allow(dead_code)]
    id: String,
    choices: Vec<GPT4AllChoice>,
    #[allow(dead_code)]
    usage: Option<GPT4AllUsage>,
}

#[derive(Debug, Deserialize)]
struct GPT4AllChoice {
    text: String,
    #[allow(dead_code)]
    index: u32,
    #[allow(dead_code)]
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GPT4AllChatResponse {
    #[allow(dead_code)]
    id: String,
    choices: Vec<GPT4AllChatChoice>,
    #[allow(dead_code)]
    usage: Option<GPT4AllUsage>,
}

#[derive(Debug, Deserialize)]
struct GPT4AllChatChoice {
    #[allow(dead_code)]
    index: u32,
    message: GPT4AllMessage,
    #[allow(dead_code)]
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GPT4AllUsage {
    #[allow(dead_code)]
    prompt_tokens: u32,
    #[allow(dead_code)]
    completion_tokens: u32,
    #[allow(dead_code)]
    total_tokens: u32,
}

#[derive(Debug, Deserialize)]
struct GPT4AllModelsResponse {
    data: Vec<GPT4AllModel>,
}

#[derive(Debug, Deserialize)]
struct GPT4AllModel {
    id: String,
    #[allow(dead_code)]
    object: String,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = GPT4AllConfig::default();
        assert_eq!(config.host, "localhost");
        assert_eq!(config.port, 4891);
        assert_eq!(config.model, "Llama-3-8B");
    }

    #[test]
    fn test_client_creation() {
        let client = GPT4AllClient::default_client();
        assert_eq!(client.config.port, 4891);
    }

    #[tokio::test]
    async fn test_health_check_offline() {
        let client = GPT4AllClient::default_client();
        // Should return false when server is not running
        let healthy = client.health_check().await.unwrap_or(false);
        assert!(!healthy);
    }
}
