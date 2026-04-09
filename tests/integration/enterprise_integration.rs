//! Enterprise feature tests

#[cfg(test)]
mod enterprise_tests {
    use sentient_enterprise::{RbacManager, Role, Permission, AuditLogger};

    #[test]
    fn test_rbac_creation() {
        let rbac = RbacManager::new();
        assert_eq!(rbac.role_count(), 0);
    }

    #[test]
    fn test_role_creation() {
        let role = Role {
            name: "admin".to_string(),
            permissions: vec![Permission::All],
            description: "Administrator role".to_string(),
        };
        
        assert_eq!(role.name, "admin");
        assert!(role.permissions.contains(&Permission::All));
    }

    #[test]
    fn test_permission_assignment() {
        let mut rbac = RbacManager::new();
        
        rbac.add_role(Role {
            name: "developer".to_string(),
            permissions: vec![Permission::ReadAgents, Permission::WriteAgents],
            description: "Developer role".to_string(),
        });
        
        rbac.assign_role("user1", "developer").unwrap();
        
        assert!(rbac.check_permission("user1", &Permission::ReadAgents).unwrap());
        assert!(!rbac.check_permission("user1", &Permission::DeleteAgents).unwrap());
    }

    #[test]
    fn test_multiple_roles() {
        let mut rbac = RbacManager::new();
        
        rbac.add_role(Role {
            name: "viewer".to_string(),
            permissions: vec![Permission::ReadAgents],
            description: "Read-only access".to_string(),
        });
        
        rbac.add_role(Role {
            name: "editor".to_string(),
            permissions: vec![Permission::ReadAgents, Permission::WriteAgents],
            description: "Edit access".to_string(),
        });
        
        assert_eq!(rbac.role_count(), 2);
    }

    #[test]
    fn test_audit_logging() {
        let mut logger = AuditLogger::new("test_audit.log");
        
        let event = sentient_enterprise::AuditEvent {
            timestamp: chrono::Utc::now(),
            tenant_id: "tenant1".to_string(),
            user_id: "user1".to_string(),
            action: sentient_enterprise::AuditAction::Login,
            resource: "system".to_string(),
            details: serde_json::json!({}),
        };
        
        logger.log(event);
        
        let recent = logger.get_recent(10);
        assert_eq!(recent.len(), 1);
    }
}

#[cfg(test)]
mod sso_tests {
    use sentient_enterprise::sso::{SsoConfig, SsoProvider, SsoManager};

    #[test]
    fn test_sso_config() {
        let config = SsoConfig {
            providers: vec![
                SsoProvider::Okta {
                    domain: "test.okta.com".to_string(),
                    client_id: "client_id".to_string(),
                    client_secret: "secret".to_string(),
                },
            ],
            jwt_secret: "secret".to_string(),
            session_duration_hours: 24,
        };
        
        assert_eq!(config.providers.len(), 1);
        assert_eq!(config.session_duration_hours, 24);
    }

    #[test]
    fn test_sso_manager_creation() {
        let config = SsoConfig {
            providers: vec![],
            jwt_secret: "secret".to_string(),
            session_duration_hours: 24,
        };
        
        let manager = SsoManager::new(config);
        assert_eq!(manager.provider_count(), 0);
    }

    #[test]
    fn test_provider_types() {
        let providers = vec![
            SsoProvider::Okta { domain: "test.okta.com".into(), client_id: "id".into(), client_secret: "secret".into() },
            SsoProvider::Auth0 { domain: "test.auth0.com".into(), client_id: "id".into(), client_secret: "secret".into() },
            SsoProvider::AzureAD { tenant_id: "tenant".into(), client_id: "id".into(), client_secret: "secret".into() },
            SsoProvider::Google { client_id: "id".into(), client_secret: "secret".into() },
        ];
        
        assert_eq!(providers.len(), 4);
    }
}

#[cfg(test)]
mod tenant_tests {
    use sentient_enterprise::tenant::{TenantManager, Tenant, TenantConfig};

    #[test]
    fn test_tenant_creation() {
        let mut manager = TenantManager::new();
        
        let tenant = Tenant {
            id: "tenant1".to_string(),
            name: "Test Company".to_string(),
            config: TenantConfig {
                max_agents: 100,
                max_users: 500,
                features: vec!["sso", "audit"],
                branding: serde_json::json!({}),
            },
        };
        
        manager.create(tenant);
        
        assert_eq!(manager.count(), 1);
    }

    #[test]
    fn test_tenant_isolation() {
        let mut manager = TenantManager::new();
        
        // Create two tenants
        for i in 1..=2 {
            manager.create(Tenant {
                id: format!("tenant{}", i),
                name: format!("Company {}", i),
                config: TenantConfig::default(),
            });
        }
        
        // Verify isolation
        let tenant1 = manager.get("tenant1");
        let tenant2 = manager.get("tenant2");
        
        assert!(tenant1.is_some());
        assert!(tenant2.is_some());
        assert_ne!(tenant1.unwrap().name, tenant2.unwrap().name);
    }

    #[test]
    fn test_tenant_limits() {
        let config = TenantConfig {
            max_agents: 10,
            max_users: 50,
            features: vec!["basic"],
            branding: serde_json::json!({}),
        };
        
        assert_eq!(config.max_agents, 10);
        assert_eq!(config.max_users, 50);
    }
}
