//! ─── SENTIENT V-GATE (VEKİL SUNUCU KATMANI) ───
//!
//! Dış API'lere doğrudan bağlanmak yerine Vekil Sunucu (Proxy)
//! katmanı üzerinden iletişim. API anahtarları asla istemcide tutulmaz.
//! Guardrails entegrasyonu ile giden/gelen istekler denetlenir.
//!
//! ════════════════════════════════════════════════════════════════
//!  GÜVENLİK NOTLARI:
//!  - API anahtarları ASLA istemciye gönderilmez
//!  - API anahtarları ASLA log'a yazılmaz  
//!  - Tüm istekler Guardrails'ten geçer
//!  - Rate limiting uygulanır
//! ════════════════════════════════════════════════════════════════

pub mod auth;
pub mod envguard;
pub mod middleware;
pub mod providers;
pub mod routes;

use sentient_common::error::{SENTIENTError, SENTIENTResult};
use sentient_common::events::{SENTIENTEvent, EventType};
use sentient_guardrails::GuardrailEngine;
use log;
use reqwest::Client;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

// ─── Legacy Exports (Backward Compatibility) ───

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LlmRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
    pub stream: Option<bool>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LlmResponse {
    pub content: String,
    pub model: String,
    pub usage: Option<UsageInfo>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UsageInfo {
    pub prompt_tokens: u64,
    pub completion_tokens: u64,
    pub total_tokens: u64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VGateConfig {
    pub proxy_url: String,
    pub listen_addr: String,
    pub request_timeout_secs: u64,
}

impl Default for VGateConfig {
    fn default() -> Self {
        Self {
            proxy_url: "http://localhost:8100".to_string(),
            listen_addr: "127.0.0.1:1071".to_string(),
            request_timeout_secs: 120,
        }
    }
}

/// ─── V-GATE Runtime State ───

pub struct VGateState {
    pub config: VGateConfig,
    pub auth: auth::ApiKeyManager,
    pub guardrails: GuardrailEngine,
    pub rate_limiter: middleware::rate_limit::RateLimiter,
    pub http_client: Client,
    pub start_time: Instant,
    request_count: Mutex<u64>,
}

impl VGateState {
    /// Yeni V-GATE state oluştur
    pub fn new(config: VGateConfig) -> Self {
        let http_client = Client::builder()
            .timeout(Duration::from_secs(config.request_timeout_secs))
            .build()
            .expect("V-GATE: HTTP istemci oluşturulamadı");

        log::info!("🚪  V-GATE: Vekil sunucu katmanı başlatıldı");
        log::info!("🚪  V-GATE: Dinleme adresi: {}", config.listen_addr);
        log::info!("🚪  V-GATE: Hedef proxy: {}", config.proxy_url);

        Self {
            config,
            auth: auth::ApiKeyManager::new(),
            guardrails: GuardrailEngine::new(),
            rate_limiter: middleware::rate_limit::RateLimiter::new(
                middleware::rate_limit::RateLimitConfig::default()
            ),
            http_client,
            start_time: Instant::now(),
            request_count: Mutex::new(0),
        }
    }

    /// Toplam istek sayısını al
    pub async fn total_requests(&self) -> u64 {
        *self.request_count.lock().await
    }

    /// İstek sayısını artır
    pub async fn increment_request_count(&self) {
        let mut count = self.request_count.lock().await;
        *count += 1;
    }
}

// ─── Legacy VGateEngine (Backward Compatibility) ───

pub struct VGateEngine {
    config: VGateConfig,
    guardrails: Arc<Mutex<GuardrailEngine>>,
    http_client: Client,
    request_count: Mutex<u64>,
}

impl VGateEngine {
    pub fn new(config: VGateConfig) -> Self {
        let http_client = Client::builder()
            .timeout(std::time::Duration::from_secs(config.request_timeout_secs))
            .build()
            .expect("V-GATE: HTTP istemci oluşturulamadı");

        log::info!("🚪  V-GATE: Vekil sunucu katmanı başlatıldı: {}", config.proxy_url);

        Self {
            config,
            guardrails: Arc::new(Mutex::new(GuardrailEngine::new())),
            http_client,
            request_count: Mutex::new(0),
        }
    }

    /// LLM'e istek gönder (Guardrails denetimli)
    pub async fn send_request(&self, request: LlmRequest) -> SENTIENTResult<SENTIENTEvent> {
        // 1) GİDEN denetim — her mesajı guardrails'ten geçir
        let guard = self.guardrails.lock().await;
        for msg in &request.messages {
            let result = guard.check_input(&msg.content);
            if !result.is_clean() {
                log::warn!("🚪  V-GATE: Giden istek Guardrails tarafından engellendi.");
                return Err(SENTIENTError::VGate(
                    "Giden istek güvenlik filtresine takıldı.".into(),
                ));
            }
        }
        drop(guard);

        // 2) Proxy'ye ilet
        let proxy_url = format!("{}/v1/chat/completions", self.config.proxy_url);
        let response = self
            .http_client
            .post(&proxy_url)
            .json(&request)
            .send()
            .await
            .map_err(|e| {
                let msg = translate_raw_error(&e.to_string());
                SENTIENTError::VGate(msg)
            })?;

        // 3) Yanıtı işle
        let status = response.status();
        if !status.is_success() {
            return Err(SENTIENTError::VGate(format!(
                "Proxy yanıt hatası: {}",
                status
            )));
        }

        let body: serde_json::Value = response.json().await.map_err(|e| {
            SENTIENTError::VGate(format!("JSON yanıt ayrıştırma hatası: {}", e))
        })?;

        // 4) Gelen yanıtı Guardrails ile denetle
        let content = extract_content_from_response(&body);
        let guard = self.guardrails.lock().await;
        let check = guard.check_output(&content);
        if !check.is_clean() {
            log::warn!("🚪  V-GATE: Gelen yanıt Guardrails tarafından engellendi.");
            return Err(SENTIENTError::VGate(
                "Gelen yanıt güvenlik filtresine takıldı.".into(),
            ));
        }

        // 5) Sayaç güncelle
        {
            let mut count = self.request_count.lock().await;
            *count += 1;
        }

        // 6) Event üret
        let event = SENTIENTEvent::new(
            EventType::VGateResponse,
            "vgate",
            serde_json::json!({
                "content": content,
                "model": request.model,
            }),
        );

        log::info!("🚪  V-GATE: Yanıt başarıyla alındı ve denetlendi.");
        Ok(event)
    }

    /// Guardrails motorunu getir
    pub fn guardrails(&self) -> Arc<Mutex<GuardrailEngine>> {
        Arc::clone(&self.guardrails)
    }

    /// Toplam istek sayısı
    pub async fn request_count(&self) -> u64 {
        *self.request_count.lock().await
    }

    /// HTTP sunucusunu başlat
    pub async fn serve(&self) -> SENTIENTResult<()> {
        let state = Arc::new(Mutex::new(VGateState::new(self.config.clone())));
        let addr: SocketAddr = self.config.listen_addr.parse().map_err(|e| {
            SENTIENTError::VGate(format!("Geçersiz dinleme adresi: {}", e))
        })?;

        let router = routes::create_router(state);

        log::info!("🚪  V-GATE: HTTP sunucu dinliyor: {}", addr);
        let listener = tokio::net::TcpListener::bind(addr)
            .await
            .map_err(|e| SENTIENTError::VGate(format!("Sunucu bağlanamadı: {}", e)))?;

        axum::serve(listener, router)
            .await
            .map_err(|e| SENTIENTError::VGate(format!("Sunucu hatası: {}", e)))?;

        Ok(())
    }
}

// ─── Yardımcı Fonksiyonlar ───

fn extract_content_from_response(body: &serde_json::Value) -> String {
    body.get("choices")
        .and_then(|c| c.get(0))
        .and_then(|c| c.get("message"))
        .and_then(|m| m.get("content"))
        .and_then(|c| c.as_str())
        .unwrap_or("")
        .to_string()
}

fn translate_raw_error(raw: &str) -> String {
    sentient_common::translate::translate_raw_error(raw)
}

// ─── Tests ───

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = VGateConfig::default();
        assert_eq!(config.proxy_url, "http://localhost:8100");
        assert_eq!(config.listen_addr, "127.0.0.1:1071");
        assert_eq!(config.request_timeout_secs, 120);
    }

    #[test]
    fn test_extract_content() {
        let body = serde_json::json!({
            "choices": [{
                "message": {
                    "content": "Hello from LLM"
                }
            }]
        });
        assert_eq!(extract_content_from_response(&body), "Hello from LLM");
    }

    #[test]
    fn test_extract_empty_content() {
        let body = serde_json::json!({});
        assert_eq!(extract_content_from_response(&body), "");
    }
}
