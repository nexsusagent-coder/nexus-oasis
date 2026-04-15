//! ═══════════════════════════════════════════════════════════════════════════════
//!  Password Policy Enforcement Module
//! ═══════════════════════════════════════════════════════════════════════════════
//! 
//! Features:
//! - Configurable password complexity rules
//! - Password strength scoring
//! - Common password blacklist
//! - Password history (prevent reuse)
//! - Password expiration
//! - Account lockout after failed attempts

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, Duration, Datelike};
use std::collections::{HashMap, HashSet};
use tokio::sync::RwLock;
use thiserror::Error;
use sha2::{Sha256, Digest};

// ═══════════════════════════════════════════════════════════════════════════════
//  ERROR TYPES
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Error)]
pub enum PasswordError {
    #[error("Password is too short (minimum {0} characters)")]
    TooShort(usize),
    
    #[error("Password is too long (maximum {0} characters)")]
    TooLong(usize),
    
    #[error("Password must contain at least {0} uppercase letter(s)")]
    MissingUppercase(usize),
    
    #[error("Password must contain at least {0} lowercase letter(s)")]
    MissingLowercase(usize),
    
    #[error("Password must contain at least {0} digit(s)")]
    MissingDigit(usize),
    
    #[error("Password must contain at least {0} special character(s)")]
    MissingSpecial(usize),
    
    #[error("Password is too weak: {0}")]
    TooWeak(String),
    
    #[error("Password has been used before")]
    PasswordReused,
    
    #[error("Password is in the blacklist of common passwords")]
    PasswordBlacklisted,
    
    #[error("Password contains user information")]
    ContainsUserInfo,
    
    #[error("Password contains keyboard pattern")]
    KeyboardPattern,
    
    #[error("Password contains repeated characters")]
    RepeatedCharacters,
    
    #[error("Password has expired")]
    PasswordExpired,
    
    #[error("Password must be changed")]
    MustChange,
    
    #[error("Account is locked")]
    AccountLocked,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  PASSWORD STRENGTH
// ═══════════════════════════════════════════════════════════════════════════════

/// Password strength level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PasswordStrength {
    /// Very weak (score 0-20)
    VeryWeak,
    /// Weak (score 21-40)
    Weak,
    /// Fair (score 41-60)
    Fair,
    /// Good (score 61-80)
    Good,
    /// Strong (score 81-100)
    Strong,
    /// Very strong (score 100+)
    VeryStrong,
}

impl PasswordStrength {
    /// Get strength from score
    pub fn from_score(score: u32) -> Self {
        match score {
            0..=20 => Self::VeryWeak,
            21..=40 => Self::Weak,
            41..=60 => Self::Fair,
            61..=80 => Self::Good,
            81..=99 => Self::Strong,
            _ => Self::VeryStrong,
        }
    }
    
    /// Get display name
    pub fn display(&self) -> &'static str {
        match self {
            Self::VeryWeak => "Very Weak",
            Self::Weak => "Weak",
            Self::Fair => "Fair",
            Self::Good => "Good",
            Self::Strong => "Strong",
            Self::VeryStrong => "Very Strong",
        }
    }
    
    /// Get CSS class for styling
    pub fn css_class(&self) -> &'static str {
        match self {
            Self::VeryWeak => "strength-very-weak",
            Self::Weak => "strength-weak",
            Self::Fair => "strength-fair",
            Self::Good => "strength-good",
            Self::Strong => "strength-strong",
            Self::VeryStrong => "strength-very-strong",
        }
    }
    
    /// Check if password meets minimum strength
    pub fn meets_minimum(&self, minimum: PasswordStrength) -> bool {
        self.score() >= minimum.score()
    }
    
    /// Get numeric score
    pub fn score(&self) -> u32 {
        match self {
            Self::VeryWeak => 0,
            Self::Weak => 21,
            Self::Fair => 41,
            Self::Good => 61,
            Self::Strong => 81,
            Self::VeryStrong => 100,
        }
    }
}

