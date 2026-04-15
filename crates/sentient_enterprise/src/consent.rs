//! ═══════════════════════════════════════════════════════════════════════════════
//!  GDPR/KVKK Consent Management System
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Privacy compliance system for GDPR (EU) and KVKK (Turkey):
//! - Consent collection and storage
//! - Right to access (Article 15 GDPR / KVKK Art. 11)
//! - Right to rectification (Article 16 GDPR / KVKK Art. 13)
//! - Right to erasure (Article 17 GDPR / KVKK Art. 14)
//! - Right to data portability (Article 20 GDPR)
//! - Right to object (Article 21 GDPR)
//! - Data retention policies

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc, Duration};
use uuid::Uuid;

// ═══════════════════════════════════════════════════════════════════════════════
//  CONSENT TYPES
// ═══════════════════════════════════════════════════════════════════════════════

/// Data processing purpose
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProcessingPurpose {
    /// Service delivery (contractual necessity)
    ServiceDelivery,
    /// Marketing communications
    Marketing,
    /// Analytics and improvement
    Analytics,
    /// Third-party sharing
    ThirdPartySharing,
    /// AI/ML training
    AiTraining,
    /// Security and fraud prevention
    Security,
    /// Legal compliance
    LegalCompliance,
    /// Custom purpose
    Custom(String),
}

impl std::fmt::Display for ProcessingPurpose {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ServiceDelivery => write!(f, "Service Delivery"),
            Self::Marketing => write!(f, "Marketing"),
            Self::Analytics => write!(f, "Analytics"),
            Self::ThirdPartySharing => write!(f, "Third Party Sharing"),
            Self::AiTraining => write!(f, "AI Training"),
            Self::Security => write!(f, "Security"),
            Self::LegalCompliance => write!(f, "Legal Compliance"),
            Self::Custom(s) => write!(f, "{}", s),
        }
    }
}

/// Consent status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConsentStatus {
    /// Consent given
    Granted,
    /// Consent withdrawn
    Withdrawn,
    /// Pending user action
    Pending,
    /// Consent expired
    Expired,
}

/// Consent record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsentRecord {
    /// Unique consent ID
    pub id: String,
    /// User ID
    pub user_id: String,
    /// Processing purpose
    pub purpose: ProcessingPurpose,
    /// Consent status
    pub status: ConsentStatus,
    /// When consent was given
    pub granted_at: Option<DateTime<Utc>>,
    /// When consent was withdrawn
    pub withdrawn_at: Option<DateTime<Utc>>,
    /// IP address when consent given
    pub ip_address: Option<String>,
    /// User agent when consent given
    pub user_agent: Option<String>,
    /// Consent text shown to user
    pub consent_text: String,
    /// Language of consent
    pub language: String,
    /// Version of privacy policy
    pub policy_version: String,
    /// Expiration date (if applicable)
    pub expires_at: Option<DateTime<Utc>>,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

impl ConsentRecord {
    /// Create a new pending consent
    pub fn new(user_id: String, purpose: ProcessingPurpose, consent_text: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            user_id,
            purpose,
            status: ConsentStatus::Pending,
            granted_at: None,
            withdrawn_at: None,
            ip_address: None,
            user_agent: None,
            consent_text,
            language: "en".to_string(),
            policy_version: "1.0".to_string(),
            expires_at: None,
            metadata: HashMap::new(),
        }
    }
    
    /// Grant consent
    pub fn grant(&mut self, ip_address: Option<String>, user_agent: Option<String>) {
        self.status = ConsentStatus::Granted;
        self.granted_at = Some(Utc::now());
        self.ip_address = ip_address;
        self.user_agent = user_agent;
    }
    
    /// Withdraw consent
    pub fn withdraw(&mut self) {
        self.status = ConsentStatus::Withdrawn;
        self.withdrawn_at = Some(Utc::now());
    }
    
    /// Check if consent is valid
    pub fn is_valid(&self) -> bool {
        if self.status != ConsentStatus::Granted {
            return false;
        }
        
        // Check expiration
        if let Some(expires) = self.expires_at {
            if Utc::now() > expires {
                return false;
            }
        }
        
        true
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  DATA SUBJECT REQUESTS (GDPR Articles 15-21)
// ═══════════════════════════════════════════════════════════════════════════════

/// Data subject request type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataSubjectRequest {
    /// Right to access (Art. 15 GDPR / KVKK Art. 11)
    Access,
    /// Right to rectification (Art. 16 GDPR / KVKK Art. 13)
    Rectification { corrections: HashMap<String, String> },
    /// Right to erasure (Art. 17 GDPR / KVKK Art. 14)
    Erasure { reason: ErasureReason },
    /// Right to restriction (Art. 18 GDPR)
    Restriction { purpose: ProcessingPurpose },
    /// Right to data portability (Art. 20 GDPR)
    Portability { format: ExportFormat },
    /// Right to object (Art. 21 GDPR)
    Object { purpose: ProcessingPurpose },
}

