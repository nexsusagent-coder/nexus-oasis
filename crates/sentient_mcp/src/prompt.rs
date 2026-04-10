//! MCP Prompt System
//!
//! Prompts are reusable templates that servers can expose to clients.
//! They allow users to quickly invoke common workflows.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Prompt definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prompt {
    /// Prompt name (unique identifier)
    pub name: String,
    /// Human-readable description
    pub description: String,
    /// Prompt arguments
    #[serde(default)]
    pub arguments: Vec<PromptArgument>,
}

impl Prompt {
    /// Create a new prompt
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            arguments: Vec::new(),
        }
    }

    /// Add an argument
    pub fn with_argument(mut self, arg: PromptArgument) -> Self {
        self.arguments.push(arg);
        self
    }
}

/// Prompt argument definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptArgument {
    /// Argument name
    pub name: String,
    /// Human-readable description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Whether this argument is required
    #[serde(default)]
    pub required: bool,
}

impl PromptArgument {
    /// Create a new prompt argument
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: None,
            required: false,
        }
    }

    /// Add description
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Mark as required
    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }
}

/// Prompt message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptMessage {
    /// Role (user or assistant)
    pub role: crate::types::Role,
    /// Message content
    pub content: crate::types::Content,
}

impl PromptMessage {
    /// Create a user message
    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: crate::types::Role::User,
            content: crate::types::Content::text(content),
        }
    }

    /// Create an assistant message
    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: crate::types::Role::Assistant,
            content: crate::types::Content::text(content),
        }
    }
}

/// Get prompt request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPromptRequest {
    /// Prompt name
    pub name: String,
    /// Prompt arguments
    #[serde(default)]
    pub arguments: HashMap<String, String>,
}

/// Get prompt result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPromptResult {
    /// Optional description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Prompt messages
    pub messages: Vec<PromptMessage>,
}

/// List prompts request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListPromptsRequest {
    /// Optional cursor for pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

/// List prompts result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListPromptsResult {
    /// List of prompts
    pub prompts: Vec<Prompt>,
    /// Next cursor for pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

/// Prompt template handler
pub trait PromptHandler: Send + Sync {
    /// Get prompt definition
    fn definition(&self) -> Prompt;
    
    /// Render prompt with arguments
    fn render(&self, arguments: &HashMap<String, String>) -> crate::Result<GetPromptResult>;
}

/// Built-in code review prompt
pub struct CodeReviewPrompt;

impl PromptHandler for CodeReviewPrompt {
    fn definition(&self) -> Prompt {
        Prompt::new("code_review", "Review code for quality and issues")
            .with_argument(
                PromptArgument::new("code")
                    .with_description("The code to review")
                    .required()
            )
            .with_argument(
                PromptArgument::new("language")
                    .with_description("Programming language")
            )
    }

    fn render(&self, arguments: &HashMap<String, String>) -> crate::Result<GetPromptResult> {
        let code = arguments.get("code")
            .ok_or_else(|| crate::McpError::invalid_params("Missing 'code' argument"))?;
        
        let language = arguments.get("language").map(|s| s.as_str()).unwrap_or("unknown");
        
        Ok(GetPromptResult {
            description: Some("Code review prompt".to_string()),
            messages: vec![
                PromptMessage::user(format!(
                    "Please review the following {} code and provide feedback on:\n\
                    1. Code quality and readability\n\
                    2. Potential bugs or issues\n\
                    3. Performance considerations\n\
                    4. Security vulnerabilities\n\
                    5. Suggestions for improvement\n\n\
                    ```{}\n{}\n```",
                    language, language, code
                )),
            ],
        })
    }
}

/// Built-in explain code prompt
pub struct ExplainCodePrompt;

impl PromptHandler for ExplainCodePrompt {
    fn definition(&self) -> Prompt {
        Prompt::new("explain_code", "Explain what a piece of code does")
            .with_argument(
                PromptArgument::new("code")
                    .with_description("The code to explain")
                    .required()
            )
    }

    fn render(&self, arguments: &HashMap<String, String>) -> crate::Result<GetPromptResult> {
        let code = arguments.get("code")
            .ok_or_else(|| crate::McpError::invalid_params("Missing 'code' argument"))?;
        
        Ok(GetPromptResult {
            description: Some("Code explanation prompt".to_string()),
            messages: vec![
                PromptMessage::user(format!(
                    "Please explain what the following code does, step by step:\n\n```\n{}\n```",
                    code
                )),
            ],
        })
    }
}

