//! ─── SENTIENT ORCHESTRATOR (AGENT LOOP) ───
//!
//! SENTIENT'nın kalbi — LLM, Browser ve Sandbox'ı orkestre eden
//! otonom görev döngüsü (main loop).
//!
//! 8. Adım: Çoklu Ajan Orkestrasyonu (Multi-Agent Swarm)
//! - Farklı uzmanlıklara sahip alt ajanlar
//! - Hiyerarşik yapı ve koordinasyon
//! - Swarm protokolü ile bilgi paylaşımı
//!
//! 11. Adım: Self-Healing (Otonom Düzeltme)
//! - Hata paterni analizi
//! - Otomatik kod düzeltme
//! - Yeniden deneme stratejileri
//!
//! 12. Adım: Watcher (Otonom Gözcü)
//! - Belirli aralıklarla Scout/Forge tetikleme
//! - Zamanlanmış görevler
//! - Tetikleyici bazlı görevler
//!
//! v4.0.0: Dynamic Complexity Routing & Multi-Key Vault
//! - Görev zorluğuna göre otomatik model seçimi
//! - Human-in-the-Loop onay mekanizması

pub mod agent;
pub mod goal;
pub mod planner;
pub mod tools;
pub mod state;
pub mod execution;
pub mod task_queue;
pub mod persistent_state;
pub mod workflow_engine;

/// ─── MEMORY BRIDGE MODÜLÜ ───
/// L7: Memory-Orchestrator entegrasyonu
pub mod memory_bridge;

/// ─── RESEARCH BRIDGE MODÜLÜ ───
/// L5: Research-Orchestrator entegrasyonu
pub mod research_bridge;

/// ─── SKILLS HUB MODÜLÜ ───
/// Asimile edilmiş rakip yeteneklerin yönetimi
pub mod skills;

/// ─── SWARM MODÜLÜ ───
/// Çoklu ajan orkestrasyon sistemi
pub mod swarm;

/// ─── COORDINATION MODÜLÜ ───
/// Multi-agent koordinasyon sistemi
pub mod coordination;

/// ─── SELF-HEALING MODÜLÜ ───
/// 11. Adım: Hata alan kodun otonom düzeltilmesi
pub mod self_healing;

/// ─── WATCHER MODÜLÜ ───
/// 12. Adım: Scout ve Forge birimlerini tetikleyen otonom gözcü
pub mod watcher;

/// ─── DYNAMIC ROUTER MODÜLÜ ─── (v4.0.0)
/// Görev zorluğuna göre otomatik model seçimi
pub mod dynamic_router;

pub use agent::{Agent, AgentConfig};
pub use goal::{Goal, Task};
pub use state::{AgentContext};
pub use planner::{Planner, ExecutionPlan};
pub use state::{AgentContext as OrchAgentContext};
pub use execution::{ExecutionResult, StepResult};

use sentient_common::error::SENTIENTResult;

/// SENTIENT Sistem Promptu (agent.rs'den re-export)
pub const SYSTEM_PROMPT: &str = agent::SYSTEM_PROMPT;

/// Swarm sistemini dışa aktar
pub use swarm::{
    SwarmCoordinator, SwarmConfig, SwarmStatus,
    SwarmTask, SwarmTaskStatus, SwarmResult,
    AgentType, AgentPersona, AgentCapability,
    Blackboard, CollectiveMemory,
};

/// Self-Healing sistemini dışa aktar
pub use self_healing::{
    SelfHealingEngine, HealingConfig, HealingStrategy,
    HealingResult, HealingRecord, HealingStats,
    ErrorPattern, ErrorCategory, CodeFix,
};

/// Watcher sistemini dışa aktar
pub use watcher::{
    Watcher, WatcherConfig, WatcherStatus, WatcherStats,
    WatcherTask, TaskSource,
    ScheduledTask, ScheduledTaskType, RepeatType,
    Trigger, TriggerCondition, TriggerAction,
};

/// Memory Bridge sistemini dışa aktar
pub use memory_bridge::{
    MemoryBridge, BridgeConfig, BridgeStats,
    ReActContext, WorkingMemory, WorkingMemoryItem, WorkingItemType,
    WorkingMemoryState, Experience, Procedure,
    ConsolidationResult,
};

/// Research Bridge sistemini dışa aktar
pub use research_bridge::{
    ResearchBridge, BridgeConfig as ResearchBridgeConfig, BridgeStatus,
    ResearchOutput, ResearchTask, ResearchType,
    ResearchMemory, WebSearchResult,
};

