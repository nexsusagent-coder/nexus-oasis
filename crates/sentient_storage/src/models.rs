//! ─── STORAGE MODELS ───
//!
//! Veritabanında saklanan veri modelleri

use sentient_common::error::{SENTIENTError, SENTIENTResult};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ═══════════════════════════════════════════════════════════════
// ERROR TYPES
// ═══════════════════════════════════════════════════════════════

#[derive(Debug)]
pub enum StorageError {
    /// Veritabanı bağlantı hatası
    ConnectionError(String),
    /// Sorgu hatası
    QueryError(String),
    /// Görev bulunamadı
    TaskNotFound(Uuid),
    /// Serileştirme hatası
    SerializationError(String),
    /// Hydration hatası
    HydrationError(String),
    /// Geçersiz durum geçişi
    InvalidStateTransition(String),
}

impl std::fmt::Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ConnectionError(msg) => write!(f, "Bağlantı hatası: {}", msg),
            Self::QueryError(msg) => write!(f, "Sorgu hatası: {}", msg),
            Self::TaskNotFound(id) => write!(f, "Görev bulunamadı: {}", id),
            Self::SerializationError(msg) => write!(f, "Serileştirme hatası: {}", msg),
            Self::HydrationError(msg) => write!(f, "Hydration hatası: {}", msg),
            Self::InvalidStateTransition(msg) => write!(f, "Geçersiz durum geçişi: {}", msg),
        }
    }
}

impl std::error::Error for StorageError {}

impl From<StorageError> for SENTIENTError {
    fn from(e: StorageError) -> Self {
        SENTIENTError::Database(format!("{}", e))
    }
}

impl From<rusqlite::Error> for StorageError {
    fn from(e: rusqlite::Error) -> Self {
        match e {
            rusqlite::Error::QueryReturnedNoRows => StorageError::TaskNotFound(Uuid::nil()),
            _ => StorageError::QueryError(e.to_string()),
        }
    }
}

pub type StorageResult<T> = Result<T, StorageError>;

// ═══════════════════════════════════════════════════════════════
// TASK STATUS
// ═══════════════════════════════════════════════════════════════

/// Görev durumu (veritabanı için)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
#[serde(rename_all = "snake_case")]
pub enum PersistedStatus {
    /// Kuyrukta bekliyor
    Queued,
    /// Başlatılıyor
    Starting,
    /// Çalışıyor
    Running,
    /// Duraklatıldı
    Paused,
    /// Bekleniyor (external)
    Waiting,
    /// Başarıyla tamamlandı
    Completed,
    /// Hata ile bitti
    Failed,
    /// İptal edildi
    Cancelled,
    /// Timeout
    Timeout,
}

impl PersistedStatus {
    /// Aktif mi? (yeniden başlatıldığında devam ettirilebilir)
    pub fn is_resumable(&self) -> bool {
        matches!(self, Self::Queued | Self::Starting | Self::Running | Self::Paused | Self::Waiting)
    }
    
    /// Terminal mi? (sonlanmış)
    pub fn is_terminal(&self) -> bool {
        matches!(self, Self::Completed | Self::Failed | Self::Cancelled | Self::Timeout)
    }
    
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Queued => "queued",
            Self::Starting => "starting",
            Self::Running => "running",
            Self::Paused => "paused",
            Self::Waiting => "waiting",
            Self::Completed => "completed",
            Self::Failed => "failed",
            Self::Cancelled => "cancelled",
            Self::Timeout => "timeout",
        }
    }
    
    pub fn from_str(s: &str) -> StorageResult<Self> {
        match s {
            "queued" => Ok(Self::Queued),
            "starting" => Ok(Self::Starting),
            "running" => Ok(Self::Running),
            "paused" => Ok(Self::Paused),
            "waiting" => Ok(Self::Waiting),
            "completed" => Ok(Self::Completed),
            "failed" => Ok(Self::Failed),
            "cancelled" => Ok(Self::Cancelled),
            "timeout" => Ok(Self::Timeout),
            _ => Err(StorageError::SerializationError(format!("Bilinmeyen durum: {}", s))),
        }
    }
}

