//! Authentication service

use crate::types::*;
use crate::{WebError, Result};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

/// JWT configuration
#[derive(Debug, Clone)]
pub struct JwtConfig {
    /// Secret key
    pub secret: String,
    /// Expiration in seconds
    pub expiration: u64,
    /// Issuer
    pub issuer: String,
}

impl Default for JwtConfig {
    fn default() -> Self {
        Self {
            secret: "secret".to_string(),
            expiration: 3600,
            issuer: "sentient".to_string(),
        }
    }
}

impl JwtConfig {
    /// Create new config
    pub fn new(secret: impl Into<String>) -> Self {
        Self {
            secret: secret.into(),
            ..Default::default()
        }
    }

    /// With expiration
    pub fn with_expiration(mut self, secs: u64) -> Self {
        self.expiration = secs;
        self
    }
}

/// Authentication service
pub struct AuthService {
    /// JWT config
    config: JwtConfig,
    /// Encoding key
    encoding_key: EncodingKey,
    /// Decoding key
    decoding_key: DecodingKey,
}

impl AuthService {
    /// Create new auth service
    pub fn new(config: JwtConfig) -> Self {
        let encoding_key = EncodingKey::from_secret(config.secret.as_bytes());
        let decoding_key = DecodingKey::from_secret(config.secret.as_bytes());

        Self {
            config,
            encoding_key,
            decoding_key,
        }
    }

    /// Generate JWT token for user
    pub fn generate_token(&self, user: &User) -> Result<String> {
        let claims = Claims::new(user, self.config.expiration);

        encode(
            &Header::default(),
            &claims,
            &self.encoding_key,
        )
        .map_err(WebError::Jwt)
    }

    /// Validate JWT token
    pub fn validate_token(&self, token: &str) -> Result<Claims> {
        let validation = Validation::default();

        decode::<Claims>(token, &self.decoding_key, &validation)
            .map(|data| data.claims)
            .map_err(|e| {
                match e.kind() {
                    jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                        WebError::TokenExpired
                    }
                    _ => WebError::InvalidToken(e.to_string()),
                }
            })
    }

    /// Refresh token
    pub fn refresh_token(&self, token: &str) -> Result<String> {
        let claims = self.validate_token(token)?;
        let user = User::new(&claims.username)
            .with_roles(claims.roles);

        self.generate_token(&user)
    }

    /// Extract claims from token without validation (for debugging)
    pub fn decode_unchecked(&self, token: &str) -> Result<Claims> {
        let mut validation = Validation::default();
        validation.insecure_disable_signature_validation();

        decode::<Claims>(token, &self.decoding_key, &validation)
            .map(|data| data.claims)
            .map_err(|e| WebError::InvalidToken(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jwt_config() {
        let config = JwtConfig::new("secret")
            .with_expiration(7200);

        assert_eq!(config.expiration, 7200);
    }

    #[test]
    fn test_auth_service() {
        let config = JwtConfig::new("test-secret");
        let auth = AuthService::new(config);

        let user = User::new("testuser");
        let token = auth.generate_token(&user).unwrap();

        assert!(!token.is_empty());
    }

    #[test]
    fn test_validate_token() {
        let config = JwtConfig::new("test-secret");
        let auth = AuthService::new(config);

        let user = User::new("testuser")
            .with_roles(vec!["admin".to_string()]);

        let token = auth.generate_token(&user).unwrap();
        let claims = auth.validate_token(&token).unwrap();

        assert_eq!(claims.username, "testuser");
        assert!(claims.roles.contains(&"admin".to_string()));
    }

    #[test]
    fn test_refresh_token() {
        let config = JwtConfig::new("test-secret");
        let auth = AuthService::new(config);

        let user = User::new("testuser");
        let token = auth.generate_token(&user).unwrap();

        let new_token = auth.refresh_token(&token).unwrap();
        assert!(!new_token.is_empty());
    }

    #[test]
    fn test_invalid_token() {
        let config = JwtConfig::new("test-secret");
        let auth = AuthService::new(config);

        let result = auth.validate_token("invalid-token");
        assert!(result.is_err());
    }
}
