//! ═══════════════════════════════════════════════════════════════════════════════
//!  Multi-Factor Authentication (MFA) Module
//! ═══════════════════════════════════════════════════════════════════════════════
//! 
//! Supports:
//! - TOTP (Time-based One-Time Password) - Google Authenticator, Authy, etc.
//! - SMS OTP
//! - Email OTP
//! - Backup Codes
//! - Hardware Security Keys (WebAuthn/FIDO2)

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, Duration};
use uuid::Uuid;
use thiserror::Error;
use std::collections::HashMap;
use tokio::sync::RwLock;

// ═══════════════════════════════════════════════════════════════════════════════
//  ERROR TYPES
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Error)]
pub enum MfaError {
    #[error("Invalid OTP code")]
    InvalidCode,
    
    #[error("OTP code expired")]
    CodeExpired,
    
    #[error("Too many failed attempts")]
    TooManyAttempts,
    
    #[error("MFA not enabled for user")]
    NotEnabled,
    
    #[error("MFA already enabled")]
    AlreadyEnabled,
    
    #[error("Invalid secret key")]
    InvalidSecret,
    
    #[error("Backup code already used")]
    BackupCodeUsed,
    
    #[error("No backup codes remaining")]
    NoBackupCodes,
    
    #[error("SMS delivery failed: {0}")]
    SmsFailed(String),
    
    #[error("Email delivery failed: {0}")]
    EmailFailed(String),
    
    #[error("Hardware key error: {0}")]
    HardwareKeyError(String),
    
    #[error("Storage error: {0}")]
    StorageError(String),
    
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
}

// ═══════════════════════════════════════════════════════════════════════════════
//  MFA METHOD TYPES
// ═══════════════════════════════════════════════════════════════════════════════

/// MFA method type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MfaMethod {
    /// Time-based One-Time Password (Google Authenticator, Authy)
    Totp,
    /// SMS One-Time Password
    Sms,
    /// Email One-Time Password
    Email,
    /// Hardware Security Key (FIDO2/WebAuthn)
    HardwareKey,
    /// Backup/Recovery Codes
    BackupCode,
}

impl std::fmt::Display for MfaMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MfaMethod::Totp => write!(f, "TOTP"),
            MfaMethod::Sms => write!(f, "SMS"),
            MfaMethod::Email => write!(f, "Email"),
            MfaMethod::HardwareKey => write!(f, "Hardware Key"),
            MfaMethod::BackupCode => write!(f, "Backup Code"),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  TOTP CONFIGURATION
// ═══════════════════════════════════════════════════════════════════════════════

/// TOTP configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TotpConfig {
    /// Issuer name (shows in authenticator app)
    pub issuer: String,
    /// Algorithm (SHA1, SHA256, SHA512)
    pub algorithm: TotpAlgorithm,
    /// Number of digits (6 or 8)
    pub digits: u32,
    /// Time step in seconds (default: 30)
    pub time_step: u32,
    /// Number of time steps to look ahead/behind for clock drift
    pub skew_tolerance: u32,
}

impl Default for TotpConfig {
    fn default() -> Self {
        Self {
            issuer: "SENTIENT AI".to_string(),
            algorithm: TotpAlgorithm::Sha1,
            digits: 6,
            time_step: 30,
            skew_tolerance: 1,
        }
    }
}

/// TOTP hash algorithm
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum TotpAlgorithm {
    Sha1,
    Sha256,
    Sha512,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  OTP CODE GENERATION
// ═══════════════════════════════════════════════════════════════════════════════

/// OTP code with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OtpCode {
    /// The OTP code
    pub code: String,
    /// When it was generated
    pub generated_at: DateTime<Utc>,
    /// When it expires
    pub expires_at: DateTime<Utc>,
    /// Method used to generate
    pub method: MfaMethod,
    /// Number of failed attempts
    pub attempts: u32,
    /// Maximum attempts allowed
    pub max_attempts: u32,
}

impl OtpCode {
    /// Create a new OTP code
    pub fn new(code: String, ttl_seconds: i64, method: MfaMethod) -> Self {
        let now = Utc::now();
        Self {
            code,
            generated_at: now,
            expires_at: now + Duration::seconds(ttl_seconds),
            method,
            attempts: 0,
            max_attempts: 5,
        }
    }
    
