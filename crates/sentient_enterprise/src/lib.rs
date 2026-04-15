//! Enterprise features for SENTIENT AI OS
//!
//! This crate provides enterprise-grade features including:
//! - RBAC (Role-Based Access Control)
//! - Audit Logging
//! - SSO (Single Sign-On) Integration
//! - Multi-tenancy Support
//! - MFA (Multi-Factor Authentication)
//! - Password Policy Enforcement

pub mod rbac;
pub mod audit;
pub mod sso;
pub mod tenant;
pub mod mfa;
pub mod password_policy;
pub mod config;
pub mod error;
pub mod scim;
pub mod consent;

pub use rbac::{Role, Permission, RBACManager};
pub use audit::{AuditLog, AuditEvent, AuditQuery};
pub use sso::{SSOProvider, SSOConfig, SSOManager};
pub use tenant::{Tenant, TenantManager};
pub use mfa::{MfaManager, MfaConfig, MfaMethod, UserMfaSettings, TotpSetup};
pub use password_policy::{
    PasswordPolicy, PasswordValidator, PasswordStrength, PasswordAnalysis,
    PasswordHistory, FailedAttemptsTracker, LockoutStatus, PasswordError, UserInfo
};
pub use config::EnterpriseConfig;
pub use error::EnterpriseError;

/// Enterprise feature flags
pub mod features {
    /// Role-Based Access Control
    pub const RBAC: &str = "rbac";
    /// Audit Logging
    pub const AUDIT: &str = "audit";
    /// Single Sign-On
    pub const SSO: &str = "sso";
    /// Trusted Execution Environment
    pub const TEE: &str = "tee";
}

/// Enterprise manager that coordinates all enterprise features
pub struct EnterpriseManager {
    rbac: RBACManager,
    audit: AuditLog,
    sso: Option<SSOManager>,
    tenants: TenantManager,
    mfa: MfaManager,
    password_policy: PasswordPolicy,
    password_validator: PasswordValidator,
    password_history: PasswordHistory,
    failed_attempts: FailedAttemptsTracker,
}

impl EnterpriseManager {
    /// Creates a new enterprise manager
    pub async fn new(config: EnterpriseConfig) -> Result<Self, EnterpriseError> {
        let rbac = RBACManager::new(config.rbac.clone()).await?;
        let audit = AuditLog::new(config.audit.clone()).await?;
        let sso = if config.sso.enabled {
            Some(SSOManager::new(config.sso.clone()).await?)
        } else {
            None
        };
        let tenants = TenantManager::new(config.tenants.clone()).await?;

        Ok(Self {
            rbac,
            audit,
            sso,
            tenants,
            mfa: MfaManager::new(config.mfa.clone()),
            password_policy: config.password_policy.clone(),
            password_validator: PasswordValidator::new(config.password_policy.clone()),
            password_history: PasswordHistory::new(config.password_policy.clone()),
            failed_attempts: FailedAttemptsTracker::new(config.password_policy.clone()),
        })
    }

    /// Get RBAC manager
    pub fn rbac(&self) -> &RBACManager {
        &self.rbac
    }

    /// Get audit log
    pub fn audit(&self) -> &AuditLog {
        &self.audit
    }

    /// Get SSO manager
    pub fn sso(&self) -> Option<&SSOManager> {
        self.sso.as_ref()
    }

    /// Get tenant manager
    pub fn tenants(&self) -> &TenantManager {
        &self.tenants
    }
    
    /// Get MFA manager
    pub fn mfa(&self) -> &MfaManager {
        &self.mfa
    }
    
    /// Get password validator
    pub fn password_validator(&self) -> &PasswordValidator {
        &self.password_validator
    }
    
    /// Get password history
    pub fn password_history(&self) -> &PasswordHistory {
        &self.password_history
    }
    
    /// Get failed attempts tracker
    pub fn failed_attempts(&self) -> &FailedAttemptsTracker {
        &self.failed_attempts
    }

    /// Check if user has permission for action
    pub async fn check_permission(
        &self,
        user_id: &str,
        resource: &str,
        action: &str,
    ) -> Result<bool, EnterpriseError> {
        // Get user roles
        let roles = self.rbac.get_user_roles(user_id).await?;

        // Check each role for permission
        for role in roles {
            if self.rbac.has_permission(&role, resource, action).await? {
                // Audit the access
                self.audit.log(AuditEvent::AccessGranted {
                    user_id: user_id.to_string(),
                    resource: resource.to_string(),
                    action: action.to_string(),
                    role: role.to_string(),
                }).await?;
                return Ok(true);
            }
        }

        // Audit the denied access
        self.audit.log(AuditEvent::AccessDenied {
            user_id: user_id.to_string(),
            resource: resource.to_string(),
            action: action.to_string(),
        }).await?;

        Ok(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_flags() {
        assert_eq!(features::RBAC, "rbac");
        assert_eq!(features::AUDIT, "audit");
        assert_eq!(features::SSO, "sso");
        assert_eq!(features::TEE, "tee");
    }
}
