//! MCP Types - Core type definitions for Model Context Protocol

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Server information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerInfo {
    /// Server name
    pub name: String,
    /// Server version
    pub version: String,
    /// Protocol version
    #[serde(default = "default_protocol_version")]
    pub protocol_version: String,
    /// Server capabilities
    #[serde(default)]
    pub capabilities: ServerCapabilities,
    /// Server instructions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
}

fn default_protocol_version() -> String {
    crate::MCP_VERSION.to_string()
}

/// Server capabilities
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ServerCapabilities {
    /// Experimental features
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experimental: Option<HashMap<String, Value>>,
    /// Logging capability
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<LoggingCapability>,
    /// Prompts capability
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompts: Option<PromptsCapability>,
    /// Resources capability
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<ResourcesCapability>,
    /// Tools capability
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<ToolsCapability>,
}

/// Logging capability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingCapability {}

/// Prompts capability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptsCapability {
    /// Whether the server supports prompt list changes notifications
    #[serde(default)]
    pub list_changed: bool,
}

/// Resources capability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourcesCapability {
    /// Whether the server supports resource list changes notifications
    #[serde(default)]
    pub list_changed: bool,
    /// Whether the server supports resource subscriptions
    #[serde(default)]
    pub subscribe: bool,
}

/// Tools capability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolsCapability {
    /// Whether the server supports tool list changes notifications
    #[serde(default)]
    pub list_changed: bool,
}

/// Client information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientInfo {
    /// Client name
    pub name: String,
    /// Client version
    pub version: String,
}

/// Client capabilities
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ClientCapabilities {
    /// Experimental features
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experimental: Option<HashMap<String, Value>>,
    /// Roots capability (for file system access)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roots: Option<RootsCapability>,
    /// Sampling capability (for LLM requests)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampling: Option<SamplingCapability>,
}

/// Roots capability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RootsCapability {
    /// Whether the client supports roots list changes notifications
    #[serde(default)]
    pub list_changed: bool,
}

/// Sampling capability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SamplingCapability {}

/// Initialize request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitializeRequest {
    /// Protocol version
    pub protocol_version: String,
    /// Client capabilities
    pub capabilities: ClientCapabilities,
    /// Client information
    pub client_info: ClientInfo,
}

/// Initialize result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitializeResult {
    /// Protocol version
    pub protocol_version: String,
    /// Server capabilities
    pub capabilities: ServerCapabilities,
    /// Server information
    pub server_info: ServerInfo,
    /// Server instructions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
}

/// Implementation information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Implementation {
    /// Name
    pub name: String,
    /// Version
    pub version: String,
}

impl Default for Implementation {
    fn default() -> Self {
        Self {
            name: "sentient-mcp".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }
}

/// Content block types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Content {
    /// Text content
    #[serde(rename = "text")]
    Text {
        /// The text content
        text: String,
    },
    /// Image content
    #[serde(rename = "image")]
    Image {
        /// Base64-encoded image data
        data: String,
        /// MIME type (e.g., "image/png")
        mime_type: String,
    },
    /// Resource content
    #[serde(rename = "resource")]
    Resource {
        /// The resource
        resource: ResourceContents,
    },
}

impl Content {
    /// Create text content
    pub fn text(text: impl Into<String>) -> Self {
        Content::Text { text: text.into() }
    }

    /// Create image content
    pub fn image(data: impl Into<String>, mime_type: impl Into<String>) -> Self {
        Content::Image {
            data: data.into(),
            mime_type: mime_type.into(),
        }
    }
}

/// Resource contents
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceContents {
    /// Resource URI
    pub uri: String,
    /// MIME type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    /// Text content (if text resource)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Binary content (if binary resource, base64-encoded)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blob: Option<String>,
}

/// Sampling message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SamplingMessage {
    /// Role (user or assistant)
    pub role: Role,
    /// Content
    pub content: Content,
}

/// Role
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    /// User role
    User,
    /// Assistant role
    Assistant,
}

/// Model preferences for sampling
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ModelPreferences {
    /// Hints for model selection
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hints: Option<Vec<ModelHint>>,
    /// Cost priority (0-1, higher = prefer cheaper)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_priority: Option<f32>,
    /// Speed priority (0-1, higher = prefer faster)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed_priority: Option<f32>,
    /// Intelligence priority (0-1, higher = prefer smarter)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intelligence_priority: Option<f32>,
}

/// Model hint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelHint {
    /// Model name hint
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Complete request for prompt/resource completion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompleteRequest {
    /// Reference to complete
    pub r#ref: Reference,
    /// Argument to complete
    pub argument: CompleteArgument,
}

/// Reference type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Reference {
    /// Prompt reference
    #[serde(rename = "ref/prompt")]
    Prompt {
        /// Prompt name
        name: String,
    },
    /// Resource reference
    #[serde(rename = "ref/resource")]
    Resource {
        /// Resource URI
        uri: String,
    },
}

/// Complete argument
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompleteArgument {
    /// Argument name
    pub name: String,
    /// Current value to complete
    pub value: String,
}

/// Complete result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompleteResult {
    /// Completion values
    pub completion: Completion,
}

/// Completion values
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Completion {
    /// Possible values
    pub values: Vec<String>,
    /// Total number of possible values (if paginated)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    /// True if there are more values
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_content_text() {
        let content = Content::text("Hello");
        let json = serde_json::to_string(&content).expect("Should serialize");
        assert!(json.contains("text"));
        assert!(json.contains("Hello"));
    }

    #[test]
    fn test_server_info() {
        let info = ServerInfo {
            name: "test-server".to_string(),
            version: "1.0.0".to_string(),
            protocol_version: crate::MCP_VERSION.to_string(),
            capabilities: ServerCapabilities::default(),
            instructions: None,
        };
        assert_eq!(info.name, "test-server");
    }

    #[test]
    fn test_implementation_default() {
        let impl_ = Implementation::default();
        assert_eq!(impl_.name, "sentient-mcp");
    }
}
