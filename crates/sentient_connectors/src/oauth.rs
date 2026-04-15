//! OAuth2 authentication helpers

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{ConnectorError, ConnectorResult};

// ═══════════════════════════════════════════════════════════════════════════════
// OAUTH CONFIG
// ═══════════════════════════════════════════════════════════════════════════════

/// OAuth2 configuration for a provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthConfig {
    /// Provider name (google, github, etc.)
    pub provider: String,
    /// Client ID
    pub client_id: String,
    /// Client Secret
    pub client_secret: String,
    /// Authorization URL
    pub auth_url: String,
    /// Token URL
    pub token_url: String,
    /// Redirect URI
    pub redirect_uri: String,
    /// Scopes to request
    pub scopes: Vec<String>,
    /// Additional parameters
    #[serde(default)]
    pub extra_params: HashMap<String, String>,
}

impl OAuthConfig {
    /// Google OAuth2 config
    pub fn google(client_id: &str, client_secret: &str, redirect_uri: &str) -> Self {
        Self {
            provider: "google".to_string(),
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
            auth_url: "https://accounts.google.com/o/oauth2/v2/auth".to_string(),
            token_url: "https://oauth2.googleapis.com/token".to_string(),
            redirect_uri: redirect_uri.to_string(),
            scopes: vec![
                "https://www.googleapis.com/auth/userinfo.email".to_string(),
                "https://www.googleapis.com/auth/userinfo.profile".to_string(),
            ],
            extra_params: HashMap::new(),
        }
    }

    /// Gmail OAuth2 config (extends Google with Gmail scopes)
    pub fn gmail(client_id: &str, client_secret: &str, redirect_uri: &str) -> Self {
        let mut config = Self::google(client_id, client_secret, redirect_uri);
        config.provider = "gmail".to_string();
        config.scopes.push("https://www.googleapis.com/auth/gmail.readonly".to_string());
        config.scopes.push("https://www.googleapis.com/auth/gmail.modify".to_string());
        config
    }

    /// Google Calendar OAuth2 config
    pub fn calendar(client_id: &str, client_secret: &str, redirect_uri: &str) -> Self {
        let mut config = Self::google(client_id, client_secret, redirect_uri);
        config.provider = "calendar".to_string();
        config.scopes.push("https://www.googleapis.com/auth/calendar.readonly".to_string());
        config.scopes.push("https://www.googleapis.com/auth/calendar.events".to_string());
        config
    }

    /// GitHub OAuth2 config
    pub fn github(client_id: &str, client_secret: &str, redirect_uri: &str) -> Self {
        Self {
            provider: "github".to_string(),
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
            auth_url: "https://github.com/login/oauth/authorize".to_string(),
            token_url: "https://github.com/login/oauth/access_token".to_string(),
            redirect_uri: redirect_uri.to_string(),
            scopes: vec!["user".to_string(), "repo".to_string(), "notifications".to_string()],
            extra_params: HashMap::new(),
        }
    }

    /// Add scope
    pub fn with_scope(mut self, scope: &str) -> Self {
        self.scopes.push(scope.to_string());
        self
    }

    /// Generate authorization URL
    pub fn get_authorization_url(&self, state: &str) -> String {
        let scopes = self.scopes.join(" ");
        format!(
            "{}?client_id={}&redirect_uri={}&scope={}&response_type=code&state={}",
            self.auth_url,
            urlencoding::encode(&self.client_id),
            urlencoding::encode(&self.redirect_uri),
            urlencoding::encode(&scopes),
            state
        )
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// OAUTH TOKEN
// ═══════════════════════════════════════════════════════════════════════════════

/// OAuth2 token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthToken {
    /// Access token
    pub access_token: String,
    /// Token type (usually "Bearer")
    pub token_type: String,
    /// Refresh token
    pub refresh_token: Option<String>,
    /// Expires in seconds
    pub expires_in: u64,
    /// Expiry timestamp
    pub expires_at: DateTime<Utc>,
    /// Scopes granted
    pub scope: Option<String>,
}

impl OAuthToken {
    pub fn new(access_token: &str, expires_in: u64) -> Self {
        Self {
            access_token: access_token.to_string(),
            token_type: "Bearer".to_string(),
            refresh_token: None,
            expires_in,
            expires_at: Utc::now() + chrono::Duration::seconds(expires_in as i64),
            scope: None,
        }
    }

