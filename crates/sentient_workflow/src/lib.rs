//! ─── SENTIENT Workflow Engine ───
//!
//! Visual flow builder for automation:
//! - n8n-style node-based workflows
//! - Pre-built templates
//! - Trigger-based execution
//! - Conditional branching

pub mod models;
pub mod builder;
pub mod executor;
pub mod triggers;
pub mod templates;

use serde::{Deserialize, Serialize};
use thiserror::Error;

pub use models::*;
pub use builder::WorkflowBuilder;
pub use executor::WorkflowExecutor;
pub use triggers::TriggerManager;
pub use templates::TemplateLibrary;

/// Workflow error
#[derive(Debug, Error)]
pub enum WorkflowError {
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Execution error: {0}")]
    Execution(String),
    
    #[error("Node error: {0}")]
    Node(String),
    
    #[error("Connection error: {0}")]
    Connection(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("Workflow not found: {0}")]
    NotFound(String),
    
    #[error("Cycle detected in workflow")]
    CycleDetected,
}

pub type WorkflowResult<T> = Result<T, WorkflowError>;

/// Workflow status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkflowStatus {
    Draft,
    Active,
    Paused,
    Completed,
    Failed,
}

impl Default for WorkflowStatus {
    fn default() -> Self {
        Self::Draft
    }
}

/// Execution status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExecutionStatus {
    Pending,
    Running,
    Success,
    Failed,
    Cancelled,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_workflow_status_default() {
        let status = WorkflowStatus::default();
        assert!(matches!(status, WorkflowStatus::Draft));
    }
}
