//! SENTIENT OS Disaster Recovery System
//!
//! Comprehensive disaster recovery with:
//! - Failover management
//! - Health monitoring
//! - Recovery orchestration
//! - Multi-region support
//! - Automated recovery

// Suppress warnings
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

pub mod failover;
pub mod health;
pub mod recovery;
pub mod region;
pub mod plan;
pub mod error;

pub use failover::*;
pub use health::*;
pub use recovery::*;
pub use region::*;
pub use plan::*;
pub use error::*;

/// Disaster recovery version
pub const VERSION: &str = "4.0.0";

/// Default health check interval in seconds
pub const DEFAULT_HEALTH_CHECK_INTERVAL_SECS: u64 = 30;

/// Default recovery timeout in seconds
pub const DEFAULT_RECOVERY_TIMEOUT_SECS: u64 = 300;

/// Default RTO (Recovery Time Objective) in seconds
pub const DEFAULT_RTO_SECS: u64 = 14400; // 4 hours

/// Default RPO (Recovery Point Objective) in seconds  
pub const DEFAULT_RPO_SECS: u64 = 3600; // 1 hour
