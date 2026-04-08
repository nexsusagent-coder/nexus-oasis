//! ─── SWARM KOORDİNATÖRÜ ───
//!
//! Tüm swarm sistemini yöneten ana koordinatör.
//! Ajanları başlatır, görevleri dağıtır, iletişimi koordine eder.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;
use tokio::sync::mpsc;

use super::{
    SwarmAgentId, SwarmTask, SwarmTaskStatus, SwarmResult, SwarmStats,
    AgentContribution,
};
use super::agent_type::{AgentType, AgentPersona, AgentCapability};
use super::message::{SwarmMessage, MessageType, MessagePriority, MessageQueue};
use super::blackboard::Blackboard;
use super::protocol::{SwarmProtocol, Negotiation};
use super::task_router::{TaskRouter, RoutingStrategy, TaskAssignment};
use super::collective::CollectiveMemory;

use crate::goal::Goal;
use crate::planner::ExecutionPlan;
use crate::execution::ExecutionResult;
use crate::state::AgentState;
use sentient_common::error::{SENTIENTError, SENTIENTResult};

/// ─── SWARM COORDINATOR ───
/// 
/// Tüm swarm'ı yöneten merkezi koordinatör.

pub struct SwarmCoordinator {
    /// Yapılandırma
    config: SwarmConfig,
    /// Kayıtlı ajanlar
    agents: Arc<RwLock<HashMap<SwarmAgentId, SwarmAgent>>>,
    /// Görev yönlendirici
    router: Arc<RwLock<TaskRouter>>,
    /// Protokol yöneticisi
    protocol: Arc<RwLock<SwarmProtocol>>,
    /// Ortak bilgi alanı
    blackboard: Arc<Blackboard>,
    /// Toplu bellek
    collective_memory: CollectiveMemory,
    /// Mesaj kanalları
    message_sender: mpsc::Sender<SwarmMessage>,
    message_receiver: Option<mpsc::Receiver<SwarmMessage>>,
    /// İstatistikler
    stats: Arc<RwLock<SwarmStats>>,
    /// Durum
    status: Arc<RwLock<SwarmStatus>>,
    /// Başlangıç zamanı
    started_at: DateTime<Utc>,
}

impl SwarmCoordinator {
    /// Yeni koordinatör oluştur
    pub fn new(config: SwarmConfig) -> Self {
        let (tx, rx) = mpsc::channel(1000);
        
        let blackboard = Arc::new(Blackboard::new());
        let collective_memory = CollectiveMemory::new(blackboard.clone());
        
        Self {
            config,
            agents: Arc::new(RwLock::new(HashMap::new())),
            router: Arc::new(RwLock::new(TaskRouter::new())),
            protocol: Arc::new(RwLock::new(SwarmProtocol::new())),
            blackboard,
            collective_memory,
            message_sender: tx,
            message_receiver: Some(rx),
            stats: Arc::new(RwLock::new(SwarmStats::default())),
            status: Arc::new(RwLock::new(SwarmStatus::Initializing)),
            started_at: Utc::now(),
        }
    }
    
    /// Swarm'ı başlat
    pub async fn start(&mut self) -> SENTIENTResult<()> {
        log::info!("════════════════════════════════════════════════════════════");
        log::info!("🐺  SWARM KOORDİNATÖR BAŞLATILIYOR");
        log::info!("════════════════════════════════════════════════════════════");
        
        *self.status.write() = SwarmStatus::Starting;
        
        // Varsayılan ajanları başlat
        self.spawn_default_agents().await?;
        
        // Mesaj işleyiciyi başlat
        self.start_message_handler().await?;
        
        *self.status.write() = SwarmStatus::Running;
        
        log::info!("✅  SWARM hazır - {} ajan aktif", self.agents.read().len());
        log::info!("════════════════════════════════════════════════════════════");
        
        Ok(())
    }
    
