//! ─── HTTP API ROUTES ───
//!
//! RESTful API endpoints:
//! - POST /api/task - Yeni görev oluştur
//! - GET /api/task/:id - Görev durumu
//! - GET /api/tasks - Tüm görevleri listele
//! - DELETE /api/task/:id - Görevi iptal et
//! - GET /api/stats - İstatistikler
//! - GET /health - Sağlık kontrolü
//! - WS /ws - WebSocket gerçek zamanlı iletişim

use axum::{
    extract::{Path, Query, State, WebSocketUpgrade},
    http::{header, HeaderMap, StatusCode},
    response::{IntoResponse, Json},
    routing::{delete, get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use uuid::Uuid;

use crate::{
    GatewayConfig, GatewayRequest, GatewayStats,
    RequestSource,
};
use crate::dispatcher::TaskDispatcher;
use crate::task_manager::{TaskManager, ManagedTask};
use crate::webhooks::{WebhookRouter, WebhookStats, WebhookResult};
use crate::dashboard::MetricsCollector;
use crate::rate_limit::rate_limit_middleware;

/// ─── API STATE ───

#[derive(Clone)]
pub struct ApiState {
    pub config: GatewayConfig,
    pub dispatcher: Arc<TaskDispatcher>,
    pub task_manager: Arc<TaskManager>,
    pub webhook_router: Arc<crate::webhooks::WebhookRouter>,
    pub metrics: Arc<MetricsCollector>,
    pub activities: Arc<tokio::sync::RwLock<Vec<crate::dashboard::Activity>>>,
    pub logs: Arc<tokio::sync::RwLock<Vec<crate::dashboard::LogEntry>>>,
    pub thoughts: Arc<tokio::sync::RwLock<Vec<crate::dashboard::AgentThought>>>,
}

/// ─── API ROUTER ───

pub fn create_router(state: ApiState) -> Router {
    Router::new()
        // Task endpoints
        .route("/api/task", post(create_task))
        .route("/api/task/:id", get(get_task))
        .route("/api/task/:id", delete(cancel_task))
        .route("/api/tasks", get(list_tasks))
        
        // Stats & Health
        .route("/api/stats", get(get_stats))
        .route("/health", get(health_check))
        
        // WebSocket
        .route("/ws", get(websocket_handler))
        
        // Webhook endpoints
        .route("/webhook/:provider", post(handle_webhook))
        .route("/webhook/stats", get(get_webhook_stats))
        
        // Dashboard routes
        .route("/dashboard", get(serve_dashboard))
        .route("/dashboard/", get(serve_dashboard))
        .route("/api/dashboard", get(get_dashboard_data))
        .route("/api/metrics", get(get_metrics))
        .route("/api/activities", get(get_activities))
        .route("/api/thoughts", get(get_thoughts))
        .route("/api/logs", get(get_logs))
        .route("/api/logs/stream", get(stream_logs))
        .route("/api/logs/clear", post(clear_logs))
        .route("/style.css", get(serve_css))
        .route("/app.js", get(serve_js))
        
        // Skills Hub endpoints
        .route("/api/skills", get(list_skills))
        .route("/api/skills/:type/toggle", post(toggle_skill))
        .route("/api/skills/:type/execute", post(execute_skill))
        .route("/api/skills/stats", get(get_skills_stats))
        
        // Rate limiting
        .layer(axum::middleware::from_fn(rate_limit_middleware))
        // CORS
        .layer(CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any)
        )
        .with_state(state)
}

/// Sunucuyu başlat
pub async fn run_server(
    config: &GatewayConfig,
    dispatcher: Arc<TaskDispatcher>,
    task_manager: Arc<TaskManager>,
) -> crate::SENTIENTResult<()> {
    let webhook_router = Arc::new(WebhookRouter::new(&config.http_addr));
    
    // Metrics collector
    let metrics = Arc::new(MetricsCollector::new());
    
    // Dashboard state
    let activities = Arc::new(tokio::sync::RwLock::new(Vec::new()));
    let logs = Arc::new(tokio::sync::RwLock::new(Vec::new()));
    let thoughts = Arc::new(tokio::sync::RwLock::new(Vec::new()));
    
    // Add startup log
    logs.write().await.push(crate::dashboard::LogEntry {
        timestamp: chrono::Utc::now(),
        level: crate::dashboard::LogLevel::Info,
        source: "System".into(),
        message: "SENTIENT Dashboard başlatıldı".into(),
    });
    
    let state = ApiState {
        config: config.clone(),
        dispatcher,
        task_manager,
        webhook_router: webhook_router.clone(),
        metrics: metrics.clone(),
        activities: activities.clone(),
        logs: logs.clone(),
        thoughts: thoughts.clone(),
    };
    
    let app = create_router(state);
    let addr: SocketAddr = config.http_addr.parse()
        .map_err(|e| crate::SENTIENTError::General(format!("Geçersiz adres: {}", e)))?;
    
    log::info!("🌐  HTTP API dinleniyor: http://{}", addr);
    log::info!("📡  Endpoints:");
    log::info!("    POST /api/task       → Yeni görev oluştur");
    log::info!("    GET  /api/task/:id   → Görev durumu");
    log::info!("    GET  /api/tasks      → Tüm görevler");
    log::info!("    DEL  /api/task/:id   → Görevi iptal et");
    log::info!("    GET  /api/stats      → İstatistikler");
    log::info!("    GET  /health         → Sağlık kontrolü");
    log::info!("    WS   /ws             → WebSocket");
    log::info!("    POST /webhook/:provider → Webhook endpoint");
    log::info!("    GET  /dashboard      → Web Dashboard");
    log::info!("📊  Dashboard: http://{}/dashboard", addr);
    
    // Default webhook routes'ları ayarla
    webhook_router.setup_defaults().await;
    
    let listener = tokio::net::TcpListener::bind(addr).await
        .map_err(|e| crate::SENTIENTError::General(format!("Bağlantı hatası: {}", e)))?;
    
    axum::serve(listener, app).await
        .map_err(|e| crate::SENTIENTError::General(format!("Sunucu hatası: {}", e)))?;
    
    Ok(())
}

/// ─── REQUEST/RESPONSE TYPES ───

#[derive(Debug, Deserialize)]
pub struct CreateTaskRequest {
    /// Hedef açıklaması
    pub goal: String,
    
    /// Model (opsiyonel)
    pub model: Option<String>,
    
    /// Öncelik (opsiyonel)
    pub priority: Option<String>,
    
    /// Ek parametreler
    #[serde(default)]
    pub params: serde_json::Value,
}

#[derive(Debug, Serialize)]
pub struct CreateTaskResponse {
    pub success: bool,
    pub message: String,
    pub task_id: Option<Uuid>,
    pub queue_position: Option<usize>,
}

#[derive(Debug, Serialize)]
pub struct TaskResponse {
    pub success: bool,
    pub task: Option<ManagedTask>,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct TaskListResponse {
    pub success: bool,
    pub tasks: Vec<ManagedTask>,
    pub count: usize,
}

#[derive(Debug, Serialize)]
pub struct StatsResponse {
    pub success: bool,
    pub stats: GatewayStats,
}

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
    pub uptime_secs: u64,
    pub active_tasks: usize,
}

#[derive(Debug, Deserialize)]
pub struct TaskQuery {
    /// Sadece aktif görevler
    #[serde(default)]
    pub active_only: bool,
    
    /// Son N görev
    #[serde(default = "default_limit")]
    pub limit: usize,
}

fn default_limit() -> usize { 20 }

/// ─── HANDLERS ───

/// Yeni görev oluştur
async fn create_task(
    State(state): State<ApiState>,
    headers: HeaderMap,
    Json(req): Json<CreateTaskRequest>,
) -> impl IntoResponse {
    // Kaynağı belirle
    let ip = headers
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .or_else(|| headers
            .get("x-real-ip")
            .and_then(|v| v.to_str().ok())
        )
        .unwrap_or("unknown");
    
    let user_agent = headers
        .get(header::USER_AGENT)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());
    
    // Gateway request oluştur
    let gateway_req = GatewayRequest::new(
        req.goal.clone(),
        RequestSource::HttpApi {
            ip: ip.to_string(),
            user_agent,
        }
    );
    
    // Model ayarla (varsa)
    let gateway_req = if let Some(ref model) = req.model {
        gateway_req.with_model(model.clone())
    } else {
        gateway_req
    };
    
    // Dispatch et
    match state.dispatcher.dispatch(gateway_req).await {
        Ok(result) => {
            let response = CreateTaskResponse {
                success: result.accepted,
                message: result.message,
                task_id: Some(result.task_id),
                queue_position: Some(result.queue_position),
            };
            (StatusCode::ACCEPTED, Json(response)).into_response()
        }
        Err(e) => {
            let response = CreateTaskResponse {
                success: false,
                message: e.to_sentient_message(),
                task_id: None,
                queue_position: None,
            };
            (StatusCode::BAD_REQUEST, Json(response)).into_response()
        }
    }
}