/// Reason for erasure request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ErasureReason {
    /// Data no longer necessary
    DataNoLongerNeeded,
    /// Consent withdrawn
    ConsentWithdrawn,
    /// Objection to processing
    Objected,
    /// Unlawful processing
    UnlawfulProcessing,
    /// Legal obligation
    LegalObligation,
    /// Other
    Other(String),
}

/// Export format for portability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExportFormat {
    Json,
    Csv,
    Xml,
    Pdf,
}

/// Data subject request status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSubjectRequestRecord {
    /// Request ID
    pub id: String,
    /// User ID
    pub user_id: String,
    /// Request type
    pub request: DataSubjectRequest,
    /// When request was made
    pub requested_at: DateTime<Utc>,
    /// Current status
    pub status: RequestStatus,
    /// When completed
    pub completed_at: Option<DateTime<Utc>>,
    /// Response data (for access requests)
    pub response_data: Option<String>,
    /// Notes
    pub notes: Option<String>,
}

/// Request processing status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RequestStatus {
    /// Request received
    Received,
    /// Identity verification needed
    IdentityVerification,
    /// Being processed
    Processing,
    /// Completed successfully
    Completed,
    /// Rejected
    Rejected { reason: String },
}

// ═══════════════════════════════════════════════════════════════════════════════
//  DATA RETENTION
// ═══════════════════════════════════════════════════════════════════════════════

/// Data retention policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetentionPolicy {
    /// Data category
    pub category: String,
    /// Retention period in days
    pub retention_days: u32,
    /// Legal basis for retention
    pub legal_basis: String,
    /// Auto-delete after retention
    pub auto_delete: bool,
    /// Anonymize instead of delete
    pub anonymize_instead: bool,
}

impl RetentionPolicy {
    /// Default retention policies
    pub fn defaults() -> Vec<Self> {
        vec![
            Self {
                category: "user_account".to_string(),
                retention_days: 365 * 7, // 7 years
                legal_basis: "Contractual obligation".to_string(),
                auto_delete: false,
                anonymize_instead: true,
            },
            Self {
                category: "transaction_records".to_string(),
                retention_days: 365 * 10, // 10 years
                legal_basis: "Legal compliance (tax)".to_string(),
                auto_delete: false,
                anonymize_instead: false,
            },
            Self {
                category: "marketing_data".to_string(),
                retention_days: 365 * 2, // 2 years
                legal_basis: "Consent".to_string(),
                auto_delete: true,
                anonymize_instead: true,
            },
            Self {
                category: "ai_training_data".to_string(),
                retention_days: 365 * 3, // 3 years
                legal_basis: "Consent".to_string(),
                auto_delete: true,
                anonymize_instead: true,
            },
            Self {
                category: "audit_logs".to_string(),
                retention_days: 365 * 5, // 5 years
                legal_basis: "Legal compliance".to_string(),
                auto_delete: false,
                anonymize_instead: false,
            },
        ]
    }
    
    /// Check if data should be deleted
    pub fn should_delete(&self, created_at: DateTime<Utc>) -> bool {
        let expiry = created_at + Duration::days(self.retention_days as i64);
        Utc::now() > expiry
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  CONSENT MANAGER
// ═══════════════════════════════════════════════════════════════════════════════

/// Consent management error
#[derive(Debug, thiserror::Error)]
pub enum ConsentError {
    #[error("Consent not found: {0}")]
    NotFound(String),
    
    #[error("Consent already granted")]
    AlreadyGranted,
    
    #[error("Consent already withdrawn")]
    AlreadyWithdrawn,
    
    #[error("Invalid request: {0}")]
    InvalidRequest(String),
    
    #[error("Verification required")]
    VerificationRequired,
    
    #[error("Deadline exceeded")]
    DeadlineExceeded,
}

/// Consent manager
pub struct ConsentManager {
    consents: HashMap<String, ConsentRecord>,
    user_consents: HashMap<String, Vec<String>>,
    requests: HashMap<String, DataSubjectRequestRecord>,
    user_requests: HashMap<String, Vec<String>>,
    policies: Vec<RetentionPolicy>,
}

impl ConsentManager {
    /// Create a new consent manager
    pub fn new() -> Self {
        Self {
            consents: HashMap::new(),
            user_consents: HashMap::new(),
            requests: HashMap::new(),
            user_requests: HashMap::new(),
            policies: RetentionPolicy::defaults(),
        }
    }
    
    /// Create a pending consent request
    pub fn create_consent(&mut self, consent: ConsentRecord) -> String {
        let id = consent.id.clone();
        let user_id = consent.user_id.clone();
        
        self.consents.insert(id.clone(), consent);
        self.user_consents.entry(user_id).or_default().push(id.clone());
        
        id
    }
    
    /// Grant consent
    pub fn grant_consent(
        &mut self,
        consent_id: &str,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<(), ConsentError> {
        let consent = self.consents.get_mut(consent_id)
            .ok_or_else(|| ConsentError::NotFound(consent_id.to_string()))?;
        
        if consent.status == ConsentStatus::Granted {
            return Err(ConsentError::AlreadyGranted);
        }
        
        consent.grant(ip_address, user_agent);
        Ok(())
    }
    
    /// Withdraw consent
    pub fn withdraw_consent(&mut self, consent_id: &str) -> Result<(), ConsentError> {
        let consent = self.consents.get_mut(consent_id)
            .ok_or_else(|| ConsentError::NotFound(consent_id.to_string()))?;
        
        if consent.status == ConsentStatus::Withdrawn {
            return Err(ConsentError::AlreadyWithdrawn);
        }
        
        consent.withdraw();
        Ok(())
    }
    
    /// Check if user has valid consent for a purpose
    pub fn has_valid_consent(&self, user_id: &str, purpose: &ProcessingPurpose) -> bool {
        self.user_consents.get(user_id)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| self.consents.get(id))
                    .any(|c| c.purpose == *purpose && c.is_valid())
            })
            .unwrap_or(false)
    }
    
