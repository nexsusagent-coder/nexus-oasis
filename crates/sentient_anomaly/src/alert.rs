//! Alert Manager

use crate::AnomalySeverity;
use serde::{Deserialize, Serialize};

/// Alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    pub id: uuid::Uuid,
    pub severity: AnomalySeverity,
    pub message: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub resolved: bool,
}

impl Alert {
    pub fn new(severity: AnomalySeverity, message: impl Into<String>) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            severity,
            message: message.into(),
            timestamp: chrono::Utc::now(),
            resolved: false,
        }
    }
}

/// Alert manager
pub struct AlertManager {
    alerts: Vec<Alert>,
    max_alerts: usize,
}

impl AlertManager {
    pub fn new(max_alerts: usize) -> Self {
        Self {
            alerts: Vec::new(),
            max_alerts,
        }
    }

    pub fn add(&mut self, severity: AnomalySeverity, message: impl Into<String>) {
        if self.alerts.len() >= self.max_alerts {
            self.alerts.remove(0);
        }
        self.alerts.push(Alert::new(severity, message));
    }

    pub fn active(&self) -> Vec<&Alert> {
        self.alerts.iter().filter(|a| !a.resolved).collect()
    }

    pub fn count(&self) -> usize {
        self.alerts.len()
    }
}

impl Default for AlertManager {
    fn default() -> Self {
        Self::new(100)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alert_manager() {
        let mut manager = AlertManager::new(10);
        manager.add(AnomalySeverity::Warning, "Test alert");
        assert_eq!(manager.count(), 1);
    }
}
