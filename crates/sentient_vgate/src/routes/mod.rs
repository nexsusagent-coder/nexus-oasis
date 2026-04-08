//! ─── SENTIENT V-GATE ROUTES ───
//!
//! HTTP API endpoint'leri. Tüm istekler Guardrails ve Rate Limit katmanlarından geçer.

pub mod chat;
pub mod models;
pub mod health;
pub mod admin;

use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::VGateState;

/// ─── Ana Router ───

pub fn create_router(state: Arc<Mutex<VGateState>>) -> Router {
    Router::new()
        // Health check
        .route("/health", get(health::health_check))
        .route("/ready", get(health::ready_check))
        
        // Chat endpoints (OpenAI uyumlu)
        .route("/v1/chat/completions", post(chat::chat_completions))
        .route("/v1/chat", post(chat::chat_completions))
        
        // Models
        .route("/v1/models", get(models::list_models))
        .route("/v1/models/{model_id}", get(models::get_model))
        
        // Admin endpoints
        .route("/admin/status", get(admin::status))
        .route("/admin/stats", get(admin::stats))
        .route("/admin/providers", get(admin::list_providers))
        .route("/admin/provider/{provider}/default", post(admin::set_default_provider))
        
        .with_state(state)
}