    pub fn with_refresh(mut self, refresh_token: &str) -> Self {
        self.refresh_token = Some(refresh_token.to_string());
        self
    }

    pub fn is_expired(&self) -> bool {
        Utc::now() >= self.expires_at
    }

    pub fn expires_in_secs(&self) -> i64 {
        (self.expires_at - Utc::now()).num_seconds()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// OAUTH MANAGER
// ═══════════════════════════════════════════════════════════════════════════════

/// OAuth2 manager - handles authorization flow
pub struct OAuthManager {
    config: OAuthConfig,
    client: reqwest::Client,
}

impl OAuthManager {
    /// Create new OAuth manager
    pub fn new(config: OAuthConfig) -> Self {
        Self {
            config,
            client: reqwest::Client::new(),
        }
    }

    /// Get the config
    pub fn config(&self) -> &OAuthConfig {
        &self.config
    }

    /// Generate authorization URL with random state
    pub fn get_authorization_url(&self) -> (String, String) {
        let state = generate_state();
        let url = self.config.get_authorization_url(&state);
        (url, state)
    }

    /// Exchange authorization code for token
    pub async fn exchange_code(&self, code: &str) -> ConnectorResult<OAuthToken> {
        let params = [
            ("client_id", self.config.client_id.as_str()),
            ("client_secret", self.config.client_secret.as_str()),
            ("code", code),
            ("redirect_uri", self.config.redirect_uri.as_str()),
            ("grant_type", "authorization_code"),
        ];

        let response = self.client
            .post(&self.config.token_url)
            .form(&params)
            .send()
            .await
            .map_err(|e| ConnectorError::OAuthError(e.to_string()))?;

        if !response.status().is_success() {
            let error = response.text().await.unwrap_or_default();
            return Err(ConnectorError::OAuthError(format!("Token exchange failed: {}", error)));
        }

        let token: TokenResponse = response
            .json()
            .await
            .map_err(|e| ConnectorError::OAuthError(e.to_string()))?;

        Ok(OAuthToken {
            access_token: token.access_token,
            token_type: token.token_type.unwrap_or_else(|| "Bearer".to_string()),
            refresh_token: token.refresh_token,
            expires_in: token.expires_in.unwrap_or(3600),
            expires_at: Utc::now() + chrono::Duration::seconds(token.expires_in.unwrap_or(3600) as i64),
            scope: token.scope,
        })
    }

    /// Refresh access token
    pub async fn refresh_token(&self, refresh_token: &str) -> ConnectorResult<OAuthToken> {
        let params = [
            ("client_id", self.config.client_id.as_str()),
            ("client_secret", self.config.client_secret.as_str()),
            ("refresh_token", refresh_token),
            ("grant_type", "refresh_token"),
        ];

        let response = self.client
            .post(&self.config.token_url)
            .form(&params)
            .send()
            .await
            .map_err(|e| ConnectorError::OAuthError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(ConnectorError::OAuthError("Token refresh failed".to_string()));
        }

        let token: TokenResponse = response
            .json()
            .await
            .map_err(|e| ConnectorError::OAuthError(e.to_string()))?;

        Ok(OAuthToken {
            access_token: token.access_token,
            token_type: token.token_type.unwrap_or_else(|| "Bearer".to_string()),
            refresh_token: token.refresh_token.or_else(|| Some(refresh_token.to_string())),
            expires_in: token.expires_in.unwrap_or(3600),
            expires_at: Utc::now() + chrono::Duration::seconds(token.expires_in.unwrap_or(3600) as i64),
            scope: token.scope,
        })
    }
}

// Token response from OAuth providers
#[derive(Debug, Clone, Deserialize)]
struct TokenResponse {
    access_token: String,
    token_type: Option<String>,
    refresh_token: Option<String>,
    expires_in: Option<u64>,
    scope: Option<String>,
}

/// Generate random state string
fn generate_state() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(42);
    
    let mut state = seed;
    (0..32)
        .map(|_| {
            state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            let idx = (state % CHARSET.len() as u64) as usize;
            CHARSET[idx] as char
        })
        .collect()
}

// URL encoding helper
mod urlencoding {
    pub fn encode(s: &str) -> String {
        url::form_urlencoded::byte_serialize(s.as_bytes()).collect()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// DEVICE FLOW (for CLI apps without redirect)
// ═══════════════════════════════════════════════════════════════════════════════

/// Device authorization response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceAuthorization {
    pub device_code: String,
    pub user_code: String,
    pub verification_uri: String,
    #[serde(rename = "verification_uri_complete")]
    pub verification_uri_complete: Option<String>,
    pub expires_in: u64,
    pub interval: u64,
}

impl OAuthManager {
    /// Start device authorization flow (for CLI apps)
    pub async fn start_device_flow(&self) -> ConnectorResult<DeviceAuthorization> {
        let device_url = match self.config.provider.as_str() {
            "google" | "gmail" | "calendar" => 
                "https://oauth2.googleapis.com/device/code",
            "github" => 
                "https://github.com/login/device/code",
            _ => return Err(ConnectorError::OAuthError(
                "Device flow not supported for this provider".to_string()
            )),
        };

        let params = [
            ("client_id", self.config.client_id.as_str()),
            ("scope", &self.config.scopes.join(" ")),
        ];

        let response = self.client
            .post(device_url)
            .form(&params)
            .send()
            .await
            .map_err(|e| ConnectorError::OAuthError(e.to_string()))?;

        response
            .json()
            .await
            .map_err(|e| ConnectorError::OAuthError(e.to_string()))
    }

