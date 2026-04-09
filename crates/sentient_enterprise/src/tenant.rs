//! Multi-tenancy Support
//!
//! Provides isolation between different organizations/customers

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Tenant (organization/workspace)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tenant {
    /// Unique tenant ID
    pub id: Uuid,

    /// Tenant slug/identifier
    pub slug: String,

    /// Display name
    pub name: String,

    /// Tenant status
    pub status: TenantStatus,

    /// Plan type
    pub plan: TenantPlan,

    /// Tenant settings
    pub settings: TenantSettings,

    /// Resource quotas
    pub quotas: ResourceQuotas,

    /// Created at
    pub created_at: chrono::DateTime<chrono::Utc>,

    /// Updated at
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Tenant status
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TenantStatus {
    Active,
    Suspended,
    Trial,
    Deleted,
}

/// Tenant plan
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TenantPlan {
    Free,
    Starter,
    Professional,
    Enterprise,
    Custom,
}

/// Tenant settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantSettings {
    /// Enable voice features
    pub voice_enabled: bool,

    /// Enable skills marketplace
    pub skills_enabled: bool,

    /// Enable analytics
    pub analytics_enabled: bool,

    /// Custom domain
    pub custom_domain: Option<String>,

    /// Branding settings
    pub branding: BrandingSettings,

    /// Security settings
    pub security: SecuritySettings,
}

impl Default for TenantSettings {
    fn default() -> Self {
        Self {
            voice_enabled: true,
            skills_enabled: true,
            analytics_enabled: true,
            custom_domain: None,
            branding: BrandingSettings::default(),
            security: SecuritySettings::default(),
        }
    }
}

/// Branding settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrandingSettings {
    pub logo_url: Option<String>,
    pub primary_color: String,
    pub secondary_color: String,
    pub custom_css: Option<String>,
}

impl Default for BrandingSettings {
    fn default() -> Self {
        Self {
            logo_url: None,
            primary_color: "#3B82F6".to_string(), // Blue
            secondary_color: "#10B981".to_string(), // Green
            custom_css: None,
        }
    }
}

/// Security settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecuritySettings {
    pub mfa_required: bool,
    pub password_policy: PasswordPolicy,
    pub ip_whitelist: Vec<String>,
    pub session_timeout_minutes: u32,
}

impl Default for SecuritySettings {
    fn default() -> Self {
        Self {
            mfa_required: false,
            password_policy: PasswordPolicy::default(),
            ip_whitelist: vec![],
            session_timeout_minutes: 1440, // 24 hours
        }
    }
}

/// Password policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordPolicy {
    pub min_length: u8,
    pub require_uppercase: bool,
    pub require_lowercase: bool,
    pub require_numbers: bool,
    pub require_symbols: bool,
}

impl Default for PasswordPolicy {
    fn default() -> Self {
        Self {
            min_length: 8,
            require_uppercase: true,
            require_lowercase: true,
            require_numbers: true,
            require_symbols: false,
        }
    }
}

/// Resource quotas
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceQuotas {
    /// Maximum number of agents
    pub max_agents: u32,

    /// Maximum number of channels
    pub max_channels: u32,

    /// Maximum number of users
    pub max_users: u32,

    /// Maximum API calls per month
    pub max_api_calls: u64,

    /// Maximum storage in MB
    pub max_storage_mb: u64,

    /// Maximum concurrent sessions
    pub max_concurrent_sessions: u32,
}

impl Default for ResourceQuotas {
    fn default() -> Self {
        Self {
            max_agents: 10,
            max_channels: 10,
            max_users: 10,
            max_api_calls: 100_000,
            max_storage_mb: 1024,
            max_concurrent_sessions: 100,
        }
    }
}

impl ResourceQuotas {
    /// Free plan quotas
    pub fn free() -> Self {
        Self {
            max_agents: 1,
            max_channels: 3,
            max_users: 1,
            max_api_calls: 1_000,
            max_storage_mb: 100,
            max_concurrent_sessions: 5,
        }
    }

    /// Starter plan quotas
    pub fn starter() -> Self {
        Self {
            max_agents: 5,
            max_channels: 10,
            max_users: 5,
            max_api_calls: 50_000,
            max_storage_mb: 500,
            max_concurrent_sessions: 25,
        }
    }

    /// Professional plan quotas
    pub fn professional() -> Self {
        Self {
            max_agents: 25,
            max_channels: 25,
            max_users: 25,
            max_api_calls: 500_000,
            max_storage_mb: 5_000,
            max_concurrent_sessions: 100,
        }
    }

    /// Enterprise plan quotas (unlimited)
    pub fn enterprise() -> Self {
        Self {
            max_agents: u32::MAX,
            max_channels: u32::MAX,
            max_users: u32::MAX,
            max_api_calls: u64::MAX,
            max_storage_mb: u64::MAX,
            max_concurrent_sessions: u32::MAX,
        }
    }
}

/// Tenant configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantConfig {
    /// Enable multi-tenancy
    pub enabled: bool,

    /// Default plan for new tenants
    pub default_plan: TenantPlan,

    /// Allow self-registration
    pub allow_self_registration: bool,

    /// Trial duration in days
    pub trial_days: u32,
}

