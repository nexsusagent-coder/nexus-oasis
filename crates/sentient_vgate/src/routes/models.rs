//! ─── MODELS ROUTES ───
//!
//! Kullanılabilir modelleri listeleme endpoint'leri.

use axum::{
    extract::{State, Path},
    http::StatusCode,
    Json,
};
use serde_json::json;
use std::sync::Arc;
use tokio::sync::Mutex;
use log;

use crate::VGateState;

/// Modelleri listele
pub async fn list_models(
    state: State<Arc<Mutex<VGateState>>>,
) -> (StatusCode, Json<serde_json::Value>) {
    log::info!("📋  MODELS: Liste istendi");
    
    let state_guard = state.lock().await;
    let providers = state_guard.auth.list_enabled().await;
    drop(state_guard);

    // Varsayılan modeller
    let models = vec![
        json!({
            "id": "openrouter/qwen/qwen3.6-plus:free",
            "object": "model",
            "created": 1700000000,
            "owned_by": "openrouter",
            "name": "Qwen 3.6 Plus (Free)",
            "context_length": 32768,
            "pricing": {
                "prompt": 0.0,
                "completion": 0.0
            }
        }),
        json!({
            "id": "openrouter/qwen/qwen3-coder:free",
            "object": "model",
            "created": 1700000000,
            "owned_by": "openrouter",
            "name": "Qwen 3 Coder (Free)",
            "context_length": 32768,
            "pricing": {
                "prompt": 0.0,
                "completion": 0.0
            }
        }),
        json!({
            "id": "openrouter/deepseek/deepseek-r1-0528:free",
            "object": "model",
            "created": 1700000000,
            "owned_by": "openrouter",
            "name": "DeepSeek R1 (Free)",
            "context_length": 16384,
            "pricing": {
                "prompt": 0.0,
                "completion": 0.0
            }
        }),
        json!({
            "id": "openai/gpt-4o",
            "object": "model",
            "created": 1700000000,
            "owned_by": "openai",
            "name": "GPT-4o",
            "context_length": 128000,
            "pricing": {
                "prompt": 0.005,
                "completion": 0.015
            }
        }),
        json!({
            "id": "anthropic/claude-3.5-sonnet",
            "object": "model",
            "created": 1700000000,
            "owned_by": "anthropic",
            "name": "Claude 3.5 Sonnet",
            "context_length": 200000,
            "pricing": {
                "prompt": 0.003,
                "completion": 0.015
            }
        }),
        json!({
            "id": "groq/llama-3.3-70b-versatile",
            "object": "model",
            "created": 1700000000,
            "owned_by": "groq",
            "name": "Llama 3.3 70B Versatile",
            "context_length": 8192,
            "pricing": {
                "prompt": 0.0,
                "completion": 0.0
            }
        }),
    ];

    (
        StatusCode::OK,
        Json(json!({
            "object": "list",
            "data": models,
            "providers": providers.iter().map(|p| p.as_str()).collect::<Vec<_>>()
        })),
    )
}

/// Tek model getir
pub async fn get_model(
    _state: State<Arc<Mutex<VGateState>>>,
    Path(model_id): Path<String>,
) -> (StatusCode, Json<serde_json::Value>) {
    log::info!("📋  MODELS: Model istendi → {}", model_id);
    
    (
        StatusCode::OK,
        Json(json!({
            "id": model_id,
            "object": "model",
            "created": 1700000000,
            "owned_by": model_id.split('/').next().unwrap_or("unknown")
        })),
    )
}
