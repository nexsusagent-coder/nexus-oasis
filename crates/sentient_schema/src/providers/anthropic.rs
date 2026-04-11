// ═══════════════════════════════════════════════════════════════════════════════
//  SENTIENT OS - Anthropic Provider
// ═══════════════════════════════════════════════════════════════════════════════
//  Supports: Claude 3.5 Sonnet, Claude 3 Opus, Claude 3 Haiku
//  Features: Tool use, Structured outputs
// ═══════════════════════════════════════════════════════════════════════════════

use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::info;

use super::SchemaProvider;
use crate::function::{FunctionCall, FunctionDef};
use crate::error::{SchemaError, Result};

/// Anthropic API client
pub struct AnthropicProvider {
    api_key: String,
    client: Client,
    model: String,
    base_url: String,
}

impl AnthropicProvider {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            client: Client::new(),
            model: "claude-3-5-sonnet-20241022".to_string(),
            base_url: "https://api.anthropic.com/v1".to_string(),
        }
    }

    /// Set model
    pub fn with_model(mut self, model: impl Into<String>) -> Self {
        self.model = model.into();
        self
    }
}

#[derive(Serialize)]
struct AnthropicRequest {
    model: String,
    max_tokens: u32,
    messages: Vec<AnthropicMessage>,
    system: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<serde_json::Value>>,
}

#[derive(Serialize)]
struct AnthropicMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct AnthropicResponse {
    content: Vec<ContentBlock>,
}

#[derive(Deserialize)]
struct ContentBlock {
    #[serde(rename = "type")]
    block_type: String,
    text: Option<String>,
    name: Option<String>,
    input: Option<serde_json::Value>,
}

#[async_trait]
impl SchemaProvider for AnthropicProvider {
    async fn generate(&self, prompt: &str, schema: serde_json::Value) -> Result<String> {
        info!("Anthropic: Generating structured output");

        let system_prompt = format!(
            "You are a data extraction assistant. Respond ONLY with valid JSON matching this schema:\n{}",
            serde_json::to_string_pretty(&schema)?
        );

        let request = AnthropicRequest {
            model: self.model.clone(),
            max_tokens: 4096,
            messages: vec![
                AnthropicMessage {
                    role: "user".to_string(),
                    content: prompt.to_string(),
                },
            ],
            system: Some(system_prompt),
            tools: None,
        };

        let response = self.client
            .post(format!("{}/messages", self.base_url))
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(SchemaError::ProviderError(format!("Anthropic error: {} - {}", status, body)));
        }

        let anthropic_response: AnthropicResponse = response.json().await?;
        
        anthropic_response
            .content
            .first()
            .and_then(|c| c.text.clone())
            .ok_or_else(|| SchemaError::InvalidResponse("No text in response".to_string()))
    }

    async fn function_call(&self, prompt: &str, functions: Vec<FunctionDef>) -> Result<FunctionCall> {
        info!("Anthropic: Tool use with {} tools", functions.len());

        let tools: Vec<_> = functions.iter().map(|f| f.to_anthropic()).collect();

        let request = AnthropicRequest {
            model: self.model.clone(),
            max_tokens: 4096,
            messages: vec![
                AnthropicMessage {
                    role: "user".to_string(),
                    content: prompt.to_string(),
                },
            ],
            system: None,
            tools: Some(tools),
        };

        let response = self.client
            .post(format!("{}/messages", self.base_url))
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(SchemaError::ProviderError(format!("Anthropic error: {} - {}", status, body)));
        }

        let anthropic_response: AnthropicResponse = response.json().await?;
        
        let tool_use = anthropic_response
            .content
            .iter()
            .find(|c| c.block_type == "tool_use")
            .ok_or_else(|| SchemaError::FunctionCallFailed("No tool use in response".to_string()))?;

        Ok(FunctionCall {
            name: tool_use.name.clone().unwrap_or_default(),
            arguments: tool_use.input.clone().unwrap_or_default(),
        })
    }

    fn name(&self) -> &'static str {
        "anthropic"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anthropic_provider_creation() {
        let provider = AnthropicProvider::new("test-key");
        assert_eq!(provider.name(), "anthropic");
    }
}
