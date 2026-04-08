//! ─── GOREV YONLENDIRME SISTEMI ───
//!
//! Gorevleri yetenek bazli olarak uygun ajanlara yonlendirme.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::{HashMap, BinaryHeap};
use std::cmp::Ordering;

use super::{SwarmAgentId, SwarmTask, SwarmTaskStatus};
use super::agent_type::{AgentType, AgentCapability, AgentPersona};

/// ─── TASK ROUTER ───
/// 
/// Gorevleri uygun ajanlara yonlendirir.

pub struct TaskRouter {
    /// Kayitli ajanlar
    agents: HashMap<SwarmAgentId, AgentInfo>,
    /// Bekleyen gorevler (priority queue)
    pending_tasks: BinaryHeap<PrioritizedTask>,
    /// Aktif gorevler
    active_tasks: HashMap<Uuid, ActiveTaskAssignment>,
    /// Yonlendirme stratejisi
    strategy: RoutingStrategy,
    /// Istatistikler
    stats: RouterStats,
}

impl TaskRouter {
    pub fn new() -> Self {
        Self {
            agents: HashMap::new(),
            pending_tasks: BinaryHeap::new(),
            active_tasks: HashMap::new(),
            strategy: RoutingStrategy::default(),
            stats: RouterStats::default(),
        }
    }
    
    /// Ajan kaydet
    pub fn register_agent(&mut self, agent_id: SwarmAgentId, persona: AgentPersona) {
        let info = AgentInfo {
            id: agent_id.clone(),
            persona,
            current_load: AgentLoad::Idle,
            assigned_tasks: Vec::new(),
            total_completed: 0,
            total_failed: 0,
            avg_completion_time_ms: 0,
            last_heartbeat: Utc::now(),
            is_active: true,
        };
        
        self.agents.insert(agent_id, info);
        log::debug!("📋 ROUTER: Ajan kaydedildi");
    }
    
    /// Ajan kaldir
    pub fn deregister_agent(&mut self, agent_id: &SwarmAgentId) -> bool {
        let removed = self.agents.remove(agent_id).is_some();
        removed
    }
    
    /// Gorev ekle
    pub fn submit_task(&mut self, task: SwarmTask) {
        let prioritized = PrioritizedTask {
            task,
            submitted_at: Utc::now(),
        };
        self.pending_tasks.push(prioritized);
        self.stats.total_submitted += 1;
    }
    
    /// Sonraki gorevi al ve ata
    pub fn assign_next(&mut self) -> Option<TaskAssignment> {
        // Musait ajani bul ve ID'sini kaydet
        let available_agent_id = self.find_available_agent_id()?;
        
        // Uygun gorevi bul
        while let Some(prioritized) = self.pending_tasks.pop() {
            let task = prioritized.task;
            
            // Ajani tekrar al
            if let Some(agent) = self.agents.get(&available_agent_id) {
                // Ajan bu gorevi yapabilir mi?
                if self.can_handle(agent, &task) {
                    let agent_id = agent.id.clone();
                    
                    // Gorevi ata
                    let assignment = TaskAssignment {
                        task: task.clone(),
                        agent_id: agent_id.clone(),
                        assigned_at: Utc::now(),
                        expected_duration_ms: self.estimate_duration(agent, &task),
                    };
                    
                    // Durumu guncelle
                    self.update_agent_state(&agent_id, &task);
                    self.active_tasks.insert(task.id, ActiveTaskAssignment {
                        task_id: task.id,
                        agent_id: agent_id.clone(),
                        started_at: Utc::now(),
                    });
                    
                    self.stats.total_assigned += 1;
                    return Some(assignment);
                }
            }
        }
        
        None
    }
    
    /// Musait ajan ID'si bul
    fn find_available_agent_id(&self) -> Option<SwarmAgentId> {
        self.agents.values()
            .filter(|a| a.is_active && a.current_load.can_accept())
            .min_by(|a, b| a.current_load.load().cmp(&b.current_load.load()))
            .map(|a| a.id.clone())
    }
    
    /// Ajan gorevi yapabilir mi?
    fn can_handle(&self, agent: &AgentInfo, task: &SwarmTask) -> bool {
        let agent_caps: std::collections::HashSet<_> = agent.persona.all_capabilities().into_iter().collect();
        task.required_capabilities.iter().all(|c| agent_caps.contains(c))
    }
    
