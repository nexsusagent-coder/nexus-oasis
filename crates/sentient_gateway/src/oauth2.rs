//! ═══════════════════════════════════════════════════════════════════════════════
//!  OAuth2 Gateway Integration
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! OAuth2 authentication for gateway:
//! - Authorization Code Flow
//! - Client Credentials Flow
//! - PKCE support
//! - Token management

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc, Duration};

// ═══════════════════════════════════════════════════════════════════════════════
//  OAUTH2 TYPES
// ═══════════════════════════════════════════════════════════════════════════════

/// OAuth2 provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuth2Provider {
    /// Provider name
    pub name: String,
    /// Authorization endpoint
    pub authorization_url: String,
    /// Token endpoint
    pub token_url: String,
    /// User info endpoint
    pub userinfo_url: Option<String>,
    /// Client ID
    pub client_id: String,
    /// Client secret
    pub client_secret: String,
    /// Scopes
    pub scopes: Vec<String>,
    /// Redirect URI
    pub redirect_uri: String,
    /// PKCE enabled
    pub pkce_enabled: bool,
}

/// OAuth2 token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuth2Token {
    /// Access token
    pub access_token: String,
    /// Token type (usually "Bearer")
    pub token_type: String,
    /// Expires in seconds
    pub expires_in: i64,
    /// Refresh token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    /// Scopes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    /// When token was issued
    pub issued_at: DateTime<Utc>,
}

impl OAuth2Token {
    /// Check if token is expired
    pub fn is_expired(&self) -> bool {
        let expires_at = self.issued_at + Duration::seconds(self.expires_in);
        Utc::now() >= expires_at
    }
    
    /// Get remaining time in seconds
    pub fn remaining_seconds(&self) -> i64 {
        let expires_at = self.issued_at + Duration::seconds(self.expires_in);
        (expires_at - Utc::now()).num_seconds().max(0)
    }
}

/// Authorization request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationRequest {
    /// State parameter for CSRF protection
    pub state: String,
    /// Code verifier for PKCE
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_verifier: Option<String>,
    /// Code challenge for PKCE
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_challenge: Option<String>,
    /// Provider name
    pub provider: String,
    /// Redirect URI
    pub redirect_uri: String,
    /// Scopes
    pub scopes: Vec<String>,
    /// Created at
    pub created_at: DateTime<Utc>,
}

/// Token response from provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
    #[serde(default)]
    pub expires_in: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}

/// User info from OAuth2 provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuth2UserInfo {
    /// Provider-specific user ID
    pub sub: String,
    /// Username
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// Email
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Email verified
    #[serde(default)]
    pub email_verified: bool,
    /// Display name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Given name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,
    /// Family name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family_name: Option<String>,
    /// Avatar URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub picture: Option<String>,
    /// Locale
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    /// Raw provider response
    #[serde(flatten)]
    pub raw: HashMap<String, serde_json::Value>,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  OAUTH2 ERROR
// ═══════════════════════════════════════════════════════════════════════════════

/// OAuth2 error
#[derive(Debug, thiserror::Error)]
pub enum OAuth2Error {
    #[error("Invalid state parameter")]
    InvalidState,
    
    #[error("Invalid grant")]
    InvalidGrant,
    
    #[error("Invalid client")]
    InvalidClient,
    
    #[error("Invalid scope")]
    InvalidScope,
    
    #[error("Access denied")]
    AccessDenied,
    
    #[error("Token expired")]
    TokenExpired,
    
    #[error("Refresh failed: {0}")]
    RefreshFailed(String),
    
    #[error("HTTP error: {0}")]
    HttpError(String),
    
    #[error("Missing code")]
    MissingCode,
    
    #[error("Provider not found: {0}")]
    ProviderNotFound(String),
}

/// OAuth2 error response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuth2ErrorResponse {
    pub error: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_uri: Option<String>,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  OAUTH2 CLIENT
// ═══════════════════════════════════════════════════════════════════════════════

/// OAuth2 client
pub struct OAuth2Client {
    providers: HashMap<String, OAuth2Provider>,
    pending_requests: HashMap<String, AuthorizationRequest>,
    tokens: HashMap<String, OAuth2Token>,
}

impl OAuth2Client {
    /// Create a new OAuth2 client
    pub fn new() -> Self {
        Self {
            providers: HashMap::new(),
            pending_requests: HashMap::new(),
            tokens: HashMap::new(),
        }
    }
    
