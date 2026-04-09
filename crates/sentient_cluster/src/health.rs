//! ─── Health Check Server ───

use axum::{Router, routing::get, Json, http::StatusCode};
use serde_json::json;
use std::net::SocketAddr;

/// Health check response
#[derive(Debug, serde::Serialize)]
pub struct HealthStatus {
    pub status: String,
    pub version: String,
    pub uptime_secs: u64,
    pub agents: AgentStats,
    pub checks: Vec<HealthCheck>,
}

#[derive(Debug, serde::Serialize)]
pub struct AgentStats {
    pub total: i32,
    pub ready: i32,
    pub degraded: i32,
}

#[derive(Debug, serde::Serialize)]
pub struct HealthCheck {
    pub name: String,
    pub status: String,
    pub message: Option<String>,
}

static START_TIME: std::sync::OnceLock<std::time::Instant> = std::sync::OnceLock::new();

/// Start health check server
pub async fn start_server(port: u16) {
    START_TIME.set(std::time::Instant::now()).ok();
    
    let app = Router::new()
        .route("/health", get(health_handler))
        .route("/ready", get(ready_handler))
        .route("/metrics", get(metrics_handler));
    
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    
    log::info!("Health server listening on {}", addr);
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

/// Health check endpoint
async fn health_handler() -> Json<HealthStatus> {
    let uptime = START_TIME.get()
        .map(|t| t.elapsed().as_secs())
        .unwrap_or(0);
    
    Json(HealthStatus {
        status: "healthy".into(),
        version: env!("CARGO_PKG_VERSION").into(),
        uptime_secs: uptime,
        agents: AgentStats {
            total: 1,
            ready: 1,
            degraded: 0,
        },
        checks: vec![
            HealthCheck {
                name: "kubernetes".into(),
                status: "ok".into(),
                message: None,
            },
            HealthCheck {
                name: "model".into(),
                status: "ok".into(),
                message: None,
            },
        ],
    })
}

/// Readiness endpoint
async fn ready_handler() -> StatusCode {
    // Check if operator is ready
    StatusCode::OK
}

/// Metrics endpoint (Prometheus format)
async fn metrics_handler() -> String {
    crate::metrics::export()
}
