//! ═══════════════════════════════════════════════════════════════════════════════
//!  MULTI-AGENT ORCHESTRATOR - Çoklu Agent Koordinasyonu
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Birden fazla agent'ı koordine eden orchestrator.
//!
//! ┌─────────────────────────────────────────────────────────────────────────┐
//! │                        ORCHESTRATOR                                     │
//! │  ┌───────────────────────────────────────────────────────────────────┐ │
//! │  │                     COORDINATOR AGENT                             │ │
//! │  └───────────────────────────────────────────────────────────────────┘ │
//! │                              │                                          │
//! │           ┌──────────────────┼──────────────────┐                      │
//! │           │                  │                  │                       │
//! │           ▼                  ▼                  ▼                       │
//! │    ┌───────────┐      ┌───────────┐      ┌───────────┐                │
//! │    │  BROWSER  │      │   HANDS   │      │   MANUS   │                │
//! │    │   AGENT   │      │   AGENT   │      │   AGENT   │                │
//! │    └───────────┘      └───────────┘      └───────────┘                │
//! │           │                  │                  │                       │
//! │           └──────────────────┴──────────────────┘                      │
//! │                              │                                          │
//! │                        MESSAGE BUS                                      │
//! └─────────────────────────────────────────────────────────────────────────┘

use crate::error::{AutonomousError, AutonomousResult};
use crate::AgentId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};

// ═══════════════════════════════════════════════════════════════════════════════
//  AGENT MESSAGE
// ═══════════════════════════════════════════════════════════════════════════════

/// Agent mesajı
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMessage {
    /// Gönderen ID
    pub from: AgentId,
    /// Alıcı ID (None = broadcast)
    pub to: Option<AgentId>,
    /// Mesaj tipi
    pub message_type: MessageType,
    /// Payload
    pub payload: serde_json::Value,
    /// Zaman damgası
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Mesaj tipi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    /// Görev ataması
    TaskAssignment,
    /// Görev sonucu
    TaskResult,
    /// Durum güncelleme
    StatusUpdate,
    /// Veri paylaşımı
    DataShare,
    /// Yardım isteği
    HelpRequest,
    /// Koordinasyon
    Coordination,
    /// Senkronizasyon
    Sync,
    /// Hata bildirimi
    Error,
    /// Özel
    Custom(String),
}

// ═══════════════════════════════════════════════════════════════════════════════
//  AGENT INFO
// ═══════════════════════════════════════════════════════════════════════════════

/// Agent bilgisi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentInfo {
    /// Agent ID
    pub id: AgentId,
    /// Agent adı
    pub name: String,
    /// Agent türü
    pub agent_type: AgentType,
    /// Durum
    pub status: AgentStatus,
    /// Yetenekler
    pub capabilities: Vec<String>,
    /// Mevcut görev
    pub current_task: Option<String>,
    /// İş yükü (0-1)
    pub workload: f32,
    /// Son aktif
    pub last_active: chrono::DateTime<chrono::Utc>,
}

/// Agent türü
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentType {
    Coordinator,
    Browser,
    Desktop,
    Executor,
    Observer,
    Planner,
    Custom,
}

