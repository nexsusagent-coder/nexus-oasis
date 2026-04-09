//! Audit Logging module
//!
//! Provides comprehensive audit logging for:
//! - Authentication events
//! - Authorization decisions
//! - Data access
//! - Configuration changes
//! - API calls

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Audit event types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AuditEvent {
    /// User authenticated successfully
    AuthSuccess {
        user_id: String,
        method: AuthMethod,
        ip_address: String,
        user_agent: Option<String>,
    },

    /// Authentication failed
    AuthFailed {
        username: String,
        method: AuthMethod,
        ip_address: String,
        reason: String,
    },

    /// User logged out
    Logout {
        user_id: String,
        session_id: String,
    },

    /// Access granted to resource
    AccessGranted {
        user_id: String,
        resource: String,
        action: String,
        role: String,
    },

    /// Access denied to resource
    AccessDenied {
        user_id: String,
        resource: String,
        action: String,
    },

    /// Data accessed
    DataAccess {
        user_id: String,
        table: String,
        operation: DataOperation,
        rows_affected: u64,
    },

    /// Configuration changed
    ConfigChanged {
        user_id: String,
        setting: String,
        old_value: Option<String>,
        new_value: Option<String>,
    },

    /// API call made
    ApiCall {
        user_id: Option<String>,
        endpoint: String,
        method: String,
        status_code: u16,
        duration_ms: u64,
    },

    /// Skill installed/updated/removed
    SkillChange {
        user_id: String,
        skill_name: String,
        change_type: SkillChangeType,
        version: Option<String>,
    },

    /// Channel configured
    ChannelConfig {
        user_id: String,
        channel: String,
        action: String,
        config: HashMap<String, serde_json::Value>,
    },

    /// Agent created/updated/deleted
    AgentChange {
        user_id: String,
        agent_id: String,
        change_type: AgentChangeType,
    },

    /// Security event
    SecurityEvent {
        event_type: SecurityEventType,
        severity: Severity,
        details: HashMap<String, String>,
    },

    /// System event
    SystemEvent {
        event_type: SystemEventType,
        details: HashMap<String, String>,
    },
}

/// Authentication method
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthMethod {
    Password,
    ApiKey,
    Sso { provider: String },
    Token,
    Certificate,
}

/// Data operation type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataOperation {
    Select,
    Insert,
    Update,
    Delete,
}

/// Skill change type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SkillChangeType {
    Installed,
    Updated,
    Removed,
    Enabled,
    Disabled,
}

/// Agent change type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AgentChangeType {
    Created,
    Updated,
    Deleted,
    Started,
    Stopped,
}

/// Security event type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityEventType {
    BruteForceAttempt,
    SuspiciousActivity,
    PrivilegeEscalation,
    DataExfiltration,
    MalwareDetected,
    VulnerabilityFound,
}

/// System event type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SystemEventType {
    Startup,
    Shutdown,
    ConfigurationReload,
    BackupCompleted,
    RestoreCompleted,
    Error,
    Warning,
}

/// Event severity
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Severity {
    Info,
    Warning,
    Error,
    Critical,
}

/// Audit log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    /// Unique ID for this entry
    pub id: Uuid,

    /// Timestamp of the event
    pub timestamp: DateTime<Utc>,

    /// The audit event
    pub event: AuditEvent,

    /// Additional metadata
    pub metadata: HashMap<String, String>,

    /// Tenant ID (for multi-tenant)
    pub tenant_id: Option<String>,

    /// Request ID for tracing
    pub request_id: Option<String>,
}

/// Audit query parameters
#[derive(Debug, Clone, Deserialize)]
pub struct AuditQuery {
    /// Filter by user ID
    pub user_id: Option<String>,

    /// Filter by event type
    pub event_type: Option<String>,

    /// Filter by resource
    pub resource: Option<String>,

    /// Filter by start time
    pub start_time: Option<DateTime<Utc>>,

    /// Filter by end time
    pub end_time: Option<DateTime<Utc>>,

    /// Filter by tenant
    pub tenant_id: Option<String>,

    /// Maximum results to return
    pub limit: Option<u32>,

    /// Offset for pagination
    pub offset: Option<u32>,
}

/// Audit log configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditConfig {
    /// Enable audit logging
    pub enabled: bool,

    /// Retention period in days
    pub retention_days: u32,

    /// Include request bodies in logs
    pub log_request_bodies: bool,

    /// Include response bodies in logs
    pub log_response_bodies: bool,

    /// Sensitive fields to redact
    pub redact_fields: Vec<String>,

    /// Minimum severity to log
    pub min_severity: Severity,
}

