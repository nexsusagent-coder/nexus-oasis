//! SENTIENT OS Backup and Restore System
//!
//! Comprehensive backup solution with:
//! - Local filesystem backup
//! - S3 cloud backup
//! - Incremental backups
//! - Encryption support
//! - Scheduled backups
//! - Point-in-time recovery

// Suppress warnings
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

pub mod backup;
pub mod restore;
pub mod scheduler;
pub mod storage;
pub mod crypto;
pub mod manifest;
pub mod error;

pub use backup::*;
pub use restore::*;
pub use scheduler::*;
pub use storage::*;
pub use crypto::*;
pub use manifest::*;
pub use error::*;

/// Backup system version
pub const VERSION: &str = "4.0.0";

/// Default backup interval in seconds (24 hours)
pub const DEFAULT_BACKUP_INTERVAL_SECS: u64 = 86400;

/// Default retention period in days (30 days)
pub const DEFAULT_RETENTION_DAYS: u64 = 30;

/// Maximum concurrent backup operations
pub const MAX_CONCURRENT_BACKUPS: usize = 3;
