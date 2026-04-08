//! ─── SENTIENT GATEWAY (İletişim ve Kontrol Katmanı) ───
//!
//! SENTIENT'nın dış dünyayla iletişim merkezi:
//! - HTTP/REST API Gateway
//! - Telegram Bot Köprüsü
//! - WebSocket Gerçek Zamanlı İletişim
//! - JWT Kimlik Doğrulama
//! - Webhook Entegrasyonu (GitHub, Stripe, n8n, Slack)
//! - Event Listener (Otomatik görev tetikleme)
//!
//! ┌─────────────────────────────────────────────────────────────┐
//! │                     GATEWAY                                 │
//! │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐        │
//! │  │  HTTP API   │  │  Telegram   │  │  WebSocket  │        │
//! │  │  (REST)     │  │    Bot      │  │   (WS)      │        │
//! │  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘        │
//! │         │                │                │                │
//! │         └────────────────┼────────────────┘                │
//! │                          ▼                                  │
//! │              ┌─────────────────────┐                       │
//! │              │   Webhook Receiver  │ ◀── GitHub, Stripe    │
//! │              └──────────┬──────────┘     n8n, Slack...     │
//! │                         │                                  │
//! │                         ▼                                  │
//! │              ┌─────────────────────┐                       │
//! │              │   Event Listener    │                       │
//! │              └──────────┬──────────┘                       │
//! │                         │                                  │
//! │                         ▼                                  │
//! │              ┌─────────────────────┐                       │
//! │              │   Task Dispatcher   │                       │
//! │              └──────────┬──────────┘                       │
//! │                         │                                  │
//! │                         ▼                                  │
//! │              ┌─────────────────────┐                       │
//! │              │   ORCHESTRATOR     │                       │
//! │              │   (Agent Loop)     │                       │
//! │              └───────────────────┘                       │
//! └─────────────────────────────────────────────────────────────┘

pub mod api;
pub mod auth;
pub mod telegram;
pub mod websocket;
pub mod webhooks;
pub mod events;
pub mod dashboard;
pub mod claw3d;

mod dispatcher;
mod task_manager;

pub use dispatcher::{TaskDispatcher, DispatchResult};
pub use task_manager::{TaskManager, ManagedTask, TaskStatus as GatewayTaskStatus};

// Webhook exports
pub use webhooks::{
    WebhookReceiver, WebhookConfig, WebhookRouter, WebhookRoute,
    WebhookResult, WebhookStats,
    WebhookProvider, WebhookPayload,
    WebhookEvent, EventType, EventPriority, EventAction,
};

// Event Listener exports
pub use events::{
    EventListener, EventListenerConfig, EventListenerStats,
    EventStatus, TaskRequest,
};

// Dashboard exports
pub use dashboard::{
    SystemMetrics, MetricsCollector, HealthStatus,
    DashboardState, DashboardConfig,
    Activity, ActivitySource, ActivityStatus,
    LogEntry, LogLevel,
    AgentThought,
};

// Claw3D exports (L8: 3D Swarm Görselleştirme)
pub use claw3d::{
    Claw3DState, ClawMessage,
    AgentNode, AgentType, AgentStatus3D,
    TaskEdge, EdgeType, FlowDirection,
    SceneData, SceneStats, CameraConfig,
    MemoryHeat, MemoryDistribution,
    DecisionStep, DecisionType,
    ToolEvent, ToolStatus,
    TimeAction,
};

// Storage exports
pub use sentient_storage::{
    TaskStore, HydrationEngine,
    PersistedTask, PersistedStep, PersistedStatus,
    TaskSnapshot, WorkflowState, TaskLogEntry,
    StorageError, StorageResult,
};

// API exports
pub use api::run_server;

use sentient_common::error::{SENTIENTError, SENTIENTResult};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

/// ─── GATEWAY CONFIG ───

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayConfig {
    /// HTTP API dinleme adresi
    pub http_addr: String,
    
    /// Telegram bot token (opsiyonel)
    pub telegram_token: Option<String>,
    
    /// JWT secret key
    pub jwt_secret: String,
    
    /// API key'ler (hash'lenmiş)
    pub api_keys: Vec<String>,
    
    /// Maksimum eşzamanlı görev
    pub max_concurrent_tasks: usize,
    
    /// Görev timeout (saniye)
    pub task_timeout_secs: u64,
    
    /// CORS izin verilen origins
    pub cors_origins: Vec<String>,
    
    /// Rate limit (istek/dakika per IP)
    pub rate_limit_per_minute: u32,
}

impl Default for GatewayConfig {
    fn default() -> Self {
        Self {
            http_addr: "0.0.0.0:8080".into(),
            telegram_token: None,
            jwt_secret: "sentient-gateway-secret-change-in-production".into(),
            api_keys: vec![],
            max_concurrent_tasks: 10,
            task_timeout_secs: 600, // 10 dakika
            cors_origins: vec!["*".into()],
            rate_limit_per_minute: 60,
        }
    }
}