impl std::fmt::Display for PasswordStrength {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display())
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  PASSWORD POLICY CONFIGURATION
// ═══════════════════════════════════════════════════════════════════════════════

/// Password policy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordPolicy {
    // ─── Length Requirements ───
    /// Minimum password length
    pub min_length: usize,
    /// Maximum password length
    pub max_length: usize,
    
    // ─── Character Requirements ───
    /// Minimum uppercase letters
    pub min_uppercase: usize,
    /// Minimum lowercase letters
    pub min_lowercase: usize,
    /// Minimum digits
    pub min_digits: usize,
    /// Minimum special characters
    pub min_special: usize,
    
    // ─── Strength Requirements ───
    /// Minimum required strength
    pub min_strength: PasswordStrength,
    
    // ─── Forbidden Patterns ───
    /// Check for keyboard patterns (qwerty, asdf, etc.)
    pub forbid_keyboard_patterns: bool,
    /// Check for repeated characters (aaa, 111, etc.)
    pub forbid_repeated_chars: bool,
    /// Check for user info in password
    pub forbid_user_info: bool,
    /// Check against common password blacklist
    pub use_blacklist: bool,
    
    // ─── History ───
    /// Number of previous passwords to remember
    pub history_count: usize,
    /// Days until password expires (0 = never)
    pub max_age_days: u32,
    /// Days before expiration to warn user
    pub expiration_warning_days: u32,
    
    // ─── Lockout ───
    /// Maximum failed attempts before lockout
    pub max_failed_attempts: u32,
    /// Lockout duration in minutes
    pub lockout_duration_minutes: u32,
    /// Reset failed attempts after minutes
    pub failed_attempts_reset_minutes: u32,
    
    // ─── Special Rules ───
    /// Force password change on first login
    pub force_change_on_first_login: bool,
    /// Force password change after reset
    pub force_change_after_reset: bool,
    /// Allow password change only after days
    pub min_password_age_days: u32,
}

impl Default for PasswordPolicy {
    fn default() -> Self {
        Self {
            min_length: 12,
            max_length: 128,
            min_uppercase: 1,
            min_lowercase: 1,
            min_digits: 1,
            min_special: 1,
            min_strength: PasswordStrength::Good,
            forbid_keyboard_patterns: true,
            forbid_repeated_chars: true,
            forbid_user_info: true,
            use_blacklist: true,
            history_count: 12,
            max_age_days: 90,
            expiration_warning_days: 14,
            max_failed_attempts: 5,
            lockout_duration_minutes: 30,
            failed_attempts_reset_minutes: 15,
            force_change_on_first_login: true,
            force_change_after_reset: true,
            min_password_age_days: 1,
        }
    }
}

impl PasswordPolicy {
    /// Create a lenient policy (less strict)
    pub fn lenient() -> Self {
        Self {
            min_length: 8,
            min_uppercase: 1,
            min_lowercase: 1,
            min_digits: 1,
            min_special: 0,
            min_strength: PasswordStrength::Fair,
            history_count: 5,
            max_age_days: 0,
            ..Self::default()
        }
    }
    
    /// Create a strict policy (enterprise-grade)
    pub fn strict() -> Self {
        Self {
            min_length: 16,
            min_uppercase: 2,
            min_lowercase: 2,
            min_digits: 2,
            min_special: 2,
            min_strength: PasswordStrength::Strong,
            history_count: 24,
            max_age_days: 60,
            max_failed_attempts: 3,
            lockout_duration_minutes: 60,
            ..Self::default()
        }
    }
    
