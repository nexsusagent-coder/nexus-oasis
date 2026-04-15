//! ─── SENTIENT SMART TODO SYSTEM ───
//!
//! AI-powered task management with self-researching capabilities
//!
//! # Features
//! - **Self-researching tasks**: Todo → research → draft → approve
//! - **Task decomposition**: Break big tasks into sub-tasks
//! - **Priority AI**: Auto-prioritize based on deadlines and importance
//! - **Progress tracking**: Track completion with smart notifications
//!
//! # Example
//! ```rust,ignore
//! use sentient_todo::{TodoSystem, Task, TaskBuilder};
//!
//! #[tokio::main]
//! async fn main() {
//!     let todo = TodoSystem::new();
//!     
//!     // Create a self-researching task
//!     let task = TaskBuilder::new("Research quantum computing for cryptography")
//!         .with_deadline(7)
//!         .with_research(true)
//!         .build();
//!     
//!     todo.add_task(task).await;
//!     
//!     // System will auto-research and create sub-tasks
//!     todo.process_pending().await;
//! }
//! ```

pub mod models;
pub mod system;
pub mod research;
pub mod decomposition;
pub mod priority;
pub mod tracking;

pub use models::{Task, TaskBuilder, TaskStatus, TaskPriority, SubTask};
pub use system::{TodoSystem, TodoConfig};
pub use research::{TaskResearcher, ResearchResult};
pub use decomposition::{TaskDecomposer, DecompositionResult};
pub use priority::{PriorityEngine, PriorityScore};
pub use tracking::{ProgressTracker, ProgressUpdate};

pub mod prelude {
    pub use crate::{TodoSystem, Task, TaskBuilder, TaskStatus};
}

/// Result type for todo operations
pub type TodoResult<T> = Result<T, TodoError>;

/// Error type
#[derive(Debug, thiserror::Error)]
pub enum TodoError {
    #[error("Task not found: {0}")]
    NotFound(String),
    
    #[error("Invalid task state: {0}")]
    InvalidState(String),
    
    #[error("Research failed: {0}")]
    ResearchFailed(String),
    
    #[error("Decomposition failed: {0}")]
    DecompositionFailed(String),
    
    #[error(transparent)]
    Io(#[from] std::io::Error),
    
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_task_creation() {
        let task = TaskBuilder::new("Test task")
            .with_priority(TaskPriority::High)
            .build();
        
        assert_eq!(task.title, "Test task");
        assert_eq!(task.priority, TaskPriority::High);
    }
}