    /// Check if code is expired
    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }
    
    /// Check if too many attempts
    pub fn is_locked(&self) -> bool {
        self.attempts >= self.max_attempts
    }
    
    /// Increment attempts
    pub fn increment_attempts(&mut self) {
        self.attempts += 1;
    }
    
    /// Verify code
    pub fn verify(&mut self, input: &str) -> Result<bool, MfaError> {
        if self.is_expired() {
            return Err(MfaError::CodeExpired);
        }
        
        if self.is_locked() {
            return Err(MfaError::TooManyAttempts);
        }
        
        if self.code == input {
            Ok(true)
        } else {
            self.increment_attempts();
            if self.is_locked() {
                Err(MfaError::TooManyAttempts)
            } else {
                Err(MfaError::InvalidCode)
            }
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  BACKUP CODES
// ═══════════════════════════════════════════════════════════════════════════════

/// Backup/recovery code
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupCode {
    /// The code (hashed for storage)
    pub code_hash: String,
    /// Whether it's been used
    pub used: bool,
    /// When it was used (if applicable)
    pub used_at: Option<DateTime<Utc>>,
}

impl BackupCode {
    /// Generate a new backup code
    pub fn generate() -> (String, Self) {
        // Generate 8-character alphanumeric code
        let code: String = (0..8)
            .map(|_| {
                const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
                let idx = rand::random::<usize>() % CHARSET.len();
                CHARSET[idx] as char
            })
            .collect();
        
        // Hash the code for storage
        let code_hash = Self::hash_code(&code);
        
        (code, Self {
            code_hash,
            used: false,
            used_at: None,
        })
    }
    
    /// Hash a code
    fn hash_code(code: &str) -> String {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(code.as_bytes());
        format!("{:x}", hasher.finalize())
    }
    
    /// Verify a code against the hash
    pub fn verify(&self, code: &str) -> bool {
        Self::hash_code(code) == self.code_hash
    }
    
    /// Mark as used
    pub fn mark_used(&mut self) {
        self.used = true;
        self.used_at = Some(Utc::now());
    }
}

/// Backup codes collection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupCodes {
    /// The backup codes
    pub codes: Vec<BackupCode>,
    /// When they were generated
    pub generated_at: DateTime<Utc>,
}

impl BackupCodes {
    /// Generate a new set of backup codes
    pub fn generate(count: usize) -> (Vec<String>, Self) {
        let mut plain_codes = Vec::new();
        let mut codes = Vec::new();
        
        for _ in 0..count {
            let (plain, code) = BackupCode::generate();
            plain_codes.push(plain);
            codes.push(code);
        }
        
        (plain_codes, Self {
            codes,
            generated_at: Utc::now(),
        })
    }
    
    /// Get remaining (unused) codes count
    pub fn remaining(&self) -> usize {
        self.codes.iter().filter(|c| !c.used).count()
    }
    
    /// Verify and consume a backup code
    pub fn verify_and_use(&mut self, code: &str) -> Result<(), MfaError> {
        for backup_code in &mut self.codes {
            if !backup_code.used && backup_code.verify(code) {
                backup_code.mark_used();
                return Ok(());
            }
        }
        
        if self.remaining() == 0 {
            Err(MfaError::NoBackupCodes)
        } else {
            Err(MfaError::BackupCodeUsed)
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  USER MFA SETTINGS
// ═══════════════════════════════════════════════════════════════════════════════

/// MFA settings for a user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserMfaSettings {
    /// User ID
    pub user_id: String,
    /// Whether MFA is enabled
    pub enabled: bool,
    /// Primary MFA method
    pub primary_method: MfaMethod,
    /// Secondary methods (fallbacks)
    pub secondary_methods: Vec<MfaMethod>,
    /// TOTP secret (base32 encoded)
    pub totp_secret: Option<String>,
    /// TOTP config
    pub totp_config: TotpConfig,
    /// Phone number for SMS
    pub phone_number: Option<String>,
    /// Email for OTP
    pub email: Option<String>,
    /// Backup codes
    pub backup_codes: Option<BackupCodes>,
    /// Hardware key credentials
    pub hardware_keys: Vec<HardwareKeyCredential>,
    /// When MFA was enabled
    pub enabled_at: Option<DateTime<Utc>>,
    /// Last successful verification
    pub last_verified_at: Option<DateTime<Utc>>,
    /// Failed verification attempts
    pub failed_attempts: u32,
    /// Locked until (if applicable)
    pub locked_until: Option<DateTime<Utc>>,
}

impl UserMfaSettings {
    /// Create new MFA settings for a user
    pub fn new(user_id: &str) -> Self {
        Self {
            user_id: user_id.to_string(),
            enabled: false,
            primary_method: MfaMethod::Totp,
            secondary_methods: vec![MfaMethod::BackupCode],
            totp_secret: None,
            totp_config: TotpConfig::default(),
            phone_number: None,
            email: None,
            backup_codes: None,
            hardware_keys: Vec::new(),
            enabled_at: None,
            last_verified_at: None,
            failed_attempts: 0,
            locked_until: None,
        }
    }
    
    /// Check if user is locked out
    pub fn is_locked(&self) -> bool {
        if let Some(locked_until) = self.locked_until {
            Utc::now() < locked_until
        } else {
            false
        }
    }
    
    /// Lock the account for specified minutes
    pub fn lock(&mut self, minutes: i64) {
        self.locked_until = Some(Utc::now() + Duration::minutes(minutes));
    }
    
    /// Unlock the account
    pub fn unlock(&mut self) {
        self.locked_until = None;
        self.failed_attempts = 0;
    }
    
    /// Record failed attempt
    pub fn record_failure(&mut self, max_attempts: u32, lock_minutes: i64) {
        self.failed_attempts += 1;
        if self.failed_attempts >= max_attempts {
            self.lock(lock_minutes);
        }
    }
    
    /// Record successful verification
    pub fn record_success(&mut self) {
        self.failed_attempts = 0;
        self.locked_until = None;
        self.last_verified_at = Some(Utc::now());
    }
    
    /// Generate TOTP secret
    pub fn generate_totp_secret(&mut self) -> String {
        // Generate 20 random bytes (160 bits)
        let secret: Vec<u8> = (0..20)
            .map(|_| rand::random::<u8>())
            .collect();
        
        // Base32 encode
        let encoded = Self::base32_encode(&secret);
        self.totp_secret = Some(encoded.clone());
        encoded
    }
    
    /// Base32 encoding (RFC 4648)
    fn base32_encode(data: &[u8]) -> String {
        const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
        let mut result = String::new();
        let mut bits = 0u32;
        let mut bit_count = 0;
        
        for &byte in data {
            bits = (bits << 8) | (byte as u32);
            bit_count += 8;
            while bit_count >= 5 {
                bit_count -= 5;
                let idx = ((bits >> bit_count) & 0x1F) as usize;
                result.push(ALPHABET[idx] as char);
            }
        }
        
        if bit_count > 0 {
            let idx = ((bits << (5 - bit_count)) & 0x1F) as usize;
            result.push(ALPHABET[idx] as char);
        }
        
        result
    }
    
    /// Get TOTP URI for QR code
    pub fn get_totp_uri(&self, username: &str) -> Option<String> {
        let secret = self.totp_secret.as_ref()?;
        let issuer = &self.totp_config.issuer;
        
        Some(format!(
            "otpauth://totp/{}:{}?secret={}&issuer={}&algorithm={}&digits={}&period={}",
            urlencoding::encode(issuer),
            urlencoding::encode(username),
            secret,
            urlencoding::encode(issuer),
            match self.totp_config.algorithm {
                TotpAlgorithm::Sha1 => "SHA1",
                TotpAlgorithm::Sha256 => "SHA256",
                TotpAlgorithm::Sha512 => "SHA512",
            },
            self.totp_config.digits,
            self.totp_config.time_step,
        ))
    }
    
    /// Generate new backup codes
    pub fn generate_backup_codes(&mut self, count: usize) -> Vec<String> {
        let (plain_codes, backup_codes) = BackupCodes::generate(count);
        self.backup_codes = Some(backup_codes);
        plain_codes
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  HARDWARE KEY SUPPORT
// ═══════════════════════════════════════════════════════════════════════════════

/// Hardware security key credential (WebAuthn/FIDO2)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareKeyCredential {
    /// Credential ID
    pub credential_id: String,
    /// Public key (COSE format, base64)
    pub public_key: String,
    /// Sign count (for clone detection)
    pub sign_count: u32,
    /// AAGUID (authenticator identifier)
    pub aaguid: String,
    /// Friendly name
    pub name: String,
    /// When it was registered
    pub registered_at: DateTime<Utc>,
    /// Last used
    pub last_used_at: Option<DateTime<Utc>>,
    /// Whether it's a platform authenticator
    pub is_platform: bool,
    /// Transports available
    pub transports: Vec<String>,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  MFA MANAGER
// ═══════════════════════════════════════════════════════════════════════════════

/// MFA configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MfaConfig {
    /// Enable MFA system-wide
    pub enabled: bool,
    /// Require MFA for all users
    pub mandatory: bool,
    /// Allowed MFA methods
    pub allowed_methods: Vec<MfaMethod>,
    /// TOTP configuration
    pub totp_config: TotpConfig,
    /// SMS OTP length
    pub sms_otp_length: u32,
    /// SMS OTP TTL in seconds
    pub sms_otp_ttl: i64,
    /// Email OTP length
    pub email_otp_length: u32,
    /// Email OTP TTL in seconds
    pub email_otp_ttl: i64,
    /// Number of backup codes to generate
    pub backup_code_count: usize,
    /// Max failed attempts before lockout
    pub max_failed_attempts: u32,
    /// Lockout duration in minutes
    pub lockout_duration_minutes: i64,
    /// Remember device duration in days (0 = disabled)
    pub remember_device_days: i64,
}

impl Default for MfaConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            mandatory: false,
            allowed_methods: vec![
                MfaMethod::Totp,
                MfaMethod::Sms,
                MfaMethod::Email,
                MfaMethod::BackupCode,
            ],
            totp_config: TotpConfig::default(),
            sms_otp_length: 6,
            sms_otp_ttl: 300,
            email_otp_length: 6,
            email_otp_ttl: 600,
            backup_code_count: 10,
            max_failed_attempts: 5,
            lockout_duration_minutes: 15,
            remember_device_days: 30,
        }
    }
}

/// Pending OTP verification (SMS/Email)
#[derive(Debug, Clone)]
struct PendingOtp {
    code: OtpCode,
    destination: String,
}

/// Trusted device
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustedDevice {
    /// Device ID
    pub device_id: String,
    /// User agent
    pub user_agent: Option<String>,
    /// IP address
    pub ip_address: Option<String>,
    /// When it was trusted
    pub trusted_at: DateTime<Utc>,
    /// When trust expires
    pub expires_at: DateTime<Utc>,
}

impl TrustedDevice {
    /// Check if trust is still valid
    pub fn is_valid(&self) -> bool {
        Utc::now() < self.expires_at
    }
}

/// MFA Manager - handles all MFA operations
pub struct MfaManager {
    config: MfaConfig,
    user_settings: RwLock<HashMap<String, UserMfaSettings>>,
    pending_otps: RwLock<HashMap<String, PendingOtp>>,
    trusted_devices: RwLock<HashMap<String, Vec<TrustedDevice>>>,
}

impl MfaManager {
    /// Create a new MFA manager
    pub fn new(config: MfaConfig) -> Self {
        Self {
            config,
            user_settings: RwLock::new(HashMap::new()),
            pending_otps: RwLock::new(HashMap::new()),
            trusted_devices: RwLock::new(HashMap::new()),
        }
    }
    
    /// Check if MFA is enabled system-wide
    pub fn is_enabled(&self) -> bool {
        self.config.enabled
    }
    
    /// Check if MFA is mandatory
    pub fn is_mandatory(&self) -> bool {
        self.config.mandatory
    }
    
    /// Get allowed MFA methods
    pub fn allowed_methods(&self) -> &[MfaMethod] {
        &self.config.allowed_methods
    }
    
    // ═══════════════════════════════════════════════════════════════════════════
    //  USER SETTINGS MANAGEMENT
    // ═══════════════════════════════════════════════════════════════════════════
    
    /// Get user MFA settings
    pub async fn get_user_settings(&self, user_id: &str) -> UserMfaSettings {
        let settings = self.user_settings.read().await;
        settings.get(user_id).cloned().unwrap_or_else(|| UserMfaSettings::new(user_id))
    }
    
    /// Save user MFA settings
    pub async fn save_user_settings(&self, settings: UserMfaSettings) {
        let mut user_settings = self.user_settings.write().await;
        user_settings.insert(settings.user_id.clone(), settings);
    }
    
    /// Check if user has MFA enabled
    pub async fn is_user_mfa_enabled(&self, user_id: &str) -> bool {
        let settings = self.get_user_settings(user_id).await;
        settings.enabled
    }
    
    // ═══════════════════════════════════════════════════════════════════════════
    //  TOTP OPERATIONS
    // ═══════════════════════════════════════════════════════════════════════════
    
    /// Setup TOTP for a user
    pub async fn setup_totp(&self, user_id: &str, username: &str) -> Result<TotpSetup, MfaError> {
        let mut settings = self.get_user_settings(user_id).await;
        
        if settings.enabled && settings.totp_secret.is_some() {
            return Err(MfaError::AlreadyEnabled);
        }
        
        // Generate secret
        let secret = settings.generate_totp_secret();
        
        // Generate backup codes
        let backup_codes = settings.generate_backup_codes(self.config.backup_code_count);
        
        // Get TOTP URI for QR code
        let uri = settings.get_totp_uri(username).ok_or(MfaError::InvalidSecret)?;
        
        // Save settings
        self.save_user_settings(settings).await;
        
        Ok(TotpSetup {
            secret,
            uri,
            backup_codes,
        })
    }
    
    /// Verify TOTP code and enable MFA
    pub async fn verify_and_enable_totp(&self, user_id: &str, code: &str) -> Result<(), MfaError> {
        let mut settings = self.get_user_settings(user_id).await;
        
        if settings.enabled {
            return Err(MfaError::AlreadyEnabled);
        }
        
        // Verify the code
        self.verify_totp_code(&settings, code)?;
        
        // Enable MFA
        settings.enabled = true;
        settings.enabled_at = Some(Utc::now());
        settings.primary_method = MfaMethod::Totp;
        settings.record_success();
        
        self.save_user_settings(settings).await;
        
        Ok(())
    }
    
    /// Verify TOTP code
    fn verify_totp_code(&self, settings: &UserMfaSettings, code: &str) -> Result<(), MfaError> {
        let secret = settings.totp_secret.as_ref().ok_or(MfaError::NotEnabled)?;
        let config = &settings.totp_config;
        
        // Decode base32 secret
        let secret_bytes = Self::base32_decode(secret)?;
        
        // Get current time step
        let time_step = Utc::now().timestamp() as u64 / config.time_step as u64;
        
        // Check current and adjacent time steps (for clock drift)
        let skew = config.skew_tolerance as i64;
        for offset in -skew..=skew {
            let step = (time_step as i64 + offset) as u64;
            let expected = Self::generate_totp(&secret_bytes, step, config)?;
            
            if Self::constant_time_compare(&expected, code) {
                return Ok(());
            }
        }
        
        Err(MfaError::InvalidCode)
    }
    
    /// Generate TOTP code
    fn generate_totp(secret: &[u8], time_step: u64, config: &TotpConfig) -> Result<String, MfaError> {
        // Convert time step to bytes (big endian)
        let time_bytes = time_step.to_be_bytes();
        
        // HMAC based on algorithm
        let hmac_result = match config.algorithm {
            TotpAlgorithm::Sha1 => {
                use hmac::{Hmac, Mac};
                use sha1::Sha1;
                let mut mac = Hmac::<Sha1>::new_from_slice(secret)
                    .map_err(|_| MfaError::InvalidSecret)?;
                mac.update(&time_bytes);
                mac.finalize().into_bytes().to_vec()
            }
            TotpAlgorithm::Sha256 => {
                use hmac::{Hmac, Mac};
                use sha2::Sha256;
                let mut mac = Hmac::<Sha256>::new_from_slice(secret)
                    .map_err(|_| MfaError::InvalidSecret)?;
                mac.update(&time_bytes);
                mac.finalize().into_bytes().to_vec()
            }
            TotpAlgorithm::Sha512 => {
                use hmac::{Hmac, Mac};
                use sha2::Sha512;
                let mut mac = Hmac::<Sha512>::new_from_slice(secret)
                    .map_err(|_| MfaError::InvalidSecret)?;
                mac.update(&time_bytes);
                mac.finalize().into_bytes().to_vec()
            }
        };
        
        // Dynamic truncation
        let offset = (hmac_result.last().unwrap() & 0x0F) as usize;
        let code: u32 = ((hmac_result[offset] as u32 & 0x7F) << 24)
            | ((hmac_result[offset + 1] as u32) << 16)
            | ((hmac_result[offset + 2] as u32) << 8)
            | (hmac_result[offset + 3] as u32);
        
        let code = code % 10u32.pow(config.digits);
        
        Ok(format!("{:0width$}", code, width = config.digits as usize))
    }
    
    /// Base32 decode
    fn base32_decode(encoded: &str) -> Result<Vec<u8>, MfaError> {
        const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
        let mut decoded = Vec::new();
        let mut bits = 0u32;
        let mut bit_count = 0;
        
        for c in encoded.chars() {
            let val = ALPHABET.iter()
                .position(|&b| b as char == c.to_ascii_uppercase())
                .ok_or(MfaError::InvalidSecret)? as u32;
            
            bits = (bits << 5) | val;
            bit_count += 5;
            
            while bit_count >= 8 {
                bit_count -= 8;
                decoded.push((bits >> bit_count) as u8);
            }
        }
        
        Ok(decoded)
    }
    
    /// Constant-time string comparison
    fn constant_time_compare(a: &str, b: &str) -> bool {
        if a.len() != b.len() {
            return false;
        }
        
        let a_bytes = a.as_bytes();
        let b_bytes = b.as_bytes();
        let mut result = 0u8;
        
        for i in 0..a.len() {
            result |= a_bytes[i] ^ b_bytes[i];
        }
        
        result == 0
    }
    
    // ═══════════════════════════════════════════════════════════════════════════
    //  SMS OTP OPERATIONS
    // ═══════════════════════════════════════════════════════════════════════════
    
    /// Send SMS OTP
    pub async fn send_sms_otp(&self, user_id: &str, phone_number: &str) -> Result<String, MfaError> {
        let mut settings = self.get_user_settings(user_id).await;
        settings.phone_number = Some(phone_number.to_string());
        
        // Generate OTP code
        let code: String = (0..self.config.sms_otp_length)
            .map(|_| rand::random::<u8>() % 10)
            .map(|d| char::from_digit(d as u32, 10).unwrap())
            .collect();
        
        let otp = OtpCode::new(code.clone(), self.config.sms_otp_ttl, MfaMethod::Sms);
        let otp_id = Uuid::new_v4().to_string();
        
        // Store pending OTP
        let mut pending = self.pending_otps.write().await;
        pending.insert(otp_id.clone(), PendingOtp {
            code: otp,
            destination: phone_number.to_string(),
        });
        
        self.save_user_settings(settings).await;
        
        // In production, send SMS here via Twilio, AWS SNS, etc.
        // For now, we return the OTP ID
        log::info!("SMS OTP sent to {}: code={}", phone_number, code);
        
        Ok(otp_id)
    }
    
    // ═══════════════════════════════════════════════════════════════════════════
    //  EMAIL OTP OPERATIONS
    // ═══════════════════════════════════════════════════════════════════════════
    
    /// Send Email OTP
    pub async fn send_email_otp(&self, user_id: &str, email: &str) -> Result<String, MfaError> {
        let mut settings = self.get_user_settings(user_id).await;
        settings.email = Some(email.to_string());
        
        // Generate OTP code
        let code: String = (0..self.config.email_otp_length)
            .map(|_| rand::random::<u8>() % 10)
            .map(|d| char::from_digit(d as u32, 10).unwrap())
            .collect();
        
        let otp = OtpCode::new(code.clone(), self.config.email_otp_ttl, MfaMethod::Email);
        let otp_id = Uuid::new_v4().to_string();
        
        // Store pending OTP
        let mut pending = self.pending_otps.write().await;
        pending.insert(otp_id.clone(), PendingOtp {
            code: otp,
            destination: email.to_string(),
        });
        
        self.save_user_settings(settings).await;
        
        // In production, send email here via SMTP, SendGrid, etc.
        log::info!("Email OTP sent to {}: code={}", email, code);
        
        Ok(otp_id)
    }
    
    // ═══════════════════════════════════════════════════════════════════════════
    //  VERIFICATION
    // ═══════════════════════════════════════════════════════════════════════════
    
    /// Verify MFA code
    pub async fn verify(&self, user_id: &str, code: &str, method: MfaMethod) -> Result<MfaVerificationResult, MfaError> {
        let mut settings = self.get_user_settings(user_id).await;
        
        // Check if locked
        if settings.is_locked() {
            return Err(MfaError::TooManyAttempts);
        }
        
        // Check if MFA is enabled
        if !settings.enabled {
            return Err(MfaError::NotEnabled);
        }
        
        let result = match method {
            MfaMethod::Totp => {
                self.verify_totp_code(&settings, code)?;
                MfaVerificationResult::Success { method }
            }
            MfaMethod::BackupCode => {
                let mut backup_codes = settings.backup_codes.clone().ok_or(MfaError::NoBackupCodes)?;
                backup_codes.verify_and_use(code)?;
                settings.backup_codes = Some(backup_codes);
                MfaVerificationResult::Success { method }
            }
            MfaMethod::Sms | MfaMethod::Email => {
                // Verify pending OTP
                let pending = self.pending_otps.read().await;
                let phone = settings.phone_number.as_ref();
                let email = settings.email.as_ref();
                let otp_entry = pending.iter()
                    .find(|(_, p)| {
                        phone.map_or(false, |pn| p.destination == *pn) ||
                        email.map_or(false, |e| p.destination == *e)
                    })
                    .ok_or(MfaError::CodeExpired)?;
                
                let mut otp = otp_entry.1.code.clone();
                let otp_key = otp_entry.0.clone();
                drop(pending);
                
                otp.verify(code)?;
                
                // Remove used OTP
                let mut pending = self.pending_otps.write().await;
                pending.remove(&otp_key);
                
                MfaVerificationResult::Success { method }
            }
            MfaMethod::HardwareKey => {
                // Hardware key verification would be done via WebAuthn
                // This is a placeholder
                return Err(MfaError::HardwareKeyError("Hardware key verification not implemented".into()));
            }
        };
        
        // Success - record and save
        settings.record_success();
        self.save_user_settings(settings).await;
        
        Ok(result)
    }
    
    // ═══════════════════════════════════════════════════════════════════════════
    //  TRUSTED DEVICES
    // ═══════════════════════════════════════════════════════════════════════════
    
    /// Trust a device
    pub async fn trust_device(&self, user_id: &str, device_info: DeviceInfo) -> String {
        if self.config.remember_device_days <= 0 {
            return String::new();
        }
        
        let device_id = Uuid::new_v4().to_string();
        let now = Utc::now();
        
        let device = TrustedDevice {
            device_id: device_id.clone(),
            user_agent: device_info.user_agent,
            ip_address: device_info.ip_address,
            trusted_at: now,
            expires_at: now + Duration::days(self.config.remember_device_days),
        };
        
        let mut devices = self.trusted_devices.write().await;
        devices.entry(user_id.to_string())
            .or_insert_with(Vec::new)
            .push(device);
        
        device_id
    }
    
    /// Check if device is trusted
    pub async fn is_device_trusted(&self, user_id: &str, device_id: &str) -> bool {
        let devices = self.trusted_devices.read().await;
        if let Some(user_devices) = devices.get(user_id) {
            user_devices.iter()
                .find(|d| d.device_id == device_id && d.is_valid())
                .is_some()
        } else {
            false
        }
    }
    
    /// Revoke trusted device
    pub async fn revoke_device(&self, user_id: &str, device_id: &str) {
        let mut devices = self.trusted_devices.write().await;
        if let Some(user_devices) = devices.get_mut(user_id) {
            user_devices.retain(|d| d.device_id != device_id);
        }
    }
    
    /// Revoke all trusted devices for user
    pub async fn revoke_all_devices(&self, user_id: &str) {
        let mut devices = self.trusted_devices.write().await;
        devices.remove(user_id);
    }
    
    // ═══════════════════════════════════════════════════════════════════════════
    //  DISABLE MFA
    // ═══════════════════════════════════════════════════════════════════════════
    
    /// Disable MFA for a user
    pub async fn disable_mfa(&self, user_id: &str, verification_code: &str) -> Result<(), MfaError> {
        // Verify the code first
        let settings = self.get_user_settings(user_id).await;
        self.verify_totp_code(&settings, verification_code)?;
        
        // Remove all settings
        let mut user_settings = self.user_settings.write().await;
        user_settings.remove(user_id);
        
        // Remove trusted devices
        self.revoke_all_devices(user_id).await;
        
        Ok(())
    }
}

/// TOTP setup result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TotpSetup {
    /// Base32 encoded secret
    pub secret: String,
    /// OTP URI for QR code
    pub uri: String,
    /// Backup codes
    pub backup_codes: Vec<String>,
}

