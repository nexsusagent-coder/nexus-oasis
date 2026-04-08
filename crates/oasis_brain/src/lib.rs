//! ═══════════════════════════════════════════════════════════════════════════════
//!  OASIS BRAIN - SENTIENT OS AUTONOMOUS THINKING MODULE
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! The cognitive engine of SENTIENT OS - powered by Gemma 4 as FIXED intelligence.
//! All autonomous thinking processes default to Gemma 4 for consistency and quality.
//!
//! ═══════════════════════════════════════════════════════════════════════════════
//!  ARCHITECTURE
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! ┌─────────────────────────────────────────────────────────────────────────────┐
//! │                          OASIS BRAIN                                        │
//! ├─────────────────────────────────────────────────────────────────────────────┤
//! │                                                                             │
//! │  ┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐       │
//! │  │   PERCEPTION    │────▶│    REASONING    │────▶│     ACTION      │       │
//! │  │    (Input)      │     │   (GEMMA 4)     │     │    (Output)     │       │
//! │  └─────────────────┘     └─────────────────┘     └─────────────────┘       │
//! │         │                       │                       │                  │
//! │         │                       │                       │                  │
//! │         ▼                       ▼                       ▼                  │
//! │  ┌─────────────────────────────────────────────────────────────────┐      │
//! │  │                    MEMORY CUBE (L3)                              │      │
//! │  │              Zero-Copy Data Flow Integration                     │      │
//! │  └─────────────────────────────────────────────────────────────────┘      │
//! │                                                                             │
//! └─────────────────────────────────────────────────────────────────────────────┘
//!
//! ═══════════════════════════════════════════════════════════════════════════════
//!  GEMMA 4 - FIXED KERNEL INTELLIGENCE
//! ═══════════════════════════════════════════════════════════════════════════════
//! • 31B parameters, multimodal
//! • 256K context length  
//! • Native thinking mode
//! • NO API KEY REQUIRED - FULLY LOCAL
//! ═══════════════════════════════════════════════════════════════════════════════

pub mod reasoning;
pub mod perception;
pub mod action;
pub mod memory_bridge;
pub mod cognitive_loop;

// Re-exports
pub use reasoning::{ReasoningEngine, ReasoningResult, ThinkingStep};
pub use perception::{PerceptionEngine, PerceptionInput, PerceptionOutput};
pub use action::{ActionEngine, Action, ActionResult};
pub use memory_bridge::{MemoryBridge, ZeroCopyConfig};
pub use cognitive_loop::{CognitiveLoop, CognitiveState, CognitiveConfig};

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, debug};

// ═══════════════════════════════════════════════════════════════════════════════
//  CONSTANTS - GEMMA 4 AS FIXED KERNEL
// ═══════════════════════════════════════════════════════════════════════════════

/// Fixed model for all autonomous thinking - GEMMA 4
pub const KERNEL_MODEL: &str = "gemma4:31b";

/// Kernel version
pub const KERNEL_VERSION: &str = "4.0.0";

/// Default context length (256K)
pub const KERNEL_CONTEXT_LENGTH: usize = 262_144;

// ═══════════════════════════════════════════════════════════════════════════════
//  OASIS BRAIN CONFIG
// ═══════════════════════════════════════════════════════════════════════════════

/// Oasis Brain Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrainConfig {
    /// Model - FIXED TO GEMMA 4
    pub model: String,
    /// Enable thinking mode
    pub thinking_mode: bool,
    /// Zero-copy memory integration
    pub zero_copy: bool,
    /// Max reasoning steps
    pub max_reasoning_steps: u32,
    /// Cognitive loop interval (ms)
    pub loop_interval_ms: u64,
    /// Memory persistence
    pub persist_memories: bool,
    /// Enable self-reflection
    pub self_reflection: bool,
}

