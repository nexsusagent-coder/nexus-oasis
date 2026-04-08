//! ─── DASHBOARD HANDLERS ───
//!
//! HTTP API endpoints ve WebSocket handler'ları

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Extension, Json, Path, Query, State,
    },
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Router,
};
use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::Arc,
    time::Duration,
};
use tokio::sync::RwLock;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use super::assets::DashboardAssets;
use crate::api::ApiState;
use crate::dashboard::metrics::{SystemMetrics, MetricsCollector, HealthStatus};

// ═══════════════════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════════════════

/// Aktivite kaydı
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub source: ActivitySource,
    pub title: String,
    pub description: String,
    pub status: ActivityStatus,
    pub duration_ms: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ActivitySource {
    Scout,
    Forge,
    Swarm,
    System,
    User,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ActivityStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
}

impl std::fmt::Display for ActivityStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ActivityStatus::Pending => write!(f, "Bekliyor"),
            ActivityStatus::Running => write!(f, "Çalışıyor"),
            ActivityStatus::Completed => write!(f, "Tamamlandı"),
            ActivityStatus::Failed => write!(f, "Başarısız"),
            ActivityStatus::Cancelled => write!(f, "İptal"),
        }
    }
}

/// Log kaydı
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: DateTime<Utc>,
    pub level: LogLevel,
    pub source: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

/// Ajan düşüncesi (Live Thoughts)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentThought {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub agent: String,
    pub agent_icon: String,
    pub thought: String,
    pub metadata: HashMap<String, String>,
}

/// Görev oluşturma request
#[derive(Debug, Deserialize)]
pub struct CreateTaskRequest {
    pub goal: String,
    #[serde(default)]
    pub agent: Option<String>,
    #[serde(default)]
    pub model: Option<String>,
    #[serde(default = "default_priority")]
    pub priority: u8,
}

fn default_priority() -> u8 { 3 }

/// Görev oluşturma response
#[derive(Debug, Serialize)]
pub struct CreateTaskResponse {
    pub task_id: Uuid,
    pub goal: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

// ═══════════════════════════════════════════════════════════════
// ROUTER
// ═══════════════════════════════════════════════════════════════

/// Dashboard state (ApiState'a dahil edilecek)
#[derive(Clone)]
pub struct DashboardState {
    pub metrics: Arc<MetricsCollector>,
    pub activities: Arc<RwLock<Vec<Activity>>>,
    pub logs: Arc<RwLock<Vec<LogEntry>>>,
    pub thoughts: Arc<RwLock<Vec<AgentThought>>>,
    pub config: DashboardConfig,
}

/// Dashboard yapılandırması
#[derive(Debug, Clone)]
pub struct DashboardConfig {
    pub max_activities: usize,
    pub max_logs: usize,
    pub max_thoughts: usize,
    pub refresh_interval_ms: u64,
}

impl Default for DashboardConfig {
    fn default() -> Self {
        Self {
            max_activities: 100,
            max_logs: 200,
            max_thoughts: 50,
            refresh_interval_ms: 1000,
        }
    }
}

// ═══════════════════════════════════════════════════════════════
// HELPER METHODS
// ═══════════════════════════════════════════════════════════════

impl DashboardState {
    /// Yeni aktivite ekle
    pub async fn add_activity(&self, activity: Activity) {
        let mut activities = self.activities.write().await;
        activities.insert(0, activity);
        
        while activities.len() > self.config.max_activities {
            activities.pop();
        }
    }
    
    /// Yeni log ekle
    pub async fn add_log(&self, level: LogLevel, source: &str, message: &str) {
        let mut logs = self.logs.write().await;
        logs.insert(0, LogEntry {
            timestamp: Utc::now(),
            level,
            source: source.to_string(),
            message: message.to_string(),
        });
        
        while logs.len() > self.config.max_logs {
            logs.pop();
        }
    }
    