    /// Sure tahmini
    fn estimate_duration(&self, agent: &AgentInfo, task: &SwarmTask) -> u64 {
        let base_estimate = match task.required_capabilities.len() {
            0 => 1000,
            1..=2 => 5000,
            3..=4 => 15000,
            _ => 30000,
        };
        
        let modifier = if agent.avg_completion_time_ms > 0 {
            agent.avg_completion_time_ms as f64 / 10000.0
        } else {
            1.0
        };
        
        (base_estimate as f64 * modifier) as u64
    }
    
    /// Ajan durumunu guncelle
    fn update_agent_state(&mut self, agent_id: &SwarmAgentId, task: &SwarmTask) {
        if let Some(agent) = self.agents.get_mut(agent_id) {
            agent.assigned_tasks.push(task.id);
            agent.current_load = agent.current_load.with_added_task();
        }
    }
    
    /// Gorev tamamlandi bildirimi
    pub fn task_completed(&mut self, task_id: Uuid, success: bool, duration_ms: u64) {
        if let Some(active) = self.active_tasks.remove(&task_id) {
            if let Some(agent) = self.agents.get_mut(&active.agent_id) {
                agent.assigned_tasks.retain(|&id| id != task_id);
                agent.current_load = agent.current_load.with_removed_task();
                
                if success {
                    agent.total_completed += 1;
                    self.stats.total_completed += 1;
                } else {
                    agent.total_failed += 1;
                    self.stats.total_failed += 1;
                }
                
                agent.avg_completion_time_ms = (agent.avg_completion_time_ms + duration_ms) / 2;
            }
        }
    }
    
    /// Ajan yuku al
    pub fn get_agent_load(&self, agent_id: &SwarmAgentId) -> Option<AgentLoad> {
        self.agents.get(agent_id).map(|a| a.current_load)
    }
    
    /// Bekleyen gorev sayisi
    pub fn pending_count(&self) -> usize {
        self.pending_tasks.len()
    }
    
    /// Aktif gorev sayisi
    pub fn active_count(&self) -> usize {
        self.active_tasks.len()
    }
    
    /// Ajan sayisi
    pub fn agent_count(&self) -> usize {
        self.agents.len()
    }
    
    /// Aktif ajan sayisi
    pub fn active_agent_count(&self) -> usize {
        self.agents.values().filter(|a| a.is_active).count()
    }
    
    /// Istatistikler
    pub fn stats(&self) -> &RouterStats {
        &self.stats
    }
    
    /// Tum gorevleri temizle
    pub fn clear(&mut self) {
        self.pending_tasks.clear();
        self.active_tasks.clear();
    }
    
    /// Rapor
    pub fn report(&self) -> RouterReport {
        RouterReport {
            total_agents: self.agents.len(),
            active_agents: self.agents.values().filter(|a| a.is_active).count(),
            pending_tasks: self.pending_tasks.len(),
            active_tasks: self.active_tasks.len(),
            stats: self.stats.clone(),
        }
    }
}

impl Default for TaskRouter {
    fn default() -> Self {
        Self::new()
    }
}

/// ─── AGENT INFO ───

#[derive(Debug, Clone)]
pub struct AgentInfo {
    pub id: SwarmAgentId,
    pub persona: AgentPersona,
    pub current_load: AgentLoad,
    pub assigned_tasks: Vec<Uuid>,
    pub total_completed: u32,
    pub total_failed: u32,
    pub avg_completion_time_ms: u64,
    pub last_heartbeat: DateTime<Utc>,
    pub is_active: bool,
}

/// ─── AGENT LOAD ───

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentLoad {
    Idle,
    Light,
    Moderate,
    Heavy,
    Overloaded,
}

impl AgentLoad {
    pub fn load(&self) -> u8 {
        match self {
            Self::Idle => 0,
            Self::Light => 1,
            Self::Moderate => 2,
            Self::Heavy => 3,
            Self::Overloaded => 4,
        }
    }
    
    pub fn can_accept(&self) -> bool {
        !matches!(self, Self::Overloaded)
    }
    
    pub fn with_added_task(&self) -> Self {
        match self {
            Self::Idle => Self::Light,
            Self::Light => Self::Moderate,
            Self::Moderate => Self::Heavy,
            Self::Heavy | Self::Overloaded => Self::Overloaded,
        }
    }
    
