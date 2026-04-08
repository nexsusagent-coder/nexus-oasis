//! ─── WEBHOOK RECEIVER ───
//!
//! HTTP endpoint olarak webhook'ları dinler.
//! Axum router entegrasyonu.

use sentient_common::error::{SENTIENTError, SENTIENTResult};
use std::sync::Arc;
use tokio::sync::RwLock;
use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Json},
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};

use super::{
    WebhookRouter, WebhookResult, WebhookStats,
    event::WebhookEvent,
};

/// ─── WEBHOOK CONFIG ───

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookConfig {
    /// V-GATE URL
    pub vgate_url: String,
    
    /// İmza doğrulama aktif mi?
    pub verify_signatures: bool,
    
    /// Timestamp toleransı (saniye)
    pub timestamp_tolerance_secs: i64,
    
    /// Maximum payload boyutu (bytes)
    pub max_payload_size: usize,
    
    /// Event buffer size
    pub event_buffer_size: usize,
    
    /// Rate limit (webhook/dakika per provider)
    pub rate_limit_per_minute: u32,
}

impl Default for WebhookConfig {
    fn default() -> Self {
        Self {
            vgate_url: "http://127.0.0.1:1071".into(),
            verify_signatures: true,
            timestamp_tolerance_secs: 300,
            max_payload_size: 1024 * 1024, // 1MB
            event_buffer_size: 1000,
            rate_limit_per_minute: 100,
        }
    }
}

/// ─── WEBHOOK RECEIVER ───

pub struct WebhookReceiver {
    config: WebhookConfig,
    router: Arc<WebhookRouter>,
    stats: Arc<RwLock<WebhookStats>>,
    event_tx: tokio::sync::mpsc::Sender<WebhookEvent>,
    event_rx: Option<tokio::sync::mpsc::Receiver<WebhookEvent>>,
}

impl WebhookReceiver {
    /// Yeni receiver oluştur
    pub fn new(config: WebhookConfig) -> Self {
        let (event_tx, event_rx) = tokio::sync::mpsc::channel(config.event_buffer_size);
        
        Self {
            config: config.clone(),
            router: Arc::new(WebhookRouter::new(&config.vgate_url)),
            stats: Arc::new(RwLock::new(WebhookStats::default())),
            event_tx,
            event_rx: Some(event_rx),
        }
    }
    
    /// Event receiver al
    pub fn take_event_receiver(&mut self) -> Option<tokio::sync::mpsc::Receiver<WebhookEvent>> {
        self.event_rx.take()
    }
    
    /// Başlat
    pub async fn start(&self) -> SENTIENTResult<()> {
        self.router.setup_defaults().await;
        
        log::info!("════════════════════════════════════════════════════════════");
        log::info!("🪝  WEBHOOK RECEIVER başlatılıyor...");
        log::info!("════════════════════════════════════════════════════════════");
        log::info!("   V-GATE: {}", self.config.vgate_url);
        log::info!("   İmza doğrulama: {}", if self.config.verify_signatures { "aktif" } else { "pasif" });
        log::info!("   Buffer boyutu: {}", self.config.event_buffer_size);
        log::info!("════════════════════════════════════════════════════════════");
        
        Ok(())
    }
    
    /// Axum router oluştur
    pub fn create_router(&self) -> Router {
        let state = WebhookState {
            router: self.router.clone(),
            stats: self.stats.clone(),
            event_tx: self.event_tx.clone(),
        };
        
        Router::new()
            .route("/webhook/:provider", post(handle_webhook))
            .route("/webhook/stats", axum::routing::get(get_stats))
            .with_state(state)
    }
    
    /// İstatistikler
    pub async fn stats(&self) -> WebhookStats {
        self.stats.read().await.clone()
    }
    
    /// Webhook işle (internal)
    pub async fn process_webhook(
        &self,
        provider: &str,
        headers: HeaderMap,
        body: String,
    ) -> SENTIENTResult<WebhookResult> {
        // Header'ları map'e çevir
        let headers_map: std::collections::HashMap<String, String> = headers
            .iter()
            .map(|(name, value)| {
                (name.to_string(), value.to_str().unwrap_or("").to_string())
            })
            .collect();
        
        // Payload boyutu kontrolü
        if body.len() > self.config.max_payload_size {
            return Err(SENTIENTError::ValidationError(
                format!("Payload çok büyük: {} bytes (max: {})", 
                    body.len(), self.config.max_payload_size)
            ));
        }
        
        // Router'a gönder
        let result = self.router.route(provider, &headers_map, &body).await?;
        
        // İstatistikleri güncelle
        {
            let mut stats = self.stats.write().await;
            stats.total_received += 1;
            stats.total_processed += 1;
            stats.last_received = Some(chrono::Utc::now());
            
            let provider_key = provider.to_string();
            *stats.by_provider.entry(provider_key).or_insert(0) += 1;
        }
        
        Ok(result)
    }
}

/// ─── API STATE ───

#[derive(Clone)]
struct WebhookState {
    router: Arc<WebhookRouter>,
    stats: Arc<RwLock<WebhookStats>>,
    event_tx: tokio::sync::mpsc::Sender<WebhookEvent>,
}

/// ─── HANDLERS ───

/// Webhook endpoint handler
async fn handle_webhook(
    State(state): State<WebhookState>,
    Path(provider): Path<String>,
    headers: HeaderMap,
    body: String,
) -> impl IntoResponse {
    log::debug!("webhook  {} endpoint'ine istek geldi", provider);
    
    // Header'ları map'e çevir
    let headers_map: std::collections::HashMap<String, String> = headers
        .iter()
        .map(|(name, value)| {
            (name.to_string(), value.to_str().unwrap_or("").to_string())
        })
        .collect();
    
    // Router'a gönder
    match state.router.route(&provider, &headers_map, &body).await {
        Ok(result) => {
            // İstatistikleri güncelle
            {
                let mut stats = state.stats.write().await;
                stats.total_received += 1;
                stats.total_processed += 1;
                stats.last_received = Some(chrono::Utc::now());
                *stats.by_provider.entry(provider.clone()).or_insert(0) += 1;
            }
            
            (StatusCode::OK, Json(result)).into_response()
        }
        Err(e) => {
            // İstatistikleri güncelle
            {
                let mut stats = state.stats.write().await;
                stats.total_received += 1;
                stats.total_failed += 1;
            }
            
            let result = WebhookResult::failure(e.to_sentient_message());
            (StatusCode::BAD_REQUEST, Json(result)).into_response()
        }
    }
}

/// İstatistikler endpoint'i
async fn get_stats(
    State(state): State<WebhookState>,
) -> impl IntoResponse {
    let stats = state.stats.read().await.clone();
    (StatusCode::OK, Json(stats))
}

// ─── Tests ───

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_webhook_config_default() {
        let config = WebhookConfig::default();
        assert!(config.verify_signatures);
        assert_eq!(config.max_payload_size, 1024 * 1024);
    }
    
    #[tokio::test]
    async fn test_webhook_receiver_creation() {
        let receiver = WebhookReceiver::new(WebhookConfig::default());
        assert!(receiver.event_rx.is_some());
    }
    
    #[tokio::test]
    async fn test_webhook_stats() {
        let receiver = WebhookReceiver::new(WebhookConfig::default());
        let stats = receiver.stats().await;
        
        assert_eq!(stats.total_received, 0);
        assert_eq!(stats.total_processed, 0);
    }
}
