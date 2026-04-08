//! Subagent Executor - DeerFlow'dan esinlenilmiş paralel agent execution
//!
//! Birden fazla subagent'i paralel çalıştırır, timeout ve cancel destekler

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use parking_lot::RwLock;
use tokio::sync::mpsc;
use tracing::{info, warn};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Subagent ID
pub type SubagentId = String;

/// Task ID
pub type TaskId = String;

/// Subagent Status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SubagentStatus {
    /// Beklemede
    Pending,
    
    /// Çalışıyor
    Running,
    
    /// Tamamlandı
    Completed,
    
    /// Başarısız
    Failed,
    
    /// İptal edildi
    Cancelled,
    
    /// Timeout
    TimedOut,
}

/// Subagent Config
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubagentConfig {
    /// Subagent adı
    pub name: String,
    
    /// Kullanılacak model (inherit = parent'tan al)
    pub model: Option<String>,
    
    /// İzin verilen tool'lar
    pub allowed_tools: Option<Vec<String>>,
    
    /// Yasaklanan tool'lar
    pub disallowed_tools: Option<Vec<String>>,
    
    /// Timeout (saniye)
    #[serde(default = "default_timeout")]
    pub timeout_secs: u64,
    
    /// Maksimum retry
    #[serde(default = "default_max_retries")]
    pub max_retries: u32,
    
    /// Parent ile state paylaşımı
    pub share_state: bool,
    
    /// Paralel execution
    pub parallel: bool,
}

fn default_timeout() -> u64 { 300 }
fn default_max_retries() -> u32 { 2 }

impl SubagentConfig {
    /// Yeni config oluştur
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            model: None,
            allowed_tools: None,
            disallowed_tools: None,
            timeout_secs: default_timeout(),
            max_retries: default_max_retries(),
            share_state: true,
            parallel: true,
        }
    }
    
    /// Model ayarla
    pub fn with_model(mut self, model: impl Into<String>) -> Self {
        self.model = Some(model.into());
        self
    }
    
    /// Tool allowlist ayarla
    pub fn with_allowed_tools(mut self, tools: Vec<String>) -> Self {
        self.allowed_tools = Some(tools);
        self
    }
    
    /// Timeout ayarla
    pub fn with_timeout(mut self, secs: u64) -> Self {
        self.timeout_secs = secs;
        self
    }
}

/// Subagent Result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubagentResult {
    /// Task ID
    pub task_id: TaskId,
    
    /// Trace ID (parent-child tracking)
    pub trace_id: String,
    
    /// Status
    pub status: SubagentStatus,
    
    /// Sonuç
    pub result: Option<String>,
    
    /// Hata mesajı
    pub error: Option<String>,
    
    /// Başlangıç zamanı
    pub started_at: Option<DateTime<Utc>>,
    
    /// Bitiş zamanı
    pub completed_at: Option<DateTime<Utc>>,
    
    /// AI mesajları
    pub messages: Vec<serde_json::Value>,
}

impl SubagentResult {
    /// Yeni result oluştur
    pub fn new(task_id: TaskId, trace_id: String) -> Self {
        Self {
            task_id,
            trace_id,
            status: SubagentStatus::Pending,
            result: None,
            error: None,
            started_at: None,
            completed_at: None,
            messages: Vec::new(),
        }
    }
    
    /// Başarılı sonuç
    pub fn success(task_id: TaskId, trace_id: String, result: String) -> Self {
        Self {
            task_id,
            trace_id,
            status: SubagentStatus::Completed,
            result: Some(result),
            error: None,
            started_at: Some(Utc::now()),
            completed_at: Some(Utc::now()),
            messages: Vec::new(),
        }
    }
    
    /// Başarısız sonuç
    pub fn failure(task_id: TaskId, trace_id: String, error: String) -> Self {
        Self {
            task_id,
            trace_id,
            status: SubagentStatus::Failed,
            result: None,
            error: Some(error),
            started_at: Some(Utc::now()),
            completed_at: Some(Utc::now()),
            messages: Vec::new(),
        }
    }
}

/// Subagent Task
#[derive(Debug, Clone)]
pub struct SubagentTask {
    /// Task ID
    pub id: TaskId,
    
    /// Config
    pub config: SubagentConfig,
    
    /// Input
    pub input: String,
    
    /// Parent context
    pub parent_context: HashMap<String, serde_json::Value>,
}

impl SubagentTask {
    /// Yeni task oluştur
    pub fn new(config: SubagentConfig, input: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            config,
            input: input.into(),
            parent_context: HashMap::new(),
        }
    }
    
    /// Context ekle
    pub fn with_context(mut self, key: impl Into<String>, value: serde_json::Value) -> Self {
        self.parent_context.insert(key.into(), value);
        self
    }
}

/// Subagent Executor
pub struct SubagentExecutor {
    /// Aktif task'ler
    tasks: Arc<RwLock<HashMap<TaskId, SubagentResult>>>,
    
    /// Maksimum paralel task
    max_parallel: usize,
    
    /// Default timeout
    default_timeout: Duration,
    
    /// Cancel channel
    cancel_tx: mpsc::Sender<TaskId>,
    cancel_rx: Option<mpsc::Receiver<TaskId>>,
}

