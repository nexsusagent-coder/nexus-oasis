//! ═══════════════════════════════════════════════════════════════════════════════
//!  AUDIT LOG - Security Audit Trail
//! ═══════════════════════════════════════════════════════════════════════════════

use crate::AccessLevel;
use serde::{Deserialize, Serialize};

// ═══════════════════════════════════════════════════════════════════════════════
//  AUDIT EVENT
// ═══════════════════════════════════════════════════════════════════════════════

/// Audit event type
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AuditEventType {
    VaultUnlock,
    VaultLock,
    SecretStore,
    SecretRetrieve,
    SecretDelete,
    KeyRotation,
    AccessDenied,
    SecretExpired,
}

/// Audit event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEvent {
    pub id: uuid::Uuid,
    pub event_type: AuditEventType,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub path: Option<String>,
    pub access_level: Option<AccessLevel>,
    pub success: bool,
    pub message: String,
}

impl AuditEvent {
    pub fn new(event_type: AuditEventType, message: impl Into<String>) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            event_type,
            timestamp: chrono::Utc::now(),
            path: None,
            access_level: None,
            success: true,
            message: message.into(),
        }
    }

    pub fn with_path(mut self, path: impl Into<String>) -> Self {
        self.path = Some(path.into());
        self
    }

    pub fn with_access_level(mut self, level: AccessLevel) -> Self {
        self.access_level = Some(level);
        self
    }

    pub fn with_success(mut self, success: bool) -> Self {
        self.success = success;
        self
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  AUDIT LOG
// ═══════════════════════════════════════════════════════════════════════════════

/// Audit log storage
#[derive(Debug, Clone, Default)]
pub struct AuditLog {
    events: Vec<AuditEvent>,
    max_events: usize,
}

impl AuditLog {
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
            max_events: 10000,
        }
    }

    pub fn with_max_events(mut self, max: usize) -> Self {
        self.max_events = max;
        self
    }

    /// Add event to log
    pub fn log(&mut self, event: AuditEvent) {
        if self.events.len() >= self.max_events {
            self.events.remove(0);
        }
        self.events.push(event);
    }

    /// Log vault unlock
    pub fn log_unlock(&mut self) {
        self.log(AuditEvent::new(
            AuditEventType::VaultUnlock,
            "Vault unlocked successfully",
        ));
    }

    /// Log vault lock
    pub fn log_lock(&mut self) {
        self.log(AuditEvent::new(
            AuditEventType::VaultLock,
            "Vault locked",
        ));
    }

    /// Log secret store
    pub fn log_store(&mut self, path: &str, level: AccessLevel) {
        self.log(
            AuditEvent::new(AuditEventType::SecretStore, "Secret stored")
                .with_path(path)
                .with_access_level(level),
        );
    }

    /// Log secret retrieve
    pub fn log_retrieve(&mut self, path: &str, level: AccessLevel) {
        self.log(
            AuditEvent::new(AuditEventType::SecretRetrieve, "Secret retrieved")
                .with_path(path)
                .with_access_level(level),
        );
    }

    /// Log secret delete
    pub fn log_delete(&mut self, path: &str) {
        self.log(
            AuditEvent::new(AuditEventType::SecretDelete, "Secret deleted")
                .with_path(path),
        );
    }

    /// Log key rotation
    pub fn log_key_rotation(&mut self) {
        self.log(AuditEvent::new(
            AuditEventType::KeyRotation,
            "Encryption key rotated",
        ));
    }

    /// Log access denied
    pub fn log_access_denied(&mut self, path: &str, reason: &str) {
        self.log(
            AuditEvent::new(
                AuditEventType::AccessDenied,
                format!("Access denied: {}", reason),
            )
            .with_path(path)
            .with_success(false),
        );
    }

    /// Log secret expired
    pub fn log_secret_expired(&mut self, path: &str) {
        self.log(
            AuditEvent::new(
                AuditEventType::SecretExpired,
                "Secret has expired",
            )
            .with_path(path)
            .with_success(false),
        );
    }

    /// Get all events
    pub fn events(&self) -> &[AuditEvent] {
        &self.events
    }

    /// Get events by type
    pub fn events_by_type(&self, event_type: AuditEventType) -> Vec<&AuditEvent> {
        self.events
            .iter()
            .filter(|e| e.event_type == event_type)
            .collect()
    }

    /// Get events for path
    pub fn events_for_path(&self, path: &str) -> Vec<&AuditEvent> {
        self.events
            .iter()
            .filter(|e| e.path.as_deref() == Some(path))
            .collect()
    }

    /// Get failed events
    pub fn failed_events(&self) -> Vec<&AuditEvent> {
        self.events.iter().filter(|e| !e.success).collect()
    }

    /// Get event count
    pub fn count(&self) -> usize {
        self.events.len()
    }

    /// Clear log
    pub fn clear(&mut self) {
        self.events.clear();
    }

    /// Export to JSON
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(&self.events)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  AUDIT REPORT
// ═══════════════════════════════════════════════════════════════════════════════