impl Default for BrainConfig {
    fn default() -> Self {
        Self {
            model: KERNEL_MODEL.to_string(),
            thinking_mode: true,
            zero_copy: true,
            max_reasoning_steps: 10,
            loop_interval_ms: 100,
            persist_memories: true,
            self_reflection: true,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  OASIS BRAIN - MAIN ENGINE
// ═══════════════════════════════════════════════════════════════════════════════

/// OASIS Brain - Autonomous Thinking Engine
pub struct OasisBrain {
    config: BrainConfig,
    /// Gemma 4 local engine
    gemma4: sentient_local::LocalEngine,
    /// Memory bridge for zero-copy integration
    memory_bridge: MemoryBridge,
    /// Cognitive state
    state: Arc<RwLock<CognitiveState>>,
    /// Request counter
    request_count: Arc<RwLock<u64>>,
}

impl OasisBrain {
    /// Create new Oasis Brain with Gemma 4
    pub fn new(config: BrainConfig) -> Self {
        info!("🧠  OASIS BRAIN: Initializing with Gemma 4 kernel...");
        
        // Initialize Gemma 4 local engine
        let local_config = sentient_local::LocalConfig {
            provider: sentient_local::LocalProvider::Gemma4,
            model: config.model.clone(),
            thinking_mode: config.thinking_mode,
            zero_copy: config.zero_copy,
            ..Default::default()
        };
        
        let gemma4 = sentient_local::LocalEngine::with_config(local_config);
        
        // Initialize memory bridge
        let memory_bridge = MemoryBridge::new(ZeroCopyConfig {
            enabled: config.zero_copy,
            auto_persist: config.persist_memories,
        });
        
        Self {
            config,
            gemma4,
            memory_bridge,
            state: Arc::new(RwLock::new(CognitiveState::default())),
            request_count: Arc::new(RwLock::new(0)),
        }
    }
    
    /// Create with default config
    pub fn default_brain() -> Self {
        Self::new(BrainConfig::default())
    }
    
    /// Initialize the brain
    pub async fn init(&self) -> Result<(), BrainError> {
        info!("🚀  OASIS BRAIN: Starting Gemma 4 kernel...");
        
        // Initialize Gemma 4
        self.gemma4.init_gemma4().await
            .map_err(|e| BrainError::Initialization(e.to_string()))?;
        
        // Set cognitive state to ready
        {
            let mut state = self.state.write().await;
            state.is_active = true;
            state.kernel_model = KERNEL_MODEL.to_string();
            state.kernel_version = KERNEL_VERSION.to_string();
        }
        
        info!("✅  OASIS BRAIN: Gemma 4 kernel ready");
        Ok(())
    }
    
    /// Think about a problem
    pub async fn think(&self, input: &str) -> Result<ReasoningResult, BrainError> {
        debug!("🧠  OASIS BRAIN: Thinking about: {}...", &input.chars().take(50).collect::<String>());
        
        // Increment counter
        {
            let mut count = self.request_count.write().await;
            *count += 1;
        }
        
        // Create reasoning engine
        let reasoning = ReasoningEngine::new(self.config.clone());
        
        // Execute reasoning with Gemma 4
        let result = reasoning.reason(input, &self.gemma4).await?;
        
        // Persist to memory (zero-copy)
        if self.config.persist_memories {
            self.memory_bridge.store_reasoning(&result).await?;
        }
        
        // Update cognitive state
        {
            let mut state = self.state.write().await;
            state.total_thoughts += 1;
            state.last_thought = Some(chrono::Utc::now());
        }
        
        Ok(result)
    }
    
    /// Process perception input
    pub async fn perceive(&self, input: PerceptionInput) -> Result<PerceptionOutput, BrainError> {
        debug!("👁️  OASIS BRAIN: Perceiving input...");
        
        let perception = PerceptionEngine::new();
        let output = perception.process(input, &self.gemma4).await?;
        
        Ok(output)
    }
    
    /// Execute an action
    pub async fn act(&self, action: Action) -> Result<ActionResult, BrainError> {
        debug!("🎯  OASIS BRAIN: Executing action: {:?}", action.action_type);
        
        let engine = ActionEngine::new();
        let result = engine.execute(action, &self.gemma4).await?;
        
        Ok(result)
    }
    
    /// Run cognitive loop
    pub async fn run_loop(&self, goal: &str) -> Result<CognitiveState, BrainError> {
        info!("🔄  OASIS BRAIN: Starting cognitive loop for: {}", goal);
        
        let loop_engine = CognitiveLoop::new(self.config.clone());
        let final_state = loop_engine.run(goal, &self.gemma4, &self.memory_bridge).await?;
        
        Ok(final_state)
    }
    
    /// Get current cognitive state
    pub async fn get_state(&self) -> CognitiveState {
        let state = self.state.read().await;
        state.clone()
    }
    
    /// Get request count
    pub async fn request_count(&self) -> u64 {
        let count = self.request_count.read().await;
        *count
    }
    
    /// Health check
    pub async fn health_check(&self) -> BrainHealth {
        let state = self.state.read().await;
        let requests = self.request_count().await;
        
        BrainHealth {
            is_healthy: state.is_active,
            kernel_model: state.kernel_model.clone(),
            kernel_version: state.kernel_version.clone(),
            total_thoughts: state.total_thoughts,
            total_requests: requests,
            uptime_secs: state.started_at.map(|t| (chrono::Utc::now() - t).num_seconds() as u64).unwrap_or(0),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  BRAIN HEALTH
// ═══════════════════════════════════════════════════════════════════════════════

/// Brain health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrainHealth {
    pub is_healthy: bool,
    pub kernel_model: String,
    pub kernel_version: String,
    pub total_thoughts: u64,
    pub total_requests: u64,
    pub uptime_secs: u64,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  ERROR TYPE
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, thiserror::Error)]
pub enum BrainError {
    #[error("Initialization failed: {0}")]
    Initialization(String),
    
    #[error("Reasoning failed: {0}")]
    Reasoning(String),
    
    #[error("Perception failed: {0}")]
    Perception(String),
    
    #[error("Action failed: {0}")]
    Action(String),
    
    #[error("Memory bridge error: {0}")]
    MemoryBridge(String),
    
    #[error("Gemma 4 engine error: {0}")]
    Gemma4(String),
    
    #[error("Cognitive loop error: {0}")]
    CognitiveLoop(String),
}

// ═══════════════════════════════════════════════════════════════════════════════
//  TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_brain_config_default() {
        let config = BrainConfig::default();
        assert_eq!(config.model, KERNEL_MODEL);
        assert!(config.thinking_mode);
        assert!(config.zero_copy);
    }

    #[test]
    fn test_kernel_constants() {
        assert_eq!(KERNEL_MODEL, "gemma4:31b");
        assert_eq!(KERNEL_VERSION, "4.0.0");
        assert_eq!(KERNEL_CONTEXT_LENGTH, 262_144);
    }

    #[test]
    fn test_brain_creation() {
        let brain = OasisBrain::default_brain();
        assert_eq!(brain.config.model, KERNEL_MODEL);
    }

    #[test]
    fn test_brain_health() {
        let health = BrainHealth {
            is_healthy: true,
            kernel_model: KERNEL_MODEL.to_string(),
            kernel_version: KERNEL_VERSION.to_string(),
            total_thoughts: 100,
            total_requests: 500,
            uptime_secs: 3600,
        };
        
        assert!(health.is_healthy);
        assert_eq!(health.kernel_model, KERNEL_MODEL);
    }
}
