//! Single Sign-On (SSO) Integration module
//!
//! Supports multiple SSO providers:
//! - Okta
//! - Auth0
//! - Azure AD
//! - Google Workspace
//! - OneLogin
//! - Keycloak

use serde::{Deserialize, Serialize};
use async_trait::async_trait;

/// SSO Provider types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SSOProviderType {
    Okta,
    Auth0,
    AzureAD,
    GoogleWorkspace,
    OneLogin,
    Keycloak,
    Custom { name: String },
}

impl std::fmt::Display for SSOProviderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SSOProviderType::Okta => write!(f, "okta"),
            SSOProviderType::Auth0 => write!(f, "auth0"),
            SSOProviderType::AzureAD => write!(f, "azure_ad"),
            SSOProviderType::GoogleWorkspace => write!(f, "google"),
            SSOProviderType::OneLogin => write!(f, "onelogin"),
            SSOProviderType::Keycloak => write!(f, "keycloak"),
            SSOProviderType::Custom { name } => write!(f, "custom:{}", name),
        }
    }
}

/// SSO Protocol types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SSOProtocol {
    /// OAuth 2.0 / OpenID Connect
    OIDC {
        client_id: String,
        client_secret: String,
        authorization_url: String,
        token_url: String,
        userinfo_url: String,
        jwks_url: String,
        scope: Vec<String>,
    },
    /// SAML 2.0
    SAML {
        entity_id: String,
        sso_url: String,
        slo_url: Option<String>,
        certificate: String,
        attribute_mapping: HashMap<String, String>,
    },
}

/// SSO Provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSOProvider {
    /// Provider type
    pub provider_type: SSOProviderType,

    /// Display name
    pub display_name: String,

    /// Protocol configuration
    pub protocol: SSOProtocol,

    /// Auto-provision users
    pub auto_provision: bool,

    /// Default role for new users
    pub default_role: String,

    /// Required domains (empty = all domains)
    pub required_domains: Vec<String>,

    /// Enabled
    pub enabled: bool,
}

impl SSOProvider {
    /// Create Okta provider
    pub fn okta(domain: &str, client_id: &str, client_secret: &str) -> Self {
        Self {
            provider_type: SSOProviderType::Okta,
            display_name: "Okta".to_string(),
            protocol: SSOProtocol::OIDC {
                client_id: client_id.to_string(),
                client_secret: client_secret.to_string(),
                authorization_url: format!("https://{}/oauth2/v1/authorize", domain),
                token_url: format!("https://{}/oauth2/v1/token", domain),
                userinfo_url: format!("https://{}/oauth2/v1/userinfo", domain),
                jwks_url: format!("https://{}/oauth2/v1/keys", domain),
                scope: vec!["openid".to_string(), "profile".to_string(), "email".to_string()],
            },
            auto_provision: true,
            default_role: "viewer".to_string(),
            required_domains: vec![],
            enabled: true,
        }
    }

    /// Create Auth0 provider
    pub fn auth0(domain: &str, client_id: &str, client_secret: &str) -> Self {
        Self {
            provider_type: SSOProviderType::Auth0,
            display_name: "Auth0".to_string(),
            protocol: SSOProtocol::OIDC {
                client_id: client_id.to_string(),
                client_secret: client_secret.to_string(),
                authorization_url: format!("https://{}/authorize", domain),
                token_url: format!("https://{}/oauth/token", domain),
                userinfo_url: format!("https://{}/userinfo", domain),
                jwks_url: format!("https://{}/.well-known/jwks.json", domain),
                scope: vec!["openid".to_string(), "profile".to_string(), "email".to_string()],
            },
            auto_provision: true,
            default_role: "viewer".to_string(),
            required_domains: vec![],
            enabled: true,
        }
    }

    /// Create Azure AD provider
    pub fn azure_ad(tenant_id: &str, client_id: &str, client_secret: &str) -> Self {
        Self {
            provider_type: SSOProviderType::AzureAD,
            display_name: "Microsoft".to_string(),
            protocol: SSOProtocol::OIDC {
                client_id: client_id.to_string(),
                client_secret: client_secret.to_string(),
                authorization_url: format!(
                    "https://login.microsoftonline.com/{}/oauth2/v2.0/authorize",
                    tenant_id
                ),
                token_url: format!(
                    "https://login.microsoftonline.com/{}/oauth2/v2.0/token",
                    tenant_id
                ),
                userinfo_url: "https://graph.microsoft.com/oidc/userinfo".to_string(),
                jwks_url: format!(
                    "https://login.microsoftonline.com/{}/discovery/v2.0/keys",
                    tenant_id
                ),
                scope: vec!["openid".to_string(), "profile".to_string(), "email".to_string()],
            },
            auto_provision: true,
            default_role: "viewer".to_string(),
            required_domains: vec![],
            enabled: true,
        }
    }

