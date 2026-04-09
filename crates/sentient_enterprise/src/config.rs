//! Enterprise configuration

use serde::{Deserialize, Serialize};
use crate::rbac::RBACConfig;
use crate::audit::AuditConfig;
use crate::sso::SSOConfig;
use crate::tenant::TenantConfig;

/// Main enterprise configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnterpriseConfig {
    /// RBAC configuration
    pub rbac: RBACConfig,

    /// Audit logging configuration
    pub audit: AuditConfig,

    /// SSO configuration
    pub sso: SSOConfig,

    /// Multi-tenancy configuration
    pub tenants: TenantConfig,

    /// Database connection string
    pub database_url: String,

    /// Redis URL for sessions
    pub redis_url: Option<String>,

    /// Encryption key for sensitive data
    pub encryption_key: String,
}

impl Default for EnterpriseConfig {
    fn default() -> Self {
        Self {
            rbac: RBACConfig::default(),
            audit: AuditConfig::default(),
            sso: SSOConfig::default(),
            tenants: TenantConfig::default(),
            database_url: "postgresql://localhost/sentient".to_string(),
            redis_url: None,
            encryption_key: "change-me-in-production".to_string(),
        }
    }
}

impl EnterpriseConfig {
    /// Load from environment
    pub fn from_env() -> Result<Self, ConfigError> {
        let mut config = Self::default();

        if let Ok(url) = std::env::var("DATABASE_URL") {
            config.database_url = url;
        }
        if let Ok(url) = std::env::var("REDIS_URL") {
            config.redis_url = Some(url);
        }
        if let Ok(key) = std::env::var("ENCRYPTION_KEY") {
            config.encryption_key = key;
        }
        if let Ok(enabled) = std::env::var("SSO_ENABLED") {
            config.sso.enabled = enabled.parse().unwrap_or(false);
        }

        Ok(config)
    }

    /// Load from file
    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let config: Self = toml::from_str(&content)?;
        Ok(config)
    }

    /// Save to file
    pub fn save_to_file(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let content = toml::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.database_url.is_empty() {
            return Err(ConfigError::MissingField("database_url".to_string()));
        }
        if self.encryption_key.is_empty() {
            return Err(ConfigError::MissingField("encryption_key".to_string()));
        }
        if self.encryption_key == "change-me-in-production" {
            tracing::warn!("Using default encryption key. Change this in production!");
        }

        Ok(())
    }
}

/// Configuration error
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Missing required field: {0}")]
    MissingField(String),

    #[error("Invalid value for {field}: {value}")]
    InvalidValue { field: String, value: String },

    #[error("Parse error: {0}")]
    ParseError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = EnterpriseConfig::default();
        assert!(!config.database_url.is_empty());
        assert!(!config.encryption_key.is_empty());
    }

    #[test]
    fn test_validate_config() {
        let config = EnterpriseConfig::default();
        assert!(config.validate().is_ok());
    }
}