impl GatewayConfig {
    /// .env dosyasından yükle
    pub fn from_env() -> SENTIENTResult<Self> {
        dotenvy::dotenv().ok();
        
        Ok(Self {
            http_addr: std::env::var("GATEWAY_HTTP_ADDR")
                .unwrap_or_else(|_| "0.0.0.0:8080".into()),
            telegram_token: std::env::var("TELEGRAM_BOT_TOKEN").ok(),
            jwt_secret: std::env::var("JWT_SECRET")
                .unwrap_or_else(|_| "sentient-gateway-secret-change-in-production".into()),
            api_keys: std::env::var("GATEWAY_API_KEYS")
                .map(|s| s.split(',').map(|k| k.trim().to_string()).collect())
                .unwrap_or_default(),
            max_concurrent_tasks: std::env::var("MAX_CONCURRENT_TASKS")
                .map(|s| s.parse().unwrap_or(10))
                .unwrap_or(10),
            task_timeout_secs: std::env::var("TASK_TIMEOUT_SECS")
                .map(|s| s.parse().unwrap_or(600))
                .unwrap_or(600),
            cors_origins: std::env::var("CORS_ORIGINS")
                .map(|s| s.split(',').map(|o| o.trim().to_string()).collect())
                .unwrap_or_else(|_| vec!["*".into()]),
            rate_limit_per_minute: std::env::var("RATE_LIMIT_PER_MINUTE")
                .map(|s| s.parse().unwrap_or(60))
                .unwrap_or(60),
        })
    }
}

/// ─── GATEWAY REQUEST ───

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayRequest {
    /// İstek ID'si
    pub id: uuid::Uuid,
    
    /// İstek kaynağı
    pub source: RequestSource,
    
    /// Kullanıcı ID'si (opsiyonel)
    pub user_id: Option<String>,
    
    /// Hedef açıklaması
    pub goal: String,
    
    /// Kullanılacak model (opsiyonel)
    pub model: Option<String>,
    
    /// Ek parametreler
    pub params: serde_json::Value,
    
    /// Oluşturulma zamanı
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl GatewayRequest {
    pub fn new(goal: impl Into<String>, source: RequestSource) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            source,
            user_id: None,
            goal: goal.into(),
            model: None,
            params: serde_json::json!({}),
            created_at: chrono::Utc::now(),
        }
    }
    
    pub fn with_user(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = Some(user_id.into());
        self
    }
    
    pub fn with_model(mut self, model: impl Into<String>) -> Self {
        self.model = Some(model.into());
        self
    }
}

/// ─── REQUEST SOURCE ───

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RequestSource {
    /// HTTP REST API
    HttpApi { 
        ip: String,
        user_agent: Option<String>,
    },
    
    /// Telegram Bot
    Telegram { 
        chat_id: i64,
        username: Option<String>,
    },
    
    /// WebSocket
    WebSocket { 
        connection_id: String,
    },
    
    /// CLI
    Cli,
    
    /// Internal
    Internal,
}

impl std::fmt::Display for RequestSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::HttpApi { ip, .. } => write!(f, "HTTP({})", ip),
            Self::Telegram { chat_id, .. } => write!(f, "Telegram({})", chat_id),
            Self::WebSocket { connection_id } => write!(f, "WS({})", connection_id),
            Self::Cli => write!(f, "CLI"),
            Self::Internal => write!(f, "Internal"),
        }
    }
}

