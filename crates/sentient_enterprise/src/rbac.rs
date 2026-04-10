//! Role-Based Access Control (RBAC) module
//!
//! Provides fine-grained access control with:
//! - Roles and Permissions
//! - User/Group management
//! - Resource-based access control

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use thiserror::Error;

/// Role in the system
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Role {
    /// Full administrative access
    Admin,
    /// Team management capabilities
    Manager,
    /// Agent development access
    Developer,
    /// Read + analytics access
    Analyst,
    /// Read-only access
    Viewer,
    /// Custom role with specific permissions
    Custom(String),
}

impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Role::Admin => write!(f, "admin"),
            Role::Manager => write!(f, "manager"),
            Role::Developer => write!(f, "developer"),
            Role::Analyst => write!(f, "analyst"),
            Role::Viewer => write!(f, "viewer"),
            Role::Custom(name) => write!(f, "custom:{}", name),
        }
    }
}

impl std::str::FromStr for Role {
    type Err = RBACError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "admin" => Ok(Role::Admin),
            "manager" => Ok(Role::Manager),
            "developer" => Ok(Role::Developer),
            "analyst" => Ok(Role::Analyst),
            "viewer" => Ok(Role::Viewer),
            s if s.starts_with("custom:") => {
                let name = s.strip_prefix("custom:").expect("operation failed");
                Ok(Role::Custom(name.to_string()))
            }
            _ => Err(RBACError::InvalidRole(s.to_string())),
        }
    }
}

/// Action that can be performed on a resource
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Action {
    Create,
    Read,
    Update,
    Delete,
    Execute,
    Manage,
    All,
}

/// Permission definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Permission {
    /// Resource pattern (supports wildcards)
    pub resource: String,
    /// Allowed actions
    pub actions: Vec<Action>,
    /// Optional conditions
    pub conditions: Vec<Condition>,
}

impl Permission {
    /// Create a new permission
    pub fn new(resource: impl Into<String>, actions: Vec<Action>) -> Self {
        Self {
            resource: resource.into(),
            actions,
            conditions: vec![],
        }
    }

    /// Add a condition
    pub fn with_condition(mut self, condition: Condition) -> Self {
        self.conditions.push(condition);
        self
    }

    /// Check if this permission matches a resource and action
    pub fn matches(&self, resource: &str, action: &Action) -> bool {
        // Check resource pattern
        if !self.matches_resource(resource) {
            return false;
        }

        // Check action
        if self.actions.contains(&Action::All) || self.actions.contains(action) {
            // Check conditions (if any)
            // For now, we'll assume conditions are checked elsewhere
            return true;
        }

        false
    }

    fn matches_resource(&self, resource: &str) -> bool {
        // Simple wildcard matching
        if self.resource == "*" {
            return true;
        }

        if self.resource.ends_with("/*") {
            let prefix = &self.resource[..self.resource.len() - 2];
            return resource.starts_with(prefix);
        }

        self.resource == resource
    }
}

/// Condition for permission
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Condition {
    /// Time-based restriction
    TimeRange {
        start: String, // HH:MM
        end: String,   // HH:MM
    },
    /// IP-based restriction
    IpRange {
        allowed: Vec<String>,
    },
    /// Attribute-based condition
    Attribute {
        key: String,
        value: String,
    },
    /// Custom condition evaluated by external service
    Custom {
        service: String,
        parameters: serde_json::Value,
    },
}

/// RBAC Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RBACConfig {
    /// Default role for new users
    pub default_role: Role,
    /// Role definitions with permissions
    pub roles: HashMap<Role, Vec<Permission>>,
    /// Super admin users (bypass all checks)
    pub super_admins: HashSet<String>,
}

impl Default for RBACConfig {
    fn default() -> Self {
        let mut roles = HashMap::new();

        // Admin: Full access
        roles.insert(Role::Admin, vec![
            Permission::new("*", vec![Action::All]),
        ]);

        // Manager: Team and agent management
        roles.insert(Role::Manager, vec![
            Permission::new("agents/*", vec![Action::All]),
            Permission::new("users/*", vec![Action::Read, Action::Update]),
            Permission::new("channels/*", vec![Action::All]),
            Permission::new("analytics/*", vec![Action::Read]),
        ]);

        // Developer: Agent development
        roles.insert(Role::Developer, vec![
            Permission::new("agents/*", vec![Action::Create, Action::Read, Action::Update, Action::Execute]),
            Permission::new("channels/*", vec![Action::Read]),
            Permission::new("skills/*", vec![Action::All]),
        ]);

        // Analyst: Read + analytics
        roles.insert(Role::Analyst, vec![
            Permission::new("agents/*", vec![Action::Read]),
            Permission::new("analytics/*", vec![Action::Read]),
            Permission::new("logs/*", vec![Action::Read]),
        ]);

        // Viewer: Read-only
        roles.insert(Role::Viewer, vec![
            Permission::new("agents/*", vec![Action::Read]),
            Permission::new("channels/*", vec![Action::Read]),
        ]);

        Self {
            default_role: Role::Viewer,
            roles,
            super_admins: HashSet::new(),
        }
    }
}

/// RBAC Error types
#[derive(Debug, Error)]
pub enum RBACError {
    #[error("Invalid role: {0}")]
    InvalidRole(String),

    #[error("Permission denied for {resource}:{action}")]
    PermissionDenied { resource: String, action: String },

    #[error("User not found: {0}")]
    UserNotFound(String),

    #[error("Role not found: {0}")]
    RoleNotFound(String),