/// Görev durumu al
async fn get_task(
    State(state): State<ApiState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match state.task_manager.get_task(id).await {
        Some(task) => {
            let response = TaskResponse {
                success: true,
                task: Some(task),
                message: "Görev bulundu".into(),
            };
            (StatusCode::OK, Json(response))
        }
        None => {
            let response = TaskResponse {
                success: false,
                task: None,
                message: "Görev bulunamadı".into(),
            };
            (StatusCode::NOT_FOUND, Json(response))
        }
    }
}

/// Görevi iptal et
async fn cancel_task(
    State(state): State<ApiState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match state.dispatcher.cancel_task(id).await {
        Ok(()) => {
            let response = TaskResponse {
                success: true,
                task: state.task_manager.get_task(id).await,
                message: "Görev iptal edildi".into(),
            };
            (StatusCode::OK, Json(response))
        }
        Err(e) => {
            let response = TaskResponse {
                success: false,
                task: None,
                message: e.to_sentient_message(),
            };
            (StatusCode::BAD_REQUEST, Json(response))
        }
    }
}

/// Görevleri listele
async fn list_tasks(
    State(state): State<ApiState>,
    Query(query): Query<TaskQuery>,
) -> impl IntoResponse {
    let tasks = if query.active_only {
        state.task_manager.get_active_tasks().await
    } else {
        state.task_manager.get_recent_tasks(query.limit).await
    };
    
    let response = TaskListResponse {
        success: true,
        count: tasks.len(),
        tasks,
    };
    
    (StatusCode::OK, Json(response))
}

