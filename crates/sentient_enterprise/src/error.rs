//! Enterprise error types

use thiserror::Error;

/// Enterprise error
#[derive(Debug, Error)]
pub enum EnterpriseError {
    /// RBAC error
    #[error("RBAC error: {0}")]
    RBAC(#[from] crate::rbac::RBACError),

    /// Audit error
    #[error("Audit error: {0}")]
    Audit(#[from] crate::audit::AuditError),

    /// SSO error
    #[error("SSO error: {0}")]
    SSO(#[from] crate::sso::SSOError),

    /// Tenant error
    #[error("Tenant error: {0}")]
    Tenant(#[from] crate::tenant::TenantError),

    /// Configuration error
    #[error("Configuration error: {0}")]
    Config(#[from] crate::config::ConfigError),

    /// Database error
    #[error("Database error: {0}")]
    Database(String),

    /// Encryption error
    #[error("Encryption error: {0}")]
    Encryption(String),

    /// Authentication error
    #[error("Authentication error: {0}")]
    Authentication(String),

    /// Authorization error
    #[error("Authorization error: {0}")]
    Authorization(String),

    /// Not found error
    #[error("Not found: {0}")]
    NotFound(String),

    /// Validation error
    #[error("Validation error: {0}")]
    Validation(String),

    /// Internal error
    #[error("Internal error: {0}")]
    Internal(String),
}

impl From<sqlx::Error> for EnterpriseError {
    fn from(err: sqlx::Error) -> Self {
        EnterpriseError::Database(err.to_string())
    }
}

impl From<serde_json::Error> for EnterpriseError {
    fn from(err: serde_json::Error) -> Self {
        EnterpriseError::Internal(err.to_string())
    }
}
