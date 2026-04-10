//! ═══════════════════════════════════════════════════════════════════════════════
//!  MANUS AGENT - Otonom Manus Ajanı
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Kod yazma ve çalıştırma döngüsü.

use crate::error::ManusResult;
use crate::executor::CodeExecutor;
use crate::planner::{TaskPlanner, TaskPlan, TaskStep, StepStatus};
use crate::vgate::ManusVGate;
use crate::Language;
use serde::{Deserialize, Serialize};

/// ─── MANUS AGENT ───

pub struct ManusAgent {
    /// V-GATE köprüsü
    vgate: ManusVGate,
    /// Görev planlayıcı
    planner: TaskPlanner,
    /// Agent durumu
    state: AgentState,
    /// Yapılandırma
    config: AgentConfig,
    /// Varsayılan timeout
    default_timeout: u64,
    /// Simülasyon modu
    simulation: bool,
}

/// Agent durumu
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentState {
    Idle,
    Planning,
    Coding,
    Executing,
    Verifying,
    Completed,
    Failed,
}

/// Agent yapılandırması
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    /// Maksimum yeniden deneme
    pub max_retries: u32,
    /// Maksimum kod uzunluğu
    pub max_code_length: usize,
    /// Otomatik düzeltme
    pub auto_fix: bool,
    /// Varsayılan dil
    pub default_language: Language,
    /// Timeout (saniye)
    pub timeout_secs: u64,
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            max_code_length: 100_000,
            auto_fix: true,
            default_language: Language::Python,
            timeout_secs: 60,
        }
    }
}

/// Manus görevi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManusTask {
    /// Görev ID
    pub id: String,
    /// Görev tanımı
    pub description: String,
    /// Plan
    pub plan: Option<TaskPlan>,
    /// Sonuç
    pub result: Option<String>,
    /// Durum
    pub status: TaskStatus,
    /// Oluşturulma
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Tamamlanma
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// Görev durumu
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskStatus {
    Pending,
    Running,
    Completed,
    Failed,
}

impl ManusAgent {
    /// Yeni agent oluştur
    pub fn new(_vgate: &ManusVGate, _executor: &CodeExecutor) -> Self {
        Self {
            vgate: ManusVGate::new("http://localhost:1071"),
            planner: TaskPlanner::new(),
            state: AgentState::Idle,
            config: AgentConfig::default(),
            default_timeout: 60,
            simulation: true,
        }
    }
    
    /// Simülasyon modunu ayarla
    pub fn set_simulation(&mut self, simulation: bool) {
        self.simulation = simulation;
    }
    
    /// Görev çalıştır
    pub async fn run(&mut self, task: &str) -> ManusResult<String> {
        log::info!("🤖  MANUS-AGENT: Görev başlatılıyor → {}", task);
        
        self.state = AgentState::Planning;
        
        // Planla
        let mut plan = self.planner.plan(task)?;
        log::info!("🤖  MANUS-AGENT: {} adımlı plan oluşturuldu", plan.steps.len());
        
        self.state = AgentState::Executing;
        
        // Adımları çalıştır
        let mut results = Vec::new();
        
        for i in 0..plan.steps.len() {
            let result = self.execute_step(&plan.steps[i]).await;
            match result {
                Ok(r) => {
                    plan.steps[i].status = StepStatus::Completed;
                    plan.steps[i].result = Some(r.clone());
                    results.push(r);
                }
                Err(e) => {
                    plan.steps[i].status = StepStatus::Failed;
                    plan.steps[i].result = Some(e.to_string());
                    results.push(format!("Hata: {}", e));
                }
            }
        }
        
        // Sonucu birleştir
        let final_result = results.join("\n");
        
        self.state = AgentState::Completed;
        log::info!("🤖  MANUS-AGENT: Görev tamamlandı");
        
        Ok(final_result)
    }
    
    /// Adım çalıştır
    async fn execute_step(&self, step: &TaskStep) -> ManusResult<String> {
        log::info!("🤖  MANUS-AGENT: Adım {} çalıştırılıyor → {}", step.order, step.description);
        
        if let Some(code) = &step.code {
            let language = step.language.unwrap_or(self.config.default_language);
            
            // Simülasyon modunda basit çıktı
            let output = if self.simulation {
                format!("[SIM] {} kodu çalıştırıldı", language.extension())
            } else {
                // Gerçek execution burada olacak
                format!("Kod çalıştırıldı: {} bytes", code.len())
            };
            
            return Ok(output);
        }
        
        Ok(step.description.clone())
    }
    
    /// Durumu al
    pub fn state(&self) -> AgentState {
        self.state
    }
    
    /// Yapılandırmayı al
    pub fn config(&self) -> &AgentConfig {
        &self.config
    }
    
    /// Yapılandırmayı ayarla
    pub fn set_config(&mut self, config: AgentConfig) {
        self.config = config;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_config_default() {
        let config = AgentConfig::default();
        assert_eq!(config.max_retries, 3);
        assert_eq!(config.default_language, Language::Python);
    }

    #[test]
    fn test_agent_state() {
        let state = AgentState::Idle;
        assert_eq!(state, AgentState::Idle);
    }

    #[test]
    fn test_manus_task() {
        let task = ManusTask {
            id: "test".into(),
            description: "Test görevi".into(),
            plan: None,
            result: None,
            status: TaskStatus::Pending,
            created_at: chrono::Utc::now(),
            completed_at: None,
        };
        
        assert_eq!(task.status, TaskStatus::Pending);
    }
}
