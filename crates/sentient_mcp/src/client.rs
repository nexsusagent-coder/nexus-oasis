//! MCP Client Implementation
//!
//! Connect to MCP servers and interact with their tools, resources, and prompts.

use crate::protocol::{Message, Request, Response, Notification, RequestId};
use crate::types::*;
use crate::tool::*;
use crate::resource::*;
use crate::prompt::*;
use crate::transport::{Transport, TransportConfig};
use crate::{McpError, Result};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// MCP Client configuration
#[derive(Debug, Clone)]
pub struct ClientConfig {
    /// Transport configuration
    pub transport: TransportConfig,
    /// Client name
    pub name: String,
    /// Client version
    pub version: String,
    /// Request timeout in seconds
    pub timeout: u64,
    /// Client capabilities
    pub capabilities: ClientCapabilities,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            transport: TransportConfig::Stdio,
            name: "sentient-mcp-client".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            timeout: 30,
            capabilities: ClientCapabilities::default(),
        }
    }
}

impl ClientConfig {
    /// Create a new client config with stdio transport
    pub fn stdio(_command: &str) -> Self {
        Self {
            transport: TransportConfig::Stdio,
            ..Default::default()
        }
    }

    /// Create a new client config with TCP transport
    pub fn tcp(host: &str, port: u16) -> Self {
        Self {
            transport: TransportConfig::Tcp {
                host: host.to_string(),
                port,
            },
            ..Default::default()
        }
    }

    /// Create a new client config with WebSocket transport
    pub fn websocket(url: &str) -> Self {
        Self {
            transport: TransportConfig::WebSocket {
                url: url.to_string(),
            },
            ..Default::default()
        }
    }
}

/// Pending request tracker
struct PendingRequest {
    sender: tokio::sync::oneshot::Sender<Response>,
}

/// MCP Client
pub struct Client {
    config: ClientConfig,
    transport: Option<Box<dyn Transport>>,
    pending_requests: Arc<RwLock<HashMap<String, PendingRequest>>>,
    server_info: Arc<RwLock<Option<InitializeResult>>>,
    notification_handler: Option<Box<dyn Fn(Notification) + Send + Sync>>,
}

impl Client {
    /// Create a new MCP client
    pub fn new(config: ClientConfig) -> Self {
        Self {
            config,
            transport: None,
            pending_requests: Arc::new(RwLock::new(HashMap::new())),
            server_info: Arc::new(RwLock::new(None)),
            notification_handler: None,
        }
    }