    /// Ajan düşüncesi ekle
    pub async fn add_thought(&self, agent: &str, thought: &str, metadata: HashMap<String, String>) {
        let mut thoughts = self.thoughts.write().await;
        
        let (agent_name, agent_icon) = match agent.to_lowercase().as_str() {
            "scout" => ("Scout", "🔍"),
            "forge" => ("Forge", "🔨"),
            "swarm" => ("Swarm", "🐝"),
            "alpha" => ("Alpha", "🅰️"),
            "beta" => ("Beta", "🅱️"),
            "gamma" => ("Gamma", "🇬"),
            "delta" => ("Delta", "🔺"),
            _ => (agent, "🤖"),
        };
        
        thoughts.insert(0, AgentThought {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            agent: agent_name.to_string(),
            agent_icon: agent_icon.to_string(),
            thought: thought.to_string(),
            metadata,
        });
        
        while thoughts.len() > self.config.max_thoughts {
            thoughts.pop();
        }
    }
    
    /// Scout aktivitesi ekle
    pub async fn log_scout(&self, message: &str, status: ActivityStatus) {
        self.add_activity(Activity {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            source: ActivitySource::Scout,
            title: "Scout Taraması".into(),
            description: message.to_string(),
            status,
            duration_ms: None,
        }).await;
        
        self.add_log(LogLevel::Info, "Scout", message).await;
    }
    
    /// Forge aktivitesi ekle
    pub async fn log_forge(&self, message: &str, status: ActivityStatus) {
        self.add_activity(Activity {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            source: ActivitySource::Forge,
            title: "Forge Üretimi".into(),
            description: message.to_string(),
            status,
            duration_ms: None,
        }).await;
        
        self.add_log(LogLevel::Info, "Forge", message).await;
    }
    
    /// Swarm aktivitesi ekle
    pub async fn log_swarm(&self, message: &str, status: ActivityStatus) {
        self.add_activity(Activity {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            source: ActivitySource::Swarm,
            title: "Swarm Diyalogu".into(),
            description: message.to_string(),
            status,
            duration_ms: None,
        }).await;
        
        self.add_log(LogLevel::Info, "Swarm", message).await;
    }
    
    /// Hata logu ekle
    pub async fn log_error(&self, source: &str, message: &str) {
        self.add_log(LogLevel::Error, source, message).await;
    }
    
    /// Uyarı logu ekle
    pub async fn log_warn(&self, source: &str, message: &str) {
        self.add_log(LogLevel::Warn, source, message).await;
    }
}

/// Dashboard router oluştur (ApiState ile)
pub fn create_dashboard_router(_state: DashboardState) -> Router<ApiState> {
    Router::new()
        // Ana sayfa
        .route("/dashboard", get(serve_dashboard))
        .route("/dashboard/", get(serve_dashboard))
        
        // L8: Claw3D - 3D Swarm Görselleştirme
        .route("/claw3d", get(serve_claw3d))
        
        // L7: Memory Bridge Görselleştirme
        .route("/memory", get(serve_memory_bridge))
        
        // PWA dosyaları
        .route("/manifest.json", get(serve_manifest))
        .route("/sw.js", get(serve_service_worker))
        
        // API endpoints
        .route("/api/dashboard", get(get_dashboard_data))
        .route("/api/metrics", get(get_metrics))
        .route("/api/activities", get(get_activities))
        .route("/api/thoughts", get(get_thoughts))
        
        // L9: Browser Sessions API
        .route("/api/browser/sessions", get(list_browser_sessions))
        .route("/api/browser/sessions", post(create_browser_session))
        .route("/api/browser/sessions/:name", get(get_browser_session))
        .route("/api/browser/sessions/:name/auth", get(get_auth_page))
        .route("/api/browser/sessions/:name/auth", post(complete_auth))
        .route("/api/browser/sessions/:name", axum::routing::delete(delete_browser_session))
        .route("/api/logs", get(get_logs))
        .route("/api/logs/stream", get(stream_logs))
        .route("/api/logs/clear", post(clear_logs))
        
        // Görev oluşturma
        .route("/api/task", post(create_task))
        .route("/api/task/:id", get(get_task_status))
        
        // Statik dosyalar
        .route("/style.css", get(serve_css))
        .route("/app.js", get(serve_js))
        
        // WebSocket
        .route("/ws", get(websocket_handler))
        .route("/ws/memory", get(memory_ws_handler))
}

// ═══════════════════════════════════════════════════════════════
// HANDLERS
// ═══════════════════════════════════════════════════════════════

/// Ana dashboard sayfası
async fn serve_dashboard() -> impl IntoResponse {
    let html = DashboardAssets::index_html();
    Html(html)
}

/// CSS dosyası
async fn serve_css() -> impl IntoResponse {
    let css = DashboardAssets::style_css();
    ([(axum::http::header::CONTENT_TYPE, "text/css; charset=utf-8")], css)
}

/// JS dosyası
async fn serve_js() -> impl IntoResponse {
    let js = DashboardAssets::app_js();
    ([(axum::http::header::CONTENT_TYPE, "application/javascript; charset=utf-8")], js)
}

/// Claw3D sayfası (L8: 3D Swarm Görselleştirme)
async fn serve_claw3d() -> impl IntoResponse {
    let html = DashboardAssets::claw3d_html();
    Html(html)
}

/// Memory Bridge sayfası (L7: Bellek Görselleştirme)
async fn serve_memory_bridge() -> impl IntoResponse {
    let html = DashboardAssets::memory_bridge_html();
    Html(html)
}

/// PWA Manifest
async fn serve_manifest() -> impl IntoResponse {
    let json = DashboardAssets::manifest_json();
    ([(axum::http::header::CONTENT_TYPE, "application/manifest+json; charset=utf-8")], json)
}

/// Service Worker
async fn serve_service_worker() -> impl IntoResponse {
    let js = DashboardAssets::service_worker_js();
    (
        [
            (axum::http::header::CONTENT_TYPE, "application/javascript; charset=utf-8"),
            (axum::http::header::CACHE_CONTROL, "no-cache"),
        ],
        js
    )
}

/// Dashboard verisi al
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
    state.logs.write().await.push(LogEntry {
        timestamp: Utc::now(),
        level: LogLevel::Info,
        source: "System".into(),
        message: "Loglar temizlendi".into(),
    });
    Json(serde_json::json!({"status": "cleared"}))
}

