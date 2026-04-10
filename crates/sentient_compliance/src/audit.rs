//! ─── Audit Trail and Logging ───

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::collections::HashMap;

/// Audit log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLog {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub events: Vec<AuditEvent>,
    pub retention_days: u32,
}

/// Audit event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEvent {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub event_type: EventType,
    pub severity: AuditSeverity,
    pub actor: Actor,
    pub action: String,
    pub resource: String,
    pub result: ActionResult,
    pub details: HashMap<String, String>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub session_id: Option<String>,
    pub correlation_id: Option<String>,
}

impl AuditEvent {
    /// Create new audit event
    pub fn new(
        event_type: EventType,
        severity: AuditSeverity,
        actor: Actor,
        action: &str,
        resource: &str,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            event_type,
            severity,
            actor,
            action: action.into(),
            resource: resource.into(),
            result: ActionResult::Success,
            details: HashMap::new(),
            ip_address: None,
            user_agent: None,
            session_id: None,
            correlation_id: None,
        }
    }
    
    /// Add detail
    pub fn with_detail(mut self, key: &str, value: &str) -> Self {
        self.details.insert(key.into(), value.into());
        self
    }
    
    /// Set IP address
    pub fn with_ip(mut self, ip: &str) -> Self {
        self.ip_address = Some(ip.into());
        self
    }
    
    /// Set result
    pub fn with_result(mut self, result: ActionResult) -> Self {
        self.result = result;
        self
    }
}

/// Event type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
    // Authentication events
    Login,
    Logout,
    LoginFailed,
    PasswordChange,
    MfaEnabled,
    MfaDisabled,
    
    // Access events
    AccessGranted,
    AccessDenied,
    PermissionChange,
    RoleChange,
    
    // Data events
    DataCreate,
    DataRead,
    DataUpdate,
    DataDelete,
    DataExport,
    DataImport,
    
    // System events
    SystemStart,
    SystemStop,
    ConfigChange,
    BackupCreate,
    BackupRestore,
    
    // Security events
    SecurityAlert,
    VulnerabilityDetected,
    IncidentCreated,
    IncidentResolved,
    
    // User management
    UserCreate,
    UserUpdate,
    UserDelete,
    UserSuspend,
    UserUnsuspend,
    
    // API events
    ApiRequest,
    ApiKeyCreate,
    ApiKeyRevoke,
    
    // Compliance events
    ControlAssessed,
    EvidenceCollected,
    ReportGenerated,
}

/// Audit severity
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum AuditSeverity {
    Debug,
    Info,
    Warning,
    Error,
    Critical,
}

impl std::fmt::Display for AuditSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuditSeverity::Debug => write!(f, "DEBUG"),
            AuditSeverity::Info => write!(f, "INFO"),
            AuditSeverity::Warning => write!(f, "WARNING"),
            AuditSeverity::Error => write!(f, "ERROR"),
            AuditSeverity::Critical => write!(f, "CRITICAL"),
        }
    }
}

/// Actor (who performed the action)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Actor {
    pub actor_type: ActorType,
    pub id: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub roles: Vec<String>,
}

impl Actor {
    /// Create system actor
    pub fn system() -> Self {
        Self {
            actor_type: ActorType::System,
            id: "system".into(),
            name: Some("System".into()),
            email: None,
            roles: vec!["system".into()],
        }
    }
    
    /// Create user actor
    pub fn user(id: &str, name: &str, email: &str) -> Self {
        Self {
            actor_type: ActorType::User,
            id: id.into(),
            name: Some(name.into()),
            email: Some(email.into()),
            roles: Vec::new(),
        }
    }
    
    /// Create API actor
    pub fn api(api_key_id: &str) -> Self {
        Self {
            actor_type: ActorType::ApiKey,
            id: api_key_id.into(),
            name: None,
            email: None,
            roles: Vec::new(),
        }
    }
}

/// Actor type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActorType {
    User,
    ServiceAccount,
    ApiKey,
    System,
    External,
}

/// Action result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionResult {
    Success,
    Failure { reason: String },
    Partial { details: String },
}

/// Audit log manager
pub struct AuditManager {
    events: Vec<AuditEvent>,
    config: AuditConfig,
}