    /// Get all consents for a user
    pub fn get_user_consents(&self, user_id: &str) -> Vec<&ConsentRecord> {
        self.user_consents.get(user_id)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| self.consents.get(id))
                    .collect()
            })
            .unwrap_or_default()
    }
    
    /// Create a data subject request
    pub fn create_request(
        &mut self,
        user_id: String,
        request: DataSubjectRequest,
    ) -> String {
        let record = DataSubjectRequestRecord {
            id: Uuid::new_v4().to_string(),
            user_id: user_id.clone(),
            request,
            requested_at: Utc::now(),
            status: RequestStatus::Received,
            completed_at: None,
            response_data: None,
            notes: None,
        };
        
        let id = record.id.clone();
        self.requests.insert(id.clone(), record);
        self.user_requests.entry(user_id).or_default().push(id.clone());
        
        id
    }
    
    /// Process a data subject request
    pub fn process_request(&mut self, request_id: &str) -> Result<DataSubjectRequestRecord, ConsentError> {
        let request = self.requests.get_mut(request_id)
            .ok_or_else(|| ConsentError::NotFound(request_id.to_string()))?;
        
        // Check deadline (GDPR requires response within 30 days)
        let deadline = request.requested_at + Duration::days(30);
        if Utc::now() > deadline {
            request.status = RequestStatus::Rejected {
                reason: "30-day deadline exceeded".to_string(),
            };
            return Err(ConsentError::DeadlineExceeded);
        }
        
        // Update status based on request type
        match &request.request {
            DataSubjectRequest::Access => {
                // In real implementation, would gather all user data
                request.status = RequestStatus::Processing;
                request.response_data = Some(r#"{"user_data": "..."}} "#.to_string());
                request.status = RequestStatus::Completed;
                request.completed_at = Some(Utc::now());
            }
            DataSubjectRequest::Erasure { .. } => {
                // Mark for erasure
                request.status = RequestStatus::Processing;
                // In real implementation, would delete/anonymize data
                request.status = RequestStatus::Completed;
                request.completed_at = Some(Utc::now());
            }
            DataSubjectRequest::Portability { format } => {
                request.status = RequestStatus::Processing;
                // Export user data in requested format
                let export_data = match format {
                    ExportFormat::Json => r#"{"export": "json"}"#.to_string(),
                    ExportFormat::Csv => "export,csv".to_string(),
                    _ => String::new(),
                };
                request.response_data = Some(export_data);
                request.status = RequestStatus::Completed;
                request.completed_at = Some(Utc::now());
            }
            _ => {
                request.status = RequestStatus::Processing;
                request.status = RequestStatus::Completed;
                request.completed_at = Some(Utc::now());
            }
        }
        
        Ok(request.clone())
    }
    
    /// Get retention policies
    pub fn get_retention_policies(&self) -> &[RetentionPolicy] {
        &self.policies
    }
    
    /// Add a retention policy
    pub fn add_retention_policy(&mut self, policy: RetentionPolicy) {
        self.policies.push(policy);
    }
}

impl Default for ConsentManager {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  CONSENT UI HELPERS
// ═══════════════════════════════════════════════════════════════════════════════

/// Consent banner content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsentBanner {
    pub title: String,
    pub message: String,
    pub purposes: Vec<ConsentPurposeUI>,
    pub accept_all_text: String,
    pub reject_all_text: String,
    pub customize_text: String,
    pub privacy_policy_url: String,
}

