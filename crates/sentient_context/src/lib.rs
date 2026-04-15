//! ─── SENTIENT Context Engineering ───
//!
//! Context management for AI interactions:
//! - AGENTS.md standard support
//! - PRP (Product Requirements Prompt) workflow
//! - Context window optimization
//! - Token-aware context building

pub mod agents_md;
pub mod prp;
pub mod builder;
pub mod optimizer;

use serde::{Deserialize, Serialize};
use thiserror::Error;

pub use agents_md::AgentsMd;
pub use prp::PrpWorkflow;
pub use builder::ContextBuilder;
pub use optimizer::ContextOptimizer;

/// Context error
#[derive(Debug, Error)]
pub enum ContextError {
    #[error("Failed to parse AGENTS.md: {0}")]
    ParseError(String),
    
    #[error("Context too large: {0} tokens")]
    TooLarge(u32),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("Template not found: {0}")]
    NotFound(String),
}

pub type ContextResult<T> = Result<T, ContextError>;

/// Context configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextConfig {
    pub max_tokens: u32,
    pub reserved_output_tokens: u32,
    pub include_file_contents: bool,
    pub include_git_info: bool,
}

impl Default for ContextConfig {
    fn default() -> Self {
        Self {
            max_tokens: 128_000,
            reserved_output_tokens: 4_096,
            include_file_contents: true,
            include_git_info: true,
        }
    }
}

/// Context section
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextSection {
    pub name: String,
    pub content: String,
    pub token_count: u32,
    pub priority: u8,
}

impl ContextSection {
    pub fn new(name: &str, content: &str) -> Self {
        Self {
            name: name.to_string(),
            content: content.to_string(),
            token_count: estimate_tokens(content),
            priority: 5,
        }
    }
    
    pub fn with_priority(mut self, priority: u8) -> Self {
        self.priority = priority;
        self
    }
}

/// Estimate token count (rough: ~4 chars per token)
fn estimate_tokens(text: &str) -> u32 {
    (text.len() / 4) as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_context_section() {
        let section = ContextSection::new("test", "Hello world");
        assert!(section.token_count > 0);
    }
}