impl SubagentExecutor {
    /// Yeni executor oluştur
    pub fn new() -> Self {
        let (cancel_tx, cancel_rx) = mpsc::channel(100);
        
        Self {
            tasks: Arc::new(RwLock::new(HashMap::new())),
            max_parallel: 5,
            default_timeout: Duration::from_secs(300),
            cancel_tx,
            cancel_rx: Some(cancel_rx),
        }
    }
    
    /// Maksimum paralel task ayarla
    pub fn with_max_parallel(mut self, max: usize) -> Self {
        self.max_parallel = max;
        self
    }
    
    /// Default timeout ayarla
    pub fn with_default_timeout(mut self, timeout: Duration) -> Self {
        self.default_timeout = timeout;
        self
    }
    
    /// Task çalıştır
    pub async fn execute(&self, task: SubagentTask) -> SubagentResult {
        let task_id = task.id.clone();
        let trace_id = Uuid::new_v4().to_string();
        
        // Task'ı kaydet
        let result = SubagentResult::new(task_id.clone(), trace_id.clone());
        {
            let mut tasks = self.tasks.write();
            tasks.insert(task_id.clone(), result);
        }
        
        info!("Executing subagent task: {} ({})", task.config.name, task_id);
        
        // Simulated execution (gerçek implementation'da LLM call olur)
        let timeout = Duration::from_secs(task.config.timeout_secs);
        
        match tokio::time::timeout(
            timeout,
            self.execute_task_internal(task)
        ).await {
            Ok(Ok(result)) => result,
            Ok(Err(e)) => SubagentResult::failure(task_id, trace_id, e),
            Err(_) => {
                warn!("Task {} timed out", task_id);
                SubagentResult::failure(task_id, trace_id, "Task timed out".to_string())
            }
        }
    }
    
    /// Internal task execution
    async fn execute_task_internal(&self, task: SubagentTask) -> Result<SubagentResult, String> {
        // TODO: Gerçek LLM integration
        // Şimdilik simüle edilmiş execution
        
        tokio::time::sleep(Duration::from_millis(100)).await;
        
        let result = format!(
            "Subagent '{}' completed task: {}",
            task.config.name,
            if task.input.len() > 50 {
                &task.input[..50]
            } else {
                &task.input
            }
        );
        
        Ok(SubagentResult::success(
            task.id,
            Uuid::new_v4().to_string(),
            result,
        ))
    }
    
    /// Birden fazla task'ı paralel çalıştır
    pub async fn execute_parallel(&self, tasks: Vec<SubagentTask>) -> Vec<SubagentResult> {
        let mut handles = Vec::new();
        
        for task in tasks {
            // Maksimum paralel kontrolü
            while self.active_count() >= self.max_parallel {
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
            
            let executor = self.clone();
            handles.push(tokio::spawn(async move {
                executor.execute(task).await
            }));
        }
        
        // Tüm task'leri bekle
        let mut results = Vec::new();
        for handle in handles {
            if let Ok(result) = handle.await {
                results.push(result);
            }
        }
        
        results
    }
    
    /// Task'ı iptal et
    pub async fn cancel(&self, task_id: &TaskId) -> bool {
        let _ = self.cancel_tx.send(task_id.clone()).await;
        
        let mut tasks = self.tasks.write();
        if let Some(result) = tasks.get_mut(task_id) {
            result.status = SubagentStatus::Cancelled;
            result.completed_at = Some(Utc::now());
            info!("Cancelled task: {}", task_id);
            true
        } else {
            false
        }
    }
    
    /// Task durumunu al
    pub fn get_status(&self, task_id: &TaskId) -> Option<SubagentStatus> {
        let tasks = self.tasks.read();
        tasks.get(task_id).map(|r| r.status)
    }
    
    /// Task sonucunu al
    pub fn get_result(&self, task_id: &TaskId) -> Option<SubagentResult> {
        let tasks = self.tasks.read();
        tasks.get(task_id).cloned()
    }
    
    /// Aktif task sayısı
    pub fn active_count(&self) -> usize {
        let tasks = self.tasks.read();
        tasks.values()
            .filter(|r| r.status == SubagentStatus::Running || r.status == SubagentStatus::Pending)
            .count()
    }
    
    /// Tüm task'leri temizle
    pub fn clear(&self) {
        let mut tasks = self.tasks.write();
        tasks.clear();
    }
}

impl Clone for SubagentExecutor {
    fn clone(&self) -> Self {
        Self {
            tasks: self.tasks.clone(),
            max_parallel: self.max_parallel,
            default_timeout: self.default_timeout,
            cancel_tx: self.cancel_tx.clone(),
            cancel_rx: None,
        }
    }
}

impl Default for SubagentExecutor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_subagent_executor() {
        let executor = SubagentExecutor::new();
        
        let config = SubagentConfig::new("test-agent")
            .with_timeout(10);
        
        let task = SubagentTask::new(config, "Test input");
        
        let result = executor.execute(task).await;
        
        assert_eq!(result.status, SubagentStatus::Completed);
        assert!(result.result.is_some());
    }
    
    #[tokio::test]
    async fn test_parallel_execution() {
        let executor = SubagentExecutor::new().with_max_parallel(3);
        
        let tasks: Vec<SubagentTask> = (0..5)
            .map(|i| {
                SubagentTask::new(
                    SubagentConfig::new(format!("agent-{}", i)),
                    format!("Task {}", i),
                )
            })
            .collect();
        
        let results = executor.execute_parallel(tasks).await;
        
        assert_eq!(results.len(), 5);
    }
}
