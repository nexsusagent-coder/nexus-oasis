//! ═══════════════════════════════════════════════════════════════════════════════
//!  SECRETS - High-Level Secret Types
//! ═══════════════════════════════════════════════════════════════════════════════

use crate::{AccessLevel, SecureBytes, VaultResult};
use serde::{Deserialize, Serialize};

// ═══════════════════════════════════════════════════════════════════════════════
//  API KEY
// ═══════════════════════════════════════════════════════════════════════════════

/// API Key secret
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKey {
    pub provider: String,
    pub key: String,
    pub prefix: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl ApiKey {
    pub fn new(provider: impl Into<String>, key: impl Into<String>) -> Self {
        Self {
            provider: provider.into(),
            key: key.into(),
            prefix: None,
            created_at: chrono::Utc::now(),
            expires_at: None,
        }
    }

    pub fn with_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.prefix = Some(prefix.into());
        self
    }

    pub fn with_expiry(mut self, days: u32) -> Self {
        self.expires_at = Some(
            chrono::Utc::now() + chrono::Duration::days(days as i64)
        );
        self
    }

    /// Check if key is expired
    pub fn is_expired(&self) -> bool {
        if let Some(expires) = self.expires_at {
            chrono::Utc::now() > expires
        } else {
            false
        }
    }

    /// Get display-safe key (masked)
    pub fn display_key(&self) -> String {
        if self.key.len() > 8 {
            format!("{}...{}", 
                &self.key[..4], 
                &self.key[self.key.len()-4..]
            )
        } else {
            "****".to_string()
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  DATABASE CREDENTIALS
// ═══════════════════════════════════════════════════════════════════════════════

/// Database credentials
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseCredentials {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub username: String,
    pub password: String,
    pub ssl_mode: SslMode,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub enum SslMode {
    #[default]
    Disable,
    Prefer,
    Require,
    VerifyCa,
    VerifyFull,
}

impl DatabaseCredentials {
    pub fn new(
        host: impl Into<String>,
        port: u16,
        database: impl Into<String>,
        username: impl Into<String>,
        password: impl Into<String>,
    ) -> Self {
        Self {
            host: host.into(),
            port,
            database: database.into(),
            username: username.into(),
            password: password.into(),
            ssl_mode: SslMode::default(),
        }
    }

    pub fn with_ssl(mut self, mode: SslMode) -> Self {
        self.ssl_mode = mode;
        self
    }

    /// Build connection string
    pub fn connection_string(&self) -> String {
        format!(
            "postgresql://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database
        )
    }

    /// Build safe connection string (password masked)
    pub fn safe_connection_string(&self) -> String {
        format!(
            "postgresql://{}:****@{}:{}/{}",
            self.username, self.host, self.port, self.database
        )
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  SSH KEY
// ═══════════════════════════════════════════════════════════════════════════════

/// SSH Key pair
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SshKey {
    pub name: String,
    pub key_type: SshKeyType,
    pub private_key: String,
    pub public_key: String,
    pub passphrase: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SshKeyType {
    Rsa,
    Ed25519,
    Ecdsa,
}

impl SshKey {
    pub fn new(
        name: impl Into<String>,
        key_type: SshKeyType,
        private_key: impl Into<String>,
        public_key: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            key_type,
            private_key: private_key.into(),
            public_key: public_key.into(),
            passphrase: None,
            created_at: chrono::Utc::now(),
        }
    }

    pub fn with_passphrase(mut self, passphrase: impl Into<String>) -> Self {
        self.passphrase = Some(passphrase.into());
        self
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  TLS CERTIFICATE
// ═══════════════════════════════════════════════════════════════════════════════

/// TLS Certificate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TlsCertificate {
    pub domain: String,
    pub certificate: String,
    pub private_key: String,
    pub chain: Option<String>,
    pub issued_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: chrono::DateTime<chrono::Utc>,
}

impl TlsCertificate {
    pub fn is_expired(&self) -> bool {
        chrono::Utc::now() > self.expires_at
    }

    pub fn days_until_expiry(&self) -> i64 {
        (self.expires_at - chrono::Utc::now()).num_days()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  SECRET BUILDER
// ═══════════════════════════════════════════════════════════════════════════════

/// Builder for creating secrets
pub struct SecretBuilder {
    name: String,
    path: String,
    access_level: AccessLevel,
    tags: Vec<String>,
    ttl_days: Option<u32>,
}

impl SecretBuilder {
    pub fn new(name: impl Into<String>) -> Self {
        let name = name.into();
        Self {
            path: format!("/secrets/{}", name),
            name,
            access_level: AccessLevel::default(),
            tags: Vec::new(),
            ttl_days: None,
        }
    }

    pub fn with_path(mut self, path: impl Into<String>) -> Self {
        self.path = path.into();
        self
    }

    pub fn with_access_level(mut self, level: AccessLevel) -> Self {
        self.access_level = level;
        self
    }

    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }

    pub fn with_ttl(mut self, days: u32) -> Self {
        self.ttl_days = Some(days);
        self
    }

    pub fn build(self) -> BuiltSecret {
        BuiltSecret {
            name: self.name,
            path: self.path,
            access_level: self.access_level,
            tags: self.tags,
            ttl_days: self.ttl_days,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BuiltSecret {
    pub name: String,
    pub path: String,
    pub access_level: AccessLevel,
    pub tags: Vec<String>,
    pub ttl_days: Option<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_key() {
        let key = ApiKey::new("openai", "sk-test-1234567890")
            .with_prefix("sk-")
            .with_expiry(30);
        
        assert_eq!(key.provider, "openai");
        assert!(!key.is_expired());
        assert_eq!(key.display_key(), "sk-t...7890");
    }

    #[test]
    fn test_database_credentials() {
        let creds = DatabaseCredentials::new(
            "localhost", 5432, "mydb", "user", "password"
        ).with_ssl(SslMode::Require);
        
        assert_eq!(creds.host, "localhost");
        assert!(creds.safe_connection_string().contains("****"));
    }

    #[test]
    fn test_secret_builder() {
        let secret = SecretBuilder::new("my_secret")
            .with_access_level(AccessLevel::Secret)
            .with_tag("production")
            .with_ttl(90)
            .build();
        
        assert_eq!(secret.name, "my_secret");
        assert_eq!(secret.access_level, AccessLevel::Secret);
    }
}
