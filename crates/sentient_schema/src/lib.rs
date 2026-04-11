// ═══════════════════════════════════════════════════════════════════════════════
//  SENTIENT OS - Structured Output & Schema
// ═══════════════════════════════════════════════════════════════════════════════
//  Reliable structured outputs from LLMs
//  - JSON Schema generation
//  - Function calling
//  - Instructor-style extraction
//  - Multi-model support (OpenAI, Claude, Gemini, Ollama)
// ═══════════════════════════════════════════════════════════════════════════════

pub mod schema;
pub mod extractor;
pub mod function;
pub mod error;
pub mod providers;

pub use schema::{Schema, SchemaBuilder, JsonSchema};
pub use extractor::{StructuredExtractor, ExtractionConfig};
pub use function::{FunctionDef, Parameter, FunctionCall};
pub use error::{SchemaError, Result};
pub use providers::{openai::OpenAIProvider, anthropic::AnthropicProvider, ollama::OllamaProvider};

use serde::{Deserialize, Serialize};

/// Core trait for structured output
pub trait StructuredOutput: Serialize + for<'de> Deserialize<'de> {
    /// Get JSON Schema for this type
    fn schema() -> serde_json::Value;
    
    /// Validate the output
    fn validate(&self) -> Result<()>;
}

/// Main client for structured outputs
pub struct StructuredLLM {
    provider: Box<dyn providers::SchemaProvider + Send + Sync>,
    config: ExtractionConfig,
}

impl StructuredLLM {
    /// Create with OpenAI provider
    pub fn openai(api_key: impl Into<String>) -> Self {
        Self {
            provider: Box::new(OpenAIProvider::new(api_key)),
            config: ExtractionConfig::default(),
        }
    }

    /// Create with Anthropic provider
    pub fn anthropic(api_key: impl Into<String>) -> Self {
        Self {
            provider: Box::new(AnthropicProvider::new(api_key)),
            config: ExtractionConfig::default(),
        }
    }

    /// Create with Ollama provider (local)
    pub fn ollama(base_url: Option<&str>) -> Self {
        Self {
            provider: Box::new(OllamaProvider::new(base_url)),
            config: ExtractionConfig::default(),
        }
    }

    /// Set extraction config
    pub fn with_config(mut self, config: ExtractionConfig) -> Self {
        self.config = config;
        self
    }

    /// Extract structured data from a prompt
    pub async fn extract<T: StructuredOutput>(&self, prompt: &str) -> Result<T> {
        let schema = T::schema();
        let response = self.provider.generate(prompt, schema).await?;
        
        // Parse and validate
        let output: T = serde_json::from_str(&response)?;
        output.validate()?;
        
        Ok(output)
    }

    /// Extract with retries
    pub async fn extract_with_retry<T: StructuredOutput>(
        &self,
        prompt: &str,
        max_retries: u32,
    ) -> Result<T> {
        let mut last_error = None;
        
        for attempt in 0..=max_retries {
            match self.extract::<T>(prompt).await {
                Ok(output) => return Ok(output),
                Err(e) => {
                    last_error = Some(e);
                    if attempt < max_retries {
                        tracing::warn!("Attempt {} failed, retrying...", attempt + 1);
                        tokio::time::sleep(tokio::time::Duration::from_millis(100 * (attempt + 1) as u64)).await;
                    }
                }
            }
        }
        
        Err(last_error.unwrap_or(SchemaError::ExtractionFailed("Unknown error".to_string())))
    }

    /// Generate with function calling
    pub async fn function_call(
        &self,
        prompt: &str,
        functions: Vec<FunctionDef>,
    ) -> Result<FunctionCall> {
        self.provider.function_call(prompt, functions).await
    }

    /// Check if provider is available
    pub async fn is_available(&self) -> bool {
        self.provider.is_available().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_structured_llm_creation() {
        let _llm = StructuredLLM::ollama(None);
    }
}