/// Agent durumu
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentStatus {
    Idle,
    Busy,
    Paused,
    Error,
    Offline,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  TASK ASSIGNMENT
// ═══════════════════════════════════════════════════════════════════════════════

/// Görev ataması
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskAssignment {
    /// Görev ID
    pub task_id: String,
    /// Görev açıklaması
    pub description: String,
    /// Öncelik
    pub priority: u8,
    /// Hedef agent
    pub assigned_to: AgentId,
    /// Bağımlılıklar
    pub dependencies: Vec<String>,
    /// Deadline
    pub deadline: Option<chrono::DateTime<chrono::Utc>>,
    /// Parametreler
    pub params: HashMap<String, serde_json::Value>,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  ORCHESTRATION STRATEGY
// ═══════════════════════════════════════════════════════════════════════════════

/// Orkestrasyon stratejisi
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrchestrationStrategy {
    /// Round-robin
    RoundRobin,
    /// En az yükte olan
    LeastLoaded,
    /// Yetenek bazlı
    CapabilityBased,
    /// Rastgele
    Random,
    /// Manuel
    Manual,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  MULTI-AGENT ORCHESTRATOR
// ═══════════════════════════════════════════════════════════════════════════════

/// Çoklu agent orkestratörü
pub struct MultiAgentOrchestrator {
    /// Kayıtlı agentlar
    agents: Arc<RwLock<HashMap<AgentId, AgentInfo>>>,
    /// Mesaj veriyolu
    message_bus: broadcast::Sender<AgentMessage>,
    /// Görev kuyruğu
    task_queue: Arc<RwLock<Vec<TaskAssignment>>>,
    /// Strateji
    strategy: OrchestrationStrategy,
    /// Koordinatör ID
    coordinator_id: AgentId,
}

impl MultiAgentOrchestrator {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(100);
        
        log::info!("🎭 ORCH: Multi-agent orchestrator başlatılıyor...");
        
        Self {
            agents: Arc::new(RwLock::new(HashMap::new())),
            message_bus: tx,
            task_queue: Arc::new(RwLock::new(Vec::new())),
            strategy: OrchestrationStrategy::LeastLoaded,
            coordinator_id: AgentId::default(),
        }
    }
    
    /// Agent kaydet
    pub async fn register_agent(&self, info: AgentInfo) {
        log::info!("🎭 ORCH: Agent registered: {} ({:?})", info.name, info.agent_type);
        
        let mut agents = self.agents.write().await;
        agents.insert(info.id.clone(), info);
    }
    
    /// Agent kaldır
    pub async fn unregister_agent(&self, id: &AgentId) {
        let mut agents = self.agents.write().await;
        agents.remove(id);
        
        log::info!("🎭 ORCH: Agent unregistered: {}", id);
    }
    
    /// Görev ata
    pub async fn assign_task(&self, assignment: TaskAssignment) -> AutonomousResult<()> {
        log::info!("🎭 ORCH: Task '{}' assigned to {}", assignment.task_id, assignment.assigned_to);
        
        // Agent'ı kontrol et
        let agents = self.agents.read().await;
        if !agents.contains_key(&assignment.assigned_to) {
            return Err(AutonomousError::OrchestrationError(
                format!("Agent not found: {}", assignment.assigned_to)
            ));
        }
        drop(agents);
        
        // Mesaj gönder
        let message = AgentMessage {
            from: self.coordinator_id.clone(),
            to: Some(assignment.assigned_to.clone()),
            message_type: MessageType::TaskAssignment,
            payload: serde_json::to_value(&assignment)?,
            timestamp: chrono::Utc::now(),
        };
        
        let _ = self.message_bus.send(message);
        
        Ok(())
    }
    
    /// Otomatik görev dağıtımı
    pub async fn distribute_task(&self, description: &str, priority: u8) -> AutonomousResult<AgentId> {
        let agents = self.agents.read().await;
        
        if agents.is_empty() {
            return Err(AutonomousError::OrchestrationError("No agents available".into()));
        }
        
        // Stratejiye göre agent seç
        let selected_id = match self.strategy {
            OrchestrationStrategy::RoundRobin => {
                self.select_round_robin(&agents)
            }
            OrchestrationStrategy::LeastLoaded => {
                self.select_least_loaded(&agents)
            }
            OrchestrationStrategy::CapabilityBased => {
                self.select_by_capability(&agents, description)
            }
            OrchestrationStrategy::Random => {
                self.select_random(&agents)
            }
            OrchestrationStrategy::Manual => {
                return Err(AutonomousError::OrchestrationError("Manual strategy requires explicit assignment".into()));
            }
        };
        
        drop(agents);
        
        // Görev ata
        let assignment = TaskAssignment {
            task_id: uuid::Uuid::new_v4().to_string(),
            description: description.into(),
            priority,
            assigned_to: selected_id.clone(),
            dependencies: vec![],
            deadline: None,
            params: HashMap::new(),
        };
        
        self.assign_task(assignment).await?;
        
        Ok(selected_id)
    }
    
    fn select_round_robin(&self, agents: &HashMap<AgentId, AgentInfo>) -> AgentId {
        // Basit round-robin
        agents.keys().next().expect("operation failed").clone()
    }
    
    fn select_least_loaded(&self, agents: &HashMap<AgentId, AgentInfo>) -> AgentId {
        agents.iter()
            .min_by(|a, b| a.1.workload.partial_cmp(&b.1.workload).expect("operation failed"))
            .map(|(id, _)| id.clone())
            .expect("operation failed")
    }
    
    fn select_by_capability(&self, agents: &HashMap<AgentId, AgentInfo>, task: &str) -> AgentId {
        // Basit yetenek eşleştirme
        for (id, info) in agents {
            for cap in &info.capabilities {
                if task.to_lowercase().contains(&cap.to_lowercase()) {
                    return id.clone();
                }
            }
        }
        
        // Bulunamazsa ilk agent
        agents.keys().next().expect("operation failed").clone()
    }
    
    fn select_random(&self, agents: &HashMap<AgentId, AgentInfo>) -> AgentId {
        use rand::seq::IteratorRandom;
        agents.keys().choose(&mut rand::thread_rng()).expect("operation failed").clone()
    }
    
    /// Broadcast mesaj
    pub fn broadcast(&self, message_type: MessageType, payload: serde_json::Value) {
        let message = AgentMessage {
            from: self.coordinator_id.clone(),
            to: None,
            message_type,
            payload,
            timestamp: chrono::Utc::now(),
        };
        
        let _ = self.message_bus.send(message);
    }
    
    /// Mesaj alıcısı oluştur
    pub fn subscribe(&self) -> broadcast::Receiver<AgentMessage> {
        self.message_bus.subscribe()
    }
    
    /// Agent durumunu güncelle
    pub async fn update_agent_status(&self, id: &AgentId, status: AgentStatus) {
        let mut agents = self.agents.write().await;
        if let Some(info) = agents.get_mut(id) {
            info.status = status;
            info.last_active = chrono::Utc::now();
        }
    }
    
    /// Agent istatistikleri
    pub async fn stats(&self) -> OrchestrationStats {
        let agents = self.agents.read().await;
        
        OrchestrationStats {
            total_agents: agents.len(),
            idle_agents: agents.values().filter(|a| a.status == AgentStatus::Idle).count(),
            busy_agents: agents.values().filter(|a| a.status == AgentStatus::Busy).count(),
            average_workload: agents.values().map(|a| a.workload).sum::<f32>() / agents.len().max(1) as f32,
        }
    }
}

impl Default for MultiAgentOrchestrator {
    fn default() -> Self {
        Self::new()
    }
}

/// Orkestrasyon istatistikleri
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestrationStats {
    pub total_agents: usize,
    pub idle_agents: usize,
    pub busy_agents: usize,
    pub average_workload: f32,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_orchestrator_creation() {
        let orch = MultiAgentOrchestrator::new();
        let stats = orch.stats().await;
        assert_eq!(stats.total_agents, 0);
    }
    
    #[tokio::test]
    async fn test_register_agent() {
        let orch = MultiAgentOrchestrator::new();
        
        let info = AgentInfo {
            id: AgentId::default(),
            name: "test".into(),
            agent_type: AgentType::Browser,
            status: AgentStatus::Idle,
            capabilities: vec!["browser".into()],
            current_task: None,
            workload: 0.0,
            last_active: chrono::Utc::now(),
        };
        
        orch.register_agent(info).await;
        
        let stats = orch.stats().await;
        assert_eq!(stats.total_agents, 1);
    }
    
    #[tokio::test]
    async fn test_distribute_task() {
        let orch = MultiAgentOrchestrator::new();
        
        let info = AgentInfo {
            id: AgentId::default(),
            name: "test".into(),
            agent_type: AgentType::Browser,
            status: AgentStatus::Idle,
            capabilities: vec!["browser".into()],
            current_task: None,
            workload: 0.0,
            last_active: chrono::Utc::now(),
        };
        
        orch.register_agent(info).await;
        
        let result = orch.distribute_task("test task", 5).await;
        assert!(result.is_ok());
    }
}
