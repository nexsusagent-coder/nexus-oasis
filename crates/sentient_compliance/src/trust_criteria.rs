//! ─── Trust Service Criteria ───

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Trust Service Criteria
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustServiceCriteria {
    pub security: CriteriaStatus,
    pub availability: CriteriaStatus,
    pub processing_integrity: CriteriaStatus,
    pub confidentiality: CriteriaStatus,
    pub privacy: CriteriaStatus,
}

impl TrustServiceCriteria {
    pub fn new() -> Self {
        Self {
            security: CriteriaStatus::new("Security (Common Criteria)"),
            availability: CriteriaStatus::new("Availability"),
            processing_integrity: CriteriaStatus::new("Processing Integrity"),
            confidentiality: CriteriaStatus::new("Confidentiality"),
            privacy: CriteriaStatus::new("Privacy"),
        }
    }
    
    /// Get overall status
    pub fn overall_status(&self) -> CriteriaLevel {
        let scores = [
            self.security.score(),
            self.availability.score(),
            self.processing_integrity.score(),
            self.confidentiality.score(),
            self.privacy.score(),
        ];
        
        let avg = scores.iter().sum::<f64>() / scores.len() as f64;
        
        if avg >= 90.0 {
            CriteriaLevel::Excellent
        } else if avg >= 75.0 {
            CriteriaLevel::Good
        } else if avg >= 60.0 {
            CriteriaLevel::Fair
        } else if avg >= 40.0 {
            CriteriaLevel::Poor
        } else {
            CriteriaLevel::Critical
        }
    }
}

impl Default for TrustServiceCriteria {
    fn default() -> Self {
        Self::new()
    }
}

/// Criteria status for each trust service
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CriteriaStatus {
    pub name: String,
    pub controls_total: u32,
    pub controls_compliant: u32,
    pub controls_partial: u32,
    pub controls_non_compliant: u32,
    pub last_assessed: Option<DateTime<Utc>>,
    pub issues: u32,
    pub evidence_collected: u32,
}

impl CriteriaStatus {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            controls_total: 0,
            controls_compliant: 0,
            controls_partial: 0,
            controls_non_compliant: 0,
            last_assessed: None,
            issues: 0,
            evidence_collected: 0,
        }
    }
    
    /// Calculate compliance score
    pub fn score(&self) -> f64 {
        if self.controls_total == 0 {
            return 0.0;
        }
        
        let compliant_score = (self.controls_compliant as f64 / self.controls_total as f64) * 100.0;
        let partial_score = (self.controls_partial as f64 / self.controls_total as f64) * 50.0;
        
        compliant_score + partial_score
    }
    
    /// Update counts
    pub fn update(
        &mut self,
        total: u32,
        compliant: u32,
        partial: u32,
        non_compliant: u32,
    ) {
        self.controls_total = total;
        self.controls_compliant = compliant;
        self.controls_partial = partial;
        self.controls_non_compliant = non_compliant;
        self.last_assessed = Some(Utc::now());
    }
}

/// Criteria level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CriteriaLevel {
    Critical,
    Poor,
    Fair,
    Good,
    Excellent,
}

/// SOC 2 Control Points
pub struct ControlPoints;