    /// Poll for device authorization completion
    pub async fn poll_device_token(
        &self,
        device_code: &str,
        interval: u64,
    ) -> ConnectorResult<OAuthToken> {
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(interval)).await;

            let params = [
                ("client_id", self.config.client_id.as_str()),
                ("client_secret", self.config.client_secret.as_str()),
                ("device_code", device_code),
                ("grant_type", "urn:ietf:params:oauth:grant-type:device_code"),
            ];

            let response = self.client
                .post(&self.config.token_url)
                .form(&params)
                .send()
                .await;

            match response {
                Ok(resp) if resp.status().is_success() => {
                    let token: TokenResponse = resp
                        .json()
                        .await
                        .map_err(|e| ConnectorError::OAuthError(e.to_string()))?;
                    
                    return Ok(OAuthToken {
                        access_token: token.access_token,
                        token_type: token.token_type.unwrap_or_else(|| "Bearer".to_string()),
                        refresh_token: token.refresh_token,
                        expires_in: token.expires_in.unwrap_or(3600),
                        expires_at: Utc::now() + chrono::Duration::seconds(token.expires_in.unwrap_or(3600) as i64),
                        scope: token.scope,
                    });
                }
                Ok(resp) => {
                    let body: serde_json::Value = resp
                        .json()
                        .await
                        .unwrap_or(serde_json::json!({}));
                    
                    if body["error"] != "authorization_pending" {
                        return Err(ConnectorError::OAuthError(
                            body["error_description"].as_str().unwrap_or("Unknown error").to_string()
                        ));
                    }
                    // Continue polling
                }
                Err(e) => {
                    return Err(ConnectorError::OAuthError(e.to_string()));
                }
            }
        }
    }
}