/// Canlı log akışı (SSE)
async fn stream_logs(State(state): State<ApiState>) -> impl IntoResponse {
    let logs = state.logs.read().await.clone();
    let body = logs.iter()
        .map(|log| format!("data: {}\n\n", serde_json::to_string(log).unwrap_or_default()))
        .collect::<Vec<_>>()
        .join("");
    
    Response::builder()
        .header("Content-Type", "text/event-stream")
        .header("Cache-Control", "no-cache")
        .header("Connection", "keep-alive")
        .body(body)
        .unwrap()
        .into_response()
}

/// Görev oluştur
async fn create_task(
    State(state): State<ApiState>,
    Json(req): Json<CreateTaskRequest>,
) -> impl IntoResponse {
    let task_id = Uuid::new_v4();
    
    // Add activity
    let activity = Activity {
        id: task_id,
        timestamp: Utc::now(),
        source: match req.agent.as_deref() {
            Some("scout") => ActivitySource::Scout,
            Some("forge") => ActivitySource::Forge,
            Some("swarm") => ActivitySource::Swarm,
            _ => ActivitySource::User,
        },
        title: "Yeni Görev".into(),
        description: req.goal.clone(),
        status: ActivityStatus::Pending,
        duration_ms: None,
    };
    
    state.activities.write().await.insert(0, activity);
    
    // Add log
    state.logs.write().await.insert(0, LogEntry {
        timestamp: Utc::now(),
        level: LogLevel::Info,
        source: "API".into(),
        message: format!("Yeni görev oluşturuldu: {}", req.goal.chars().take(50).collect::<String>()),
    });
    
    Json(CreateTaskResponse {
        task_id,
        goal: req.goal,
        status: "pending".into(),
        created_at: Utc::now(),
    })
}

/// Görev durumu
async fn get_task_status(
    State(state): State<ApiState>,
    Path(task_id): Path<Uuid>,
) -> impl IntoResponse {
    let activities = state.activities.read().await;
    let activity = activities.iter().find(|a| a.id == task_id);
    
    Json(serde_json::json!({
        "task_id": task_id,
        "status": activity.map(|a| a.status.to_string()).unwrap_or_else(|| "not_found".into())
    }))
}

/// WebSocket handler
async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<ApiState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_websocket(socket, state))
}

