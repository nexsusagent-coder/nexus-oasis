//! ─── OPENAI SAĞLAYICISI ───
//!
//! OpenAI ve OpenAI uyumlu API'ler (Groq, Local) için sağlayıcı.

use crate::providers::base::*;
use crate::translate_raw_error;
use sentient_common::error::{SENTIENTError, SENTIENTResult};
use async_trait::async_trait;
use log;
use reqwest::Client;
use serde_json::json;

pub struct OpenAIProvider {
    base_url: String,
    api_key: String,
    client: Client,
}

impl OpenAIProvider {
    pub fn new(base_url: String, api_key: String) -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(120))
            .build()
            .expect("OpenAI: HTTP istemci oluşturulamadı");

        log::info!("🤖  OPENAI: Sağlayıcı başlatıldı → {}", base_url);
        Self { base_url, api_key, client }
    }

    fn build_headers(&self) -> reqwest::header::HeaderMap {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "Authorization",
            format!("Bearer {}", self.api_key).parse().expect("operation failed"),
        );
        headers.insert("Content-Type", "application/json".parse().expect("operation failed"));
        headers
    }
}

#[async_trait]
impl LlmProvider for OpenAIProvider {
    async fn chat_completion(&self, request: LlmRequest) -> SENTIENTResult<LlmResponse> {
        let url = format!("{}/chat/completions", self.base_url);

        let body = json!({
            "model": request.model,
            "messages": request.messages.iter().map(|m| json!({
                "role": m.role,
                "content": m.content
            })).collect::<Vec<_>>(),
            "max_tokens": request.max_tokens,
            "temperature": request.temperature,
            "stream": false,
        });

        log::debug!("🤖  OPENAI: İstek gönderiliyor → {}", request.model);

        let response = self.client
            .post(&url)
            .headers(self.build_headers())
            .json(&body)
            .send()
            .await
            .map_err(|e| {
                log::warn!("🤖  OPENAI HATA → {}", translate_raw_error(&e.to_string()));
                SENTIENTError::VGate(translate_raw_error(&e.to_string()))
            })?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(SENTIENTError::VGate(format!(
                "OpenAI API hatası [{}]: {}",
                status,
                error_text.chars().take(200).collect::<String>()
            )));
        }

        let response_text = response.text().await
            .map_err(|e| SENTIENTError::VGate(format!("Yanıt okuma hatası: {}", e)))?;

        let llm_response: LlmResponse = serde_json::from_str(&response_text)
            .map_err(|e| {
                log::error!("🤖  OPENAI JSON hatası: {}", e);
                SENTIENTError::VGate(format!("JSON ayrıştırma hatası: {}", e))
            })?;

        log::info!(
            "🤖  OPENAI: Yanıt alındı → {} token",
            llm_response.usage.as_ref().map(|u| u.total_tokens).unwrap_or(0)
        );

        Ok(llm_response)
    }

    async fn list_models(&self) -> SENTIENTResult<Vec<ModelInfo>> {
        let url = format!("{}/models", self.base_url);

        let response = self.client
            .get(&url)
            .headers(self.build_headers())
            .send()
            .await
            .map_err(|e| SENTIENTError::VGate(translate_raw_error(&e.to_string())))?;

        if !response.status().is_success() {
            return Err(SENTIENTError::VGate("Model listesi alınamadı".into()));
        }

        let body: serde_json::Value = response.json().await
            .map_err(|e| SENTIENTError::VGate(format!("JSON hatası: {}", e)))?;

        let models: Vec<ModelInfo> = body["data"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|m| {
                        Some(ModelInfo {
                            id: m["id"].as_str()?.to_string(),
                            name: m["id"].as_str().unwrap_or("Bilinmeyen").to_string(),
                            provider: "openai".into(),
                            context_length: m["context_length"].as_u64(),
                            pricing: None,
                        })
                    })
                    .take(50)
                    .collect()
            })
            .unwrap_or_default();

        Ok(models)
    }

    fn name(&self) -> &str {
        "openai"
    }
}
