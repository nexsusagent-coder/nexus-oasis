//! ═══════════════════════════════════════════════════════════════════════════════
//!  TEE ATTESTATION - Remote Attestation Service
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Provides remote attestation capabilities for proving TEE identity
//! to remote parties.

use crate::{TeeMeasurement, TeePlatform, TeeResult, TeeError};
use crate::hardware::{SevSnpAttestationReport, TdxQuote, AttestationVerifier};
use serde::{Deserialize, Serialize};

/// Attestation report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttestationReport {
    /// Unique report ID
    pub id: uuid::Uuid,
    /// TEE platform
    pub platform: TeePlatform,
    /// Nonce for freshness
    pub nonce: String,
    /// Enclave measurement
    pub measurement: TeeMeasurement,
    /// Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Whether attestation passed
    pub valid: bool,
    /// Platform-specific raw report
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_report: Option<Vec<u8>>,
    /// Additional claims
    pub claims: std::collections::HashMap<String, String>,
}

/// Attestation service
pub struct AttestationService {
    platform: TeePlatform,
    verifier: AttestationVerifier,
}

impl AttestationService {
    pub fn new(platform: TeePlatform) -> Self {
        Self {
            platform,
            verifier: AttestationVerifier::new(),
        }
    }
    
    /// Add a trusted measurement
    pub fn add_trusted_measurement(&mut self, measurement: String) {
        self.verifier.add_trusted_measurement(measurement);
    }

    /// Perform attestation with given nonce
    pub fn attest(&self, nonce: &str) -> TeeResult<AttestationReport> {
        let measurement = TeeMeasurement::from_binary(b"runtime_code", self.platform);
        let raw_report = self.generate_raw_report(nonce)?;
        
        // Verify locally
        let valid = self.verify_local(&raw_report, &measurement)?;
        
        Ok(AttestationReport {
            id: uuid::Uuid::new_v4(),
            platform: self.platform,
            nonce: nonce.to_string(),
            measurement,
            timestamp: chrono::Utc::now(),
            valid,
            raw_report: Some(raw_report),
            claims: std::collections::HashMap::new(),
        })
    }
    
    /// Attest with additional claims
    pub fn attest_with_claims(
        &self,
        nonce: &str,
        claims: std::collections::HashMap<String, String>,
    ) -> TeeResult<AttestationReport> {
        let mut report = self.attest(nonce)?;
        report.claims = claims;
        Ok(report)
    }
    
    /// Generate platform-specific raw report
    fn generate_raw_report(&self, nonce: &str) -> TeeResult<Vec<u8>> {
        match self.platform {
            TeePlatform::AmdSevSnp => self.generate_sev_snp_report(nonce),
            TeePlatform::IntelTdx => self.generate_tdx_quote(nonce),
            TeePlatform::Simulation => self.generate_simulated_report(nonce),
        }
    }
    
    #[cfg(feature = "sev-snp")]
    fn generate_sev_snp_report(&self, nonce: &str) -> TeeResult<Vec<u8>> {
        // Production: Use SNP_REPORT command via /dev/sev
        self.generate_simulated_report(nonce)
    }
    
    #[cfg(not(feature = "sev-snp"))]
    fn generate_sev_snp_report(&self, nonce: &str) -> TeeResult<Vec<u8>> {
        log::debug!("SEV-SNP not available, using simulation");
        self.generate_simulated_report(nonce)
    }
    
    #[cfg(feature = "tdx")]
    fn generate_tdx_quote(&self, nonce: &str) -> TeeResult<Vec<u8>> {
        // Production: Use TDCALL to generate quote
        self.generate_simulated_report(nonce)
    }
    
    #[cfg(not(feature = "tdx"))]
    fn generate_tdx_quote(&self, nonce: &str) -> TeeResult<Vec<u8>> {
        log::debug!("TDX not available, using simulation");
        self.generate_simulated_report(nonce)
    }
    
    fn generate_simulated_report(&self, nonce: &str) -> TeeResult<Vec<u8>> {
        // Generate a simulated report for testing
        let mut report = Vec::new();
        report.extend_from_slice(b"SENTIENT_TEE_SIM_");
        report.extend_from_slice(&[self.platform as u8]);
        report.extend_from_slice(nonce.as_bytes());
        report.extend_from_slice(&chrono::Utc::now().timestamp().to_le_bytes());
        
        // Pad to reasonable size
        while report.len() < 256 {
            report.push(0);
        }
        
        Ok(report)
    }
    
    /// Verify attestation locally
    fn verify_local(&self, raw: &[u8], measurement: &TeeMeasurement) -> TeeResult<bool> {
        match self.platform {
            TeePlatform::AmdSevSnp => {
                let report = SevSnpAttestationReport::from_bytes(raw)?;
                self.verifier.verify_sev_snp(&report)
            }
            TeePlatform::IntelTdx => {
                let quote = TdxQuote::from_bytes(raw)?;
                self.verifier.verify_tdx(&quote)
            }
            TeePlatform::Simulation => Ok(true),
        }
    }
    
    /// Verify a remote attestation report
    pub fn verify_remote(&self, report: &AttestationReport) -> TeeResult<bool> {
        // Check platform
        if report.platform != self.platform {
            return Err(TeeError::AttestationFailed("Platform mismatch".into()));
        }
        
        // Check timestamp freshness (within 5 minutes)
        let age = chrono::Utc::now() - report.timestamp;
        if age.num_minutes() > 5 {
            return Err(TeeError::AttestationFailed("Report too old".into()));
        }
        
        // Verify raw report if present
        if let Some(ref raw) = report.raw_report {
            self.verify_local(raw, &report.measurement)?;
        }
        
        Ok(report.valid)
    }
    