/// WebSocket bağlantı işleyicisi
async fn handle_websocket(socket: WebSocket, state: ApiState) {
    let (mut sender, mut receiver) = socket.split();
    
    // Initial metrics
    let metrics = state.metrics.collect().await;
    let initial_msg = serde_json::json!({
        "type": "metrics",
        "payload": metrics
    });
    
    let _ = sender.send(Message::Text(initial_msg.to_string())).await;
    
    // Message handling loop
    while let Some(msg) = futures::StreamExt::next(&mut receiver).await {
        match msg {
            Ok(Message::Text(text)) => {
                if let Ok(data) = serde_json::from_str::<serde_json::Value>(&text) {
                    match data.get("type").and_then(|t| t.as_str()) {
                        Some("subscribe") => {
                            state.logs.write().await.insert(0, LogEntry {
                                timestamp: Utc::now(),
                                level: LogLevel::Info,
                                source: "WS".into(),
                                message: "Kanallara abone olundu".into(),
                            });
                        }
                        Some("create_task") => {
                            if let Some(payload) = data.get("payload") {
                                if let Ok(req) = serde_json::from_value::<CreateTaskRequest>(payload.clone()) {
                                    let task_id = Uuid::new_v4();
                                    state.logs.write().await.insert(0, LogEntry {
                                        timestamp: Utc::now(),
                                        level: LogLevel::Info,
                                        source: "WS".into(),
                                        message: format!("WebSocket'ten görev: {}", req.goal),
                                    });
                                    
                                    let response = serde_json::json!({
                                        "type": "task_created",
                                        "payload": {
                                            "task_id": task_id,
                                            "goal": req.goal
                                        }
                                    });
                                    let _ = sender.send(Message::Text(response.to_string())).await;
                                }
                            }
                        }
                        Some("ping") => {
                            let _ = sender.send(Message::Text(r#"{\"type\":\"pong\"}\""#.into())).await;
                        }
                        _ => {}
                    }
                }
            }
            Ok(Message::Ping(data)) => {
                let _ = sender.send(Message::Pong(data)).await;
            }
            Ok(Message::Close(_)) | Err(_) => break,
            _ => {}
        }
    }
}

/// Memory Bridge WebSocket handler
async fn memory_ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<ApiState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_memory_websocket(socket, state))
}

/// Memory WebSocket bağlantı işleyicisi
async fn handle_memory_websocket(socket: WebSocket, state: ApiState) {
    let (mut sender, mut receiver) = socket.split();
    
    // Initial memory stats
    let memory_stats = serde_json::json!({
        "type": "memory_stats",
        "episodic": 0,
        "semantic": 0,
        "procedural": 0,
        "working": 0
    });
    
    let _ = sender.send(Message::Text(memory_stats.to_string())).await;
    
    // Periodic updates
    let mut update_interval = tokio::time::interval(std::time::Duration::from_secs(5));
    
    loop {
        tokio::select! {
            _ = update_interval.tick() => {
                let stats = serde_json::json!({
                    "type": "memory_stats",
                    "episodic": 0,
                    "semantic": 0,
                    "procedural": 0,
                    "working": 0
                });
                if sender.send(Message::Text(stats.to_string())).await.is_err() {
                    break;
                }
            }
            Some(msg) = futures::StreamExt::next(&mut receiver) => {
                match msg {
                    Ok(Message::Close(_)) | Err(_) => break,
                    Ok(Message::Ping(data)) => {
                        let _ = sender.send(Message::Pong(data)).await;
                    }
                    _ => {}
                }
            }
        }
    }
}

// ═══════════════════════════════════════════════════════════════
// BROWSER SESSIONS API
// ═══════════════════════════════════════════════════════════════

/// Browser oturum listesi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserSessionInfo {
    pub name: String,
    pub domain: String,
    pub is_authenticated: bool,
    pub last_used: DateTime<Utc>,
    pub status: String,
    pub tags: Vec<String>,
}

