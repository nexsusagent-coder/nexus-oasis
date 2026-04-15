//! ─── SENTIENT Continuous Learning ───
//!
//! Learn from user interactions:
//! - Behavior analysis
//! - Preference learning
//! - Adaptive personality
//! - Pattern recognition

pub mod behavior;
pub mod preferences;
pub mod personality;
pub mod patterns;

use serde::{Deserialize, Serialize};
use thiserror::Error;

pub use behavior::BehaviorAnalyzer;
pub use preferences::PreferenceLearner;
pub use personality::AdaptivePersonality;
pub use patterns::PatternRecognizer;

/// Learning error
#[derive(Debug, Error)]
pub enum LearningError {
    #[error("Insufficient data: {0}")]
    InsufficientData(String),
    
    #[error("Model error: {0}")]
    ModelError(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

pub type LearningResult<T> = Result<T, LearningError>;

/// User interaction event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionEvent {
    pub id: String,
    pub user_id: String,
    pub event_type: EventType,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub metadata: serde_json::Value,
}

impl InteractionEvent {
    pub fn new(user_id: &str, event_type: EventType) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            user_id: user_id.to_string(),
            event_type,
            timestamp: chrono::Utc::now(),
            metadata: serde_json::json!({}),
        }
    }
    
    pub fn with_metadata(mut self, metadata: serde_json::Value) -> Self {
        self.metadata = metadata;
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
    Message,
    VoiceCommand,
    FileOpen,
    FileEdit,
    Search,
    TaskComplete,
    PreferenceChange,
    Feedback,
}

/// Learning model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningModel {
    pub user_id: String,
    pub behavior_profile: behavior::BehaviorProfile,
    pub preferences: preferences::UserPreferences,
    pub personality_traits: personality::PersonalityTraits,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_interaction_event() {
        let event = InteractionEvent::new("user1", EventType::Message);
        assert!(!event.id.is_empty());
    }
}
