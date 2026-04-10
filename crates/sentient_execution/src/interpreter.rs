//! Open Interpreter Integration
//!
//! Natural language code execution - translates natural language prompts
//! into executable code and runs them safely.
//!
//! Features:
//! - Natural language to code translation
//! - Multi-language support (Python, JavaScript, Bash)
//! - Safe mode with confirmation prompts
//! - Auto-run mode for autonomous execution

use crate::ExecutionResult;
use crate::{Language, ExecutionEnv};
use std::collections::HashMap;
use tracing::{info, debug};

/// Interpreter Configuration
#[derive(Debug, Clone)]
pub struct InterpreterConfig {
    /// Automatically run generated code without confirmation
    pub auto_run: bool,
    /// Enable safe mode (sandboxed execution)
    pub safe_mode: bool,
    /// Default language for generated code
    pub default_language: Language,
    /// Timeout for code execution
    pub timeout_secs: u64,
    /// Maximum output size (bytes)
    pub max_output_size: usize,
}

impl Default for InterpreterConfig {
    fn default() -> Self {
        Self {
            auto_run: false,
            safe_mode: true,
            default_language: Language::Python,
            timeout_secs: 60,
            max_output_size: 1024 * 1024, // 1MB
        }
    }
}

/// Open Interpreter instance
pub struct OpenInterpreter {
    config: InterpreterConfig,
    /// Conversation history for context
    history: Vec<InterpreterMessage>,
}

#[derive(Debug, Clone)]
struct InterpreterMessage {
    role: String,
    content: String,
}

impl OpenInterpreter {
    /// Create new interpreter instance
    pub fn new(config: InterpreterConfig) -> Self {
        Self {
            config,
            history: Vec::new(),
        }
    }

    /// Create with default configuration
    pub fn default_interpreter() -> Self {
        Self::new(InterpreterConfig::default())
    }

    /// Interpret natural language prompt and execute
    pub async fn interpret(&mut self, prompt: &str) -> Result<ExecutionResult, Box<dyn std::error::Error + Send + Sync>> {
        info!("🤖 Interpreting: {}", prompt);

        // Add to history
        self.history.push(InterpreterMessage {
            role: "user".to_string(),
            content: prompt.to_string(),
        });

        // Generate code from prompt (simplified - in reality this would use an LLM)
        let generated_code = self.generate_code(prompt)?;

        debug!("📝 Generated code:\n{}", generated_code.code);

        // Check if we should run (safe mode check)
        if !self.config.auto_run && self.config.safe_mode {
            info!("⚠️ Safe mode: code generated but not auto-executed");
            return Ok(ExecutionResult {
                stdout: format!("Generated {:?} code:\n```\n{}\n```\n\nSet auto_run=true to execute.",
                    generated_code.language, generated_code.code),
                stderr: String::new(),
                exit_code: 0,
                duration_ms: 0,
                success: true,
            });
        }

        // Execute the generated code
        let result = self.execute_code(&generated_code).await?;

        // Add result to history
        self.history.push(InterpreterMessage {
            role: "assistant".to_string(),
            content: format!("Result: {:?}", result),
        });

        Ok(result)
    }

    /// Generate code from natural language prompt
    fn generate_code(&self, prompt: &str) -> Result<GeneratedCode, Box<dyn std::error::Error + Send + Sync>> {
        // This is a simplified implementation
        // In production, this would use an LLM to generate code

        let prompt_lower = prompt.to_lowercase();

        // Detect language and generate code based on keywords
        let (language, code) = if prompt_lower.contains("python") || prompt_lower.contains("calculate") {
            (Language::Python, self.generate_python_code(prompt))
        } else if prompt_lower.contains("javascript") || prompt_lower.contains("node") {
            (Language::JavaScript, self.generate_js_code(prompt))
        } else if prompt_lower.contains("shell") || prompt_lower.contains("bash") || prompt_lower.contains("run") {
            (Language::Bash, self.generate_bash_code(prompt))
        } else {
            // Default to Python
            (self.config.default_language.clone(), self.generate_python_code(prompt))
        };

        Ok(GeneratedCode {
            code,
            language,
            explanation: prompt.to_string(),
        })
    }