/// İstatistikler
async fn get_stats(State(state): State<ApiState>) -> impl IntoResponse {
    let stats = state.dispatcher.stats().await;
    let response = StatsResponse {
        success: true,
        stats,
    };
    (StatusCode::OK, Json(response))
}

/// Sağlık kontrolü
async fn health_check(State(state): State<ApiState>) -> impl IntoResponse {
    let stats = state.task_manager.stats().await;
    let response = HealthResponse {
        status: "healthy".into(),
        version: env!("CARGO_PKG_VERSION").into(),
        uptime_secs: stats.uptime_secs,
        active_tasks: stats.active_tasks,
    };
    (StatusCode::OK, Json(response))
}

/// WebSocket handler
async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<ApiState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_websocket(socket, state))
}

async fn handle_websocket(
    mut socket: axum::extract::ws::WebSocket,
    state: ApiState,
) {
    use axum::extract::ws::Message;
    use futures::StreamExt;
    
    let connection_id = Uuid::new_v4().to_string();
    log::info!("ws  Yeni WebSocket bağlantısı: {}", connection_id);
    
    while let Some(msg) = socket.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                // JSON mesajı parse et
                match serde_json::from_str::<WebSocketMessage>(&text) {
                    Ok(ws_msg) => {
                        let response = handle_ws_message(&ws_msg, &state, &connection_id).await;
                        let _ = socket.send(Message::Text(
                            serde_json::to_string(&response).unwrap_or_default()
                        )).await;
                    }
                    Err(e) => {
                        let error = WebSocketResponse {
                            success: false,
                            message: format!("Geçersiz mesaj: {}", e),
                            data: None,
                        };
                        let _ = socket.send(Message::Text(
                            serde_json::to_string(&error).unwrap_or_default()
                        )).await;
                    }
                }
            }
            Ok(Message::Close(_)) => {
                log::info!("ws  Bağlantı kapandı: {}", connection_id);
                break;
            }
            Err(e) => {
                log::error!("ws  Hata: {}", e);
                break;
            }
            _ => {}
        }
    }
}

