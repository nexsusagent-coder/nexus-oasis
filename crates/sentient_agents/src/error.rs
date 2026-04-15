//! Agent Error Types

use thiserror::Error;

#[derive(Debug, Error)]
pub enum AgentError {
    #[error("Agent initialization failed: {0}")]
    InitFailed(String),
    
    #[error("Task execution failed: {0}")]
    TaskFailed(String),
    
    #[error("Agent communication error: {0}")]
    CommunicationError(String),
    
    #[error("Timeout exceeded: {0}")]
    Timeout(String),
    
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
    
    #[error("Agent not found: {0}")]
    AgentNotFound(String),
    
    #[error("Framework error: {0}")]
    FrameworkError(String),
    
    #[error("Maximum agent capacity reached")]
    CapacityReached,
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub type AgentResult<T> = Result<T, AgentError>;
