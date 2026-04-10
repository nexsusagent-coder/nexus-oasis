//! ─── TASK DISPATCHER ───
//!
//! Gateway'den gelen istekleri Orchestrator'a dispatch eder.
//! Task Manager ile koordineli çalışır.

use sentient_common::error::{SENTIENTError, SENTIENTResult};
use sentient_orchestrator::{
    Agent, AgentConfig, Goal, 
    SYSTEM_PROMPT,
};
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;

use crate::{GatewayRequest, GatewayResponse, ResponseStatus, RequestSource};
use crate::task_manager::{TaskManager, ManagedTask, TaskStatus, TaskPriority};

/// ─── DISPATCH RESULT ───

#[derive(Debug, Clone)]
pub struct DispatchResult {
    pub task_id: Uuid,
    pub accepted: bool,
    pub message: String,
    pub queue_position: usize,
}

impl DispatchResult {
    pub fn accepted(task_id: Uuid, position: usize) -> Self {
        Self {
            task_id,
            accepted: true,
            message: "Görev kabul edildi ve kuyruğa alındı".into(),
            queue_position: position,
        }
    }
    
    pub fn rejected(reason: impl Into<String>) -> Self {
        Self {
            task_id: Uuid::nil(),
            accepted: false,
            message: reason.into(),
            queue_position: 0,
        }
    }
}

/// ─── TASK DISPATCHER ───

pub struct TaskDispatcher {
    task_manager: Arc<TaskManager>,
    vgate_url: String,
    default_model: String,
}

impl TaskDispatcher {
    pub fn new(task_manager: Arc<TaskManager>) -> Self {
        Self {
            task_manager,
            vgate_url: "http://127.0.0.1:1071".into(),
            default_model: "qwen/qwen3-1.7b:free".into(),
        }
    }
    
    /// V-GATE URL ayarla
    pub fn with_vgate_url(mut self, url: impl Into<String>) -> Self {
        self.vgate_url = url.into();
        self
    }
    
    /// Default model ayarla
    pub fn with_default_model(mut self, model: impl Into<String>) -> Self {
        self.default_model = model.into();
        self
    }
    
    /// İstek dispatch et
    pub async fn dispatch(&self, request: GatewayRequest) -> SENTIENTResult<DispatchResult> {
        log::info!("dispatcher  Yeni istek: {} (kaynak: {})", 
            request.goal.chars().take(50).collect::<String>(),
            request.source
        );
        
        // İsteği doğrula
        self.validate_request(&request)?;
        
        // Öncelik belirle
        let priority = self.determine_priority(&request);
        
        // Görev oluştur
        let task = ManagedTask::new(request.clone(), priority);
        let task_id = task.id;
        
        // Görevi kuyruğa ekle
        self.task_manager.submit(task).await?;
        
        // Aktif görev sayısını kontrol et
        let active_count = self.task_manager.get_active_tasks().await.len();
        
        // Görevi arka planda çalıştır
        let task_manager = self.task_manager.clone();
        let task_id_clone = task_id;
        let request_clone = request.clone();
        let vgate_url = self.vgate_url.clone();
        let model = request.model.clone().unwrap_or_else(|| self.default_model.clone());
        
        tokio::spawn(async move {
            if let Err(e) = Self::execute_task(
                task_manager,
                task_id_clone,
                request_clone,
                vgate_url,
                model,
            ).await {
                log::error!("dispatcher  Görev hatası {}: {}", task_id_clone, e.to_sentient_message());
            }
        });
        
        Ok(DispatchResult::accepted(task_id, active_count))
    }
    
    /// İstek doğrulama
    fn validate_request(&self, request: &GatewayRequest) -> SENTIENTResult<()> {
        // Hedef boş mu?
        if request.goal.trim().is_empty() {
            return Err(SENTIENTError::ValidationError("Hedef boş olamaz".into()));
        }
        
        // Hedef çok uzun mu? (max 2000 karakter)
        if request.goal.len() > 2000 {
            return Err(SENTIENTError::ValidationError("Hedef çok uzun (max 2000 karakter)".into()));
        }
        
        Ok(())
    }
    
    /// Öncelik belirleme
    fn determine_priority(&self, request: &GatewayRequest) -> TaskPriority {
        // CLI'den gelenler yüksek öncelik
        if matches!(request.source, RequestSource::Cli) {
            return TaskPriority::High;
        }
        
        // Telegram'dan gelen komutlar
        if let RequestSource::Telegram { .. } = &request.source {
            // "acil" veya "öncelikli" geçiyorsa yüksek öncelik
            let goal_lower = request.goal.to_lowercase();
            if goal_lower.contains("acil") || goal_lower.contains("öncelikli") || goal_lower.contains("urgent") {
                return TaskPriority::High;
            }
        }
        
        TaskPriority::Normal
    }
    
