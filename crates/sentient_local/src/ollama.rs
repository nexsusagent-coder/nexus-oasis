//! Ollama Local LLM Integration
//! 
//! Source: integrations/framework/ollama

use crate::{LocalProvider, LocalConfig, ChatMessage};
use reqwest::Client;
use serde::{Deserialize, Serialize};

pub struct OllamaClient {
    host: String,
    port: u16,
    client: Client,
}

impl OllamaClient {
    pub fn new(host: &str, port: u16) -> Self {
        Self {
            host: host.to_string(),
            port,
            client: Client::new(),
        }
    }
    
    pub async fn list_models(&self) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
        let url = format!("http://{}:{}/api/tags", self.host, self.port);
        let resp = self.client.get(&url).send().await?;
        let data: OllamaModels = resp.json().await?;
        Ok(data.models.iter().map(|m| m.name.clone()).collect())
    }
    
    pub async fn generate(&self, model: &str, prompt: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let url = format!("http://{}:{}/api/generate", self.host, self.port);
        let body = serde_json::json!({
            "model": model,
            "prompt": prompt,
            "stream": false
        });
        let resp = self.client.post(&url).json(&body).send().await?;
        let data: OllamaResponse = resp.json().await?;
        Ok(data.response)
    }
    
    pub async fn chat(&self, model: &str, messages: Vec<ChatMessage>) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let url = format!("http://{}:{}/api/chat", self.host, self.port);
        let body = serde_json::json!({
            "model": model,
            "messages": messages,
            "stream": false
        });
        let resp = self.client.post(&url).json(&body).send().await?;
        let data: OllamaChatResponse = resp.json().await?;
        Ok(data.message.content)
    }
}

#[derive(Debug, Deserialize)]
struct OllamaModels {
    models: Vec<OllamaModel>,
}

#[derive(Debug, Deserialize)]
struct OllamaModel {
    name: String,
}

#[derive(Debug, Deserialize)]
struct OllamaResponse {
    response: String,
}

#[derive(Debug, Deserialize)]
struct OllamaChatResponse {
    message: OllamaMessage,
}

#[derive(Debug, Deserialize)]
struct OllamaMessage {
    content: String,
}
