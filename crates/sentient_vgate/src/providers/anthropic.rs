//! ─── ANTHROPIC SAĞLAYICISI ───
//!
//! Claude modelleri için Anthropic API sağlayıcısı.

use crate::providers::base::*;
use crate::translate_raw_error;
use sentient_common::error::{SENTIENTError, SENTIENTResult};
use async_trait::async_trait;
use log;
use reqwest::Client;
use serde_json::json;

pub struct AnthropicProvider {
    base_url: String,
    api_key: String,
    client: Client,
}

impl AnthropicProvider {
    pub fn new(base_url: String, api_key: String) -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(120))
            .build()
            .expect("Anthropic: HTTP istemci oluşturulamadı");

        log::info!("🎭  ANTHROPIC: Sağlayıcı başlatıldı → {}", base_url);
        Self { base_url, api_key, client }
    }

    fn build_headers(&self) -> reqwest::header::HeaderMap {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("x-api-key", self.api_key.parse().unwrap());
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers.insert("anthropic-version", "2023-06-01".parse().unwrap());
        headers
    }

    /// Anthropic API formatına çevir
    fn convert_request(&self, request: &LlmRequest) -> serde_json::Value {
        let system_prompt = request.messages
            .iter()
            .filter(|m| m.role == "system")
            .map(|m| m.content.clone())
            .collect::<Vec<_>>()
            .join("\n");

        let messages: Vec<_> = request.messages
            .iter()
            .filter(|m| m.role != "system")
            .map(|m| json!({
                "role": if m.role == "assistant" { "assistant" } else { "user" },
                "content": m.content
            }))
            .collect();

        let mut body = json!({
            "model": request.model,
            "messages": messages,
            "max_tokens": request.max_tokens.unwrap_or(4096),
        });

        if !system_prompt.is_empty() {
            body["system"] = json!(system_prompt);
        }

        body
    }

    /// Anthropic yanıtını standart formata çevir
    fn convert_response(&self, body: &serde_json::Value) -> LlmResponse {
        let content = body["content"]
            .as_array()
            .and_then(|arr| arr.first())
            .and_then(|c| c["text"].as_str())
            .unwrap_or("")
            .to_string();

        LlmResponse {
            id: body["id"].as_str().unwrap_or("unknown").to_string(),
            model: body["model"].as_str().unwrap_or("claude").to_string(),
            choices: vec![ResponseChoice {
                index: 0,
                message: ResponseMessage {
                    role: "assistant".into(),
                    content: Some(content),
                },
                finish_reason: body["stop_reason"].as_str().map(|s| s.to_string()),
            }],
            usage: body["usage"].as_object().map(|u| UsageStats {
                prompt_tokens: u["input_tokens"].as_u64().unwrap_or(0),
                completion_tokens: u["output_tokens"].as_u64().unwrap_or(0),
                total_tokens: u["input_tokens"].as_u64().unwrap_or(0)
                    + u["output_tokens"].as_u64().unwrap_or(0),
            }),
            created: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
}

#[async_trait]
impl LlmProvider for AnthropicProvider {
    async fn chat_completion(&self, request: LlmRequest) -> SENTIENTResult<LlmResponse> {
        let url = format!("{}/messages", self.base_url);

        let body = self.convert_request(&request);

        log::debug!("🎭  ANTHROPIC: İstek gönderiliyor → {}", request.model);

        let response = self.client
            .post(&url)
            .headers(self.build_headers())
            .json(&body)
            .send()
            .await
            .map_err(|e| {
                log::warn!("🎭  ANTHROPIC HATA → {}", translate_raw_error(&e.to_string()));
                SENTIENTError::VGate(translate_raw_error(&e.to_string()))
            })?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(SENTIENTError::VGate(format!(
                "Anthropic API hatası [{}]: {}",
                status,
                error_text.chars().take(200).collect::<String>()
            )));
        }

        let response_body: serde_json::Value = response.json().await
            .map_err(|e| SENTIENTError::VGate(format!("JSON hatası: {}", e)))?;

        let llm_response = self.convert_response(&response_body);

        log::info!(
            "🎭  ANTHROPIC: Yanıt alındı → {} token",
            llm_response.usage.as_ref().map(|u| u.total_tokens).unwrap_or(0)
        );

        Ok(llm_response)
    }

    async fn list_models(&self) -> SENTIENTResult<Vec<ModelInfo>> {
        // Anthropic'ın model listesi endpoint'i yok, sabit liste döndür
        Ok(vec![
            ModelInfo {
                id: "claude-3-5-sonnet-20241022".into(),
                name: "Claude 3.5 Sonnet".into(),
                provider: "anthropic".into(),
                context_length: Some(200000),
                pricing: None,
            },
            ModelInfo {
                id: "claude-3-5-haiku-20241022".into(),
                name: "Claude 3.5 Haiku".into(),
                provider: "anthropic".into(),
                context_length: Some(200000),
                pricing: None,
            },
            ModelInfo {
                id: "claude-3-opus-20240229".into(),
                name: "Claude 3 Opus".into(),
                provider: "anthropic".into(),
                context_length: Some(200000),
                pricing: None,
            },
        ])
    }

    fn name(&self) -> &str {
        "anthropic"
    }
}
