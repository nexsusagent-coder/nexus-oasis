//! ═══════════════════════════════════════════════════════════════════════════════
//!  SCIM (System for Cross-domain Identity Management) Provisioning
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! SCIM 2.0 protocol implementation for enterprise user provisioning:
//! - User provisioning/deprovisioning
//! - Group management
//! - Schema discovery
//! - Bulk operations
//! - Filtering and pagination

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

// ═══════════════════════════════════════════════════════════════════════════════
//  SCIM SCHEMA DEFINITIONS
// ═══════════════════════════════════════════════════════════════════════════════

/// SCIM schema identifier
pub const SCIM_CORE_USER: &str = "urn:ietf:params:scim:schemas:core:2.0:User";
pub const SCIM_CORE_GROUP: &str = "urn:ietf:params:scim:schemas:core:2.0:Group";
pub const SCIM_ENTERPRISE_USER: &str = "urn:ietf:params:scim:schemas:extension:enterprise:2.0:User";

/// SCIM User resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScimUser {
    /// Schemas
    pub schemas: Vec<String>,
    /// Unique identifier
    pub id: String,
    /// External identifier (from IdP)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// User meta information
    pub meta: ScimMeta,
    /// User name
    pub name: ScimUserName,
    /// Display name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// Username
    pub user_name: String,
    /// Email addresses
    #[serde(default)]
    pub emails: Vec<ScimEmail>,
    /// Phone numbers
    #[serde(default)]
    pub phone_numbers: Vec<ScimPhone>,
    /// User status (active/inactive)
    #[serde(default = "default_active")]
    pub active: bool,
    /// Preferred language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_language: Option<String>,
    /// Locale
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    /// Timezone
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// Groups the user belongs to
    #[serde(default)]
    pub groups: Vec<ScimGroupRef>,
    /// Enterprise extension
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "urn:ietf:params:scim:schemas:extension:enterprise:2.0:User")]
    pub enterprise: Option<ScimEnterpriseUser>,
}

fn default_active() -> bool { true }

/// User name components
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScimUserName {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formatted: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub honorific_prefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub honorific_suffix: Option<String>,
}

/// Email address
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScimEmail {
    pub value: String,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
}

/// Phone number
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScimPhone {
    pub value: String,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_type: Option<String>,
}

/// SCIM Group resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScimGroup {
    pub schemas: Vec<String>,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    pub meta: ScimMeta,
    pub display_name: String,
    #[serde(default)]
    pub members: Vec<ScimMemberRef>,
}

/// Meta information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScimMeta {
    pub resource_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// Group reference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScimGroupRef {
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "$ref")]
    pub ref_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_type: Option<String>,
}

/// Member reference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScimMemberRef {
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "$ref")]
    pub ref_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_type: Option<String>,
}

/// Enterprise user extension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScimEnterpriseUser {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_center: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub division: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manager: Option<ScimManager>,
}

/// Manager reference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScimManager {
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "$ref")]
    pub ref_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  SCIM LIST RESPONSE
// ═══════════════════════════════════════════════════════════════════════════════

/// Paginated list response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScimListResponse<T> {
    pub schemas: Vec<String>,
    pub total_results: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_index: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items_per_page: Option<i64>,
    pub resources: Vec<T>,
}

/// Error response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScimErrorResponse {
    pub schemas: Vec<String>,
    #[serde(rename = "scimType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scim_type: Option<String>,
    pub detail: String,
    pub status: u16,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  SCIM FILTER PARSER
// ═══════════════════════════════════════════════════════════════════════════════

/// Filter expression
#[derive(Debug, Clone)]
pub enum ScimFilter {
    /// Attribute eq "value"
    Equals { attribute: String, value: String },
    /// Attribute ne "value"
    NotEquals { attribute: String, value: String },
    /// Attribute co "value" (contains)
    Contains { attribute: String, value: String },
    /// Attribute sw "value" (starts with)
    StartsWith { attribute: String, value: String },
    /// Attribute ew "value" (ends with)
    EndsWith { attribute: String, value: String },
    /// Attribute pr (present)
    Present { attribute: String },
    /// And combination
    And(Box<ScimFilter>, Box<ScimFilter>),
    /// Or combination
    Or(Box<ScimFilter>, Box<ScimFilter>),
    /// Not negation
    Not(Box<ScimFilter>),
}

