//! ─── AUTH MODULE ───
//!
//! Kimlik doğrulama ve yetkilendirme:
//! - JWT token doğrulama
//! - API key yönetimi
//! - Rate limiting

use sentient_common::error::{SENTIENTError, SENTIENTResult};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use chrono::{Duration, Utc};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// ─── JWT CLAIMS ───

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    /// Subject (user ID)
    pub sub: String,
    
    /// Issuer
    pub iss: String,
    
    /// Audience
    pub aud: String,
    
    /// Issued at (timestamp)
    pub iat: i64,
    
    /// Expiration (timestamp)
    pub exp: i64,
    
    /// Role
    pub role: UserRole,
    
    /// Custom claims
    #[serde(default)]
    pub custom: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    Admin,
    User,
    Guest,
}

impl Default for UserRole {
    fn default() -> Self {
        Self::Guest
    }
}

/// ─── AUTH CONFIG ───

#[derive(Debug, Clone)]
pub struct AuthConfig {
    /// JWT secret key
    pub jwt_secret: String,
    
    /// Token süresi (saat)
    pub token_duration_hours: i64,
    
    /// Issuer
    pub issuer: String,
    
    /// Audience
    pub audience: String,
    
    /// Geçerli API key'ler
    pub valid_api_keys: Vec<String>,
    
    /// Rate limit (istek/dakika)
    pub rate_limit: u32,
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            jwt_secret: "sentient-secret-key-change-in-production".into(),
            token_duration_hours: 24,
            issuer: "sentient-gateway".into(),
            audience: "sentient-users".into(),
            valid_api_keys: vec![],
            rate_limit: 60,
        }
    }
}

/// ─── AUTH MANAGER ───

pub struct AuthManager {
    config: AuthConfig,
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    rate_limiter: Arc<RateLimiter>,
}

impl AuthManager {
    pub fn new(config: AuthConfig) -> Self {
        let encoding_key = EncodingKey::from_secret(config.jwt_secret.as_bytes());
        let decoding_key = DecodingKey::from_secret(config.jwt_secret.as_bytes());
        
        Self {
            rate_limiter: Arc::new(RateLimiter::new(config.rate_limit)),
            encoding_key,
            decoding_key,
            config,
        }
    }
    
    /// JWT token oluştur
    pub fn create_token(&self, user_id: &str, role: UserRole) -> SENTIENTResult<String> {
        let now = Utc::now();
        let exp = now + Duration::hours(self.config.token_duration_hours);
        
        let claims = Claims {
            sub: user_id.into(),
            iss: self.config.issuer.clone(),
            aud: self.config.audience.clone(),
            iat: now.timestamp(),
            exp: exp.timestamp(),
            role,
            custom: HashMap::new(),
        };
        
        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|e| SENTIENTError::General(format!("Token oluşturulamadı: {}", e)))
    }
    
    /// JWT token doğrula
    pub fn verify_token(&self, token: &str) -> SENTIENTResult<Claims> {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.set_issuer(&[&self.config.issuer]);
        validation.set_audience(&[&self.config.audience]);
        
        decode::<Claims>(token, &self.decoding_key, &validation)
            .map(|data| data.claims)
            .map_err(|e| match e.kind() {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                    SENTIENTError::AuthError("Token süresi dolmuş".into())
                }
                jsonwebtoken::errors::ErrorKind::InvalidToken => {
                    SENTIENTError::AuthError("Geçersiz token".into())
                }
                _ => SENTIENTError::AuthError(format!("Token doğrulama hatası: {}", e)),
            })
    }
    
    /// API key doğrula
    pub fn verify_api_key(&self, api_key: &str) -> SENTIENTResult<bool> {
        if self.config.valid_api_keys.is_empty() {
            // API key kontrolü kapalı
            return Ok(true);
        }
        
        // API key hash'le ve karşılaştır
        let hash = self.hash_api_key(api_key);
        
        self.config.valid_api_keys.iter().any(|k| {
            bcrypt::verify(api_key, k).unwrap_or(false) || k == &hash || k == api_key
        }).then_some(true)
        .ok_or_else(|| SENTIENTError::AuthError("Geçersiz API key".into()))
    }
    
    /// API key hash'le
    fn hash_api_key(&self, key: &str) -> String {
        bcrypt::hash(key, bcrypt::DEFAULT_COST).unwrap_or_default()
    }
    
    /// Rate limit kontrolü
    pub async fn check_rate_limit(&self, ip: &str) -> SENTIENTResult<()> {
        self.rate_limiter.check(ip).await
    }
    
    /// Role yetki kontrolü
    pub fn check_permission(&self, claims: &Claims, required_role: UserRole) -> SENTIENTResult<()> {
        let role_level = |r: &UserRole| match r {
            UserRole::Admin => 3,
            UserRole::User => 2,
            UserRole::Guest => 1,
        };
        
        if role_level(&claims.role) >= role_level(&required_role) {
            Ok(())
        } else {
            Err(SENTIENTError::AuthError("Yetkisiz erişim".into()))
        }
    }
}

/// ─── RATE LIMITER ───

pub struct RateLimiter {
    requests: Arc<RwLock<HashMap<String, Vec<i64>>>>,
    max_requests: u32,
    window_secs: i64,
}

