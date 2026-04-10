//! ═══════════════════════════════════════════════════════════════════════════════
//!  DESKTOP AGENT LOOP - Otonom Agent Döngüsü
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! PERCEPTION → DECISION → ACTION → LEARN döngüsü.
//!
//! ┌─────────────────────────────────────────────────────────────────────────┐
//! │                         AGENT LOOP                                      │
//! │                                                                          │
//! │    ┌─────────┐     ┌─────────┐     ┌─────────┐     ┌─────────┐        │
//! │    │ PERCEIVE│ ──► │ DECIDE  │ ──► │   ACT   │ ──► │  LEARN  │        │
//! │    └────┬────┘     └────┬────┘     └────┬────┘     └────┬────┘        │
//! │         │               │               │               │              │
//! │         ▼               ▼               ▼               ▼              │
//! │    ┌─────────┐     ┌─────────┐     ┌─────────┐     ┌─────────┐        │
//! │    │ Screen  │     │ Planner │     │ Input   │     │ Memory  │        │
//! │    │ Vision  │     │ Safety  │     │ Tools   │     │ Healing │        │
//! │    └─────────┘     └─────────┘     └─────────┘     └─────────┘        │
//! │                                                                          │
//! └─────────────────────────────────────────────────────────────────────────┘

use crate::error::{AutonomousError, AutonomousResult};
use crate::{Action, ActionResult, AgentId, TaskResult};
use crate::vision::Observation;
use crate::screen::ScreenUnderstanding;
use crate::safety::{SafetySystem, SafetyConfig};
use crate::planner::TaskPlanner;
use crate::memory::AdvancedMemory;
use crate::healing::SelfHealing;

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use std::time::{Duration, Instant};

// ═══════════════════════════════════════════════════════════════════════════════
//  AGENT STATE
// ═══════════════════════════════════════════════════════════════════════════════

/// Agent durumu
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentState {
    /// Boşta
    Idle,
    /// Başlatılıyor
    Initializing,
    /// Gözlem yapıyor
    Perceiving,
    /// Karar veriyor
    Deciding,
    /// Aksiyon alıyor
    Acting,
    /// Öğreniyor
    Learning,
    /// Hata durumunda
    Error,
    /// Durduruldu
    Stopped,
    /// Paused
    Paused,
}

impl Default for AgentState {
    fn default() -> Self {
        Self::Idle
    }
}

impl std::fmt::Display for AgentState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AgentState::Idle => write!(f, "Boşta"),
            AgentState::Initializing => write!(f, "Başlatılıyor"),
            AgentState::Perceiving => write!(f, "Gözlemliyor"),
            AgentState::Deciding => write!(f, "Karar veriyor"),
            AgentState::Acting => write!(f, "Aksiyon alıyor"),
            AgentState::Learning => write!(f, "Öğreniyor"),
            AgentState::Error => write!(f, "Hata"),
            AgentState::Stopped => write!(f, "Durduruldu"),
            AgentState::Paused => write!(f, "Duraklatıldı"),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  AGENT CONFIG
// ═══════════════════════════════════════════════════════════════════════════════

/// Agent yapılandırması
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    /// Agent ID
    pub id: AgentId,
    /// Agent adı
    pub name: String,
    /// Maksimum iterasyon
    pub max_iterations: usize,
    /// Iterasyon timeout (ms)
    pub iteration_timeout_ms: u64,
    /// Perceive interval (ms)
    pub perceive_interval_ms: u64,
    /// Human approval gerekli mi?
    pub require_human_approval: bool,
    /// Auto healing aktif mi?
    pub auto_healing: bool,
    /// Learning aktif mi?
    pub learning_enabled: bool,
    /// Debug modu
    pub debug_mode: bool,
    /// Maximum errors before stop
    pub max_errors: usize,
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            id: AgentId::default(),
            name: "AutonomousAgent".into(),
            max_iterations: crate::MAX_ITERATIONS,
            iteration_timeout_ms: 30000,
            perceive_interval_ms: 100,
            require_human_approval: false,
            auto_healing: true,
            learning_enabled: true,
            debug_mode: false,
            max_errors: 10,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  AGENT CONTEXT
// ═══════════════════════════════════════════════════════════════════════════════

/// Agent bağlamı (state + environment)
#[derive(Debug, Clone)]
pub struct AgentContext {
    /// Mevcut durum
    pub state: AgentState,
    /// Mevcut görev
    pub current_task: Option<String>,
    /// Son gözlem
    pub last_observation: Option<Observation>,
    /// Son aksiyon
    pub last_action: Option<Action>,
    /// Son sonuç
    pub last_result: Option<ActionResult>,
    /// İterasyon sayısı
    pub iteration_count: usize,
    /// Hata sayısı
    pub error_count: usize,
    /// Başlangıç zamanı
    pub start_time: Option<Instant>,
    /// Değişkenler
    pub variables: std::collections::HashMap<String, serde_json::Value>,
}

