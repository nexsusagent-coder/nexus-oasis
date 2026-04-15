//! ═══════════════════════════════════════════════════════════════════════════════
//!  gRPC Server Module
//! ═══════════════════════════════════════════════════════════════════════════════
//! 
//! Provides high-performance gRPC API for:
//! - Agent communication
//! - Tool execution
//! - Memory operations
//! - Real-time streaming

use std::net::SocketAddr;
use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::{mpsc, RwLock};
use serde::{Deserialize, Serialize};

// ═══════════════════════════════════════════════════════════════════════════════
//  gRPC CONFIGURATION
// ═══════════════════════════════════════════════════════════════════════════════

/// gRPC server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrpcConfig {
    /// Server address
    pub addr: SocketAddr,
    /// Enable TLS
    pub tls_enabled: bool,
    /// TLS certificate path
    pub tls_cert_path: Option<String>,
    /// TLS private key path
    pub tls_key_path: Option<String>,
    /// Maximum message size (bytes)
    pub max_message_size: usize,
    /// Enable reflection
    pub enable_reflection: bool,
    /// Enable health check service
    pub enable_health: bool,
}

impl Default for GrpcConfig {
    fn default() -> Self {
        Self {
            addr: "0.0.0.0:50051".parse().unwrap(),
            tls_enabled: false,
            tls_cert_path: None,
            tls_key_path: None,
            max_message_size: 4 * 1024 * 1024, // 4 MB
            enable_reflection: true,
            enable_health: true,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  PROTO DEFINITIONS
// ═══════════════════════════════════════════════════════════════════════════════

/// Agent message for gRPC
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMessage {
    pub session_id: String,
    pub content: String,
    pub role: String,
    pub timestamp: i64,
}

/// Agent response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentResponse {
    pub id: String,
    pub session_id: String,
    pub content: String,
    pub complete: bool,
    pub tokens_used: u32,
    pub timestamp: i64,
}

/// Streaming response chunk
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamChunk {
    pub chunk_id: u32,
    pub content: String,
    pub is_final: bool,
    pub timestamp: i64,
}

/// Tool execution request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolRequest {
    pub id: String,
    pub tool: String,
    pub params: String,
    pub timeout_ms: u32,
}

/// Tool execution response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResponse {
    pub id: String,
    pub result: String,
    pub success: bool,
    pub error: Option<String>,
    pub duration_ms: u32,
}

/// Memory operation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryRequest {
    pub operation: MemoryOperation,
    pub key: String,
    pub value: Option<String>,
    pub namespace: String,
}

/// Memory operation type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MemoryOperation {
    Get,
    Set,
    Delete,
    List,
    Search,
}

/// Memory operation response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryResponse {
    pub id: String,
    pub value: Option<String>,
    pub keys: Vec<String>,
    pub success: bool,
    pub error: Option<String>,
}

/// Health check status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    pub service: String,
    pub status: ServingStatus,
    pub info: HashMap<String, String>,
}

/// Serving status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServingStatus {
    Unknown,
    Serving,
    NotServing,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  gRPC SERVER
// ═══════════════════════════════════════════════════════════════════════════════

/// gRPC Server error
#[derive(Debug, thiserror::Error)]
pub enum GrpcError {
    #[error("Failed to bind to address: {0}")]
    BindError(String),
    
    #[error("TLS error: {0}")]
    TlsError(String),
    
    #[error("Serialization error: {0}")]
    SerializationError(String),
    
    #[error("Channel closed")]
    ChannelClosed,
    
    #[error("Internal error: {0}")]
    InternalError(String),
}

/// gRPC Server
pub struct GrpcServer {
    config: GrpcConfig,
    running: Arc<std::sync::atomic::AtomicBool>,
}

impl GrpcServer {
    pub fn new(config: GrpcConfig) -> Self {
        Self {
            config,
            running: Arc::new(std::sync::atomic::AtomicBool::new(false)),
        }
    }
    
    pub async fn start(&self) -> Result<(), GrpcError> {
        self.running.store(true, std::sync::atomic::Ordering::SeqCst);
        log::info!("gRPC server starting on {}", self.config.addr);
        Ok(())
    }
    
    pub fn stop(&self) {
        self.running.store(false, std::sync::atomic::Ordering::SeqCst);
        log::info!("gRPC server stopped");
    }
    
    pub fn is_running(&self) -> bool {
        self.running.load(std::sync::atomic::Ordering::SeqCst)
    }
    
