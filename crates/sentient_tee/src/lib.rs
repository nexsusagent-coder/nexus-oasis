//! ═══════════════════════════════════════════════════════════════════════════════
//!  SENTIENT TEE - Trusted Execution Environment (Enterprise Grade 2026)
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! AMD SEV-SNP ve Intel TDX uyumlu güvenli yürütme ortamı.
//!
//! ## Desteklenen Platformlar:
//! - AMD SEV-SNP (Secure Encrypted Virtualization - Secure Nested Paging)
//! - Intel TDX (Trust Domain Extensions)
//! - Simülasyon modu (geliştirme için)

// Suppress warnings
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(deprecated)]
#![allow(ambiguous_glob_reexports)]
//!
//! ## Güvenlik Özellikleri:
//! - Memory encryption
//! - Remote attestation
//! - Sealed storage
//! - Secure channels

pub mod attestation;
pub mod enclave;
pub mod hardware;
pub mod monitor;
pub mod sealing;

pub use attestation::*;
pub use enclave::*;
pub use hardware::*;
pub use monitor::*;
pub use sealing::*;

use sentient_common::error::SENTIENTError;
use serde::{Deserialize, Serialize};

// ═══════════════════════════════════════════════════════════════════════════════
//  TEE PLATFORM CONFIGURATION
// ═══════════════════════════════════════════════════════════════════════════════

/// TEE Platform type
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum TeePlatform {
    /// AMD SEV-SNP - Confidential computing with VM isolation
    AmdSevSnp,
    /// Intel TDX - Hardware-isolated trust domains
    IntelTdx,
    /// Simulation mode - For development/testing
    #[default]
    Simulation,
}

impl std::fmt::Display for TeePlatform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TeePlatform::AmdSevSnp => write!(f, "AMD SEV-SNP"),
            TeePlatform::IntelTdx => write!(f, "Intel TDX"),
            TeePlatform::Simulation => write!(f, "Simulation"),
        }
    }
}

/// TEE Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeeConfig {
    /// Target TEE platform
    pub platform: TeePlatform,
    /// Enable remote attestation
    pub attestation_enabled: bool,
    /// Enable memory encryption
    pub memory_encryption: bool,
    /// Enable secure boot verification
    pub secure_boot: bool,
    /// Maximum enclave memory in MB
    pub max_enclave_memory_mb: u64,
    /// Attestation provider URL (for cloud TEEs)
    pub attestation_provider: Option<String>,
    /// Debug mode (reduces security for development)
    pub debug_mode: bool,
    /// Enable migration blocker (SEV-SNP)
    pub migration_blocker: bool,
}

impl Default for TeeConfig {
    fn default() -> Self {
        Self {
            platform: TeePlatform::default(),
            attestation_enabled: true,
            memory_encryption: true,
            secure_boot: true,
            max_enclave_memory_mb: 1024,
            attestation_provider: None,
            debug_mode: false,
            migration_blocker: true,
        }
    }
}

impl TeeConfig {
    /// Create config for AMD SEV-SNP
    pub fn sev_snp() -> Self {
        Self {
            platform: TeePlatform::AmdSevSnp,
            migration_blocker: true,
            ..Default::default()
        }
    }
    
    /// Create config for Intel TDX
    pub fn tdx() -> Self {
        Self {
            platform: TeePlatform::IntelTdx,
            ..Default::default()
        }
    }
    
