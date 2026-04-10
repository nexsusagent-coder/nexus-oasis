//! ─── SOC 2 Controls ───

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Control implementation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Control {
    /// Control ID (e.g., CC6.1, A1.1)
    pub id: String,
    
    /// Control name
    pub name: String,
    
    /// Control category
    pub category: ControlCategory,
    
    /// Control description
    pub description: String,
    
    /// Implementation details
    pub implementation: Option<String>,
    
    /// Control status
    pub status: ControlStatus,
    
    /// Risk level
    pub risk_level: RiskLevel,
    
    /// Owner
    pub owner: Option<String>,
    
    /// Last assessment date
    pub last_assessed: Option<DateTime<Utc>>,
    
    /// Next assessment date
    pub next_assessment: Option<DateTime<Utc>>,
    
    /// Evidence references
    pub evidence_refs: Vec<String>,
    
    /// Test procedures
    pub test_procedures: Vec<TestProcedure>,
    
    /// Issues identified
    pub issues: Vec<ControlIssue>,
}

impl Control {
    pub fn new(id: &str, name: &str, category: ControlCategory, description: &str) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            category,
            description: description.into(),
            implementation: None,
            status: ControlStatus::NotAssessed,
            risk_level: RiskLevel::Medium,
            owner: None,
            last_assessed: None,
            next_assessment: None,
            evidence_refs: Vec::new(),
            test_procedures: Vec::new(),
            issues: Vec::new(),
        }
    }
    
    /// Set implementation
    pub fn implement(&mut self, details: &str, owner: &str) {
        self.implementation = Some(details.into());
        self.owner = Some(owner.into());
        self.status = ControlStatus::Implemented;
    }
    
    /// Add evidence reference
    pub fn add_evidence(&mut self, evidence_id: &str) {
        self.evidence_refs.push(evidence_id.into());
    }
    
    /// Add test procedure
    pub fn add_test_procedure(&mut self, procedure: TestProcedure) {
        self.test_procedures.push(procedure);
    }
    
    /// Report issue
    pub fn report_issue(&mut self, issue: ControlIssue) {
        let severity = issue.severity;
        self.issues.push(issue);
        if severity == IssueSeverity::High || severity == IssueSeverity::Critical {
            self.status = ControlStatus::NonCompliant;
        }
    }
}

/// Control category (Trust Service Criteria)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ControlCategory {
    /// Security (Common Criteria)
    Security,
    /// Availability
    Availability,
    /// Processing Integrity
    ProcessingIntegrity,
    /// Confidentiality
    Confidentiality,
    /// Privacy
    Privacy,
}

impl std::fmt::Display for ControlCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ControlCategory::Security => write!(f, "Security"),
            ControlCategory::Availability => write!(f, "Availability"),
            ControlCategory::ProcessingIntegrity => write!(f, "Processing Integrity"),
            ControlCategory::Confidentiality => write!(f, "Confidentiality"),
            ControlCategory::Privacy => write!(f, "Privacy"),
        }
    }
}

/// Control status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ControlStatus {
    /// Not yet assessed
    NotAssessed,
    /// Implemented but not tested
    Implemented,
    /// Tested and compliant
    Compliant,
    /// Not compliant
    NonCompliant,
    /// Partially implemented
    Partial,
    /// Not applicable
    NotApplicable,
}

/// Risk level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

/// Test procedure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestProcedure {
    pub id: String,
    pub name: String,
    pub description: String,
    pub test_type: TestType,
    pub frequency: TestFrequency,
    pub last_performed: Option<DateTime<Utc>>,
    pub result: Option<TestResult>,
}

/// Test type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TestType {
    /// Observation of process
    Observation,
    /// Document review
    Inspection,
    /// Inquiry with personnel
    Inquiry,
    /// Technical testing
    TechnicalTest,
    /// Sampling test
    Sampling,
}

/// Test frequency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TestFrequency {
    Daily,
    Weekly,
    Monthly,
    Quarterly,
    Annually,
    AdHoc,
}

/// Test result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TestResult {
    Pass,
    Fail { reason: String },
    Partial { details: String },
    NotTested,
}

/// Control issue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlIssue {
    pub id: String,
    pub title: String,
    pub description: String,
    pub severity: IssueSeverity,
    pub status: IssueStatus,
    pub created_at: DateTime<Utc>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub remediation: Option<String>,
}

/// Issue severity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IssueSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Issue status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IssueStatus {
    Open,
    InProgress,
    Remediated,
    Closed,
    Accepted,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_control_creation() {
        let control = Control::new(
            "CC6.1",
            "Access Control",
            ControlCategory::Security,
            "Test control",
        );
        assert_eq!(control.id, "CC6.1");
        assert_eq!(control.status, ControlStatus::NotAssessed);
    }
    
    #[test]
    fn test_control_implementation() {
        let mut control = Control::new(
            "CC6.1",
            "Access Control",
            ControlCategory::Security,
            "Test",
        );
        control.implement("MFA enabled", "Security Team");
        assert_eq!(control.status, ControlStatus::Implemented);
    }
}