    pub fn with_removed_task(&self) -> Self {
        match self {
            Self::Overloaded => Self::Heavy,
            Self::Heavy => Self::Moderate,
            Self::Moderate => Self::Light,
            Self::Light | Self::Idle => Self::Idle,
        }
    }
    
    pub fn indicator(&self) -> &'static str {
        match self {
            Self::Idle => "💤",
            Self::Light => "🟢",
            Self::Moderate => "🟡",
            Self::Heavy => "🟠",
            Self::Overloaded => "🔴",
        }
    }
}

impl Default for AgentLoad {
    fn default() -> Self {
        Self::Idle
    }
}

/// ─── ROUTING STRATEGY ───

#[derive(Debug, Clone)]
pub struct RoutingStrategy {
    pub prefer_least_loaded: bool,
    pub prefer_best_match: bool,
    pub randomize: bool,
    pub prefer_local: bool,
}

impl Default for RoutingStrategy {
    fn default() -> Self {
        Self {
            prefer_least_loaded: true,
            prefer_best_match: true,
            randomize: false,
            prefer_local: true,
        }
    }
}

/// ─── PRIORITIZED TASK ───

#[derive(Debug, Clone)]
pub struct PrioritizedTask {
    pub task: SwarmTask,
    pub submitted_at: DateTime<Utc>,
}

impl PartialEq for PrioritizedTask {
    fn eq(&self, other: &Self) -> bool {
        self.task.priority == other.task.priority &&
        self.submitted_at == other.submitted_at
    }
}

impl Eq for PrioritizedTask {}

impl PartialOrd for PrioritizedTask {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PrioritizedTask {
    fn cmp(&self, other: &Self) -> Ordering {
        self.task.priority.cmp(&other.task.priority)
            .then_with(|| other.submitted_at.cmp(&self.submitted_at))
    }
}

/// ─── TASK ASSIGNMENT ───

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskAssignment {
    pub task: SwarmTask,
    pub agent_id: SwarmAgentId,
    pub assigned_at: DateTime<Utc>,
    pub expected_duration_ms: u64,
}

#[derive(Debug, Clone)]
pub struct ActiveTaskAssignment {
    pub task_id: Uuid,
    pub agent_id: SwarmAgentId,
    pub started_at: DateTime<Utc>,
}

/// ─── ROUTER STATS ───

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RouterStats {
    pub total_submitted: u64,
    pub total_assigned: u64,
    pub total_completed: u64,
    pub total_failed: u64,
    pub total_rerouted: u64,
}

impl RouterStats {
    pub fn success_rate(&self) -> f64 {
        if self.total_completed + self.total_failed == 0 {
            return 100.0;
        }
        (self.total_completed as f64 / (self.total_completed + self.total_failed) as f64) * 100.0
    }
}

/// ─── ROUTER REPORT ───

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouterReport {
    pub total_agents: usize,
    pub active_agents: usize,
    pub pending_tasks: usize,
    pub active_tasks: usize,
    pub stats: RouterStats,
}

impl std::fmt::Display for RouterReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Router: {} agent ({} active) | {} pending | {} active | %{:.1} success",
            self.total_agents,
            self.active_agents,
            self.pending_tasks,
            self.active_tasks,
            self.stats.success_rate()
        )
    }
}

// ─── Tests ───

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_agent_registration() {
        let mut router = TaskRouter::new();
        let agent_id = SwarmAgentId::new();
        let persona = AgentPersona::new(AgentType::Researcher);
        
        router.register_agent(agent_id.clone(), persona);
        assert_eq!(router.agent_count(), 1);
    }
    
    #[test]
    fn test_task_submission() {
        let mut router = TaskRouter::new();
        let task = SwarmTask::new("Test gorevi");
        
        router.submit_task(task);
        assert_eq!(router.pending_count(), 1);
    }
    
    #[test]
    fn test_agent_load() {
        let load = AgentLoad::Idle;
        assert!(load.can_accept());
        
        let with_task = load.with_added_task();
        assert_eq!(with_task, AgentLoad::Light);
    }
    
    #[test]
    fn test_router_stats() {
        let mut stats = RouterStats::default();
        stats.total_completed = 80;
        stats.total_failed = 20;
        
        assert_eq!(stats.success_rate(), 80.0);
    }
}