    /// Create config for simulation mode
    pub fn simulation() -> Self {
        Self {
            platform: TeePlatform::Simulation,
            debug_mode: true,
            ..Default::default()
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  ERROR HANDLING
// ═══════════════════════════════════════════════════════════════════════════════

/// TEE Error
#[derive(Debug, thiserror::Error)]
pub enum TeeError {
    #[error("Attestation failed: {0}")]
    AttestationFailed(String),
    
    #[error("Measurement mismatch")]
    MeasurementMismatch,
    
    #[error("Enclave creation failed: {0}")]
    EnclaveCreationFailed(String),
    
    #[error("Sealing failed: {0}")]
    SealingFailed(String),
    
    #[error("Unsealing failed: {0}")]
    UnsealingFailed(String),
    
    #[error("TEE not available")]
    TeeNotAvailable,
    
    #[error("Security violation: {0}")]
    SecurityViolation(String),
    
    #[error("Platform not supported: {0}")]
    PlatformNotSupported(String),
    
    #[error("Key derivation failed: {0}")]
    KeyDerivationFailed(String),
    
    #[error("Memory allocation failed: {0}")]
    MemoryAllocationFailed(String),
    
    #[error("IO error: {0}")]
    IoError(String),
}

impl From<std::io::Error> for TeeError {
    fn from(e: std::io::Error) -> Self {
        TeeError::IoError(e.to_string())
    }
}

impl From<TeeError> for SENTIENTError {
    fn from(e: TeeError) -> Self {
        SENTIENTError::Core(format!("SENTIENT_TEE: {}", e))
    }
}

/// Result type for TEE operations
pub type TeeResult<T> = Result<T, TeeError>;

// ═══════════════════════════════════════════════════════════════════════════════
//  MEASUREMENT & STATUS
// ═══════════════════════════════════════════════════════════════════════════════

/// TEE Measurement - Cryptographic hash of enclave identity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeeMeasurement {
    /// Hash of enclave code and data
    pub hash: String,
    /// Hash algorithm used
    pub algorithm: String,
    /// Platform that generated this measurement
    pub platform: TeePlatform,
    /// When measurement was taken
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl TeeMeasurement {
    pub fn from_binary(data: &[u8], platform: TeePlatform) -> Self {
        let hash = blake3::hash(data).to_hex().to_string();
        Self {
            hash,
            algorithm: "blake3".into(),
            platform,
            timestamp: chrono::Utc::now(),
        }
    }
    
    /// Verify this measurement matches expected
    pub fn verify(&self, expected: &str) -> bool {
        self.hash == expected
    }
}

/// TEE Status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeeStatus {
    /// Whether TEE is available on this system
    pub available: bool,
    /// Whether TEE is initialized
    pub initialized: bool,
    /// Active platform
    pub platform: TeePlatform,
    /// Current security level
    pub security_level: TeeSecurityLevel,
}

/// Security level of TEE
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum TeeSecurityLevel {
    /// Hardware-backed security (SEV-SNP or TDX)
    Hardware,
    /// Software simulation
    #[default]
    Simulated,
    /// No TEE available
    None,
}

impl TeeSecurityLevel {
    pub fn is_hardware(&self) -> bool {
        matches!(self, TeeSecurityLevel::Hardware)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  HELPER FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════════

/// Detect available TEE platform
pub fn detect_platform() -> TeePlatform {
    // Check for AMD SEV-SNP
    #[cfg(target_os = "linux")]
    {
        // Check SEV-SNP via kernel module
        if std::path::Path::new("/sys/module/kvm_amd/parameters/sev_snp").exists() {
            if let Ok(content) = std::fs::read_to_string("/sys/module/kvm_amd/parameters/sev_snp") {
                if content.trim() == "Y" || content.trim() == "1" {
                    log::info!("🔍 Detected AMD SEV-SNP support");
                    return TeePlatform::AmdSevSnp;
                }
            }
        }
        
        // Check TDX via kernel module
        if std::path::Path::new("/sys/module/kvm_intel/parameters/tdx").exists() {
            if let Ok(content) = std::fs::read_to_string("/sys/module/kvm_intel/parameters/tdx") {
                if content.trim() == "Y" || content.trim() == "1" {
                    log::info!("🔍 Detected Intel TDX support");
                    return TeePlatform::IntelTdx;
                }
            }
        }
        
        // Check CPUID for SEV support (fallback)
        if let Ok(cpuinfo) = std::fs::read_to_string("/proc/cpuinfo") {
            if cpuinfo.contains("sev") {
                log::debug!("CPU supports SEV, checking for SNP...");
            }
        }
    }
    
    #[cfg(target_os = "windows")]
    {
        // Windows VBS (Virtualization Based Security) check
        // Could check registry or WMI for VBS status
        log::debug!("Windows platform - VBS check not implemented");
    }
    
    log::info!("🔍 No hardware TEE detected, using simulation mode");
    TeePlatform::Simulation
}

/// Check if TEE is available on this system
pub fn is_tee_available() -> bool {
    let platform = detect_platform();
    !matches!(platform, TeePlatform::Simulation)
}

/// Get detailed TEE capability information
pub fn get_tee_capabilities() -> TeeCapabilities {
    let platform = detect_platform();
    
    TeeCapabilities {
        platform,
        hardware_available: !matches!(platform, TeePlatform::Simulation),
        attestation_supported: true,
        sealing_supported: true,
        secure_channels: true,
        memory_encryption: !matches!(platform, TeePlatform::Simulation),
        migration_support: matches!(platform, TeePlatform::AmdSevSnp),
    }
}

/// TEE capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeeCapabilities {
    pub platform: TeePlatform,
    pub hardware_available: bool,
    pub attestation_supported: bool,
    pub sealing_supported: bool,
    pub secure_channels: bool,
    pub memory_encryption: bool,
    pub migration_support: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tee_config_default() {
        let config = TeeConfig::default();
        assert_eq!(config.platform, TeePlatform::Simulation);
        assert!(config.attestation_enabled);
    }

    #[test]
    fn test_tee_config_platforms() {
        let sev = TeeConfig::sev_snp();
        assert_eq!(sev.platform, TeePlatform::AmdSevSnp);
        
        let tdx = TeeConfig::tdx();
        assert_eq!(tdx.platform, TeePlatform::IntelTdx);
    }

    #[test]
    fn test_tee_measurement() {
        let measurement = TeeMeasurement::from_binary(b"test", TeePlatform::Simulation);
        assert!(!measurement.hash.is_empty());
        assert!(measurement.verify(&measurement.hash));
    }

    #[test]
    fn test_security_level() {
        assert!(TeeSecurityLevel::Hardware.is_hardware());
        assert!(!TeeSecurityLevel::Simulated.is_hardware());
    }
}
