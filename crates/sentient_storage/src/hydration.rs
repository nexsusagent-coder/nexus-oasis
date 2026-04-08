//! ─── HYDRATION ENGINE ───
//!
//! Sunucu yeniden başlatıldığında görevlerin otomatik olarak
//! kaldığı yerden devam etmesini sağlayan sistem.

use crate::models::*;
use crate::store::TaskStore;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

// ═══════════════════════════════════════════════════════════════
// HYDRATION TYPES
// ═══════════════════════════════════════════════════════════════

/// Hydration sonucu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HydrationResult {
    /// Toplam geri yüklenen görev sayısı
    pub restored_count: usize,
    /// Başarıyla devam eden görevler
    pub resumed_tasks: Vec<ResumedTask>,
    /// Hata ile karşılaşılan görevler
    pub failed_tasks: Vec<FailedTask>,
    /// Hydration süresi (ms)
    pub duration_ms: u64,
}

/// Devam eden görev
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResumedTask {
    pub task_id: Uuid,
    pub goal: String,
    pub agent: String,
    pub progress: f32,
    pub checkpoint_id: Option<Uuid>,
    pub resumed_at: String,
}

/// Başarısız görev
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailedTask {
    pub task_id: Uuid,
    pub goal: String,
    pub error: String,
    pub original_status: String,
}

/// Checkpoint verisi (detaylı durum)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskCheckpoint {
    pub task_id: Uuid,
    pub agent_state: serde_json::Value,
    pub conversation: Vec<ConversationTurn>,
    pub last_action: Option<String>,
    pub context: serde_json::Value,
    pub timestamp: chrono::DateTime<Utc>,
}

/// Konuşma turu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationTurn {
    pub role: String,
    pub content: String,
    pub timestamp: chrono::DateTime<Utc>,
}

// ═══════════════════════════════════════════════════════════════
// HYDRATION ENGINE
// ═══════════════════════════════════════════════════════════════

/// Hydration motoru
pub struct HydrationEngine {
    store: Arc<TaskStore>,
    config: HydrationConfig,
}

impl Clone for HydrationEngine {
    fn clone(&self) -> Self {
        Self {
            store: self.store.clone(),
            config: self.config.clone(),
        }
    }
}

/// Hydration yapılandırması
#[derive(Debug, Clone)]
pub struct HydrationConfig {
    pub max_concurrent_hydration: usize,
    pub timeout_secs: u64,
    pub auto_retry: bool,
    pub max_retries: u8,
}

impl Default for HydrationConfig {
    fn default() -> Self {
        Self {
            max_concurrent_hydration: 5,
            timeout_secs: 300,
            auto_retry: true,
            max_retries: 3,
        }
    }
}

impl HydrationEngine {
    /// Yeni hydration motoru oluştur
    pub fn new(store: Arc<TaskStore>) -> Self {
        Self {
            store,
            config: HydrationConfig::default(),
        }
    }
    
    /// Yapılandırmayı ayarla
    pub fn with_config(mut self, config: HydrationConfig) -> Self {
        self.config = config;
        self
    }
    
    /// Aktif görevleri geri yükle (sunucu başlatıldığında çağrılır)
    pub async fn hydrate(&self) -> StorageResult<HydrationResult> {
        let start = std::time::Instant::now();
        log::info!("🔄 Hydration başlatılıyor...");
        
        // 1. Aktif görevleri veritabanından çek
        let active_tasks = self.store.get_active_tasks().await?;
        log::info!("📋 {} aktif görev bulundu", active_tasks.len());
        
        let mut resumed_tasks = Vec::new();
        let mut failed_tasks = Vec::new();
        
        // 2. Her görevi geri yükle
        for task in active_tasks {
            match self.restore_task(&task).await {
                Ok(resumed) => {
                    log::info!(
                        "✅ Görev geri yüklendi: {} ({})",
                        task.id,
                        task.goal.chars().take(30).collect::<String>()
                    );
                    resumed_tasks.push(resumed);
                }
                Err(e) => {
                    log::warn!("❌ Görev geri yüklenemedi: {} - {}", task.id, e);
                    failed_tasks.push(FailedTask {
                        task_id: task.id,
                        goal: task.goal.clone(),
                        error: e.to_string(),
                        original_status: task.status.as_str().to_string(),
                    });
                }
            }
        }
        
        let duration_ms = start.elapsed().as_millis() as u64;
        
        // 3. Workflow'ları da geri yükle
        let workflows = self.store.get_active_workflows().await?;
        log::info!("📋 {} aktif workflow bulundu", workflows.len());
        
        // 4. Log kaydı
        self.store.add_log(
            Uuid::nil(),
            "INFO",
            "Hydration",
            &format!(
                "{} görev geri yüklendi, {} başarısız, {} workflow ({}ms)",
                resumed_tasks.len(),
                failed_tasks.len(),
                workflows.len(),
                duration_ms
            ),
        ).await.ok();
        
        log::info!(
            "🔄 Hydration tamamlandı: {} başarılı, {} başarısız",
            resumed_tasks.len(),
            failed_tasks.len()
        );
        
        Ok(HydrationResult {
            restored_count: resumed_tasks.len(),
            resumed_tasks,
            failed_tasks,
            duration_ms,
        })
    }
    