impl RateLimiter {
    pub fn new(max_requests: u32) -> Self {
        Self {
            requests: Arc::new(RwLock::new(HashMap::new())),
            max_requests,
            window_secs: 60,
        }
    }
    
    pub async fn check(&self, key: &str) -> SENTIENTResult<()> {
        let now = Utc::now().timestamp();
        let window_start = now - self.window_secs;
        
        let mut requests = self.requests.write().await;
        
        // Eski kayıtları temizle
        if let Some(timestamps) = requests.get_mut(key) {
            timestamps.retain(|&t| t > window_start);
            
            // Limit kontrolü
            if timestamps.len() >= self.max_requests as usize {
                return Err(SENTIENTError::RateLimitError(
                    format!("Rate limit aşıldı: {}/dk", self.max_requests)
                ));
            }
            
            // Yeni istek ekle
            timestamps.push(now);
        } else {
            requests.insert(key.into(), vec![now]);
        }
        
        Ok(())
    }
    
    /// Periyodik temizlik
    pub async fn cleanup(&self) {
        let now = Utc::now().timestamp();
        let window_start = now - self.window_secs;
        
        let mut requests = self.requests.write().await;
        requests.retain(|_, timestamps| {
            timestamps.retain(|&t| t > window_start);
            !timestamps.is_empty()
        });
    }
}

/// ─── AUTH HEADER PARSER ───

pub struct AuthHeader;

impl AuthHeader {
    /// Bearer token'ı ayıkla
    pub fn extract_bearer(auth_header: &str) -> Option<&str> {
        auth_header
            .strip_prefix("Bearer ")
            .or_else(|| auth_header.strip_prefix("bearer "))
            .map(|s| s.trim())
    }
    
    /// API key'i ayıkla
    pub fn extract_api_key(auth_header: &str) -> Option<&str> {
        auth_header
            .strip_prefix("ApiKey ")
            .or_else(|| auth_header.strip_prefix("apikey "))
            .map(|s| s.trim())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_auth_config_default() {
        let config = AuthConfig::default();
        assert_eq!(config.token_duration_hours, 24);
        assert_eq!(config.rate_limit, 60);
    }
    
    #[test]
    fn test_user_role_default() {
        let role = UserRole::default();
        assert_eq!(role, UserRole::Guest);
    }
    
    #[test]
    fn test_auth_header_bearer_extraction() {
        let header = "Bearer my_token_123";
        let token = AuthHeader::extract_bearer(header);
        assert_eq!(token, Some("my_token_123"));
        
        let header = "Basic something";
        let token = AuthHeader::extract_bearer(header);
        assert!(token.is_none());
    }
    
    #[test]
    fn test_auth_header_api_key_extraction() {
        let header = "ApiKey my_api_key";
        let key = AuthHeader::extract_api_key(header);
        assert_eq!(key, Some("my_api_key"));
    }
    
    #[test]
    fn test_create_and_verify_token() {
        let config = AuthConfig::default();
        let manager = AuthManager::new(config);
        
        // Token oluştur
        let token = manager.create_token("user123", UserRole::User).unwrap();
        assert!(!token.is_empty());
        
        // Doğrula
        let claims = manager.verify_token(&token).unwrap();
        assert_eq!(claims.sub, "user123");
        assert_eq!(claims.role, UserRole::User);
    }
    
    #[test]
    fn test_verify_invalid_token() {
        let config = AuthConfig::default();
        let manager = AuthManager::new(config);
        
        let result = manager.verify_token("invalid_token");
        assert!(result.is_err());
    }
    
    #[test]
    fn test_role_permission_check() {
        let config = AuthConfig::default();
        let manager = AuthManager::new(config);
        
        // Admin her şeyi yapabilir
        let admin_claims = Claims {
            sub: "admin".into(),
            iss: "sentient".into(),
            aud: "sentient".into(),
            iat: 0,
            exp: 9999999999,
            role: UserRole::Admin,
            custom: HashMap::new(),
        };
        
        assert!(manager.check_permission(&admin_claims, UserRole::Admin).is_ok());
        assert!(manager.check_permission(&admin_claims, UserRole::User).is_ok());
        
        // Guest admin yapamaz
        let guest_claims = Claims {
            sub: "guest".into(),
            iss: "sentient".into(),
            aud: "sentient".into(),
            iat: 0,
            exp: 9999999999,
            role: UserRole::Guest,
            custom: HashMap::new(),
        };
        
        assert!(manager.check_permission(&guest_claims, UserRole::Admin).is_err());
    }
    
    #[tokio::test]
    async fn test_rate_limiter() {
        let limiter = RateLimiter::new(3);
        
        // İlk 3 istek başarılı
        assert!(limiter.check("127.0.0.1").await.is_ok());
        assert!(limiter.check("127.0.0.1").await.is_ok());
        assert!(limiter.check("127.0.0.1").await.is_ok());
        
        // 4. istek hata vermeli
        assert!(limiter.check("127.0.0.1").await.is_err());
        
        // Farklı IP başarılı
        assert!(limiter.check("127.0.0.2").await.is_ok());
    }
}
