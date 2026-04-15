//! ═══════════════════════════════════════════════════════════════════════════════
//!  SENTIENT A2A Protocol - Agent-to-Agent Communication
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//!  Sprint 4: Personal AI - Agent'ler arası iletişim protokolü
//!  
//!  Özellikler:
//!  - Agent discovery ve registration
//!  - Message passing (sync/async)
//!  - WebSocket ve HTTP transport
//!  - Capability-based routing

pub mod message;
pub mod agent;
pub mod registry;
pub mod transport;
pub mod error;
pub mod protocol;

pub use message::{Message, MessageType, MessageBuilder};
pub use agent::{Agent, AgentId, AgentCapability, AgentMetadata};
pub use registry::{AgentRegistry, RegistryConfig, RegistryStats};
pub use transport::{Transport, TransportConfig, HttpTransport, WebSocketTransport};
pub use error::{A2AError, A2AResult};
pub use protocol::{A2AProtocol, ProtocolConfig};
