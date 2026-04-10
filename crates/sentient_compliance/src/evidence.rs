//! ─── Evidence Collection ───

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use sha2::{Sha256, Digest};
use crate::controls::Control;

/// Evidence record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evidence {
    pub id: Uuid,
    pub control_id: String,
    pub evidence_type: EvidenceType,
    pub title: String,
    pub description: String,
    pub collected_at: DateTime<Utc>,
    pub collected_by: String,
    pub source: EvidenceSource,
    pub content_hash: String,
    pub content_location: Option<String>,
    pub validity_period: Option<u32>, // days
    pub expires_at: Option<DateTime<Utc>>,
    pub verified: bool,
    pub verified_by: Option<String>,
    pub verified_at: Option<DateTime<Utc>>,
}

impl Evidence {
    pub fn new(control_id: &str, title: &str, evidence_type: EvidenceType) -> Self {
        Self {
            id: Uuid::new_v4(),
            control_id: control_id.into(),
            evidence_type,
            title: title.into(),
            description: String::new(),
            collected_at: Utc::now(),
            collected_by: String::new(),
            source: EvidenceSource::Automated,
            content_hash: String::new(),
            content_location: None,
            validity_period: None,
            expires_at: None,
            verified: false,
            verified_by: None,
            verified_at: None,
        }
    }
    
    /// Set content and compute hash
    pub fn set_content(&mut self, content: &[u8]) {
        let mut hasher = Sha256::new();
        hasher.update(content);
        self.content_hash = format!("{:x}", hasher.finalize());
    }
    
    /// Verify evidence
    pub fn verify(&mut self, verifier: &str) {
        self.verified = true;
        self.verified_by = Some(verifier.into());
        self.verified_at = Some(Utc::now());
    }
    
    /// Check if expired
    pub fn is_expired(&self) -> bool {
        if let Some(expires) = self.expires_at {
            expires < Utc::now()
        } else {
            false
        }
    }
}

/// Evidence type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EvidenceType {
    /// Document (policy, procedure)
    Document,
    /// Screenshot
    Screenshot,
    /// Configuration file
    Configuration,
    /// Log file
    Log,
    /// System output
    SystemOutput,
    /// Interview transcript
    Interview,
    /// Test result
    TestResult,
    /// Certificate
    Certificate,
    /// Report
    Report,
    /// Code/configuration
    Artifact,
}

/// Evidence source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EvidenceSource {
    /// Manually uploaded
    Manual,
    /// Automatically collected
    Automated,
    /// API integration
    ApiIntegration,
    /// External audit
    ExternalAudit,
}

/// Evidence collector
pub struct EvidenceCollector {
    collected: Vec<Evidence>,
}

impl EvidenceCollector {
    pub fn new() -> Self {
        Self {
            collected: Vec::new(),
        }
    }
    
    /// Collect evidence for control
    pub async fn collect(&mut self, control: &Control) -> Result<Evidence, EvidenceError> {
        let mut evidence = Evidence::new(
            &control.id,
            &format!("Evidence for {}", control.id),
            EvidenceType::SystemOutput,
        );
        
        // Collect based on control type
        match control.id.as_str() {
            "CC6.1" => self.collect_access_control_evidence(&mut evidence).await?,
            "CC6.6" => self.collect_encryption_evidence(&mut evidence).await?,
            "A1.2" => self.collect_backup_evidence(&mut evidence).await?,
            _ => self.collect_generic_evidence(&mut evidence, control).await?,
        }
        
        evidence.collected_by = "system".into();
        self.collected.push(evidence.clone());
        
        Ok(evidence)
    }
    
    async fn collect_access_control_evidence(&self, evidence: &mut Evidence) -> Result<(), EvidenceError> {
        evidence.description = "Access control configuration and logs".into();
        evidence.evidence_type = EvidenceType::Configuration;
        
        // In production, this would collect actual config
        let sample_content = r#"# Access Control Configuration
mfa_enabled: true
password_policy:
  min_length: 12
  require_uppercase: true
  require_lowercase: true
  require_numbers: true
  require_special: true
  max_age_days: 90
session_policy:
  max_duration_minutes: 480
  idle_timeout_minutes: 30
"#;
        evidence.set_content(sample_content.as_bytes());
        
        Ok(())
    }
    
    async fn collect_encryption_evidence(&self, evidence: &mut Evidence) -> Result<(), EvidenceError> {
        evidence.description = "Encryption configuration and certificates".into();
        evidence.evidence_type = EvidenceType::Certificate;
        
        let sample_content = r#"# Encryption Configuration
tls_version: "1.3"
cipher_suites:
  - TLS_AES_256_GCM_SHA384
  - TLS_CHACHA20_POLY1305_SHA256
certificate_expiry: "2025-12-31"
key_rotation_days: 365
"#;
        evidence.set_content(sample_content.as_bytes());
        
        Ok(())
    }
    
    async fn collect_backup_evidence(&self, evidence: &mut Evidence) -> Result<(), EvidenceError> {
        evidence.description = "Backup configuration and test results".into();
        evidence.evidence_type = EvidenceType::TestResult;
        
        let sample_content = r#"# Backup Test Results
last_backup: "2024-01-15T03:00:00Z"
backup_type: "full"
size_gb: 125
duration_minutes: 45
verification: "passed"
restore_test_date: "2024-01-01"
restore_test_result: "success"
"#;
        evidence.set_content(sample_content.as_bytes());
        
        Ok(())
    }
    
    async fn collect_generic_evidence(&self, evidence: &mut Evidence, control: &Control) -> Result<(), EvidenceError> {
        evidence.description = format!("Evidence for control: {}", control.description);
        evidence.set_content(format!("Control: {}\nStatus: {:?}", control.id, control.status).as_bytes());
        Ok(())
    }
    
    /// Get all collected evidence
    pub fn all(&self) -> &[Evidence] {
        &self.collected
    }
    
    /// Get evidence for control
    pub fn for_control(&self, control_id: &str) -> Vec<&Evidence> {
        self.collected.iter()
            .filter(|e| e.control_id == control_id)
            .collect()
    }
}

impl Default for EvidenceCollector {
    fn default() -> Self {
        Self::new()
    }
}

/// Evidence errors
#[derive(Debug, thiserror::Error)]
pub enum EvidenceError {
    #[error("Collection failed: {0}")]
    CollectionFailed(String),
    
    #[error("Invalid evidence: {0}")]
    InvalidEvidence(String),
    
    #[error("Storage error: {0}")]
    StorageError(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_evidence_creation() {
        let evidence = Evidence::new("CC6.1", "Test", EvidenceType::Document);
        assert_eq!(evidence.control_id, "CC6.1");
    }
    
    #[test]
    fn test_content_hash() {
        let mut evidence = Evidence::new("CC6.1", "Test", EvidenceType::Document);
        evidence.set_content(b"test content");
        assert!(!evidence.content_hash.is_empty());
    }
}
