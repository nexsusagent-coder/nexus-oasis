//! ═══════════════════════════════════════════════════════════════════════════════
//!  SENTIENT SOC 2 Compliance Framework
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//!  Features:
//!  - SOC 2 Type I and Type II audit framework
//!  - All 5 Trust Service Criteria controls
//!  - Automated evidence collection
//!  - Continuous monitoring
//!  - Audit trail and logging
//!  - Compliance reporting

pub mod controls;
pub mod audit;
pub mod evidence;
pub mod monitor;
pub mod report;
pub mod trust_criteria;

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::collections::HashMap;

pub use controls::{Control, ControlStatus, ControlCategory};
pub use audit::{AuditLog, AuditEvent, AuditSeverity};
pub use evidence::{Evidence, EvidenceCollector};
pub use monitor::{ComplianceMonitor, ComplianceStatus};
pub use report::{ComplianceReport, ReportType};
pub use trust_criteria::{TrustServiceCriteria, CriteriaStatus};

/// SOC 2 Compliance Manager
pub struct ComplianceManager {
    /// Control implementations
    controls: HashMap<String, Control>,
    
    /// Audit log storage
    audit_log: Vec<AuditEvent>,
    
    /// Evidence collector
    evidence: EvidenceCollector,
    
    /// Compliance monitor
    monitor: ComplianceMonitor,
    
    /// Current certification status
    certification: Option<Soc2Certification>,
}

impl ComplianceManager {
    pub fn new() -> Self {
        Self {
            controls: Self::initialize_controls(),
            audit_log: Vec::new(),
            evidence: EvidenceCollector::new(),
            monitor: ComplianceMonitor::new(),
            certification: None,
        }
    }
    
    /// Initialize all SOC 2 controls
    fn initialize_controls() -> HashMap<String, Control> {
        use ControlCategory::*;
        
        let mut controls = HashMap::new();
        
        // Security Controls (Common Criteria)
        controls.insert("CC6.1".into(), Control::new(
            "CC6.1",
            "Logical and Physical Access",
            Security,
            "Logical access to systems is restricted to authorized users.",
        ));
        
        controls.insert("CC6.2".into(), Control::new(
            "CC6.2",
            "System Account Management",
            Security,
            "System accounts are managed to prevent unauthorized access.",
        ));
        
        controls.insert("CC6.3".into(), Control::new(
            "CC6.3",
            "Network Access Control",
            Security,
            "Network access is controlled and monitored.",
        ));
        
        controls.insert("CC6.4".into(), Control::new(
            "CC6.4",
            "Data Access Control",
            Security,
            "Access to data is restricted based on need-to-know basis.",
        ));
        
        controls.insert("CC6.5".into(), Control::new(
            "CC6.5",
            "Input/Output Controls",
            Security,
            "Input and output data is validated and controlled.",
        ));
        
        controls.insert("CC6.6".into(), Control::new(
            "CC6.6",
            "Transmission Controls",
            Security,
            "Data transmission is protected using encryption.",
        ));
        
        controls.insert("CC6.7".into(), Control::new(
            "CC6.7",
            "Boundary Protection",
            Security,
            "System boundaries are protected by firewalls and DMZ.",
        ));
        
        controls.insert("CC6.8".into(), Control::new(
            "CC6.8",
            "Malware Protection",
            Security,
            "Malware protection is implemented and maintained.",
        ));
        
        controls.insert("CC7.1".into(), Control::new(
            "CC7.1",
            "Vulnerability Management",
            Security,
            "Vulnerabilities are identified and remediated.",
        ));
        
        controls.insert("CC7.2".into(), Control::new(
            "CC7.2",
            "Change Management",
            Security,
            "Changes are authorized, tested, and approved.",
        ));
        
        // Availability Controls
        controls.insert("A1.1".into(), Control::new(
            "A1.1",
            "Capacity Management",
            Availability,
            "System capacity is monitored and managed.",
        ));
        
        controls.insert("A1.2".into(), Control::new(
            "A1.2",
            "Backup and Recovery",
            Availability,
            "Data backups are performed and tested regularly.",
        ));
        
        controls.insert("A1.3".into(), Control::new(
            "A1.3",
            "Recovery Procedures",
            Availability,
            "Recovery procedures are documented and tested.",
        ));
        
        // Processing Integrity Controls
        controls.insert("PI1.1".into(), Control::new(
            "PI1.1",
            "Data Processing Accuracy",
            ProcessingIntegrity,
            "Data processing is accurate and complete.",
        ));
        
        controls.insert("PI1.2".into(), Control::new(
            "PI1.2",
            "Processing Authorization",
            ProcessingIntegrity,
            "Processing is authorized and valid.",
        ));
        
        // Confidentiality Controls
        controls.insert("C1.1".into(), Control::new(
            "C1.1",
            "Confidential Information Protection",
            Confidentiality,
            "Confidential information is identified and protected.",
        ));
        
        controls.insert("C1.2".into(), Control::new(
            "C1.2",
            "Data Classification",
            Confidentiality,
            "Data is classified based on sensitivity.",
        ));
        
        // Privacy Controls
        controls.insert("P1.1".into(), Control::new(
            "P1.1",
            "Privacy Notice",
            Privacy,
            "Privacy notice is provided to data subjects.",
        ));
        
        controls.insert("P2.1".into(), Control::new(
            "P2.1",
            "Consent Management",
            Privacy,
            "Consent is obtained for data collection and processing.",
        ));
        
        controls.insert("P3.1".into(), Control::new(
            "P3.1",
            "Data Subject Rights",
            Privacy,
            "Data subject rights are respected and processed.",
        ));
        
        controls.insert("P4.1".into(), Control::new(
            "P4.1",
            "Data Retention",
            Privacy,
            "Data retention policies are implemented.",
        ));
        
        controls.insert("P5.1".into(), Control::new(
            "P5.1",
            "Data Disposal",
            Privacy,
            "Data is securely disposed when no longer needed.",
        ));
        
        controls
    }
    