impl std::fmt::Display for PersistedStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Queued => write!(f, "Kuyrukta"),
            Self::Starting => write!(f, "Başlatılıyor"),
            Self::Running => write!(f, "Çalışıyor"),
            Self::Paused => write!(f, "Duraklatıldı"),
            Self::Waiting => write!(f, "Bekliyor"),
            Self::Completed => write!(f, "Tamamlandı"),
            Self::Failed => write!(f, "Başarısız"),
            Self::Cancelled => write!(f, "İptal"),
            Self::Timeout => write!(f, "Zaman Aşımı"),
        }
    }
}

// ═══════════════════════════════════════════════════════════════
// PERSISTED TASK
// ═══════════════════════════════════════════════════════════════

/// Veritabanında saklanan görev
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistedTask {
    /// Benzersiz ID
    pub id: Uuid,
    /// İstek ID
    pub request_id: Uuid,
    /// Hedef açıklaması
    pub goal: String,
    /// Kullanılan model
    pub model: String,
    /// Durum
    pub status: PersistedStatus,
    /// Öncelik (0-3)
    pub priority: u8,
    /// Atanan ajan
    pub assigned_agent: Option<String>,
    /// Mevcut aşama
    pub current_step: Option<u32>,
    /// Toplam aşama
    pub total_steps: u32,
    /// İlerleme (0-100)
    pub progress: f32,
    /// Başlangıç zamanı (ISO 8601)
    pub started_at: String,
    /// Güncelleme zamanı (ISO 8601)
    pub updated_at: String,
    /// Bitiş zamanı (ISO 8601)
    pub completed_at: Option<String>,
    /// Sonuç (JSON)
    pub result: Option<serde_json::Value>,
    /// Hata mesajı
    pub error: Option<String>,
    /// Kaynak
    pub source: String,
    /// Kullanıcı ID
    pub user_id: Option<String>,
    /// Yeniden deneme sayısı
    pub retry_count: u8,
    /// Checkpoint verisi
    pub checkpoint: Option<serde_json::Value>,
}

impl PersistedTask {
    pub fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Self> {
        Ok(Self {
            id: Uuid::parse_str(&row.get::<_, String>("id")?).map_err(|_| rusqlite::Error::InvalidQuery)?,
            request_id: Uuid::parse_str(&row.get::<_, String>("request_id")?).map_err(|_| rusqlite::Error::InvalidQuery)?,
            goal: row.get("goal")?,
            model: row.get("model")?,
            status: PersistedStatus::from_str(&row.get::<_, String>("status")?).map_err(|_| rusqlite::Error::InvalidQuery)?,
            priority: row.get::<_, i32>("priority")? as u8,
            assigned_agent: row.get("assigned_agent")?,
            current_step: row.get::<_, Option<i32>>("current_step")?.map(|s| s as u32),
            total_steps: row.get::<_, i32>("total_steps")? as u32,
            progress: row.get("progress")?,
            started_at: row.get("started_at")?,
            updated_at: row.get("updated_at")?,
            completed_at: row.get("completed_at")?,
            result: row.get::<_, Option<String>>("result")?.and_then(|s| serde_json::from_str(&s).ok()),
            error: row.get("error")?,
            source: row.get("source")?,
            user_id: row.get("user_id")?,
            retry_count: row.get::<_, i32>("retry_count")? as u8,
            checkpoint: row.get::<_, Option<String>>("checkpoint")?.and_then(|s| serde_json::from_str(&s).ok()),
        })
    }
}

// ═══════════════════════════════════════════════════════════════
// PERSISTED STEP
// ═══════════════════════════════════════════════════════════════