    /// Create Google Workspace provider
    pub fn google_workspace(client_id: &str, client_secret: &str) -> Self {
        Self {
            provider_type: SSOProviderType::GoogleWorkspace,
            display_name: "Google".to_string(),
            protocol: SSOProtocol::OIDC {
                client_id: client_id.to_string(),
                client_secret: client_secret.to_string(),
                authorization_url: "https://accounts.google.com/o/oauth2/v2/auth".to_string(),
                token_url: "https://oauth2.googleapis.com/token".to_string(),
                userinfo_url: "https://openidconnect.googleapis.com/v1/userinfo".to_string(),
                jwks_url: "https://www.googleapis.com/oauth2/v3/certs".to_string(),
                scope: vec!["openid".to_string(), "profile".to_string(), "email".to_string()],
            },
            auto_provision: true,
            default_role: "viewer".to_string(),
            required_domains: vec![],
            enabled: true,
        }
    }

    /// Create Keycloak provider
    pub fn keycloak(
        server_url: &str,
        realm: &str,
        client_id: &str,
        client_secret: &str,
    ) -> Self {
        Self {
            provider_type: SSOProviderType::Keycloak,
            display_name: "Keycloak".to_string(),
            protocol: SSOProtocol::OIDC {
                client_id: client_id.to_string(),
                client_secret: client_secret.to_string(),
                authorization_url: format!(
                    "{}/realms/{}/protocol/openid-connect/auth",
                    server_url, realm
                ),
                token_url: format!(
                    "{}/realms/{}/protocol/openid-connect/token",
                    server_url, realm
                ),
                userinfo_url: format!(
                    "{}/realms/{}/protocol/openid-connect/userinfo",
                    server_url, realm
                ),
                jwks_url: format!(
                    "{}/realms/{}/protocol/openid-connect/certs",
                    server_url, realm
                ),
                scope: vec!["openid".to_string(), "profile".to_string(), "email".to_string()],
            },
            auto_provision: true,
            default_role: "viewer".to_string(),
            required_domains: vec![],
            enabled: true,
        }
    }
}

/// User information from SSO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSOUser {
    /// Unique user ID from provider
    pub external_id: String,

    /// Email address
    pub email: String,

    /// Display name
    pub name: Option<String>,

    /// Given name
    pub given_name: Option<String>,

    /// Family name
    pub family_name: Option<String>,

    /// Profile picture URL
    pub picture: Option<String>,

    /// Custom attributes
    pub attributes: HashMap<String, serde_json::Value>,
}

/// SSO Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSOConfig {
    /// Enable SSO
    pub enabled: bool,

    /// Configured providers
    pub providers: Vec<SSOProvider>,

    /// Session duration in seconds
    pub session_duration: u64,

    /// Allow password login alongside SSO
    pub allow_password_login: bool,

    /// Redirect URL after login
    pub redirect_url: String,
}

impl Default for SSOConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            providers: vec![],
            session_duration: 86400, // 24 hours
            allow_password_login: true,
            redirect_url: "/".to_string(),
        }
    }
}

/// SSO Manager
pub struct SSOManager {
    config: SSOConfig,
}

impl SSOManager {
    /// Create a new SSO manager
    pub async fn new(config: SSOConfig) -> Result<Self, SSOError> {
        Ok(Self { config })
    }

    /// Get authorization URL for provider
    pub fn get_authorization_url(
        &self,
        provider_type: &SSOProviderType,
        redirect_uri: &str,
        state: &str,
    ) -> Result<String, SSOError> {
        let provider = self.find_provider(provider_type)?;

        match &provider.protocol {
            SSOProtocol::OIDC {
                client_id,
                authorization_url,
                scope,
                ..
            } => {
                let scope_str = scope.join(" ");
                Ok(format!(
                    "{}?client_id={}&redirect_uri={}&response_type=code&scope={}&state={}",
                    authorization_url,
                    urlencoding::encode(client_id),
                    urlencoding::encode(redirect_uri),
                    urlencoding::encode(&scope_str),
                    urlencoding::encode(state),
                ))
            }
            SSOProtocol::SAML { .. } => {
                // SAML uses different flow
                Err(SSOError::UnsupportedProtocol("SAML".to_string()))
            }
        }
    }