    /// Tek bir görevi geri yükle
    async fn restore_task(&self, task: &PersistedTask) -> StorageResult<ResumedTask> {
        // Görev adımlarını yükle
        let steps = self.store.get_task_steps(task.id).await?;
        
        // Durumu 'starting' olarak güncelle (resuming anlamında)
        self.store.update_task_status(task.id, PersistedStatus::Starting).await?;
        
        // Ajan belirle
        let agent = task.assigned_agent.clone()
            .unwrap_or_else(|| self.determine_agent_for_task(&task.goal));
        
        // Log
        self.store.add_log(
            task.id,
            "INFO",
            "Hydration",
            &format!(
                "Görev '{}' aşamasından devam ettiriliyor ({} adım tamamlandı)",
                task.status,
                steps.iter().filter(|s| s.status == "completed").count()
            ),
        ).await.ok();
        
        Ok(ResumedTask {
            task_id: task.id,
            goal: task.goal.clone(),
            agent,
            progress: task.progress,
            checkpoint_id: None,
            resumed_at: Utc::now().to_rfc3339(),
        })
    }
    
    /// Göreve uygun ajanı belirle
    fn determine_agent_for_task(&self, goal: &str) -> String {
        let goal_lower = goal.to_lowercase();
        
        if goal_lower.contains("araştır") 
            || goal_lower.contains("bul") 
            || goal_lower.contains("tara")
            || goal_lower.contains("keşfet")
            || goal_lower.contains("analiz")
            || goal_lower.contains("search")
            || goal_lower.contains("research")
        {
            "scout".to_string()
        } else if goal_lower.contains("kod")
            || goal_lower.contains("yaz")
            || goal_lower.contains("düzenle")
            || goal_lower.contains("oluştur")
            || goal_lower.contains("implement")
            || goal_lower.contains("write")
            || goal_lower.contains("fix")
            || goal_lower.contains("refactor")
        {
            "forge".to_string()
        } else if goal_lower.contains("koordinat")
            || goal_lower.contains("dağıt")
            || goal_lower.contains("parçala")
            || goal_lower.contains("yönet")
            || goal_lower.contains("coordinate")
            || goal_lower.contains("distribute")
        {
            "swarm".to_string()
        } else {
            "scout".to_string() // Varsayılan
        }
    }
    
    /// Yeni checkpoint oluştur ve kaydet
    pub async fn create_checkpoint(
        &self,
        task_id: Uuid,
        agent_state: serde_json::Value,
        conversation: Vec<ConversationTurn>,
        last_action: Option<String>,
        context: serde_json::Value,
    ) -> StorageResult<()> {
        let checkpoint = TaskCheckpoint {
            task_id,
            agent_state,
            conversation,
            last_action,
            context,
            timestamp: Utc::now(),
        };
        
        let checkpoint_json = serde_json::to_value(&checkpoint)
            .map_err(|e| StorageError::SerializationError(e.to_string()))?;
        
        self.store.save_checkpoint(task_id, &checkpoint_json).await?;
        
        // Log
        self.store.add_log(
            task_id,
            "DEBUG",
            "Checkpoint",
            "Checkpoint kaydedildi",
        ).await.ok();
        
        Ok(())
    }
    
    /// Workflow'ları geri yükle
    pub async fn hydrate_workflows(&self) -> StorageResult<Vec<WorkflowState>> {
        let workflows = self.store.get_active_workflows().await?;
        
        log::info!("📋 {} aktif workflow bulundu", workflows.len());
        
        for workflow in &workflows {
            log::info!(
                "  ⚡ {} ({}) - Durum: {}",
                workflow.name,
                workflow.workflow_type,
                workflow.status
            );
        }
        
        Ok(workflows)
    }
    
    /// Hydration özeti
    pub async fn get_hydration_summary(&self) -> String {
        let stats = self.store.get_stats().await.ok();
        match stats {
            Some(s) => format!(
                "📊 Veritabanı: {} toplam, {} aktif, {} tamamlandı, {} başarısız",
                s.total, s.active, s.completed, s.failed
            ),
            None => "📊 Veritabanı istatistikleri alınamadı".to_string(),
        }
    }
}