    /// Varsayılan ajanları oluştur
    async fn spawn_default_agents(&mut self) -> SENTIENTResult<()> {
        // Koordinatör ajan
        self.spawn_agent(AgentPersona::new(AgentType::Coordinator)
            .with_name("Ana Koordinatör")
            .with_priority(2.0)
            .with_max_tasks(10)
        ).await?;
        
        // Araştırmacı ajan
        self.spawn_agent(AgentPersona::new(AgentType::Researcher)
            .with_name("Baş Araştırmacı")
            .specialize("Genel araştırma")
        ).await?;
        
        // Yazılımcı ajan
        self.spawn_agent(AgentPersona::new(AgentType::Coder)
            .with_name("Python Uzmanı")
            .specialize("Python, JavaScript")
        ).await?;
        
        // Eleştirmen ajan
        self.spawn_agent(AgentPersona::new(AgentType::Critic)
            .with_name("Kalite Kontrol")
        ).await?;
        
        // Planlayıcı ajan
        self.spawn_agent(AgentPersona::new(AgentType::Planner)
            .with_name("Stratejist")
        ).await?;
        
        // Yürütücü ajanlar (birden fazla)
        for i in 0..self.config.executor_count {
            self.spawn_agent(AgentPersona::new(AgentType::Executor)
                .with_name(format!("Yürütücü {}", i + 1))
            ).await?;
        }
        
        // Web uzmanı
        self.spawn_agent(AgentPersona::new(AgentType::WebSurfer)
            .with_name("Web Gezgini")
        ).await?;
        
        // Bellek uzmanı
        self.spawn_agent(AgentPersona::new(AgentType::MemoryKeeper)
            .with_name("Arşivci")
        ).await?;
        
        Ok(())
    }
    
    /// Yeni ajan oluştur
    pub async fn spawn_agent(&mut self, persona: AgentPersona) -> SENTIENTResult<SwarmAgentId> {
        let agent_type = persona.agent_type;
        let display_name = persona.display_name();
        let agent_id = SwarmAgentId::from_type(persona.agent_type);
        let persona_clone = persona.clone();
        
        let agent = SwarmAgent {
            id: agent_id.clone(),
            persona,
            state: AgentState::Idle,
            current_task: None,
            completed_tasks: 0,
            failed_tasks: 0,
            total_tokens: 0,
            created_at: Utc::now(),
            last_active: Utc::now(),
        };
        
        self.agents.write().insert(agent_id.clone(), agent);
        self.router.write().register_agent(agent_id.clone(), persona_clone);
        
        log::info!("  {} {} spwan edildi", 
            agent_type.emoji(),
            display_name
        );
        
        Ok(agent_id)
    }
    
    /// Ajanı kaldır
    pub async fn kill_agent(&mut self, agent_id: &SwarmAgentId) -> SENTIENTResult<bool> {
        let removed = self.agents.write().remove(agent_id).is_some();
        self.router.write().deregister_agent(agent_id);
        
        if removed {
            log::info!("  👋 Ajan kaldırıldı: {}", agent_id.as_str());
        }
        
        Ok(removed)
    }
    
    /// Mesaj işleyiciyi başlat
    async fn start_message_handler(&self) -> SENTIENTResult<()> {
        // Mesaj işleyici arka planda çalışacak
        // Gerçek implementasyonda tokio::spawn kullanılır
        log::debug!("📨  Mesaj işleyici başlatıldı");
        Ok(())
    }
    
    /// ─── ANA GÖREV YÜRÜTME ───
    