impl AuditManager {
    pub fn new(config: AuditConfig) -> Self {
        Self {
            events: Vec::new(),
            config,
        }
    }
    
    /// Log event
    pub fn log(&mut self, event: AuditEvent) {
        self.events.push(event);
        
        // Check retention
        self.enforce_retention();
    }
    
    /// Query events
    pub fn query(&self, filter: AuditFilter) -> Vec<&AuditEvent> {
        self.events.iter()
            .filter(|e| {
                if let Some(ref event_type) = filter.event_type {
                    if std::mem::discriminant(&e.event_type) != std::mem::discriminant(event_type) {
                        return false;
                    }
                }
                
                if let Some(ref severity) = filter.severity {
                    if e.severity < *severity {
                        return false;
                    }
                }
                
                if let Some(start) = filter.start_time {
                    if e.timestamp < start {
                        return false;
                    }
                }
                
                if let Some(end) = filter.end_time {
                    if e.timestamp > end {
                        return false;
                    }
                }
                
                if let Some(ref actor_id) = filter.actor_id {
                    if e.actor.id != *actor_id {
                        return false;
                    }
                }
                
                if let Some(ref resource) = filter.resource {
                    if !e.resource.contains(resource) {
                        return false;
                    }
                }
                
                true
            })
            .collect()
    }
    
    /// Export audit log
    pub fn export(&self, format: ExportFormat) -> Result<Vec<u8>, AuditError> {
        match format {
            ExportFormat::Json => {
                serde_json::to_vec_pretty(&self.events)
                    .map_err(|e| AuditError::ExportError(e.to_string()))
            }
            ExportFormat::Csv => {
                let mut csv = String::from("id,timestamp,event_type,severity,actor,action,resource,result\n");
                for event in &self.events {
                    csv.push_str(&format!(
                        "{},{},{:?},{},{},{},{},{:?}\n",
                        event.id,
                        event.timestamp.to_rfc3339(),
                        event.event_type,
                        event.severity,
                        event.actor.id,
                        event.action,
                        event.resource,
                        event.result,
                    ));
                }
                Ok(csv.into_bytes())
            }
        }
    }
    
    /// Enforce retention policy
    fn enforce_retention(&mut self) {
        let cutoff = Utc::now() - chrono::Duration::days(self.config.retention_days as i64);
        self.events.retain(|e| e.timestamp > cutoff);
    }
    
    /// Get event count
    pub fn event_count(&self) -> usize {
        self.events.len()
    }
    
    /// Get events by severity
    pub fn by_severity(&self, severity: AuditSeverity) -> Vec<&AuditEvent> {
        self.events.iter()
            .filter(|e| e.severity == severity)
            .collect()
    }
}

/// Audit configuration
#[derive(Debug, Clone)]
pub struct AuditConfig {
    pub enabled: bool,
    pub retention_days: u32,
    pub log_internal: bool,
    pub log_api_requests: bool,
    pub log_data_access: bool,
}

impl Default for AuditConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            retention_days: 365, // 1 year for SOC 2
            log_internal: true,
            log_api_requests: true,
            log_data_access: true,
        }
    }
}

/// Audit filter for queries
#[derive(Debug, Clone, Default)]
pub struct AuditFilter {
    pub event_type: Option<EventType>,
    pub severity: Option<AuditSeverity>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub actor_id: Option<String>,
    pub resource: Option<String>,
}

/// Export format
#[derive(Debug, Clone, Copy)]
pub enum ExportFormat {
    Json,
    Csv,
}

/// Audit errors
#[derive(Debug, thiserror::Error)]
pub enum AuditError {
    #[error("Export error: {0}")]
    ExportError(String),
    
    #[error("Storage error: {0}")]
    StorageError(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_audit_event() {
        let event = AuditEvent::new(
            EventType::Login,
            AuditSeverity::Info,
            Actor::user("123", "Test User", "test@example.com"),
            "login",
            "auth",
        );
        assert!(event.timestamp <= Utc::now());
    }
    
    #[test]
    fn test_audit_manager() {
        let mut manager = AuditManager::new(AuditConfig::default());
        let event = AuditEvent::new(
            EventType::Login,
            AuditSeverity::Info,
            Actor::system(),
            "test",
            "test",
        );
        manager.log(event);
        assert_eq!(manager.event_count(), 1);
    }
}
