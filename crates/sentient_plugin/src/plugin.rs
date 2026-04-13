//! Plugin trait and lifecycle management

use crate::types::*;
use crate::{PluginError, Result};
use async_trait::async_trait;
use serde_json::Value;

/// Plugin trait - all plugins must implement this
#[async_trait]
pub trait Plugin: Send + Sync {
    /// Get plugin metadata
    fn metadata(&self) -> &PluginManifest;

    /// Initialize plugin
    async fn initialize(&mut self, config: &PluginConfig) -> Result<()>;

    /// Shutdown plugin
    async fn shutdown(&mut self) -> Result<()>;

    /// Get plugin status
    fn status(&self) -> PluginStatus;

    /// Execute plugin command
    async fn execute(&self, command: &str, args: Value, context: &PluginContext) -> Result<Value>;

    /// Get plugin tools (if capable)
    fn tools(&self) -> Vec<ToolDefinition> {
        Vec::new()
    }

    /// Get plugin resources (if capable)
    fn resources(&self) -> Vec<ResourceDefinition> {
        Vec::new()
    }

    /// Get plugin prompts (if capable)
    fn prompts(&self) -> Vec<PromptDefinition> {
        Vec::new()
    }

    /// Handle event (if capable)
    async fn on_event(&self, _event: &PluginEvent) -> Result<()> {
        Ok(())
    }

    /// Called before configuration change
    fn on_config_change(&mut self, _old: &PluginConfig, _new: &PluginConfig) -> Result<()> {
        Ok(())
    }

    /// Health check
    async fn health_check(&self) -> Result<bool> {
        Ok(true)
    }
}

/// Tool definition for plugins
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    pub input_schema: Value,
    #[serde(default)]
    pub tags: Vec<String>,
}

impl ToolDefinition {
    pub fn new(name: impl Into<String>, description: impl Into<String>, input_schema: Value) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            input_schema,
            tags: Vec::new(),
        }
    }

    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }
}

/// Resource definition for plugins
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ResourceDefinition {
    pub uri: String,
    pub name: String,
    pub description: String,
    pub mime_type: String,
}

impl ResourceDefinition {
    pub fn new(uri: impl Into<String>, name: impl Into<String>, mime_type: impl Into<String>) -> Self {
        Self {
            uri: uri.into(),
            name: name.into(),
            description: String::new(),
            mime_type: mime_type.into(),
        }
    }

    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = desc.into();
        self
    }
}

/// Prompt definition for plugins
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PromptDefinition {
    pub name: String,
    pub description: String,
    pub template: String,
    #[serde(default)]
    pub arguments: Vec<PromptArgument>,
}

impl PromptDefinition {
    pub fn new(name: impl Into<String>, template: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: String::new(),
            template: template.into(),
            arguments: Vec::new(),
        }
    }

    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = desc.into();
        self
    }
}

/// Prompt argument
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PromptArgument {
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub required: bool,
    #[serde(default)]
    pub default: Option<String>,
}

