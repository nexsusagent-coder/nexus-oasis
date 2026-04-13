// ═══════════════════════════════════════════════════════════════════════════════
//  SENTIENT OS - E2B Code Sandbox Integration
// ═══════════════════════════════════════════════════════════════════════════════
//  Secure code execution using E2B's Firecracker microVMs
//  - Isolated environments
//  - Multiple language support (Python, JS, Rust, etc.)
//  - AI agent safe code execution
//  - File system operations
// ═══════════════════════════════════════════════════════════════════════════════

pub mod sandbox;
pub mod templates;
pub mod files;
pub mod terminal;
pub mod error;
pub mod local_sandbox;

pub use sandbox::{Sandbox, SandboxBuilder, SandboxMetadata};
pub use templates::{Template, TemplateLanguage, BuiltinTemplate};
pub use files::{FileInfo, FileType};
pub use terminal::{TerminalOutput, TerminalError};
pub use error::{SandboxError, Result};

use serde::{Deserialize, Serialize};

/// E2B API configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxConfig {
    /// API key (get from https://e2b.dev)
    pub api_key: String,
    /// Base URL (default: https://api.e2b.dev)
    pub base_url: String,
    /// Default template
    pub default_template: String,
    /// Sandbox timeout in seconds (default: 300)
    pub timeout_secs: u64,
    /// Request timeout
    pub request_timeout_secs: u64,
}

impl SandboxConfig {
    /// Create new config with API key
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            base_url: "https://api.e2b.dev".to_string(),
            default_template: "base".to_string(),
            timeout_secs: 300,      // 5 minutes
            request_timeout_secs: 30,
        }
    }

    /// Set base URL
    pub fn with_base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into();
        self
    }

    /// Set default template
    pub fn with_template(mut self, template: impl Into<String>) -> Self {
        self.default_template = template.into();
        self
    }

    /// Set sandbox timeout
    pub fn with_timeout(mut self, secs: u64) -> Self {
        self.timeout_secs = secs;
        self
    }

    /// Load from environment variable E2B_API_KEY
    pub fn from_env() -> Result<Self> {
        let api_key = std::env::var("E2B_API_KEY")
            .map_err(|_| SandboxError::MissingApiKey)?;
        Ok(Self::new(api_key))
    }
}

impl Default for SandboxConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            base_url: "https://api.e2b.dev".to_string(),
            default_template: "base".to_string(),
            timeout_secs: 300,
            request_timeout_secs: 30,
        }
    }
}

/// Execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResult {
    /// stdout output
    pub stdout: String,
    /// stderr output
    pub stderr: String,
    /// Exit code
    pub exit_code: i32,
    /// Execution time in milliseconds
    pub duration_ms: u64,
    /// Whether execution succeeded
    pub success: bool,
}

impl ExecutionResult {
    /// Check if execution was successful
    pub fn is_success(&self) -> bool {
        self.exit_code == 0
    }

    /// Get combined output
    pub fn output(&self) -> String {
        if self.stderr.is_empty() {
            self.stdout.clone()
        } else {
            format!("{}\n{}", self.stdout, self.stderr)
        }
    }
}

/// Code snippet for execution
#[derive(Debug, Clone, Serialize)]
pub struct CodeSnippet {
    /// Source code
    pub source: String,
    /// Language
    pub language: String,
    /// Entry point file (default: main.py for Python)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entrypoint: Option<String>,
}

impl CodeSnippet {
    /// Create new code snippet
    pub fn new(language: impl Into<String>, source: impl Into<String>) -> Self {
        Self {
            source: source.into(),
            language: language.into(),
            entrypoint: None,
        }
    }

    /// Python code
    pub fn python(source: impl Into<String>) -> Self {
        Self::new("python", source).with_entrypoint("main.py")
    }

    /// JavaScript code
    pub fn javascript(source: impl Into<String>) -> Self {
        Self::new("javascript", source).with_entrypoint("index.js")
    }

    /// TypeScript code
    pub fn typescript(source: impl Into<String>) -> Self {
        Self::new("typescript", source).with_entrypoint("index.ts")
    }

    /// Rust code
    pub fn rust(source: impl Into<String>) -> Self {
        Self::new("rust", source).with_entrypoint("main.rs")
    }

    /// Set entry point
    pub fn with_entrypoint(mut self, entry: impl Into<String>) -> Self {
        self.entrypoint = Some(entry.into());
        self
    }
}

// Re-export for convenience
pub mod prelude {
    pub use crate::{Sandbox, SandboxBuilder, SandboxConfig};
    pub use crate::{CodeSnippet, ExecutionResult};
    pub use crate::templates::BuiltinTemplate;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_creation() {
        let config = SandboxConfig::new("test-key");
        assert_eq!(config.api_key, "test-key");
        assert_eq!(config.timeout_secs, 300);
    }

    #[test]
    fn test_config_builder() {
        let config = SandboxConfig::new("test-key")
            .with_template("python")
            .with_timeout(600);
        
        assert_eq!(config.default_template, "python");
        assert_eq!(config.timeout_secs, 600);
    }

    #[test]
    fn test_code_snippet_python() {
        let snippet = CodeSnippet::python("print('hello')");
        assert_eq!(snippet.language, "python");
        assert_eq!(snippet.entrypoint, Some("main.py".to_string()));
    }

    #[test]
    fn test_execution_result() {
        let result = ExecutionResult {
            stdout: "output".to_string(),
            stderr: String::new(),
            exit_code: 0,
            duration_ms: 100,
            success: true,
        };
        
        assert!(result.is_success());
        assert_eq!(result.output(), "output");
    }
}
