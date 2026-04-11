// ═══════════════════════════════════════════════════════════════════════════════
//  SENTIENT OS - Ollama Provider (Local)
// ═══════════════════════════════════════════════════════════════════════════════
//  Supports: Llama 3, Mistral, Gemma, Phi-3, etc.
//  Features: JSON mode, Grammar-based structured output
//  Free: Runs locally, no API key required
// ═══════════════════════════════════════════════════════════════════════════════

use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::info;

use super::SchemaProvider;
use crate::function::{FunctionCall, FunctionDef};
use crate::error::{SchemaError, Result};

/// Ollama API client (local)
pub struct OllamaProvider {
    client: Client,
    model: String,
    base_url: String,
}

impl OllamaProvider {
    pub fn new(base_url: Option<&str>) -> Self {
        Self {
            client: Client::new(),
            model: "llama3.2".to_string(),
            base_url: base_url.unwrap_or("http://localhost:11434").to_string(),
        }
    }

    /// Set model
    pub fn with_model(mut self, model: impl Into<String>) -> Self {
        self.model = model.into();
        self
    }

    /// Check if Ollama is running
    pub async fn check_available(&self) -> bool {
        self.client
            .get(format!("{}/api/tags", self.base_url))
            .send()
            .await
            .map(|r| r.status().is_success())
            .unwrap_or(false)
    }

    /// List available models
    pub async fn list_models(&self) -> Result<Vec<String>> {
        #[derive(Deserialize)]
        struct ModelsResponse {
            models: Vec<ModelInfo>,
        }

        #[derive(Deserialize)]
        struct ModelInfo {
            name: String,
        }

        let response = self.client
            .get(format!("{}/api/tags", self.base_url))
            .send()
            .await?;

        let models: ModelsResponse = response.json().await?;
        Ok(models.models.into_iter().map(|m| m.name).collect())
    }
}

#[derive(Serialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    format: Option<String>,
    stream: bool,
    options: Option<OllamaOptions>,
}

#[derive(Serialize)]
struct OllamaOptions {
    temperature: f32,
}

#[derive(Deserialize)]
struct OllamaResponse {
    response: String,
}

#[async_trait]
impl SchemaProvider for OllamaProvider {
    async fn generate(&self, prompt: &str, schema: serde_json::Value) -> Result<String> {
        info!("Ollama: Generating structured output with model {}", self.model);

        let system_prompt = format!(
            "You are a data extraction assistant. Respond ONLY with valid JSON matching this schema:\n{}\n\nUser request: {}",
            serde_json::to_string_pretty(&schema)?,
            prompt
        );

        let request = OllamaRequest {
            model: self.model.clone(),
            prompt: system_prompt,
            format: Some("json".to_string()),
            stream: false,
            options: Some(OllamaOptions { temperature: 0.0 }),
        };

        let response = self.client
            .post(format!("{}/api/generate", self.base_url))
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(SchemaError::ProviderError(format!("Ollama error: {} - {}", status, body)));
        }

        let ollama_response: OllamaResponse = response.json().await?;
        Ok(ollama_response.response)
    }

    async fn function_call(&self, prompt: &str, functions: Vec<FunctionDef>) -> Result<FunctionCall> {
        info!("Ollama: Function calling with {} functions", functions.len());

        // Ollama doesn't have native function calling, so we use JSON mode
        // and instruct the model to respond with function call format

        let functions_desc: String = functions
            .iter()
            .map(|f| format!("- {}: {}", f.name, f.description))
            .collect::<Vec<_>>()
            .join("\n");

        let function_prompt = format!(
            "Available functions:\n{}\n\nRespond with a JSON object containing 'name' and 'arguments' for the function to call.\nUser request: {}",
            functions_desc,
            prompt
        );

        let schema = serde_json::json!({
            "type": "object",
            "properties": {
                "name": {"type": "string"},
                "arguments": {"type": "object"}
            },
            "required": ["name", "arguments"]
        });

        let response = self.generate(&function_prompt, schema).await?;
        let call: FunctionCall = serde_json::from_str(&response)?;

        Ok(call)
    }

    fn name(&self) -> &'static str {
        "ollama"
    }

    async fn is_available(&self) -> bool {
        self.client
            .get(format!("{}/api/tags", self.base_url))
            .send()
            .await
            .map(|r| r.status().is_success())
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ollama_provider_creation() {
        let provider = OllamaProvider::new(None);
        assert_eq!(provider.name(), "ollama");
        assert_eq!(provider.base_url, "http://localhost:11434");
    }

    #[test]
    fn test_ollama_provider_custom_url() {
        let provider = OllamaProvider::new(Some("http://custom:11434"));
        assert_eq!(provider.base_url, "http://custom:11434");
    }
}
