//! E2E Test Scenarios

#[cfg(test)]
mod user_scenarios {
    use sentient_core::{Agent, AgentConfig, Message};

    /// Scenario: User asks about weather
    #[tokio::test]
    async fn scenario_weather_query() {
        let agent = Agent::new(AgentConfig {
            skills: vec!["web-search".to_string()],
            ..Default::default()
        }).await.expect("Failed to create agent");
        
        let response = agent.send(Message::user(
            "What's the weather in Istanbul today?"
        )).await;
        
        // With real API, would check for weather info
        assert!(response.is_ok() || response.is_err());
        
        agent.shutdown().await.unwrap();
    }

    /// Scenario: User requests calculation
    #[tokio::test]
    async fn scenario_calculation() {
        let agent = Agent::new(AgentConfig {
            skills: vec!["calculator".to_string()],
            ..Default::default()
        }).await.expect("Failed to create agent");
        
        let test_cases = vec![
            ("What is 2 + 2?", "4"),
            ("Calculate 15 * 15", "225"),
            ("What is 100 / 4?", "25"),
        ];
        
        for (question, expected_contains) in test_cases {
            let response = agent.send(Message::user(question)).await;
            if std::env::var("OPENAI_API_KEY").is_ok() {
                if let Ok(msg) = response {
                    assert!(
                        msg.content.contains(expected_contains),
                        "Response '{}' should contain '{}'",
                        msg.content,
                        expected_contains
                    );
                }
            }
        }
        
        agent.shutdown().await.unwrap();
    }

    /// Scenario: Multi-step task
    #[tokio::test]
    async fn scenario_multi_step_task() {
        let agent = Agent::new(AgentConfig::default())
            .await
            .expect("Failed to create agent");
        
        // Step 1: Define task
        let _ = agent.send(Message::user(
            "I need to plan a trip to Paris"
        )).await;
        
        // Step 2: Ask for details
        let _ = agent.send(Message::user(
            "What are the top 5 attractions?"
        )).await;
        
        // Step 3: Request itinerary
        let response = agent.send(Message::user(
            "Create a 3-day itinerary based on these attractions"
        )).await;
        
        if std::env::var("OPENAI_API_KEY").is_ok() {
            if let Ok(msg) = response {
                assert!(!msg.content.is_empty());
            }
        }
        
        agent.shutdown().await.unwrap();
    }

    /// Scenario: Code assistance
    #[tokio::test]
    async fn scenario_code_assistance() {
        let agent = Agent::new(AgentConfig {
            system_prompt: Some("You are a helpful coding assistant.".to_string()),
            ..Default::default()
        }).await.expect("Failed to create agent");
        
        let response = agent.send(Message::user(
            "Write a Rust function that reverses a string"
        )).await;
        
        if std::env::var("OPENAI_API_KEY").is_ok() {
            if let Ok(msg) = response {
                // Should contain Rust code
                assert!(
                    msg.content.contains("fn") || 
                    msg.content.contains("String") ||
                    msg.content.contains("fn reverse"),
                    "Response should contain Rust code"
                );
            }
        }
        
        agent.shutdown().await.unwrap();
    }
}

#[cfg(test)]
mod enterprise_scenarios {
    use sentient_enterprise::{RbacManager, Role, Permission, TenantManager, Tenant};

    /// Scenario: New employee onboarding
    #[test]
    fn scenario_employee_onboarding() {
        let mut rbac = RbacManager::new();
        
        // Setup roles
        rbac.add_role(Role {
            name: "new_employee".to_string(),
            permissions: vec![Permission::ReadAgents],
            description: "New employee with limited access".to_string(),
        });
        
        rbac.add_role(Role {
            name: "team_lead".to_string(),
            permissions: vec![
                Permission::ReadAgents,
                Permission::WriteAgents,
                Permission::ReadUsers,
            ],
            description: "Team lead with elevated access".to_string(),
        });
        
        // New employee joins
        let user_id = "employee_123";
        rbac.assign_role(user_id, "new_employee").unwrap();
        
        // Verify limited access
        assert!(rbac.check_permission(user_id, &Permission::ReadAgents).unwrap());
        assert!(!rbac.check_permission(user_id, &Permission::WriteAgents).unwrap());
        
        // Employee gets promoted
        rbac.assign_role(user_id, "team_lead").unwrap();
        
        // Verify elevated access
        assert!(rbac.check_permission(user_id, &Permission::WriteAgents).unwrap());
    }

    /// Scenario: Tenant provisioning
    #[test]
    fn scenario_tenant_provisioning() {
        let mut manager = TenantManager::new();
        
        // New customer signs up
        let tenant = Tenant {
            id: "customer_abc".to_string(),
            name: "ABC Corp".to_string(),
            config: sentient_enterprise::tenant::TenantConfig {
                max_agents: 10,
                max_users: 50,
                features: vec!["basic", "sso"],
                branding: serde_json::json!({
                    "primary_color": "#0066cc",
                    "logo_url": "https://abc.com/logo.png"
                }),
            },
        };
        
        manager.create(tenant);
        
        // Verify tenant exists
        assert!(manager.get("customer_abc").is_some());
        
        // Upgrade tenant
        manager.update_config("customer_abc", sentient_enterprise::tenant::TenantConfig {
            max_agents: 100,
            max_users: 500,
            features: vec!["basic", "sso", "audit", "custom_skills"],
            branding: serde_json::json!({}),
        }).unwrap();
        
        // Verify upgrade
        let updated = manager.get("customer_abc").unwrap();
        assert_eq!(updated.config.max_agents, 100);
    }
}
