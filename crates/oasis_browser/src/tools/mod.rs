//! Oasis Browser Tools Module
//! Lightpanda & browser-use pattern'inden adapte

mod browser_tool;

pub use browser_tool::{BrowserTool, BrowserToolInput, BrowserToolOutput, BrowserMetadata};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Tool context
#[derive(Debug, Clone)]
pub struct ToolContext {
    pub working_directory: PathBuf,
    pub session_id: String,
}

impl Default for ToolContext {
    fn default() -> Self {
        Self {
            working_directory: std::env::current_dir().unwrap_or_default(),
            session_id: uuid::Uuid::new_v4().to_string(),
        }
    }
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
        &self,
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
    
    #[error("Ağ hatası: {0}")]
    NetworkError(String),
}
