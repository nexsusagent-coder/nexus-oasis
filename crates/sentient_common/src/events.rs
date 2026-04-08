//! ═════════════════════════════════════════════════════════════════
//!  EVENTS MODULE
//! ═════════════════════════════════════════════════════════════════

use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// ─── Olay / Event Sistemi ───
/// GraphBit düğümleri arası iletişim bu eventler üzerinden akar.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SENTIENTEvent {
    pub id: Uuid,
    pub r#type: EventType,
    pub payload: serde_json::Value,
    pub timestamp: DateTime<Utc>,
    pub source: String,          // hangi modül üret
    pub correlation_id: Option<Uuid>, // zincir takibi
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventType {
    // ─── Çekirdek ───
    SystemStart,
    SystemShutdown,
    GraphTick,

    // ─── Bellek ───
    MemoryStore,
    MemoryRecall,

    // ─── V-GATE ───
    VGateRequest,
    VGateResponse,

    // ─── Guardrails ───
    GuardrailBlock,
    GuardrailAllow,

    // ─── Python / Araç ───
    ToolCall,
    ToolResult,

    // ─── Görev ───
    TaskCreated,
    TaskCompleted,
    TaskFailed,

    // ─── Browser-Use ───
    BrowserReady,      // Tarayıcı hazır
    BrowserResult,     // Tarayıcı sonucu
    BrowserError,      // Tarayıcı hatası
    ResearchStart,     // Araştırma başladı
    ResearchComplete,  // Araştırma tamamlandı
    DataExtracted,     // Veri çıkarıldı
    PageLoaded,        // Sayfa yüklendi
    ScreenshotTaken,   // Ekran görüntüsü alındı

    // ─── Sandbox / Docker ───
    SandboxCreated,    // Sandbox oluşturuldu
    SandboxStarted,    // Sandbox başlatıldı
    SandboxStopped,    // Sandbox durduruldu
    SandboxDestroyed,  // Sandbox silindi
    CodeExecuteStart,  // Kod çalıştırma başladı
    CodeExecuteResult, // Kod çalıştırma sonucu
    SandboxTimeout,    // Sandbox zaman aşımı
    SandboxError,      // Sandbox hatası
}

impl SENTIENTEvent {
    pub fn new(r#type: EventType, source: impl Into<String>, payload: serde_json::Value) -> Self {
        Self {
            id: Uuid::new_v4(),
            r#type,
            payload,
            timestamp: Utc::now(),
            source: source.into(),
            correlation_id: None,
        }
    }

    pub fn with_correlation(mut self, cid: Uuid) -> Self {
        self.correlation_id = Some(cid);
        self
    }
}