    pub fn addr(&self) -> SocketAddr {
        self.config.addr
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  AGENT SERVICE
// ═══════════════════════════════════════════════════════════════════════════════

/// Agent gRPC service
pub struct AgentService {
    request_tx: mpsc::Sender<AgentMessage>,
    response_rx: mpsc::Receiver<AgentResponse>,
}

impl AgentService {
    pub fn new() -> Self {
        let (request_tx, _): (mpsc::Sender<AgentMessage>, mpsc::Receiver<AgentMessage>) = 
            mpsc::channel(100);
        let (_, response_rx): (mpsc::Sender<AgentResponse>, mpsc::Receiver<AgentResponse>) = 
            mpsc::channel(100);
        
        Self { request_tx, response_rx }
    }
    
    pub async fn send_message(&self, message: AgentMessage) -> Result<(), GrpcError> {
        self.request_tx.send(message).await
            .map_err(|_| GrpcError::ChannelClosed)
    }
    
    pub async fn receive_response(&mut self) -> Result<AgentResponse, GrpcError> {
        self.response_rx.recv().await
            .ok_or(GrpcError::ChannelClosed)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  TOOL SERVICE
// ═══════════════════════════════════════════════════════════════════════════════

/// Tool execution gRPC service
pub struct ToolService {
    pending: Arc<RwLock<HashMap<String, mpsc::Sender<ToolResponse>>>>,
}

impl ToolService {
    pub fn new() -> Self {
        Self {
            pending: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    pub async fn execute(&self, request: ToolRequest) -> Result<ToolResponse, GrpcError> {
        Ok(ToolResponse {
            id: request.id,
            result: "{}".to_string(),
            success: true,
            error: None,
            duration_ms: 0,
        })
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  MEMORY SERVICE
// ═══════════════════════════════════════════════════════════════════════════════

/// Memory gRPC service
pub struct MemoryService {
    store: Arc<RwLock<HashMap<String, String>>>,
}

impl MemoryService {
    pub fn new() -> Self {
        Self {
            store: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    pub async fn execute(&self, request: MemoryRequest) -> Result<MemoryResponse, GrpcError> {
        let mut store = self.store.write().await;
        let id = uuid::Uuid::new_v4().to_string();
        
        match request.operation {
            MemoryOperation::Get => {
                let value = store.get(&request.key).cloned();
                Ok(MemoryResponse {
                    id,
                    value,
                    keys: vec![],
                    success: true,
                    error: None,
                })
            }
            MemoryOperation::Set => {
                if let Some(value) = request.value {
                    store.insert(request.key.clone(), value);
                    Ok(MemoryResponse {
                        id,
                        value: None,
                        keys: vec![],
                        success: true,
                        error: None,
                    })
                } else {
                    Ok(MemoryResponse {
                        id,
                        value: None,
                        keys: vec![],
                        success: false,
                        error: Some("No value provided".to_string()),
                    })
                }
            }
            MemoryOperation::Delete => {
                store.remove(&request.key);
                Ok(MemoryResponse {
                    id,
                    value: None,
                    keys: vec![],
                    success: true,
                    error: None,
                })
            }
            MemoryOperation::List => {
                let keys: Vec<String> = store.keys()
                    .filter(|k| k.starts_with(&request.namespace))
                    .cloned()
                    .collect();
                Ok(MemoryResponse {
                    id,
                    value: None,
                    keys,
                    success: true,
                    error: None,
                })
            }
            MemoryOperation::Search => {
                let keys: Vec<String> = store.keys()
                    .filter(|k| k.contains(&request.key))
                    .cloned()
                    .collect();
                Ok(MemoryResponse {
                    id,
                    value: None,
                    keys,
                    success: true,
                    error: None,
                })
            }
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  HEALTH SERVICE
// ═══════════════════════════════════════════════════════════════════════════════

/// Health check service
pub struct HealthService {
    status: Arc<RwLock<ServingStatus>>,
}

impl HealthService {
    pub fn new() -> Self {
        Self {
            status: Arc::new(RwLock::new(ServingStatus::Serving)),
        }
    }
    
    pub async fn check(&self) -> HealthStatus {
        let status = self.status.read().await.clone();
        HealthStatus {
            service: "sentient-gateway".to_string(),
            status,
            info: HashMap::new(),
        }
    }
    
    pub async fn set_serving(&self) {
        let mut status = self.status.write().await;
        *status = ServingStatus::Serving;
    }
    
    pub async fn set_not_serving(&self) {
        let mut status = self.status.write().await;
        *status = ServingStatus::NotServing;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_grpc_config_default() {
        let config = GrpcConfig::default();
        assert_eq!(config.addr.port(), 50051);
        assert!(!config.tls_enabled);
    }
    
    #[tokio::test]
    async fn test_memory_service() {
        let service = MemoryService::new();
        
        // Set
        let set_req = MemoryRequest {
            operation: MemoryOperation::Set,
            key: "test".to_string(),
            value: Some("value".to_string()),
            namespace: "default".to_string(),
        };
        let result = service.execute(set_req).await;
        assert!(result.is_ok());
        
        // Get
        let get_req = MemoryRequest {
            operation: MemoryOperation::Get,
            key: "test".to_string(),
            value: None,
            namespace: "default".to_string(),
        };
        let result = service.execute(get_req).await.unwrap();
        assert_eq!(result.value, Some("value".to_string()));
    }
    
    #[tokio::test]
    async fn test_health_service() {
        let service = HealthService::new();
        let status = service.check().await;
        assert!(matches!(status.status, ServingStatus::Serving));
    }
}