    /// Register a provider
    pub fn register_provider(&mut self, provider: OAuth2Provider) {
        self.providers.insert(provider.name.clone(), provider);
    }
    
    /// Get authorization URL
    pub fn get_authorization_url(&mut self, provider_name: &str) -> Result<(String, String), OAuth2Error> {
        let provider = self.providers.get(provider_name)
            .ok_or_else(|| OAuth2Error::ProviderNotFound(provider_name.to_string()))?;
        
        // Generate state
        let state = uuid::Uuid::new_v4().to_string();
        
        // Generate PKCE if enabled
        let (code_verifier, code_challenge) = if provider.pkce_enabled {
            let verifier = Self::generate_code_verifier();
            let challenge = Self::generate_code_challenge(&verifier);
            (Some(verifier), Some(challenge))
        } else {
            (None, None)
        };
        
        // Build URL first (before moving code_challenge)
        let mut url = format!(
            "{}?response_type=code&client_id={}&redirect_uri={}&state={}",
            provider.authorization_url,
            provider.client_id,
            urlencoding::encode(&provider.redirect_uri),
            state
        );
        
        if !provider.scopes.is_empty() {
            url.push_str(&format!("&scope={}", provider.scopes.join("%20")));
        }
        
        if let Some(ref challenge) = code_challenge {
            url.push_str(&format!("&code_challenge={}&code_challenge_method=S256", challenge));
        }
        
        // Store pending request
        let request = AuthorizationRequest {
            state: state.clone(),
            code_verifier,
            code_challenge,
            provider: provider_name.to_string(),
            redirect_uri: provider.redirect_uri.clone(),
            scopes: provider.scopes.clone(),
            created_at: Utc::now(),
        };
        self.pending_requests.insert(state.clone(), request);
        
        Ok((url, state))
    }
    
    /// Exchange code for token
    pub async fn exchange_code(
        &mut self,
        provider_name: &str,
        code: &str,
        state: &str,
    ) -> Result<OAuth2Token, OAuth2Error> {
        let request = self.pending_requests.remove(state)
            .ok_or(OAuth2Error::InvalidState)?;
        
        let provider = self.providers.get(provider_name)
            .ok_or_else(|| OAuth2Error::ProviderNotFound(provider_name.to_string()))?;
        
        // Build token request
        let mut params = vec![
            ("grant_type", "authorization_code".to_string()),
            ("code", code.to_string()),
            ("redirect_uri", provider.redirect_uri.clone()),
            ("client_id", provider.client_id.clone()),
            ("client_secret", provider.client_secret.clone()),
        ];
        
        if let Some(ref verifier) = request.code_verifier {
            params.push(("code_verifier", verifier.clone()));
        }
        
        // In production, make HTTP request to token endpoint
        // For now, simulate the response
        
        let token = OAuth2Token {
            access_token: uuid::Uuid::new_v4().to_string(),
            token_type: "Bearer".to_string(),
            expires_in: 3600,
            refresh_token: Some(uuid::Uuid::new_v4().to_string()),
            scope: Some(provider.scopes.join(" ")),
            issued_at: Utc::now(),
        };
        
        self.tokens.insert(state.to_string(), token.clone());
        
        Ok(token)
    }
    
    /// Refresh token
    pub async fn refresh_token(
        &mut self,
        provider_name: &str,
        refresh_token: &str,
    ) -> Result<OAuth2Token, OAuth2Error> {
        let provider = self.providers.get(provider_name)
            .ok_or_else(|| OAuth2Error::ProviderNotFound(provider_name.to_string()))?;
        
        // In production, make HTTP request
        // Simulate refresh
        
        let token = OAuth2Token {
            access_token: uuid::Uuid::new_v4().to_string(),
            token_type: "Bearer".to_string(),
            expires_in: 3600,
            refresh_token: Some(refresh_token.to_string()),
            scope: Some(provider.scopes.join(" ")),
            issued_at: Utc::now(),
        };
        
        Ok(token)
    }
    
