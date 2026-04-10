//! ─── MEMSCHEDULER - BELLEK İŞLEM KUYRUĞU ───
//!
//! Asenkron bellek işlemleri için görev kuyruğu sistemi.
//! - Store: Bellek kaydetme işlemleri
//! - Index: Vektör indeksleme
//! - Consolidate: Bellek birleştirme
//! - Decay: Bellek azaltma
//! - Cleanup: Süresi dolmuş kayıtları temizleme

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock, Notify};
use uuid::Uuid;

use super::types::{MemoryEntry, MemoryInput, MemoryType};

// ─────────────────────────────────────────────────────────────────────────────
// GÖREV TİPLERİ
// ─────────────────────────────────────────────────────────────────────────────

/// Zamanlanmış görev tipi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MemTask {
    /// Belleğe kaydet
    Store {
        cube_id: Uuid,
        entry: MemoryEntry,
        priority: TaskPriority,
    },
    
    /// Vektör indeksle
    Index {
        cube_id: Uuid,
        memory_id: Uuid,
        content: String,
    },
    
    /// Bellekleri birleştir
    Consolidate {
        cube_id: Uuid,
        memory_ids: Vec<Uuid>,
    },
    
    /// Bellek azaltma uygula
    Decay {
        cube_id: Uuid,
        rate: f32,
    },
    
    /// Süresi dolmuş kayıtları temizle
    Cleanup {
        cube_id: Uuid,
    },
    
    /// Arşivle
    Archive {
        cube_id: Uuid,
        memory_id: Uuid,
    },
    
    /// RAG için context hazırla
    PrepareContext {
        cube_id: Uuid,
        query: String,
        max_tokens: usize,
    },
    
    /// FTS5 indeksini güncelle
    UpdateFTS {
        cube_id: Uuid,
        memory_id: Uuid,
        content: String,
    },
    
    /// Bilgi grafı güncelle
    UpdateGraph {
        cube_id: Uuid,
        source_id: Uuid,
        target_id: Option<Uuid>,
    },
}

/// Görev önceliği
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum TaskPriority {
    Low = 0,
    Normal = 1,
    High = 2,
    Critical = 3,
}

impl Default for TaskPriority {
    fn default() -> Self {
        Self::Normal
    }
}

/// Görev durumu
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
}

/// Zamanlanmış görev
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledTask {
    /// Görev ID
    pub id: Uuid,
    /// Görev
    pub task: MemTask,
    /// Öncelik
    pub priority: TaskPriority,
    /// Durum
    pub status: TaskStatus,
    /// Oluşturulma zamanı
    pub created_at: DateTime<Utc>,
    /// Başlangıç zamanı
    pub started_at: Option<DateTime<Utc>>,
    /// Bitiş zamanı
    pub completed_at: Option<DateTime<Utc>>,
    /// Deneme sayısı
    pub retry_count: u32,
    /// Maksimum deneme
    pub max_retries: u32,
    /// Hata mesajı
    pub error: Option<String>,
    /// Sonuç
    pub result: Option<TaskResult>,
}

/// Görev sonucu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskResult {
    Stored { memory_id: Uuid },
    Indexed { memory_id: Uuid },
    Consolidated { new_memories: usize },
    Decayed { affected: usize },
    Cleaned { removed: usize },
    Archived { memory_id: Uuid },
    ContextPrepared { token_count: usize },
    FTSUpdated { memory_id: Uuid },
    GraphUpdated { nodes: usize, edges: usize },
    Error { message: String },
}

// ─────────────────────────────────────────────────────────────────────────────
// GÖREV KUYRUĞU
// ─────────────────────────────────────────────────────────────────────────────

/// Öncelikli görev kuyruğu
#[derive(Debug)]
pub struct TaskQueue {
    /// Kritik öncelikli görevler
    critical: VecDeque<ScheduledTask>,
    /// Yüksek öncelikli görevler
    high: VecDeque<ScheduledTask>,
    /// Normal öncelikli görevler
    normal: VecDeque<ScheduledTask>,
    /// Düşük öncelikli görevler
    low: VecDeque<ScheduledTask>,
}

