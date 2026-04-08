//! ─── SENTIENT SWARM ORKESTRASYON SİSTEMİ ───
//!
//! Çoklu ajan koordinasyonu - Uzmanlaşmış alt ajanların hiyerarşik yönetimi.
//! Araştırmacı, Yazılımcı, Eleştirmen gibi uzman ajanların koordineli çalışması.

pub mod agent_type;
pub mod coordinator;
pub mod message;
pub mod blackboard;
pub mod protocol;
pub mod task_router;
pub mod collective;

pub use agent_type::{AgentType, AgentPersona, AgentCapability};
pub use coordinator::{SwarmCoordinator, SwarmConfig, SwarmStatus};
pub use message::{SwarmMessage, MessageType, MessagePriority};
pub use blackboard::{Blackboard, KnowledgeEntry, SharedContext};
pub use protocol::{SwarmProtocol, Handshake, Negotiation};
pub use task_router::{TaskRouter, RoutingStrategy, AgentLoad};
pub use collective::{CollectiveMemory, KnowledgeSync};

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// ─── SWARM AGENT ID ───
/// 
/// Swarm içindeki bir ajanın benzersiz tanımlayıcısı.

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SwarmAgentId(String);

impl SwarmAgentId {
    pub fn new() -> Self {
        Self(format!("swarm_{}", Uuid::new_v4()))
    }
    
    pub fn from_type(agent_type: AgentType) -> Self {
        Self(format!("{}_{}", agent_type.short_code(), Uuid::new_v4().to_string().split('-').next().unwrap_or("0")))
    }
    
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for SwarmAgentId {
    fn default() -> Self {
        Self::new()
    }
}

/// ─── SWARM TASK ───
/// 
/// Swarm'a atanacak görev tanımı.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwarmTask {
    /// Görev ID
    pub id: Uuid,
    /// Görev açıklaması
    pub description: String,
    /// Gerekli yetenekler
    pub required_capabilities: Vec<AgentCapability>,
    /// Öncelik
    pub priority: MessagePriority,
    /// Ana görev mi?
    pub is_root: bool,
    /// Üst görev ID
    pub parent_id: Option<Uuid>,
    /// Bağımlılıklar
    pub dependencies: Vec<Uuid>,
    /// Sonuç
    pub result: Option<serde_json::Value>,
    /// Durum
    pub status: SwarmTaskStatus,
    /// Atanan ajan
    pub assigned_to: Option<SwarmAgentId>,
    /// Oluşturulma zamanı
    pub created_at: DateTime<Utc>,
    /// Tamamlanma zamanı
    pub completed_at: Option<DateTime<Utc>>,
}

impl SwarmTask {
    pub fn new(description: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            description: description.into(),
            required_capabilities: Vec::new(),
            priority: MessagePriority::Normal,
            is_root: false,
            parent_id: None,
            dependencies: Vec::new(),
            result: None,
            status: SwarmTaskStatus::Pending,
            assigned_to: None,
            created_at: Utc::now(),
            completed_at: None,
        }
    }
    
    pub fn root(description: impl Into<String>) -> Self {
        let mut task = Self::new(description);
        task.is_root = true;
        task.priority = MessagePriority::Critical;
        task
    }
    
    pub fn subtask(mut self, parent: &SwarmTask) -> Self {
        self.parent_id = Some(parent.id);
        self.priority = parent.priority;
        self
    }
    
    pub fn require(mut self, capability: AgentCapability) -> Self {
        self.required_capabilities.push(capability);
        self
    }
    
    pub fn depends_on(mut self, task_id: Uuid) -> Self {
        self.dependencies.push(task_id);
        self
    }
    
    pub fn assign(&mut self, agent_id: SwarmAgentId) {
        self.assigned_to = Some(agent_id);
        self.status = SwarmTaskStatus::Assigned;
    }
    
    pub fn start(&mut self) {
        self.status = SwarmTaskStatus::Running;
    }
    
    pub fn complete(&mut self, result: serde_json::Value) {
        self.result = Some(result);
        self.status = SwarmTaskStatus::Completed;
        self.completed_at = Some(Utc::now());
    }
    
    pub fn fail(&mut self, error: String) {
        self.result = Some(serde_json::json!({"error": error}));
        self.status = SwarmTaskStatus::Failed;
        self.completed_at = Some(Utc::now());
    }
    
    pub fn duration_ms(&self) -> Option<i64> {
        self.completed_at.map(|end| {
            (end - self.created_at).num_milliseconds()
        })
    }
}

/// ─── SWARM TASK STATUS ───

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SwarmTaskStatus {
    /// Beklemede
    Pending,
    /// Atandı
    Assigned,
    /// Çalışıyor
    Running,
    /// Tamamlandı
    Completed,
    /// Başarısız
    Failed,
    /// İptal
    Cancelled,
    /// Delege edildi (başka swarm'a)
    Delegated,
}