    /// Hedefi swarm'a ilet ve yürüt
    pub async fn execute(&mut self, goal: Goal) -> SENTIENTResult<SwarmResult> {
        let start_time = std::time::Instant::now();
        
        log::info!("════════════════════════════════════════════════════════════");
        log::info!("🎯  SWARM GÖREVİ BAŞLATILIYOR");
        log::info!("════════════════════════════════════════════════════════════");
        log::info!("📋  Hedef: {}", goal.description.chars().take(60).collect::<String>());
        log::info!("🐺  Aktif ajanlar: {}", self.agents.read().len());
        
        // 1. Ana görev oluştur
        let root_task = SwarmTask::root(&goal.description)
            .require(AgentCapability::DecisionMaking);
        
        // 2. Görevi böl (Planner ajan ile)
        let subtasks = self.decompose_task(&root_task, &goal).await?;
        
        log::info!("📊  {} alt görev oluşturuldu", subtasks.len());
        
        // 3. Görevleri kuyruğa ekle
        for task in &subtasks {
            self.router.write().submit_task(task.clone());
        }
        
        // 4. Görevleri ata ve yürüt
        let mut completed = Vec::new();
        let mut failed = Vec::new();
        let mut iteration = 0;
        
        while iteration < self.config.max_iterations {
            iteration += 1;
            
            // Sonraki gorevi ata
            let assignment_opt = self.router.write().assign_next();
            
            if let Some(assignment) = assignment_opt {
                let task_id = assignment.task.id;
                let task_desc = assignment.task.description.clone();
                
                log::debug!("  -> {} -> {}", 
                    task_desc.chars().take(30).collect::<String>(),
                    assignment.agent_id.as_str()
                );
                
                // Gorevi yurut (simulasyon)
                let result = self.execute_task(&assignment).await;
                
                match result {
                    Ok(task_result) => {
                        self.router.write().task_completed(
                            task_id,
                            true,
                            100
                        );
                        completed.push(task_result);
                    }
                    Err(e) => {
                        self.router.write().task_completed(
                            task_id,
                            false,
                            0
                        );
                        failed.push(e.to_sentient_message());
                    }
                }
            } else if self.router.read().active_count() == 0 {
                // Tum gorevler tamamlandi
                break;
            }
            
            // İlerleme kontrolü
            if iteration % 10 == 0 {
                self.update_progress().await;
            }
        }
        
        // 5. Sonuçları topla
        let duration_ms = start_time.elapsed().as_millis() as u64;
        let success = failed.is_empty() || (completed.len() as f32 / (completed.len() + failed.len()) as f32) > 0.7;
        
        let result = SwarmResult {
            id: Uuid::new_v4(),
            root_task_id: root_task.id,
            success,
            summary: if success {
                "Swarm görevi başarıyla tamamlandı".into()
            } else {
                format!("Swarm görevi kısmen tamamlandı: {} başarılı, {} başarısız", 
                    completed.len(), failed.len())
            },
            details: serde_json::json!({
                "completed": completed.len(),
                "failed": failed.len()
            }),
            contributors: self.get_contributors(),
            total_duration_ms: duration_ms,
            total_tokens: self.stats.read().total_tokens,
            stats: SwarmStats {
                total_agents: self.agents.read().len(),
                active_agents: self.agents.read().values().filter(|a| a.state != AgentState::Idle).count(),
                total_tasks: completed.len() as u32 + failed.len() as u32,
                completed_tasks: completed.len() as u32,
                failed_tasks: failed.len() as u32,
                messages_exchanged: iteration as u32,
                total_tokens: self.stats.read().total_tokens,
            },
        };
        
        log::info!("════════════════════════════════════════════════════════════");
        log::info!("{}", result.report());
        
        Ok(result)
    }
    
    /// Görevi alt görevlere böl
    async fn decompose_task(&mut self, task: &SwarmTask, goal: &Goal) -> SENTIENTResult<Vec<SwarmTask>> {
        let mut tasks = Vec::new();
        
        // Basit görev bölme mantığı
        // Gerçek implementasyonda Planner ajan LLM ile karar verir
        
        let desc = &goal.description.to_lowercase();
        
        if desc.contains("araştır") || desc.contains("bul") {
            tasks.push(SwarmTask::new("Web'de araştırma yap")
                .subtask(task)
                .require(AgentCapability::WebSearch));
            tasks.push(SwarmTask::new("Bulguları kaydet")
                .subtask(task)
                .require(AgentCapability::MemoryStorage));
        }
        
        if desc.contains("kod") || desc.contains("program") || desc.contains("yaz") {
            tasks.push(SwarmTask::new("Kod yaz")
                .subtask(task)
                .require(AgentCapability::CodeGeneration));
            tasks.push(SwarmTask::new("Kodu test et")
                .subtask(task)
                .require(AgentCapability::Testing));
        }
        
        if desc.contains("web") || desc.contains("site") {
            tasks.push(SwarmTask::new("Siteye git")
                .subtask(task)
                .require(AgentCapability::WebBrowsing));
        }
        
        // Varsayılan görevler
        if tasks.is_empty() {
            tasks.push(SwarmTask::new("1. Analiz et")
                .subtask(task)
                .require(AgentCapability::DecisionMaking));
            tasks.push(SwarmTask::new("2. Bilgi topla")
                .subtask(task)
                .require(AgentCapability::InformationSynthesis));
            tasks.push(SwarmTask::new("3. Görevi gerçekleştir")
                .subtask(task)
                .require(AgentCapability::Execution));
            tasks.push(SwarmTask::new("4. Sonucu kontrol et")
                .subtask(task)
                .require(AgentCapability::Evaluation));
        }
        
        Ok(tasks)
    }
    
