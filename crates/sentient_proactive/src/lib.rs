//! ─── SENTIENT PROACTIVE ENGINE ───
//!
//! The proactive AI engine that acts without explicit user requests
//!
//! # Features
//! - **Time-based triggers**: "Saat 09:00 → Güne hazırlan"
//! - **Event-based triggers**: "Email geldi → Acil mi?"
//! - **Pattern-based triggers**: "Her Cuma → Haftalık rapor"
//! - **Cron scheduling**: Flexible scheduling system
//!
//! # Example
//! ```rust,ignore
//! use sentient_proactive::{ProactiveEngine, Trigger, TriggerType};
//!
//! #[tokio::main]
//! async fn main() {
//!     let engine = ProactiveEngine::new();
//!     
//!     // Add a time-based trigger
//!     engine.add_trigger(Trigger {
//!         id: "morning-brief".into(),
//!         name: "Morning Briefing".into(),
//!         trigger_type: TriggerType::TimeBased {
//!             time: "09:00".into(),
//!             days: vec![1, 2, 3, 4, 5], // Weekdays
//!         },
//!         action: Action::GenerateBriefing,
//!         enabled: true,
//!     }).await;
//!     
//!     engine.start().await;
//! }
//! ```

pub mod trigger;
pub mod scheduler;
pub mod event;
pub mod pattern;
pub mod action;
pub mod engine;
pub mod cron;

pub use trigger::{Trigger, TriggerType, TriggerConfig};
pub use scheduler::{Scheduler, ScheduledTask, ScheduleStatus};
pub use event::{EventBus, EventType, Event, EventListener};
pub use pattern::{PatternMatcher, Pattern, PatternResult};
pub use action::{Action, ActionExecutor, ActionResult};
pub use engine::{ProactiveEngine, EngineConfig, EngineStats};
pub use cron::{CronParser, CronSchedule};

pub mod prelude {
    pub use crate::{ProactiveEngine, Trigger, TriggerType, Action, EventBus};
}

// Re-exports for convenience
pub type TriggerId = String;
pub type EventId = uuid::Uuid;
pub type Timestamp = chrono::DateTime<chrono::Utc>;

/// Result type for proactive operations
pub type ProactiveResult<T> = Result<T, ProactiveError>;

/// Error type
#[derive(Debug, thiserror::Error)]
pub enum ProactiveError {
    #[error("Trigger not found: {0}")]
    TriggerNotFound(String),
    
    #[error("Invalid cron expression: {0}")]
    InvalidCron(String),
    
    #[error("Scheduler error: {0}")]
    SchedulerError(String),
    
    #[error("Action execution failed: {0}")]
    ActionFailed(String),
    
    #[error("Event bus error: {0}")]
    EventBusError(String),
    
    #[error("Pattern matching error: {0}")]
    PatternError(String),
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[error(transparent)]
    Io(#[from] std::io::Error),
    
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_trigger_creation() {
        let trigger = Trigger::new(
            "test-trigger",
            "Test Trigger",
            TriggerType::TimeBased {
                time: "09:00".into(),
                days: vec![1, 2, 3, 4, 5],
            },
        );
        
        assert_eq!(trigger.id, "test-trigger");
        assert!(trigger.enabled);
    }
}