/// UI representation of a consent purpose
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsentPurposeUI {
    pub purpose: ProcessingPurpose,
    pub title: String,
    pub description: String,
    pub required: bool,
    pub default_checked: bool,
}

impl ConsentBanner {
    /// Create a default consent banner
    pub fn default_en() -> Self {
        Self {
            title: "Privacy Preferences".to_string(),
            message: "We use cookies and process your data for the following purposes. Please select your preferences.".to_string(),
            purposes: vec![
                ConsentPurposeUI {
                    purpose: ProcessingPurpose::ServiceDelivery,
                    title: "Essential".to_string(),
                    description: "Required for the service to function. Cannot be disabled.".to_string(),
                    required: true,
                    default_checked: true,
                },
                ConsentPurposeUI {
                    purpose: ProcessingPurpose::Analytics,
                    title: "Analytics".to_string(),
                    description: "Help us understand how you use our service so we can improve it.".to_string(),
                    required: false,
                    default_checked: true,
                },
                ConsentPurposeUI {
                    purpose: ProcessingPurpose::Marketing,
                    title: "Marketing".to_string(),
                    description: "Receive personalized marketing communications.".to_string(),
                    required: false,
                    default_checked: false,
                },
                ConsentPurposeUI {
                    purpose: ProcessingPurpose::AiTraining,
                    title: "AI Training".to_string(),
                    description: "Allow your data to be used for training our AI models.".to_string(),
                    required: false,
                    default_checked: false,
                },
            ],
            accept_all_text: "Accept All".to_string(),
            reject_all_text: "Reject All".to_string(),
            customize_text: "Customize".to_string(),
            privacy_policy_url: "/privacy".to_string(),
        }
    }
    
    /// Create Turkish KVKK-compliant banner
    pub fn default_tr() -> Self {
        Self {
            title: "Gizlilik Tercihleri".to_string(),
            message: "KVKK kapsamında kişisel verileriniz aşağıdaki amaçlarla işlenmektedir. Tercihlerinizi seçiniz.".to_string(),
            purposes: vec![
                ConsentPurposeUI {
                    purpose: ProcessingPurpose::ServiceDelivery,
                    title: "Zorunlu".to_string(),
                    description: "Hizmetin çalışması için gereklidir. Devre dışı bırakılamaz.".to_string(),
                    required: true,
                    default_checked: true,
                },
                ConsentPurposeUI {
                    purpose: ProcessingPurpose::Analytics,
                    title: "Analiz".to_string(),
                    description: "Hizmetimizi nasıl kullandığınızı anlamamıza yardımcı olur.".to_string(),
                    required: false,
                    default_checked: true,
                },
                ConsentPurposeUI {
                    purpose: ProcessingPurpose::Marketing,
                    title: "Pazarlama".to_string(),
                    description: "Kişiselleştirilmiş pazarlama iletişimi almak için onay verin.".to_string(),
                    required: false,
                    default_checked: false,
                },
            ],
            accept_all_text: "Tümünü Kabul Et".to_string(),
            reject_all_text: "Tümünü Reddet".to_string(),
            customize_text: "Özelleştir".to_string(),
            privacy_policy_url: "/gizlilik".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_consent_lifecycle() {
        let mut manager = ConsentManager::new();
        
        let consent = ConsentRecord::new(
            "user-123".to_string(),
            ProcessingPurpose::Marketing,
            "I agree to receive marketing emails".to_string(),
        );
        
        let id = manager.create_consent(consent);
        
        // Grant consent
        manager.grant_consent(&id, Some("192.168.1.1".to_string()), None).unwrap();
        
        // Check valid
        assert!(manager.has_valid_consent("user-123", &ProcessingPurpose::Marketing));
        
        // Withdraw
        manager.withdraw_consent(&id).unwrap();
        
        // Check not valid
        assert!(!manager.has_valid_consent("user-123", &ProcessingPurpose::Marketing));
    }
    
    #[test]
    fn test_data_subject_request() {
        let mut manager = ConsentManager::new();
        
        let request_id = manager.create_request(
            "user-123".to_string(),
            DataSubjectRequest::Access,
        );
        
        let result = manager.process_request(&request_id).unwrap();
        
        assert!(matches!(result.status, RequestStatus::Completed));
        assert!(result.response_data.is_some());
    }
    
    #[test]
    fn test_retention_policy() {
        let policies = RetentionPolicy::defaults();
        assert!(!policies.is_empty());
        
        let marketing_policy = policies.iter()
            .find(|p| p.category == "marketing_data")
            .unwrap();
        
        assert_eq!(marketing_policy.retention_days, 365 * 2);
        assert!(marketing_policy.auto_delete);
    }
}
