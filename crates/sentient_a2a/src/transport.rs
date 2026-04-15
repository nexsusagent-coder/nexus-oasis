//! A2A Transport layer

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::{Message, A2AError, A2AResult};

/// Transport trait
#[async_trait]
pub trait Transport: Send + Sync {
    /// Send a message
    async fn send(&self, message: &Message) -> A2AResult<()>;
    
    /// Receive a message (blocking)
    async fn receive(&self) -> A2AResult<Message>;
    
    /// Check if connected
    fn is_connected(&self) -> bool;
    
    /// Close connection
    async fn close(&self) -> A2AResult<()>;
}

/// Transport configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransportConfig {
    /// Transport type
    pub transport_type: TransportType,
    /// Timeout in seconds
    pub timeout: u64,
    /// Max retries
    pub max_retries: u32,
    /// Buffer size
    pub buffer_size: usize,
}

impl Default for TransportConfig {
    fn default() -> Self {
        Self {
            transport_type: TransportType::Http,
            timeout: 30,
            max_retries: 3,
            buffer_size: 1000,
        }
    }
}

/// Transport types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransportType {
    Http,
    WebSocket,
    Grpc,
    InProcess,
}

/// HTTP Transport
pub struct HttpTransport {
    client: reqwest::Client,
    base_url: String,
    config: TransportConfig,
}

impl HttpTransport {
    pub fn new(base_url: &str, config: TransportConfig) -> Self {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeout))
            .build()
            .unwrap_or_default();

        Self {
            client,
            base_url: base_url.to_string(),
            config,
        }
    }

    pub fn with_default_config(base_url: &str) -> Self {
        Self::new(base_url, TransportConfig::default())
    }
}

#[async_trait]
impl Transport for HttpTransport {
    async fn send(&self, message: &Message) -> A2AResult<()> {
        let url = format!("{}/message", self.base_url);
        
        for attempt in 0..self.config.max_retries {
            match self.client
                .post(&url)
                .json(message)
                .send()
                .await
            {
                Ok(response) if response.status().is_success() => {
                    return Ok(());
                }
                Ok(response) => {
                    let status = response.status();
                    if attempt == self.config.max_retries - 1 {
                        return Err(A2AError::TransportError(
                            format!("HTTP error: {}", status)
                        ));
                    }
                }
                Err(e) => {
                    if attempt == self.config.max_retries - 1 {
                        return Err(A2AError::TransportError(e.to_string()));
                    }
                }
            }
            
            tokio::time::sleep(std::time::Duration::from_millis(100 * (attempt + 1) as u64)).await;
        }

        Err(A2AError::TransportError("Max retries exceeded".to_string()))
    }

    async fn receive(&self) -> A2AResult<Message> {
        // HTTP is request/response, not push-based
        // This would typically be used with long-polling
        let url = format!("{}/message/poll", self.base_url);
        
        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| A2AError::TransportError(e.to_string()))?;

        if response.status().is_success() {
            response
                .json::<Message>()
                .await
                .map_err(|e| A2AError::TransportError(e.to_string()))
        } else {
            Err(A2AError::TransportError("No message available".to_string()))
        }
    }

    fn is_connected(&self) -> bool {
        true // HTTP is stateless
    }

    async fn close(&self) -> A2AResult<()> {
        Ok(()) // HTTP doesn't need explicit close
    }
}

/// WebSocket Transport
pub struct WebSocketTransport {
    url: String,
    config: TransportConfig,
    connected: std::sync::Arc<std::sync::atomic::AtomicBool>,
}

impl WebSocketTransport {
    pub fn new(url: &str, config: TransportConfig) -> Self {
        Self {
            url: url.to_string(),
            config,
            connected: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
        }
    }
}

#[async_trait]
impl Transport for WebSocketTransport {
    async fn send(&self, message: &Message) -> A2AResult<()> {
        // WebSocket implementation would go here
        // For now, just log
        log::debug!("WebSocket send: {:?}", message.id);
        Ok(())
    }

    async fn receive(&self) -> A2AResult<Message> {
        // WebSocket implementation would go here
        Err(A2AError::TransportError("Not implemented".to_string()))
    }

    fn is_connected(&self) -> bool {
        self.connected.load(std::sync::atomic::Ordering::Relaxed)
    }

    async fn close(&self) -> A2AResult<()> {
        self.connected.store(false, std::sync::atomic::Ordering::Relaxed);
        Ok(())
    }
}

/// In-process transport (for testing)
pub struct InProcessTransport {
    messages: std::sync::Arc<tokio::sync::Mutex<Vec<Message>>>,
}

impl InProcessTransport {
    pub fn new() -> Self {
        Self {
            messages: std::sync::Arc::new(tokio::sync::Mutex::new(Vec::new())),
        }
    }
}

impl Default for InProcessTransport {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Transport for InProcessTransport {
    async fn send(&self, message: &Message) -> A2AResult<()> {
        let mut messages = self.messages.lock().await;
        messages.push(message.clone());
        Ok(())
    }

    async fn receive(&self) -> A2AResult<Message> {
        let mut messages = self.messages.lock().await;
        messages
            .pop()
            .ok_or_else(|| A2AError::TransportError("No messages".to_string()))
    }

    fn is_connected(&self) -> bool {
        true
    }

    async fn close(&self) -> A2AResult<()> {
        Ok(())
    }
}