/// Plugin event
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PluginEvent {
    pub event_type: EventType,
    pub source: String,
    pub data: Value,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl PluginEvent {
    pub fn new(event_type: EventType, source: impl Into<String>, data: Value) -> Self {
        Self {
            event_type,
            source: source.into(),
            data,
            timestamp: chrono::Utc::now(),
        }
    }
}

/// Event types
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventType {
    // Lifecycle events
    PluginLoaded,
    PluginUnloaded,
    PluginEnabled,
    PluginDisabled,
    PluginError,

    // System events
    SystemStart,
    SystemShutdown,
    SystemConfigChange,

    // Agent events
    AgentCreated,
    AgentDestroyed,
    AgentMessage,

    // Tool events
    ToolRegistered,
    ToolUnregistered,
    ToolExecuted,

    // Memory events
    MemoryStored,
    MemoryRetrieved,
    MemoryCleared,

    // Custom event
    Custom,
}

/// Built-in example plugin
pub struct ExamplePlugin {
    manifest: PluginManifest,
    status: PluginStatus,
    config: Option<PluginConfig>,
}

impl ExamplePlugin {
    pub fn new() -> Self {
        let manifest = PluginManifest {
            id: "sentient-example".to_string(),
            name: "Example Plugin".to_string(),
            version: "1.0.0".to_string(),
            description: "Built-in example plugin for testing".to_string(),
            author: "SENTIENT OS".to_string(),
            license: "MIT".to_string(),
            api_version: crate::types::PLUGIN_API_VERSION.to_string(),
            entry_point: "builtin".to_string(),
            capabilities: vec![PluginCapability::Tools, PluginCapability::Events],
            ..Default::default()
        };

        Self {
            manifest,
            status: PluginStatus::Unloaded,
            config: None,
        }
    }
}

impl Default for ExamplePlugin {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Plugin for ExamplePlugin {
    fn metadata(&self) -> &PluginManifest {
        &self.manifest
    }

    async fn initialize(&mut self, config: &PluginConfig) -> Result<()> {
        self.config = Some(config.clone());
        self.status = PluginStatus::Active;
        tracing::info!("Example plugin initialized");
        Ok(())
    }

    async fn shutdown(&mut self) -> Result<()> {
        self.status = PluginStatus::Unloaded;
        self.config = None;
        tracing::info!("Example plugin shutdown");
        Ok(())
    }

    fn status(&self) -> PluginStatus {
        self.status
    }

    async fn execute(&self, command: &str, args: Value, _context: &PluginContext) -> Result<Value> {
        match command {
            "echo" => {
                Ok(args)
            }
            "version" => {
                Ok(serde_json::json!({
                    "version": self.manifest.version,
                    "api_version": self.manifest.api_version
                }))
            }
            _ => Err(PluginError::execution_failed(format!(
                "Unknown command: {}",
                command
            ))),
        }
    }

    fn tools(&self) -> Vec<ToolDefinition> {
        vec![
            ToolDefinition::new(
                "example_echo",
                "Echo the input back",
                serde_json::json!({
                    "type": "object",
                    "properties": {
                        "message": { "type": "string" }
                    }
                }),
            ),
        ]
    }

    async fn on_event(&self, event: &PluginEvent) -> Result<()> {
        tracing::debug!("Example plugin received event: {:?}", event.event_type);
        Ok(())
    }
}

/// Plugin hook for extending functionality
#[async_trait]
pub trait PluginHook: Send + Sync {
    /// Hook name
    fn name(&self) -> &str;

    /// Execute hook
    async fn execute(&self, context: &PluginContext, data: Value) -> Result<Value>;
}

/// Plugin middleware
#[async_trait]
pub trait PluginMiddleware: Send + Sync {
    /// Process request before plugin execution
    async fn before(&self, context: &PluginContext, command: &str, args: &mut Value) -> Result<()>;

    /// Process response after plugin execution
    async fn after(&self, context: &PluginContext, result: &mut Result<Value>);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_example_plugin() {
        let mut plugin = ExamplePlugin::new();
        assert_eq!(plugin.status(), PluginStatus::Unloaded);

        let config = PluginConfig::new("sentient-example");
        plugin.initialize(&config).await.unwrap();
        assert_eq!(plugin.status(), PluginStatus::Active);

        let ctx = PluginContext::new();
        let result = plugin.execute("version", serde_json::json!({}), &ctx).await.unwrap();
        assert_eq!(result["version"], "1.0.0");

        plugin.shutdown().await.unwrap();
        assert_eq!(plugin.status(), PluginStatus::Unloaded);
    }

    #[test]
    fn test_tool_definition() {
        let tool = ToolDefinition::new(
            "test_tool",
            "A test tool",
            serde_json::json!({"type": "object"}),
        );

        assert_eq!(tool.name, "test_tool");
    }

    #[test]
    fn test_event_creation() {
        let event = PluginEvent::new(
            EventType::PluginLoaded,
            "test-plugin",
            serde_json::json!({"version": "1.0.0"}),
        );

        assert_eq!(event.source, "test-plugin");
    }
}