/// Tüm tarayıcı oturumlarını listele
async fn list_browser_sessions(
    State(_state): State<ApiState>,
) -> impl IntoResponse {
    // Mock data - gerçek impl'de ProfileManager'dan gelecek
    let sessions = vec![
        BrowserSessionInfo {
            name: "twitter".to_string(),
            domain: "x.com".to_string(),
            is_authenticated: false,
            last_used: Utc::now() - chrono::Duration::hours(24),
            status: "pending_auth".to_string(),
            tags: vec!["social".to_string()],
        },
        BrowserSessionInfo {
            name: "linkedin".to_string(),
            domain: "linkedin.com".to_string(),
            is_authenticated: false,
            last_used: Utc::now() - chrono::Duration::days(7),
            status: "pending_auth".to_string(),
            tags: vec!["professional".to_string()],
        },
        BrowserSessionInfo {
            name: "github".to_string(),
            domain: "github.com".to_string(),
            is_authenticated: true,
            last_used: Utc::now() - chrono::Duration::minutes(30),
            status: "active".to_string(),
            tags: vec!["coding".to_string()],
        },
    ];
    
    Json(serde_json::json!({
        "success": true,
        "sessions": sessions
    }))
}

/// Yeni oturum oluştur
#[derive(Debug, Deserialize)]
pub struct CreateSessionRequest {
    pub name: String,
    pub domain: String,
}

async fn create_browser_session(
    State(_state): State<ApiState>,
    Json(req): Json<CreateSessionRequest>,
) -> impl IntoResponse {
    // Gerçek impl'de ProfileManager::create_profile çağrılacak
    Json(serde_json::json!({
        "success": true,
        "message": format!("Oturum oluşturuldu: {} ({})", req.name, req.domain),
        "auth_url": format!("/api/browser/sessions/{}/auth", req.name)
    }))
}

/// Tek bir oturum bilgisi
async fn get_browser_session(
    Path(name): Path<String>,
    State(_state): State<ApiState>,
) -> impl IntoResponse {
    Json(serde_json::json!({
        "success": true,
        "session": {
            "name": name,
            "domain": "example.com",
            "is_authenticated": false,
            "status": "pending_auth"
        }
    }))
}

/// Auth sayfasını getir (headless browser açılır)
async fn get_auth_page(
    Path(name): Path<String>,
    State(_state): State<ApiState>,
) -> impl IntoResponse {
    // Bu endpoint, browser-use tarafından siteyi açar ve kullanıcı login olabilir
    let domains: HashMap<&str, &str> = [
        ("twitter", "https://x.com/login"),
        ("linkedin", "https://www.linkedin.com/login"),
        ("github", "https://github.com/login"),
        ("google", "https://accounts.google.com"),
        ("facebook", "https://www.facebook.com/login"),
        ("instagram", "https://www.instagram.com/accounts/login/"),
        ("reddit", "https://www.reddit.com/login"),
        ("discord", "https://discord.com/login"),
        ("slack", "https://slack.com/signin"),
        ("notion", "https://www.notion.so/login"),
    ].iter().cloned().collect();
    
    let login_url = domains.get(name.as_str()).unwrap_or(&"https://example.com");
    
    Json(serde_json::json!({
        "success": true,
        "message": format!("Auth sayfası hazır: {}", name),
        "login_url": login_url,
        "instructions": [
            "1. Tarayıcı açılacak (headless=false)",
            "2. Siteye manuel giriş yapın",
            "3. Giriş başarılı olduğunda 'Complete Auth' butonuna basın",
            "4. Cookie ve localStorage şifreli olarak kaydedilecek"
        ]
    }))
}

/// Auth tamamlama
#[derive(Debug, Deserialize)]
pub struct CompleteAuthRequest {
    pub session_name: String,
}

async fn complete_auth(
    Path(name): Path<String>,
    State(_state): State<ApiState>,
) -> impl IntoResponse {
    // Gerçek impl'de:
    // 1. Browser-use'dan cookies/localStorage al
    // 2. ProfileManager::update_cookies çağır
    // 3. ProfileManager::set_authenticated(true)
    
    Json(serde_json::json!({
        "success": true,
        "message": format!("Auth tamamlandı: {}", name),
        "profile_path": format!("~/.sentient/browser_profiles/{}.profile", name)
    }))
}

/// Oturum sil
async fn delete_browser_session(
    Path(name): Path<String>,
    State(_state): State<ApiState>,
) -> impl IntoResponse {
    Json(serde_json::json!({
        "success": true,
        "message": format!("Oturum silindi: {}", name)
    }))
}