/// MFA verification result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MfaVerificationResult {
    /// Verification successful
    Success { method: MfaMethod },
    /// New device registered (for hardware keys)
    DeviceRegistered { device_id: String },
}

/// Device information
#[derive(Debug, Clone)]
pub struct DeviceInfo {
    pub user_agent: Option<String>,
    pub ip_address: Option<String>,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_backup_code_generation() {
        let (plain_codes, backup_codes) = BackupCodes::generate(10);
        
        assert_eq!(plain_codes.len(), 10);
        assert_eq!(backup_codes.codes.len(), 10);
        assert_eq!(backup_codes.remaining(), 10);
        
        // Each code should be 8 characters
        for code in &plain_codes {
            assert_eq!(code.len(), 8);
        }
    }
    
    #[test]
    fn test_backup_code_verification() {
        let (plain_codes, mut backup_codes) = BackupCodes::generate(5);
        let code = &plain_codes[0];
        
        // Verify should work
        assert!(backup_codes.verify_and_use(code).is_ok());
        assert_eq!(backup_codes.remaining(), 4);
        
        // Second use should fail
        assert!(backup_codes.verify_and_use(code).is_err());
    }
    
    #[test]
    fn test_mfa_settings() {
        let mut settings = UserMfaSettings::new("user123");
        
        assert!(!settings.enabled);
        assert!(settings.totp_secret.is_none());
        
        let secret = settings.generate_totp_secret();
        assert!(!secret.is_empty());
        assert!(settings.totp_secret.is_some());
    }
    
    #[tokio::test]
    async fn test_mfa_manager_totp_setup() {
        let manager = MfaManager::new(MfaConfig::default());
        
        let setup = manager.setup_totp("user123", "testuser").await.unwrap();
        
        assert!(!setup.secret.is_empty());
        assert!(setup.uri.contains("otpauth://totp"));
        assert_eq!(setup.backup_codes.len(), 10);
    }
    
    #[test]
    fn test_otp_code() {
        let mut code = OtpCode::new("123456".to_string(), 300, MfaMethod::Sms);
        
        // Should verify correctly
        assert!(code.verify("123456").is_ok());
        
        // Wrong code should fail
        let mut code2 = OtpCode::new("123456".to_string(), 300, MfaMethod::Sms);
        assert!(code2.verify("654321").is_err());
        assert_eq!(code2.attempts, 1);
    }
    
    #[test]
    fn test_base32_encoding() {
        let secret = vec![0x48, 0x65, 0x6c, 0x6c, 0x6f]; // "Hello"
        let encoded = UserMfaSettings::base32_encode(&secret);
        
        // Should be valid base32
        assert!(encoded.chars().all(|c| c.is_ascii_uppercase() || c.is_ascii_digit()));
    }
}
