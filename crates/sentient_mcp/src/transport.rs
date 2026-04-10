//! MCP Transport Layer
//!
//! Transports handle the underlying communication between MCP clients and servers.
//! Supported transports: stdio, TCP, WebSocket, SSE

use async_trait::async_trait;
use std::pin::Pin;
use tokio::io::{AsyncBufRead, AsyncWrite};
use crate::protocol::Message;
use crate::Result;

/// Transport trait for MCP communication
#[async_trait]
pub trait Transport: Send {
    /// Send a message
    async fn send(&mut self, message: Message) -> Result<()>;
    
    /// Receive a message
    async fn receive(&mut self) -> Result<Option<Message>>;
    
    /// Close the transport
    async fn close(&mut self) -> Result<()>;
}

/// Transport configuration
#[derive(Debug, Clone)]
pub enum TransportConfig {
    /// Standard I/O transport
    Stdio,
    /// TCP transport
    Tcp {
        /// Host to connect to
        host: String,
        /// Port
        port: u16,
    },
    /// WebSocket transport
    WebSocket {
        /// WebSocket URL
        url: String,
    },
    /// Server-Sent Events transport
    Sse {
        /// SSE URL
        url: String,
    },
}

impl Default for TransportConfig {
    fn default() -> Self {
        Self::Stdio
    }
}

/// Stdio transport for process communication
pub struct StdioTransport {
    stdin: Pin<Box<dyn AsyncWrite + Send>>,
    stdout: Pin<Box<dyn AsyncBufRead + Send>>,
    buffer: String,
}

impl StdioTransport {
    /// Create a new stdio transport
    pub fn new<R: AsyncBufRead + Send + 'static, W: AsyncWrite + Send + 'static>(
        stdin: W,
        stdout: R,
    ) -> Self {
        Self {
            stdin: Box::pin(stdin),
            stdout: Box::pin(stdout),
            buffer: String::new(),
        }
    }

    /// Create using process stdio
    pub fn from_process() -> Self {
        Self::new(
            tokio::io::stdout(),
            tokio::io::BufReader::new(tokio::io::stdin()),
        )
    }
}

#[async_trait]
impl Transport for StdioTransport {
    async fn send(&mut self, message: Message) -> Result<()> {
        use tokio::io::AsyncWriteExt;
        
        let json = message.to_json()?;
        self.stdin.write_all(json.as_bytes()).await?;
        self.stdin.write_all(b"\n").await?;
        self.stdin.flush().await?;
        
        Ok(())
    }

    async fn receive(&mut self) -> Result<Option<Message>> {
        use tokio::io::AsyncBufReadExt;
        
        self.buffer.clear();
        let bytes_read = self.stdout.read_line(&mut self.buffer).await?;
        
        if bytes_read == 0 {
            return Ok(None);
        }
        
        let message = Message::from_json(self.buffer.trim())?;
        Ok(Some(message))
    }

    async fn close(&mut self) -> Result<()> {
        Ok(())
    }
}

/// TCP transport for network communication
#[cfg(feature = "tcp")]
pub struct TcpTransport {
    stream: tokio::net::TcpStream,
    buffer: Vec<u8>,
}

#[cfg(feature = "tcp")]
impl TcpTransport {
    /// Connect to a TCP server
    pub async fn connect(host: &str, port: u16) -> Result<Self> {
        let addr = format!("{}:{}", host, port);
        let stream = tokio::net::TcpStream::connect(&addr).await
            .map_err(|e| crate::McpError::connection(format!("Failed to connect to {}: {}", addr, e)))?;
        
        Ok(Self {
            stream,
            buffer: Vec::new(),
        })
    }

    /// Create from an existing TCP stream
    pub fn from_stream(stream: tokio::net::TcpStream) -> Self {
        Self {
            stream,
            buffer: Vec::new(),
        }
    }
}

#[cfg(feature = "tcp")]
#[async_trait]
impl Transport for TcpTransport {
    async fn send(&mut self, message: Message) -> Result<()> {
        use tokio::io::AsyncWriteExt;
        
        let json = message.to_json()?;
        let len = json.len() as u32;
        
        // Write length prefix (4 bytes, big-endian)
        self.stream.write_all(&len.to_be_bytes()).await?;
        // Write message
        self.stream.write_all(json.as_bytes()).await?;
        self.stream.flush().await?;
        
        Ok(())
    }

    async fn receive(&mut self) -> Result<Option<Message>> {
        use tokio::io::AsyncReadExt;
        
        // Read length prefix
        let mut len_buf = [0u8; 4];
        let bytes_read = self.stream.read_exact(&mut len_buf).await.ok();
        
        if bytes_read.is_none() {
            return Ok(None);
        }
        
        let len = u32::from_be_bytes(len_buf) as usize;
        
        // Read message
        self.buffer.resize(len, 0);
        self.stream.read_exact(&mut self.buffer).await?;
        
        let json = std::str::from_utf8(&self.buffer)
            .map_err(|e| crate::McpError::transport(format!("Invalid UTF-8: {}", e)))?;
        
        let message = Message::from_json(json)?;
        Ok(Some(message))
    }

    async fn close(&mut self) -> Result<()> {
        Ok(())
    }
}

/// WebSocket transport
#[cfg(feature = "websocket")]
pub struct WebSocketTransport {
    ws: tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>,
}

#[cfg(feature = "websocket")]
impl WebSocketTransport {
    /// Connect to a WebSocket server
    pub async fn connect(url: &str) -> Result<Self> {
        let (ws, _) = tokio_tungstenite::connect_async(url).await
            .map_err(|e| crate::McpError::connection(format!("WebSocket connect failed: {}", e)))?;
        
        Ok(Self { ws })
    }
}

#[cfg(feature = "websocket")]
#[async_trait]
impl Transport for WebSocketTransport {
    async fn send(&mut self, message: Message) -> Result<()> {
        use tokio_tungstenite::tungstenite::Message as WsMessage;
        
        let json = message.to_json()?;
        self.ws.send(WsMessage::Text(json)).await?;
        
        Ok(())
    }

    async fn receive(&mut self) -> Result<Option<Message>> {
        use tokio_tungstenite::tungstenite::Message as WsMessage;
        
        let msg = self.ws.next().await
            .transpose()
            .map_err(|e| crate::McpError::transport(format!("WebSocket error: {}", e)))?;
        
        match msg {
            Some(WsMessage::Text(text)) => {
                let message = Message::from_json(&text)?;
                Ok(Some(message))
            }
            Some(WsMessage::Binary(data)) => {
                let text = String::from_utf8(data)
                    .map_err(|e| crate::McpError::transport(format!("Invalid UTF-8: {}", e)))?;
                let message = Message::from_json(&text)?;
                Ok(Some(message))
            }
            Some(WsMessage::Close(_)) => Ok(None),
            None => Ok(None),
            _ => self.receive().await,
        }
    }

    async fn close(&mut self) -> Result<()> {
        use tokio_tungstenite::tungstenite::Message as WsMessage;
        self.ws.send(WsMessage::Close(None)).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transport_config_default() {
        let config = TransportConfig::default();
        assert!(matches!(config, TransportConfig::Stdio));
    }

    #[test]
    fn test_tcp_config() {
        let config = TransportConfig::Tcp {
            host: "localhost".to_string(),
            port: 8080,
        };
        
        if let TransportConfig::Tcp { host, port } = config {
            assert_eq!(host, "localhost");
            assert_eq!(port, 8080);
        }
    }
}