impl SwarmTaskStatus {
    pub fn indicator(&self) -> &'static str {
        match self {
            Self::Pending => "⏳",
            Self::Assigned => "📌",
            Self::Running => "🔄",
            Self::Completed => "✅",
            Self::Failed => "❌",
            Self::Cancelled => "🛑",
            Self::Delegated => "📤",
        }
    }
    
    pub fn is_terminal(&self) -> bool {
        matches!(self, Self::Completed | Self::Failed | Self::Cancelled)
    }
}

/// ─── SWARM EXECUTION RESULT ───

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwarmResult {
    /// Sonuç ID
    pub id: Uuid,
    /// Ana görev ID
    pub root_task_id: Uuid,
    /// Başarı durumu
    pub success: bool,
    /// Sonuç özeti
    pub summary: String,
    /// Detaylı sonuç
    pub details: serde_json::Value,
    /// Katkıda bulunan ajanlar
    pub contributors: Vec<AgentContribution>,
    /// Toplam süre (ms)
    pub total_duration_ms: u64,
    /// Toplam token
    pub total_tokens: u64,
    /// Swarm istatistikleri
    pub stats: SwarmStats,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AgentContribution {
    pub agent_id: String,
    pub agent_type: AgentType,
    pub tasks_completed: u32,
    pub tokens_used: u64,
    pub duration_ms: u64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SwarmStats {
    pub total_agents: usize,
    pub active_agents: usize,
    pub total_tasks: u32,
    pub completed_tasks: u32,
    pub failed_tasks: u32,
    pub messages_exchanged: u32,
    #[serde(default)]
    pub total_tokens: u64,
}

impl SwarmResult {
    pub fn report(&self) -> String {
        let status = if self.success { "✅ BAŞARILI" } else { "❌ BAŞARISIZ" };
        
        format!(
            r#"
╔══════════════════════════════════════════════════════════════╗
║                 🐺 SWARM YÜRÜTME SONUCU                      ║
╠══════════════════════════════════════════════════════════════╣
║  Durum:        {:<44} ║
║  Süre:         {:.2}s                                           ║
║  Token:        {:<44} ║
╠══════════════════════════════════════════════════════════════╣
║  Görevler:                                                  ║
║    ✓ Tamamlanan: {:<38} ║
║    ✗ Başarısız:  {:<38} ║
╠══════════════════════════════════════════════════════════════╣
║  Ajanlar:        {:<43} ║
╠══════════════════════════════════════════════════════════════╣
║  Özet:                                                      ║
║  {:60} ║
╚══════════════════════════════════════════════════════════════╝"#,
            status,
            self.total_duration_ms as f64 / 1000.0,
            self.total_tokens,
            self.stats.completed_tasks,
            self.stats.failed_tasks,
            self.stats.active_agents,
            self.summary.chars().take(50).collect::<String>()
        )
    }
}

// ─── Tests ───

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_swarm_agent_id() {
        let id = SwarmAgentId::new();
        assert!(id.as_str().starts_with("swarm_"));
    }
    
    #[test]
    fn test_swarm_task_creation() {
        let task = SwarmTask::new("Test görevi");
        assert!(!task.is_root);
        assert_eq!(task.status, SwarmTaskStatus::Pending);
    }
    
    #[test]
    fn test_swarm_task_root() {
        let task = SwarmTask::root("Ana görev");
        assert!(task.is_root);
        assert_eq!(task.priority, MessagePriority::Critical);
    }
    
    #[test]
    fn test_swarm_task_lifecycle() {
        let mut task = SwarmTask::new("Test");
        let agent_id = SwarmAgentId::new();
        
        task.assign(agent_id.clone());
        assert_eq!(task.status, SwarmTaskStatus::Assigned);
        assert!(task.assigned_to.is_some());
        
        task.start();
        assert_eq!(task.status, SwarmTaskStatus::Running);
        
        task.complete(serde_json::json!({"result": 42}));
        assert_eq!(task.status, SwarmTaskStatus::Completed);
        assert!(task.completed_at.is_some());
        assert!(task.duration_ms().is_some());
    }
    
    #[test]
    fn test_swarm_result_report() {
        let result = SwarmResult {
            id: Uuid::new_v4(),
            root_task_id: Uuid::new_v4(),
            success: true,
            summary: "Test özeti".into(),
            details: serde_json::Value::Null,
            contributors: vec![],
            total_duration_ms: 1500,
            total_tokens: 1000,
            stats: SwarmStats {
                total_agents: 3,
                active_agents: 3,
                total_tasks: 10,
                completed_tasks: 8,
                failed_tasks: 2,
                messages_exchanged: 25,
                total_tokens: 1000,
            },
        };
        
        let report = result.report();
        assert!(report.contains("BAŞARILI"));
    }
}