    /// Set notification handler
    pub fn on_notification<F: Fn(Notification) + Send + Sync + 'static>(mut self, handler: F) -> Self {
        self.notification_handler = Some(Box::new(handler));
        self
    }

    /// Connect to the server
    pub async fn connect(&mut self) -> Result<()> {
        // Initialize transport based on config
        #[cfg(feature = "stdio")]
        if matches!(self.config.transport, TransportConfig::Stdio) {
            self.transport = Some(Box::new(crate::transport::StdioTransport::from_process()));
        }

        #[cfg(feature = "tcp")]
        if let TransportConfig::Tcp { host, port } = &self.config.transport {
            let transport = crate::transport::TcpTransport::connect(host, *port).await?;
            self.transport = Some(Box::new(transport));
        }

        #[cfg(feature = "websocket")]
        if let TransportConfig::WebSocket { url } = &self.config.transport {
            let transport = crate::transport::WebSocketTransport::connect(url).await?;
            self.transport = Some(Box::new(transport));
        }

        // Initialize connection
        self.initialize().await?;

        Ok(())
    }

    /// Initialize the MCP connection
    async fn initialize(&mut self) -> Result<()> {
        let init_request = InitializeRequest {
            protocol_version: crate::MCP_VERSION.to_string(),
            capabilities: self.config.capabilities.clone(),
            client_info: ClientInfo {
                name: self.config.name.clone(),
                version: self.config.version.clone(),
            },
        };

        let response = self.request("initialize", Some(serde_json::to_value(init_request)?)).await?;
        
        let result: InitializeResult = serde_json::from_value(response.result.unwrap_or(serde_json::json!({})))?;
        
        // Store server info
        {
            let mut server_info = self.server_info.write().await;
            *server_info = Some(result);
        }

        // Send initialized notification
        self.notify("notifications/initialized", None).await?;

        Ok(())
    }

    /// Send a request and wait for response
    pub async fn request(&mut self, method: &str, params: Option<serde_json::Value>) -> Result<Response> {
        let transport = self.transport.as_mut()
            .ok_or_else(|| McpError::connection("Not connected"))?;

        let id = Uuid::new_v4().to_string();
        let request = if let Some(p) = params {
            Request::with_params(method, p).with_id(id.clone())
        } else {
            Request::new(method).with_id(id.clone())
        };

        // Create oneshot channel for response
        let (tx, rx) = tokio::sync::oneshot::channel();
        
        // Register pending request
        {
            let mut pending = self.pending_requests.write().await;
            pending.insert(id.clone(), PendingRequest { sender: tx });
        }

        // Send request
        transport.send(Message::Request(request)).await?;

        // Wait for response with timeout
        let timeout_duration = tokio::time::Duration::from_secs(self.config.timeout);
        let response = tokio::time::timeout(timeout_duration, rx).await
            .map_err(|_| McpError::timeout("Request timed out"))?
            .map_err(|_| McpError::internal("Response channel closed"))?;

        // Handle error response
        if let Some(error) = &response.error {
            return Err(McpError::Protocol(error.clone()));
        }

        Ok(response)
    }

    /// Send a notification (no response expected)
    pub async fn notify(&mut self, method: &str, params: Option<serde_json::Value>) -> Result<()> {
        let transport = self.transport.as_mut()
            .ok_or_else(|| McpError::connection("Not connected"))?;

        let notification = if let Some(p) = params {
            Notification::with_params(method, p)
        } else {
            Notification::new(method)
        };

        transport.send(Message::Notification(notification)).await?;
        Ok(())
    }

    /// List available tools
    pub async fn list_tools(&mut self) -> Result<Vec<Tool>> {
        let response = self.request("tools/list", None).await?;
        
        let result: ListToolsResult = serde_json::from_value(
            response.result.unwrap_or(serde_json::json!({}))
        )?;

        Ok(result.tools)
    }

    /// Call a tool
    pub async fn call_tool(&mut self, name: &str, arguments: HashMap<String, serde_json::Value>) -> Result<CallToolResult> {
        let params = serde_json::json!({
            "name": name,
            "arguments": arguments
        });

        let response = self.request("tools/call", Some(params)).await?;
        
        let result: CallToolResult = serde_json::from_value(
            response.result.unwrap_or(serde_json::json!({}))
        )?;

        Ok(result)
    }

    /// List available resources
    pub async fn list_resources(&mut self) -> Result<Vec<Resource>> {
        let response = self.request("resources/list", None).await?;
        
        let result: ListResourcesResult = serde_json::from_value(
            response.result.unwrap_or(serde_json::json!({}))
        )?;

        Ok(result.resources)
    }

    /// Read a resource
    pub async fn read_resource(&mut self, uri: &str) -> Result<ResourceContents> {
        let params = serde_json::json!({ "uri": uri });
        let response = self.request("resources/read", Some(params)).await?;
        
        let result: ReadResourceResult = serde_json::from_value(
            response.result.unwrap_or(serde_json::json!({}))
        )?;

        result.contents.into_iter().next()
            .ok_or_else(|| McpError::resource_not_found(uri))
    }

    /// List available prompts
    pub async fn list_prompts(&mut self) -> Result<Vec<Prompt>> {
        let response = self.request("prompts/list", None).await?;
        
        let result: ListPromptsResult = serde_json::from_value(
            response.result.unwrap_or(serde_json::json!({}))
        )?;

        Ok(result.prompts)
    }

    /// Get a prompt
    pub async fn get_prompt(&mut self, name: &str, arguments: HashMap<String, String>) -> Result<GetPromptResult> {
        let params = serde_json::json!({
            "name": name,
            "arguments": arguments
        });

        let response = self.request("prompts/get", Some(params)).await?;
        
        let result: GetPromptResult = serde_json::from_value(
            response.result.unwrap_or(serde_json::json!({}))
        )?;

        Ok(result)
    }

    /// Get server info
    pub async fn server_info(&self) -> Option<InitializeResult> {
        self.server_info.read().await.clone()
    }

    /// Close the connection
    pub async fn close(&mut self) -> Result<()> {
        if let Some(mut transport) = self.transport.take() {
            transport.close().await?;
        }
        Ok(())
    }

    /// Receive and handle incoming messages
    pub async fn poll(&mut self) -> Result<()> {
        let transport = self.transport.as_mut()
            .ok_or_else(|| McpError::connection("Not connected"))?;

        if let Some(message) = transport.receive().await? {
            match message {
                Message::Response(response) => {
                    // Find and complete pending request
                    let id = match &response.id {
                        RequestId::String(s) => s.clone(),
                        RequestId::Number(n) => n.to_string(),
                    };

                    let mut pending = self.pending_requests.write().await;
                    if let Some(req) = pending.remove(&id) {
                        let _ = req.sender.send(response);
                    }
                }
                Message::Notification(notification) => {
                    if let Some(handler) = &self.notification_handler {
                        handler(notification);
                    }
                }
                Message::Request(_) => {
                    // Clients typically don't receive requests
                    tracing::warn!("Received unexpected request from server");
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_config_default() {
        let config = ClientConfig::default();
        assert_eq!(config.name, "sentient-mcp-client");
        assert_eq!(config.timeout, 30);
    }

    #[test]
    fn test_client_config_tcp() {
        let config = ClientConfig::tcp("localhost", 8080);
        assert!(matches!(config.transport, TransportConfig::Tcp { .. }));
    }
}