/// Dynamic Router sistemini dışa aktar (v4.0.0)
pub use dynamic_router::{
    DynamicRouter, RouterConfig, RouterDecision,
    ComplexityAnalyzer, TaskAnalysis, TaskType,
};

/// Task Queue + Priority Queue + Agent Pool + Distributed Swarm + Marketplace
pub use task_queue::{
    QueuedTask, TaskPriority as OrchTaskPriority, TaskType as OrchTaskType, TaskStatus as QueueTaskStatus,
    PersistentTaskQueue, QueueStats,
    AgentPool, AgentPoolConfig, AgentState as PoolAgentState, PooledAgent, PoolStats,
    DistributedSwarmCoordinator, DistributedSwarmConfig,
    SwarmNodeConfig, SwarmNodeRole, SwarmNodeState, SwarmNode, ClusterStatus,
    AgentMarketplace, AgentListing,
};

/// ─── ORCHESTRATOR ───
/// 
/// SENTIENT'nın ana orkestrasyon merkezi.
/// Tüm bileşenleri koordine eder ve görev döngüsünü yönetir.

pub struct Orchestrator {
    /// Ajan yapılandırması
    config: OrchestratorConfig,
    /// Bellek sistemi
    memory: std::sync::Arc<tokio::sync::RwLock<sentient_memory::MemoryCube>>,
    /// Sandbox (optional)
    sandbox: Option<std::sync::Arc<tokio::sync::RwLock<sentient_sandbox::Sandbox>>>,
    /// Aktif ajanlar
    agents: std::sync::Arc<tokio::sync::RwLock<Vec<Agent>>>,
    /// Global bağlam
    context: std::sync::Arc<tokio::sync::RwLock<AgentContext>>,
    /// Başlangıç zamanı
    start_time: std::time::Instant,
}

/// Orchestrator yapılandırması
#[derive(Debug, Clone)]
pub struct OrchestratorConfig {
    /// V-GATE sunucu adresi
    pub vgate_url: String,
    /// Varsayılan LLM modeli
    pub default_model: String,
    /// Maksimum görev süresi (saniye)
    pub max_task_duration_secs: u64,
    /// Maksimum döngü sayısı
    pub max_iterations: u32,
    /// Paralel ajan sayısı
    pub max_parallel_agents: usize,
    /// Hata durumunda yeniden deneme
    pub retry_on_error: bool,
    /// Maksimum yeniden deneme
    pub max_retries: u32,
    /// Swarm modu aktif mi?
    pub use_swarm: bool,
}

impl Default for OrchestratorConfig {
    fn default() -> Self {
        Self {
            vgate_url: "http://127.0.0.1:1071".into(),
            default_model: "qwen/qwen3-1.7b:free".into(),
            max_task_duration_secs: 300, // 5 dakika
            max_iterations: 50,
            max_parallel_agents: 3,
            retry_on_error: true,
            max_retries: 3,
            use_swarm: true, // Varsayılan olarak swarm aktif
        }
    }
}

impl Orchestrator {
    /// Yeni orchestrator oluştur
    pub async fn new(config: OrchestratorConfig) -> SENTIENTResult<Self> {
        log::info!("🧠  ORCHESTRATOR başlatılıyor...");
        
        // Bellek sistemini başlat
        let memory = std::sync::Arc::new(tokio::sync::RwLock::new(
            sentient_memory::MemoryCube::new("data/orchestrator_memory.db")?
        ));
        
        // Sandbox (optional - requires E2B_API_KEY and running service)
        // Sandbox will be None by default, created on-demand when needed
        let sandbox = None;
        
        // Global bağlam
        let context = std::sync::Arc::new(tokio::sync::RwLock::new(
            AgentContext::default()
        ));
        
        log::info!("✅  ORCHESTRATOR hazır");
        
        Ok(Self {
            config,
            memory,
            sandbox,
            agents: std::sync::Arc::new(tokio::sync::RwLock::new(Vec::new())),
            context,
            start_time: std::time::Instant::now(),
        })
    }
    