impl Default for AuditConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            retention_days: 365,
            log_request_bodies: false,
            log_response_bodies: false,
            redact_fields: vec![
                "password".to_string(),
                "api_key".to_string(),
                "token".to_string(),
                "secret".to_string(),
            ],
            min_severity: Severity::Info,
        }
    }
}

/// Audit log manager
pub struct AuditLog {
    config: AuditConfig,
    entries: Vec<AuditEntry>,
}

impl AuditLog {
    /// Create a new audit log
    pub async fn new(config: AuditConfig) -> Result<Self, AuditError> {
        Ok(Self {
            config,
            entries: Vec::new(),
        })
    }

    /// Log an audit event
    pub async fn log(&self, event: AuditEvent) -> Result<Uuid, AuditError> {
        if !self.config.enabled {
            return Ok(Uuid::nil());
        }

        let entry = AuditEntry {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            event,
            metadata: HashMap::new(),
            tenant_id: None,
            request_id: None,
        };

        // In production, this would write to database
        // self.db.insert(&entry).await?;

        tracing::info!(
            audit_id = %entry.id,
            audit_event = ?entry.event,
            "Audit event logged"
        );

        Ok(entry.id)
    }

    /// Log with metadata
    pub async fn log_with_metadata(
        &self,
        event: AuditEvent,
        metadata: HashMap<String, String>,
        tenant_id: Option<String>,
        request_id: Option<String>,
    ) -> Result<Uuid, AuditError> {
        if !self.config.enabled {
            return Ok(Uuid::nil());
        }

        let entry = AuditEntry {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            event,
            metadata,
            tenant_id,
            request_id,
        };

        tracing::info!(
            audit_id = %entry.id,
            tenant_id = ?entry.tenant_id,
            audit_event = ?entry.event,
            "Audit event logged"
        );

        Ok(entry.id)
    }

    /// Query audit logs
    pub async fn query(&self, query: AuditQuery) -> Result<Vec<AuditEntry>, AuditError> {
        // In production, this would query the database
        // For now, return empty
        Ok(vec![])
    }

    /// Get audit entry by ID
    pub async fn get(&self, id: Uuid) -> Result<Option<AuditEntry>, AuditError> {
        // In production, this would query the database
        Ok(None)
    }

    /// Export audit logs
    pub async fn export(
        &self,
        query: AuditQuery,
        format: ExportFormat,
    ) -> Result<Vec<u8>, AuditError> {
        let entries = self.query(query).await?;

        match format {
            ExportFormat::Json => {
                serde_json::to_vec(&entries).map_err(AuditError::SerializationError)
            }
            ExportFormat::Csv => {
                // CSV export implementation
                let mut csv = String::from("id,timestamp,event_type,user_id\n");
                for entry in entries {
                    csv.push_str(&format!(
                        "{},{},{:?},{}\n",
                        entry.id,
                        entry.timestamp.to_rfc3339(),
                        entry.event,
                        entry.metadata.get("user_id").unwrap_or(&"".to_string()),
                    ));
                }
                Ok(csv.into_bytes())
            }
        }
    }

    /// Purge old audit entries
    pub async fn purge(&self) -> Result<u64, AuditError> {
        // In production, this would delete old entries from database
        Ok(0)
    }
}

/// Export format
#[derive(Debug, Clone, Copy)]
pub enum ExportFormat {
    Json,
    Csv,
}

/// Audit error types
#[derive(Debug, thiserror::Error)]
pub enum AuditError {
    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Query error: {0}")]
    QueryError(String),

    #[error("Export error: {0}")]
    ExportError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_audit_log() {
        let config = AuditConfig::default();
        let audit = AuditLog::new(config).await.unwrap();

        let event = AuditEvent::AuthSuccess {
            user_id: "user123".to_string(),
            method: AuthMethod::Password,
            ip_address: "192.168.1.1".to_string(),
            user_agent: Some("Mozilla/5.0".to_string()),
        };

        let id = audit.log(event).await.unwrap();
        assert!(!id.is_nil());
    }

    #[test]
    fn test_audit_config_default() {
        let config = AuditConfig::default();
        assert!(config.enabled);
        assert_eq!(config.retention_days, 365);
        assert!(!config.log_request_bodies);
    }
}
