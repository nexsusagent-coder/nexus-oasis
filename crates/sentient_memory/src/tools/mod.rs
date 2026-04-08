//! SENTIENT Memory Tools Module
//! Mem0 pattern'inden adapte

mod memory_tool;

pub use memory_tool::{MemoryTool, MemoryToolInput, MemoryToolOutput, MemoryEntrySummary};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Tool context
#[derive(Debug, Clone, Default)]
pub struct ToolContext {
    pub working_directory: PathBuf,
    pub session_id: String,
}

/// Tool trait
#[async_trait]
pub trait Tool: Send + Sync {
    type Input: Serialize + for<'de> Deserialize<'de> + Send + Sync;
    type Output: Serialize + for<'de> Deserialize<'de> + Send + Sync;
    
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn is_read_only(&self, input: &Self::Input) -> bool;
    
    async fn execute(
        &mut self,
        input: Self::Input,
        context: &ToolContext,
    ) -> ToolResult<Self::Output>;
}

pub type ToolResult<T> = Result<T, ToolError>;

#[derive(Debug, thiserror::Error)]
pub enum ToolError {
    #[error("İzin reddedildi: {0}")]
    PermissionDenied(String),
    
    #[error("Geçersiz input: {0}")]
    InvalidInput(String),
}
