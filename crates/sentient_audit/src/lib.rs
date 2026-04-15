//! ─── SENTIENT AUDIT SYSTEM ───
//!
//! Comprehensive audit logging:
//! - Event tracking
//! - Compliance logging
//! - Retention management
//! - Search and query

pub mod storage;

pub use storage::{
    AuditEntry, AuditStorage, AuditStorageConfig, AuditQuery, AuditQueryResult,
    Actor, ActorType, Resource, Outcome, RetentionCategory,
    StorageBackend, AuditStats,
};