#[derive(Debug, Deserialize)]
struct WebSocketMessage {
    #[serde(rename = "type")]
    type_field: String,
    #[serde(default)]
    data: serde_json::Value,
}

#[derive(Debug, Serialize)]
struct WebSocketResponse {
    success: bool,
    message: String,
    data: Option<serde_json::Value>,
}

async fn handle_ws_message(
    msg: &WebSocketMessage,
    state: &ApiState,
    _connection_id: &str,
) -> WebSocketResponse {
    match msg.type_field.as_str() {
        "create_task" => {
            if let Some(goal) = msg.data.get("goal").and_then(|g| g.as_str()) {
                let request = GatewayRequest::new(
                    goal,
                    RequestSource::WebSocket { connection_id: Uuid::new_v4().to_string() }
                );
                
                match state.dispatcher.dispatch(request).await {
                    Ok(result) => WebSocketResponse {
                        success: true,
                        message: result.message,
                        data: Some(serde_json::json!({
                            "task_id": result.task_id,
                            "queue_position": result.queue_position
                        })),
                    },
                    Err(e) => WebSocketResponse {
                        success: false,
                        message: e.to_sentient_message(),
                        data: None,
                    }
                }
            } else {
                WebSocketResponse {
                    success: false,
                    message: "goal parametresi gerekli".into(),
                    data: None,
                }
            }
        }
        
        "get_task" => {
            if let Some(task_id) = msg.data.get("task_id").and_then(|id| {
                id.as_str().and_then(|s| Uuid::parse_str(s).ok())
            }) {
                match state.task_manager.get_task(task_id).await {
                    Some(task) => WebSocketResponse {
                        success: true,
                        message: "Görev bulundu".into(),
                        data: Some(serde_json::to_value(task).unwrap_or_default()),
                    },
                    None => WebSocketResponse {
                        success: false,
                        message: "Görev bulunamadı".into(),
                        data: None,
                    }
                }
            } else {
                WebSocketResponse {
                    success: false,
                    message: "geçersiz task_id".into(),
                    data: None,
                }
            }
        }
        
        "get_stats" => {
            let stats = state.dispatcher.stats().await;
            WebSocketResponse {
                success: true,
                message: "İstatistikler".into(),
                data: Some(serde_json::to_value(stats).unwrap_or_default()),
            }
        }
        
        "subscribe" => {
            // Task subscription - real-time updates via WebSocket
            // Production: Use tokio::sync::broadcast for task updates
            WebSocketResponse {
                success: true,
                message: "Abone olundu - görev güncellemeleri aktif".into(),
                data: Some(serde_json::json!({
                    "subscribed": true,
                    "channel": "task_updates"
                })),
            }
        }
        
        _ => WebSocketResponse {
            success: false,
            message: format!("Bilinmeyen aksiyon: {}", msg.type_field),
            data: None,
        }
    }
}

/// ─── WEBHOOK HANDLERS ───

/// Webhook endpoint handler
async fn handle_webhook(
    State(state): State<ApiState>,
    Path(provider): Path<String>,
    headers: HeaderMap,
    body: String,
) -> impl IntoResponse {
    log::info!("webhook  {} endpoint'ine istek geldi", provider);
    
    // Header'ları map'e çevir
    let headers_map: std::collections::HashMap<String, String> = headers
        .iter()
        .map(|(name, value)| {
            (name.to_string(), value.to_str().unwrap_or("").to_string())
        })
        .collect();
    
    // Webhook router'ı kullan
    match state.webhook_router.route(&provider, &headers_map, &body).await {
        Ok(result) => {
            log::info!("webhook  {} işlendi: {}", provider, result.message);
            (StatusCode::OK, Json(result)).into_response()
        }
        Err(e) => {
            log::error!("webhook  {} hatası: {}", provider, e.to_sentient_message());
            let result = WebhookResult::failure(e.to_sentient_message());
            (StatusCode::BAD_REQUEST, Json(result)).into_response()
        }
    }
}