impl TaskQueue {
    pub fn new() -> Self {
        Self {
            critical: VecDeque::new(),
            high: VecDeque::new(),
            normal: VecDeque::new(),
            low: VecDeque::new(),
        }
    }
    
    /// Görev ekle
    pub fn push(&mut self, task: ScheduledTask) {
        match task.priority {
            TaskPriority::Critical => self.critical.push_back(task),
            TaskPriority::High => self.high.push_back(task),
            TaskPriority::Normal => self.normal.push_back(task),
            TaskPriority::Low => self.low.push_back(task),
        }
    }
    
    /// Sonraki görevi al (önceliğe göre)
    pub fn pop(&mut self) -> Option<ScheduledTask> {
        if let Some(task) = self.critical.pop_front() {
            return Some(task);
        }
        if let Some(task) = self.high.pop_front() {
            return Some(task);
        }
        if let Some(task) = self.normal.pop_front() {
            return Some(task);
        }
        self.low.pop_front()
    }
    
    /// Toplam görev sayısı
    pub fn len(&self) -> usize {
        self.critical.len() + self.high.len() + self.normal.len() + self.low.len()
    }
    
    /// Boş mu?
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    
    /// Öncelik bazlı sayı
    pub fn count_by_priority(&self) -> (usize, usize, usize, usize) {
        (
            self.critical.len(),
            self.high.len(),
            self.normal.len(),
            self.low.len(),
        )
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// MEMSCHEDULER
// ─────────────────────────────────────────────────────────────────────────────

/// Bellek zamanlayıcı yapılandırması
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchedulerConfig {
    /// Maksimum eşzamanlı görev
    pub max_concurrent: usize,
    /// Görev zaman aşımı (saniye)
    pub task_timeout_secs: u64,
    /// Otomatik temizleme aralığı (saniye)
    pub cleanup_interval_secs: u64,
    /// Maksimum deneme sayısı
    pub max_retries: u32,
    /// Konsolidasyon aralığı (saniye)
    pub consolidation_interval_secs: u64,
    /// Decay aralığı (saniye)
    pub decay_interval_secs: u64,
}

impl Default for SchedulerConfig {
    fn default() -> Self {
        Self {
            max_concurrent: 4,
            task_timeout_secs: 30,
            cleanup_interval_secs: 300,
            max_retries: 3,
            consolidation_interval_secs: 3600,
            decay_interval_secs: 7200,
        }
    }
}

/// Zamanlayıcı istatistikleri
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SchedulerStats {
    pub total_tasks: u64,
    pub completed_tasks: u64,
    pub failed_tasks: u64,
    pub pending_tasks: usize,
    pub avg_duration_ms: f64,
    pub last_task_at: Option<DateTime<Utc>>,
}

/// MemScheduler - Bellek işlem zamanlayıcısı
pub struct MemScheduler {
    /// Görev kuyruğu
    queue: Arc<Mutex<TaskQueue>>,
    /// Yapılandırma
    config: SchedulerConfig,
    /// İstatistikler
    stats: Arc<RwLock<SchedulerStats>>,
    /// Yeni görev bildirimi
    notify: Arc<Notify>,
    /// Çalışıyor mu?
    running: Arc<RwLock<bool>>,
}

impl MemScheduler {
    /// Yeni zamanlayıcı oluştur
    pub fn new(config: SchedulerConfig) -> Self {
        Self {
            queue: Arc::new(Mutex::new(TaskQueue::new())),
            config,
            stats: Arc::new(RwLock::new(SchedulerStats::default())),
            notify: Arc::new(Notify::new()),
            running: Arc::new(RwLock::new(false)),
        }
    }
    
    /// Varsayılan yapılandırma ile oluştur
    pub fn default_scheduler() -> Self {
        Self::new(SchedulerConfig::default())
    }
    
    /// Görev zamanla
    pub async fn schedule(&self, task: MemTask, priority: TaskPriority) -> Uuid {
        let scheduled = ScheduledTask {
            id: Uuid::new_v4(),
            task,
            priority,
            status: TaskStatus::Pending,
            created_at: Utc::now(),
            started_at: None,
            completed_at: None,
            retry_count: 0,
            max_retries: self.config.max_retries,
            error: None,
            result: None,
        };
        
        let task_id = scheduled.id;
        
        self.queue.lock().await.push(scheduled);
        self.notify.notify_one();
        
        // İstatistikleri güncelle
        let mut stats = self.stats.write().await;
        stats.total_tasks += 1;
        stats.pending_tasks = self.queue.lock().await.len();
        
        log::debug!("📤 Görev zamanlandı: {} (öncelik: {:?})", task_id, priority);
        
        task_id
    }
    
    /// Bellek kaydetme görevi zamanla
    pub async fn schedule_store(&self, cube_id: Uuid, entry: MemoryEntry) -> Uuid {
        self.schedule(
            MemTask::Store {
                cube_id,
                entry,
                priority: TaskPriority::Normal,
            },
            TaskPriority::Normal,
        ).await
    }
    
    /// Yüksek öncelikli kayıt
    pub async fn schedule_store_urgent(&self, cube_id: Uuid, entry: MemoryEntry) -> Uuid {
        self.schedule(
            MemTask::Store {
                cube_id,
                entry,
                priority: TaskPriority::High,
            },
            TaskPriority::High,
        ).await
    }
    
    /// İndeksleme görevi
    pub async fn schedule_index(&self, cube_id: Uuid, memory_id: Uuid, content: String) -> Uuid {
        self.schedule(
            MemTask::Index { cube_id, memory_id, content },
            TaskPriority::Normal,
        ).await
    }
    
    /// Konsolidasyon görevi
    pub async fn schedule_consolidate(&self, cube_id: Uuid) -> Uuid {
        self.schedule(
            MemTask::Consolidate { cube_id, memory_ids: vec![] },
            TaskPriority::Low,
        ).await
    }
    
    /// Decay görevi
    pub async fn schedule_decay(&self, cube_id: Uuid, rate: f32) -> Uuid {
        self.schedule(
            MemTask::Decay { cube_id, rate },
            TaskPriority::Low,
        ).await
    }
    
    /// Temizleme görevi
    pub async fn schedule_cleanup(&self, cube_id: Uuid) -> Uuid {
        self.schedule(
            MemTask::Cleanup { cube_id },
            TaskPriority::Low,
        ).await
    }
    
    /// FTS5 güncelleme
    pub async fn schedule_fts_update(&self, cube_id: Uuid, memory_id: Uuid, content: String) -> Uuid {
        self.schedule(
            MemTask::UpdateFTS { cube_id, memory_id, content },
            TaskPriority::Normal,
        ).await
    }
    
    /// Sonraki görevi al
    pub async fn next_task(&self) -> Option<ScheduledTask> {
        self.queue.lock().await.pop()
    }
    
    /// Kuyruk boşalana kadar bekle
    pub async fn wait_for_task(&self) {
        self.notify.notified().await;
    }
    
    /// Görev tamamlandı işaretle
    pub async fn complete_task(&self, task_id: Uuid, result: TaskResult) {
        let mut stats = self.stats.write().await;
        stats.completed_tasks += 1;
        stats.pending_tasks = self.queue.lock().await.len();
        stats.last_task_at = Some(Utc::now());
        
        match &result {
            TaskResult::Stored { memory_id } => {
                log::debug!("✅ Görev tamamlandı: {} → Bellek: {}", task_id, memory_id);
            }
            TaskResult::Indexed { memory_id } => {
                log::debug!("✅ İndekslendi: {} → {}", task_id, memory_id);
            }
            TaskResult::Cleaned { removed } => {
                log::debug!("✅ Temizlendi: {} → {} kayıt", task_id, removed);
            }
            _ => {
                log::debug!("✅ Görev tamamlandı: {}", task_id);
            }
        }
    }
    
    /// Görev başarısız işaretle
    pub async fn fail_task(&self, task_id: Uuid, error: String) {
        let mut stats = self.stats.write().await;
        stats.failed_tasks += 1;
        stats.pending_tasks = self.queue.lock().await.len();
        
        log::error!("❌ Görev başarısız: {} → {}", task_id, error);
    }
    
    /// İstatistikleri al
    pub async fn stats(&self) -> SchedulerStats {
        self.stats.read().await.clone()
    }
    
    /// Kuyruk durumunu al
    pub async fn queue_status(&self) -> (usize, usize, usize, usize) {
        self.queue.lock().await.count_by_priority()
    }
    
    /// Zamanlayıcıyı başlat
    pub async fn start(&self) {
        let mut running = self.running.write().await;
        *running = true;
        log::info!("⏰ MemScheduler başlatıldı");
    }
    
    /// Zamanlayıcıyı durdur
    pub async fn stop(&self) {
        let mut running = self.running.write().await;
        *running = false;
        log::info!("⏹️ MemScheduler durduruldu");
    }
    
    /// Çalışıyor mu?
    pub async fn is_running(&self) -> bool {
        *self.running.read().await
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// TESTLER
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_task_queue_priority() {
        let mut queue = TaskQueue::new();
        
        let low_task = ScheduledTask {
            id: Uuid::new_v4(),
            task: MemTask::Cleanup { cube_id: Uuid::new_v4() },
            priority: TaskPriority::Low,
            status: TaskStatus::Pending,
            created_at: Utc::now(),
            started_at: None,
            completed_at: None,
            retry_count: 0,
            max_retries: 3,
            error: None,
            result: None,
        };
        
        let high_task = ScheduledTask {
            id: Uuid::new_v4(),
            task: MemTask::Cleanup { cube_id: Uuid::new_v4() },
            priority: TaskPriority::High,
            status: TaskStatus::Pending,
            created_at: Utc::now(),
            started_at: None,
            completed_at: None,
            retry_count: 0,
            max_retries: 3,
            error: None,
            result: None,
        };
        
        // Düşük öncelikli önce eklensin
        queue.push(low_task);
        queue.push(high_task);
        
        // Yüksek öncelikli önce çıkmalı
        let first = queue.pop().expect("operation failed");
        assert_eq!(first.priority, TaskPriority::High);
        
        let second = queue.pop().expect("operation failed");
        assert_eq!(second.priority, TaskPriority::Low);
        
        assert!(queue.is_empty());
    }
    
    #[tokio::test]
    async fn test_scheduler_schedule() {
        let scheduler = MemScheduler::default_scheduler();
        
        let entry = MemoryEntry::from_input(
            MemoryInput::new("Test bellek")
        );
        
        let task_id = scheduler.schedule_store(Uuid::new_v4(), entry).await;
        
        assert!(!task_id.is_nil());
        
        let stats = scheduler.stats().await;
        assert_eq!(stats.total_tasks, 1);
        assert_eq!(stats.pending_tasks, 1);
    }
    
    #[tokio::test]
    async fn test_queue_status() {
        let scheduler = MemScheduler::default_scheduler();
        
        let entry = MemoryEntry::from_input(
            MemoryInput::new("Test")
        );
        
        scheduler.schedule_store(Uuid::new_v4(), entry.clone()).await;
        scheduler.schedule_store_urgent(Uuid::new_v4(), entry).await;
        
        let (critical, high, normal, low) = scheduler.queue_status().await;
        
        assert_eq!(high, 1);
        assert_eq!(normal, 1);
        assert_eq!(low, 0);
    }
}
