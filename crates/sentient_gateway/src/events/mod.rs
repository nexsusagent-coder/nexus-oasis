//! ─── EVENT LISTENER ───
//!
//! Webhook event'lerini dinler ve Orchestrator'ı tetikler.
//! Event -> Task dönüşümü yapar.
//!
//! ┌─────────────────────────────────────────────────────────────┐
//! │                  EVENT LISTENER FLOW                        │
//! │                                                              │
//! │  Webhook ──▶ Event Queue ──▶ Event Listener                │
//! │                                    │                         │
//! │                                    ▼                         │
//! │                              Event Processor                 │
//! │                                    │                         │
//! │                    ┌───────────────┼───────────────┐        │
//! │                    ▼               ▼               ▼        │
//! │              Create Task    Send Notify    Update DB       │
//! │                    │               │                         │
//! │                    ▼               ▼                         │
//! │              Orchestrator    Webhook Response               │
//! └─────────────────────────────────────────────────────────────┘

use sentient_common::error::{SENTIENTError, SENTIENTResult};
use sentient_orchestrator::{Goal, TaskPriority};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::webhooks::{WebhookEvent, WebhookProvider, EventType, EventAction, EventPriority};

/// ─── EVENT LISTENER CONFIG ───

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventListenerConfig {
    /// Otomatik görev oluşturma aktif mi?
    pub auto_create_tasks: bool,
    
    /// Event buffer boyutu
    pub buffer_size: usize,
    
    /// Maksimum concurrent görev
    pub max_concurrent_tasks: usize,
    
    /// Default model
    pub default_model: String,
    
    /// V-GATE URL
    pub vgate_url: String,
    
    /// Event timeout (saniye)
    pub event_timeout_secs: u64,
    
    /// Provider bazlı aksiyonlar
    pub provider_actions: std::collections::HashMap<String, ProviderAction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderAction {
    /// Otomatik görev oluştur
    pub create_task: bool,
    
    /// Default öncelik
    pub priority: String,
    
    /// Default aksiyon
    pub action: String,
    
    /// Task template (opsiyonel)
    pub task_template: Option<String>,
}

impl Default for EventListenerConfig {
    fn default() -> Self {
        let mut provider_actions = std::collections::HashMap::new();
        
        provider_actions.insert("github".into(), ProviderAction {
            create_task: true,
            priority: "normal".into(),
            action: "analyze".into(),
            task_template: Some("GitHub event: {{event_type}} from {{source}}".into()),
        });
        
        provider_actions.insert("stripe".into(), ProviderAction {
            create_task: true,
            priority: "high".into(),
            action: "log".into(),
            task_template: Some("Stripe payment: {{event_type}}".into()),
        });
        
        provider_actions.insert("n8n".into(), ProviderAction {
            create_task: true,
            priority: "normal".into(),
            action: "execute".into(),
            task_template: None,
        });
        
        provider_actions.insert("slack".into(), ProviderAction {
            create_task: true,
            priority: "normal".into(),
            action: "respond".into(),
            task_template: Some("Slack message in {{source}}: {{message}}".into()),
        });
        
        Self {
            auto_create_tasks: true,
            buffer_size: 1000,
            max_concurrent_tasks: 10,
            default_model: "qwen/qwen3-1.7b:free".into(),
            vgate_url: "http://127.0.0.1:1071".into(),
            event_timeout_secs: 300,
            provider_actions,
        }
    }
}

/// ─── EVENT PROCESSOR ───

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessedEvent {
    /// Event ID
    pub event_id: Uuid,
    
    /// Görev oluşturuldu mu?
    pub task_created: bool,
    
    /// Görev ID (varsa)
    pub task_id: Option<Uuid>,
    
    /// İşlem durumu
    pub status: EventStatus,
    
    /// Mesaj
    pub message: String,
    
    /// İşlem süresi (ms)
    pub duration_ms: u64,
    
    /// Zaman damgası
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventStatus {
    Processed,
    Ignored,
    Failed,
    Queued,
}

impl std::fmt::Display for EventStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Processed => write!(f, "Processed"),
            Self::Ignored => write!(f, "Ignored"),
            Self::Failed => write!(f, "Failed"),
            Self::Queued => write!(f, "Queued"),
        }
    }
}

/// ─── EVENT LISTENER ───

pub struct EventListener {
    config: EventListenerConfig,
    
    /// Event receiver
    event_rx: Option<mpsc::Receiver<WebhookEvent>>,
    
    /// Event sender (dışarıya vermek için)
    event_tx: mpsc::Sender<WebhookEvent>,
    
    /// Task sender (Orchestrator'a)
    task_tx: Option<mpsc::Sender<TaskRequest>>,
    
    /// İstatistikler
    stats: Arc<RwLock<EventListenerStats>>,
    
