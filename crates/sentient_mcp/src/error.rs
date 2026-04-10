//! MCP Error Types

use std::fmt;

/// MCP Result type
pub type Result<T> = std::result::Result<T, McpError>;

/// MCP Error type
#[derive(Debug)]
pub enum McpError {
    /// Protocol error (JSON-RPC)
    Protocol(crate::protocol::Error),
    /// Transport error
    Transport(String),
    /// Serialization error
    Serialization(serde_json::Error),
    /// IO error
    Io(std::io::Error),
    /// Timeout error
    Timeout(String),
    /// Connection error
    Connection(String),
    /// Tool execution error
    ToolExecution(String),
    /// Resource not found
    ResourceNotFound(String),
    /// Invalid parameters
    InvalidParams(String),
    /// Not implemented
    NotImplemented(String),
    /// Internal error
    Internal(String),
    /// Cancelled
    Cancelled,
}

impl fmt::Display for McpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Protocol(e) => write!(f, "Protocol error: {}", e),
            Self::Transport(msg) => write!(f, "Transport error: {}", msg),
            Self::Serialization(e) => write!(f, "Serialization error: {}", e),
            Self::Io(e) => write!(f, "IO error: {}", e),
            Self::Timeout(msg) => write!(f, "Timeout: {}", msg),
            Self::Connection(msg) => write!(f, "Connection error: {}", msg),
            Self::ToolExecution(msg) => write!(f, "Tool execution error: {}", msg),
            Self::ResourceNotFound(uri) => write!(f, "Resource not found: {}", uri),
            Self::InvalidParams(msg) => write!(f, "Invalid parameters: {}", msg),
            Self::NotImplemented(msg) => write!(f, "Not implemented: {}", msg),
            Self::Internal(msg) => write!(f, "Internal error: {}", msg),
            Self::Cancelled => write!(f, "Operation cancelled"),
        }
    }
}

impl std::error::Error for McpError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Protocol(e) => Some(e),
            Self::Serialization(e) => Some(e),
            Self::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<serde_json::Error> for McpError {
    fn from(e: serde_json::Error) -> Self {
        Self::Serialization(e)
    }
}

impl From<std::io::Error> for McpError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<crate::protocol::Error> for McpError {
    fn from(e: crate::protocol::Error) -> Self {
        Self::Protocol(e)
    }
}

impl From<tokio::time::error::Elapsed> for McpError {
    fn from(_: tokio::time::error::Elapsed) -> Self {
        Self::Timeout("Operation timed out".to_string())
    }
}

#[cfg(feature = "websocket")]
impl From<tokio_tungstenite::tungstenite::Error> for McpError {
    fn from(e: tokio_tungstenite::tungstenite::Error) -> Self {
        Self::Transport(e.to_string())
    }
}

impl McpError {
    /// Create a protocol error
    pub fn protocol(code: i64, message: impl Into<String>) -> Self {
        Self::Protocol(crate::protocol::Error {
            code,
            message: message.into(),
            data: None,
        })
    }

    /// Create a transport error
    pub fn transport(msg: impl Into<String>) -> Self {
        Self::Transport(msg.into())
    }

    /// Create a timeout error
    pub fn timeout(msg: impl Into<String>) -> Self {
        Self::Timeout(msg.into())
    }

    /// Create a connection error
    pub fn connection(msg: impl Into<String>) -> Self {
        Self::Connection(msg.into())
    }

    /// Create a tool execution error
    pub fn tool_execution(msg: impl Into<String>) -> Self {
        Self::ToolExecution(msg.into())
    }

    /// Create a resource not found error
    pub fn resource_not_found(uri: impl Into<String>) -> Self {
        Self::ResourceNotFound(uri.into())
    }

    /// Create an invalid params error
    pub fn invalid_params(msg: impl Into<String>) -> Self {
        Self::InvalidParams(msg.into())
    }

    /// Create a not implemented error
    pub fn not_implemented(msg: impl Into<String>) -> Self {
        Self::NotImplemented(msg.into())
    }

    /// Create an internal error
    pub fn internal(msg: impl Into<String>) -> Self {
        Self::Internal(msg.into())
    }

    /// Check if this is a protocol error
    pub fn is_protocol(&self) -> bool {
        matches!(self, Self::Protocol(_))
    }

    /// Check if this is a transport error
    pub fn is_transport(&self) -> bool {
        matches!(self, Self::Transport(_))
    }

    /// Check if this is a timeout error
    pub fn is_timeout(&self) -> bool {
        matches!(self, Self::Timeout(_))
    }

    /// Convert to JSON-RPC error
    pub fn to_jsonrpc_error(&self) -> crate::protocol::Error {
        match self {
            Self::Protocol(e) => e.clone(),
            Self::InvalidParams(msg) => crate::protocol::Error::invalid_params(msg),
            Self::ResourceNotFound(uri) => {
                crate::protocol::Error::server_error(-32001, format!("Resource not found: {}", uri))
            }
            Self::NotImplemented(msg) => {
                crate::protocol::Error::server_error(-32002, format!("Not implemented: {}", msg))
            }
            Self::Timeout(msg) => {
                crate::protocol::Error::server_error(-32003, format!("Timeout: {}", msg))
            }
            Self::Cancelled => {
                crate::protocol::Error::server_error(-32004, "Operation cancelled")
            }
            _ => crate::protocol::Error::internal_error(self.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = McpError::transport("connection failed");
        assert!(err.to_string().contains("connection failed"));
    }

    #[test]
    fn test_error_from_io() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let mcp_err: McpError = io_err.into();
        assert!(matches!(mcp_err, McpError::Io(_)));
    }

    #[test]
    fn test_error_to_jsonrpc() {
        let err = McpError::invalid_params("missing field");
        let jsonrpc = err.to_jsonrpc_error();
        assert_eq!(jsonrpc.code, -32602);
    }
}