/// Görev adımı
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistedStep {
    pub id: Uuid,
    pub task_id: Uuid,
    pub step_number: u32,
    pub agent: String,
    pub action: String,
    pub status: String,
    pub input: Option<serde_json::Value>,
    pub output: Option<serde_json::Value>,
    pub started_at: String,
    pub completed_at: Option<String>,
    pub tokens_used: u64,
}

impl PersistedStep {
    pub fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Self> {
        Ok(Self {
            id: Uuid::parse_str(&row.get::<_, String>("id")?).map_err(|_| rusqlite::Error::InvalidQuery)?,
            task_id: Uuid::parse_str(&row.get::<_, String>("task_id")?).map_err(|_| rusqlite::Error::InvalidQuery)?,
            step_number: row.get::<_, i32>("step_number")? as u32,
            agent: row.get("agent")?,
            action: row.get("action")?,
            status: row.get("status")?,
            input: row.get::<_, Option<String>>("input")?.and_then(|s| serde_json::from_str(&s).ok()),
            output: row.get::<_, Option<String>>("output")?.and_then(|s| serde_json::from_str(&s).ok()),
            started_at: row.get("started_at")?,
            completed_at: row.get("completed_at")?,
            tokens_used: row.get::<_, i64>("tokens_used")? as u64,
        })
    }
}

// ═══════════════════════════════════════════════════════════════
// TASK SNAPSHOT (Hydration için)
// ═══════════════════════════════════════════════════════════════

/// Görev anlık görüntüsü (hydration için)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskSnapshot {
    pub task: PersistedTask,
    pub steps: Vec<PersistedStep>,
    pub context: serde_json::Value,
}

// ═══════════════════════════════════════════════════════════════
// WORKFLOW STATE
// ═══════════════════════════════════════════════════════════════

/// İş akışı durumu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowState {
    pub id: Uuid,
    pub name: String,
    pub workflow_type: String,
    pub status: String,
    pub current_task_id: Option<Uuid>,
    pub dependencies: Vec<Uuid>,
    pub params: serde_json::Value,
    pub created_at: String,
    pub updated_at: String,
}

impl WorkflowState {
    pub fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Self> {
        let deps_str: Option<String> = row.get("dependencies")?;
        let params_str: Option<String> = row.get("params")?;
        
        Ok(Self {
            id: Uuid::parse_str(&row.get::<_, String>("id")?).map_err(|_| rusqlite::Error::InvalidQuery)?,
            name: row.get("name")?,
            workflow_type: row.get("workflow_type")?,
            status: row.get("status")?,
            current_task_id: row.get::<_, Option<String>>("current_task_id")?
                .and_then(|s| Uuid::parse_str(&s).ok()),
            dependencies: deps_str.and_then(|s| serde_json::from_str(&s).ok()).unwrap_or_default(),
            params: params_str.and_then(|s| serde_json::from_str(&s).ok()).unwrap_or(serde_json::Value::Null),
            created_at: row.get("created_at")?,
            updated_at: row.get("updated_at")?,
        })
    }
}

// ═══════════════════════════════════════════════════════════════
// TASK LOG
// ═══════════════════════════════════════════════════════════════

/// Görev log kaydı
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskLogEntry {
    pub id: Uuid,
    pub task_id: Uuid,
    pub timestamp: String,
    pub level: String,
    pub source: String,
    pub message: String,
}

impl TaskLogEntry {
    pub fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Self> {
        Ok(Self {
            id: Uuid::parse_str(&row.get::<_, String>("id")?).map_err(|_| rusqlite::Error::InvalidQuery)?,
            task_id: Uuid::parse_str(&row.get::<_, String>("task_id")?).map_err(|_| rusqlite::Error::InvalidQuery)?,
            timestamp: row.get("timestamp")?,
            level: row.get("level")?,
            source: row.get("source")?,
            message: row.get("message")?,
        })
    }
}
