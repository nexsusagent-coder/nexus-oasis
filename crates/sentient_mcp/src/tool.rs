//! MCP Tool System
//!
//! Tools are the primary way for MCP servers to expose functionality
//! that can be invoked by clients (like Claude Desktop).

use serde::{Deserialize, Serialize};
use serde_json::Value;
use async_trait::async_trait;
use std::collections::HashMap;

/// Tool definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    /// Tool name (unique identifier)
    pub name: String,
    /// Human-readable description
    pub description: String,
    /// JSON Schema for input parameters
    pub input_schema: Value,
}

impl Tool {
    /// Create a new tool
    pub fn new(name: impl Into<String>, description: impl Into<String>, input_schema: Value) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            input_schema,
        }
    }

    /// Create a simple tool with string input
    pub fn simple(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "input": {
                        "type": "string",
                        "description": "Input string"
                    }
                },
                "required": []
            }),
        }
    }

    /// Create a tool with no parameters
    pub fn no_params(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {},
                "required": []
            }),
        }
    }
}

/// Tool call request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    /// Tool name
    pub name: String,
    /// Tool arguments
    #[serde(default)]
    pub arguments: HashMap<String, Value>,
}

impl ToolCall {
    /// Create a new tool call
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            arguments: HashMap::new(),
        }
    }

    /// Add an argument
    pub fn with_arg(mut self, key: impl Into<String>, value: Value) -> Self {
        self.arguments.insert(key.into(), value);
        self
    }
}

/// Tool execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    /// Result content
    pub content: Vec<crate::types::Content>,
    /// Whether the tool execution failed
    #[serde(default)]
    pub is_error: bool,
}

impl ToolResult {
    /// Create a successful text result
    pub fn text(text: impl Into<String>) -> Self {
        Self {
            content: vec![crate::types::Content::text(text)],
            is_error: false,
        }
    }

    /// Create an error result
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            content: vec![crate::types::Content::text(message)],
            is_error: true,
        }
    }

    /// Create a result with multiple content items
    pub fn with_content(content: Vec<crate::types::Content>) -> Self {
        Self {
            content,
            is_error: false,
        }
    }

    /// Add content to the result
    pub fn add_content(mut self, content: crate::types::Content) -> Self {
        self.content.push(content);
        self
    }
}

/// Tool executor trait
#[async_trait]
pub trait ToolExecutor: Send + Sync {
    /// Execute the tool
    async fn execute(&self, call: ToolCall) -> crate::Result<ToolResult>;
    
    /// Get tool definition
    fn definition(&self) -> Tool;
    
    /// Validate input against schema (optional)
    fn validate_input(&self, _arguments: &HashMap<String, Value>) -> crate::Result<()> {
        Ok(())
    }
}

/// Built-in echo tool for testing
pub struct EchoTool;

#[async_trait]
impl ToolExecutor for EchoTool {
    async fn execute(&self, call: ToolCall) -> crate::Result<ToolResult> {
        let input = call.arguments.get("input")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        Ok(ToolResult::text(format!("Echo: {}", input)))
    }

    fn definition(&self) -> Tool {
        Tool::simple("echo", "Echo the input text back")
    }
}

/// Built-in current time tool
pub struct CurrentTimeTool;

#[async_trait]
impl ToolExecutor for CurrentTimeTool {
    async fn execute(&self, _call: ToolCall) -> crate::Result<ToolResult> {
        let now = chrono::Local::now();
        Ok(ToolResult::text(format!("Current time: {}", now.format("%Y-%m-%d %H:%M:%S"))))
    }

    fn definition(&self) -> Tool {
        Tool::no_params("current_time", "Get the current date and time")
    }
}

/// Tool registry for managing tools
pub struct ToolRegistry {
    tools: HashMap<String, Box<dyn ToolExecutor>>,
}

impl ToolRegistry {
    /// Create a new tool registry
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
        }
    }

    /// Register a tool
    pub fn register<T: ToolExecutor + 'static>(&mut self, tool: T) {
        let name = tool.definition().name.clone();
        self.tools.insert(name, Box::new(tool));
    }

    /// Get a tool by name
    pub fn get(&self, name: &str) -> Option<&dyn ToolExecutor> {
        self.tools.get(name).map(|t| t.as_ref())
    }

    /// List all tools
    pub fn list(&self) -> Vec<Tool> {
        self.tools.values().map(|t| t.definition()).collect()
    }

    /// Execute a tool
    pub async fn execute(&self, call: ToolCall) -> crate::Result<ToolResult> {
        let tool = self.tools.get(&call.name)
            .ok_or_else(|| crate::McpError::invalid_params(format!("Tool not found: {}", call.name)))?;
        
        tool.validate_input(&call.arguments)?;
        tool.execute(call).await
    }

    /// Check if a tool exists
    pub fn contains(&self, name: &str) -> bool {
        self.tools.contains_key(name)
    }

    /// Get tool count
    pub fn len(&self) -> usize {
        self.tools.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.tools.is_empty()
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        let mut registry = Self::new();
        registry.register(EchoTool);
        registry.register(CurrentTimeTool);
        registry
    }
}

/// List tools request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListToolsRequest {
    /// Optional cursor for pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

/// List tools result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListToolsResult {
    /// List of tools
    pub tools: Vec<Tool>,
    /// Next cursor for pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

/// Call tool request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallToolRequest {
    /// Tool name
    pub name: String,
    /// Tool arguments
    #[serde(default)]
    pub arguments: HashMap<String, Value>,
}

/// Call tool result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallToolResult {
    /// Result content
    pub content: Vec<crate::types::Content>,
    /// Whether the tool execution failed
    #[serde(default)]
    pub is_error: bool,
}

impl From<ToolResult> for CallToolResult {
    fn from(result: ToolResult) -> Self {
        Self {
            content: result.content,
            is_error: result.is_error,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_creation() {
        let tool = Tool::simple("test", "A test tool");
        assert_eq!(tool.name, "test");
        assert_eq!(tool.description, "A test tool");
    }

    #[tokio::test]
    async fn test_echo_tool() {
        let echo = EchoTool;
        let call = ToolCall::new("echo").with_arg("input", serde_json::json!("Hello"));
        let result = echo.execute(call).await.expect("Should execute");
        assert!(!result.is_error);
        // Check the text content
        match &result.content[0] {
            crate::types::Content::Text { text } => assert!(text.contains("Hello")),
            _ => panic!("Expected text content"),
        }
    }

    #[tokio::test]
    async fn test_tool_registry() {
        let mut registry = ToolRegistry::new();
        registry.register(EchoTool);
        
        assert!(registry.contains("echo"));
        assert!(registry.get("echo").is_some());
        
        let tools = registry.list();
        assert!(!tools.is_empty());
    }

    #[test]
    fn test_tool_result() {
        let result = ToolResult::text("Success");
        assert!(!result.is_error);
        
        let error = ToolResult::error("Failed");
        assert!(error.is_error);
    }
}