    /// Handle OAuth callback
    pub async fn handle_callback(
        &self,
        provider_type: &SSOProviderType,
        code: &str,
        redirect_uri: &str,
    ) -> Result<(SSOUser, String), SSOError> {
        let provider = self.find_provider(provider_type)?;

        match &provider.protocol {
            SSOProtocol::OIDC {
                client_id,
                client_secret,
                token_url,
                userinfo_url,
                ..
            } => {
                // Exchange code for token
                let token_response = self.exchange_code(
                    token_url,
                    client_id,
                    client_secret,
                    code,
                    redirect_uri,
                ).await?;

                // Get user info
                let user = self.get_userinfo(userinfo_url, &token_response.access_token).await?;

                Ok((user, token_response.access_token))
            }
            SSOProtocol::SAML { .. } => {
                Err(SSOError::UnsupportedProtocol("SAML".to_string()))
            }
        }
    }

    /// Get enabled providers
    pub fn get_enabled_providers(&self) -> Vec<&SSOProvider> {
        self.config.providers.iter().filter(|p| p.enabled).collect()
    }

    fn find_provider(&self, provider_type: &SSOProviderType) -> Result<&SSOProvider, SSOError> {
        self.config.providers
            .iter()
            .find(|p| std::mem::discriminant(&p.provider_type) == std::mem::discriminant(provider_type))
            .ok_or_else(|| SSOError::ProviderNotFound(provider_type.to_string()))
    }

    async fn exchange_code(
        &self,
        token_url: &str,
        client_id: &str,
        client_secret: &str,
        code: &str,
        redirect_uri: &str,
    ) -> Result<TokenResponse, SSOError> {
        let client = reqwest::Client::new();

        let params = [
            ("grant_type", "authorization_code".to_string()),
            ("code", code.to_string()),
            ("redirect_uri", redirect_uri.to_string()),
            ("client_id", client_id.to_string()),
            ("client_secret", client_secret.to_string()),
        ];

        let response = client
            .post(token_url)
            .form(&params)
            .send()
            .await
            .map_err(|e| SSOError::RequestError(e.to_string()))?;

        response
            .json()
            .await
            .map_err(|e| SSOError::ParseError(e.to_string()))
    }

    async fn get_userinfo(
        &self,
        userinfo_url: &str,
        access_token: &str,
    ) -> Result<SSOUser, SSOError> {
        let client = reqwest::Client::new();

        let response = client
            .get(userinfo_url)
            .bearer_auth(access_token)
            .send()
            .await
            .map_err(|e| SSOError::RequestError(e.to_string()))?;

        let info: serde_json::Value = response
            .json()
            .await
            .map_err(|e| SSOError::ParseError(e.to_string()))?;

        Ok(SSOUser {
            external_id: info["sub"].as_str().unwrap_or("").to_string(),
            email: info["email"].as_str().unwrap_or("").to_string(),
            name: info["name"].as_str().map(|s| s.to_string()),
            given_name: info["given_name"].as_str().map(|s| s.to_string()),
            family_name: info["family_name"].as_str().map(|s| s.to_string()),
            picture: info["picture"].as_str().map(|s| s.to_string()),
            attributes: HashMap::new(),
        })
    }
}

/// Token response from OAuth
#[derive(Debug, Deserialize)]
struct TokenResponse {
    access_token: String,
    token_type: String,
    expires_in: Option<u64>,
    refresh_token: Option<String>,
}

/// SSO Error types
#[derive(Debug, thiserror::Error)]
pub enum SSOError {
    #[error("Provider not found: {0}")]
    ProviderNotFound(String),

    #[error("Unsupported protocol: {0}")]
    UnsupportedProtocol(String),

    #[error("Request error: {0}")]
    RequestError(String),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Invalid state")]
    InvalidState,

    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),
}

use std::collections::HashMap;
use urlencoding;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_okta_provider() {
        let provider = SSOProvider::okta(
            "example.okta.com",
            "client123",
            "secret123",
        );

        assert_eq!(provider.provider_type, SSOProviderType::Okta);
        assert!(provider.enabled);
        assert!(provider.auto_provision);
    }

    #[test]
    fn test_azure_ad_provider() {
        let provider = SSOProvider::azure_ad(
            "tenant-id",
            "client123",
            "secret123",
        );

        assert_eq!(provider.provider_type, SSOProviderType::AzureAD);
    }

    #[tokio::test]
    async fn test_sso_manager() {
        let config = SSOConfig::default();
        let manager = SSOManager::new(config).await.unwrap();

        let providers = manager.get_enabled_providers();
        assert!(providers.is_empty());
    }
}