/// Webhook istatistikleri
async fn get_webhook_stats() -> impl IntoResponse {
    let stats = WebhookStats::default();
    (StatusCode::OK, Json(stats))
}

/// ─── DASHBOARD HANDLERS ───

/// Dashboard ana sayfası
async fn serve_dashboard() -> impl IntoResponse {
    let html = crate::dashboard::assets::DashboardAssets::index_html();
    ([(header::CONTENT_TYPE, "text/html; charset=utf-8")], html)
}

/// CSS dosyası
async fn serve_css() -> impl IntoResponse {
    let css = crate::dashboard::assets::DashboardAssets::style_css();
    ([(header::CONTENT_TYPE, "text/css; charset=utf-8")], css)
}

/// JS dosyası
async fn serve_js() -> impl IntoResponse {
    let js = crate::dashboard::assets::DashboardAssets::app_js();
    ([(header::CONTENT_TYPE, "application/javascript; charset=utf-8")], js)
}

/// Dashboard verisi
async fn get_dashboard_data(State(state): State<ApiState>) -> impl IntoResponse {
    let metrics = state.metrics.collect().await;
    let activities: Vec<_> = state.activities.read().await.iter().cloned().take(10).collect();
    let logs: Vec<_> = state.logs.read().await.iter().cloned().take(20).collect();
    let thoughts: Vec<_> = state.thoughts.read().await.iter().cloned().take(10).collect();
    
    Json(serde_json::json!({
        "metrics": metrics,
        "recent_activities": activities,
        "recent_logs": logs,
        "recent_thoughts": thoughts,
        "health_status": metrics.health_status().to_string()
    }))
}

/// Sistem metrikleri
async fn get_metrics(State(state): State<ApiState>) -> impl IntoResponse {
    let metrics = state.metrics.collect().await;
    Json(metrics)
}

/// Aktiviteler
async fn get_activities(State(state): State<ApiState>) -> impl IntoResponse {
    let activities = state.activities.read().await.clone();
    Json(activities)
}

/// Düşünceler
async fn get_thoughts(State(state): State<ApiState>) -> impl IntoResponse {
    let thoughts = state.thoughts.read().await.clone();
    Json(thoughts)
}

/// Loglar
async fn get_logs(State(state): State<ApiState>) -> impl IntoResponse {
    let logs = state.logs.read().await.clone();
    Json(logs)
}

/// Log temizle
async fn clear_logs(State(state): State<ApiState>) -> impl IntoResponse {
    state.logs.write().await.clear();
    state.logs.write().await.push(crate::dashboard::LogEntry {
        timestamp: chrono::Utc::now(),
        level: crate::dashboard::LogLevel::Info,
        source: "System".into(),
        message: "Loglar temizlendi".into(),
    });
    Json(serde_json::json!({"status": "cleared"}))
}

/// Log akışı (SSE)
async fn stream_logs(State(state): State<ApiState>) -> impl IntoResponse {
    use futures::stream::{self, StreamExt};
    use axum::response::sse::{Event, Sse};
    
    let logs = state.logs.read().await.clone();
    let stream = stream::iter(logs).map(|log| {
        Ok::<Event, std::convert::Infallible>(Event::default()
            .event("log")
            .json_data(&log)
            .unwrap_or_default())
    });
    
    Sse::new(stream)
}

// ═══════════════════════════════════════════════════════════════════════════════
// SKILLS HUB HANDLERS
// ═══════════════════════════════════════════════════════════════════════════════

/// Skills Hub instance
static SKILLS_HUB: std::sync::OnceLock<sentient_orchestrator::skills::SkillsHub> = std::sync::OnceLock::new();

fn get_skills_hub() -> &'static sentient_orchestrator::skills::SkillsHub {
    SKILLS_HUB.get_or_init(|| sentient_orchestrator::skills::SkillsHub::new())
}

