//! ─── WEBHOOK ENTegrasyon SİSTEMİ ───
//!
//! Dış servislerden (n8n, GitHub, Stripe, Slack, vb.) gelen
//! webhook bildirimlerini dinler ve işler.
//!
//! Güvenlik: V-GATE üzerinden imza doğrulama
//!
//! ┌─────────────────────────────────────────────────────────────┐
//! │                     WEBHOOK FLOW                            │
//! │                                                              │
//! │  GitHub  ──┐                                                 │
//! │  Stripe  ──┼──▶ /webhook/:provider ──▶ Signature Verify    │
//! │  n8n     ──┤         │                   │                  │
//! │  Slack   ──┘         ▼                   ▼                  │
//! │                  WebhookRouter    V-GATE Security           │
//! │                       │                                      │
//! │                       ▼                                      │
//! │                  Event Parser                                │
//! │                       │                                      │
//! │                       ▼                                      │
//! │                  Event Listener                              │
//! │                       │                                      │
//! │                       ▼                                      │
//! │                  Orchestrator                                │
//! └─────────────────────────────────────────────────────────────┘

mod receiver;
mod router;
mod providers;
mod signature;
mod event;

pub use receiver::{WebhookReceiver, WebhookConfig};
pub use router::{WebhookRouter, WebhookRoute};
pub use providers::{
    WebhookProvider, WebhookPayload, 
    GitHubPayload, StripePayload, N8nPayload, SlackPayload,
    GenericPayload,
};
pub use signature::{SignatureVerifier, SignatureAlgorithm};
pub use event::{WebhookEvent, EventType, EventPriority, EventAction};

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Webhook işleme sonucu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookResult {
    /// İşlem ID'si
    pub id: Uuid,
    
    /// Başarılı mı?
    pub success: bool,
    
    /// Mesaj
    pub message: String,
    
    /// Oluşturulan görev ID'si (varsa)
    pub task_id: Option<Uuid>,
    
    /// İşlem süresi (ms)
    pub duration_ms: u64,
    
    /// Zaman damgası
    pub timestamp: DateTime<Utc>,
}

impl WebhookResult {
    pub fn success(message: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            success: true,
            message: message.into(),
            task_id: None,
            duration_ms: 0,
            timestamp: Utc::now(),
        }
    }
    
    pub fn with_task(mut self, task_id: Uuid) -> Self {
        self.task_id = Some(task_id);
        self
    }
    
    pub fn failure(message: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            success: false,
            message: message.into(),
            task_id: None,
            duration_ms: 0,
            timestamp: Utc::now(),
        }
    }
    
    pub fn with_duration(mut self, ms: u64) -> Self {
        self.duration_ms = ms;
        self
    }
}

/// Webhook istatistikleri
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WebhookStats {
    pub total_received: u64,
    pub total_processed: u64,
    pub total_failed: u64,
    pub by_provider: std::collections::HashMap<String, u64>,
    pub last_received: Option<DateTime<Utc>>,
}
