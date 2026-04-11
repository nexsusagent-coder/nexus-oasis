// ═══════════════════════════════════════════════════════════════════════════════
//  SENTIENT OS - OpenAI Provider
// ═══════════════════════════════════════════════════════════════════════════════
//  Supports: GPT-4, GPT-3.5, GPT-4o
//  Features: Function calling, JSON mode, Structured outputs
// ═══════════════════════════════════════════════════════════════════════════════

use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::info;

use super::SchemaProvider;
use crate::function::{FunctionCall, FunctionDef};
use crate::error::{SchemaError, Result};

/// OpenAI API client
pub struct OpenAIProvider {
    api_key: String,
    client: Client,
    model: String,
    base_url: String,
}

impl OpenAIProvider {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            client: Client::new(),
            model: "gpt-4o".to_string(),
            base_url: "https://api.openai.com/v1".to_string(),
        }
    }

    /// Set model
    pub fn with_model(mut self, model: impl Into<String>) -> Self {
        self.model = model.into();
        self
    }

    /// Set base URL (for Azure or proxies)
    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }
}

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    functions: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    function_call: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_format: Option<serde_json::Value>,
    temperature: f32,
}

#[derive(Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: ResponseMessage,
}

#[derive(Deserialize)]
struct ResponseMessage {
    content: Option<String>,
    function_call: Option<FunctionCallResponse>,
}

#[derive(Deserialize)]
struct FunctionCallResponse {
    name: String,
    arguments: String,
}

#[async_trait]
impl SchemaProvider for OpenAIProvider {
    async fn generate(&self, prompt: &str, schema: serde_json::Value) -> Result<String> {
        info!("OpenAI: Generating structured output");

        let system_prompt = format!(
            "You are a data extraction assistant. Respond ONLY with valid JSON matching this schema:\n{}",
            serde_json::to_string_pretty(&schema)?
        );

        let request = ChatRequest {
            model: self.model.clone(),
            messages: vec![
                Message { role: "system".to_string(), content: system_prompt },
                Message { role: "user".to_string(), content: prompt.to_string() },
            ],
            functions: None,
            function_call: None,
            response_format: Some(json!({"type": "json_object"})),
            temperature: 0.0,
        };

        let response = self.client
            .post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(SchemaError::ProviderError(format!("OpenAI error: {} - {}", status, body)));
        }

        let chat_response: ChatResponse = response.json().await?;
        
        chat_response
            .choices
            .first()
            .and_then(|c| c.message.content.clone())
            .ok_or_else(|| SchemaError::InvalidResponse("No content in response".to_string()))
    }

    async fn function_call(&self, prompt: &str, functions: Vec<FunctionDef>) -> Result<FunctionCall> {
        info!("OpenAI: Function calling with {} functions", functions.len());

        let functions_json: Vec<_> = functions.iter().map(|f| f.to_openai()).collect();

        let request = ChatRequest {
            model: self.model.clone(),
            messages: vec![
                Message { role: "user".to_string(), content: prompt.to_string() },
            ],
            functions: Some(functions_json),
            function_call: Some("auto".to_string()),
            response_format: None,
            temperature: 0.0,
        };

        let response = self.client
            .post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(SchemaError::ProviderError(format!("OpenAI error: {} - {}", status, body)));
        }

        let chat_response: ChatResponse = response.json().await?;
        
        let choice = chat_response
            .choices
            .first()
            .ok_or_else(|| SchemaError::InvalidResponse("No choices in response".to_string()))?;

        let func_call = choice
            .message
            .function_call
            .as_ref()
            .ok_or_else(|| SchemaError::FunctionCallFailed("No function call in response".to_string()))?;

        let arguments: serde_json::Value = serde_json::from_str(&func_call.arguments)?;

        Ok(FunctionCall {
            name: func_call.name.clone(),
            arguments,
        })
    }

    fn name(&self) -> &'static str {
        "openai"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_openai_provider_creation() {
        let provider = OpenAIProvider::new("test-key");
        assert_eq!(provider.name(), "openai");
    }
}