    /// Görevi çalıştır
    async fn execute_task(
        task_manager: Arc<TaskManager>,
        task_id: Uuid,
        request: GatewayRequest,
        vgate_url: String,
        model: String,
    ) -> SENTIENTResult<()> {
        // Semaphore permit bekle (concurrency control)
        let semaphore = task_manager.semaphore();
        let _permit = semaphore.acquire().await
            .map_err(|_| SENTIENTError::General("Semaphore kapalı".into()))?;
        
        // Durumu güncelle: Başlatılıyor
        task_manager.update_status(task_id, TaskStatus::Starting).await?;
        
        log::info!("dispatcher  Görev başlatılıyor: {}", task_id);
        
        // Agent config
        let config = AgentConfig {
            model: model.clone(),
            vgate_url,
            max_iterations: 50,
            timeout_secs: 600,
            system_prompt: SYSTEM_PROMPT.into(),
        };
        
        // Goal oluştur
        let goal = Goal::new(&request.goal)
            .with_success_criteria(vec![
                "Görev başarıyla tamamlandı".into(),
                "Sonuçlar doğrulandı".into(),
            ]);
        
        // Durumu güncelle: Çalışıyor
        task_manager.update_status(task_id, TaskStatus::Running).await?;
        
        // Agent oluştur ve çalıştır
        let mut agent = Agent::new(goal, config);
        
        // Timeout ile çalıştır
        let result = tokio::time::timeout(
            task_manager.timeout_duration(),
            agent.run()
        ).await;
        
        match result {
            Ok(Ok(execution_result)) => {
                // Başarılı
                task_manager.set_result(task_id, execution_result.clone()).await?;
                log::info!("dispatcher  Görev tamamlandı: {} ({} iterasyon)", 
                    task_id, execution_result.total_iterations);
            }
            Ok(Err(e)) => {
                // Agent hatası
                task_manager.set_error(task_id, e.to_sentient_message()).await?;
            }
            Err(_) => {
                // Timeout
                task_manager.update_status(task_id, TaskStatus::Timeout).await?;
                log::warn!("dispatcher  Görev timeout: {}", task_id);
            }
        }
        
        Ok(())
    }
    
    /// Görevi iptal et
    pub async fn cancel_task(&self, task_id: Uuid) -> SENTIENTResult<()> {
        self.task_manager.cancel(task_id).await
    }
    
    /// Görev durumu al
    pub async fn get_task(&self, task_id: Uuid) -> Option<ManagedTask> {
        self.task_manager.get_task(task_id).await
    }
    
    /// Aktif görevleri al
    pub async fn get_active_tasks(&self) -> Vec<ManagedTask> {
        self.task_manager.get_active_tasks().await
    }
    
    /// İstatistikler
    pub async fn stats(&self) -> crate::GatewayStats {
        self.task_manager.stats().await
    }
}

/// ─── REQUEST TO RESPONSE ───

impl TaskDispatcher {
    /// İstekten hızlı yanıt oluştur (enqueue olmadan)
    pub fn quick_response(request: GatewayRequest, task_id: Uuid, status: ResponseStatus) -> GatewayResponse {
        let status_msg = match &status {
            ResponseStatus::Accepted => "Görev kabul edildi",
            ResponseStatus::Processing => "Görev işleniyor",
            ResponseStatus::Completed => "Görev tamamlandı",
            ResponseStatus::Failed => "Görev başarısız",
            ResponseStatus::Cancelled => "Görev iptal edildi",
            ResponseStatus::Timeout => "Görev zaman aşıımı",
        };
        GatewayResponse {
            request_id: request.id,
            task_id,
            status,
            message: status_msg.into(),
            result: None,
            duration_ms: 0,
            timestamp: Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_dispatch_result_accepted() {
        let result = DispatchResult::accepted(Uuid::new_v4(), 5);
        assert!(result.accepted);
        assert_eq!(result.queue_position, 5);
    }
    
    #[test]
    fn test_dispatch_result_rejected() {
        let result = DispatchResult::rejected("Test hatası");
        assert!(!result.accepted);
        assert!(result.message.contains("Test hatası"));
    }
    
    #[tokio::test]
    async fn test_dispatcher_creation() {
        let manager = Arc::new(TaskManager::new(10, 600));
        let dispatcher = TaskDispatcher::new(manager);
        
        assert_eq!(dispatcher.default_model, "qwen/qwen3-1.7b:free");
        assert_eq!(dispatcher.vgate_url, "http://127.0.0.1:1071");
    }
    
    #[test]
    fn test_request_validation_empty() {
        let manager = Arc::new(TaskManager::new(10, 600));
        let dispatcher = TaskDispatcher::new(manager);
        
        let request = GatewayRequest::new("", RequestSource::Cli);
        let result = dispatcher.validate_request(&request);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_request_validation_long() {
        let manager = Arc::new(TaskManager::new(10, 600));
        let dispatcher = TaskDispatcher::new(manager);
        
        let long_goal = "x".repeat(2500);
        let request = GatewayRequest::new(long_goal, RequestSource::Cli);
        let result = dispatcher.validate_request(&request);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_priority_determination() {
        let manager = Arc::new(TaskManager::new(10, 600));
        let dispatcher = TaskDispatcher::new(manager);
        
        // CLI yüksek öncelik
        let request = GatewayRequest::new("Test", RequestSource::Cli);
        assert_eq!(dispatcher.determine_priority(&request), TaskPriority::High);
        
        // Telegram normal
        let request = GatewayRequest::new(
            "Test", 
            RequestSource::Telegram { chat_id: 123, username: None }
        );
        assert_eq!(dispatcher.determine_priority(&request), TaskPriority::Normal);
        
        // Telegram acil
        let request = GatewayRequest::new(
            "Acil test", 
            RequestSource::Telegram { chat_id: 123, username: None }
        );
        assert_eq!(dispatcher.determine_priority(&request), TaskPriority::High);
    }
}
