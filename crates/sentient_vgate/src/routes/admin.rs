//! ─── ADMIN ROUTES ───
//!
//! Yönetim ve izleme endpoint'leri.

use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use serde_json::json;
use std::sync::Arc;
use tokio::sync::Mutex;
use log;

use crate::VGateState;

/// Sistem durumu
pub async fn status(
    state: State<Arc<Mutex<VGateState>>>,
) -> (StatusCode, Json<serde_json::Value>) {
    log::info!("⚙️  ADMIN: Durum istendi");
    
    let state_guard = state.lock().await;
    
    let status = json!({
        "service": "SENTIENT V-GATE",
        "version": env!("CARGO_PKG_VERSION"),
        "status": "running",
        "uptime_seconds": state_guard.start_time.elapsed().as_secs(),
        "components": {
            "guardrails": {
                "status": "active",
                "policies": state_guard.guardrails.list_policies().len()
            },
            "auth": {
                "providers": state_guard.auth.list_enabled().await.len(),
                "default_provider": state_guard.auth.get_default_provider().await.as_str()
            }
        }
    });
    
    (StatusCode::OK, Json(status))
}

/// İstatistikler
pub async fn stats(
    state: State<Arc<Mutex<VGateState>>>,
) -> (StatusCode, Json<serde_json::Value>) {
    log::info!("📊  ADMIN: İstatistik istendi");
    
    let state_guard = state.lock().await;
    
    // Rate limiter istatistikleri
    let rate_stats = state_guard.rate_limiter.stats().await;
    
    let stats = json!({
        "uptime_seconds": state_guard.start_time.elapsed().as_secs(),
        "rate_limiter": {
            "requests_per_second": rate_stats.global_requests_per_second,
            "requests_per_minute": rate_stats.global_requests_per_minute,
            "active_ips": rate_stats.active_ips
        }
    });
    
    (StatusCode::OK, Json(stats))
}

/// Sağlayıcıları listele
pub async fn list_providers(
    state: State<Arc<Mutex<VGateState>>>,
) -> (StatusCode, Json<serde_json::Value>) {
    log::info!("🔌  ADMIN: Sağlayıcılar istendi");
    
    let state_guard = state.lock().await;
    let providers = state_guard.auth.list_enabled().await;
    
    let provider_list: Vec<_> = providers
        .iter()
        .map(|p| {
            json!({
                "name": p.as_str(),
                "enabled": true,
                "base_url": p.default_base_url()
            })
        })
        .collect();
    
    (
        StatusCode::OK,
        Json(json!({
            "providers": provider_list,
            "default": state_guard.auth.get_default_provider().await.as_str()
        })),
    )
}

/// Varsayılan sağlayıcı ayarla
pub async fn set_default_provider(
    state: State<Arc<Mutex<VGateState>>>,
    Json(payload): Json<serde_json::Value>,
) -> (StatusCode, Json<serde_json::Value>) {
    let provider_name = payload.get("provider").and_then(|v| v.as_str()).unwrap_or("");
    
    log::info!("🔌  ADMIN: Varsayılan sağlayıcı değiştiriliyor → {}", provider_name);
    
    let state_guard = state.lock().await;
    
    match crate::auth::Provider::from_str(provider_name) {
        Some(provider) => {
            match state_guard.auth.set_default_provider(provider).await {
                Ok(_) => {
                    (
                        StatusCode::OK,
                        Json(json!({
                            "success": true,
                            "default_provider": provider.as_str()
                        })),
                    )
                }
                Err(e) => {
                    (
                        StatusCode::BAD_REQUEST,
                        Json(json!({
                            "success": false,
                            "error": e.to_sentient_message()
                        })),
                    )
                }
            }
        }
        None => {
            (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "success": false,
                    "error": format!("Bilinmeyen sağlayıcı: {}", provider_name)
                })),
            )
        }
    }
}
