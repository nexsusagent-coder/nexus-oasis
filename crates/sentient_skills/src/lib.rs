//! ─── SENTIENT SKILLS SYSTEM ───
//!
//! Automatic skill generation from user patterns
//!
//! # Features
//! - **Screen Watcher**: Observe user actions
//! - **Pattern Extraction**: Detect repetitive tasks
//! - **Skill Generation**: Auto-create skills from patterns
//! - **Skill Library**: Manage and execute skills
//!
//! # Example
//! ```rust,ignore
//! use sentient_skills::{SkillWeaver, ScreenWatcher};
//!
//! #[tokio::main]
//! async fn main() {
//!     let weaver = SkillWeaver::new();
//!     
//!     // Start watching user actions
//!     weaver.start_watching().await;
//!     
//!     // Auto-generate skills from patterns
//!     let skill = weaver.generate_skill("morning_routine").await;
//! }
//! ```

pub mod models;
pub mod weaver;
pub mod watcher;
pub mod patterns;
pub mod library;
pub mod executor;
pub mod dependency;
pub mod intent;
pub mod testing;

pub use models::{Skill, SkillAction, SkillTrigger, SkillCategory};
pub use weaver::{SkillWeaver, WeaverConfig};
pub use watcher::{ScreenWatcher, WatcherEvent, UserAction};
pub use patterns::{PatternDetector, ActionPattern};
pub use library::{SkillLibrary, SkillStats};
pub use executor::{SkillExecutor, ExecutionResult};
pub use dependency::{
    Dependency, DependencyGraph, DependencyResolver, DependencyError,
    Version, VersionConstraint, ResolvedDependency,
};

pub mod prelude {
    pub use crate::{SkillWeaver, Skill, SkillLibrary};
}

/// Result type for skill operations
pub type SkillResult<T> = Result<T, SkillError>;

/// Error type
#[derive(Debug, thiserror::Error)]
pub enum SkillError {
    #[error("Skill not found: {0}")]
    NotFound(String),
    
    #[error("Skill execution failed: {0}")]
    ExecutionFailed(String),
    
    #[error("Pattern detection failed: {0}")]
    DetectionFailed(String),
    
    #[error("Invalid skill definition: {0}")]
    InvalidDefinition(String),
    
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_error_creation() {
        let err = SkillError::NotFound("test_skill".into());
        assert!(err.to_string().contains("test_skill"));
    }
}