    /// Create NIST compliant policy
    pub fn nist() -> Self {
        Self {
            min_length: 8,
            max_length: 64,
            min_uppercase: 0,
            min_lowercase: 0,
            min_digits: 0,
            min_special: 0,
            min_strength: PasswordStrength::Fair,
            forbid_keyboard_patterns: true,
            forbid_repeated_chars: true,
            forbid_user_info: true,
            use_blacklist: true,
            history_count: 0, // NIST says history not required with good policies
            max_age_days: 0, // NIST says no forced expiration
            ..Self::default()
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  PASSWORD VALIDATOR
// ═══════════════════════════════════════════════════════════════════════════════

/// Password analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordAnalysis {
    /// Overall strength
    pub strength: PasswordStrength,
    /// Numeric score (0-100+)
    pub score: u32,
    /// Password length
    pub length: usize,
    /// Uppercase count
    pub uppercase_count: usize,
    /// Lowercase count
    pub lowercase_count: usize,
    /// Digit count
    pub digit_count: usize,
    /// Special character count
    pub special_count: usize,
    /// Has keyboard pattern
    pub has_keyboard_pattern: bool,
    /// Has repeated characters
    pub has_repeated_chars: bool,
    /// Entropy estimate (bits)
    pub entropy: f64,
    /// Time to crack estimate
    pub crack_time_estimate: String,
    /// Suggestions for improvement
    pub suggestions: Vec<String>,
    /// Warnings
    pub warnings: Vec<String>,
}

/// Password validator
pub struct PasswordValidator {
    policy: PasswordPolicy,
    blacklist: HashSet<String>,
    keyboard_patterns: Vec<&'static str>,
}

impl PasswordValidator {
    /// Create a new password validator
    pub fn new(policy: PasswordPolicy) -> Self {
        Self {
            policy,
            blacklist: Self::load_default_blacklist(),
            keyboard_patterns: Self::load_keyboard_patterns(),
        }
    }
    
    /// Create validator with default policy
    pub fn default_validator() -> Self {
        Self::new(PasswordPolicy::default())
    }
    
    /// Load default password blacklist
    fn load_default_blacklist() -> HashSet<String> {
        let common_passwords = [
            // Top 100 most common passwords
            "password", "123456", "12345678", "qwerty", "abc123",
            "monkey", "1234567", "letmein", "trustno1", "dragon",
            "baseball", "iloveyou", "master", "sunshine", "ashley",
            "bailey", "shadow", "123123", "654321", "superman",
            "qazwsx", "michael", "football", "password1", "password123",
            "welcome", "jesus", "ninja", "mustang", "password2",
            "hosting", "computer", "internet", "service", "admin",
            "administrator", "root", "toor", "test", "guest",
            "login", "pass", "passw0rd", "p@ssw0rd", "p@ssword",
            "p@ss", "pa$$word", "pa55word", "passw0rd!", "Password1",
            // Turkish common passwords
            "sifre", "sifre123", "parola", "parola123", "gizli",
            "gizli123", "sifrem", "parolam", "selam", "merhaba",
        ];
        
        common_passwords.iter().map(|s| s.to_lowercase()).collect()
    }
    
    /// Load keyboard patterns
    fn load_keyboard_patterns() -> Vec<&'static str> {
        vec![
            // QWERTY row patterns
            "qwerty", "qwertyuiop", "asdf", "asdfghjkl", "zxcv", "zxcvbnm",
            // Reverse patterns
            "poiuytrewq", "lkjhgfdsa", "mnbvcxz",
            // Number sequences
            "123456", "234567", "345678", "456789", "567890",
            "098765", "987654", "876543", "765432", "654321",
            // Common patterns
            "qazwsx", "qweasd", "zasxsd", "1qaz2wsx", "!qaz2wsx",
            // Turkish keyboard
            "fghjklşi", "qwertyuıopğü",
        ]
    }
    
    /// Analyze a password
    pub fn analyze(&self, password: &str, user_info: Option<&UserInfo>) -> PasswordAnalysis {
        let mut score: u32 = 0;
        let mut suggestions = Vec::new();
        let mut warnings = Vec::new();
        
        let length = password.len();
        let uppercase_count = password.chars().filter(|c| c.is_uppercase()).count();
        let lowercase_count = password.chars().filter(|c| c.is_lowercase()).count();
        let digit_count = password.chars().filter(|c| c.is_numeric()).count();
        let special_count = password.chars().filter(|c| !c.is_alphanumeric()).count();
        
        // ─── Length Score ───
        score += (length as u32).min(20); // Max 20 points for length
        
        if length < 8 {
            warnings.push("Password is very short".to_string());
            suggestions.push("Use at least 12 characters".to_string());
        } else if length < 12 {
            suggestions.push("Consider using a longer password (12+ characters)".to_string());
        }
        
        // ─── Character Variety Score ───
        if uppercase_count > 0 {
            score += 10;
            score += (uppercase_count as u32).min(5) * 2; // Bonus for more uppercase
        } else {
            suggestions.push("Add uppercase letters".to_string());
        }
        
        if lowercase_count > 0 {
            score += 10;
        } else {
            suggestions.push("Add lowercase letters".to_string());
        }
        
        if digit_count > 0 {
            score += 10;
            score += (digit_count as u32).min(3) * 2; // Bonus for more digits
        } else {
            suggestions.push("Add numbers".to_string());
        }
        
        if special_count > 0 {
            score += 15;
            score += (special_count as u32).min(3) * 3; // Bonus for more special chars
        } else {
            suggestions.push("Add special characters (!@#$%^&*)".to_string());
        }
        
        // ─── Pattern Detection ───
        let has_keyboard_pattern = self.detect_keyboard_pattern(password);
        let has_repeated_chars = self.detect_repeated_chars(password);
        
        if has_keyboard_pattern {
            score = score.saturating_sub(20);
            warnings.push("Contains keyboard pattern".to_string());
            suggestions.push("Avoid keyboard patterns like 'qwerty'".to_string());
        }
        
        if has_repeated_chars {
            score = score.saturating_sub(15);
            warnings.push("Contains repeated characters".to_string());
            suggestions.push("Avoid repeated characters like 'aaa' or '111'".to_string());
        }
        
        // ─── User Info Check ───
        if let Some(info) = user_info {
            if self.contains_user_info(password, info) {
                score = score.saturating_sub(25);
                warnings.push("Contains personal information".to_string());
                suggestions.push("Don't use your name, email, or birthdate in password".to_string());
            }
        }
        
        // ─── Blacklist Check ───
        if self.policy.use_blacklist && self.is_blacklisted(password) {
            score = score.saturating_sub(50);
            warnings.push("This is a commonly used password".to_string());
            suggestions.push("Choose a more unique password".to_string());
        }
        
        // ─── Entropy Calculation ───
        let charset_size = self.calculate_charset_size(password);
        let entropy = (length as f64) * (charset_size as f64).log2();
        
        // ─── Crack Time Estimate ───
        let crack_time = self.estimate_crack_time(entropy);
        
        let strength = PasswordStrength::from_score(score);
        
        PasswordAnalysis {
            strength,
            score,
            length,
            uppercase_count,
            lowercase_count,
            digit_count,
            special_count,
            has_keyboard_pattern,
            has_repeated_chars,
            entropy,
            crack_time_estimate: crack_time,
            suggestions,
            warnings,
        }
    }
    
    /// Validate password against policy
    pub fn validate(&self, password: &str, user_info: Option<&UserInfo>) -> Result<PasswordAnalysis, Vec<PasswordError>> {
        let mut errors = Vec::new();
        
        // Length checks
        if password.len() < self.policy.min_length {
            errors.push(PasswordError::TooShort(self.policy.min_length));
        }
        
        if password.len() > self.policy.max_length {
            errors.push(PasswordError::TooLong(self.policy.max_length));
        }
        
        // Character requirements
        let uppercase_count = password.chars().filter(|c| c.is_uppercase()).count();
        let lowercase_count = password.chars().filter(|c| c.is_lowercase()).count();
        let digit_count = password.chars().filter(|c| c.is_numeric()).count();
        let special_count = password.chars().filter(|c| !c.is_alphanumeric()).count();
        
        if uppercase_count < self.policy.min_uppercase {
            errors.push(PasswordError::MissingUppercase(self.policy.min_uppercase));
        }
        
        if lowercase_count < self.policy.min_lowercase {
            errors.push(PasswordError::MissingLowercase(self.policy.min_lowercase));
        }
        
        if digit_count < self.policy.min_digits {
            errors.push(PasswordError::MissingDigit(self.policy.min_digits));
        }
        
        if special_count < self.policy.min_special {
            errors.push(PasswordError::MissingSpecial(self.policy.min_special));
        }
        
        // Pattern checks
        if self.policy.forbid_keyboard_patterns && self.detect_keyboard_pattern(password) {
            errors.push(PasswordError::KeyboardPattern);
        }
        
        if self.policy.forbid_repeated_chars && self.detect_repeated_chars(password) {
            errors.push(PasswordError::RepeatedCharacters);
        }
        
        // User info check
        if self.policy.forbid_user_info {
            if let Some(info) = user_info {
                if self.contains_user_info(password, info) {
                    errors.push(PasswordError::ContainsUserInfo);
                }
            }
        }
        
        // Blacklist check
        if self.policy.use_blacklist && self.is_blacklisted(password) {
            errors.push(PasswordError::PasswordBlacklisted);
        }
        
        if !errors.is_empty() {
            return Err(errors);
        }
        
        // Analyze password
        let analysis = self.analyze(password, user_info);
        
        // Check minimum strength
        if !analysis.strength.meets_minimum(self.policy.min_strength) {
            return Err(vec![PasswordError::TooWeak(format!(
                "Password strength is {} but {} is required",
                analysis.strength, self.policy.min_strength
            ))]);
        }
        
        Ok(analysis)
    }
    
    /// Check if password is in blacklist
    fn is_blacklisted(&self, password: &str) -> bool {
        self.blacklist.contains(&password.to_lowercase())
    }
    
    /// Detect keyboard patterns
    fn detect_keyboard_pattern(&self, password: &str) -> bool {
        let lower = password.to_lowercase();
        
        for pattern in &self.keyboard_patterns {
            if lower.contains(pattern) {
                return true;
            }
        }
        
        false
    }
    
    /// Detect repeated characters (3+ in a row)
    fn detect_repeated_chars(&self, password: &str) -> bool {
        let chars: Vec<char> = password.chars().collect();
        
        for i in 0..chars.len().saturating_sub(2) {
            if chars[i] == chars[i + 1] && chars[i] == chars[i + 2] {
                return true;
            }
        }
        
        false
    }
    
    /// Check if password contains user info
    fn contains_user_info(&self, password: &str, user_info: &UserInfo) -> bool {
        let lower = password.to_lowercase();
        
        // Check username
        if !user_info.username.is_empty() && lower.contains(&user_info.username.to_lowercase()) {
            return true;
        }
        
        // Check email local part
        if let Some(email) = &user_info.email {
            let local = email.split('@').next().unwrap_or("");
            if !local.is_empty() && lower.contains(&local.to_lowercase()) {
                return true;
            }
        }
        
        // Check first name
        if let Some(name) = &user_info.first_name {
            if !name.is_empty() && lower.contains(&name.to_lowercase()) {
                return true;
            }
        }
        
        // Check last name
        if let Some(name) = &user_info.last_name {
            if !name.is_empty() && lower.contains(&name.to_lowercase()) {
                return true;
            }
        }
        
        // Check birth date (various formats)
        if let Some(birth_date) = &user_info.birth_date {
            let formats = [
                format!("{:04}", birth_date.year()),
                format!("{:02}", birth_date.month()),
                format!("{:02}", birth_date.day()),
                format!("{:02}{:02}", birth_date.day(), birth_date.month()),
                format!("{:02}{:02}", birth_date.month(), birth_date.day()),
            ];
            
            for fmt in formats {
                if lower.contains(&fmt.to_lowercase()) {
                    return true;
                }
            }
        }
        
        false
    }
    
    /// Calculate charset size for entropy
    fn calculate_charset_size(&self, password: &str) -> u32 {
        let mut size = 0u32;
        
        if password.chars().any(|c| c.is_lowercase()) {
            size += 26;
        }
        if password.chars().any(|c| c.is_uppercase()) {
            size += 26;
        }
        if password.chars().any(|c| c.is_numeric()) {
            size += 10;
        }
        if password.chars().any(|c| !c.is_alphanumeric()) {
            size += 32; // Common special chars
        }
        
        size.max(26) // Minimum charset size
    }
    
    /// Estimate time to crack
    fn estimate_crack_time(&self, entropy: f64) -> String {
        // Assume 10 billion guesses per second (modern GPU)
        let guesses_per_second = 10_000_000_000.0;
        let seconds = 2_f64.powf(entropy) / guesses_per_second;
        
        if seconds < 1.0 {
            "Instant".to_string()
        } else if seconds < 60.0 {
            format!("{:.0} seconds", seconds)
        } else if seconds < 3600.0 {
            format!("{:.0} minutes", seconds / 60.0)
        } else if seconds < 86400.0 {
            format!("{:.1} hours", seconds / 3600.0)
        } else if seconds < 2592000.0 {
            format!("{:.0} days", seconds / 86400.0)
        } else if seconds < 31536000.0 {
            format!("{:.1} months", seconds / 2592000.0)
        } else if seconds < 31536000.0 * 100.0 {
            format!("{:.1} years", seconds / 31536000.0)
        } else if seconds < 31536000.0 * 1_000_000.0 {
            format!("{:.0} centuries", seconds / 31536000.0 / 100.0)
        } else {
            "Millions of years".to_string()
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  USER INFO
// ═══════════════════════════════════════════════════════════════════════════════

/// User information for password validation
#[derive(Debug, Clone, Default)]
pub struct UserInfo {
    pub username: String,
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub birth_date: Option<chrono::NaiveDate>,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  PASSWORD HISTORY
// ═══════════════════════════════════════════════════════════════════════════════

/// Password history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordHistoryEntry {
    /// Hashed password
    pub hash: String,
    /// When it was set
    pub set_at: DateTime<Utc>,
    /// Whether it's the current password
    pub is_current: bool,
}

/// Password history manager
pub struct PasswordHistory {
    policy: PasswordPolicy,
    histories: RwLock<HashMap<String, Vec<PasswordHistoryEntry>>>,
}

impl PasswordHistory {
    /// Create new password history manager
    pub fn new(policy: PasswordPolicy) -> Self {
        Self {
            policy,
            histories: RwLock::new(HashMap::new()),
        }
    }
    
    /// Hash a password
    fn hash_password(password: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(password.as_bytes());
        format!("{:x}", hasher.finalize())
    }
    
    /// Check if password is in history
    pub async fn is_in_history(&self, user_id: &str, password: &str) -> bool {
        if self.policy.history_count == 0 {
            return false;
        }
        
        let histories = self.histories.read().await;
        if let Some(history) = histories.get(user_id) {
            let hash = Self::hash_password(password);
            return history.iter().any(|entry| entry.hash == hash);
        }
        
        false
    }
    
    /// Add password to history
    pub async fn add_to_history(&self, user_id: &str, password: &str) {
        if self.policy.history_count == 0 {
            return;
        }
        
        let mut histories = self.histories.write().await;
        let history = histories.entry(user_id.to_string()).or_insert_with(Vec::new);
        
        // Mark previous passwords as not current
        for entry in history.iter_mut() {
            entry.is_current = false;
        }
        
        // Add new entry
        history.push(PasswordHistoryEntry {
            hash: Self::hash_password(password),
            set_at: Utc::now(),
            is_current: true,
        });
        
        // Trim to max history count
        if history.len() > self.policy.history_count {
            history.remove(0);
        }
    }
    
    /// Get password age in days
    pub async fn get_password_age(&self, user_id: &str) -> Option<u32> {
        let histories = self.histories.read().await;
        if let Some(history) = histories.get(user_id) {
            if let Some(current) = history.iter().find(|e| e.is_current) {
                let age = Utc::now() - current.set_at;
                return Some((age.num_days() as u32).max(0));
            }
        }
        None
    }
    
    /// Check if password is expiring soon
    pub async fn is_expiring_soon(&self, user_id: &str) -> bool {
        if self.policy.max_age_days == 0 {
            return false;
        }
        
        if let Some(age) = self.get_password_age(user_id).await {
            let days_remaining = self.policy.max_age_days.saturating_sub(age);
            return days_remaining <= self.policy.expiration_warning_days;
        }
        
        false
    }
    
    /// Check if password is expired
    pub async fn is_expired(&self, user_id: &str) -> bool {
        if self.policy.max_age_days == 0 {
            return false;
        }
        
        if let Some(age) = self.get_password_age(user_id).await {
            return age >= self.policy.max_age_days;
        }
        
        false
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  FAILED ATTEMPTS TRACKER
// ═══════════════════════════════════════════════════════════════════════════════

/// Failed login attempt
#[derive(Debug, Clone)]
struct FailedAttempt {
    at: DateTime<Utc>,
}

/// Account lockout status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockoutStatus {
    pub is_locked: bool,
    pub failed_attempts: u32,
    pub locked_until: Option<DateTime<Utc>>,
    pub remaining_attempts: u32,
}

/// Failed attempts tracker
pub struct FailedAttemptsTracker {
    policy: PasswordPolicy,
    attempts: RwLock<HashMap<String, Vec<FailedAttempt>>>,
}

impl FailedAttemptsTracker {
    /// Create new tracker
    pub fn new(policy: PasswordPolicy) -> Self {
        Self {
            policy,
            attempts: RwLock::new(HashMap::new()),
        }
    }
    
    /// Record a failed attempt
    pub async fn record_failure(&self, user_id: &str) -> LockoutStatus {
        let mut attempts = self.attempts.write().await;
        let user_attempts = attempts.entry(user_id.to_string()).or_insert_with(Vec::new);
        
        // Clean old attempts
        let reset_before = Utc::now() - Duration::minutes(self.policy.failed_attempts_reset_minutes as i64);
        user_attempts.retain(|a| a.at > reset_before);
        
        // Add new attempt
        user_attempts.push(FailedAttempt { at: Utc::now() });
        
        let count = user_attempts.len() as u32;
        
        LockoutStatus {
            is_locked: count >= self.policy.max_failed_attempts,
            failed_attempts: count,
            locked_until: if count >= self.policy.max_failed_attempts {
                Some(Utc::now() + Duration::minutes(self.policy.lockout_duration_minutes as i64))
            } else {
                None
            },
            remaining_attempts: self.policy.max_failed_attempts.saturating_sub(count),
        }
    }
    
    /// Get lockout status
    pub async fn get_status(&self, user_id: &str) -> LockoutStatus {
        let mut attempts = self.attempts.write().await;
        
        if let Some(user_attempts) = attempts.get_mut(user_id) {
            // Check if lockout period has passed
            let reset_before = Utc::now() - Duration::minutes(self.policy.failed_attempts_reset_minutes as i64);
            user_attempts.retain(|a| a.at > reset_before);
            
            let count = user_attempts.len() as u32;
            
            // Check if still locked
            if count >= self.policy.max_failed_attempts {
                let lockout_ends = user_attempts.last()
                    .map(|a| a.at + Duration::minutes(self.policy.lockout_duration_minutes as i64));
                
                if let Some(ends) = lockout_ends {
                    if Utc::now() < ends {
                        return LockoutStatus {
                            is_locked: true,
                            failed_attempts: count,
                            locked_until: Some(ends),
                            remaining_attempts: 0,
                        };
                    } else {
                        // Lockout expired, clear attempts
                        user_attempts.clear();
                    }
                }
            }
            
            let count = user_attempts.len() as u32;
            LockoutStatus {
                is_locked: false,
                failed_attempts: count,
                locked_until: None,
                remaining_attempts: self.policy.max_failed_attempts.saturating_sub(count),
            }
        } else {
            LockoutStatus {
                is_locked: false,
                failed_attempts: 0,
                locked_until: None,
                remaining_attempts: self.policy.max_failed_attempts,
            }
        }
    }
    
    /// Clear failed attempts (on successful login)
    pub async fn clear(&self, user_id: &str) {
        let mut attempts = self.attempts.write().await;
        attempts.remove(user_id);
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_password_strength() {
        assert!(PasswordStrength::Strong.meets_minimum(PasswordStrength::Good));
        assert!(!PasswordStrength::Weak.meets_minimum(PasswordStrength::Good));
    }
    
    #[test]
    fn test_validator_length() {
        let validator = PasswordValidator::default_validator();
        
        // Too short
        let result = validator.validate("short", None);
        assert!(result.is_err());
        
        // Good length
        let result = validator.validate("GoodP@ssw0rd123!", None);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_validator_requirements() {
        let validator = PasswordValidator::default_validator();
        
        // Missing uppercase
        let result = validator.validate("lowercase123!pass", None);
        assert!(result.is_err());
        
        // Missing lowercase
        let result = validator.validate("UPPERCASE123!PASS", None);
        assert!(result.is_err());
        
        // Missing digit
        let result = validator.validate("NoDigitsHere!Only", None);
        assert!(result.is_err());
        
        // Missing special
        let result = validator.validate("NoSpecial123Chars", None);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_keyboard_pattern_detection() {
        let validator = PasswordValidator::default_validator();
        
        assert!(validator.detect_keyboard_pattern("qwerty123"));
        assert!(validator.detect_keyboard_pattern("asdfgh"));
        assert!(!validator.detect_keyboard_pattern("xkcd4life"));
    }
    
    #[test]
    fn test_repeated_chars_detection() {
        let validator = PasswordValidator::default_validator();
        
        assert!(validator.detect_repeated_chars("aaa"));
        assert!(validator.detect_repeated_chars("password111"));
        assert!(!validator.detect_repeated_chars("password1"));
    }
    
    #[test]
    fn test_user_info_detection() {
        let validator = PasswordValidator::default_validator();
        let user_info = UserInfo {
            username: "john_doe".to_string(),
            email: Some("john.doe@example.com".to_string()),
            first_name: Some("John".to_string()),
            last_name: Some("Doe".to_string()),
            birth_date: None,
        };
        
        assert!(validator.contains_user_info("john_doe123!", &user_info));
        assert!(validator.contains_user_info("JohnP@ssw0rd", &user_info));
        assert!(!validator.contains_user_info("C0mpl3x!P@ss", &user_info));
    }
    
    #[test]
    fn test_blacklist() {
        let validator = PasswordValidator::default_validator();
        
        assert!(validator.is_blacklisted("password"));
        assert!(validator.is_blacklisted("PASSWORD")); // Case insensitive
        assert!(validator.is_blacklisted("123456"));
        assert!(!validator.is_blacklisted("uniquEpass123!"));
    }
    
    #[test]
    fn test_password_analysis() {
        let validator = PasswordValidator::default_validator();
        let analysis = validator.analyze("Str0ng!Pass123", None);
        
        assert!(analysis.score > 60);
        assert_eq!(analysis.uppercase_count, 2);
        assert_eq!(analysis.digit_count, 4);
        assert_eq!(analysis.special_count, 1);
    }
    
    #[tokio::test]
    async fn test_password_history() {
        let policy = PasswordPolicy::default();
        let history = PasswordHistory::new(policy);
        
        history.add_to_history("user1", "Password1!").await;
        
        assert!(history.is_in_history("user1", "Password1!").await);
        assert!(!history.is_in_history("user1", "Password2!").await);
    }
    
    #[tokio::test]
    async fn test_failed_attempts() {
        let policy = PasswordPolicy::default();
        let tracker = FailedAttemptsTracker::new(policy);
        
        // Record failures
        for _ in 0..4 {
            tracker.record_failure("user1").await;
        }
        
        let status = tracker.get_status("user1").await;
        assert!(!status.is_locked);
        assert_eq!(status.failed_attempts, 4);
        
        // One more failure should lock
        tracker.record_failure("user1").await;
        let status = tracker.get_status("user1").await;
        assert!(status.is_locked);
        
        // Clear should unlock
        tracker.clear("user1").await;
        let status = tracker.get_status("user1").await;
        assert!(!status.is_locked);
    }
}
