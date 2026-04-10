//! ─── Incident Management ───

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::collections::HashMap;

/// Incident manager
pub struct IncidentManager {
    incidents: HashMap<Uuid, Incident>,
}

impl IncidentManager {
    pub fn new() -> Self {
        Self {
            incidents: HashMap::new(),
        }
    }
    
    /// Create new incident
    pub fn create_incident(
        &mut self,
        title: &str,
        severity: IncidentSeverity,
        description: &str,
    ) -> Uuid {
        let id = Uuid::new_v4();
        let incident = Incident {
            id,
            title: title.into(),
            severity,
            status: IncidentStatus::Investigating,
            description: description.into(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            resolved_at: None,
            acknowledged_at: None,
            affected_components: vec!["API".into(), "Dashboard".into()],
            updates: Vec::new(),
            sla_breach: false,
        };
        
        self.incidents.insert(id, incident);
        id
    }
    
    /// Get incident
    pub fn get(&self, id: Uuid) -> Option<&Incident> {
        self.incidents.get(&id)
    }
    
    /// Update incident status
    pub fn update_status(&mut self, id: Uuid, status: IncidentStatus) -> Option<&Incident> {
        if let Some(incident) = self.incidents.get_mut(&id) {
            incident.status = status;
            incident.updated_at = Utc::now();
            
            if status == IncidentStatus::Resolved {
                incident.resolved_at = Some(Utc::now());
            }
            
            return Some(incident);
        }
        None
    }
    
    /// Resolve incident
    pub fn resolve(&mut self, id: Uuid) -> Option<Incident> {
        if let Some(mut incident) = self.incidents.remove(&id) {
            incident.status = IncidentStatus::Resolved;
            incident.resolved_at = Some(Utc::now());
            incident.updated_at = Utc::now();
            
            // Check if SLA breach
            let duration = incident.duration_seconds();
            let breach_threshold = match incident.severity {
                IncidentSeverity::Critical => 15 * 60,    // 15 minutes
                IncidentSeverity::High => 60 * 60,        // 1 hour
                IncidentSeverity::Medium => 4 * 60 * 60,  // 4 hours
                IncidentSeverity::Low => 24 * 60 * 60,    // 24 hours
            };
            incident.sla_breach = duration > breach_threshold;
            
            return Some(incident);
        }
        None
    }
    
    /// Add update to incident
    pub fn add_update(&mut self, id: Uuid, message: &str) {
        if let Some(incident) = self.incidents.get_mut(&id) {
            incident.updates.push(IncidentUpdate {
                timestamp: Utc::now(),
                message: message.into(),
                status: incident.status,
            });
            incident.updated_at = Utc::now();
        }
    }
    
    /// Get all active incidents
    pub fn active(&self) -> Vec<&Incident> {
        self.incidents.values()
            .filter(|i| i.status != IncidentStatus::Resolved)
            .collect()
    }
    
    /// Get incidents by severity
    pub fn by_severity(&self, severity: IncidentSeverity) -> Vec<&Incident> {
        self.incidents.values()
            .filter(|i| i.severity == severity)
            .collect()
    }
}

impl Default for IncidentManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Incident record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Incident {
    pub id: Uuid,
    pub title: String,
    pub severity: IncidentSeverity,
    pub status: IncidentStatus,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub acknowledged_at: Option<DateTime<Utc>>,
    pub affected_components: Vec<String>,
    pub updates: Vec<IncidentUpdate>,
    pub sla_breach: bool,
}

impl Incident {
    /// Calculate incident duration in seconds
    pub fn duration_seconds(&self) -> u64 {
        let end = self.resolved_at.unwrap_or_else(Utc::now);
        (end - self.created_at).num_seconds().max(0) as u64
    }
    
    /// Check if this is an SLA breach
    pub fn is_sla_breach(&self) -> bool {
        self.sla_breach
    }
}

/// Incident severity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IncidentSeverity {
    Critical,  // Complete service outage
    High,      // Major functionality broken
    Medium,    // Partial functionality degraded
    Low,       // Minor issue, workaround available
}

/// Incident status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IncidentStatus {
    Investigating,
    Identified,
    Monitoring,
    Resolved,
    Postmortem,
}

/// Incident update
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncidentUpdate {
    pub timestamp: DateTime<Utc>,
    pub message: String,
    pub status: IncidentStatus,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_incident_creation() {
        let mut manager = IncidentManager::new();
        let id = manager.create_incident("Test", IncidentSeverity::High, "Test incident");
        assert!(manager.get(id).is_some());
    }
    
    #[test]
    fn test_incident_resolution() {
        let mut manager = IncidentManager::new();
        let id = manager.create_incident("Test", IncidentSeverity::High, "Test");
        let incident = manager.resolve(id);
        assert!(incident.unwrap().resolved_at.is_some());
    }
}
