//! ═══════════════════════════════════════════════════════════════════════════════
//!  OASIS HANDS SETUP - Gelişmiş Kurulum Sistemi
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Kullanıcı onay sistemi ile masaüstü kontrolü kurulumu.
//! - Interaktif Setup Wizard
//! - Otomatik kurulum
//! - Profil yönetimi
//! - Platform testleri
//! - İzin yönetimi
//!
//! ═──────────────────────────────────────────────────────────────────────────────

pub mod wizard;
pub mod config;
pub mod permissions;
pub mod profiles;
pub mod tests;
pub mod approval;

pub use wizard::SetupWizard;
pub use config::{SetupConfig, SetupState};
pub use permissions::{PermissionManager, Permission, PermissionStatus};
pub use profiles::{ProfileManager, SetupProfile, ProfileType};
pub use tests::{SystemTester, TestResult, TestCategory};
pub use approval::{ApprovalManager, ApprovalRequest, ApprovalStatus};

use std::path::PathBuf;

/// Setup modülü sürümü
pub const SETUP_VERSION: &str = "1.0.0";

/// Config dosyası yolu
pub fn config_path() -> PathBuf {
    std::env::var("HOME")
        .map(|h| PathBuf::from(h))
        .unwrap_or_else(|_| PathBuf::from("/root"))
        .join(".config")
        .join("sentient")
        .join("hands_setup.toml")
}

/// Profiler dizini
pub fn profiles_dir() -> PathBuf {
    std::env::var("HOME")
        .map(|h| PathBuf::from(h))
        .unwrap_or_else(|_| PathBuf::from("/root"))
        .join(".config")
        .join("sentient")
        .join("profiles")
}

/// Onay kayıtları dizini
pub fn approvals_dir() -> PathBuf {
    std::env::var("HOME")
        .map(|h| PathBuf::from(h))
        .unwrap_or_else(|_| PathBuf::from("/root"))
        .join(".config")
        .join("sentient")
        .join("approvals")
}