/// Built-in debug prompt
pub struct DebugPrompt;

impl PromptHandler for DebugPrompt {
    fn definition(&self) -> Prompt {
        Prompt::new("debug", "Help debug an issue")
            .with_argument(
                PromptArgument::new("error")
                    .with_description("The error message or description")
                    .required()
            )
            .with_argument(
                PromptArgument::new("code")
                    .with_description("The problematic code")
            )
            .with_argument(
                PromptArgument::new("context")
                    .with_description("Additional context")
            )
    }

    fn render(&self, arguments: &HashMap<String, String>) -> crate::Result<GetPromptResult> {
        let error = arguments.get("error")
            .ok_or_else(|| crate::McpError::invalid_params("Missing 'error' argument"))?;
        
        let mut message = format!(
            "Help me debug this issue:\n\n**Error:**\n{}\n",
            error
        );
        
        if let Some(code) = arguments.get("code") {
            message.push_str(&format!("\n**Code:**\n```\n{}\n```\n", code));
        }
        
        if let Some(context) = arguments.get("context") {
            message.push_str(&format!("\n**Context:**\n{}\n", context));
        }
        
        message.push_str("\nPlease analyze the problem and suggest solutions.");
        
        Ok(GetPromptResult {
            description: Some("Debug assistance prompt".to_string()),
            messages: vec![PromptMessage::user(message)],
        })
    }
}

/// Prompt manager for managing prompts
pub struct PromptManager {
    prompts: HashMap<String, Box<dyn PromptHandler>>,
}

impl PromptManager {
    /// Create a new prompt manager
    pub fn new() -> Self {
        Self {
            prompts: HashMap::new(),
        }
    }

    /// Register a prompt
    pub fn register<P: PromptHandler + 'static>(&mut self, prompt: P) {
        let name = prompt.definition().name.clone();
        self.prompts.insert(name, Box::new(prompt));
    }

    /// Get a prompt handler by name
    pub fn get(&self, name: &str) -> Option<&dyn PromptHandler> {
        self.prompts.get(name).map(|p| p.as_ref())
    }

    /// List all prompts
    pub fn list(&self) -> Vec<Prompt> {
        self.prompts.values().map(|p| p.definition()).collect()
    }

    /// Render a prompt
    pub fn render(&self, name: &str, arguments: &HashMap<String, String>) -> crate::Result<GetPromptResult> {
        let handler = self.prompts.get(name)
            .ok_or_else(|| crate::McpError::invalid_params(format!("Prompt not found: {}", name)))?;
        
        handler.render(arguments)
    }

    /// Check if a prompt exists
    pub fn contains(&self, name: &str) -> bool {
        self.prompts.contains_key(name)
    }
}

impl Default for PromptManager {
    fn default() -> Self {
        let mut manager = Self::new();
        manager.register(CodeReviewPrompt);
        manager.register(ExplainCodePrompt);
        manager.register(DebugPrompt);
        manager
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prompt_creation() {
        let prompt = Prompt::new("test", "A test prompt")
            .with_argument(PromptArgument::new("input").required());
        
        assert_eq!(prompt.name, "test");
        assert_eq!(prompt.arguments.len(), 1);
        assert!(prompt.arguments[0].required);
    }

    #[test]
    fn test_code_review_prompt() {
        let handler = CodeReviewPrompt;
        let def = handler.definition();
        
        assert_eq!(def.name, "code_review");
        assert!(!def.arguments.is_empty());
    }

    #[test]
    fn test_render_prompt() {
        let handler = CodeReviewPrompt;
        let mut args = HashMap::new();
        args.insert("code".to_string(), "fn main() {}".to_string());
        
        let result = handler.render(&args).expect("Should render");
        assert!(!result.messages.is_empty());
    }

    #[test]
    fn test_prompt_manager() {
        let manager = PromptManager::default();
        
        assert!(manager.contains("code_review"));
        assert!(manager.contains("explain_code"));
        assert!(manager.contains("debug"));
        
        let prompts = manager.list();
        assert!(!prompts.is_empty());
    }
}