/// ─── GATEWAY RESPONSE ───

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayResponse {
    /// İstek ID'si
    pub request_id: uuid::Uuid,
    
    /// Görev ID'si
    pub task_id: uuid::Uuid,
    
    /// Durum
    pub status: ResponseStatus,
    
    /// Mesaj
    pub message: String,
    
    /// Sonuç (varsa)
    pub result: Option<serde_json::Value>,
    
    /// İşlem süresi (ms)
    pub duration_ms: u64,
    
    /// Zaman damgası
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ResponseStatus {
    Accepted,
    Processing,
    Completed,
    Failed,
    Cancelled,
    Timeout,
}

/// ─── GATEWAY ───

pub struct Gateway {
    config: GatewayConfig,
    task_dispatcher: Arc<TaskDispatcher>,
    task_manager: Arc<TaskManager>,
    shutdown: Arc<RwLock<bool>>,
}

impl Gateway {
    pub fn new(config: GatewayConfig) -> Self {
        let task_manager = Arc::new(TaskManager::new(
            config.max_concurrent_tasks,
            config.task_timeout_secs,
        ));
        
        let task_dispatcher = Arc::new(TaskDispatcher::new(
            task_manager.clone(),
        ));
        
        Self {
            config,
            task_dispatcher,
            task_manager,
            shutdown: Arc::new(RwLock::new(false)),
        }
    }
    
    /// Gateway'i başlat
    pub async fn start(&self) -> SENTIENTResult<()> {
        log::info!("════════════════════════════════════════════════════════════");
        log::info!("🌐  SENTIENT GATEWAY başlatılıyor...");
        log::info!("════════════════════════════════════════════════════════════");
        
        // Task Manager'ı başlat
        self.task_manager.start().await?;
        
        // HTTP API sunucusunu başlat
        let http_handle = self.start_http_server();
        
        // Telegram bot'unu başlat (eğer token varsa)
        #[cfg(feature = "telegram")]
        let telegram_handle = self.start_telegram_bot();
        
        // Shutdown sinyalini bekle
        let shutdown = self.shutdown.clone();
        tokio::spawn(async move {
            tokio::signal::ctrl_c().await.ok();
            log::info!("🛑  Shutdown sinyali alındı");
            *shutdown.write().await = true;
        });
        
        // Sunucuları çalıştır
        http_handle.await?;
        
        log::info!("👋  Gateway kapatıldı");
        Ok(())
    }
    
    /// Durdur
    pub async fn shutdown(&self) {
        log::info!("🛑  Gateway durduruluyor...");
        *self.shutdown.write().await = true;
        self.task_manager.shutdown().await;
    }
    
    /// HTTP sunucusunu başlat
    async fn start_http_server(&self) -> SENTIENTResult<()> {
        let addr = &self.config.http_addr;
        log::info!("📡  HTTP API dinleniyor: {}", addr);
        
        api::run_server(
            &self.config,
            self.task_dispatcher.clone(),
            self.task_manager.clone(),
        ).await
    }
    
    /// Telegram bot'unu başlat
    #[cfg(feature = "telegram")]
    async fn start_telegram_bot(&self) -> SENTIENTResult<()> {
        if let Some(token) = &self.config.telegram_token {
            log::info!("🤖  Telegram Bot başlatılıyor...");
            telegram::run_bot(
                token,
                self.task_dispatcher.clone(),
                self.task_manager.clone(),
            ).await
        } else {
            log::info!("⏭️  Telegram Bot token yok, atlanıyor");
            Ok(())
        }
    }
    
    /// Görev durumu al
    pub async fn get_task_status(&self, task_id: uuid::Uuid) -> Option<ManagedTask> {
        self.task_manager.get_task(task_id).await
    }
    
    /// Aktif görevleri al
    pub async fn get_active_tasks(&self) -> Vec<ManagedTask> {
        self.task_manager.get_active_tasks().await
    }
    
    /// İstatistikler
    pub async fn stats(&self) -> GatewayStats {
        self.task_manager.stats().await
    }
}

/// ─── GATEWAY STATS ───

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayStats {
    pub total_requests: u64,
    pub active_tasks: usize,
    pub completed_tasks: u64,
    pub failed_tasks: u64,
    pub cancelled_tasks: u64,
    pub uptime_secs: u64,
    pub requests_per_source: std::collections::HashMap<String, u64>,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_gateway_config_default() {
        let config = GatewayConfig::default();
        assert_eq!(config.http_addr, "0.0.0.0:8080");
        assert_eq!(config.max_concurrent_tasks, 10);
        assert!(config.telegram_token.is_none());
    }
    
    #[test]
    fn test_gateway_request_creation() {
        let req = GatewayRequest::new(
            "Test hedefi", 
            RequestSource::Cli
        );
        assert_eq!(req.goal, "Test hedefi");
        assert_eq!(req.source, RequestSource::Cli);
        assert!(req.user_id.is_none());
    }
    
    #[test]
    fn test_gateway_request_with_user() {
        let req = GatewayRequest::new(
            "Test", 
            RequestSource::Telegram { chat_id: 12345, username: Some("test_user".into()) }
        ).with_user("user123");
        
        assert_eq!(req.user_id, Some("user123".into()));
    }
    
    #[test]
    fn test_request_source_display() {
        let src = RequestSource::HttpApi { 
            ip: "127.0.0.1".into(), 
            user_agent: Some("test".into()) 
        };
        assert_eq!(format!("{}", src), "HTTP(127.0.0.1)");
        
        let src = RequestSource::Telegram { chat_id: 12345, username: None };
        assert_eq!(format!("{}", src), "Telegram(12345)");
    }
    
    #[test]
    fn test_gateway_response_serialization() {
        let response = GatewayResponse {
            request_id: uuid::Uuid::new_v4(),
            task_id: uuid::Uuid::new_v4(),
            status: ResponseStatus::Accepted,
            message: "Görev kabul edildi".into(),
            result: None,
            duration_ms: 10,
            timestamp: chrono::Utc::now(),
        };
        
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"status\":\"accepted\""));
        assert!(json.contains("\"message\":\"Görev kabul edildi\""));
    }
}
