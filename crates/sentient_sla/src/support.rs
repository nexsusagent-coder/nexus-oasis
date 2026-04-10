//! ─── Support Tier Management ───

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::collections::HashMap;

/// Support manager
pub struct SupportManager {
    tickets: HashMap<Uuid, Ticket>,
}

impl SupportManager {
    pub fn new() -> Self {
        Self {
            tickets: HashMap::new(),
        }
    }
    
    /// Create support ticket
    pub fn create_ticket(
        &mut self,
        user_id: &str,
        tier: Option<&SupportTier>,
        subject: &str,
        description: &str,
        priority: TicketPriority,
    ) -> Uuid {
        let id = Uuid::new_v4();
        let response_time_hours = tier.map(|t| t.response_time_hours).unwrap_or(72);
        
        let ticket = Ticket {
            id,
            user_id: user_id.into(),
            tier_id: tier.map(|t| t.id.clone()).unwrap_or_default(),
            subject: subject.into(),
            description: description.into(),
            priority,
            status: TicketStatus::Open,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            first_response_at: None,
            resolved_at: None,
            sla_deadline: Utc::now() + chrono::Duration::hours(response_time_hours as i64),
            assigned_to: None,
            messages: Vec::new(),
        };
        
        self.tickets.insert(id, ticket);
        id
    }
    
    /// Get ticket
    pub fn get(&self, id: Uuid) -> Option<&Ticket> {
        self.tickets.get(&id)
    }
    
    /// Add response to ticket
    pub fn respond(&mut self, id: Uuid, from: &str, message: &str) {
        if let Some(ticket) = self.tickets.get_mut(&id) {
            ticket.messages.push(TicketMessage {
                timestamp: Utc::now(),
                from: from.into(),
                message: message.into(),
                is_staff: !from.starts_with("user-"),
            });
            
            if ticket.first_response_at.is_none() {
                ticket.first_response_at = Some(Utc::now());
            }
            
            ticket.updated_at = Utc::now();
        }
    }
    
    /// Resolve ticket
    pub fn resolve(&mut self, id: Uuid) {
        if let Some(ticket) = self.tickets.get_mut(&id) {
            ticket.status = TicketStatus::Resolved;
            ticket.resolved_at = Some(Utc::now());
            ticket.updated_at = Utc::now();
        }
    }
    
    /// Get open tickets
    pub fn open(&self) -> Vec<&Ticket> {
        self.tickets.values()
            .filter(|t| t.status == TicketStatus::Open)
            .collect()
    }
    
    /// Get tickets by user
    pub fn by_user(&self, user_id: &str) -> Vec<&Ticket> {
        self.tickets.values()
            .filter(|t| t.user_id == user_id)
            .collect()
    }
    
    /// Check SLA breach
    pub fn check_sla_breach(&self, id: Uuid) -> bool {
        if let Some(ticket) = self.tickets.get(&id) {
            ticket.first_response_at.is_none() && Utc::now() > ticket.sla_deadline
        } else {
            false
        }
    }
}

impl Default for SupportManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Support tier configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupportTier {
    pub id: String,
    pub name: String,
    pub price_monthly: f64,
    pub uptime_sla: f64,
    pub response_time_hours: u32,
    pub resolution_time_hours: u32,
    pub support_channels: Vec<String>,
    pub priority_support: bool,
    pub dedicated_manager: bool,
    pub custom_sla: bool,
    pub sla_credits: bool,
    pub features: Vec<String>,
}

impl SupportTier {
    /// Calculate monthly cost per uptime percentage
    pub fn cost_per_nine(&self) -> f64 {
        let nines = self.uptime_sla.log10() / (-1.0 * (1.0 - self.uptime_sla / 100.0).log10());
        self.price_monthly / nines
    }
}

/// Support ticket
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ticket {
    pub id: Uuid,
    pub user_id: String,
    pub tier_id: String,
    pub subject: String,
    pub description: String,
    pub priority: TicketPriority,
    pub status: TicketStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub first_response_at: Option<DateTime<Utc>>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub sla_deadline: DateTime<Utc>,
    pub assigned_to: Option<String>,
    pub messages: Vec<TicketMessage>,
}

impl Ticket {
    /// Check if SLA is breached
    pub fn is_sla_breach(&self) -> bool {
        self.first_response_at.map_or(true, |t| t > self.sla_deadline)
    }
    
    /// Get response time in hours
    pub fn response_time_hours(&self) -> Option<f64> {
        self.first_response_at.map(|t| {
            (t - self.created_at).num_minutes() as f64 / 60.0
        })
    }
}

/// Ticket priority
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TicketPriority {
    Low,
    Normal,
    High,
    Urgent,
}

/// Ticket status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TicketStatus {
    Open,
    InProgress,
    WaitingForCustomer,
    Resolved,
    Closed,
}

/// Ticket message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketMessage {
    pub timestamp: DateTime<Utc>,
    pub from: String,
    pub message: String,
    pub is_staff: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ticket_creation() {
        let mut manager = SupportManager::new();
        let id = manager.create_ticket("user-1", None, "Test", "Test ticket", TicketPriority::Normal);
        assert!(manager.get(id).is_some());
    }
    
    #[test]
    fn test_ticket_response() {
        let mut manager = SupportManager::new();
        let id = manager.create_ticket("user-1", None, "Test", "Test", TicketPriority::Normal);
        manager.respond(id, "staff-1", "Response");
        
        let ticket = manager.get(id).unwrap();
        assert!(ticket.first_response_at.is_some());
    }
}