    /// Çalışıyor mu?
    running: Arc<RwLock<bool>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventListenerStats {
    pub total_events: u64,
    pub processed_events: u64,
    pub tasks_created: u64,
    pub ignored_events: u64,
    pub failed_events: u64,
    pub by_provider: std::collections::HashMap<String, u64>,
    pub by_event_type: std::collections::HashMap<String, u64>,
    pub last_event: Option<DateTime<Utc>>,
}

impl Default for EventListenerStats {
    fn default() -> Self {
        Self {
            total_events: 0,
            processed_events: 0,
            tasks_created: 0,
            ignored_events: 0,
            failed_events: 0,
            by_provider: std::collections::HashMap::new(),
            by_event_type: std::collections::HashMap::new(),
            last_event: None,
        }
    }
}

/// ─── TASK REQUEST ───

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskRequest {
    /// Görev ID
    pub id: Uuid,
    
    /// Hedef açıklaması
    pub goal: String,
    
    /// Öncelik
    pub priority: TaskPriority,
    
    /// Model
    pub model: String,
    
    /// Kaynak event
    pub source_event: Option<Uuid>,
    
    /// Provider
    pub provider: WebhookProvider,
    
    /// Metadata
    pub metadata: std::collections::HashMap<String, String>,
    
    /// Zaman damgası
    pub timestamp: DateTime<Utc>,
}

impl TaskRequest {
    pub fn from_event(event: &WebhookEvent, model: impl Into<String>) -> Self {
        let priority = match event.priority {
            EventPriority::Low => TaskPriority::Low,
            EventPriority::Normal => TaskPriority::Normal,
            EventPriority::High => TaskPriority::High,
            EventPriority::Critical => TaskPriority::Critical,
        };
        
        Self {
            id: Uuid::new_v4(),
            goal: event.to_task_description(),
            priority,
            model: model.into(),
            source_event: Some(event.id),
            provider: event.provider.clone(),
            metadata: event.metadata.clone(),
            timestamp: Utc::now(),
        }
    }
}

impl EventListener {
    /// Yeni listener oluştur
    pub fn new(config: EventListenerConfig) -> Self {
        let (event_tx, event_rx) = mpsc::channel(config.buffer_size);
        
        Self {
            config,
            event_rx: Some(event_rx),
            event_tx,
            task_tx: None,
            stats: Arc::new(RwLock::new(EventListenerStats::default())),
            running: Arc::new(RwLock::new(false)),
        }
    }
    
    /// Event sender al (webhook receiver için)
    pub fn event_sender(&self) -> mpsc::Sender<WebhookEvent> {
        self.event_tx.clone()
    }
    
    /// Task sender ayarla (Orchestrator'a bağlanmak için)
    pub fn with_task_sender(mut self, tx: mpsc::Sender<TaskRequest>) -> Self {
        self.task_tx = Some(tx);
        self
    }
    
    /// Başlat
    pub async fn start(&mut self) -> SENTIENTResult<()> {
        *self.running.write().await = true;
        
        log::info!("════════════════════════════════════════════════════════════");
        log::info!("👂  EVENT LISTENER başlatılıyor...");
        log::info!("════════════════════════════════════════════════════════════");
        log::info!("   Buffer: {}", self.config.buffer_size);
        log::info!("   Otomatik görev: {}", if self.config.auto_create_tasks { "aktif" } else { "pasif" });
        log::info!("   Model: {}", self.config.default_model);
        log::info!("════════════════════════════════════════════════════════════");
        
        // Event loop'u başlat
        if let Some(rx) = self.event_rx.take() {
            let stats = self.stats.clone();
            let running = self.running.clone();
            let task_tx = self.task_tx.clone();
            let config = self.config.clone();
            
            tokio::spawn(async move {
                Self::event_loop(rx, stats, running, task_tx, config).await;
            });
        }
        
        Ok(())
    }
    
    /// Ana event döngüsü
    async fn event_loop(
        mut rx: mpsc::Receiver<WebhookEvent>,
        stats: Arc<RwLock<EventListenerStats>>,
        running: Arc<RwLock<bool>>,
        task_tx: Option<mpsc::Sender<TaskRequest>>,
        config: EventListenerConfig,
    ) {
        log::info!("👂  Event listener dinliyor...");
        
        while *running.read().await {
            match rx.recv().await {
                Some(event) => {
                    let start = std::time::Instant::now();
                    
                    // İstatistikleri güncelle
                    {
                        let mut s = stats.write().await;
                        s.total_events += 1;
                        s.last_event = Some(Utc::now());
                        *s.by_provider.entry(event.provider.to_string()).or_insert(0) += 1;
                        *s.by_event_type.entry(event.event_type.to_string()).or_insert(0) += 1;
                    }
                    
                    // Event'i işle
                    let result = Self::process_event(
                        &event,
                        &task_tx,
                        &config,
                    ).await;
                    
                    // İstatistikleri güncelle
                    {
                        let mut s = stats.write().await;
                        match result.status {
                            EventStatus::Processed => s.processed_events += 1,
                            EventStatus::Ignored => s.ignored_events += 1,
                            EventStatus::Failed => s.failed_events += 1,
                            EventStatus::Queued => {}
                        }
                        if result.task_created {
                            s.tasks_created += 1;
                        }
                    }
                    
                    log::info!(
                        "event  {} [{}] {}ms",
                        event.summary(),
                        result.status,
                        start.elapsed().as_millis()
                    );
                }
                None => {
                    log::warn!("event  Channel kapandı");
                    break;
                }
            }
        }
        
        log::info!("👂  Event listener durduruldu");
    }
    
