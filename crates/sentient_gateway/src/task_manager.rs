//! ─── TASK MANAGER ───
//! 
//! Gateway üzerinden gelen görevleri yönetir:
//! - Görev oluşturma ve izleme
//! - Eşzamanlılık limiti
//! - Timeout yönetimi
//! - Sonuç saklama

use sentient_common::error::{SENTIENTError, SENTIENTResult};
use sentient_orchestrator::ExecutionResult;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{RwLock, mpsc, Semaphore};
use tokio::time::Duration;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::{GatewayRequest, RequestSource, GatewayStats};

/// ─── MANAGED TASK ───

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManagedTask {
    /// Görev ID'si
    pub id: Uuid,
    
    /// İlgili istek
    pub request_id: Uuid,
    
    /// İstek kaynağı
    pub source: RequestSource,
    
    /// Kullanıcı ID'si
    pub user_id: Option<String>,
    
    /// Hedef
    pub goal: String,
    
    /// Model
    pub model: String,
    
    /// Durum
    pub status: TaskStatus,
    
    /// Öncelik
    pub priority: TaskPriority,
    
    /// Başlangıç zamanı
    pub started_at: DateTime<Utc>,
    
    /// Bitiş zamanı (varsa)
    pub completed_at: Option<DateTime<Utc>>,
    
    /// Sonuç (varsa)
    pub result: Option<ExecutionResult>,
    
    /// Hata mesajı (varsa)
    pub error: Option<String>,
    
    /// İlerleme yüzdesi
    pub progress: f32,
    
    /// Log kayıtları
    pub logs: Vec<TaskLogEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    /// Kuyrukta bekliyor
    Queued,
    
    /// Başlatılıyor
    Starting,
    
    /// Çalışıyor
    Running,
    
    /// Başarıyla tamamlandı
    Completed,
    
    /// Hata ile bitti
    Failed,
    
    /// İptal edildi
    Cancelled,
    
    /// Timeout
    Timeout,
}

impl TaskStatus {
    pub fn is_active(&self) -> bool {
        matches!(self, Self::Queued | Self::Starting | Self::Running)
    }
    