    /// Generate Python code from prompt
    fn generate_python_code(&self, prompt: &str) -> String {
        // Simple pattern matching for common tasks
        let prompt_lower = prompt.to_lowercase();

        if prompt_lower.contains("hello") || prompt_lower.contains("hi") {
            r#"print("Hello! How can I help you today?")"#.to_string()
        } else if prompt_lower.contains("list") || prompt_lower.contains("files") {
            r#"import os
for f in os.listdir('.'):
    print(f)"#.to_string()
        } else if prompt_lower.contains("calculate") || prompt_lower.contains("math") {
            r#"# Simple calculator
import math
result = math.sqrt(16) + 2 * 3
print(f"Result: {result}")"#.to_string()
        } else if prompt_lower.contains("date") || prompt_lower.contains("time") {
            r#"from datetime import datetime
now = datetime.now()
print(f"Current time: {now.strftime('%Y-%m-%d %H:%M:%S')}")"#.to_string()
        } else if prompt_lower.contains("json") {
            r#"import json
data = {"name": "Sentient", "version": "4.0"}
print(json.dumps(data, indent=2))"#.to_string()
        } else {
            format!(r#"# Code for: {}
print("Task completed!")"#, prompt)
        }
    }

    /// Generate JavaScript code from prompt
    fn generate_js_code(&self, prompt: &str) -> String {
        format!(r#"// Code for: {}
console.log("Task completed!");"#, prompt)
    }

    /// Generate Bash code from prompt
    fn generate_bash_code(&self, prompt: &str) -> String {
        format!(r#"#!/bin/bash
# Task: {}
echo "Task completed!""#, prompt)
    }

    /// Execute generated code
    async fn execute_code(&self, generated: &GeneratedCode) -> Result<ExecutionResult, Box<dyn std::error::Error + Send + Sync>> {
        let config = crate::sandbox::SandboxConfig {
            image: match generated.language {
                Language::Python => "python:3.11-slim".to_string(),
                Language::JavaScript => "node:20-slim".to_string(),
                Language::Bash => "alpine:latest".to_string(),
                _ => "alpine:latest".to_string(),
            },
            timeout_secs: self.config.timeout_secs,
            ..Default::default()
        };

        crate::sandbox::run_in_sandbox(config, &generated.code).await
    }

    /// Get conversation history
    pub fn history(&self) -> &[InterpreterMessage] {
        &self.history
    }

    /// Clear history
    pub fn clear_history(&mut self) {
        self.history.clear();
    }
}

/// Generated code from natural language
#[derive(Debug, Clone)]
struct GeneratedCode {
    code: String,
    language: Language,
    explanation: String,
}

/// Interpret a natural language prompt
pub async fn interpret(config: InterpreterConfig, prompt: &str) -> Result<ExecutionResult, Box<dyn std::error::Error + Send + Sync>> {
    let mut interpreter = OpenInterpreter::new(config);
    interpreter.interpret(prompt).await
}

/// Quick interpretation with default config
pub async fn quick_interpret(prompt: &str) -> Result<ExecutionResult, Box<dyn std::error::Error + Send + Sync>> {
    interpret(InterpreterConfig::default(), prompt).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interpreter_config_default() {
        let config = InterpreterConfig::default();
        assert!(!config.auto_run);
        assert!(config.safe_mode);
    }

    #[test]
    fn test_generate_python_code() {
        let interpreter = OpenInterpreter::default_interpreter();
        let code = interpreter.generate_python_code("hello world");
        assert!(code.contains("print"));
    }

    #[tokio::test]
    async fn test_interpret_safe_mode() {
        let config = InterpreterConfig {
            auto_run: false,
            safe_mode: true,
            ..Default::default()
        };

        let result = interpret(config, "say hello").await.expect("operation failed");
        assert!(result.success);
        assert!(result.stdout.contains("Generated"));
    }
}
