//! Settings API - Ayarlar API endpoint'leri

use axum::{
    extract::State,
    routing::{get, post, put, delete},
    Json, Router,
};
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::SettingsManager;

pub fn settings_routes(manager: Arc<RwLock<SettingsManager>>) -> Router {
    Router::new()
        .route("/api/settings", get(get_settings).post(update_settings))
        .route("/api/settings/:category", get(get_category).put(update_category))
        .route("/api/settings/:key", put(set_value))
        .route("/api/settings/reset", post(reset_settings))
        .route("/api/settings/export", get(export_settings))
        .route("/api/settings/import", post(import_settings))
        .with_state(manager)
}

/// Tüm ayarları al
async fn get_settings(
    State(manager): State<Arc<RwLock<SettingsManager>>>,
) -> Json<crate::Settings> {
    let mgr = manager.read().await;
    Json(mgr.get().await)
}

/// Ayarları güncelle
async fn update_settings(
    State(manager): State<Arc<RwLock<SettingsManager>>>,
    Json(settings): Json<crate::Settings>,
) -> Json<serde_json::Value> {
    let mgr = manager.read().await;
    // Tüm ayarları güncelle
    mgr.update(|s| *s = settings).await.ok();
    Json(serde_json::json!({"success": true}))
}

/// Kategori ayarlarını al
async fn get_category(
    State(manager): State<Arc<RwLock<SettingsManager>>>,
    axum::extract::Path(category): axum::extract::Path<String>,
) -> Json<serde_json::Value> {
    let mgr = manager.read().await;
    let settings = mgr.get().await;
    
    let value = match category.as_str() {
        "general" => serde_json::to_value(&settings.general),
        "llm" => serde_json::to_value(&settings.llm),
        "security" => serde_json::to_value(&settings.security),
        "automation" => serde_json::to_value(&settings.automation),
        "integrations" => serde_json::to_value(&settings.integrations),
        "memory" => serde_json::to_value(&settings.memory),
        _ => return Json(serde_json::json!({"error": "Unknown category"})),
    };
    
    Json(value.unwrap_or_default())
}

/// Kategori ayarlarını güncelle
async fn update_category(
    State(manager): State<Arc<RwLock<SettingsManager>>>,
    axum::extract::Path(category): axum::extract::Path<String>,
    Json(value): Json<Value>,
) -> Json<serde_json::Value> {
    let mgr = manager.read().await;
    
    let result = match category.as_str() {
        "general" => {
            if let Ok(v) = serde_json::from_value(value) {
                mgr.update(|s| s.general = v).await.ok();
                true
            } else {
                false
            }
        }
        "llm" => {
            if let Ok(v) = serde_json::from_value(value) {
                mgr.update(|s| s.llm = v).await.ok();
                true
            } else {
                false
            }
        }
        _ => false,
    };
    
    Json(serde_json::json!({"success": result}))
}

/// Tek bir değeri ayarla
async fn set_value(
    State(manager): State<Arc<RwLock<SettingsManager>>>,
    axum::extract::Path(key): axum::extract::Path<String>,
    Json(value): Json<Value>,
) -> Json<serde_json::Value> {
    let mgr = manager.read().await;
    match mgr.set(&key, value).await {
        Ok(_) => Json(serde_json::json!({"success": true})),
        Err(e) => Json(serde_json::json!({"success": false, "error": e.to_string()})),
    }
}

/// Ayarları sıfırla
async fn reset_settings(
    State(manager): State<Arc<RwLock<SettingsManager>>>,
) -> Json<serde_json::Value> {
    let mgr = manager.read().await;
    mgr.reset().await.ok();
    Json(serde_json::json!({"success": true}))
}

/// Ayarları dışa aktar
async fn export_settings(
    State(manager): State<Arc<RwLock<SettingsManager>>>,
) -> Json<crate::Settings> {
    let mgr = manager.read().await;
    Json(mgr.get().await)
}

/// Ayarları içe aktar
async fn import_settings(
    State(manager): State<Arc<RwLock<SettingsManager>>>,
    Json(settings): Json<crate::Settings>,
) -> Json<serde_json::Value> {
    let mgr = manager.read().await;
    mgr.update(|s| *s = settings).await.ok();
    Json(serde_json::json!({"success": true}))
}
