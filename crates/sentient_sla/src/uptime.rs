//! ─── Uptime Monitoring ───

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::collections::VecDeque;

/// Uptime monitor
pub struct UptimeMonitor {
    checks: VecDeque<UptimeCheck>,
    total_checks: u64,
    successful_checks: u64,
    downtime_seconds: u64,
    last_check: Option<DateTime<Utc>>,
    current_status: UptimeStatus,
}

impl UptimeMonitor {
    pub fn new() -> Self {
        Self {
            checks: VecDeque::with_capacity(10000),
            total_checks: 0,
            successful_checks: 0,
            downtime_seconds: 0,
            last_check: None,
            current_status: UptimeStatus::Operational,
        }
    }
    
    /// Record uptime check
    pub fn record_check(&mut self, is_up: bool) {
        let check = UptimeCheck {
            timestamp: Utc::now(),
            is_up,
            response_time_ms: None,
        };
        
        self.checks.push_back(check);
        self.total_checks += 1;
        
        if is_up {
            self.successful_checks += 1;
        }
        
        self.last_check = Some(Utc::now());
        self.update_status();
        
        // Keep only last 10000 checks
        while self.checks.len() > 10000 {
            self.checks.pop_front();
        }
    }
    
    /// Record downtime duration
    pub fn record_downtime(&mut self, seconds: u64) {
        self.downtime_seconds += seconds;
    }
    
    /// Calculate current uptime percentage
    pub fn current_uptime(&self) -> f64 {
        if self.total_checks == 0 {
            return 100.0;
        }
        
        (self.successful_checks as f64 / self.total_checks as f64) * 100.0
    }
    
    /// Calculate uptime for period
    pub fn uptime_for_period(&self, period: UptimePeriod) -> f64 {
        let cutoff = Self::period_cutoff(period);
        let checks: Vec<_> = self.checks.iter()
            .filter(|c| c.timestamp > cutoff)
            .collect();
        
        if checks.is_empty() {
            return 100.0;
        }
        
        let successful = checks.iter().filter(|c| c.is_up).count();
        (successful as f64 / checks.len() as f64) * 100.0
    }
    
    fn period_cutoff(period: UptimePeriod) -> DateTime<Utc> {
        Utc::now() - match period {
            UptimePeriod::Last24Hours => chrono::Duration::hours(24),
            UptimePeriod::Last7Days => chrono::Duration::days(7),
            UptimePeriod::Last30Days => chrono::Duration::days(30),
            UptimePeriod::Last90Days => chrono::Duration::days(90),
            UptimePeriod::LastYear => chrono::Duration::days(365),
        }
    }
    
    fn update_status(&mut self) {
        let uptime = self.current_uptime();
        
        self.current_status = if uptime >= 99.9 {
            UptimeStatus::Operational
        } else if uptime >= 99.0 {
            UptimeStatus::Degraded
        } else if uptime >= 95.0 {
            UptimeStatus::PartialOutage
        } else {
            UptimeStatus::MajorOutage
        };
    }
    
    /// Get current status
    pub fn status(&self) -> &UptimeStatus {
        &self.current_status
    }
    
    /// Generate uptime report
    pub fn report(&self, period: UptimePeriod) -> UptimeReport {
        UptimeReport {
            period: format!("{:?}", period),
            uptime_percentage: self.uptime_for_period(period),
            total_checks: self.checks.len() as u64,
            successful_checks: self.successful_checks,
            failed_checks: self.total_checks - self.successful_checks,
            downtime_seconds: self.downtime_seconds,
            status: self.current_status,
            checks: self.checks.iter()
                .take(100)
                .cloned()
                .collect(),
        }
    }
}

impl Default for UptimeMonitor {
    fn default() -> Self {
        Self::new()
    }
}

/// Uptime check record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UptimeCheck {
    pub timestamp: DateTime<Utc>,
    pub is_up: bool,
    pub response_time_ms: Option<u64>,
}

/// Uptime status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UptimeStatus {
    Operational,
    Degraded,
    PartialOutage,
    MajorOutage,
    Maintenance,
}

impl std::fmt::Display for UptimeStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UptimeStatus::Operational => write!(f, "✅ Operational"),
            UptimeStatus::Degraded => write!(f, "⚠️ Degraded"),
            UptimeStatus::PartialOutage => write!(f, "🔶 Partial Outage"),
            UptimeStatus::MajorOutage => write!(f, "🔴 Major Outage"),
            UptimeStatus::Maintenance => write!(f, "🔧 Maintenance"),
        }
    }
}

/// Uptime period
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum UptimePeriod {
    Last24Hours,
    Last7Days,
    Last30Days,
    Last90Days,
    LastYear,
}

/// Uptime report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UptimeReport {
    pub period: String,
    pub uptime_percentage: f64,
    pub total_checks: u64,
    pub successful_checks: u64,
    pub failed_checks: u64,
    pub downtime_seconds: u64,
    pub status: UptimeStatus,
    pub checks: Vec<UptimeCheck>,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_uptime_monitor() {
        let mut monitor = UptimeMonitor::new();
        monitor.record_check(true);
        monitor.record_check(true);
        monitor.record_check(false);
        
        assert!(monitor.current_uptime() < 100.0);
        assert!(monitor.current_uptime() > 50.0);
    }
}