impl ScimFilter {
    /// Parse a SCIM filter string
    pub fn parse(filter: &str) -> Option<Self> {
        let filter = filter.trim();
        
        if filter.is_empty() {
            return None;
        }
        
        // Handle parentheses
        let filter = filter.trim_start_matches('(').trim_end_matches(')');
        
        // Handle NOT
        if filter.starts_with("not ") {
            let inner = Self::parse(&filter[4..])?;
            return Some(Self::Not(Box::new(inner)));
        }
        
        // Handle AND
        if let Some(pos) = Self::find_operator(filter, "and") {
            let left = Self::parse(&filter[..pos])?;
            let right = Self::parse(&filter[pos + 3..])?;
            return Some(Self::And(Box::new(left), Box::new(right)));
        }
        
        // Handle OR
        if let Some(pos) = Self::find_operator(filter, "or") {
            let left = Self::parse(&filter[..pos])?;
            let right = Self::parse(&filter[pos + 2..])?;
            return Some(Self::Or(Box::new(left), Box::new(right)));
        }
        
        // Handle operators
        if let Some(pos) = filter.find(" eq ") {
            let attr = filter[..pos].trim().to_string();
            let value = Self::extract_value(&filter[pos + 4..])?;
            return Some(Self::Equals { attribute: attr, value });
        }
        
        if let Some(pos) = filter.find(" ne ") {
            let attr = filter[..pos].trim().to_string();
            let value = Self::extract_value(&filter[pos + 4..])?;
            return Some(Self::NotEquals { attribute: attr, value });
        }
        
        if let Some(pos) = filter.find(" co ") {
            let attr = filter[..pos].trim().to_string();
            let value = Self::extract_value(&filter[pos + 4..])?;
            return Some(Self::Contains { attribute: attr, value });
        }
        
        if let Some(pos) = filter.find(" sw ") {
            let attr = filter[..pos].trim().to_string();
            let value = Self::extract_value(&filter[pos + 4..])?;
            return Some(Self::StartsWith { attribute: attr, value });
        }
        
        if let Some(pos) = filter.find(" ew ") {
            let attr = filter[..pos].trim().to_string();
            let value = Self::extract_value(&filter[pos + 4..])?;
            return Some(Self::EndsWith { attribute: attr, value });
        }
        
        if filter.contains(" pr") {
            let attr = filter.replace(" pr", "").trim().to_string();
            return Some(Self::Present { attribute: attr });
        }
        
        None
    }
    
    fn find_operator(filter: &str, op: &str) -> Option<usize> {
        let mut depth = 0;
        let chars: Vec<char> = filter.chars().collect();
        let op_chars: Vec<char> = format!(" {} ", op).chars().collect();
        
        for i in 0..chars.len().saturating_sub(op_chars.len()) {
            if chars[i] == '(' { depth += 1; }
            else if chars[i] == ')' { depth -= 1; }
            else if depth == 0 {
                if chars[i..i + op_chars.len()] == op_chars[..] {
                    return Some(i);
                }
            }
        }
        None
    }
    
    fn extract_value(s: &str) -> Option<String> {
        let s = s.trim();
        if s.starts_with('"') && s.ends_with('"') {
            Some(s[1..s.len()-1].to_string())
        } else {
            Some(s.to_string())
        }
    }
    
    /// Evaluate filter against a user
    pub fn matches(&self, user: &ScimUser) -> bool {
        match self {
            Self::Equals { attribute, value } => {
                Self::get_attribute(user, attribute).map_or(false, |v| v == value.as_str())
            }
            Self::NotEquals { attribute, value } => {
                Self::get_attribute(user, attribute).map_or(true, |v| v != value.as_str())
            }
            Self::Contains { attribute, value } => {
                Self::get_attribute(user, attribute).map_or(false, |v| v.contains(value.as_str()))
            }
            Self::StartsWith { attribute, value } => {
                Self::get_attribute(user, attribute).map_or(false, |v| v.starts_with(value.as_str()))
            }
            Self::EndsWith { attribute, value } => {
                Self::get_attribute(user, attribute).map_or(false, |v| v.ends_with(value.as_str()))
            }
            Self::Present { attribute } => {
                Self::get_attribute(user, attribute).is_some()
            }
            Self::And(left, right) => {
                left.matches(user) && right.matches(user)
            }
            Self::Or(left, right) => {
                left.matches(user) || right.matches(user)
            }
            Self::Not(inner) => {
                !inner.matches(user)
            }
        }
    }
    
