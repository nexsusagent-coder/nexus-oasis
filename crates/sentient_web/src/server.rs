//! Web server implementation

use crate::types::*;
use crate::{WebError, Result};
use axum::{
    routing::{get, post, put, delete},
    Router, Extension,
};
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
    compression::CompressionLayer,
};
use tracing::info;

/// Server state
#[derive(Debug)]
pub struct ServerState {
    /// Configuration
    pub config: ServerConfig,
    /// Server start time
    pub start_time: Instant,
}

/// Web server
pub struct WebServer {
    /// Configuration
    config: ServerConfig,
    /// Router
    router: Router,
    /// State
    state: Arc<ServerState>,
}

impl WebServer {
    /// Create new web server
    pub fn new(config: ServerConfig) -> Self {
        let state = Arc::new(ServerState {
            start_time: Instant::now(),
            config: config.clone(),
        });

        let router = Self::create_router(&config, state.clone());

        Self { config, router, state }
    }

    /// Create router
    fn create_router(config: &ServerConfig, state: Arc<ServerState>) -> Router {
        let mut router = Router::new()
            // Health check
            .route("/health", get(crate::routes::health))
            .route("/api/v1/status", get(crate::routes::status))

            // Auth routes
            .route("/api/v1/auth/login", post(crate::routes::auth_login))
            .route("/api/v1/auth/logout", post(crate::routes::auth_logout))
            .route("/api/v1/auth/refresh", post(crate::routes::auth_refresh))

            // User routes
            .route("/api/v1/users", get(crate::routes::users_list))
            .route("/api/v1/users/:id", get(crate::routes::users_get))
            .route("/api/v1/users/:id", put(crate::routes::users_update))
            .route("/api/v1/users/:id", delete(crate::routes::users_delete))

            // Agent routes
            .route("/api/v1/agents", get(crate::routes::agents_list))
            .route("/api/v1/agents", post(crate::routes::agents_create))
            .route("/api/v1/agents/:id", get(crate::routes::agents_get))
            .route("/api/v1/agents/:id/chat", post(crate::routes::agents_chat))
            .route("/api/v1/agents/:id/stream", post(crate::routes::agents_stream))

            // Skill routes
            .route("/api/v1/skills", get(crate::routes::skills_list))
            .route("/api/v1/skills/:id", get(crate::routes::skills_get))

            // WebSocket
            .route("/ws", get(crate::routes::websocket_handler))

            // Add state
            .layer(Extension(state));

        // Add CORS
        if config.cors {
            let cors = CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any);

            router = router.layer(cors);
        }

        // Add compression
        if config.compression {
            router = router.layer(CompressionLayer::new());
        }

        // Add tracing
        router = router.layer(TraceLayer::new_for_http());

        router
    }

    /// Run server
    pub async fn run(self) -> Result<()> {
        let addr: SocketAddr = self.config.listen_addr()
            .parse()
            .map_err(|e| WebError::Config(format!("Invalid address: {}", e)))?;

        info!("Starting server on {}", addr);

        let listener = tokio::net::TcpListener::bind(addr)
            .await
            .map_err(|e| WebError::Server(format!("Failed to bind: {}", e)))?;

        axum::serve(listener, self.router)
            .await
            .map_err(|e| WebError::Server(e.to_string()))?;

        Ok(())
    }

    /// Get server address
    pub fn addr(&self) -> String {
        self.config.listen_addr()
    }

    /// Get uptime
    pub fn uptime(&self) -> Duration {
        self.state.start_time.elapsed()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_creation() {
        let config = ServerConfig::new(8080);
        let server = WebServer::new(config);

        assert_eq!(server.addr(), "0.0.0.0:8080");
    }

    #[test]
    fn test_uptime() {
        let config = ServerConfig::new(8080);
        let server = WebServer::new(config);

        // Should be very small
        assert!(server.uptime().as_secs() < 1);
    }
}