    /// Log audit event
    pub fn log_event(&mut self, event: AuditEvent) {
        self.audit_log.push(event.clone());
        self.monitor.process_event(&event);
    }
    
    /// Update control status
    pub fn update_control(&mut self, control_id: &str, status: ControlStatus) {
        if let Some(control) = self.controls.get_mut(control_id) {
            control.status = status;
            control.last_assessed = Some(Utc::now());
        }
    }
    
    /// Collect evidence for control
    pub async fn collect_evidence(&mut self, control_id: &str) -> Result<Evidence, ComplianceError> {
        let control = self.controls.get(control_id)
            .ok_or_else(|| ComplianceError::ControlNotFound(control_id.into()))?;
        
        let evidence = self.evidence.collect(control).await?;
        Ok(evidence)
    }
    
    /// Generate compliance report
    pub fn generate_report(&self, report_type: ReportType) -> ComplianceReport {
        ComplianceReport::new(
            report_type,
            &self.controls,
            &self.audit_log,
            &self.certification,
        )
    }
    
    /// Get compliance score (0-100)
    pub fn compliance_score(&self) -> f64 {
        let total = self.controls.len();
        if total == 0 {
            return 0.0;
        }
        
        let compliant = self.controls.values()
            .filter(|c| c.status == ControlStatus::Compliant)
            .count();
        
        (compliant as f64 / total as f64) * 100.0
    }
    
    /// Get controls by category
    pub fn controls_by_category(&self, category: ControlCategory) -> Vec<&Control> {
        self.controls.values()
            .filter(|c| c.category == category)
            .collect()
    }
    
    /// Get all controls
    pub fn all_controls(&self) -> &HashMap<String, Control> {
        &self.controls
    }
    
    /// Start certification process
    pub fn start_certification(&mut self, cert_type: CertificationType) -> Soc2Certification {
        let certification = Soc2Certification {
            id: Uuid::new_v4(),
            cert_type,
            status: CertificationStatus::InProgress,
            start_date: Utc::now(),
            expected_completion: Utc::now() + chrono::Duration::days(90),
            completed_date: None,
            auditor: None,
            scope: vec!["SENTIENT OS Platform".into()],
            controls_addressed: self.controls.keys().cloned().collect(),
        };
        
        self.certification = Some(certification.clone());
        certification
    }
    
    /// Check continuous monitoring status
    pub fn check_monitoring(&self) -> &ComplianceStatus {
        self.monitor.status()
    }
}

impl Default for ComplianceManager {
    fn default() -> Self {
        Self::new()
    }
}

/// SOC 2 Certification record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Soc2Certification {
    pub id: Uuid,
    pub cert_type: CertificationType,
    pub status: CertificationStatus,
    pub start_date: DateTime<Utc>,
    pub expected_completion: DateTime<Utc>,
    pub completed_date: Option<DateTime<Utc>>,
    pub auditor: Option<String>,
    pub scope: Vec<String>,
    pub controls_addressed: Vec<String>,
}

/// Certification type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CertificationType {
    /// Point-in-time audit
    Type1,
    /// Period audit (6-12 months)
    Type2,
}

/// Certification status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CertificationStatus {
    NotStarted,
    InProgress,
    UnderReview,
    Certified,
    Expired,
    Revoked,
}

/// Compliance errors
#[derive(Debug, thiserror::Error)]
pub enum ComplianceError {
    #[error("Control not found: {0}")]
    ControlNotFound(String),
    
    #[error("Evidence collection failed: {0}")]
    EvidenceCollectionFailed(String),
    
    #[error("Audit error: {0}")]
    AuditError(String),
    
    #[error("Compliance check failed: {0}")]
    ComplianceCheckFailed(String),
    
    #[error("Database error: {0}")]
    DatabaseError(String),
}

impl From<evidence::EvidenceError> for ComplianceError {
    fn from(e: evidence::EvidenceError) -> Self {
        ComplianceError::EvidenceCollectionFailed(e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_compliance_manager() {
        let manager = ComplianceManager::new();
        assert!(manager.compliance_score() >= 0.0);
    }
    
    #[test]
    fn test_controls_initialized() {
        let manager = ComplianceManager::new();
        assert!(manager.all_controls().len() > 20);
    }
    
    #[test]
    fn test_compliance_report() {
        let manager = ComplianceManager::new();
        let report = manager.generate_report(ReportType::Summary);
        assert!(report.summary.controls_total >= 0);
    }
}
