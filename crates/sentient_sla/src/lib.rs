//! ═══════════════════════════════════════════════════════════════════════════════
//!  SENTIENT SLA Monitoring System
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//!  Features:
//!  - Uptime monitoring and tracking
//!  - SLA breach detection and alerting
//!  - Support tier management (Free/Pro/Enterprise)
//!  - Incident tracking
//!  - Performance metrics
//!  - SLA credits calculation

pub mod uptime;
pub mod incidents;
pub mod support;
pub mod metrics;
pub mod credits;

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::collections::HashMap;

pub use uptime::{UptimeMonitor, UptimeStatus, UptimePeriod};
pub use incidents::{IncidentManager, Incident, IncidentSeverity, IncidentStatus};
pub use support::{SupportTier, SupportManager, Ticket, TicketPriority};
pub use metrics::{MetricsCollector, Metric, MetricType};
pub use credits::{SlaCreditManager, SlaCredit, CreditReason};

/// SLA Manager
pub struct SlaManager {
    /// Support tiers configuration
    tiers: HashMap<String, SupportTier>,
    
    /// Uptime monitor
    uptime: UptimeMonitor,
    
    /// Incident manager
    incidents: IncidentManager,
    
    /// Support manager
    support: SupportManager,
    
    /// Metrics collector
    metrics: MetricsCollector,
    
    /// SLA credit manager
    credits: SlaCreditManager,
    
    /// Current SLA status
    current_status: SlaStatus,
}

impl SlaManager {
    pub fn new() -> Self {
        Self {
            tiers: Self::initialize_tiers(),
            uptime: UptimeMonitor::new(),
            incidents: IncidentManager::new(),
            support: SupportManager::new(),
            metrics: MetricsCollector::new(),
            credits: SlaCreditManager::new(),
            current_status: SlaStatus::default(),
        }
    }
    
    /// Initialize support tiers
    fn initialize_tiers() -> HashMap<String, SupportTier> {
        let mut tiers = HashMap::new();
        
        // Free tier
        tiers.insert("free".into(), SupportTier {
            id: "free".into(),
            name: "Free".into(),
            price_monthly: 0.0,
            uptime_sla: 99.0,
            response_time_hours: 72,
            resolution_time_hours: 168, // 7 days
            support_channels: vec!["email".into(), "community".into()],
            priority_support: false,
            dedicated_manager: false,
            custom_sla: false,
            sla_credits: false,
            features: vec!["Basic features".into()],
        });
        
        // Pro tier
        tiers.insert("pro".into(), SupportTier {
            id: "pro".into(),
            name: "Pro".into(),
            price_monthly: 29.0,
            uptime_sla: 99.9,
            response_time_hours: 24,
            resolution_time_hours: 48,
            support_channels: vec!["email".into(), "chat".into(), "ticket".into()],
            priority_support: true,
            dedicated_manager: false,
            custom_sla: false,
            sla_credits: true,
            features: vec!["All features".into(), "Priority support".into()],
        });
        
        // Enterprise tier
        tiers.insert("enterprise".into(), SupportTier {
            id: "enterprise".into(),
            name: "Enterprise".into(),
            price_monthly: 299.0,
            uptime_sla: 99.99,
            response_time_hours: 4,
            resolution_time_hours: 8,
            support_channels: vec!["email".into(), "chat".into(), "ticket".into(), "phone".into(), "slack".into()],
            priority_support: true,
            dedicated_manager: true,
            custom_sla: true,
            sla_credits: true,
            features: vec!["All features".into(), "Dedicated support".into(), "Custom SLA".into()],
        });
        
        tiers
    }
    
    /// Check SLA status
    pub fn check_status(&mut self) -> &SlaStatus {
        let uptime = self.uptime.current_uptime();
        let tier = self.tiers.values().next().unwrap(); // Default tier
        
        self.current_status.uptime_percentage = uptime;
        self.current_status.sla_target = tier.uptime_sla;
        self.current_status.is_breach = uptime < tier.uptime_sla;
        self.current_status.uptime_delta = uptime - tier.uptime_sla;
        
        &self.current_status
    }
    
