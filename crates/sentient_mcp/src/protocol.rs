//! MCP Protocol - JSON-RPC 2.0 Message Types
//!
//! This module implements the Model Context Protocol specification,
//! which is based on JSON-RPC 2.0.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

/// JSON-RPC 2.0 Request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request {
    /// JSON-RPC version (always "2.0")
    pub jsonrpc: String,
    /// Request ID (string or number)
    pub id: RequestId,
    /// Method name
    pub method: String,
    /// Optional parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Value>,
}

impl Request {
    /// Create a new request
    pub fn new(method: impl Into<String>) -> Self {
        Self {
            jsonrpc: crate::JSONRPC_VERSION.to_string(),
            id: RequestId::String(Uuid::new_v4().to_string()),
            method: method.into(),
            params: None,
        }
    }

    /// Create a request with parameters
    pub fn with_params(method: impl Into<String>, params: Value) -> Self {
        Self {
            jsonrpc: crate::JSONRPC_VERSION.to_string(),
            id: RequestId::String(Uuid::new_v4().to_string()),
            method: method.into(),
            params: Some(params),
        }
    }

    /// Set the request ID
    pub fn with_id(mut self, id: impl Into<RequestId>) -> Self {
        self.id = id.into();
        self
    }
}

/// Request ID type (string or number)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RequestId {
    /// String ID
    String(String),
    /// Numeric ID
    Number(i64),
}

impl From<String> for RequestId {
    fn from(s: String) -> Self {
        RequestId::String(s)
    }
}

impl From<&str> for RequestId {
    fn from(s: &str) -> Self {
        RequestId::String(s.to_string())
    }
}

impl From<i64> for RequestId {
    fn from(n: i64) -> Self {
        RequestId::Number(n)
    }
}

/// JSON-RPC 2.0 Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    /// JSON-RPC version (always "2.0")
    pub jsonrpc: String,
    /// Request ID this response corresponds to
    pub id: RequestId,
    /// Result (if successful)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    /// Error (if failed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<Error>,
}

impl Response {
    /// Create a successful response
    pub fn success(id: RequestId, result: Value) -> Self {
        Self {
            jsonrpc: crate::JSONRPC_VERSION.to_string(),
            id,
            result: Some(result),
            error: None,
        }
    }

    /// Create an error response
    pub fn error(id: RequestId, error: Error) -> Self {
        Self {
            jsonrpc: crate::JSONRPC_VERSION.to_string(),
            id,
            result: None,
            error: Some(error),
        }
    }

    /// Check if this is a success response
    pub fn is_success(&self) -> bool {
        self.error.is_none() && self.result.is_some()
    }

    /// Check if this is an error response
    pub fn is_error(&self) -> bool {
        self.error.is_some()
    }
}

/// JSON-RPC 2.0 Error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Error {
    /// Error code
    pub code: i64,
    /// Error message
    pub message: String,
    /// Additional error data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

impl Error {
    /// Parse error (-32700)
    pub fn parse_error(message: impl Into<String>) -> Self {
        Self {
            code: -32700,
            message: message.into(),
            data: None,
        }
    }

    /// Invalid request (-32600)
    pub fn invalid_request(message: impl Into<String>) -> Self {
        Self {
            code: -32600,
            message: message.into(),
            data: None,
        }
    }

    /// Method not found (-32601)
    pub fn method_not_found(method: impl Into<String>) -> Self {
        Self {
            code: -32601,
            message: format!("Method not found: {}", method.into()),
            data: None,
        }
    }

    /// Invalid params (-32602)
    pub fn invalid_params(message: impl Into<String>) -> Self {
        Self {
            code: -32602,
            message: message.into(),
            data: None,
        }
    }

    /// Internal error (-32603)
    pub fn internal_error(message: impl Into<String>) -> Self {
        Self {
            code: -32603,
            message: message.into(),
            data: None,
        }
    }

    /// Server error (-32000 to -32099)
    pub fn server_error(code: i64, message: impl Into<String>) -> Self {
        Self {
            code: code.clamp(-32099, -32000),
            message: message.into(),
            data: None,
        }
    }

    /// Create error with data
    pub fn with_data(mut self, data: Value) -> Self {
        self.data = Some(data);
        self
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MCP Error {}({}): {}", self.code, 
            match self.code {
                -32700 => "Parse error",
                -32600 => "Invalid request",
                -32601 => "Method not found",
                -32602 => "Invalid params",
                -32603 => "Internal error",
                -32099..=-32000 => "Server error",
                _ => "Unknown error",
            },
            self.message
        )
    }
}

impl std::error::Error for Error {}

/// JSON-RPC 2.0 Notification (no response expected)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    /// JSON-RPC version (always "2.0")
    pub jsonrpc: String,
    /// Method name
    pub method: String,
    /// Optional parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Value>,
}

impl Notification {
    /// Create a new notification
    pub fn new(method: impl Into<String>) -> Self {
        Self {
            jsonrpc: crate::JSONRPC_VERSION.to_string(),
            method: method.into(),
            params: None,
        }
    }

    /// Create a notification with parameters
    pub fn with_params(method: impl Into<String>, params: Value) -> Self {
        Self {
            jsonrpc: crate::JSONRPC_VERSION.to_string(),
            method: method.into(),
            params: Some(params),
        }
    }
}

/// MCP Message type (union of Request, Response, Notification)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Message {
    /// Request message
    Request(Request),
    /// Response message
    Response(Response),
    /// Notification message
    Notification(Notification),
}

impl Message {
    /// Parse a message from JSON
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Serialize to JSON
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    /// Serialize to pretty JSON
    pub fn to_json_pretty(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_creation() {
        let req = Request::new("test/method");
        assert_eq!(req.jsonrpc, "2.0");
        assert_eq!(req.method, "test/method");
        assert!(req.params.is_none());
    }

    #[test]
    fn test_request_with_params() {
        let req = Request::with_params(
            "tools/call",
            serde_json::json!({"name": "echo"}),
        );
        assert_eq!(req.method, "tools/call");
        assert!(req.params.is_some());
    }

    #[test]
    fn test_response_success() {
        let res = Response::success(
            RequestId::String("test-id".to_string()),
            serde_json::json!({"result": "ok"}),
        );
        assert!(res.is_success());
        assert!(!res.is_error());
    }

    #[test]
    fn test_response_error() {
        let res = Response::error(
            RequestId::String("test-id".to_string()),
            Error::method_not_found("unknown"),
        );
        assert!(!res.is_success());
        assert!(res.is_error());
    }

    #[test]
    fn test_error_codes() {
        let e1 = Error::parse_error("test");
        assert_eq!(e1.code, -32700);

        let e2 = Error::method_not_found("test");
        assert_eq!(e2.code, -32601);

        let e3 = Error::internal_error("test");
        assert_eq!(e3.code, -32603);
    }

    #[test]
    fn test_message_serde() {
        let req = Request::new("test");
        let msg = Message::Request(req);
        let json = msg.to_json().expect("Should serialize");
        let parsed = Message::from_json(&json).expect("Should deserialize");
        assert!(matches!(parsed, Message::Request(_)));
    }
}
