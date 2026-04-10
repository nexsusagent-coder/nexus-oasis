//! # SENTIENT MCP - Model Context Protocol Implementation
//!
//! Native Rust implementation of Anthropic's Model Context Protocol (MCP).
//! MCP enables seamless integration with Claude Desktop, Cursor, Windsurf,
//! and other AI tools that support the protocol.
//!
//! ## Features
//!
//! - **Transport Layer**: stdio, TCP, WebSocket, SSE
//! - **Client**: Connect to MCP servers
//! - **Server**: Expose tools, resources, and prompts
//! - **Resource Management**: Files, databases, APIs
//! - **Tool Calling**: Register and execute custom tools
//!
//! ## Quick Start
//!
//! ### Server Example
//!
//! ```rust
//! use sentient_mcp::{Server, ServerConfig, tool::{ToolExecutor, ToolCall, Tool, ToolResult}};
//! use async_trait::async_trait;
//!
//! // Define a custom tool
//! struct EchoTool;
//!
//! #[async_trait]
//! impl ToolExecutor for EchoTool {
//!     async fn execute(&self, call: ToolCall) -> sentient_mcp::Result<ToolResult> {
//!         let input = call.arguments.get("text")
//!             .and_then(|v| v.as_str())
//!             .unwrap_or("");
//!         Ok(ToolResult::text(format!("Echo: {}", input)))
//!     }
//!     
//!     fn definition(&self) -> Tool {
//!         Tool::simple("echo", "Echo the input text back")
//!     }
//! }
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let mut server = Server::new(ServerConfig::default());
//!
//! // Register the tool
//! server.register_tool(EchoTool);
//!
//! // Server is ready to accept connections
//! // server.run().await?;
//! # Ok(())
//! # }
//! ```
//!
//! ### Client Example
//!
//! ```rust
//! use sentient_mcp::{Client, ClientConfig};
//! use std::collections::HashMap;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Create client config
//! let config = ClientConfig::tcp("localhost", 8080);
//! let client = Client::new(config);
//!
//! // In real usage:
//! // client.connect().await?;
//! // let tools = client.list_tools().await?;
//! // let args = HashMap::from([("text".to_string(), serde_json::json!("Hello"))]);
//! // let result = client.call_tool("echo", args).await?;
//!
//! # Ok(())
//! # }
//! ```

#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]
#![allow(clippy::too_many_arguments)]

pub mod protocol;
pub mod client;
pub mod server;
pub mod transport;
pub mod types;
pub mod error;
pub mod resource;
pub mod tool;
pub mod prompt;

// Re-exports for convenience
pub use protocol::*;
pub use client::{Client, ClientConfig};
pub use server::{Server, ServerConfig};
pub use transport::{Transport, TransportConfig};
pub use types::*;
pub use error::{McpError, Result};
pub use resource::{Resource, ResourceManager};
pub use tool::{Tool, ToolExecutor, ToolResult};
pub use prompt::{Prompt, PromptManager};

/// MCP version supported by this implementation
pub const MCP_VERSION: &str = "2024-11-05";

/// Protocol version for JSON-RPC
pub const JSONRPC_VERSION: &str = "2.0";

/// Prelude module for common imports
pub mod prelude {
    pub use crate::protocol::*;
    pub use crate::client::{Client, ClientConfig};
    pub use crate::server::{Server, ServerConfig};
    pub use crate::types::*;
    pub use crate::error::{McpError, Result};
    pub use crate::tool::{Tool, ToolResult};
    pub use crate::resource::Resource;
    pub use crate::prompt::Prompt;
}

/// Internal module for tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mcp_version() {
        assert!(!MCP_VERSION.is_empty());
    }

    #[test]
    fn test_jsonrpc_version() {
        assert_eq!(JSONRPC_VERSION, "2.0");
    }
}
