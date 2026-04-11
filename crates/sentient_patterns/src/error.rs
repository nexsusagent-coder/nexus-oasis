// ═══════════════════════════════════════════════════════════════════════════════
//  SENTIENT OS - Pattern Errors
// ═══════════════════════════════════════════════════════════════════════════════

use thiserror::Error;

pub type Result<T> = std::result::Result<T, PatternError>;

/// Pattern errors
#[derive(Debug, Error)]
pub enum PatternError {
    #[error("Maximum iterations ({0}) exceeded")]
    MaxIterationsExceeded(usize),

    #[error("No valid action found")]
    NoValidAction,

    #[error("Tool execution failed: {0}")]
    ToolExecutionFailed(String),

    #[error("Invalid reasoning step: {0}")]
    InvalidStep(String),

    #[error("Plan execution failed at step {step}: {reason}")]
    PlanExecutionFailed { step: usize, reason: String },

    #[error("Reflection failed: {0}")]
    ReflectionFailed(String),

    #[error("No answer found")]
    NoAnswerFound,

    #[error("JSON parsing failed: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    IoError(String),
}