impl ControlPoints {
    /// Get all control points for Security (CC)
    pub fn security_controls() -> Vec<ControlPoint> {
        vec![
            ControlPoint::new("CC1.1", "Entity Culture", "Organizational culture supports internal control"),
            ControlPoint::new("CC1.2", "Board Independence", "Board oversight of internal control"),
            ControlPoint::new("CC1.3", "Management Philosophy", "Management establishes structure and authority"),
            ControlPoint::new("CC1.4", "Competence", "Personnel competence requirements"),
            ControlPoint::new("CC1.5", "Accountability", "Accountability for internal control responsibilities"),
            
            ControlPoint::new("CC2.1", "Communication", "Internal communication of objectives"),
            ControlPoint::new("CC2.2", "External Communication", "Communication with external parties"),
            ControlPoint::new("CC2.3", "Information Quality", "Quality of information"),
            
            ControlPoint::new("CC3.1", "Risk Assessment", "Entity objectives and risk identification"),
            ControlPoint::new("CC3.2", "Fraud Risk", "Fraud risk assessment"),
            ControlPoint::new("CC3.3", "Change Management", "Assessing changes that impact internal control"),
            
            ControlPoint::new("CC4.1", "Monitoring", "Ongoing and periodic evaluation"),
            ControlPoint::new("CC4.2", "Issue Resolution", "Evaluating and communicating deficiencies"),
            
            ControlPoint::new("CC5.1", "Control Selection", "Selecting and developing controls"),
            ControlPoint::new("CC5.2", "Control Deployment", "Deploying controls"),
            
            ControlPoint::new("CC6.1", "Logical Access", "Logical access security"),
            ControlPoint::new("CC6.2", "Account Management", "System account management"),
            ControlPoint::new("CC6.3", "Network Access", "Network access control"),
            ControlPoint::new("CC6.4", "Data Access", "Data access control"),
            ControlPoint::new("CC6.5", "Input Controls", "Input and output controls"),
            ControlPoint::new("CC6.6", "Transmission", "Transmission protection"),
            ControlPoint::new("CC6.7", "Boundary Protection", "System boundary protection"),
            ControlPoint::new("CC6.8", "Malware Protection", "Malware protection"),
            
            ControlPoint::new("CC7.1", "Vulnerability Management", "Vulnerability identification and remediation"),
            ControlPoint::new("CC7.2", "Change Management", "Change management process"),
            ControlPoint::new("CC7.3", "Incident Response", "Security incident response"),
            ControlPoint::new("CC7.4", "Monitoring", "System monitoring"),
            ControlPoint::new("CC7.5", "Incident Detection", "Security event detection"),
            
            ControlPoint::new("CC8.1", "Change Management", "Change management procedures"),
            ControlPoint::new("CC8.2", "Development", "Development and acquisition"),
            
            ControlPoint::new("CC9.1", "Service Organization", "Service organization controls"),
            ControlPoint::new("CC9.2", "Vendor Management", "Vendor risk management"),
        ]
    }
    
    /// Get all control points for Availability (A)
    pub fn availability_controls() -> Vec<ControlPoint> {
        vec![
            ControlPoint::new("A1.1", "Capacity Management", "System capacity management"),
            ControlPoint::new("A1.2", "Backup", "Backup procedures"),
            ControlPoint::new("A1.3", "Recovery", "Recovery procedures"),
            ControlPoint::new("A1.4", "Business Continuity", "Business continuity planning"),
            ControlPoint::new("A1.5", "Disaster Recovery", "Disaster recovery planning"),
        ]
    }
    
    /// Get all control points for Processing Integrity (PI)
    pub fn processing_integrity_controls() -> Vec<ControlPoint> {
        vec![
            ControlPoint::new("PI1.1", "Data Accuracy", "Data processing accuracy"),
            ControlPoint::new("PI1.2", "Processing Authorization", "Processing authorization"),
            ControlPoint::new("PI1.3", "Input Validation", "Input validation"),
            ControlPoint::new("PI1.4", "Output Validation", "Output validation"),
        ]
    }
    
    /// Get all control points for Confidentiality (C)
    pub fn confidentiality_controls() -> Vec<ControlPoint> {
        vec![
            ControlPoint::new("C1.1", "Information Protection", "Confidential information protection"),
            ControlPoint::new("C1.2", "Data Classification", "Data classification"),
            ControlPoint::new("C1.3", "Encryption", "Encryption at rest and in transit"),
        ]
    }
    
    /// Get all control points for Privacy (P)
    pub fn privacy_controls() -> Vec<ControlPoint> {
        vec![
            ControlPoint::new("P1.1", "Privacy Notice", "Privacy notice provision"),
            ControlPoint::new("P2.1", "Consent", "Consent management"),
            ControlPoint::new("P3.1", "Data Subject Rights", "Data subject rights processing"),
            ControlPoint::new("P4.1", "Data Retention", "Data retention policies"),
            ControlPoint::new("P5.1", "Data Disposal", "Secure data disposal"),
            ControlPoint::new("P6.1", "Data Quality", "Data quality management"),
            ControlPoint::new("P7.1", "Third Party", "Third party privacy controls"),
            ControlPoint::new("P8.1", "Breach Notification", "Breach notification procedures"),
        ]
    }
}

/// Control point definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlPoint {
    pub id: String,
    pub name: String,
    pub description: String,
}

impl ControlPoint {
    pub fn new(id: &str, name: &str, description: &str) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            description: description.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_trust_service_criteria() {
        let criteria = TrustServiceCriteria::new();
        assert!(criteria.security.score() >= 0.0);
    }
    
    #[test]
    fn test_security_controls() {
        let controls = ControlPoints::security_controls();
        assert!(!controls.is_empty());
    }
}