    /// Tek bir görevi yürüt
    async fn execute_task(&mut self, assignment: &TaskAssignment) -> SENTIENTResult<serde_json::Value> {
        // Simülasyon - gerçek implementasyonda ajan LLM'i kullanır
        log::debug!("🔧  Yürütülüyor: {}", assignment.task.description.chars().take(40).collect::<String>());
        
        // Bağlamı güncelle
        self.blackboard.update_context("current_task", serde_json::json!({
            "id": assignment.task.id,
            "description": assignment.task.description
        }));
        
        // Ajan durumunu güncelle
        if let Some(agent) = self.agents.write().get_mut(&assignment.agent_id) {
            agent.state = AgentState::Acting;
            agent.current_task = Some(assignment.task.id);
        }
        
        // Simüle edilmiş sonuç
        let result = serde_json::json!({
            "task_id": assignment.task.id,
            "status": "completed",
            "output": format!("{} tamamlandı", assignment.task.description)
        });
        
        // İstatistikleri güncelle
        self.stats.write().total_tokens += 100;
        
        // Ajanı serbest bırak
        if let Some(agent) = self.agents.write().get_mut(&assignment.agent_id) {
            agent.state = AgentState::Idle;
            agent.current_task = None;
            agent.completed_tasks += 1;
            agent.last_active = Utc::now();
        }
        
        Ok(result)
    }
    
    /// İlerlemeyi güncelle
    async fn update_progress(&self) {
        let router_report = self.router.read().report();
        let memory_stats = self.collective_memory.stats();
        
        log::debug!("📊  {} | {}", router_report, memory_stats);
    }
    
    /// Katkıda bulunanları al
    fn get_contributors(&self) -> Vec<AgentContribution> {
        self.agents.read().values()
            .filter(|a| a.completed_tasks > 0)
            .map(|a| AgentContribution {
                agent_id: a.id.as_str().to_string(),
                agent_type: a.persona.agent_type,
                tasks_completed: a.completed_tasks,
                tokens_used: a.total_tokens,
                duration_ms: (a.last_active - a.created_at).num_milliseconds() as u64,
            })
            .collect()
    }
    
    /// Durum raporu
    pub fn status(&self) -> SwarmStatusReport {
        let agents = self.agents.read();
        let stats = self.stats.read();
        
        SwarmStatusReport {
            status: *self.status.read(),
            total_agents: agents.len(),
            active_agents: agents.values().filter(|a| a.state != AgentState::Idle).count(),
            pending_tasks: self.router.read().pending_count(),
            active_tasks: self.router.read().active_count(),
            uptime_secs: (Utc::now() - self.started_at).num_seconds() as u64,
            stats: stats.clone(),
        }
    }
    
    /// Broadcast mesaj
    pub fn broadcast(&self, message_type: MessageType, content: serde_json::Value) {
        let msg = SwarmMessage::new(
            SwarmAgentId::from_type(AgentType::Coordinator),
            message_type,
            content
        ).broadcast();
        
        // Mesajı gönder (kanal kapalıysa ignore et)
        let _ = self.message_sender.try_send(msg);
    }
    
    /// Belirli bir ajana mesaj
    pub fn send_to(&self, to: SwarmAgentId, message_type: MessageType, content: serde_json::Value) {
        let msg = SwarmMessage::new(
            SwarmAgentId::from_type(AgentType::Coordinator),
            message_type,
            content
        ).to(to);
        
        let _ = self.message_sender.try_send(msg);
    }
    
