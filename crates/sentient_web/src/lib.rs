//! # Sentient Web
//!
//! Web server for SENTIENT OS - REST API, WebSocket, Dashboard.
//!
//! ## Features
//!
//! - **REST API**: Full REST API with authentication
//! - **WebSocket**: Real-time bidirectional communication
//! - **Dashboard**: Built-in web dashboard
//! - **Authentication**: JWT-based authentication
//! - **Rate Limiting**: Request rate limiting
//! - **CORS**: Cross-origin resource sharing support
//!
//! ## Example
//!
//! ```rust
//! use sentient_web::{WebServer, ServerConfig};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let config = ServerConfig::new(8080)
//!     .with_auth("jwt-secret")
//!     .with_cors(vec!["http://localhost:3000".to_string()])
//!     .with_rate_limit(100);
//!
//! let server = WebServer::new(config);
//! server.run().await?;
//! # Ok(())
//! # }
//! ```

pub mod error;
pub mod types;
pub mod server;
pub mod routes;
pub mod auth;
pub mod middleware;

pub use error::{WebError, Result};
pub use types::*;
pub use server::WebServer;
pub use auth::{AuthService, JwtConfig};

/// Web server version
pub const WEB_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!WEB_VERSION.is_empty());
    }
}