    /// Event'i işle
    async fn process_event(
        event: &WebhookEvent,
        task_tx: &Option<mpsc::Sender<TaskRequest>>,
        config: &EventListenerConfig,
    ) -> ProcessedEvent {
        // Provider aksiyonunu al
        let provider_key = event.provider.to_string();
        let provider_action = config.provider_actions.get(&provider_key);
        
        // Aksiyon belirle
        let should_create_task = provider_action
            .map(|pa| pa.create_task && config.auto_create_tasks)
            .unwrap_or(config.auto_create_tasks);
        
        let action = event.action;
        
        // Ignore aksiyonu
        if action == EventAction::Ignore {
            return ProcessedEvent {
                event_id: event.id,
                task_created: false,
                task_id: None,
                status: EventStatus::Ignored,
                message: "Event yoksayıldı".into(),
                duration_ms: 0,
                timestamp: Utc::now(),
            };
        }
        
        // Görev oluştur
        if should_create_task {
            if let Some(tx) = task_tx {
                let task = TaskRequest::from_event(event, &config.default_model);
                let task_id = task.id;
                
                match tx.send(task).await {
                    Ok(()) => {
                        return ProcessedEvent {
                            event_id: event.id,
                            task_created: true,
                            task_id: Some(task_id),
                            status: EventStatus::Processed,
                            message: "Görev oluşturuldu".into(),
                            duration_ms: 0,
                            timestamp: Utc::now(),
                        };
                    }
                    Err(e) => {
                        return ProcessedEvent {
                            event_id: event.id,
                            task_created: false,
                            task_id: None,
                            status: EventStatus::Failed,
                            message: format!("Görev gönderilemedi: {}", e),
                            duration_ms: 0,
                            timestamp: Utc::now(),
                        };
                    }
                }
            }
        }
        
        ProcessedEvent {
            event_id: event.id,
            task_created: false,
            task_id: None,
            status: EventStatus::Processed,
            message: "Event işlendi (görev oluşturulmadı)".into(),
            duration_ms: 0,
            timestamp: Utc::now(),
        }
    }
    
    /// Durdur
    pub async fn stop(&self) {
        *self.running.write().await = false;
    }
    
    /// İstatistikler
    pub async fn stats(&self) -> EventListenerStats {
        self.stats.read().await.clone()
    }
    
    /// Rapor
    pub async fn report(&self) -> String {
        let stats = self.stats.read().await;
        
        format!(
            r#"
════════════════════════════════════════════════════════════
  👂  EVENT LISTENER RAPORU
════════════════════════════════════════════════════════════
  Toplam Event:       {}
  İşlenen:            {}
  Görev Oluşturulan:  {}
  Yoksayılan:         {}
  Hatalı:             {}
  ────────────────────────────────────────────────────────────
  Son Event:          {}
════════════════════════════════════════════════════════════"#,
            stats.total_events,
            stats.processed_events,
            stats.tasks_created,
            stats.ignored_events,
            stats.failed_events,
            stats.last_event.map(|t| t.to_rfc3339()).unwrap_or("-".into())
        )
    }
}

// ─── Tests ───

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_event_listener_config_default() {
        let config = EventListenerConfig::default();
        assert!(config.auto_create_tasks);
        assert!(config.provider_actions.contains_key("github"));
    }
    
    #[test]
    fn test_task_request_from_event() {
        let event = WebhookEvent::github_push("user/repo", "main", 3);
        let task = TaskRequest::from_event(&event, "qwen/qwen3-1.7b:free");
        
        assert!(task.goal.contains("analiz"));
        assert_eq!(task.provider, WebhookProvider::GitHub);
        assert_eq!(task.priority, TaskPriority::Normal);
    }
    
    #[tokio::test]
    async fn test_event_listener_creation() {
        let listener = EventListener::new(EventListenerConfig::default());
        let stats = listener.stats().await;
        
        assert_eq!(stats.total_events, 0);
    }
    
    #[test]
    fn test_processed_event() {
        let event = ProcessedEvent {
            event_id: Uuid::new_v4(),
            task_created: true,
            task_id: Some(Uuid::new_v4()),
            status: EventStatus::Processed,
            message: "Test".into(),
            duration_ms: 10,
            timestamp: Utc::now(),
        };
        
        assert!(event.task_created);
        assert_eq!(event.status, EventStatus::Processed);
    }
}
