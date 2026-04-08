//! SENTIENT Skills - Skills System inspired by DeerFlow
//!
//! DeerFlow 2.0'dan esinlenilmiş skill yönetim sistemi

pub mod types;
pub mod loader;
pub mod manager;
pub mod guardrails;
pub mod subagent;
pub mod executor;

pub use types::{Skill, SkillMetadata, SkillTrigger, SkillCategory, TriggerType};
pub use loader::SkillLoader;
pub use manager::SkillManager;
pub use guardrails::{GuardrailMiddleware, GuardrailProvider, GuardrailDecision};
pub use subagent::{SubagentExecutor, SubagentConfig, SubagentResult};
pub use executor::SkillExecutor;

/// Skill error types
#[derive(Debug, thiserror::Error)]
pub enum SkillError {
    #[error("Skill not found: {0}")]
    NotFound(String),
    
    #[error("Skill parsing failed: {0}")]
    ParseError(String),
    
    #[error("Skill execution failed: {0}")]
    ExecutionError(String),
    
    #[error("Invalid skill format: {0}")]
    InvalidFormat(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("YAML parsing error: {0}")]
    Yaml(#[from] serde_yaml::Error),
}

/// Skill yükücü result
pub type SkillResult<T> = Result<T, SkillError>;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_skill_error_display() {
        let err = SkillError::NotFound("deep-research".to_string());
        assert!(err.to_string().contains("deep-research"));
    }
}