    /// Swarm'ı durdur
    pub async fn shutdown(&mut self) -> SENTIENTResult<()> {
        log::info!("🛑  SWARM kapatılıyor...");
        
        *self.status.write() = SwarmStatus::ShuttingDown;
        
        // Aktif görevleri iptal et
        self.router.write().clear();
        
        // Ajanları serbest bırak
        self.agents.write().clear();
        
        *self.status.write() = SwarmStatus::Stopped;
        
        log::info!("👋  SWARM durduruldu");
        Ok(())
    }
}

/// ─── SWARM CONFIG ───

#[derive(Debug, Clone)]
pub struct SwarmConfig {
    /// Maksimum eşzamanlı ajan
    pub max_agents: usize,
    /// Yürütücü ajan sayısı
    pub executor_count: usize,
    /// Maksimum iterasyon
    pub max_iterations: u32,
    /// Görev zaman aşımı (saniye)
    pub task_timeout_secs: u64,
    /// Hata durumunda yeniden deneme
    pub retry_on_failure: bool,
    /// Maksimum yeniden deneme
    pub max_retries: u32,
    /// LLM modeli
    pub model: String,
    /// V-GATE URL
    pub vgate_url: String,
}

impl Default for SwarmConfig {
    fn default() -> Self {
        Self {
            max_agents: 20,
            executor_count: 3,
            max_iterations: 100,
            task_timeout_secs: 300,
            retry_on_failure: true,
            max_retries: 3,
            model: "qwen/qwen3-1.7b:free".into(),
            vgate_url: "http://127.0.0.1:1071".into(),
        }
    }
}

/// ─── SWARM STATUS ───

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SwarmStatus {
    Initializing,
    Starting,
    Running,
    Paused,
    ShuttingDown,
    Stopped,
    Error,
}

impl SwarmStatus {
    pub fn indicator(&self) -> &'static str {
        match self {
            Self::Initializing => "🔄",
            Self::Starting => "🚀",
            Self::Running => "✅",
            Self::Paused => "⏸️",
            Self::ShuttingDown => "🛑",
            Self::Stopped => "💤",
            Self::Error => "❌",
        }
    }
}

/// ─── SWARM AGENT ───

#[derive(Debug, Clone)]
pub struct SwarmAgent {
    pub id: SwarmAgentId,
    pub persona: AgentPersona,
    pub state: AgentState,
    pub current_task: Option<Uuid>,
    pub completed_tasks: u32,
    pub failed_tasks: u32,
    pub total_tokens: u64,
    pub created_at: DateTime<Utc>,
    pub last_active: DateTime<Utc>,
}

/// ─── SWARM STATUS REPORT ───

#[derive(Debug, Clone, Serialize)]
pub struct SwarmStatusReport {
    pub status: SwarmStatus,
    pub total_agents: usize,
    pub active_agents: usize,
    pub pending_tasks: usize,
    pub active_tasks: usize,
    pub uptime_secs: u64,
    pub stats: SwarmStats,
}

impl std::fmt::Display for SwarmStatusReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} Swarm: {} ajan ({} aktif) | {} görev | uptime: {}s",
            self.status.indicator(),
            self.total_agents,
            self.active_agents,
            self.pending_tasks + self.active_tasks,
            self.uptime_secs
        )
    }
}

// ─── Tests ───

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_swarm_config_default() {
        let config = SwarmConfig::default();
        assert_eq!(config.max_agents, 20);
        assert_eq!(config.executor_count, 3);
    }
    
    #[test]
    fn test_swarm_status_indicator() {
        assert!(!SwarmStatus::Running.indicator().is_empty());
    }
    
    #[tokio::test]
    async fn test_coordinator_creation() {
        let config = SwarmConfig::default();
        let coordinator = SwarmCoordinator::new(config);
        assert_eq!(*coordinator.status.read(), SwarmStatus::Initializing);
    }
    
    #[tokio::test]
    async fn test_agent_spawn() {
        let config = SwarmConfig::default();
        let mut coordinator = SwarmCoordinator::new(config);
        
        let persona = AgentPersona::new(AgentType::Researcher)
            .with_name("Test Araştırmacı");
        
        let result = coordinator.spawn_agent(persona).await;
        assert!(result.is_ok());
    }
}