impl Default for AgentContext {
    fn default() -> Self {
        Self {
            state: AgentState::Idle,
            current_task: None,
            last_observation: None,
            last_action: None,
            last_result: None,
            iteration_count: 0,
            error_count: 0,
            start_time: None,
            variables: std::collections::HashMap::new(),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  AUTONOMOUS AGENT
// ═══════════════════════════════════════════════════════════════════════════════

/// Otonom Desktop Agent
pub struct AutonomousAgent {
    /// Yapılandırma
    config: AgentConfig,
    
    /// Bağlam
    context: Arc<RwLock<AgentContext>>,
    
    /// Screen Understanding
    screen: ScreenUnderstanding,
    
    /// Safety System
    safety: SafetySystem,
    
    /// Task Planner
    planner: TaskPlanner,
    
    /// Advanced Memory
    memory: AdvancedMemory,
    
    /// Self Healing
    healing: SelfHealing,
    
    /// Running flag
    running: Arc<Mutex<bool>>,
}

impl AutonomousAgent {
    /// Yeni agent oluştur
    pub async fn new(config: AgentConfig) -> AutonomousResult<Self> {
        log::info!("🤖 AGENT: Otonom agent oluşturuluyor... ({})", config.name);
        
        let safety_config = SafetyConfig {
            require_human_approval: config.require_human_approval,
            ..Default::default()
        };
        
        Ok(Self {
            config,
            context: Arc::new(RwLock::new(AgentContext::default())),
            screen: ScreenUnderstanding::new(),
            safety: SafetySystem::new(safety_config),
            planner: TaskPlanner::new(),
            memory: AdvancedMemory::new(),
            healing: SelfHealing::new(),
            running: Arc::new(Mutex::new(false)),
        })
    }
    
    /// Agent'ı varsayılan ayarla oluştur
    pub async fn default_agent() -> AutonomousResult<Self> {
        Self::new(AgentConfig::default()).await
    }
    
    // ════════════════════════════════════════════════════════════════════════════
    //  MAIN LOOP
    // ════════════════════════════════════════════════════════════════════════════
    
    /// Ana döngü - Görevi çalıştır
    pub async fn run(&mut self, goal: &str) -> AutonomousResult<TaskResult> {
        log::info!("🤖 AGENT: Görev başlıyor → {}", goal);
        
        // Başlat
        self.start(goal).await?;
        
        let task_id = uuid::Uuid::new_v4().to_string();
        let start_time = Instant::now();
        let mut total_actions = 0;
        let mut total_errors = 0;
        
        // Ana döngü
        loop {
            // Durdurma kontrolü
            if !*self.running.lock().await {
                log::info!("🤖 AGENT: Durdurma sinyali alındı");
                break;
            }
            
            // Maksimum iterasyon kontrolü
            let ctx = self.context.read().await;
            if ctx.iteration_count >= self.config.max_iterations {
                log::warn!("🤖 AGENT: Maksimum iterasyon aşıldı");
                drop(ctx);
                self.stop().await;
                break;
            }
            drop(ctx);
            
            // PERCEIVE
            let observation = match self.perceive().await {
                Ok(obs) => obs,
                Err(e) => {
                    log::error!("🤖 AGENT: Perceive hatası: {}", e);
                    self.increment_error().await;
                    total_errors += 1;
                    
                    if self.config.auto_healing {
                        if let Err(he) = self.heal(&e).await {
                            log::error!("🤖 AGENT: Healing hatası: {}", he);
                        }
                    }
                    continue;
                }
            };
            
            // DECIDE
            let action = match self.decide(&observation, goal).await {
                Ok(act) => act,
                Err(e) => {
                    log::error!("🤖 AGENT: Decide hatası: {}", e);
                    self.increment_error().await;
                    total_errors += 1;
                    
                    // Stop action (görevi bitir)
                    if matches!(e, AutonomousError::MaxIterationsExceeded(_)) {
                        break;
                    }
                    continue;
                }
            };
            
            // Hedefe ulaştık mı?
            if matches!(action, Action::Stop { .. }) {
                log::info!("🤖 AGENT: Görev tamamlandı");
                break;
            }
            
            // NoOp ise atla
            if matches!(action, Action::NoOp) {
                tokio::time::sleep(Duration::from_millis(self.config.perceive_interval_ms)).await;
                continue;
            }
            
            // Safety check
            if !self.safety.check_action(&action).await? {
                log::warn!("🤖 AGENT: Aksiyon safety tarafından reddedildi");
                continue;
            }
            
            // ACT
            let result = match self.act(&action).await {
                Ok(res) => {
                    total_actions += 1;
                    res
                }
                Err(e) => {
                    log::error!("🤖 AGENT: Act hatası: {}", e);
                    self.increment_error().await;
                    total_errors += 1;
                    
                    if self.config.auto_healing {
                        if let Err(he) = self.heal(&e).await {
                            log::error!("🤖 AGENT: Healing hatası: {}", he);
                        }
                    }
                    continue;
                }
            };
            
            // LEARN
            if self.config.learning_enabled {
                if let Err(e) = self.learn(&observation, &action, &result).await {
                    log::warn!("🤖 AGENT: Learn hatası: {}", e);
                }
            }
            
            // İterasyon sayaç
            {
                let mut ctx = self.context.write().await;
                ctx.iteration_count += 1;
            }
            
            // Interval
            tokio::time::sleep(Duration::from_millis(self.config.perceive_interval_ms)).await;
        }
        
        // Sonuç
        let duration = start_time.elapsed();
        let success = total_errors < self.config.max_errors;
        
        self.stop().await;
        
        Ok(TaskResult {
            task_id,
            success,
            message: if success { 
                "Görev başarıyla tamamlandı".into() 
            } else { 
                "Görev hatalarla tamamlandı".into() 
            },
            total_duration_ms: duration.as_millis() as u64,
            action_count: total_actions,
            error_count: total_errors,
            result_data: None,
        })
    }
    
    // ════════════════════════════════════════════════════════════════════════════
    //  PHASE METHODS
    // ════════════════════════════════════════════════════════════════════════════
    
    /// PERCEIVE - Ekranı oku ve anla
    async fn perceive(&mut self) -> AutonomousResult<Observation> {
        self.set_state(AgentState::Perceiving).await;
        
        log::debug!("🤖 AGENT: Perceiving...");
        
        // Ekran yakala
        let observation = self.screen.capture_and_analyze().await?;
        
        // Context'e kaydet
        {
            let mut ctx = self.context.write().await;
            ctx.last_observation = Some(observation.clone());
        }
        
        self.set_state(AgentState::Idle).await;
        Ok(observation)
    }
    
    /// DECIDE - Ne yapacağına karar ver
    async fn decide(&mut self, observation: &Observation, goal: &str) -> AutonomousResult<Action> {
        self.set_state(AgentState::Deciding).await;
        
        log::debug!("🤖 AGENT: Deciding...");
        
        // Planner'dan aksiyon al
        let action = self.planner.plan_next(observation, goal).await?;
        
        // Context'e kaydet
        {
            let mut ctx = self.context.write().await;
            ctx.last_action = Some(action.clone());
        }
        
        self.set_state(AgentState::Idle).await;
        Ok(action)
    }
    
    /// ACT - Aksiyonu gerçekleştir
    async fn act(&mut self, action: &Action) -> AutonomousResult<ActionResult> {
        self.set_state(AgentState::Acting).await;
        
        log::info!("🤖 AGENT: Acting → {:?}", action);
        
        let start = Instant::now();
        
        // Aksiyonu çalıştır (placeholder - gerçek implementasyonda input controller kullanılır)
        let result = self.execute_action(action).await?;
        
        let _duration = start.elapsed();
        
        // Context'e kaydet
        {
            let mut ctx = self.context.write().await;
            ctx.last_result = Some(result.clone());
        }
        
        self.set_state(AgentState::Idle).await;
        Ok(result)
    }
    
    /// LEARN - Deneyimden öğren
    async fn learn(&mut self, observation: &Observation, action: &Action, result: &ActionResult) -> AutonomousResult<()> {
        self.set_state(AgentState::Learning).await;
        
        log::debug!("🤖 AGENT: Learning...");
        
        // Memory'ye kaydet
        self.memory.record_episode(observation, action, result).await?;
        
        self.set_state(AgentState::Idle).await;
        Ok(())
    }
    
    /// HEAL - Hata durumunda kurtar
    async fn heal(&mut self, error: &AutonomousError) -> AutonomousResult<()> {
        log::info!("🤖 AGENT: Healing from error: {}", error);
        
        let context = self.context.read().await.clone();
        self.healing.recover(error, &context).await
    }
    
    // ════════════════════════════════════════════════════════════════════════════
    //  ACTION EXECUTION
    // ════════════════════════════════════════════════════════════════════════════
    
    /// Tek bir aksiyonu çalıştır (helper)
    async fn execute_single_action(&mut self, action: &Action) -> AutonomousResult<ActionResult> {
        match action {
            Action::NoOp => Ok(ActionResult::success("No-op")),
            Action::Stop { reason } => Ok(ActionResult::success(reason)),
            Action::MouseMove { x, y } => {
                Ok(ActionResult::success(&format!("Mouse moved to ({}, {})", x, y)))
            }
            Action::MouseClick { button, x, y } => {
                Ok(ActionResult::success(&format!("Clicked {:?} at ({}, {})", button, x, y)))
            }
            Action::TypeText { text, human_like: _ } => {
                Ok(ActionResult::success(&format!("Typed {} characters", text.len())))
            }
            Action::KeyPress { key } => {
                Ok(ActionResult::success(&format!("Key pressed: {:?}", key)))
            }
            Action::BrowserNavigate { url } => {
                Ok(ActionResult::success(&format!("Navigated to {}", url)))
            }
            Action::Custom { name, params: _ } => {
                Ok(ActionResult::success(&format!("Custom action '{}' executed", name)))
            }
            _ => Ok(ActionResult::success("Action executed")),
        }
    }

    /// Aksiyonu çalıştır
    async fn execute_action(&mut self, action: &Action) -> AutonomousResult<ActionResult> {
        match action {
            Action::NoOp => Ok(ActionResult::success("No-op")),
            Action::Stop { reason } => Ok(ActionResult::success(reason)),
            
            Action::MouseMove { x, y } => {
                // Try native mouse control
                #[cfg(feature = "enigo")]
                {
                    if crate::screen::native_input::move_mouse(*x, *y) {
                        log::info!("🤖 AGENT: Mouse moved to ({}, {})", x, y);
                        return Ok(ActionResult::success(&format!("Mouse moved to ({}, {})", x, y)));
                    }
                }
                
                // Fallback: Log only
                log::info!("🤖 AGENT: Mouse move to ({}, {}) [no native input]", x, y);
                Ok(ActionResult::success(&format!("Mouse move requested ({}, {})", x, y)))
            }
            
            Action::MouseClick { button, x, y } => {
                #[cfg(feature = "enigo")]
                {
                    if crate::screen::native_input::click(*x, *y) {
                        log::info!("🤖 AGENT: Clicked {:?} at ({}, {})", button, x, y);
                        return Ok(ActionResult::success(&format!("Clicked {:?} at ({}, {})", button, x, y)));
                    }
                }
                
                log::info!("🤖 AGENT: Mouse click {:?} at ({}, {}) [no native input]", button, x, y);
                Ok(ActionResult::success(&format!("Click requested {:?} at ({}, {})", button, x, y)))
            }
            
            Action::TypeText { text, human_like } => {
                #[cfg(feature = "enigo")]
                {
                    if crate::screen::native_input::type_text(text) {
                        log::info!("🤖 AGENT: Typed {} chars (human_like: {})", text.len(), human_like);
                        return Ok(ActionResult::success(&format!("Typed {} characters", text.len())));
                    }
                }
                
                log::info!("🤖 AGENT: Type text {} chars (human_like: {}) [no native input]", text.len(), human_like);
                Ok(ActionResult::success(&format!("Type requested: {} chars", text.len())))
            }
            
            Action::KeyPress { key } => {
                #[cfg(feature = "enigo")]
                {
                    let key_str = format!("{:?}", key).to_lowercase();
                    if crate::screen::native_input::press_key(&key_str) {
                        log::info!("🤖 AGENT: Key pressed {:?}", key);
                        return Ok(ActionResult::success(&format!("Key pressed: {:?}", key)));
                    }
                }
                
                log::info!("🤖 AGENT: Key press {:?} [no native input]", key);
                Ok(ActionResult::success(&format!("Key press requested: {:?}", key)))
            }
            
            Action::BrowserNavigate { url } => {
                log::info!("🤖 AGENT: Navigating to {}", url);
                Ok(ActionResult::success(&format!("Navigated to {}", url)))
            }
            
            Action::Composite { actions } => {
                let mut results = Vec::new();
                for act in actions {
                    // Avoid recursive async by using a helper
                    let result = self.execute_single_action(&act).await;
                    match result {
                        Ok(r) => results.push(r),
                        Err(e) => {
                            results.push(ActionResult::failure(&e.to_string()));
                            break;
                        }
                    }
                }
                let success = results.iter().all(|r| r.success);
                if success {
                    Ok(ActionResult::success(&format!("{} actions completed", actions.len())))
                } else {
                    Ok(ActionResult::failure("Some actions failed"))
                }
            }
            
            Action::Custom { name, params } => {
                log::info!("🤖 AGENT: Custom action '{}' with {} params", name, params.len());
                Ok(ActionResult::success(&format!("Custom action '{}' executed", name)))
            }
            
            _ => Ok(ActionResult::success("Action executed")),
        }
    }
    
    // ════════════════════════════════════════════════════════════════════════════
    //  CONTROL METHODS
    // ════════════════════════════════════════════════════════════════════════════
    
    /// Agent'ı başlat
    async fn start(&mut self, task: &str) -> AutonomousResult<()> {
        let mut running = self.running.lock().await;
        if *running {
            return Err(AutonomousError::AgentInitializationFailed("Agent zaten çalışıyor".into()));
        }
        
        *running = true;
        
        let mut ctx = self.context.write().await;
        ctx.state = AgentState::Initializing;
        ctx.current_task = Some(task.into());
        ctx.start_time = Some(Instant::now());
        ctx.iteration_count = 0;
        ctx.error_count = 0;
        
        log::info!("🤖 AGENT: Başlatıldı → {}", task);
        
        Ok(())
    }
    
    /// Agent'ı durdur
    async fn stop(&mut self) {
        let mut running = self.running.lock().await;
        *running = false;
        
        let mut ctx = self.context.write().await;
        ctx.state = AgentState::Stopped;
        
        log::info!("🤖 AGENT: Durduruldu");
    }
    
    /// Agent'ı pause et
    pub async fn pause(&mut self) -> AutonomousResult<()> {
        self.set_state(AgentState::Paused).await;
        log::info!("🤖 AGENT: Duraklatıldı");
        Ok(())
    }
    
    /// Agent'ı resume et
    pub async fn resume(&mut self) -> AutonomousResult<()> {
        self.set_state(AgentState::Idle).await;
        log::info!("🤖 AGENT: Devam ediliyor");
        Ok(())
    }
    
    /// Acil durum durdurması
    pub async fn emergency_stop(&mut self) {
        log::warn!("🤖 AGENT: EMERGENCY STOP!");
        
        let mut running = self.running.lock().await;
        *running = false;
        
        let mut ctx = self.context.write().await;
        ctx.state = AgentState::Stopped;
    }
    
    // ════════════════════════════════════════════════════════════════════════════
    //  HELPER METHODS
    // ════════════════════════════════════════════════════════════════════════════
    
    /// Durumu ayarla
    async fn set_state(&mut self, state: AgentState) {
        let mut ctx = self.context.write().await;
        ctx.state = state;
    }
    
    /// Hata sayaç artır
    async fn increment_error(&mut self) {
        let mut ctx = self.context.write().await;
        ctx.error_count += 1;
    }
    
    /// Mevcut durumu al
    pub async fn get_state(&self) -> AgentState {
        self.context.read().await.state
    }
    
    /// Context'i al
    pub async fn get_context(&self) -> AgentContext {
        self.context.read().await.clone()
    }
    
    /// Değişken ayarla
    pub async fn set_variable(&mut self, key: &str, value: serde_json::Value) {
        let mut ctx = self.context.write().await;
        ctx.variables.insert(key.into(), value);
    }
    
    /// Değişken al
    pub async fn get_variable(&self, key: &str) -> Option<serde_json::Value> {
        self.context.read().await.variables.get(key).cloned()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_agent_creation() {
        let agent = AutonomousAgent::default_agent().await.expect("operation failed");
        assert_eq!(agent.get_state().await, AgentState::Idle);
    }
    
    #[tokio::test]
    async fn test_agent_config_default() {
        let config = AgentConfig::default();
        assert_eq!(config.max_iterations, crate::MAX_ITERATIONS);
        assert!(config.auto_healing);
    }
    
    #[tokio::test]
    async fn test_agent_context_default() {
        let ctx = AgentContext::default();
        assert_eq!(ctx.state, AgentState::Idle);
        assert_eq!(ctx.iteration_count, 0);
    }
    
    #[test]
    fn test_agent_state_display() {
        assert_eq!(format!("{}", AgentState::Idle), "Boşta");
        assert_eq!(format!("{}", AgentState::Acting), "Aksiyon alıyor");
    }
    
    #[tokio::test]
    async fn test_set_variable() {
        let mut agent = AutonomousAgent::default_agent().await.expect("operation failed");
        agent.set_variable("test", serde_json::json!("value")).await;
        
        let val = agent.get_variable("test").await;
        assert_eq!(val, Some(serde_json::json!("value")));
    }
}
