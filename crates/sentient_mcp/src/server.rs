//! MCP Server Implementation
//!
//! Expose tools, resources, and prompts to MCP clients.

use crate::protocol::{Message, Request, Response, Notification, Error};
use crate::types::*;
use crate::tool::*;
use crate::resource::*;
use crate::prompt::*;
use crate::transport::{Transport, TransportConfig};
use crate::{McpError, Result};

/// MCP Server configuration
#[derive(Debug, Clone)]
pub struct ServerConfig {
    /// Server name
    pub name: String,
    /// Server version
    pub version: String,
    /// Protocol version
    pub protocol_version: String,
    /// Server capabilities
    pub capabilities: ServerCapabilities,
    /// Server instructions
    pub instructions: Option<String>,
    /// Transport configuration
    pub transport: TransportConfig,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            name: "sentient-mcp-server".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            protocol_version: crate::MCP_VERSION.to_string(),
            capabilities: ServerCapabilities {
                tools: Some(ToolsCapability { list_changed: true }),
                resources: Some(ResourcesCapability { 
                    list_changed: true, 
                    subscribe: false 
                }),
                prompts: Some(PromptsCapability { list_changed: true }),
                ..Default::default()
            },
            instructions: Some("SENTIENT MCP Server - Exposes tools, resources, and prompts for AI assistants.".to_string()),
            transport: TransportConfig::Stdio,
        }
    }
}

/// MCP Server
pub struct Server {
    config: ServerConfig,
    tools: ToolRegistry,
    resources: ResourceManager,
    prompts: PromptManager,
    transport: Option<Box<dyn Transport>>,
    running: bool,
}

impl Server {
    /// Create a new MCP server
    pub fn new(config: ServerConfig) -> Self {
        Self {
            config,
            tools: ToolRegistry::default(),
            resources: ResourceManager::default(),
            prompts: PromptManager::default(),
            transport: None,
            running: false,
        }
    }

    /// Create server with default config
    pub fn default_server() -> Self {
        Self::new(ServerConfig::default())
    }

