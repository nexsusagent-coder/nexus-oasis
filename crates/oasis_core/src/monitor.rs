//! ═══════════════════════════════════════════════════════════════════════════════
//!  RUNTIME MONITOR - Real-time Monitoring
//! ═══════════════════════════════════════════════════════════════════════════════

use serde::{Deserialize, Serialize};

/// Alert severity levels
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AlertSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

impl Default for AlertSeverity {
    fn default() -> Self {
        Self::Info
    }
}

/// Runtime alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeAlert {
    pub id: uuid::Uuid,
    pub severity: AlertSeverity,
    pub message: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub resolved: bool,
}

impl RuntimeAlert {
    pub fn new(severity: AlertSeverity, message: impl Into<String>) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            severity,
            message: message.into(),
            timestamp: chrono::Utc::now(),
            resolved: false,
        }
    }

    pub fn resolve(&mut self) {
        self.resolved = true;
    }
}

/// Alert manager
pub struct AlertManager {
    alerts: Vec<RuntimeAlert>,
    max_alerts: usize,
}

impl AlertManager {
    pub fn new(max_alerts: usize) -> Self {
        Self {
            alerts: Vec::new(),
            max_alerts,
        }
    }

    /// Add new alert
    pub fn add_alert(&mut self, severity: AlertSeverity, message: impl Into<String>) -> uuid::Uuid {
        let message = message.into();
        let alert = RuntimeAlert::new(severity, message.clone());
        let id = alert.id;
        
        if self.alerts.len() >= self.max_alerts {
            if let Some(pos) = self.alerts.iter().position(|a| a.resolved) {
                self.alerts.remove(pos);
            } else {
                self.alerts.remove(0);
            }
        }
        
        self.alerts.push(alert);
        
        match severity {
            AlertSeverity::Info => log::info!("📋 ALERT: {}", message),
            AlertSeverity::Warning => log::warn!("⚠️  ALERT: {}", message),
            AlertSeverity::Error => log::error!("🔴 ALERT: {}", message),
            AlertSeverity::Critical => log::error!("🚨 CRITICAL ALERT: {}", message),
        }
        
        id
    }

    /// Resolve alert
    pub fn resolve_alert(&mut self, id: uuid::Uuid) -> bool {
        if let Some(alert) = self.alerts.iter_mut().find(|a| a.id == id) {
            alert.resolve();
            true
        } else {
            false
        }
    }

    /// Get active alerts
    pub fn active_alerts(&self) -> Vec<&RuntimeAlert> {
        self.alerts.iter().filter(|a| !a.resolved).collect()
    }

    /// Get alert count
    pub fn alert_count(&self) -> usize {
        self.alerts.len()
    }

    /// Get active count
    pub fn active_count(&self) -> usize {
        self.alerts.iter().filter(|a| !a.resolved).count()
    }

    /// Clear all
    pub fn clear(&mut self) {
        self.alerts.clear();
    }
}

impl Default for AlertManager {
    fn default() -> Self {
        Self::new(100)
    }
}

/// Runtime monitor for metrics
pub struct RuntimeMonitor {
    start_time: std::time::Instant,
    transaction_count: u64,
    error_count: u64,
}

impl RuntimeMonitor {
    pub fn new() -> Self {
        Self {
            start_time: std::time::Instant::now(),
            transaction_count: 0,
            error_count: 0,
        }
    }

    pub fn transaction_started(&mut self) {
        self.transaction_count += 1;
    }

    pub fn transaction_completed(&mut self, _duration_ms: u64, success: bool) {
        if !success {
            self.error_count += 1;
        }
    }

    pub fn uptime_secs(&self) -> u64 {
        self.start_time.elapsed().as_secs()
    }

    pub fn transaction_count(&self) -> u64 {
        self.transaction_count
    }

    pub fn error_count(&self) -> u64 {
        self.error_count
    }

    pub fn gather(&self) -> String {
        format!(
            "# HELP oasis_transactions_total Total transactions\n\
             # TYPE oasis_transactions_total counter\n\
             oasis_transactions_total {}\n\
             # HELP oasis_errors_total Total errors\n\
             # TYPE oasis_errors_total counter\n\
             oasis_errors_total {}\n\
             # HELP oasis_uptime_seconds Runtime uptime\n\
             # TYPE oasis_uptime_seconds gauge\n\
             oasis_uptime_seconds {}\n",
            self.transaction_count,
            self.error_count,
            self.uptime_secs()
        )
    }
}

impl Default for RuntimeMonitor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runtime_monitor() {
        let mut monitor = RuntimeMonitor::new();
        monitor.transaction_started();
        monitor.transaction_completed(100, true);
        
        assert_eq!(monitor.transaction_count(), 1);
        assert_eq!(monitor.error_count(), 0);
    }

    #[test]
    fn test_alert_manager() {
        let mut manager = AlertManager::new(10);
        
        let id = manager.add_alert(AlertSeverity::Warning, "Test alert");
        assert_eq!(manager.active_count(), 1);
        
        manager.resolve_alert(id);
        assert_eq!(manager.active_count(), 0);
    }
}
