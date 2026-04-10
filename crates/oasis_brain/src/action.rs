//! ─── ACTION ENGINE ───
//!
//! Action execution and results

use crate::BrainError;
use sentient_local::LocalEngine;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Action Engine
pub struct ActionEngine;

impl ActionEngine {
    pub fn new() -> Self {
        Self
    }
    
    /// Execute an action
    pub async fn execute(&self, action: Action, gemma4: &LocalEngine) -> Result<ActionResult, BrainError> {
        let start = std::time::Instant::now();
        
        // Generate action plan
        let _plan = self.plan_action(&action, gemma4).await?;
        
        // Execute based on action type
        let _plan = self.plan_action(&action, gemma4).await?;
        
        let result = match action.action_type {
            ActionType::Respond => {
                self.execute_respond(&action, gemma4).await?
            }
            ActionType::Store => {
                self.execute_store(&action, gemma4).await?
            }
            ActionType::Query => {
                self.execute_query(&action, gemma4).await?
            }
            ActionType::Analyze => {
                self.execute_analyze(&action, gemma4).await?
            }
            ActionType::Custom(ref custom_type) => {
                self.execute_custom(&action, custom_type, gemma4).await?
            }
        };
        
        Ok(ActionResult {
            action_id: action.id,
            success: true,
            output: result,
            duration_ms: start.elapsed().as_millis() as u64,
            timestamp: Utc::now(),
        })
    }
    
    async fn plan_action(&self, action: &Action, gemma4: &LocalEngine) -> Result<String, BrainError> {
        let prompt = format!(
            "Plan the execution steps for this action:\n\nType: {:?}\nInput: {}",
            action.action_type, action.input
        );
        
        gemma4.generate(&prompt).await
            .map_err(|e| BrainError::Action(e.to_string()))
    }
    
    async fn execute_respond(&self, action: &Action, gemma4: &LocalEngine) -> Result<String, BrainError> {
        gemma4.generate(&action.input).await
            .map_err(|e| BrainError::Action(e.to_string()))
    }
    
    async fn execute_store(&self, action: &Action, _gemma4: &LocalEngine) -> Result<String, BrainError> {
        // Store in memory (via memory bridge)
        Ok(format!("Stored: {}", action.input.chars().take(50).collect::<String>()))
    }
    
    async fn execute_query(&self, action: &Action, gemma4: &LocalEngine) -> Result<String, BrainError> {
        let prompt = format!("Answer this query comprehensively:\n\n{}", action.input);
        gemma4.generate(&prompt).await
            .map_err(|e| BrainError::Action(e.to_string()))
    }
    
    async fn execute_analyze(&self, action: &Action, gemma4: &LocalEngine) -> Result<String, BrainError> {
        let prompt = format!(
            "Perform a detailed analysis:\n\n{}\n\nProvide:\n1. Summary\n2. Key insights\n3. Recommendations",
            action.input
        );
        gemma4.generate(&prompt).await
            .map_err(|e| BrainError::Action(e.to_string()))
    }
    
    async fn execute_custom(&self, action: &Action, custom_type: &str, gemma4: &LocalEngine) -> Result<String, BrainError> {
        let prompt = format!(
            "Execute custom action type '{}':\n\n{}",
            custom_type, action.input
        );
        gemma4.generate(&prompt).await
            .map_err(|e| BrainError::Action(e.to_string()))
    }
}

// ─── Types ───

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    pub id: String,
    pub action_type: ActionType,
    pub input: String,
    pub parameters: serde_json::Value,
    pub priority: u8,
}

impl Action {
    pub fn new(action_type: ActionType, input: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            action_type,
            input: input.to_string(),
            parameters: serde_json::json!({}),
            priority: 0,
        }
    }
    
    pub fn respond(input: &str) -> Self {
        Self::new(ActionType::Respond, input)
    }
    
    pub fn store(input: &str) -> Self {
        Self::new(ActionType::Store, input)
    }
    
    pub fn query(input: &str) -> Self {
        Self::new(ActionType::Query, input)
    }
    
    pub fn analyze(input: &str) -> Self {
        Self::new(ActionType::Analyze, input)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionType {
    Respond,
    Store,
    Query,
    Analyze,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionResult {
    pub action_id: String,
    pub success: bool,
    pub output: String,
    pub duration_ms: u64,
    pub timestamp: DateTime<Utc>,
}

impl Default for ActionEngine {
    fn default() -> Self {
        Self::new()
    }
}