    fn get_attribute(user: &ScimUser, attr: &str) -> Option<String> {
        match attr {
            "userName" => Some(user.user_name.clone()),
            "displayName" => user.display_name.clone(),
            "active" => Some(user.active.to_string()),
            "id" => Some(user.id.clone()),
            "externalId" => user.external_id.clone(),
            "name.givenName" => user.name.given_name.clone(),
            "name.familyName" => user.name.family_name.clone(),
            "name.formatted" => user.name.formatted.clone(),
            _ => {
                // Check emails
                if attr.starts_with("emails") {
                    let parts: Vec<&str> = attr.split('.').collect();
                    if parts.len() == 2 && parts[1] == "value" {
                        return user.emails.first().map(|e| e.value.clone());
                    }
                }
                None
            }
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  SCIM SERVICE PROVIDER
// ═══════════════════════════════════════════════════════════════════════════════

/// SCIM service provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScimServiceProviderConfig {
    pub schemas: Vec<String>,
    pub patch: ScimFeature,
    pub bulk: ScimBulkFeature,
    pub filter: ScimFilterFeature,
    pub change_password: ScimFeature,
    pub sort: ScimSortFeature,
    pub etag: ScimFeature,
    pub authentication_schemes: Vec<ScimAuthenticationScheme>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScimFeature {
    pub supported: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScimBulkFeature {
    pub supported: bool,
    pub max_operations: i32,
    pub max_payload_size: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScimFilterFeature {
    pub supported: bool,
    pub max_results: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScimSortFeature {
    pub supported: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScimAuthenticationScheme {
    pub name: String,
    pub description: String,
    pub spec_uri: String,
    pub documentation_uri: String,
    #[serde(rename = "type")]
    pub auth_type: String,
    pub primary: bool,
}

impl Default for ScimServiceProviderConfig {
    fn default() -> Self {
        Self {
            schemas: vec!["urn:ietf:params:scim:schemas:core:2.0:ServiceProviderConfig".to_string()],
            patch: ScimFeature { supported: true },
            bulk: ScimBulkFeature {
                supported: true,
                max_operations: 100,
                max_payload_size: 1048576,
            },
            filter: ScimFilterFeature {
                supported: true,
                max_results: 1000,
            },
            change_password: ScimFeature { supported: true },
            sort: ScimSortFeature { supported: true },
            etag: ScimFeature { supported: true },
            authentication_schemes: vec![
                ScimAuthenticationScheme {
                    name: "OAuth Bearer Token".to_string(),
                    description: "OAuth 2.0 Bearer Token authentication".to_string(),
                    spec_uri: "http://www.rfc-editor.org/info/rfc6750".to_string(),
                    documentation_uri: String::new(),
                    auth_type: "oauthbearertoken".to_string(),
                    primary: true,
                },
            ],
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  SCIM PROVISIONING MANAGER
// ═══════════════════════════════════════════════════════════════════════════════

/// SCIM provisioning error
#[derive(Debug, thiserror::Error)]
pub enum ScimError {
    #[error("User not found: {0}")]
    UserNotFound(String),
    
    #[error("Group not found: {0}")]
    GroupNotFound(String),
    
    #[error("Invalid filter: {0}")]
    InvalidFilter(String),
    
    #[error("Duplicate user: {0}")]
    DuplicateUser(String),
    
    #[error("Schema validation failed: {0}")]
    SchemaValidation(String),
    
    #[error("Internal error: {0}")]
    Internal(String),
}

/// SCIM provisioning manager
pub struct ScimProvisioner {
    users: HashMap<String, ScimUser>,
    groups: HashMap<String, ScimGroup>,
    config: ScimServiceProviderConfig,
}

impl ScimProvisioner {
    /// Create a new SCIM provisioner
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
            groups: HashMap::new(),
            config: ScimServiceProviderConfig::default(),
        }
    }
    
    /// Get service provider config
    pub fn get_service_provider_config(&self) -> &ScimServiceProviderConfig {
        &self.config
    }
    
    /// Create a user
    pub fn create_user(&mut self, mut user: ScimUser) -> Result<ScimUser, ScimError> {
        // Validate schemas
        if !user.schemas.contains(&SCIM_CORE_USER.to_string()) {
            user.schemas.push(SCIM_CORE_USER.to_string());
        }
        
        // Check for duplicate
        if self.users.values().any(|u| u.user_name == user.user_name) {
            return Err(ScimError::DuplicateUser(user.user_name));
        }
        
        // Generate ID if not present
        if user.id.is_empty() {
            user.id = uuid::Uuid::new_v4().to_string();
        }
        
        // Set meta
        user.meta = ScimMeta {
            resource_type: "User".to_string(),
            created: Some(Utc::now()),
            last_modified: Some(Utc::now()),
            location: Some(format!("/scim/v2/Users/{}", user.id)),
            version: Some("1".to_string()),
        };
        
        self.users.insert(user.id.clone(), user.clone());
        Ok(user)
    }
    
    /// Get a user by ID
    pub fn get_user(&self, id: &str) -> Result<ScimUser, ScimError> {
        self.users.get(id).cloned()
            .ok_or_else(|| ScimError::UserNotFound(id.to_string()))
    }
    
    /// Update a user
    pub fn update_user(&mut self, id: &str, mut user: ScimUser) -> Result<ScimUser, ScimError> {
        if !self.users.contains_key(id) {
            return Err(ScimError::UserNotFound(id.to_string()));
        }
        
        user.id = id.to_string();
        user.meta.last_modified = Some(Utc::now());
        user.meta.version = Some(
            (user.meta.version.as_ref()
                .and_then(|v| v.parse::<i32>().ok())
                .unwrap_or(0) + 1)
                .to_string()
        );
        
        self.users.insert(id.to_string(), user.clone());
        Ok(user)
    }
    
    /// Delete a user
    pub fn delete_user(&mut self, id: &str) -> Result<(), ScimError> {
        self.users.remove(id)
            .map(|_| ())
            .ok_or_else(|| ScimError::UserNotFound(id.to_string()))
    }
    
    /// List users with optional filter
    pub fn list_users(&self, filter: Option<&str>, start_index: i64, count: i64) -> ScimListResponse<ScimUser> {
        let parsed_filter = filter.and_then(|f| ScimFilter::parse(f));
        
        let filtered: Vec<ScimUser> = self.users.values()
            .filter(|u| {
                if let Some(ref f) = parsed_filter {
                    f.matches(u)
                } else {
                    true
                }
            })
            .cloned()
            .collect();
        
        let total = filtered.len() as i64;
        let start = ((start_index - 1).max(0) as usize).min(filtered.len());
        let end = (start + count as usize).min(filtered.len());
        
        ScimListResponse {
            schemas: vec!["urn:ietf:params:scim:api:messages:2.0:ListResponse".to_string()],
            total_results: total,
            start_index: Some(start_index),
            items_per_page: Some(count),
            resources: filtered[start..end].to_vec(),
        }
    }
    
    /// Create a group
    pub fn create_group(&mut self, mut group: ScimGroup) -> Result<ScimGroup, ScimError> {
        if !group.schemas.contains(&SCIM_CORE_GROUP.to_string()) {
            group.schemas.push(SCIM_CORE_GROUP.to_string());
        }
        
        if group.id.is_empty() {
            group.id = uuid::Uuid::new_v4().to_string();
        }
        
        group.meta = ScimMeta {
            resource_type: "Group".to_string(),
            created: Some(Utc::now()),
            last_modified: Some(Utc::now()),
            location: Some(format!("/scim/v2/Groups/{}", group.id)),
            version: Some("1".to_string()),
        };
        
        self.groups.insert(group.id.clone(), group.clone());
        Ok(group)
    }
    
    /// Get a group by ID
    pub fn get_group(&self, id: &str) -> Result<ScimGroup, ScimError> {
        self.groups.get(id).cloned()
            .ok_or_else(|| ScimError::GroupNotFound(id.to_string()))
    }
    
    /// Add user to group
    pub fn add_user_to_group(&mut self, group_id: &str, user_id: &str) -> Result<(), ScimError> {
        let user = self.get_user(user_id)?;
        let group = self.groups.get_mut(group_id)
            .ok_or_else(|| ScimError::GroupNotFound(group_id.to_string()))?;
        
        if !group.members.iter().any(|m| m.value == user_id) {
            group.members.push(ScimMemberRef {
                value: user_id.to_string(),
                ref_: Some(format!("/scim/v2/Users/{}", user_id)),
                display: user.display_name.clone(),
                member_type: Some("User".to_string()),
            });
            group.meta.last_modified = Some(Utc::now());
        }
        
        Ok(())
    }
    
    /// Remove user from group
    pub fn remove_user_from_group(&mut self, group_id: &str, user_id: &str) -> Result<(), ScimError> {
        let group = self.groups.get_mut(group_id)
            .ok_or_else(|| ScimError::GroupNotFound(group_id.to_string()))?;
        
        group.members.retain(|m| m.value != user_id);
        group.meta.last_modified = Some(Utc::now());
        
        Ok(())
    }
}

impl Default for ScimProvisioner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_create_user() {
        let mut provisioner = ScimProvisioner::new();
        
        let user = ScimUser {
            schemas: vec![SCIM_CORE_USER.to_string()],
            id: String::new(),
            external_id: None,
            meta: ScimMeta {
                resource_type: "User".to_string(),
                created: None,
                last_modified: None,
                location: None,
                version: None,
            },
            name: ScimUserName {
                formatted: Some("John Doe".to_string()),
                family_name: Some("Doe".to_string()),
                given_name: Some("John".to_string()),
                middle_name: None,
                honorific_prefix: None,
                honorific_suffix: None,
            },
            display_name: Some("John Doe".to_string()),
            user_name: "john.doe@example.com".to_string(),
            emails: vec![ScimEmail {
                value: "john.doe@example.com".to_string(),
                email_type: Some("work".to_string()),
                primary: Some(true),
            }],
            phone_numbers: vec![],
            active: true,
            preferred_language: Some("en".to_string()),
            locale: Some("en_US".to_string()),
            timezone: Some("America/New_York".to_string()),
            groups: vec![],
            enterprise: None,
        };
        
        let result = provisioner.create_user(user).unwrap();
        assert!(!result.id.is_empty());
    }
    
    #[test]
    fn test_filter_parsing() {
        let filter = ScimFilter::parse("userName eq \"john.doe@example.com\"").unwrap();
        assert!(matches!(filter, ScimFilter::Equals { .. }));
    }
    
    #[test]
    fn test_filter_matching() {
        let mut provisioner = ScimProvisioner::new();
        
        let user = ScimUser {
            schemas: vec![SCIM_CORE_USER.to_string()],
            id: String::new(),
            external_id: None,
            meta: ScimMeta {
                resource_type: "User".to_string(),
                created: None,
                last_modified: None,
                location: None,
                version: None,
            },
            name: ScimUserName {
                formatted: None,
                family_name: None,
                given_name: None,
                middle_name: None,
                honorific_prefix: None,
                honorific_suffix: None,
            },
            display_name: None,
            user_name: "john.doe@example.com".to_string(),
            emails: vec![],
            phone_numbers: vec![],
            active: true,
            preferred_language: None,
            locale: None,
            timezone: None,
            groups: vec![],
            enterprise: None,
        };
        
        let user = provisioner.create_user(user).unwrap();
        
        let filter = ScimFilter::parse("userName eq \"john.doe@example.com\"").unwrap();
        assert!(filter.matches(&user));
        
        let filter2 = ScimFilter::parse("userName eq \"other@example.com\"").unwrap();
        assert!(!filter2.matches(&user));
    }
}