    #[error("Database error: {0}")]
    DatabaseError(String),
}

/// RBAC Manager
pub struct RBACManager {
    config: RBACConfig,
    user_roles: HashMap<String, HashSet<Role>>,
    group_members: HashMap<String, HashSet<String>>,
    group_roles: HashMap<String, HashSet<Role>>,
}

impl RBACManager {
    /// Create a new RBAC manager
    pub async fn new(config: RBACConfig) -> Result<Self, RBACError> {
        Ok(Self {
            config,
            user_roles: HashMap::new(),
            group_members: HashMap::new(),
            group_roles: HashMap::new(),
        })
    }

    /// Get roles for a user
    pub async fn get_user_roles(&self, user_id: &str) -> Result<Vec<Role>, RBACError> {
        // Check super admin
        if self.config.super_admins.contains(user_id) {
            return Ok(vec![Role::Admin]);
        }

        // Get direct roles
        let mut roles = self.user_roles
            .get(user_id)
            .cloned()
            .unwrap_or_default();

        // Add roles from groups
        for (group_id, members) in &self.group_members {
            if members.contains(user_id) {
                if let Some(group_roles) = self.group_roles.get(group_id) {
                    roles.extend(group_roles.iter().cloned());
                }
            }
        }

        // If no roles, use default
        if roles.is_empty() {
            roles.insert(self.config.default_role.clone());
        }

        Ok(roles.into_iter().collect())
    }

    /// Check if a role has permission
    pub async fn has_permission(
        &self,
        role: &Role,
        resource: &str,
        action: &str,
    ) -> Result<bool, RBACError> {
        let action = Self::parse_action(action)?;

        // Get permissions for role
        let permissions = self.config.roles.get(role).ok_or_else(|| {
            RBACError::RoleNotFound(role.to_string())
        })?;

        // Check each permission
        for permission in permissions {
            if permission.matches(resource, &action) {
                return Ok(true);
            }
        }

        Ok(false)
    }

    /// Assign role to user
    pub async fn assign_role(
        &mut self,
        user_id: &str,
        role: Role,
    ) -> Result<(), RBACError> {
        self.user_roles
            .entry(user_id.to_string())
            .or_insert_with(HashSet::new)
            .insert(role);
        Ok(())
    }

    /// Remove role from user
    pub async fn remove_role(
        &mut self,
        user_id: &str,
        role: &Role,
    ) -> Result<(), RBACError> {
        if let Some(roles) = self.user_roles.get_mut(user_id) {
            roles.remove(role);
        }
        Ok(())
    }

    /// Add user to group
    pub async fn add_user_to_group(
        &mut self,
        user_id: &str,
        group_id: &str,
    ) -> Result<(), RBACError> {
        self.group_members
            .entry(group_id.to_string())
            .or_insert_with(HashSet::new)
            .insert(user_id.to_string());
        Ok(())
    }

    /// Assign role to group
    pub async fn assign_role_to_group(
        &mut self,
        group_id: &str,
        role: Role,
    ) -> Result<(), RBACError> {
        self.group_roles
            .entry(group_id.to_string())
            .or_insert_with(HashSet::new)
            .insert(role);
        Ok(())
    }

    /// Create a custom role
    pub async fn create_custom_role(
        &mut self,
        name: &str,
        permissions: Vec<Permission>,
    ) -> Result<Role, RBACError> {
        let role = Role::Custom(name.to_string());
        self.config.roles.insert(role.clone(), permissions);
        Ok(role)
    }

    fn parse_action(s: &str) -> Result<Action, RBACError> {
        match s.to_lowercase().as_str() {
            "create" => Ok(Action::Create),
            "read" => Ok(Action::Read),
            "update" => Ok(Action::Update),
            "delete" => Ok(Action::Delete),
            "execute" => Ok(Action::Execute),
            "manage" => Ok(Action::Manage),
            "all" | "*" => Ok(Action::All),
            _ => Err(RBACError::PermissionDenied {
                resource: "action".to_string(),
                action: s.to_string(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_role_display() {
        assert_eq!(Role::Admin.to_string(), "admin");
        assert_eq!(Role::Custom("custom_role".to_string()).to_string(), "custom:custom_role");
    }

    #[test]
    fn test_permission_matching() {
        let perm = Permission::new("agents/*", vec![Action::Read, Action::Execute]);

        assert!(perm.matches("agents/chat", &Action::Read));
        assert!(perm.matches("agents/voice", &Action::Execute));
        assert!(!perm.matches("agents/chat", &Action::Delete));
        assert!(!perm.matches("users/chat", &Action::Read));
    }

    #[test]
    fn test_wildcard_permission() {
        let perm = Permission::new("*", vec![Action::All]);

        assert!(perm.matches("anything", &Action::Create));
        assert!(perm.matches("anything", &Action::Delete));
    }

    #[tokio::test]
    async fn test_rbac_manager() {
        let config = RBACConfig::default();
        let manager = RBACManager::new(config).await.expect("operation failed");

        // Admin should have all permissions
        let has_perm = manager.has_permission(&Role::Admin, "anything", "all").await.expect("operation failed");
        assert!(has_perm);

        // Viewer should only have read access
        let has_read = manager.has_permission(&Role::Viewer, "agents/test", "read").await.expect("operation failed");
        assert!(has_read);

        let has_delete = manager.has_permission(&Role::Viewer, "agents/test", "delete").await.expect("operation failed");
        assert!(!has_delete);
    }
}