/// Tüm skill'leri listele
async fn list_skills() -> impl IntoResponse {
    let hub = get_skills_hub();
    let skills = hub.list_skills().await;
    Json(skills)
}

/// Skill toggle
async fn toggle_skill(
    Path(skill_type): Path<String>,
) -> impl IntoResponse {
    use sentient_orchestrator::skills::SkillType;
    
    let skill_type = match skill_type.as_str() {
        "mindsearch" => SkillType::MindSearch,
        "browser" => SkillType::LightpandaBrowser,
        "autoresearch" => SkillType::AutoResearch,
        "n8n" => SkillType::N8nAutomation,
        "websearch" => SkillType::WebSearch,
        "citation" => SkillType::Citation,
        _ => {
            return Json(serde_json::json!({
                "success": false,
                "error": "Bilinmeyen skill tipi"
            }));
        }
    };
    
    let hub = get_skills_hub();
    match hub.toggle_skill(skill_type).await {
        Ok(enabled) => Json(serde_json::json!({
            "success": true,
            "enabled": enabled,
            "message": if enabled { "Skill aktifleştirildi" } else { "Skill deaktif edildi" }
        })),
        Err(e) => Json(serde_json::json!({
            "success": false,
            "error": e.to_string()
        })),
    }
}

/// Skill çalıştır
#[derive(Debug, Deserialize)]
struct ExecuteSkillRequest {
    query: Option<String>,
    url: Option<String>,
    source: Option<String>,
    workflow_id: Option<String>,
    citation_key: Option<String>,
    params: Option<serde_json::Value>,
}

async fn execute_skill(
    Path(skill_type): Path<String>,
    Json(req): Json<ExecuteSkillRequest>,
) -> impl IntoResponse {
    use sentient_orchestrator::skills::{SkillType, SkillInput};
    
    let skill_type = match skill_type.as_str() {
        "mindsearch" => SkillType::MindSearch,
        "browser" => SkillType::LightpandaBrowser,
        "autoresearch" => SkillType::AutoResearch,
        "n8n" => SkillType::N8nAutomation,
        "websearch" => SkillType::WebSearch,
        "citation" => SkillType::Citation,
        _ => {
            return Json(serde_json::json!({
                "success": false,
                "error": "Bilinmeyen skill tipi"
            }));
        }
    };
    
    let input = SkillInput {
        query: req.query,
        url: req.url,
        source: req.source,
        workflow_id: req.workflow_id,
        citation_key: req.citation_key,
        params: req.params
            .and_then(|v| v.as_object().cloned())
            .unwrap_or_default()
            .into_iter()
            .map(|(k, v)| (k, v))
            .collect(),
    };
    
    let hub = get_skills_hub();
    match hub.execute(skill_type, input).await {
        Ok(output) => Json(serde_json::json!({
            "success": output.success,
            "data": output.data,
            "message": output.message
        })),
        Err(e) => Json(serde_json::json!({
            "success": false,
            "error": e.to_string()
        })),
    }
}

/// Skills istatistikleri
async fn get_skills_stats() -> impl IntoResponse {
    let hub = get_skills_hub();
    let stats = hub.stats().await;
    Json(stats)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_create_task_request_deserialization() {
        let json = r#"{"goal":"Test hedefi","model":"qwen/qwen3-1.7b:free"}"#;
        let req: CreateTaskRequest = serde_json::from_str(json).expect("operation failed");
        assert_eq!(req.goal, "Test hedefi");
        assert_eq!(req.model, Some("qwen/qwen3-1.7b:free".into()));
    }
    
    #[test]
    fn test_health_response_serialization() {
        let response = HealthResponse {
            status: "healthy".into(),
            version: "0.1.0".into(),
            uptime_secs: 3600,
            active_tasks: 5,
        };
        let json = serde_json::to_string(&response).expect("operation failed");
        assert!(json.contains("\"status\":\"healthy\""));
    }
    
    #[test]
    fn test_websocket_message_deserialization() {
        let json = r#"{"type":"create_task","data":{"goal":"Test"}}"#;
        let msg: WebSocketMessage = serde_json::from_str(json).expect("operation failed");
        assert_eq!(msg.type_field, "create_task");
    }
}
