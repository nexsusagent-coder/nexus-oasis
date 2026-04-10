//! Core types for web server

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// User ID type
pub type UserId = Uuid;

/// Session ID type
pub type SessionId = Uuid;

/// API Key ID type
pub type ApiKeyId = Uuid;

/// Server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    /// Server host
    pub host: String,
    /// Server port
    pub port: u16,
    /// Enable CORS
    pub cors: bool,
    /// CORS allowed origins
    pub cors_origins: Vec<String>,
    /// Enable authentication
    pub auth_enabled: bool,
    /// JWT secret
    pub jwt_secret: String,
    /// JWT expiration (seconds)
    pub jwt_expiration: u64,
    /// Enable rate limiting
    pub rate_limit: bool,
    /// Rate limit (requests per minute)
    pub rate_limit_per_minute: u32,
    /// Enable compression
    pub compression: bool,
    /// Dashboard path
    pub dashboard_path: Option<String>,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 8080,
            cors: true,
            cors_origins: vec!["*".to_string()],
            auth_enabled: false,
            jwt_secret: "secret".to_string(),
            jwt_expiration: 3600,
            rate_limit: false,
            rate_limit_per_minute: 60,
            compression: true,
            dashboard_path: None,
        }
    }
}

impl ServerConfig {
    /// Create new config
    pub fn new(port: u16) -> Self {
        Self {
            port,
            ..Default::default()
        }
    }

    /// With host
    pub fn with_host(mut self, host: impl Into<String>) -> Self {
        self.host = host.into();
        self
    }

    /// With authentication
    pub fn with_auth(mut self, secret: impl Into<String>) -> Self {
        self.auth_enabled = true;
        self.jwt_secret = secret.into();
        self
    }

    /// With CORS origins
    pub fn with_cors(mut self, origins: Vec<String>) -> Self {
        self.cors = true;
        self.cors_origins = origins;
        self
    }

    /// With rate limiting
    pub fn with_rate_limit(mut self, per_minute: u32) -> Self {
        self.rate_limit = true;
        self.rate_limit_per_minute = per_minute;
        self
    }

    /// Listen address
    pub fn listen_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

/// User information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// User ID
    pub id: UserId,
    /// Username
    pub username: String,
    /// Email
    pub email: Option<String>,
    /// Roles
    pub roles: Vec<String>,
    /// Created at
    pub created_at: DateTime<Utc>,
    /// Last login
    pub last_login: Option<DateTime<Utc>>,
}

impl User {
    /// Create new user
    pub fn new(username: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            username: username.into(),
            email: None,
            roles: vec!["user".to_string()],
            created_at: Utc::now(),
            last_login: None,
        }
    }

    /// With email
    pub fn with_email(mut self, email: impl Into<String>) -> Self {
        self.email = Some(email.into());
        self
    }

    /// With roles
    pub fn with_roles(mut self, roles: Vec<String>) -> Self {
        self.roles = roles;
        self
    }

    /// Check if user has role
    pub fn has_role(&self, role: &str) -> bool {
        self.roles.iter().any(|r| r == role)
    }

    /// Check if user is admin
    pub fn is_admin(&self) -> bool {
        self.has_role("admin")
    }
}

/// Authentication claims
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    /// Subject (user ID)
    pub sub: String,
    /// Username
    pub username: String,
    /// Roles
    pub roles: Vec<String>,
    /// Issued at
    pub iat: u64,
    /// Expiration
    pub exp: u64,
}

impl Claims {
    /// Create new claims
    pub fn new(user: &User, expiration_secs: u64) -> Self {
        let now = chrono::Utc::now().timestamp() as u64;

        Self {
            sub: user.id.to_string(),
            username: user.username.clone(),
            roles: user.roles.clone(),
            iat: now,
            exp: now + expiration_secs,
        }
    }

    /// Check if expired
    pub fn is_expired(&self) -> bool {
        let now = chrono::Utc::now().timestamp() as u64;
        self.exp < now
    }
}

/// API Key
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKey {
    /// Key ID
    pub id: ApiKeyId,
    /// User ID
    pub user_id: UserId,
    /// Key name
    pub name: String,
    /// Key hash (not the actual key)
    pub key_hash: String,
    /// Permissions
    pub permissions: Vec<String>,
    /// Created at
    pub created_at: DateTime<Utc>,
    /// Last used
    pub last_used: Option<DateTime<Utc>>,
    /// Expires at
    pub expires_at: Option<DateTime<Utc>>,
}

impl ApiKey {
    /// Create new API key
    pub fn new(user_id: UserId, name: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            name: name.into(),
            key_hash: String::new(),
            permissions: vec!["read".to_string()],
            created_at: Utc::now(),
            last_used: None,
            expires_at: None,
        }
    }

    /// Generate key string
    pub fn generate_key() -> String {
        format!("sk-{}", Uuid::new_v4())
    }
}

/// API Response wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    /// Success status
    pub success: bool,
    /// Response data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    /// Error message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Metadata
    #[serde(default)]
    pub meta: ResponseMeta,
}

