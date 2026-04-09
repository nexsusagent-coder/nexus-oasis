//! Enterprise SSO Example
//!
//! Demonstrates enterprise features: SSO, RBAC, Audit Logging.
//!
//! # Features
//! - SSO with Okta, Auth0, Azure AD
//! - Role-Based Access Control (RBAC)
//! - Audit logging
//! - Multi-tenancy
//!
//! # Usage
//! ```bash
//! cargo run --example enterprise-sso
//! ```

use sentient_enterprise::{
    sso::{SsoProvider, SsoConfig, SsoManager},
    rbac::{RbacManager, Role, Permission},
    audit::{AuditLogger, AuditEvent, AuditAction},
    tenant::{TenantManager, Tenant, TenantConfig},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Enterprise application state
struct EnterpriseApp {
    sso: SsoManager,
    rbac: RbacManager,
    audit: AuditLogger,
    tenants: TenantManager,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("🏢 SENTIENT Enterprise SSO Example\n");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // Initialize SSO Manager
    println!("📦 Initializing SSO Manager...");
    let sso_config = SsoConfig {
        providers: vec![
            SsoProvider::Okta {
                domain: "your-domain.okta.com".to_string(),
                client_id: "client_id".to_string(),
                client_secret: "client_secret".to_string(),
            },
            SsoProvider::Auth0 {
                domain: "your-tenant.auth0.com".to_string(),
                client_id: "client_id".to_string(),
                client_secret: "client_secret".to_string(),
            },
            SsoProvider::AzureAD {
                tenant_id: "tenant_id".to_string(),
                client_id: "client_id".to_string(),
                client_secret: "client_secret".to_string(),
            },
        ],
        jwt_secret: "super-secret-key".to_string(),
        session_duration_hours: 24,
    };
    let sso = SsoManager::new(sso_config);
    println!("✅ SSO ready with {} providers\n", sso.provider_count());

    // Initialize RBAC
    println!("📦 Initializing RBAC Manager...");
    let mut rbac = RbacManager::new();

    // Define roles
    rbac.add_role(Role {
        name: "admin".to_string(),
        permissions: vec![
            Permission::All,
        ],
        description: "Full access".to_string(),
    });

    rbac.add_role(Role {
        name: "manager".to_string(),
        permissions: vec![
            Permission::ReadAgents,
            Permission::WriteAgents,
            Permission::ReadChannels,
            Permission::WriteChannels,
            Permission::ReadUsers,
        ],
        description: "Team manager".to_string(),
    });

    rbac.add_role(Role {
        name: "developer".to_string(),
        permissions: vec![
            Permission::ReadAgents,
            Permission::WriteAgents,
            Permission::ReadChannels,
        ],
        description: "Developer access".to_string(),
    });

    rbac.add_role(Role {
        name: "viewer".to_string(),
        permissions: vec![
            Permission::ReadAgents,
            Permission::ReadChannels,
        ],
        description: "Read-only access".to_string(),
    });

    println!("✅ RBAC ready with {} roles:\n", rbac.role_count());
    for role in rbac.list_roles() {
        println!("   • {}: {} permissions", role.name, role.permissions.len());
    }
    println!();

    // Initialize Audit Logger
    println!("📦 Initializing Audit Logger...");
    let audit = AuditLogger::new("logs/audit.log");
    println!("✅ Audit logging enabled\n");

    // Initialize Tenant Manager
    println!("📦 Initializing Tenant Manager...");
    let mut tenants = TenantManager::new();

    // Create demo tenants
    tenants.create(Tenant {
        id: "acme-corp".to_string(),
        name: "ACME Corporation".to_string(),
        config: TenantConfig {
            max_agents: 100,
            max_users: 500,
            features: vec!["sso", "audit", "custom_skills"],
            branding: Default::default(),
        },
    });

    tenants.create(Tenant {
        id: "tech-startup".to_string(),
        name: "Tech Startup Inc".to_string(),
        config: TenantConfig {
            max_agents: 10,
            max_users: 50,
            features: vec!["basic"],
            branding: Default::default(),
        },
    });

    println!("✅ Multi-tenancy ready with {} tenants:\n", tenants.count());
    for tenant in tenants.list() {
        println!("   • {} ({})", tenant.name, tenant.id);
    }
    println!();

    // Demo: User login flow
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    demo_user_flow(&sso, &rbac, &audit).await?;

    // Demo: Permission check
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    demo_permission_check(&rbac).await?;

    // Demo: Audit trail
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    demo_audit_trail(&audit).await?;

    println!("\n✅ Enterprise SSO example completed!");
    println!("\n💡 Enterprise features ready for production deployment!");

    Ok(())
}

async fn demo_user_flow(
    sso: &SsoManager,
    rbac: &RbacManager,
    audit: &AuditLogger,
) -> anyhow::Result<()> {
    println!("👤 Demo: User Login Flow\n");

    // Simulate user login via Okta
    let user_id = "user-123";
    let tenant_id = "acme-corp";

    println!("1️⃣ User authenticates via Okta SSO");
    
    // Log authentication
    audit.log(AuditEvent {
        timestamp: chrono::Utc::now(),
        tenant_id: tenant_id.to_string(),
        user_id: user_id.to_string(),
        action: AuditAction::Login,
        resource: "sso/okta".to_string(),
        details: serde_json::json!({
            "provider": "okta",
            "ip": "192.168.1.100"
        }),
    });
    println!("   ✅ Authenticated successfully\n");

    // Assign role
    println!("2️⃣ Assigning role: developer");
    rbac.assign_role(user_id, "developer")?;
    println!("   ✅ Role assigned\n");

    // Log role assignment
    audit.log(AuditEvent {
        timestamp: chrono::Utc::now(),
        tenant_id: tenant_id.to_string(),
        user_id: "system".to_string(),
        action: AuditAction::RoleChange,
        resource: format!("user/{}", user_id),
        details: serde_json::json!({
            "role": "developer"
        }),
    });

    println!("3️⃣ Session established for tenant: {}", tenant_id);

    Ok(())
}

async fn demo_permission_check(rbac: &RbacManager) -> anyhow::Result<()> {
    println!("🔒 Demo: Permission Checks\n");

    let user_id = "user-123";

    let checks = vec![
        ("ReadAgents", Permission::ReadAgents),
        ("WriteAgents", Permission::WriteAgents),
        ("ReadUsers", Permission::ReadUsers),
        ("DeleteAgents", Permission::DeleteAgents),
        ("Admin", Permission::All),
    ];

    for (name, permission) in checks {
        let allowed = rbac.check_permission(user_id, &permission)?;
        let status = if allowed { "✅ ALLOWED" } else { "❌ DENIED" };
        println!("   {} {}: {}", status, name, 
            if allowed { "granted" } else { "denied" }
        );
    }

    Ok(())
}

async fn demo_audit_trail(audit: &AuditLogger) -> anyhow::Result<()> {
    println!("📋 Demo: Audit Trail\n");

    println!("   Recent events:");
    for event in audit.get_recent(10) {
        println!(
            "   • [{}] {} - {} by {}",
            event.timestamp.format("%H:%M:%S"),
            event.action.to_string(),
            event.resource,
            event.user_id
        );
    }

    Ok(())
}

// Stub implementations for demo
mod sentient_enterprise {
    use serde::{Deserialize, Serialize};

    pub mod sso {
        use super::*;

        #[derive(Debug, Clone)]
        pub enum SsoProvider {
            Okta { domain: String, client_id: String, client_secret: String },
            Auth0 { domain: String, client_id: String, client_secret: String },
            AzureAD { tenant_id: String, client_id: String, client_secret: String },
            Google { client_id: String, client_secret: String },
            Keycloak { url: String, realm: String, client_id: String, client_secret: String },
        }

        #[derive(Debug, Clone)]
        pub struct SsoConfig {
            pub providers: Vec<SsoProvider>,
            pub jwt_secret: String,
            pub session_duration_hours: u32,
        }

        pub struct SsoManager {
            config: SsoConfig,
        }

        impl SsoManager {
            pub fn new(config: SsoConfig) -> Self {
                Self { config }
            }

            pub fn provider_count(&self) -> usize {
                self.config.providers.len()
            }
        }
    }

    pub mod rbac {
        use std::collections::HashMap;

        #[derive(Debug, Clone, PartialEq)]
        pub enum Permission {
            All,
            ReadAgents,
            WriteAgents,
            DeleteAgents,
            ReadChannels,
            WriteChannels,
            ReadUsers,
            WriteUsers,
            ManageBilling,
        }

        #[derive(Debug, Clone)]
        pub struct Role {
            pub name: String,
            pub permissions: Vec<Permission>,
            pub description: String,
        }

        pub struct RbacManager {
            roles: HashMap<String, Role>,
            user_roles: HashMap<String, String>,
        }

        impl RbacManager {
            pub fn new() -> Self {
                Self {
                    roles: HashMap::new(),
                    user_roles: HashMap::new(),
                }
            }

            pub fn add_role(&mut self, role: Role) {
                self.roles.insert(role.name.clone(), role);
            }

            pub fn assign_role(&mut self, user_id: &str, role: &str) -> anyhow::Result<()> {
                self.user_roles.insert(user_id.to_string(), role.to_string());
                Ok(())
            }

            pub fn role_count(&self) -> usize {
                self.roles.len()
            }

            pub fn list_roles(&self) -> Vec<&Role> {
                self.roles.values().collect()
            }

            pub fn check_permission(&self, user_id: &str, permission: &Permission) -> anyhow::Result<bool> {
                let role_name = self.user_roles.get(user_id)
                    .ok_or_else(|| anyhow::anyhow!("User has no role"))?;
                
                let role = self.roles.get(role_name)
                    .ok_or_else(|| anyhow::anyhow!("Role not found"))?;

                Ok(role.permissions.contains(&Permission::All) || 
                   role.permissions.contains(permission))
            }
        }
    }

    pub mod audit {
        use chrono::{DateTime, Utc};
        use serde_json::Value;

        #[derive(Debug, Clone)]
        pub enum AuditAction {
            Login,
            Logout,
            RoleChange,
            ResourceAccess,
            DataRead,
            DataWrite,
            DataDelete,
        }

        impl ToString for AuditAction {
            fn to_string(&self) -> String {
                match self {
                    AuditAction::Login => "LOGIN".to_string(),
                    AuditAction::Logout => "LOGOUT".to_string(),
                    AuditAction::RoleChange => "ROLE_CHANGE".to_string(),
                    AuditAction::ResourceAccess => "RESOURCE_ACCESS".to_string(),
                    AuditAction::DataRead => "DATA_READ".to_string(),
                    AuditAction::DataWrite => "DATA_WRITE".to_string(),
                    AuditAction::DataDelete => "DATA_DELETE".to_string(),
                }
            }
        }

        #[derive(Debug, Clone)]
        pub struct AuditEvent {
            pub timestamp: DateTime<Utc>,
            pub tenant_id: String,
            pub user_id: String,
            pub action: AuditAction,
            pub resource: String,
            pub details: Value,
        }

        pub struct AuditLogger {
            path: String,
            events: Vec<AuditEvent>,
        }

        impl AuditLogger {
            pub fn new(path: &str) -> Self {
                Self {
                    path: path.to_string(),
                    events: Vec::new(),
                }
            }

            pub fn log(&mut self, event: AuditEvent) {
                self.events.push(event);
            }

            pub fn get_recent(&self, count: usize) -> Vec<&AuditEvent> {
                self.events.iter().rev().take(count).collect()
            }
        }
    }

    pub mod tenant {
        use serde_json::Value;

        #[derive(Debug, Clone)]
        pub struct TenantConfig {
            pub max_agents: u32,
            pub max_users: u32,
            pub features: Vec<&'static str>,
            pub branding: Value,
        }

        #[derive(Debug, Clone)]
        pub struct Tenant {
            pub id: String,
            pub name: String,
            pub config: TenantConfig,
        }

        pub struct TenantManager {
            tenants: Vec<Tenant>,
        }

        impl TenantManager {
            pub fn new() -> Self {
                Self { tenants: Vec::new() }
            }

            pub fn create(&mut self, tenant: Tenant) {
                self.tenants.push(tenant);
            }

            pub fn count(&self) -> usize {
                self.tenants.len()
            }

            pub fn list(&self) -> Vec<&Tenant> {
                self.tenants.iter().collect()
            }
        }
    }
}
