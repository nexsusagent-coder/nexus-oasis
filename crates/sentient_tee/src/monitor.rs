//! TEE Monitor

use serde::{Deserialize, Serialize};

/// Security event type
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SecurityEvent {
    TeeInitialized,
    AttestationPassed,
    AttestationFailed,
    SecurityViolation,
}

/// Security log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityLogEntry {
    pub event: SecurityEvent,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub message: String,
}

/// TEE Monitor
pub struct TeeMonitor {
    log: Vec<SecurityLogEntry>,
}

impl TeeMonitor {
    pub fn new() -> Self {
        Self { log: Vec::new() }
    }

    pub fn log_event(&mut self, event: SecurityEvent, message: impl Into<String>) {
        self.log.push(SecurityLogEntry {
            event,
            timestamp: chrono::Utc::now(),
            message: message.into(),
        });
    }

    pub fn entries(&self) -> &[SecurityLogEntry] {
        &self.log
    }
}

impl Default for TeeMonitor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monitor() {
        let mut monitor = TeeMonitor::new();
        monitor.log_event(SecurityEvent::TeeInitialized, "TEE started");
        assert_eq!(monitor.entries().len(), 1);
    }
}
