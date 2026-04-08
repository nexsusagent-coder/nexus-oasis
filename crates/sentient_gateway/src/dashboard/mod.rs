//! ─── DASHBOARD MODULE ───
//!
//! Modern, mobil uyumlu web dashboard:
//! - Canlı log akışı (WebSocket)
//! - Sistem metrikleri (CPU, RAM)
//! - Scout/Forge/Swarm aktiviteleri
//! - API maliyet takibi
//! - Ajan düşünceleri (Live Thoughts)

pub mod metrics;
pub mod handlers;
pub mod assets;

// Re-exports
pub use metrics::{SystemMetrics, MetricsCollector, HealthStatus};
pub use handlers::{
    DashboardState, DashboardConfig,
    Activity, ActivitySource, ActivityStatus,
    LogEntry, LogLevel,
    AgentThought,
    CreateTaskRequest, CreateTaskResponse,
    create_dashboard_router,
    BrowserSessionInfo,
};
pub use assets::DashboardAssets;