    pub fn is_terminal(&self) -> bool {
        matches!(self, Self::Completed | Self::Failed | Self::Cancelled | Self::Timeout)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum TaskPriority {
    Low = 0,
    Normal = 1,
    High = 2,
    Critical = 3,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskLogEntry {
    pub timestamp: DateTime<Utc>,
    pub level: String,
    pub message: String,
}

impl ManagedTask {
    pub fn new(request: GatewayRequest, priority: TaskPriority) -> Self {
        Self {
            id: Uuid::new_v4(),
            request_id: request.id,
            source: request.source,
            user_id: request.user_id,
            goal: request.goal,
            model: request.model.unwrap_or_else(|| "qwen/qwen3-1.7b:free".into()),
            status: TaskStatus::Queued,
            priority,
            started_at: Utc::now(),
            completed_at: None,
            result: None,
            error: None,
            progress: 0.0,
            logs: Vec::new(),
        }
    }
    
    pub fn log(&mut self, level: &str, message: impl Into<String>) {
        self.logs.push(TaskLogEntry {
            timestamp: Utc::now(),
            level: level.into(),
            message: message.into(),
        });
    }
    
    pub fn duration_secs(&self) -> i64 {
        match self.completed_at {
            Some(end) => (end - self.started_at).num_seconds(),
            None => (Utc::now() - self.started_at).num_seconds(),
        }
    }
}

/// ─── TASK MANAGER ───

pub struct TaskManager {
    /// Aktif görevler
    tasks: Arc<RwLock<HashMap<Uuid, ManagedTask>>>,
    
    /// Maksimum eşzamanlı görev
    max_concurrent: usize,
    
    /// Semaphore (concurrency control)
    semaphore: Arc<Semaphore>,
    
    /// Görev timeout (saniye)
    timeout_secs: u64,
    
    /// Görev kanalı
    #[allow(dead_code)]
    task_tx: mpsc::Sender<TaskCommand>,
    #[allow(dead_code)]
    task_rx: Option<mpsc::Receiver<TaskCommand>>,
    
    /// İstatistikler
    stats: Arc<RwLock<GatewayStats>>,
    
    /// Başlangıç zamanı
    start_time: DateTime<Utc>,
    
    /// Çalışıyor mu?
    running: Arc<RwLock<bool>>,
}

#[allow(dead_code)]
enum TaskCommand {
    Execute(Box<ManagedTask>),
    Cancel(Uuid),
    Status(Uuid, mpsc::Sender<Option<ManagedTask>>),
    Shutdown,
}

impl TaskManager {
    pub fn new(max_concurrent: usize, timeout_secs: u64) -> Self {
        let (task_tx, task_rx) = mpsc::channel(256);
        
        Self {
            tasks: Arc::new(RwLock::new(HashMap::new())),
            max_concurrent,
            semaphore: Arc::new(Semaphore::new(max_concurrent)),
            timeout_secs,
            task_tx,
            task_rx: Some(task_rx),
            stats: Arc::new(RwLock::new(GatewayStats {
                total_requests: 0,
                active_tasks: 0,
                completed_tasks: 0,
                failed_tasks: 0,
                cancelled_tasks: 0,
                uptime_secs: 0,
                requests_per_source: HashMap::new(),
            })),
            start_time: Utc::now(),
            running: Arc::new(RwLock::new(false)),
        }
    }
    
    /// Yöneticiyi başlat
    pub async fn start(&self) -> SENTIENTResult<()> {
        *self.running.write().await = true;
        log::info!("📊  Task Manager başlatıldı (max: {} concurrent)", self.max_concurrent);
        Ok(())
    }
    
    /// Durdur
    pub async fn shutdown(&self) {
        *self.running.write().await = false;
        log::info!("📊  Task Manager durduruldu");
        
        // Tüm aktif görevleri iptal et
        let mut tasks = self.tasks.write().await;
        for (_, task) in tasks.iter_mut() {
            if task.status.is_active() {
                task.status = TaskStatus::Cancelled;
                task.completed_at = Some(Utc::now());
            }
        }
    }
    
    /// Yeni görev ekle
    pub async fn submit(&self, mut task: ManagedTask) -> SENTIENTResult<Uuid> {
        let task_id = task.id;
        
        // İstatistik güncelle
        {
            let mut stats = self.stats.write().await;
            stats.total_requests += 1;
            *stats.requests_per_source.entry(format!("{}", task.source)).or_insert(0) += 1;
        }
        
        // Görevi kaydet
        task.status = TaskStatus::Queued;
        task.log("INFO", "Görev kuyruğa eklendi");
        
        self.tasks.write().await.insert(task_id, task);
        
        log::info!("📥  Görev kuyruğa eklendi: {} ({})", task_id, self.tasks.read().await.get(&task_id).map(|t| t.goal.as_str()).unwrap_or("?"));
        
        Ok(task_id)
    }
    
    /// Görev durumunu güncelle
    pub async fn update_status(&self, task_id: Uuid, status: TaskStatus) -> SENTIENTResult<()> {
        let mut tasks = self.tasks.write().await;
        
        if let Some(task) = tasks.get_mut(&task_id) {
            let old_status = task.status.clone();
            task.status = status.clone();
            
            if status.is_terminal() {
                task.completed_at = Some(Utc::now());
            }
            
            task.log("INFO", format!("Durum: {:?} → {:?}", old_status, status));
            
            // İstatistik güncelle
            let mut stats = self.stats.write().await;
            match status {
                TaskStatus::Completed => stats.completed_tasks += 1,
                TaskStatus::Failed => stats.failed_tasks += 1,
                TaskStatus::Cancelled => stats.cancelled_tasks += 1,
                _ => {}
            }
        }
        
        Ok(())
    }
    
    /// İlerleme güncelle
    pub async fn update_progress(&self, task_id: Uuid, progress: f32) -> SENTIENTResult<()> {
        let mut tasks = self.tasks.write().await;
        
        if let Some(task) = tasks.get_mut(&task_id) {
            task.progress = progress.clamp(0.0, 100.0);
        }
        
        Ok(())
    }
    
    /// Sonuç kaydet
    pub async fn set_result(&self, task_id: Uuid, result: ExecutionResult) -> SENTIENTResult<()> {
        let mut tasks = self.tasks.write().await;
        
        if let Some(task) = tasks.get_mut(&task_id) {
            task.result = Some(result);
            task.status = TaskStatus::Completed;
            task.completed_at = Some(Utc::now());
            task.progress = 100.0;
            task.log("INFO", "Görev başarıyla tamamlandı");
        }
        
        let mut stats = self.stats.write().await;
        stats.completed_tasks += 1;
        
        Ok(())
    }
    
    /// Hata kaydet
    pub async fn set_error(&self, task_id: Uuid, error: String) -> SENTIENTResult<()> {
        let mut tasks = self.tasks.write().await;
        
        if let Some(task) = tasks.get_mut(&task_id) {
            task.error = Some(error.clone());
            task.status = TaskStatus::Failed;
            task.completed_at = Some(Utc::now());
            task.log("ERROR", format!("Hata: {}", error));
        }
        
        let mut stats = self.stats.write().await;
        stats.failed_tasks += 1;
        
        Ok(())
    }
    
    /// Görevi iptal et
    pub async fn cancel(&self, task_id: Uuid) -> SENTIENTResult<()> {
        let mut tasks = self.tasks.write().await;
        
        if let Some(task) = tasks.get_mut(&task_id) {
            if !task.status.is_active() {
                return Err(SENTIENTError::General("Görev zaten tamamlandı".into()));
            }
            
            task.status = TaskStatus::Cancelled;
            task.completed_at = Some(Utc::now());
            task.log("INFO", "Görev iptal edildi");
        }
        
        Ok(())
    }
    
    /// Görev al
    pub async fn get_task(&self, task_id: Uuid) -> Option<ManagedTask> {
        self.tasks.read().await.get(&task_id).cloned()
    }
    
    /// Aktif görevleri al
    pub async fn get_active_tasks(&self) -> Vec<ManagedTask> {
        self.tasks
            .read()
            .await
            .values()
            .filter(|t| t.status.is_active())
            .cloned()
            .collect()
    }
    
    /// Tüm görevleri al
    pub async fn get_all_tasks(&self) -> Vec<ManagedTask> {
        self.tasks.read().await.values().cloned().collect()
    }
    
    /// Son N görevi al
    pub async fn get_recent_tasks(&self, n: usize) -> Vec<ManagedTask> {
        let mut tasks: Vec<_> = self.tasks.read().await.values().cloned().collect();
        tasks.sort_by(|a, b| b.started_at.cmp(&a.started_at));
        tasks.into_iter().take(n).collect()
    }
    
    /// İstatistikler
    pub async fn stats(&self) -> GatewayStats {
        let mut stats = self.stats.read().await.clone();
        stats.active_tasks = self.get_active_tasks().await.len();
        stats.uptime_secs = (Utc::now() - self.start_time).num_seconds() as u64;
        stats
    }
    
    /// Semaphore permit al
    pub fn semaphore(&self) -> Arc<Semaphore> {
        self.semaphore.clone()
    }
    
    /// Timeout süresi
    pub fn timeout_duration(&self) -> Duration {
        Duration::from_secs(self.timeout_secs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_managed_task_creation() {
        let request = crate::GatewayRequest::new("Test hedefi", RequestSource::Cli);
        let task = ManagedTask::new(request, TaskPriority::Normal);
        
        assert_eq!(task.status, TaskStatus::Queued);
        assert_eq!(task.priority, TaskPriority::Normal);
        assert!(task.result.is_none());
    }
    
    #[test]
    fn test_task_status_active() {
        assert!(TaskStatus::Queued.is_active());
        assert!(TaskStatus::Running.is_active());
        assert!(!TaskStatus::Completed.is_active());
        assert!(!TaskStatus::Failed.is_active());
    }
    
    #[test]
    fn test_task_status_terminal() {
        assert!(TaskStatus::Completed.is_terminal());
        assert!(TaskStatus::Failed.is_terminal());
        assert!(TaskStatus::Cancelled.is_terminal());
        assert!(!TaskStatus::Running.is_terminal());
    }
    
    #[test]
    fn test_task_log() {
        let request = crate::GatewayRequest::new("Test", RequestSource::Cli);
        let mut task = ManagedTask::new(request, TaskPriority::Normal);
        
        task.log("INFO", "Test mesajı");
        assert_eq!(task.logs.len(), 1);
        assert_eq!(task.logs[0].level, "INFO");
        assert_eq!(task.logs[0].message, "Test mesajı");
    }
    
    #[test]
    fn test_task_priority_ordering() {
        assert!(TaskPriority::Critical > TaskPriority::High);
        assert!(TaskPriority::High > TaskPriority::Normal);
        assert!(TaskPriority::Normal > TaskPriority::Low);
    }
    
    #[tokio::test]
    async fn test_task_manager_creation() {
        let manager = TaskManager::new(10, 600);
        assert_eq!(manager.max_concurrent, 10);
        assert_eq!(manager.timeout_secs, 600);
    }
}