/// Audit report generator
pub struct AuditReport<'a> {
    log: &'a AuditLog,
}

impl<'a> AuditReport<'a> {
    pub fn from_log(log: &'a AuditLog) -> Self {
        Self { log }
    }

    /// Generate summary report
    pub fn summary(&self) -> AuditSummary {
        let total = self.log.count();
        let unlocks = self.log.events_by_type(AuditEventType::VaultUnlock).len();
        let stores = self.log.events_by_type(AuditEventType::SecretStore).len();
        let retrieves = self.log.events_by_type(AuditEventType::SecretRetrieve).len();
        let deletes = self.log.events_by_type(AuditEventType::SecretDelete).len();
        let rotations = self.log.events_by_type(AuditEventType::KeyRotation).len();
        let denied = self.log.events_by_type(AuditEventType::AccessDenied).len();
        let expired = self.log.events_by_type(AuditEventType::SecretExpired).len();

        AuditSummary {
            total_events: total,
            unlocks,
            stores,
            retrieves,
            deletes,
            key_rotations: rotations,
            access_denied: denied,
            secrets_expired: expired,
        }
    }

    /// Generate full report as string
    pub fn full_report(&self) -> String {
        let summary = self.summary();
        format!(
            "╔══════════════════════════════════════════════════════════╗\n\
             ║              OASIS VAULT AUDIT REPORT                    ║\n\
             ╠══════════════════════════════════════════════════════════╣\n\
             ║  Total Events:        {:>10}                         ║\n\
             ║  Vault Unlocks:       {:>10}                         ║\n\
             ║  Secrets Stored:      {:>10}                         ║\n\
             ║  Secrets Retrieved:   {:>10}                         ║\n\
             ║  Secrets Deleted:     {:>10}                         ║\n\
             ║  Key Rotations:       {:>10}                         ║\n\
             ║  Access Denied:       {:>10} ⚠️                      ║\n\
             ║  Secrets Expired:     {:>10} ⚠️                      ║\n\
             ╚══════════════════════════════════════════════════════════╝",
            summary.total_events,
            summary.unlocks,
            summary.stores,
            summary.retrieves,
            summary.deletes,
            summary.key_rotations,
            summary.access_denied,
            summary.secrets_expired,
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditSummary {
    pub total_events: usize,
    pub unlocks: usize,
    pub stores: usize,
    pub retrieves: usize,
    pub deletes: usize,
    pub key_rotations: usize,
    pub access_denied: usize,
    pub secrets_expired: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audit_log() {
        let mut log = AuditLog::new();
        
        log.log_unlock();
        log.log_store("/test/secret", AccessLevel::Secret);
        log.log_retrieve("/test/secret", AccessLevel::Secret);
        
        assert_eq!(log.count(), 3);
    }

    #[test]
    fn test_audit_report() {
        let mut log = AuditLog::new();
        
        log.log_unlock();
        log.log_store("/test/secret", AccessLevel::Secret);
        log.log_access_denied("/test/secret", "Insufficient privileges");
        
        let report = AuditReport::from_log(&log);
        let summary = report.summary();
        
        assert_eq!(summary.total_events, 3);
        assert_eq!(summary.access_denied, 1);
    }

    #[test]
    fn test_audit_events_by_type() {
        let mut log = AuditLog::new();
        
        log.log_unlock();
        log.log_unlock();
        log.log_lock();
        
        let unlocks = log.events_by_type(AuditEventType::VaultUnlock);
        assert_eq!(unlocks.len(), 2);
    }
}
