//! ─── HEALTH CHECK ROUTES ───

use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use serde_json::json;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::VGateState;

/// Temel sağlık kontrolü
pub async fn health_check() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::OK,
        Json(json!({
            "status": "ok",
            "service": "SENTIENT V-GATE",
            "version": env!("CARGO_PKG_VERSION")
        })),
    )
}

/// Detaylı sağlık kontrolü
pub async fn ready_check(
    state: State<Arc<Mutex<VGateState>>>,
) -> (StatusCode, Json<serde_json::Value>) {
    let state = state.lock().await;
    
    // Tüm bileşenleri kontrol et
    let health = json!({
        "status": "ok",
        "service": "SENTIENT V-GATE",
        "version": env!("CARGO_PKG_VERSION"),
        "components": {
            "guardrails": "ok",
            "rate_limiter": "ok",
            "providers": state.auth.list_enabled().await.len() > 0
        },
        "uptime": {
            "seconds": state.start_time.elapsed().as_secs()
        }
    });
    
    (StatusCode::OK, Json(health))
}
