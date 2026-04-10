//! Disaster recovery error types

use thiserror::Error;

#[derive(Debug, Error)]
pub enum DRError {
    #[error("Failover failed: {0}")]
    FailoverFailed(String),

    #[error("Recovery failed: {0}")]
    RecoveryFailed(String),

    #[error("Health check failed: {0}")]
    HealthCheckFailed(String),

    #[error("Region unavailable: {0}")]
    RegionUnavailable(String),

    #[error("No healthy regions available")]
    NoHealthyRegions,

    #[error("Recovery plan not found: {0}")]
    PlanNotFound(String),

    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    #[error("Timeout exceeded: {0}")]
    Timeout(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(String),
}

pub type Result<T> = std::result::Result<T, DRError>;
