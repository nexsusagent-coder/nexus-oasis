//! ─── PERCEPTION ENGINE ───
//!
//! Input perception and processing

use crate::BrainError;
use sentient_local::LocalEngine;
use serde::{Deserialize, Serialize};
use log::debug;

/// Perception Engine
pub struct PerceptionEngine;

impl PerceptionEngine {
    pub fn new() -> Self {
        Self
    }
    
    /// Process perception input
    pub async fn process(&self, input: PerceptionInput, gemma4: &LocalEngine) -> Result<PerceptionOutput, BrainError> {
        // Classify input type
        let input_type = self.classify_input(&input.content);
        
        // Extract entities
        let entities = self.extract_entities(&input.content, gemma4).await?;
        
        // Determine intent
        let intent = self.determine_intent(&input.content, gemma4).await?;
        
        // Calculate importance
        let importance = self.calculate_importance(&input, &intent);
        
        Ok(PerceptionOutput {
            original: input,
            input_type,
            entities,
            intent,
            importance,
        })
    }
    
    fn classify_input(&self, content: &str) -> InputType {
        if content.contains("?") {
            InputType::Question
        } else if content.starts_with("do ") || content.starts_with("execute ") {
            InputType::Command
        } else if content.contains("remember") || content.contains("save") {
            InputType::MemoryStore
        } else {
            InputType::Statement
        }
    }
    
    async fn extract_entities(&self, content: &str, gemma4: &LocalEngine) -> Result<Vec<Entity>, BrainError> {
        let prompt = format!(
            "Extract named entities from this text. Return as JSON array:\n\n{}",
            content
        );
        
        let _response = gemma4.generate(&prompt).await
            .map_err(|e| BrainError::Perception(e.to_string()))?;
        
        // Parse entities (simplified)
        Ok(vec![Entity {
            name: "extracted".to_string(),
            entity_type: EntityType::Unknown,
            confidence: 0.8,
        }])
    }
    
    async fn determine_intent(&self, content: &str, gemma4: &LocalEngine) -> Result<Intent, BrainError> {
        let prompt = format!(
            "What is the user's intent? Choose: Query, Command, Conversation, Memory, Analysis.\n\nText: {}",
            content
        );
        
        let response = gemma4.generate(&prompt).await
            .map_err(|e| BrainError::Perception(e.to_string()))?;
        
        let intent_type = if response.to_lowercase().contains("query") {
            IntentType::Query
        } else if response.to_lowercase().contains("command") {
            IntentType::Command
        } else if response.to_lowercase().contains("memory") {
            IntentType::Memory
        } else if response.to_lowercase().contains("analysis") {
            IntentType::Analysis
        } else {
            IntentType::Conversation
        };
        
        Ok(Intent {
            intent_type,
            confidence: 0.9,
        })
    }
    
    fn calculate_importance(&self, input: &PerceptionInput, intent: &Intent) -> f32 {
        let mut score: f32 = 0.5;
        
        match intent.intent_type {
            IntentType::Command => score += 0.3,
            IntentType::Query => score += 0.1,
            IntentType::Memory => score += 0.2,
            _ => {}
        }
        
        if input.priority > 0 {
            score += 0.1;
        }
        
        score.min(1.0)
    }
}

// ─── Types ───

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerceptionInput {
    pub content: String,
    pub source: String,
    pub priority: u8,
    pub metadata: serde_json::Value,
}

impl PerceptionInput {
    pub fn new(content: &str) -> Self {
        Self {
            content: content.to_string(),
            source: "user".to_string(),
            priority: 0,
            metadata: serde_json::json!({}),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerceptionOutput {
    pub original: PerceptionInput,
    pub input_type: InputType,
    pub entities: Vec<Entity>,
    pub intent: Intent,
    pub importance: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InputType {
    Question,
    Command,
    Statement,
    MemoryStore,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    pub name: String,
    pub entity_type: EntityType,
    pub confidence: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EntityType {
    Person,
    Location,
    Organization,
    Date,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Intent {
    pub intent_type: IntentType,
    pub confidence: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IntentType {
    Query,
    Command,
    Conversation,
    Memory,
    Analysis,
}

impl Default for PerceptionEngine {
    fn default() -> Self {
        Self::new()
    }
}