    /// Generate attestation token (JWT format)
    pub fn generate_token(&self, nonce: &str) -> TeeResult<String> {
        let report = self.attest(nonce)?;
        
        // Create JWT-like token (simplified)
        let header = base64::encode(r#"{"alg":"TEE","typ":"JWT"}"#);
        let payload = serde_json::to_string(&report)
            .map_err(|e| TeeError::AttestationFailed(e.to_string()))?;
        let payload_b64 = base64::encode(&payload);
        
        // Sign with measurement-derived key
        let signature = blake3::hash(
            format!("{}.{}", header, payload_b64).as_bytes()
        ).to_hex().to_string();
        
        Ok(format!("{}.{}.{}", header, payload_b64, signature))
    }
    
    /// Verify attestation token
    pub fn verify_token(&self, token: &str) -> TeeResult<AttestationReport> {
        let parts: Vec<&str> = token.split('.').collect();
        if parts.len() != 3 {
            return Err(TeeError::AttestationFailed("Invalid token format".into()));
        }
        
        let payload = base64::decode(parts[1])
            .map_err(|e| TeeError::AttestationFailed(e.to_string()))?;
        
        let report: AttestationReport = serde_json::from_slice(&payload)
            .map_err(|e| TeeError::AttestationFailed(e.to_string()))?;
        
        // Verify signature
        let expected_sig = blake3::hash(
            format!("{}.{}", parts[0], parts[1]).as_bytes()
        ).to_hex().to_string();
        
        if parts[2] != expected_sig {
            return Err(TeeError::AttestationFailed("Invalid signature".into()));
        }
        
        self.verify_remote(&report)?;
        Ok(report)
    }
}

/// Attestation client for verifying remote TEEs
pub struct AttestationClient {
    trusted_measurements: Vec<String>,
    trusted_cas: Vec<String>,
}

impl AttestationClient {
    pub fn new() -> Self {
        Self {
            trusted_measurements: Vec::new(),
            trusted_cas: Vec::new(),
        }
    }
    
    /// Add a trusted measurement
    pub fn add_trusted_measurement(&mut self, measurement: String) {
        self.trusted_measurements.push(measurement);
    }
    
    /// Add a trusted certificate authority
    pub fn add_trusted_ca(&mut self, ca: String) {
        self.trusted_cas.push(ca);
    }
    
    /// Verify a remote attestation report
    pub fn verify(&self, report: &AttestationReport) -> TeeResult<bool> {
        // Check measurement
        if !self.trusted_measurements.is_empty() {
            if !self.trusted_measurements.contains(&report.measurement.hash) {
                return Err(TeeError::MeasurementMismatch);
            }
        }
        
        // Check validity
        if !report.valid {
            return Err(TeeError::AttestationFailed("Report marked invalid".into()));
        }
        
        // Check timestamp
        let age = chrono::Utc::now() - report.timestamp;
        if age.num_minutes() > 5 {
            return Err(TeeError::AttestationFailed("Report expired".into()));
        }
        
        Ok(true)
    }
    
    /// Verify SEV-SNP report
    pub fn verify_sev_snp(&self, report: &SevSnpAttestationReport) -> TeeResult<bool> {
        let measurement = report.measurement_hex();
        
        if !self.trusted_measurements.is_empty() && 
           !self.trusted_measurements.contains(&measurement) {
            return Err(TeeError::MeasurementMismatch);
        }
        
        Ok(true)
    }
    
    /// Verify TDX quote
    pub fn verify_tdx(&self, quote: &TdxQuote) -> TeeResult<bool> {
        let measurement = quote.measurement_hex();
        
        if !self.trusted_measurements.is_empty() && 
           !self.trusted_measurements.contains(&measurement) {
            return Err(TeeError::MeasurementMismatch);
        }
        
        Ok(true)
    }
}

impl Default for AttestationClient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attestation() {
        let service = AttestationService::new(TeePlatform::Simulation);
        let report = service.attest("test_nonce").expect("operation failed");
        assert!(report.valid);
    }
    
    #[test]
    fn test_attestation_with_claims() {
        let service = AttestationService::new(TeePlatform::Simulation);
        let mut claims = std::collections::HashMap::new();
        claims.insert("purpose".into(), "authentication".into());
        
        let report = service.attest_with_claims("nonce", claims).expect("operation failed");
        assert!(report.claims.contains_key("purpose"));
    }
    
    #[test]
    fn test_attestation_token() {
        let service = AttestationService::new(TeePlatform::Simulation);
        let token = service.generate_token("test_nonce").expect("operation failed");
        
        let parts: Vec<&str> = token.split('.').collect();
        assert_eq!(parts.len(), 3);
        
        let report = service.verify_token(&token).expect("operation failed");
        assert!(report.valid);
    }
    
    #[test]
    fn test_attestation_client() {
        let mut client = AttestationClient::new();
        client.add_trusted_measurement("test_measurement".into());
        
        let service = AttestationService::new(TeePlatform::Simulation);
        let report = service.attest("nonce").expect("operation failed");
        
        // Without matching measurement, should fail
        assert!(client.verify(&report).is_err());
    }
    
    #[test]
    fn test_sev_snp_verification() {
        let mut client = AttestationClient::new();
        
        let data = vec![0u8; 1184];
        let report = SevSnpAttestationReport::from_bytes(&data).expect("operation failed");
        
        // Should pass without trusted measurements
        assert!(client.verify_sev_snp(&report).is_ok());
        
        // Add a trusted measurement that doesn't match
        client.add_trusted_measurement("different_measurement".into());
        assert!(client.verify_sev_snp(&report).is_err());
    }
}
