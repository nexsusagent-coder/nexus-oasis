//! ─── Continuous Monitoring ───

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use crate::audit::AuditEvent;

/// Compliance monitor
pub struct ComplianceMonitor {
    status: ComplianceStatus,
    alerts: Vec<ComplianceAlert>,
    metrics: HashMap<String, Metric>,
    last_check: Option<DateTime<Utc>>,
}

impl ComplianceMonitor {
    pub fn new() -> Self {
        Self {
            status: ComplianceStatus::default(),
            alerts: Vec::new(),
            metrics: Self::initialize_metrics(),
            last_check: None,
        }
    }
    
    fn initialize_metrics() -> HashMap<String, Metric> {
        let mut metrics = HashMap::new();
        
        metrics.insert("security_score".into(), Metric::new("Security Score", 0.0, 100.0));
        metrics.insert("availability_score".into(), Metric::new("Availability Score", 0.0, 100.0));
        metrics.insert("control_compliance_rate".into(), Metric::new("Control Compliance Rate", 0.0, 100.0));
        metrics.insert("open_issues".into(), Metric::new("Open Issues", 0.0, 1000.0));
        metrics.insert("failed_logins_24h".into(), Metric::new("Failed Logins (24h)", 0.0, 1000.0));
        metrics.insert("api_errors_24h".into(), Metric::new("API Errors (24h)", 0.0, 10000.0));
        metrics.insert("backup_success_rate".into(), Metric::new("Backup Success Rate", 0.0, 100.0));
        metrics.insert("patch_compliance".into(), Metric::new("Patch Compliance", 0.0, 100.0));
        
        metrics
    }
    
    /// Process audit event for monitoring
    pub fn process_event(&mut self, event: &AuditEvent) {
        self.last_check = Some(Utc::now());
        
        // Check for anomalies
        if let Some(alert) = self.check_anomaly(event) {
            self.alerts.push(alert);
        }
        
        // Update relevant metrics
        self.update_metrics(event);
    }
    
    /// Check for anomalies
    fn check_anomaly(&self, event: &AuditEvent) -> Option<ComplianceAlert> {
        use crate::audit::{EventType, AuditSeverity};
        
        match (&event.event_type, event.severity) {
            (EventType::LoginFailed, AuditSeverity::Warning) => {
                Some(ComplianceAlert::new(
                    AlertSeverity::Medium,
                    "Multiple failed login attempts",
                    &format!("User {} failed login", event.actor.id),
                ))
            }
            (EventType::AccessDenied, AuditSeverity::Warning) => {
                Some(ComplianceAlert::new(
                    AlertSeverity::High,
                    "Access denied event",
                    &format!("Access denied for {} to {}", event.actor.id, event.resource),
                ))
            }
            (EventType::SecurityAlert, _) => {
                Some(ComplianceAlert::new(
                    AlertSeverity::Critical,
                    "Security alert detected",
                    &event.action,
                ))
            }
            _ => None,
        }
    }
    
    /// Update metrics based on event
    fn update_metrics(&mut self, event: &AuditEvent) {
        use crate::audit::EventType;
        
        match event.event_type {
            EventType::LoginFailed => {
                if let Some(metric) = self.metrics.get_mut("failed_logins_24h") {
                    metric.increment();
                }
            }
            EventType::ApiRequest => {
                if let crate::audit::ActionResult::Failure { .. } = event.result {
                    if let Some(metric) = self.metrics.get_mut("api_errors_24h") {
                        metric.increment();
                    }
                }
            }
            _ => {}
        }
    }
    
