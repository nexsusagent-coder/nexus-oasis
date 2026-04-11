// ═══════════════════════════════════════════════════════════════════════════════
//  SENTIENT OS - Groq Client
// ═══════════════════════════════════════════════════════════════════════════════
//  Main API client for Groq LPU
// ═══════════════════════════════════════════════════════════════════════════════

use reqwest::Client;
use serde::Deserialize;
use std::time::Duration;
use tracing::{debug, info, warn};

use crate::{GroqConfig, GroqModel, GroqError, Result};
use crate::chat::{ChatRequest, ChatResponse};
use crate::models::ModelInfo;

/// Groq API client
pub struct GroqClient {
    config: GroqConfig,
    http: Client,
}

impl GroqClient {
    /// Create a new Groq client
    pub fn new(config: GroqConfig) -> Result<Self> {
        if config.api_key.is_empty() {
            return Err(GroqError::MissingApiKey);
        }

        let http = Client::builder()
            .timeout(Duration::from_secs(config.timeout_secs))
            .build()
            .map_err(GroqError::HttpError)?;

        Ok(Self { config, http })
    }

    /// Create from environment variable
    pub fn from_env() -> Result<Self> {
        Self::new(GroqConfig::from_env()?)
    }

    /// Create with API key
    pub fn with_key(api_key: impl Into<String>) -> Result<Self> {
        Self::new(GroqConfig::new(api_key))
    }

    /// Chat completion
    pub async fn chat(&self, request: ChatRequest) -> Result<ChatResponse> {
        let model = request.model.clone();
        info!("Groq: Chat completion with model {}", model);

        let response = self.http
            .post(format!("{}/chat/completions", self.config.base_url))
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// Simple chat with messages
    pub async fn chat_simple(
        &self,
        messages: Vec<crate::chat::ChatMessage>,
    ) -> Result<ChatResponse> {
        let request = ChatRequest::new(self.config.default_model.id(), messages);
        self.chat(request).await
    }

    /// Chat with a single prompt
    pub async fn complete(&self, prompt: &str) -> Result<String> {
        let messages = vec![
            crate::chat::ChatMessage::user(prompt),
        ];

        let response = self.chat_simple(messages).await?;
        Ok(response.content().unwrap_or_default().to_string())
    }

    /// Chat with system prompt
    pub async fn chat_with_system(
        &self,
        system: &str,
        user: &str,
    ) -> Result<String> {
        let messages = vec![
            crate::chat::ChatMessage::system(system),
            crate::chat::ChatMessage::user(user),
        ];

        let response = self.chat_simple(messages).await?;
        Ok(response.content().unwrap_or_default().to_string())
    }

    /// Chat with retry on failure
    pub async fn chat_with_retry(
        &self,
        request: ChatRequest,
        max_retries: u32,
    ) -> Result<ChatResponse> {
        let mut last_error = None;

        for attempt in 0..=max_retries {
            match self.chat(request.clone()).await {
                Ok(response) => return Ok(response),
                Err(e) => {
                    if e.is_retryable() && attempt < max_retries {
                        let wait_ms = e.retry_after_ms().unwrap_or(1000 * (attempt + 1) as u64);
                        warn!("Retry {} after {}ms: {}", attempt + 1, wait_ms, e);
                        tokio::time::sleep(Duration::from_millis(wait_ms)).await;
                    }
                    last_error = Some(e);
                }
            }
        }

        Err(last_error.unwrap_or(GroqError::MaxRetriesExceeded(max_retries)))
    }

    /// List available models
    pub async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        debug!("Groq: Listing models");

        #[derive(Deserialize)]
        struct ModelsResponse {
            data: Vec<ModelInfo>,
        }

        let response = self.http
            .get(format!("{}/models", self.config.base_url))
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .send()
            .await?;

        let models: ModelsResponse = self.handle_response(response).await?;
        Ok(models.data)
    }

    /// Get current model
    pub fn default_model(&self) -> GroqModel {
        self.config.default_model
    }

    /// Set default model
    pub fn set_model(&mut self, model: GroqModel) {
        self.config.default_model = model;
    }

    /// Check API key validity
    pub async fn check_api_key(&self) -> Result<bool> {
        match self.list_models().await {
            Ok(_) => Ok(true),
            Err(GroqError::InvalidApiKey) => Ok(false),
            Err(e) => Err(e),
        }
    }

    /// Handle HTTP response
    async fn handle_response<T: for<'de> Deserialize<'de>>(
        &self,
        response: reqwest::Response,
    ) -> Result<T> {
        let status = response.status();

        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            
            // Parse error message
            #[derive(Deserialize)]
            struct ErrorResponse {
                error: Option<ErrorDetail>,
            }
            #[derive(Deserialize)]
            struct ErrorDetail {
                message: Option<String>,
                #[serde(rename = "type")]
                error_type: Option<String>,
            }

            let error_msg = if let Ok(err) = serde_json::from_str::<ErrorResponse>(&body) {
                err.error.map(|e| e.message.unwrap_or_default())
                    .unwrap_or(body.clone())
            } else {
                body
            };

            return Err(match status.as_u16() {
                401 => GroqError::InvalidApiKey,
                404 => GroqError::ModelNotFound(error_msg),
                429 => GroqError::RateLimitExceeded,
                _ => GroqError::ApiError(format!("HTTP {}: {}", status, error_msg)),
            });
        }

        response.json().await.map_err(GroqError::HttpError)
    }
}

/// Builder for fluent API
pub struct GroqClientBuilder {
    config: GroqConfig,
}

impl GroqClientBuilder {
    pub fn new() -> Self {
        Self {
            config: GroqConfig::default(),
        }
    }

    pub fn api_key(mut self, key: impl Into<String>) -> Self {
        self.config.api_key = key.into();
        self
    }

    pub fn model(mut self, model: GroqModel) -> Self {
        self.config.default_model = model;
        self
    }

    pub fn timeout(mut self, secs: u64) -> Self {
        self.config.timeout_secs = secs;
        self
    }

    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.config.base_url = url.into();
        self
    }

    pub fn build(self) -> Result<GroqClient> {
        GroqClient::new(self.config)
    }
}

impl Default for GroqClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_builder() {
        let result = GroqClientBuilder::new()
            .api_key("test-key")
            .model(GroqModel::Mixtral_8x7B)
            .timeout(60)
            .build();

        assert!(result.is_ok());
        let client = result.unwrap();
        assert_eq!(client.default_model(), GroqModel::Mixtral_8x7B);
    }

    #[test]
    fn test_client_requires_api_key() {
        let result = GroqClient::new(GroqConfig::default());
        assert!(matches!(result, Err(GroqError::MissingApiKey)));
    }
}