    /// Record uptime check
    pub fn record_uptime(&mut self, is_up: bool) {
        self.uptime.record_check(is_up);
    }
    
    /// Create incident
    pub fn create_incident(
        &mut self,
        title: &str,
        severity: IncidentSeverity,
        description: &str,
    ) -> Uuid {
        self.incidents.create_incident(title, severity, description)
    }
    
    /// Resolve incident
    pub fn resolve_incident(&mut self, incident_id: Uuid) {
        if let Some(incident) = self.incidents.resolve(incident_id) {
            // Update uptime
            if incident.severity == IncidentSeverity::Critical {
                self.uptime.record_downtime(incident.duration_seconds());
            }
            
            // Check for SLA credit eligibility
            if incident.is_sla_breach() {
                self.credits.issue_credit(incident);
            }
        }
    }
    
    /// Create support ticket
    pub fn create_ticket(
        &mut self,
        user_id: &str,
        tier_id: &str,
        subject: &str,
        description: &str,
        priority: TicketPriority,
    ) -> Uuid {
        let tier = self.tiers.get(tier_id);
        self.support.create_ticket(user_id, tier, subject, description, priority)
    }
    
    /// Get tier
    pub fn get_tier(&self, tier_id: &str) -> Option<&SupportTier> {
        self.tiers.get(tier_id)
    }
    
    /// Get all tiers
    pub fn all_tiers(&self) -> Vec<&SupportTier> {
        self.tiers.values().collect()
    }
    
    /// Get uptime report
    pub fn uptime_report(&self, period: UptimePeriod) -> uptime::UptimeReport {
        self.uptime.report(period)
    }
    
    /// Get active incidents
    pub fn active_incidents(&self) -> Vec<&Incident> {
        self.incidents.active()
    }
    
    /// Calculate SLA credits
    pub fn calculate_credits(&self, user_id: &str, period_start: DateTime<Utc>, period_end: DateTime<Utc>) -> f64 {
        self.credits.calculate(user_id, period_start, period_end)
    }
}

impl Default for SlaManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Current SLA status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlaStatus {
    pub uptime_percentage: f64,
    pub sla_target: f64,
    pub is_breach: bool,
    pub uptime_delta: f64,
    pub active_incidents: u32,
    pub last_incident: Option<DateTime<Utc>>,
    pub mttr_hours: f64, // Mean Time To Resolution
    pub mtbf_hours: f64, // Mean Time Between Failures
}

impl Default for SlaStatus {
    fn default() -> Self {
        Self {
            uptime_percentage: 100.0,
            sla_target: 99.9,
            is_breach: false,
            uptime_delta: 0.1,
            active_incidents: 0,
            last_incident: None,
            mttr_hours: 0.0,
            mtbf_hours: 0.0,
        }
    }
}

/// SLA errors
#[derive(Debug, thiserror::Error)]
pub enum SlaError {
    #[error("Incident not found: {0}")]
    IncidentNotFound(String),
    
    #[error("Tier not found: {0}")]
    TierNotFound(String),
    
    #[error("SLA breach detected")]
    SlaBreach,
    
    #[error("Calculation error: {0}")]
    CalculationError(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_sla_manager() {
        let manager = SlaManager::new();
        assert!(manager.all_tiers().len() == 3);
    }
    
    #[test]
    fn test_support_tiers() {
        let manager = SlaManager::new();
        let free = manager.get_tier("free").unwrap();
        assert_eq!(free.uptime_sla, 99.0);
        
        let pro = manager.get_tier("pro").unwrap();
        assert_eq!(pro.uptime_sla, 99.9);
        
        let enterprise = manager.get_tier("enterprise").unwrap();
        assert_eq!(enterprise.uptime_sla, 99.99);
    }
    
    #[test]
    fn test_uptime_tracking() {
        let mut manager = SlaManager::new();
        manager.record_uptime(true);
        manager.record_uptime(true);
        manager.record_uptime(false);
        
        let status = manager.check_status();
        assert!(status.uptime_percentage < 100.0);
    }
}