    /// Run compliance check
    pub fn run_check(&mut self) -> &ComplianceStatus {
        self.last_check = Some(Utc::now());
        
        // Calculate overall score
        let security = self.metrics.get("security_score")
            .map(|m| m.current).unwrap_or(0.0);
        let availability = self.metrics.get("availability_score")
            .map(|m| m.current).unwrap_or(0.0);
        let compliance = self.metrics.get("control_compliance_rate")
            .map(|m| m.current).unwrap_or(0.0);
        
        self.status.overall_score = (security + availability + compliance) / 3.0;
        
        // Determine status
        self.status.status = if self.status.overall_score >= 90.0 {
            ComplianceLevel::Excellent
        } else if self.status.overall_score >= 75.0 {
            ComplianceLevel::Good
        } else if self.status.overall_score >= 60.0 {
            ComplianceLevel::Fair
        } else if self.status.overall_score >= 40.0 {
            ComplianceLevel::Poor
        } else {
            ComplianceLevel::Critical
        };
        
        &self.status
    }
    
    /// Get current status
    pub fn status(&self) -> &ComplianceStatus {
        &self.status
    }
    
    /// Get active alerts
    pub fn active_alerts(&self) -> Vec<&ComplianceAlert> {
        self.alerts.iter()
            .filter(|a| a.acknowledged_at.is_none())
            .collect()
    }
    
    /// Acknowledge alert
    pub fn acknowledge_alert(&mut self, alert_id: &str, acknowledged_by: &str) {
        if let Some(alert) = self.alerts.iter_mut().find(|a| a.id == alert_id) {
            alert.acknowledge(acknowledged_by);
        }
    }
    
    /// Update metric
    pub fn update_metric(&mut self, name: &str, value: f64) {
        if let Some(metric) = self.metrics.get_mut(name) {
            metric.current = value;
        }
    }
}

impl Default for ComplianceMonitor {
    fn default() -> Self {
        Self::new()
    }
}

/// Compliance status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceStatus {
    pub status: ComplianceLevel,
    pub overall_score: f64,
    pub last_check: Option<DateTime<Utc>>,
    pub checks_total: u32,
    pub checks_passed: u32,
    pub checks_failed: u32,
    pub critical_issues: u32,
    pub open_alerts: u32,
}

impl Default for ComplianceStatus {
    fn default() -> Self {
        Self {
            status: ComplianceLevel::NotAssessed,
            overall_score: 0.0,
            last_check: None,
            checks_total: 0,
            checks_passed: 0,
            checks_failed: 0,
            critical_issues: 0,
            open_alerts: 0,
        }
    }
}

/// Compliance level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComplianceLevel {
    NotAssessed,
    Critical,
    Poor,
    Fair,
    Good,
    Excellent,
}

/// Compliance alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceAlert {
    pub id: String,
    pub severity: AlertSeverity,
    pub title: String,
    pub message: String,
    pub created_at: DateTime<Utc>,
    pub acknowledged_at: Option<DateTime<Utc>>,
    pub acknowledged_by: Option<String>,
}

impl ComplianceAlert {
    pub fn new(severity: AlertSeverity, title: &str, message: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            severity,
            title: title.into(),
            message: message.into(),
            created_at: Utc::now(),
            acknowledged_at: None,
            acknowledged_by: None,
        }
    }
    
    pub fn acknowledge(&mut self, by: &str) {
        self.acknowledged_at = Some(Utc::now());
        self.acknowledged_by = Some(by.into());
    }
}

/// Alert severity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlertSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Metric
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metric {
    pub name: String,
    pub current: f64,
    pub target: f64,
    pub min: f64,
    pub max: f64,
    pub unit: String,
}

impl Metric {
    pub fn new(name: &str, min: f64, max: f64) -> Self {
        Self {
            name: name.into(),
            current: 0.0,
            target: max,
            min,
            max,
            unit: "%".into(),
        }
    }
    
    pub fn increment(&mut self) {
        self.current = (self.current + 1.0).min(self.max);
    }
    
    pub fn percentage(&self) -> f64 {
        (self.current / self.max) * 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_monitor_creation() {
        let monitor = ComplianceMonitor::new();
        assert!(monitor.status().overall_score >= 0.0);
    }
    
    #[test]
    fn test_alert_creation() {
        let alert = ComplianceAlert::new(AlertSeverity::High, "Test", "Test message");
        assert_eq!(alert.title, "Test");
    }
}