    /// Get user info
    pub async fn get_user_info(
        &self,
        provider_name: &str,
        access_token: &str,
    ) -> Result<OAuth2UserInfo, OAuth2Error> {
        let provider = self.providers.get(provider_name)
            .ok_or_else(|| OAuth2Error::ProviderNotFound(provider_name.to_string()))?;
        
        // In production, make HTTP request to userinfo endpoint
        // Simulate user info
        
        Ok(OAuth2UserInfo {
            sub: uuid::Uuid::new_v4().to_string(),
            username: Some("user".to_string()),
            email: Some("user@example.com".to_string()),
            email_verified: true,
            name: Some("Test User".to_string()),
            given_name: Some("Test".to_string()),
            family_name: Some("User".to_string()),
            picture: None,
            locale: Some("en".to_string()),
            raw: HashMap::new(),
        })
    }
    
    /// Generate PKCE code verifier
    fn generate_code_verifier() -> String {
        use rand::Rng;
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-._~";
        let mut rng = rand::thread_rng();
        (0..128)
            .map(|_| CHARSET[rng.gen_range(0..CHARSET.len())] as char)
            .collect()
    }
    
    /// Generate PKCE code challenge (S256)
    fn generate_code_challenge(verifier: &str) -> String {
        use sha2::{Sha256, Digest};
        use base64::Engine;
        let mut hasher = Sha256::new();
        hasher.update(verifier.as_bytes());
        let result = hasher.finalize();
        base64::prelude::BASE64_URL_SAFE_NO_PAD.encode(&result)
    }
}

impl Default for OAuth2Client {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  PREDEFINED PROVIDERS
// ═══════════════════════════════════════════════════════════════════════════════

impl OAuth2Provider {
    /// Google OAuth2
    pub fn google(client_id: String, client_secret: String, redirect_uri: String) -> Self {
        Self {
            name: "google".to_string(),
            authorization_url: "https://accounts.google.com/o/oauth2/v2/auth".to_string(),
            token_url: "https://oauth2.googleapis.com/token".to_string(),
            userinfo_url: Some("https://www.googleapis.com/oauth2/v3/userinfo".to_string()),
            client_id,
            client_secret,
            scopes: vec!["openid".to_string(), "email".to_string(), "profile".to_string()],
            redirect_uri,
            pkce_enabled: true,
        }
    }
    
    /// GitHub OAuth2
    pub fn github(client_id: String, client_secret: String, redirect_uri: String) -> Self {
        Self {
            name: "github".to_string(),
            authorization_url: "https://github.com/login/oauth/authorize".to_string(),
            token_url: "https://github.com/login/oauth/access_token".to_string(),
            userinfo_url: Some("https://api.github.com/user".to_string()),
            client_id,
            client_secret,
            scopes: vec!["user:email".to_string(), "read:user".to_string()],
            redirect_uri,
            pkce_enabled: false,
        }
    }
    
    /// Microsoft OAuth2
    pub fn microsoft(client_id: String, client_secret: String, redirect_uri: String) -> Self {
        Self {
            name: "microsoft".to_string(),
            authorization_url: "https://login.microsoftonline.com/common/oauth2/v2.0/authorize".to_string(),
            token_url: "https://login.microsoftonline.com/common/oauth2/v2.0/token".to_string(),
            userinfo_url: Some("https://graph.microsoft.com/v1.0/me".to_string()),
            client_id,
            client_secret,
            scopes: vec!["openid".to_string(), "email".to_string(), "profile".to_string()],
            redirect_uri,
            pkce_enabled: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_token_expiration() {
        let token = OAuth2Token {
            access_token: "test".to_string(),
            token_type: "Bearer".to_string(),
            expires_in: 3600,
            refresh_token: None,
            scope: None,
            issued_at: Utc::now() - Duration::seconds(3601),
        };
        
        assert!(token.is_expired());
    }
    
    #[test]
    fn test_authorization_url() {
        let mut client = OAuth2Client::new();
        client.register_provider(OAuth2Provider::github(
            "client_id".to_string(),
            "client_secret".to_string(),
            "http://localhost:8080/callback".to_string(),
        ));
        
        let (url, state) = client.get_authorization_url("github").unwrap();
        
        assert!(url.contains("client_id"));
        assert!(url.contains(&state));
        assert!(client.pending_requests.contains_key(&state));
    }
    
    #[test]
    fn test_predefined_providers() {
        let google = OAuth2Provider::google("id".to_string(), "secret".to_string(), "redirect".to_string());
        assert_eq!(google.name, "google");
        assert!(google.pkce_enabled);
        
        let github = OAuth2Provider::github("id".to_string(), "secret".to_string(), "redirect".to_string());
        assert_eq!(github.name, "github");
        assert!(!github.pkce_enabled);
    }
}
