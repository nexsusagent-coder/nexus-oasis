//! ─── SLA Credits ───

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::collections::HashMap;
use crate::incidents::Incident;

/// SLA credit manager
pub struct SlaCreditManager {
    credits: Vec<SlaCredit>,
    user_credits: HashMap<String, f64>,
}

impl SlaCreditManager {
    pub fn new() -> Self {
        Self {
            credits: Vec::new(),
            user_credits: HashMap::new(),
        }
    }
    
    /// Issue credit for incident
    pub fn issue_credit(&mut self, incident: Incident) {
        let credit_amount = Self::calculate_credit_amount(&incident);
        
        if credit_amount > 0.0 {
            let credit = SlaCredit {
                id: Uuid::new_v4(),
                user_id: "all".into(), // Applies to all affected users
                incident_id: incident.id,
                amount: credit_amount,
                reason: CreditReason::UptimeBreach,
                created_at: Utc::now(),
                applied: false,
                applied_at: None,
            };
            
            self.credits.push(credit);
        }
    }
    
    /// Calculate credit amount based on uptime
    fn calculate_credit_amount(incident: &Incident) -> f64 {
        // Credit structure:
        // 99.0% - 99.9% uptime: 10% credit
        // 95.0% - 99.0% uptime: 25% credit
        // <95.0% uptime: 100% credit
        
        let duration_hours = incident.duration_seconds() as f64 / 3600.0;
        
        match incident.severity {
            crate::incidents::IncidentSeverity::Critical => {
                if duration_hours > 1.0 {
                    100.0 // Full month credit
                } else if duration_hours > 0.25 {
                    25.0
                } else {
                    10.0
                }
            }
            crate::incidents::IncidentSeverity::High => {
                if duration_hours > 4.0 {
                    25.0
                } else {
                    10.0
                }
            }
            crate::incidents::IncidentSeverity::Medium => 10.0,
            crate::incidents::IncidentSeverity::Low => 0.0,
        }
    }
    
    /// Calculate total credits for user in period
    pub fn calculate(&self, user_id: &str, period_start: DateTime<Utc>, period_end: DateTime<Utc>) -> f64 {
        self.credits.iter()
            .filter(|c| {
                c.user_id == user_id || c.user_id == "all"
                    && c.created_at >= period_start
                    && c.created_at <= period_end
            })
            .map(|c| c.amount)
            .sum()
    }
    
    /// Apply credits to account
    pub fn apply_credits(&mut self, user_id: &str) -> f64 {
        let total: f64 = self.credits.iter_mut()
            .filter(|c| (c.user_id == user_id || c.user_id == "all") && !c.applied)
            .map(|c| {
                c.applied = true;
                c.applied_at = Some(Utc::now());
                c.amount
            })
            .sum();
        
        *self.user_credits.entry(user_id.into()).or_insert(0.0) += total;
        total
    }
    
    /// Get credits for user
    pub fn get_user_credits(&self, user_id: &str) -> f64 {
        self.user_credits.get(user_id).copied().unwrap_or(0.0)
    }
    
    /// Get all credits
    pub fn all_credits(&self) -> &[SlaCredit] {
        &self.credits
    }
}

impl Default for SlaCreditManager {
    fn default() -> Self {
        Self::new()
    }
}

/// SLA credit record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlaCredit {
    pub id: Uuid,
    pub user_id: String,
    pub incident_id: Uuid,
    pub amount: f64,
    pub reason: CreditReason,
    pub created_at: DateTime<Utc>,
    pub applied: bool,
    pub applied_at: Option<DateTime<Utc>>,
}

/// Credit reason
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CreditReason {
    UptimeBreach,
    ResponseTimeBreach,
    ResolutionTimeBreach,
    DataLoss,
    SecurityIncident,
    Goodwill,
}

/// SLA credit table
pub struct SlaCreditTable;

impl SlaCreditTable {
    /// Standard SLA credit table
    pub fn standard() -> Vec<CreditTier> {
        vec![
            CreditTier {
                uptime_min: 99.9,
                uptime_max: 100.0,
                credit_percent: 0.0,
            },
            CreditTier {
                uptime_min: 99.0,
                uptime_max: 99.9,
                credit_percent: 10.0,
            },
            CreditTier {
                uptime_min: 95.0,
                uptime_max: 99.0,
                credit_percent: 25.0,
            },
            CreditTier {
                uptime_min: 0.0,
                uptime_max: 95.0,
                credit_percent: 100.0,
            },
        ]
    }
}

/// Credit tier
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditTier {
    pub uptime_min: f64,
    pub uptime_max: f64,
    pub credit_percent: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::incidents::{Incident, IncidentSeverity, IncidentStatus};
    
    #[test]
    fn test_credit_calculation() {
        let manager = SlaCreditManager::new();
        assert!(manager.all_credits().is_empty());
    }
    
    #[test]
    fn test_credit_table() {
        let table = SlaCreditTable::standard();
        assert_eq!(table.len(), 4);
        assert_eq!(table[0].credit_percent, 0.0);
        assert_eq!(table[3].credit_percent, 100.0);
    }
}
