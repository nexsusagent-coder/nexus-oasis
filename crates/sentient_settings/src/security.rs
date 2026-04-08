//! Security Settings - Güvenlik ayarları

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecuritySettings {
    /// V-GATE proxy aktif
    pub vgate_enabled: bool,
    
    /// Guardrails aktif
    pub guardrails_enabled: bool,
    
    /// Input sanitization
    pub sanitize_input: bool,
    
    /// Session timeout (dakika)
    pub session_timeout: u32,
    
    /// Max request size (MB)
    pub max_request_size: u32,
    
    /// Rate limiting
    pub rate_limit_enabled: bool,
    
    /// Requests per minute
    pub rate_limit_rpm: u32,
    
    /// Log seviyesi
    pub log_level: String,
    
    /// Tehlikeli komutları engelle
    pub block_dangerous_commands: bool,
    
    /// Şüpheli pattern tespit
    pub detect_suspicious_patterns: bool,
    
    /// API key rotasyonu
    pub api_key_rotation: bool,
    
    /// Rotasyon süresi (gün)
    pub rotation_days: u32,
}

impl Default for SecuritySettings {
    fn default() -> Self {
        Self {
            vgate_enabled: true,
            guardrails_enabled: true,
            sanitize_input: true,
            session_timeout: 30,
            max_request_size: 10,
            rate_limit_enabled: true,
            rate_limit_rpm: 60,
            log_level: "info".to_string(),
            block_dangerous_commands: true,
            detect_suspicious_patterns: true,
            api_key_rotation: false,
            rotation_days: 90,
        }
    }
}