impl<T: Serialize> ApiResponse<T> {
    /// Success response
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            meta: ResponseMeta::default(),
        }
    }

    /// Error response
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message.into()),
            meta: ResponseMeta::default(),
        }
    }
}

/// Response metadata
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResponseMeta {
    /// Timestamp
    pub timestamp: String,
    /// Request ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// Pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Pagination>,
}

impl ResponseMeta {
    /// Create with request ID
    pub fn with_request_id(id: impl Into<String>) -> Self {
        Self {
            timestamp: chrono::Utc::now().to_rfc3339(),
            request_id: Some(id.into()),
            pagination: None,
        }
    }
}

/// Pagination info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pagination {
    /// Current page
    pub page: u32,
    /// Items per page
    pub per_page: u32,
    /// Total items
    pub total: u64,
    /// Total pages
    pub total_pages: u32,
}

impl Pagination {
    /// Create new pagination
    pub fn new(page: u32, per_page: u32, total: u64) -> Self {
        let total_pages = ((total as f64) / (per_page as f64)).ceil() as u32;

        Self {
            page,
            per_page,
            total,
            total_pages,
        }
    }

    /// Calculate offset
    pub fn offset(&self) -> u64 {
        ((self.page - 1) * self.per_page) as u64
    }
}

/// WebSocket message
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WsMessage {
    /// Chat message
    Chat {
        content: String,
        conversation_id: Option<String>,
    },
    /// Stream chunk
    StreamChunk {
        content: String,
        done: bool,
    },
    /// Error
    Error {
        message: String,
    },
    /// Ping
    Ping,
    /// Pong
    Pong,
    /// Status update
    Status {
        status: String,
        data: HashMap<String, serde_json::Value>,
    },
}

/// Health check response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResponse {
    /// Status
    pub status: String,
    /// Version
    pub version: String,
    /// Uptime seconds
    pub uptime_secs: u64,
    /// Components
    pub components: HashMap<String, ComponentHealth>,
}

impl HealthResponse {
    /// Create healthy response
    pub fn healthy(version: impl Into<String>, uptime: u64) -> Self {
        Self {
            status: "healthy".to_string(),
            version: version.into(),
            uptime_secs: uptime,
            components: HashMap::new(),
        }
    }

    /// Add component
    pub fn with_component(mut self, name: impl Into<String>, healthy: bool) -> Self {
        self.components.insert(
            name.into(),
            ComponentHealth {
                status: if healthy { "healthy" } else { "unhealthy" }.to_string(),
            },
        );
        self
    }
}

/// Component health
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentHealth {
    /// Status
    pub status: String,
}

/// Request context
#[derive(Debug, Clone)]
pub struct RequestContext {
    /// Request ID
    pub request_id: String,
    /// User (if authenticated)
    pub user: Option<User>,
    /// API Key (if used)
    pub api_key: Option<ApiKey>,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

impl RequestContext {
    /// Create new context
    pub fn new() -> Self {
        Self {
            request_id: Uuid::new_v4().to_string(),
            user: None,
            api_key: None,
            metadata: HashMap::new(),
        }
    }

    /// With user
    pub fn with_user(mut self, user: User) -> Self {
        self.user = Some(user);
        self
    }

    /// With API key
    pub fn with_api_key(mut self, key: ApiKey) -> Self {
        self.api_key = Some(key);
        self
    }

    /// Check if authenticated
    pub fn is_authenticated(&self) -> bool {
        self.user.is_some() || self.api_key.is_some()
    }
}

impl Default for RequestContext {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_config() {
        let config = ServerConfig::new(3000)
            .with_host("localhost")
            .with_auth("secret");

        assert_eq!(config.port, 3000);
        assert!(config.auth_enabled);
    }

    #[test]
    fn test_user() {
        let user = User::new("testuser")
            .with_email("test@example.com")
            .with_roles(vec!["admin".to_string()]);

        assert!(user.is_admin());
        assert!(user.has_role("admin"));
    }

    #[test]
    fn test_claims() {
        let user = User::new("test");
        let claims = Claims::new(&user, 3600);

        assert!(!claims.is_expired());
    }

    #[test]
    fn test_api_response() {
        let response = ApiResponse::success("data");
        assert!(response.success);
        assert_eq!(response.data, Some("data"));

        let error = ApiResponse::<()>::error("failed");
        assert!(!error.success);
        assert_eq!(error.error, Some("failed".to_string()));
    }

    #[test]
    fn test_pagination() {
        let pagination = Pagination::new(2, 10, 95);

        assert_eq!(pagination.page, 2);
        assert_eq!(pagination.total_pages, 10);
        assert_eq!(pagination.offset(), 10);
    }

    #[test]
    fn test_health_response() {
        let health = HealthResponse::healthy("1.0.0", 100)
            .with_component("database", true)
            .with_component("cache", true);

        assert_eq!(health.status, "healthy");
        assert_eq!(health.components.len(), 2);
    }

    #[test]
    fn test_request_context() {
        let ctx = RequestContext::new()
            .with_user(User::new("test"));

        assert!(ctx.is_authenticated());
    }
}