    /// Register a tool
    pub fn register_tool<T: ToolExecutor + 'static>(&mut self, tool: T) -> &mut Self {
        self.tools.register(tool);
        self
    }

    /// Register a resource provider
    pub fn register_resource(&mut self, scheme: &str, provider: Box<dyn ResourceProvider>) -> &mut Self {
        self.resources.register(scheme, provider);
        self
    }

    /// Register a prompt
    pub fn register_prompt<P: PromptHandler + 'static>(&mut self, prompt: P) -> &mut Self {
        self.prompts.register(prompt);
        self
    }

    /// Get server info
    pub fn server_info(&self) -> ServerInfo {
        ServerInfo {
            name: self.config.name.clone(),
            version: self.config.version.clone(),
            protocol_version: self.config.protocol_version.clone(),
            capabilities: self.config.capabilities.clone(),
            instructions: self.config.instructions.clone(),
        }
    }

    /// Handle an incoming request
    async fn handle_request(&self, request: Request) -> Result<Response> {
        let result = match request.method.as_str() {
            // Lifecycle methods
            "initialize" => self.handle_initialize(request.params).await?,
            
            // Tool methods
            "tools/list" => self.handle_list_tools(request.params).await?,
            "tools/call" => self.handle_call_tool(request.params).await?,
            
            // Resource methods
            "resources/list" => self.handle_list_resources(request.params).await?,
            "resources/read" => self.handle_read_resource(request.params).await?,
            "resources/templates/list" => self.handle_list_templates(request.params).await?,
            
            // Prompt methods
            "prompts/list" => self.handle_list_prompts(request.params).await?,
            "prompts/get" => self.handle_get_prompt(request.params).await?,
            
            // Logging
            "logging/setLevel" => {
                // TODO: Implement logging level setting
                serde_json::json!({})
            }
            
            // Completion
            "completion/complete" => self.handle_complete(request.params).await?,
            
            // Ping
            "ping" => serde_json::json!({}),
            
            // Unknown method
            _ => return Ok(Response::error(request.id, Error::method_not_found(&request.method))),
        };

        Ok(Response::success(request.id, result))
    }

    /// Handle initialize request
    async fn handle_initialize(&self, params: Option<serde_json::Value>) -> Result<serde_json::Value> {
        let _init_request: InitializeRequest = if let Some(p) = params {
            serde_json::from_value(p)?
        } else {
            return Err(McpError::invalid_params("Missing initialize params"));
        };

        Ok(serde_json::to_value(InitializeResult {
            protocol_version: self.config.protocol_version.clone(),
            capabilities: self.config.capabilities.clone(),
            server_info: self.server_info(),
            instructions: self.config.instructions.clone(),
        })?)
    }

    /// Handle list tools request
    async fn handle_list_tools(&self, _params: Option<serde_json::Value>) -> Result<serde_json::Value> {
        Ok(serde_json::to_value(ListToolsResult {
            tools: self.tools.list(),
            next_cursor: None,
        })?)
    }

    /// Handle call tool request
    async fn handle_call_tool(&self, params: Option<serde_json::Value>) -> Result<serde_json::Value> {
        let request: CallToolRequest = if let Some(p) = params {
            serde_json::from_value(p)?
        } else {
            return Err(McpError::invalid_params("Missing tool call params"));
        };

        let call = ToolCall {
            name: request.name,
            arguments: request.arguments,
        };

        let result = self.tools.execute(call).await?;
        Ok(serde_json::to_value(CallToolResult::from(result))?)
    }

    /// Handle list resources request
    async fn handle_list_resources(&self, _params: Option<serde_json::Value>) -> Result<serde_json::Value> {
        Ok(serde_json::to_value(ListResourcesResult {
            resources: self.resources.list_all().await?,
            next_cursor: None,
        })?)
    }

    /// Handle read resource request
    async fn handle_read_resource(&self, params: Option<serde_json::Value>) -> Result<serde_json::Value> {
        let request: ReadResourceRequest = if let Some(p) = params {
            serde_json::from_value(p)?
        } else {
            return Err(McpError::invalid_params("Missing resource read params"));
        };

        let contents = self.resources.read(&request.uri).await?;
        Ok(serde_json::to_value(ReadResourceResult {
            contents: vec![contents],
        })?)
    }

    /// Handle list templates request
    async fn handle_list_templates(&self, _params: Option<serde_json::Value>) -> Result<serde_json::Value> {
        // TODO: Implement resource templates
        Ok(serde_json::json!({ "resourceTemplates": [] }))
    }

    /// Handle list prompts request
    async fn handle_list_prompts(&self, _params: Option<serde_json::Value>) -> Result<serde_json::Value> {
        Ok(serde_json::to_value(ListPromptsResult {
            prompts: self.prompts.list(),
            next_cursor: None,
        })?)
    }

    /// Handle get prompt request
    async fn handle_get_prompt(&self, params: Option<serde_json::Value>) -> Result<serde_json::Value> {
        let request: GetPromptRequest = if let Some(p) = params {
            serde_json::from_value(p)?
        } else {
            return Err(McpError::invalid_params("Missing prompt get params"));
        };

        let result = self.prompts.render(&request.name, &request.arguments)?;
        Ok(serde_json::to_value(result)?)
    }

    /// Handle completion request
    async fn handle_complete(&self, _params: Option<serde_json::Value>) -> Result<serde_json::Value> {
        // TODO: Implement completion
        Ok(serde_json::to_value(CompleteResult {
            completion: Completion {
                values: vec![],
                total: Some(0),
                has_more: Some(false),
            },
        })?)
    }

    /// Handle notification
    async fn handle_notification(&self, notification: Notification) {
        match notification.method.as_str() {
            "notifications/initialized" => {
                tracing::info!("Client initialized");
            }
            "notifications/cancelled" => {
                tracing::info!("Request cancelled by client");
            }
            _ => {
                tracing::debug!("Unknown notification: {}", notification.method);
            }
        }
    }

    /// Run the server
    pub async fn run(&mut self) -> Result<()> {
        // Initialize transport
        #[cfg(feature = "stdio")]
        if matches!(self.config.transport, TransportConfig::Stdio) {
            self.transport = Some(Box::new(crate::transport::StdioTransport::from_process()));
        }

        self.running = true;

        while self.running {
            // Receive message
            let message = {
                let transport = self.transport.as_mut()
                    .ok_or_else(|| McpError::internal("Transport not initialized"))?;
                transport.receive().await
            };

            match message {
                Ok(Some(msg)) => {
                    match msg {
                        Message::Request(request) => {
                            let response = self.handle_request(request).await?;
                            let transport = self.transport.as_mut()
                                .ok_or_else(|| McpError::internal("Transport not initialized"))?;
                            transport.send(Message::Response(response)).await?;
                        }
                        Message::Notification(notification) => {
                            self.handle_notification(notification).await;
                        }
                        Message::Response(_) => {
                            // Servers don't typically receive responses
                            tracing::warn!("Received unexpected response from client");
                        }
                    }
                }
                Ok(None) => {
                    // Connection closed
                    tracing::info!("Client disconnected");
                    break;
                }
                Err(e) => {
                    tracing::error!("Error receiving message: {}", e);
                    break;
                }
            }
        }

        Ok(())
    }

    /// Stop the server
    pub fn stop(&mut self) {
        self.running = false;
    }

    /// Get tool count
    pub fn tool_count(&self) -> usize {
        self.tools.len()
    }

    /// Get prompt count
    pub fn prompt_count(&self) -> usize {
        self.prompts.list().len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_config_default() {
        let config = ServerConfig::default();
        assert_eq!(config.name, "sentient-mcp-server");
        assert!(config.capabilities.tools.is_some());
    }

    #[test]
    fn test_server_creation() {
        let server = Server::default_server();
        assert!(server.tools.len() >= 2); // Echo and CurrentTime
    }

    #[test]
    fn test_server_info() {
        let server = Server::default_server();
        let info = server.server_info();
        assert_eq!(info.name, "sentient-mcp-server");
    }
}