impl Default for TenantConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            default_plan: TenantPlan::Free,
            allow_self_registration: true,
            trial_days: 14,
        }
    }
}

/// Tenant Manager
pub struct TenantManager {
    config: TenantConfig,
    tenants: HashMap<Uuid, Tenant>,
}

impl TenantManager {
    /// Create a new tenant manager
    pub async fn new(config: TenantConfig) -> Result<Self, TenantError> {
        Ok(Self {
            config,
            tenants: HashMap::new(),
        })
    }

    /// Create a new tenant
    pub async fn create_tenant(
        &mut self,
        name: String,
        slug: String,
        plan: TenantPlan,
    ) -> Result<Tenant, TenantError> {
        let id = Uuid::new_v4();
        let now = chrono::Utc::now();

        let quotas = match plan {
            TenantPlan::Free => ResourceQuotas::free(),
            TenantPlan::Starter => ResourceQuotas::starter(),
            TenantPlan::Professional => ResourceQuotas::professional(),
            TenantPlan::Enterprise => ResourceQuotas::enterprise(),
            TenantPlan::Custom => ResourceQuotas::professional(),
        };

        let tenant = Tenant {
            id,
            slug,
            name,
            status: TenantStatus::Active,
            plan,
            settings: TenantSettings::default(),
            quotas,
            created_at: now,
            updated_at: now,
        };

        self.tenants.insert(id, tenant.clone());

        Ok(tenant)
    }

    /// Get tenant by ID
    pub async fn get_tenant(&self, id: Uuid) -> Option<Tenant> {
        self.tenants.get(&id).cloned()
    }

    /// Get tenant by slug
    pub async fn get_tenant_by_slug(&self, slug: &str) -> Option<Tenant> {
        self.tenants.values().find(|t| t.slug == slug).cloned()
    }

    /// Update tenant
    pub async fn update_tenant(
        &mut self,
        id: Uuid,
        updates: TenantUpdates,
    ) -> Result<Tenant, TenantError> {
        let tenant = self.tenants.get_mut(&id)
            .ok_or(TenantError::NotFound(id))?;

        if let Some(name) = updates.name {
            tenant.name = name;
        }
        if let Some(status) = updates.status {
            tenant.status = status;
        }
        if let Some(plan) = updates.plan {
            tenant.plan = plan;
            tenant.quotas = match plan {
                TenantPlan::Free => ResourceQuotas::free(),
                TenantPlan::Starter => ResourceQuotas::starter(),
                TenantPlan::Professional => ResourceQuotas::professional(),
                TenantPlan::Enterprise => ResourceQuotas::enterprise(),
                TenantPlan::Custom => tenant.quotas.clone(),
            };
        }

        tenant.updated_at = chrono::Utc::now();

        Ok(tenant.clone())
    }

    /// Delete tenant
    pub async fn delete_tenant(&mut self, id: Uuid) -> Result<(), TenantError> {
        self.tenants.remove(&id)
            .map(|_| ())
            .ok_or(TenantError::NotFound(id))
    }

    /// List all tenants
    pub async fn list_tenants(&self) -> Vec<Tenant> {
        self.tenants.values().cloned().collect()
    }

    /// Check if tenant has capacity
    pub fn check_capacity(
        &self,
        tenant_id: Uuid,
        resource: ResourceType,
        current: u32,
    ) -> Result<bool, TenantError> {
        let tenant = self.tenants.get(&tenant_id)
            .ok_or(TenantError::NotFound(tenant_id))?;

        let limit = match resource {
            ResourceType::Agents => tenant.quotas.max_agents,
            ResourceType::Channels => tenant.quotas.max_channels,
            ResourceType::Users => tenant.quotas.max_users,
            ResourceType::ConcurrentSessions => tenant.quotas.max_concurrent_sessions,
        };

        Ok(current < limit)
    }
}

/// Resource type for quota checking
#[derive(Debug, Clone, Copy)]
pub enum ResourceType {
    Agents,
    Channels,
    Users,
    ConcurrentSessions,
}

/// Tenant updates
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TenantUpdates {
    pub name: Option<String>,
    pub status: Option<TenantStatus>,
    pub plan: Option<TenantPlan>,
}

/// Tenant error types
#[derive(Debug, thiserror::Error)]
pub enum TenantError {
    #[error("Tenant not found: {0}")]
    NotFound(Uuid),

    #[error("Slug already taken: {0}")]
    SlugTaken(String),

    #[error("Quota exceeded for {0}")]
    QuotaExceeded(String),

    #[error("Invalid slug: {0}")]
    InvalidSlug(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_tenant() {
        let config = TenantConfig::default();
        let mut manager = TenantManager::new(config).await.unwrap();

        let tenant = manager.create_tenant(
            "Test Company".to_string(),
            "test-company".to_string(),
            TenantPlan::Professional,
        ).await.unwrap();

        assert_eq!(tenant.name, "Test Company");
        assert_eq!(tenant.slug, "test-company");
        assert_eq!(tenant.plan, TenantPlan::Professional);
    }

    #[test]
    fn test_resource_quotas() {
        let free = ResourceQuotas::free();
        assert_eq!(free.max_agents, 1);
        assert_eq!(free.max_users, 1);

        let enterprise = ResourceQuotas::enterprise();
        assert_eq!(enterprise.max_agents, u32::MAX);
    }
}
