//! Approval - Onay Sistemi

use serde::{Deserialize, Serialize};
use crate::setup::approvals_dir;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApprovalStatus {
    Approved,
    Denied,
    Pending,
    Expired,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalRequest {
    pub id: String,
    pub action: String,
    pub description: String,
    pub risk_level: RiskLevel,
    pub status: ApprovalStatus,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug)]
pub struct ApprovalManager {
    pending: Vec<ApprovalRequest>,
    approved: Vec<ApprovalRequest>,
}

impl ApprovalManager {
    pub fn new() -> Self {
        Self { pending: vec![], approved: vec![] }
    }
    
    pub fn request(&mut self, action: &str, desc: &str, risk: RiskLevel) -> String {
        let id = format!("apr_{}", chrono::Utc::now().timestamp());
        
        let req = ApprovalRequest {
            id: id.clone(),
            action: action.into(),
            description: desc.into(),
            risk_level: risk,
            status: ApprovalStatus::Pending,
            created_at: chrono::Utc::now(),
            expires_at: Some(chrono::Utc::now() + chrono::Duration::seconds(30)),
        };
        
        self.pending.push(req);
        id
    }
    
    pub fn approve(&mut self, id: &str) -> bool {
        if let Some(pos) = self.pending.iter().position(|r| r.id == id) {
            let mut req = self.pending.remove(pos);
            req.status = ApprovalStatus::Approved;
            self.approved.push(req);
            return true;
        }
        false
    }
    
    pub fn deny(&mut self, id: &str) -> bool {
        if let Some(pos) = self.pending.iter().position(|r| r.id == id) {
            self.pending[pos].status = ApprovalStatus::Denied;
            return true;
        }
        false
    }
    
    pub fn is_approved(&self, action: &str) -> bool {
        self.approved.iter().any(|r| r.action == action)
    }
    
    pub fn pending_count(&self) -> usize {
        self.pending.len()
    }
    
    pub fn save_approved(&self) -> Result<(), std::io::Error> {
        let dir = approvals_dir();
        std::fs::create_dir_all(&dir)?;
        let path = dir.join("approved.toml");
        let s = toml::to_string_pretty(&self.approved)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        std::fs::write(&path, s)
    }
    
    pub fn load_approved(&mut self) -> Result<(), std::io::Error> {
        let path = approvals_dir().join("approved.toml");
        if path.exists() {
            let s = std::fs::read_to_string(&path)?;
            self.approved = toml::from_str(&s)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        }
        Ok(())
    }
}

impl Default for ApprovalManager {
    fn default() -> Self { Self::new() }
}
