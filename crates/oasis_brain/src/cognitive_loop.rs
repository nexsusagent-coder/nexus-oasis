//! ─── COGNITIVE LOOP ───
//!
//! Autonomous thinking cycle

use crate::{BrainConfig, BrainError, memory_bridge::MemoryBridge, KERNEL_MODEL};
use sentient_local::LocalEngine;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use log::info;

/// Cognitive Loop Engine
pub struct CognitiveLoop {
    config: BrainConfig,
}

impl CognitiveLoop {
    pub fn new(config: BrainConfig) -> Self {
        Self { config }
    }
    
    /// Run cognitive loop until goal is achieved
    pub async fn run(
        &self,
        goal: &str,
        gemma4: &LocalEngine,
        memory: &MemoryBridge,
    ) -> Result<CognitiveState, BrainError> {
        let mut state = CognitiveState::new(goal);
        
        info!("🔄  COGNITIVE LOOP: Starting for goal: {}", goal);
        
        loop {
            // 1. Observe
            let observation = self.observe(&state, gemma4).await?;
            state.observations.push(observation.clone());
            
            // 2. Think
            let thought = self.think(&state, &observation, gemma4).await?;
            state.thoughts.push(thought.clone());
            
            // 3. Decide
            let decision = self.decide(&state, &thought, gemma4).await?;
            state.decisions.push(decision.clone());
            
            // 4. Act
            if decision.should_act {
                let action_result = self.act(&decision, gemma4).await?;
                state.actions.push(action_result.clone());
                
                // Store in memory (zero-copy)
                if self.config.persist_memories {
                    memory.store(action_result.output.clone(), crate::memory_bridge::MemoryType::Action).await?;
                }
            }
            
            // 5. Evaluate
            state.iteration += 1;
            state.progress = self.evaluate_progress(&state, gemma4).await?;
            
            // Check termination conditions
            if state.progress >= 0.95 || state.iteration >= self.config.max_reasoning_steps {
                state.is_complete = true;
                state.final_result = decision.conclusion.clone();
                break;
            }
            
            // Self-reflection
            if self.config.self_reflection && state.iteration % 3 == 0 {
                let reflection = self.reflect(&state, gemma4).await?;
                state.reflections.push(reflection);
            }
        }
        
        state.completed_at = Some(Utc::now());
        state.is_active = false;
        
        info!("✅  COGNITIVE LOOP: Completed in {} iterations", state.iteration);
        
        Ok(state)
    }
    
    async fn observe(&self, state: &CognitiveState, gemma4: &LocalEngine) -> Result<String, BrainError> {
        let prompt = format!(
            "Goal: {}\nCurrent state: {} iterations completed.\nWhat do you observe about the current situation?",
            state.goal, state.iteration
        );
        gemma4.generate(&prompt).await.map_err(|e| BrainError::CognitiveLoop(e.to_string()))
    }
    
    async fn think(&self, state: &CognitiveState, observation: &str, gemma4: &LocalEngine) -> Result<String, BrainError> {
        let prompt = format!(
            "Goal: {}\nObservation: {}\n\nWhat are your thoughts on how to proceed?",
            state.goal, observation
        );
        gemma4.generate(&prompt).await.map_err(|e| BrainError::CognitiveLoop(e.to_string()))
    }
    
    async fn decide(&self, state: &CognitiveState, thought: &str, gemma4: &LocalEngine) -> Result<Decision, BrainError> {
        let prompt = format!(
            "Goal: {}\nThought: {}\n\nDecide: What action should be taken? If goal is achieved, respond with 'GOAL_ACHIEVED: <conclusion>'. Otherwise, describe the next action.",
            state.goal, thought
        );
        
        let response = gemma4.generate(&prompt).await.map_err(|e| BrainError::CognitiveLoop(e.to_string()))?;
        
        let is_goal_achieved = response.contains("GOAL_ACHIEVED") || response.to_lowercase().contains("goal achieved");
        
        Ok(Decision {
            thought: thought.to_string(),
            action: response.clone(),
            should_act: !is_goal_achieved,
            conclusion: if is_goal_achieved { Some(response) } else { None },
        })
    }
    
    async fn act(&self, decision: &Decision, gemma4: &LocalEngine) -> Result<ActionResult, BrainError> {
        let response = gemma4.generate(&decision.action).await
            .map_err(|e| BrainError::CognitiveLoop(e.to_string()))?;
        
        Ok(ActionResult {
            action: decision.action.clone(),
            output: response,
            timestamp: Utc::now(),
        })
    }
    
    async fn evaluate_progress(&self, state: &CognitiveState, gemma4: &LocalEngine) -> Result<f32, BrainError> {
        let prompt = format!(
            "Goal: {}\nCompleted actions: {}\n\nRate progress toward goal (0.0 to 1.0). Return only the number.",
            state.goal,
            state.actions.len()
        );
        
        let response = gemma4.generate(&prompt).await.map_err(|e| BrainError::CognitiveLoop(e.to_string()))?;
        
        // Parse progress value
        response.chars()
            .filter(|c| c.is_digit(10) || *c == '.' || *c == '0')
            .collect::<String>()
            .parse::<f32>()
            .unwrap_or(0.5)
            .min(1.0)
            .max(0.0)
            .pipe(Ok)
    }
    
    async fn reflect(&self, state: &CognitiveState, gemma4: &LocalEngine) -> Result<String, BrainError> {
        let prompt = format!(
            "Goal: {}\nIteration: {}\n\nReflect on the progress. What is working? What could be improved?",
            state.goal, state.iteration
        );
        gemma4.generate(&prompt).await.map_err(|e| BrainError::CognitiveLoop(e.to_string()))
    }
}

// Helper trait for Option
trait Pipe<T> {
    fn pipe<U, F: FnOnce(T) -> U>(self, f: F) -> U;
}

impl<T> Pipe<T> for T {
    fn pipe<U, F: FnOnce(T) -> U>(self, f: F) -> U {
        f(self)
    }
}

// ─── Types ───

/// Cognitive state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitiveState {
    pub goal: String,
    pub is_active: bool,
    pub is_complete: bool,
    pub iteration: u32,
    pub progress: f32,
    pub observations: Vec<String>,
    pub thoughts: Vec<String>,
    pub decisions: Vec<Decision>,
    pub actions: Vec<ActionResult>,
    pub reflections: Vec<String>,
    pub final_result: Option<String>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub total_thoughts: u64,
    pub last_thought: Option<DateTime<Utc>>,
    pub kernel_model: String,
    pub kernel_version: String,
}

impl CognitiveState {
    pub fn new(goal: &str) -> Self {
        Self {
            goal: goal.to_string(),
            is_active: true,
            is_complete: false,
            iteration: 0,
            progress: 0.0,
            observations: Vec::new(),
            thoughts: Vec::new(),
            decisions: Vec::new(),
            actions: Vec::new(),
            reflections: Vec::new(),
            final_result: None,
            started_at: Some(Utc::now()),
            completed_at: None,
            total_thoughts: 0,
            last_thought: None,
            kernel_model: KERNEL_MODEL.to_string(),
            kernel_version: crate::KERNEL_VERSION.to_string(),
        }
    }
}

impl Default for CognitiveState {
    fn default() -> Self {
        Self::new("No goal specified")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Decision {
    pub thought: String,
    pub action: String,
    pub should_act: bool,
    pub conclusion: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionResult {
    pub action: String,
    pub output: String,
    pub timestamp: DateTime<Utc>,
}

/// Cognitive config alias
pub use crate::BrainConfig as CognitiveConfig;