    /// Yeni bir görev başlat (ANA GİRİŞ NOKTASI)
    pub async fn execute(&self, goal: Goal) -> SENTIENTResult<ExecutionResult> {
        log::info!("════════════════════════════════════════════════════════════");
        log::info!("🎯  YENİ GÖREV: {}", goal.description.chars().take(60).collect::<String>());
        log::info!("════════════════════════════════════════════════════════════");
        
        let start_time = std::time::Instant::now();
        
        // Swarm ile mi yoksa tek ajan ile mi çalışsın?
        if self.config.use_swarm {
            return self.execute_with_swarm(goal).await;
        }
        
        // Tek ajan modu
        let agent_config = AgentConfig {
            model: self.config.default_model.clone(),
            vgate_url: self.config.vgate_url.clone(),
            max_iterations: self.config.max_iterations,
            timeout_secs: self.config.max_task_duration_secs,
            system_prompt: SYSTEM_PROMPT.into(),
        };
        
        let mut agent = Agent::new(goal.clone(), agent_config);
        
        // Görevi yürüt
        let result = agent.run().await?;
        
        let duration = start_time.elapsed();
        log::info!("════════════════════════════════════════════════════════════");
        log::info!("🎯  GÖREV TAMAMLANDI: {:.2}s", duration.as_secs_f64());
        log::info!("════════════════════════════════════════════════════════════");
        
        // Sonucu belleğe kaydet
        let input = sentient_memory::MemoryInput::new(
            serde_json::to_string(&result).unwrap_or_default()
        )
        .with_type(sentient_memory::MemoryType::Semantic)
        .with_source(sentient_memory::MemorySource::InternalInference)
        .with_importance(sentient_memory::Importance::high());
        
        self.memory.write().await.create_with_metadata(
            input.content,
            input.memory_type,
            Some(serde_json::to_value(&input.metadata).unwrap_or(serde_json::json!({}))),
            None,
        )?;
        
        Ok(result)
    }
    
    /// Swarm ile yürüt
    async fn execute_with_swarm(&self, goal: Goal) -> SENTIENTResult<ExecutionResult> {
        log::info!("🐺  Swarm modu aktif");
        
        let swarm_config = swarm::SwarmConfig {
            model: self.config.default_model.clone(),
            vgate_url: self.config.vgate_url.clone(),
            max_iterations: self.config.max_iterations,
            ..Default::default()
        };
        
        let mut coordinator = swarm::SwarmCoordinator::new(swarm_config);
        coordinator.start().await?;
        
        let result = coordinator.execute(goal).await?;
        
        // ExecutionResult'a donustur
        Ok(ExecutionResult::success(
            &Goal::new(&result.summary),
            vec![],
            result.stats.total_tasks, // iterations
            result.total_tokens,
            result.total_duration_ms,
        ))
    }
    
    /// Paralel görevler başlat
    pub async fn execute_parallel(&self, goals: Vec<Goal>) -> SENTIENTResult<Vec<ExecutionResult>> {
        log::info!("🔀  {} paralel görev başlatılıyor...", goals.len());
        
        let mut tasks = Vec::new();
        
        for goal in goals {
            let agent_config = AgentConfig {
                model: self.config.default_model.clone(),
                vgate_url: self.config.vgate_url.clone(),
                max_iterations: self.config.max_iterations,
                timeout_secs: self.config.max_task_duration_secs,
                system_prompt: SYSTEM_PROMPT.into(),
            };
            
            let mut agent = Agent::new(goal, agent_config);
            tasks.push(tokio::spawn(async move { agent.run().await }));
        }
        
        let results: Vec<_> = futures::future::join_all(tasks).await;
        
        let mut final_results = Vec::new();
        for result in results {
            match result {
                Ok(Ok(r)) => final_results.push(r),
                Ok(Err(e)) => log::error!("Görev hatası: {}", e.to_sentient_message()),
                Err(e) => log::error!("Tokio hatası: {}", e),
            }
        }
        
        Ok(final_results)
    }
    
    /// Durum raporu
    pub async fn status(&self) -> OrchestratorStatus {
        let agents = self.agents.read().await;
        let memory = self.memory.read().await;
        let memory_count = memory.count().unwrap_or(0);
        
        OrchestratorStatus {
            active_agents: agents.len(),
            memory_entries: memory_count as usize,
            uptime_secs: self.start_time.elapsed().as_secs(),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct OrchestratorStatus {
    pub active_agents: usize,
    pub memory_entries: usize,
    pub uptime_secs: u64,
}

// ─── Tests ───

#[cfg(test)]
mod tests {
    use super::*;
    use crate::goal::TaskPriority;
    
    #[test]
    fn test_orchestrator_config_default() {
        let config = OrchestratorConfig::default();
        assert_eq!(config.max_iterations, 50);
        assert_eq!(config.max_parallel_agents, 3);
    }
    
    #[test]
    fn test_goal_creation() {
        let goal = Goal::new("Test hedefi");
        assert!(!goal.description.is_empty());
        assert_eq!(goal.priority, TaskPriority::Normal);
    }
}
