// ═══════════════════════════════════════════════════════════════════════════════
//  SENTIENT OS - Schema Providers
// ═══════════════════════════════════════════════════════════════════════════════

pub mod openai;
pub mod anthropic;
pub mod ollama;

use async_trait::async_trait;
use crate::function::FunctionDef;
use crate::error::Result;

pub use openai::OpenAIProvider;
pub use anthropic::AnthropicProvider;
pub use ollama::OllamaProvider;

/// Provider trait for structured output
#[async_trait]
pub trait SchemaProvider {
    /// Generate structured output
    async fn generate(&self, prompt: &str, schema: serde_json::Value) -> Result<String>;
    
    /// Function calling
    async fn function_call(&self, prompt: &str, functions: Vec<FunctionDef>) -> Result<crate::function::FunctionCall>;
    
    /// Provider name
    fn name(&self) -> &'static str;
    
    /// Check if provider is available
    async fn is_available(&self) -> bool {
        false
    }
}
