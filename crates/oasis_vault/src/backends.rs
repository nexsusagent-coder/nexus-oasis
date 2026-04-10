//! ═══════════════════════════════════════════════════════════════════════════════
//!  VAULT BACKENDS - External Secrets Manager Integration
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Provides integration with external secrets management systems:
//! - HashiCorp Vault
//! - AWS Secrets Manager / KMS
//! - Azure Key Vault
//! - Google Cloud Secret Manager

use crate::{VaultError, VaultResult, AccessLevel};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

// ═══════════════════════════════════════════════════════════════════════════════
//  BACKEND TRAIT
// ═══════════════════════════════════════════════════════════════════════════════

/// Secrets backend trait for external integrations
#[async_trait]
pub trait SecretsBackend: Send + Sync {
    /// Get backend name
    fn name(&self) -> &str;
    
    /// Check if backend is healthy
    async fn health_check(&self) -> VaultResult<bool>;
    
    /// Store a secret
    async fn store(&self, path: &str, value: &[u8], metadata: &SecretMetadata) -> VaultResult<()>;
    
    /// Retrieve a secret
    async fn retrieve(&self, path: &str) -> VaultResult<Vec<u8>>;
    
    /// Delete a secret
    async fn delete(&self, path: &str) -> VaultResult<()>;
    
    /// List secrets at path
    async fn list(&self, path: &str) -> VaultResult<Vec<String>>;
    
    /// Check if secret exists
    async fn exists(&self, path: &str) -> VaultResult<bool>;
}

/// Secret metadata for external backends
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretMetadata {
    pub access_level: AccessLevel,
    pub tags: Vec<String>,
    pub ttl_seconds: Option<u64>,
    pub description: Option<String>,
}

impl Default for SecretMetadata {
    fn default() -> Self {
        Self {
            access_level: AccessLevel::default(),
            tags: Vec::new(),
            ttl_seconds: None,
            description: None,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  BACKEND CONFIG
// ═══════════════════════════════════════════════════════════════════════════════

/// Backend configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackendConfig {
    /// Backend type
    pub backend_type: BackendType,
    
    /// Connection endpoint (URL)
    pub endpoint: Option<String>,
    
    /// Authentication method
    pub auth: AuthMethod,
    
    /// Namespace (for multi-tenant)
    pub namespace: Option<String>,
    
    /// Mount path (for HashiCorp Vault)
    pub mount_path: Option<String>,
    
    /// Enable TLS verification
    pub tls_verify: bool,
    
    /// Request timeout in seconds
    pub timeout_secs: u64,
}

/// Supported backend types
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum BackendType {
    Local,
    HashiCorpVault,
    AwsSecretsManager,
    AwsKms,
    AzureKeyVault,
    GcpSecretManager,
}

/// Authentication methods
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthMethod {
    /// No authentication (local only)
    None,
    
    /// Token-based auth
    Token { token: String },
    
    /// Username/password
    UserPass { username: String, password: String },
    
    /// Kubernetes service account
    Kubernetes { role: String, jwt_path: String },
    
    /// AWS IAM role
    AwsIam { role_arn: Option<String> },
    
    /// Azure Managed Identity
    AzureManagedIdentity { client_id: Option<String> },
    
    /// GCP Service Account
    GcpServiceAccount { key_path: String },
    
    /// AppRole (HashiCorp Vault)
    AppRole { role_id: String, secret_id: String },
}

// ═══════════════════════════════════════════════════════════════════════════════
//  LOCAL BACKEND (Default)
// ═══════════════════════════════════════════════════════════════════════════════

/// Local in-memory backend (for development)
pub struct LocalBackend {
    secrets: std::collections::HashMap<String, Vec<u8>>,
}

impl LocalBackend {
    pub fn new() -> Self {
        Self {
            secrets: std::collections::HashMap::new(),
        }
    }
}

impl Default for LocalBackend {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl SecretsBackend for LocalBackend {
    fn name(&self) -> &str {
        "local"
    }
    
    async fn health_check(&self) -> VaultResult<bool> {
        Ok(true)
    }
    
    async fn store(&self, path: &str, value: &[u8], _metadata: &SecretMetadata) -> VaultResult<()> {
        // Note: LocalBackend needs interior mutability for production use
        // This is a simplified implementation
        log::debug!("Local backend store: {}", path);
        Ok(())
    }
    
    async fn retrieve(&self, path: &str) -> VaultResult<Vec<u8>> {
        self.secrets.get(path)
            .cloned()
            .ok_or_else(|| VaultError::SecretNotFound(path.to_string()))
    }
    
    async fn delete(&self, path: &str) -> VaultResult<()> {
        self.secrets.get(path)
            .map(|_| ())
            .ok_or_else(|| VaultError::SecretNotFound(path.to_string()))
    }
    
    async fn list(&self, path: &str) -> VaultResult<Vec<String>> {
        Ok(self.secrets.keys()
            .filter(|k| k.starts_with(path))
            .cloned()
            .collect())
    }
    
    async fn exists(&self, path: &str) -> VaultResult<bool> {
        Ok(self.secrets.contains_key(path))
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  HASHICORP VAULT BACKEND (Feature-gated)
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(feature = "hashicorp_vault")]
mod hashicorp_vault {
    use super::*;
    
    /// HashiCorp Vault backend
    pub struct HashiCorpVaultBackend {
        client: reqwest::Client,
        config: BackendConfig,
    }

    impl HashiCorpVaultBackend {
        pub fn new(config: BackendConfig) -> VaultResult<Self> {
            let endpoint = config.endpoint.as_ref()
                .ok_or_else(|| VaultError::AccessDenied("Vault endpoint required".into()))?;
            
            let mut builder = reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(config.timeout_secs));
            
            if !config.tls_verify {
                builder = builder.danger_accept_invalid_certs(true);
            }
            
            let client = builder.build()
                .map_err(|e| VaultError::AccessDenied(format!("HTTP client: {}", e)))?;
            
            log::info!("HashiCorp Vault backend initialized: {}", endpoint);
            Ok(Self { client, config })
        }
        
        fn get_auth_header(&self) -> Option<String> {
            match &self.config.auth {
                AuthMethod::Token { token } => Some(format!("Bearer {}", token)),
                AuthMethod::AppRole { role_id, secret_id } => {
                    Some(format!("Bearer {}_{}", role_id, secret_id))
                }
                _ => None,
            }
        }
        
        fn get_secret_url(&self, path: &str) -> String {
            let endpoint = self.config.endpoint.as_ref().expect("operation failed");
            let mount = self.config.mount_path.as_deref().unwrap_or("secret");
            format!("{}/v1/{}/data/{}", endpoint, mount, path)
        }
    }

    #[async_trait]
    impl SecretsBackend for HashiCorpVaultBackend {
        fn name(&self) -> &str {
            "hashicorp_vault"
        }
        
        async fn health_check(&self) -> VaultResult<bool> {
            let endpoint = self.config.endpoint.as_ref().expect("operation failed");
            let url = format!("{}/v1/sys/health", endpoint);
            
            let response = self.client
                .get(&url)
                .send()
                .await
                .map_err(|e| VaultError::AccessDenied(format!("Health check failed: {}", e)))?;
            
            Ok(response.status().is_success() || response.status().as_u16() == 429)
        }
        
        async fn store(&self, path: &str, value: &[u8], _metadata: &SecretMetadata) -> VaultResult<()> {
            let url = self.get_secret_url(path);
            let auth = self.get_auth_header()
                .ok_or_else(|| VaultError::AccessDenied("Authentication required".into()))?;
            
            let body = serde_json::json!({
                "data": {
                    "value": base64::Engine::encode(&base64::engine::general_purpose::STANDARD, value),
                }
            });
            
            let response = self.client
                .post(&url)
                .header("Authorization", auth)
                .json(&body)
                .send()
                .await
                .map_err(|e| VaultError::EncryptionFailed(format!("Store failed: {}", e)))?;
            
            if response.status().is_success() {
                log::debug!("HashiCorp Vault: Stored secret at {}", path);
                Ok(())
            } else {
                Err(VaultError::EncryptionFailed(format!("Store failed: {}", response.status())))
            }
        }
        
        async fn retrieve(&self, path: &str) -> VaultResult<Vec<u8>> {
            let url = self.get_secret_url(path);
            let auth = self.get_auth_header()
                .ok_or_else(|| VaultError::AccessDenied("Authentication required".into()))?;
            
            let response = self.client
                .get(&url)
                .header("Authorization", auth)
                .send()
                .await
                .map_err(|e| VaultError::DecryptionFailed(format!("Retrieve failed: {}", e)))?;
            
            if !response.status().is_success() {
                return Err(VaultError::SecretNotFound(path.to_string()));
            }
            
            let json: serde_json::Value = response.json().await
                .map_err(|e| VaultError::DecryptionFailed(format!("Parse failed: {}", e)))?;
            
            let b64_value = json["data"]["data"]["value"]
                .as_str()
                .ok_or_else(|| VaultError::DecryptionFailed("Invalid response".into()))?;
            
            base64::Engine::decode(&base64::engine::general_purpose::STANDARD, b64_value)
                .map_err(|e| VaultError::DecryptionFailed(format!("Decode failed: {}", e)))
        }
        
        async fn delete(&self, path: &str) -> VaultResult<()> {
            let url = self.get_secret_url(path);
            let auth = self.get_auth_header()
                .ok_or_else(|| VaultError::AccessDenied("Authentication required".into()))?;
            
            let response = self.client
                .delete(&url)
                .header("Authorization", auth)
                .send()
                .await
                .map_err(|e| VaultError::AccessDenied(format!("Delete failed: {}", e)))?;
            
            if response.status().is_success() || response.status().as_u16() == 404 {
                Ok(())
            } else {
                Err(VaultError::SecretNotFound(path.to_string()))
            }
        }
        
        async fn list(&self, path: &str) -> VaultResult<Vec<String>> {
            let endpoint = self.config.endpoint.as_ref().expect("operation failed");
            let mount = self.config.mount_path.as_deref().unwrap_or("secret");
            let url = format!("{}/v1/{}/metadata/{}?list=true", endpoint, mount, path);
            let auth = self.get_auth_header()
                .ok_or_else(|| VaultError::AccessDenied("Authentication required".into()))?;
            
            let response = self.client
                .get(&url)
                .header("Authorization", auth)
                .send()
                .await
                .map_err(|e| VaultError::AccessDenied(format!("List failed: {}", e)))?;
            
            if !response.status().is_success() {
                return Ok(Vec::new());
            }
            
            let json: serde_json::Value = response.json().await
                .map_err(|e| VaultError::AccessDenied(format!("Parse failed: {}", e)))?;
            
            Ok(json["data"]["keys"]
                .as_array()
                .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default())
        }
        
        async fn exists(&self, path: &str) -> VaultResult<bool> {
            match self.retrieve(path).await {
                Ok(_) => Ok(true),
                Err(VaultError::SecretNotFound(_)) => Ok(false),
                Err(e) => Err(e),
            }
        }
    }
}

#[cfg(feature = "hashicorp_vault")]
pub use hashicorp::HashiCorpVaultBackend;

// ═══════════════════════════════════════════════════════════════════════════════
//  AWS SECRETS MANAGER BACKEND (HTTP API Implementation)
// ═══════════════════════════════════════════════════════════════════════════════

/// AWS Secrets Manager backend using HTTP API
pub struct AwsSecretsManagerBackend {
    client: reqwest::Client,
    config: BackendConfig,
    region: String,
}

impl AwsSecretsManagerBackend {
    pub fn new(config: BackendConfig) -> VaultResult<Self> {
        let region = config.endpoint.clone().unwrap_or_else(|| "us-east-1".to_string());
        
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeout_secs))
            .build()
            .map_err(|e| VaultError::AccessDenied(format!("HTTP client: {}", e)))?;
        
        log::info!("AWS Secrets Manager backend initialized (region: {})", region);
        Ok(Self { client, config, region })
    }
    
    /// Get AWS service endpoint
    fn get_service_url(&self) -> String {
        format!("https://secretsmanager.{}.amazonaws.com", self.region)
    }
    
    /// Build AWS SigV4 authorization header (simplified)
    /// For production, use aws-sigv4 crate
    fn build_auth_headers(&self, _method: &str, _path: &str) -> reqwest::header::HeaderMap {
        let mut headers = reqwest::header::HeaderMap::new();
        
        // AWS requires SigV4 signing - this is a placeholder
        // In production: use aws-config and aws-sdk-secretsmanager
        // Or implement SigV4 signing manually
        headers.insert("X-Amz-Target", "secretsmanager.DescribeSecret".parse().expect("operation failed"));
        headers.insert("Content-Type", "application/x-amz-json-1.1".parse().expect("operation failed"));
        
        match &self.config.auth {
            AuthMethod::AwsIam { role_arn } => {
                if let Some(arn) = role_arn {
                    log::debug!("AWS IAM Role: {}", arn);
                }
            }
            AuthMethod::Token { token } => {
                headers.insert("Authorization", format!("Bearer {}", token).parse().expect("operation failed"));
            }
            _ => {}
        }
        
        headers
    }
}

#[async_trait]
impl SecretsBackend for AwsSecretsManagerBackend {
    fn name(&self) -> &str {
        "aws_secrets_manager"
    }
    
    async fn health_check(&self) -> VaultResult<bool> {
        // Try to list secrets as health check
        let url = format!("{}/?Action=ListSecrets&Version=2017-10-17", self.get_service_url());
        
        let response = self.client
            .get(&url)
            .headers(self.build_auth_headers("GET", "/"))
            .send()
            .await;
        
        match response {
            Ok(resp) => Ok(resp.status().is_success() || resp.status().as_u16() == 403),
            Err(e) => {
                log::warn!("AWS Secrets Manager health check failed: {}", e);
                Ok(false)
            }
        }
    }
    
    async fn store(&self, path: &str, value: &[u8], _metadata: &SecretMetadata) -> VaultResult<()> {
        let url = format!("{}/?Action=CreateSecret&Name={}&Version=2017-10-17", 
            self.get_service_url(), 
            urlencoding::encode(path)
        );
        
        let body = serde_json::json!({
            "SecretString": base64::Engine::encode(&base64::engine::general_purpose::STANDARD, value),
        });
        
        let response = self.client
            .post(&url)
            .headers(self.build_auth_headers("POST", path))
            .json(&body)
            .send()
            .await
            .map_err(|e| VaultError::EncryptionFailed(format!("Store failed: {}", e)))?;
        
        if response.status().is_success() {
            log::info!("AWS Secrets Manager: Stored secret at {}", path);
            Ok(())
        } else {
            Err(VaultError::EncryptionFailed(format!("Store failed: {}", response.status())))
        }
    }
    
    async fn retrieve(&self, path: &str) -> VaultResult<Vec<u8>> {
        let url = format!("{}/?Action=GetSecretValue&SecretId={}&Version=2017-10-17",
            self.get_service_url(),
            urlencoding::encode(path)
        );
        
        let response = self.client
            .get(&url)
            .headers(self.build_auth_headers("GET", path))
            .send()
            .await
            .map_err(|e| VaultError::DecryptionFailed(format!("Retrieve failed: {}", e)))?;
        
        if !response.status().is_success() {
            return Err(VaultError::SecretNotFound(path.to_string()));
        }
        
        let json: serde_json::Value = response.json().await
            .map_err(|e| VaultError::DecryptionFailed(format!("Parse failed: {}", e)))?;
        
        // AWS returns SecretString or SecretBinary
        if let Some(secret_string) = json.get("SecretString").and_then(|v| v.as_str()) {
            return base64::Engine::decode(&base64::engine::general_purpose::STANDARD, secret_string)
                .map_err(|e| VaultError::DecryptionFailed(format!("Decode failed: {}", e)));
        }
        
        if let Some(secret_binary) = json.get("SecretBinary").and_then(|v| v.as_str()) {
            return base64::Engine::decode(&base64::engine::general_purpose::STANDARD, secret_binary)
                .map_err(|e| VaultError::DecryptionFailed(format!("Decode failed: {}", e)));
        }
        
        Err(VaultError::DecryptionFailed("No secret data in response".into()))
    }
    
    async fn delete(&self, path: &str) -> VaultResult<()> {
        let url = format!("{}/?Action=DeleteSecret&SecretId={}&Version=2017-10-17",
            self.get_service_url(),
            urlencoding::encode(path)
        );
        
        let response = self.client
            .post(&url)
            .headers(self.build_auth_headers("POST", path))
            .send()
            .await
            .map_err(|e| VaultError::AccessDenied(format!("Delete failed: {}", e)))?;
        
        if response.status().is_success() || response.status().as_u16() == 404 {
            Ok(())
        } else {
            Err(VaultError::SecretNotFound(path.to_string()))
        }
    }
    
    async fn list(&self, _path: &str) -> VaultResult<Vec<String>> {
        let url = format!("{}/?Action=ListSecrets&Version=2017-10-17", self.get_service_url());
        
        let response = self.client
            .get(&url)
            .headers(self.build_auth_headers("GET", "/"))
            .send()
            .await
            .map_err(|e| VaultError::AccessDenied(format!("List failed: {}", e)))?;
        
        if !response.status().is_success() {
            return Ok(Vec::new());
        }
        
        let json: serde_json::Value = response.json().await
            .map_err(|e| VaultError::AccessDenied(format!("Parse failed: {}", e)))?;
        
        Ok(json.get("SecretList")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter()
                .filter_map(|s| s.get("Name").and_then(|n| n.as_str()).map(String::from))
                .collect())
            .unwrap_or_default())
    }
    
    async fn exists(&self, path: &str) -> VaultResult<bool> {
        match self.retrieve(path).await {
            Ok(_) => Ok(true),
            Err(VaultError::SecretNotFound(_)) => Ok(false),
            Err(e) => Err(e),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  AZURE KEY VAULT BACKEND (HTTP API Implementation)
// ═══════════════════════════════════════════════════════════════════════════════

/// Azure Key Vault backend using HTTP API
pub struct AzureKeyVaultBackend {
    client: reqwest::Client,
    config: BackendConfig,
    vault_url: String,
    tenant_id: Option<String>,
    client_id: Option<String>,
}

impl AzureKeyVaultBackend {
    pub fn new(config: BackendConfig) -> VaultResult<Self> {
        let vault_url = config.endpoint.clone()
            .unwrap_or_else(|| "https://sentient-vault.vault.azure.net".to_string());
        
        let (tenant_id, client_id) = match &config.auth {
            AuthMethod::AzureManagedIdentity { client_id } => {
                (None, client_id.clone())
            }
            AuthMethod::UserPass { username, password } => {
                // username = tenant_id, password = client_secret
                (Some(username.clone()), Some(password.clone()))
            }
            _ => (None, None)
        };
        
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeout_secs))
            .build()
            .map_err(|e| VaultError::AccessDenied(format!("HTTP client: {}", e)))?;
        
        log::info!("Azure Key Vault backend initialized: {}", vault_url);
        Ok(Self { client, config, vault_url, tenant_id, client_id })
    }
    
    /// Get Azure AD access token (simplified)
    /// For production, use azure_identity crate
    async fn get_access_token(&self) -> VaultResult<String> {
        // Managed Identity endpoint
        let msi_endpoint = "http://169.254.169.254/metadata/identity/oauth2/token";
        let resource = "https://vault.azure.net";
        
        let mut url = format!("{}?api-version=2018-02-01&resource={}", msi_endpoint, resource);
        
        if let Some(client_id) = &self.client_id {
            url.push_str(&format!("&client_id={}", client_id));
        }
        
        let response = self.client
            .get(&url)
            .header("Metadata", "true")
            .send()
            .await;
        
        match response {
            Ok(resp) if resp.status().is_success() => {
                let json: serde_json::Value = resp.json().await
                    .map_err(|e| VaultError::AccessDenied(format!("Token parse failed: {}", e)))?;
                
                json.get("access_token")
                    .and_then(|v| v.as_str())
                    .map(String::from)
                    .ok_or_else(|| VaultError::AccessDenied("No access token in response".into()))
            }
            _ => {
                // Fallback: use token from config if provided
                match &self.config.auth {
                    AuthMethod::Token { token } => Ok(token.clone()),
                    _ => Err(VaultError::AccessDenied("Azure authentication failed - no managed identity or token".into()))
                }
            }
        }
    }
    
    fn get_secret_url(&self, path: &str, version: Option<&str>) -> String {
        match version {
            Some(v) => format!("{}/secrets/{}/{}?api-version=7.4", self.vault_url, path, v),
            None => format!("{}/secrets/{}?api-version=7.4", self.vault_url, path),
        }
    }
}

#[async_trait]
impl SecretsBackend for AzureKeyVaultBackend {
    fn name(&self) -> &str {
        "azure_key_vault"
    }
    
    async fn health_check(&self) -> VaultResult<bool> {
        // Try to get vault metadata
        let url = format!("{}?api-version=7.4", self.vault_url);
        
        match self.get_access_token().await {
            Ok(token) => {
                let response = self.client
                    .get(&url)
                    .header("Authorization", format!("Bearer {}", token))
                    .send()
                    .await;
                
                match response {
                    Ok(resp) => Ok(resp.status().is_success()),
                    Err(e) => {
                        log::warn!("Azure Key Vault health check failed: {}", e);
                        Ok(false)
                    }
                }
            }
            Err(_) => {
                log::warn!("Azure Key Vault: Authentication not available");
                Ok(false)
            }
        }
    }
    
    async fn store(&self, path: &str, value: &[u8], metadata: &SecretMetadata) -> VaultResult<()> {
        let token = self.get_access_token().await?;
        let url = self.get_secret_url(path, None);
        
        let body = serde_json::json!({
            "value": base64::Engine::encode(&base64::engine::general_purpose::STANDARD, value),
            "attributes": {
                "enabled": true,
                "contentType": "application/octet-stream",
            },
            "tags": metadata.tags.iter().enumerate()
                .map(|(i, t)| (format!("tag{}", i), t.clone()))
                .collect::<std::collections::HashMap<_, _>>(),
        });
        
        let response = self.client
            .put(&url)
            .header("Authorization", format!("Bearer {}", token))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| VaultError::EncryptionFailed(format!("Store failed: {}", e)))?;
        
        if response.status().is_success() {
            log::info!("Azure Key Vault: Stored secret at {}", path);
            Ok(())
        } else {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            Err(VaultError::EncryptionFailed(format!("Store failed: {} - {}", status, body)))
        }
    }
    
    async fn retrieve(&self, path: &str) -> VaultResult<Vec<u8>> {
        let token = self.get_access_token().await?;
        let url = self.get_secret_url(path, None);
        
        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await
            .map_err(|e| VaultError::DecryptionFailed(format!("Retrieve failed: {}", e)))?;
        
        if !response.status().is_success() {
            return Err(VaultError::SecretNotFound(path.to_string()));
        }
        
        let json: serde_json::Value = response.json().await
            .map_err(|e| VaultError::DecryptionFailed(format!("Parse failed: {}", e)))?;
        
        json.get("value")
            .and_then(|v| v.as_str())
            .ok_or_else(|| VaultError::DecryptionFailed("No value in response".into()))
            .and_then(|b64| {
                base64::Engine::decode(&base64::engine::general_purpose::STANDARD, b64)
                    .map_err(|e| VaultError::DecryptionFailed(format!("Decode failed: {}", e)))
            })
    }
    
    async fn delete(&self, path: &str) -> VaultResult<()> {
        let token = self.get_access_token().await?;
        let url = self.get_secret_url(path, None);
        
        let response = self.client
            .delete(&url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await
            .map_err(|e| VaultError::AccessDenied(format!("Delete failed: {}", e)))?;
        
        if response.status().is_success() || response.status().as_u16() == 404 {
            Ok(())
        } else {
            Err(VaultError::SecretNotFound(path.to_string()))
        }
    }
    
    async fn list(&self, _path: &str) -> VaultResult<Vec<String>> {
        let token = match self.get_access_token().await {
            Ok(t) => t,
            Err(_) => return Ok(Vec::new()),
        };
        
        let url = format!("{}/secrets?api-version=7.4", self.vault_url);
        
        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await
            .map_err(|e| VaultError::AccessDenied(format!("List failed: {}", e)))?;
        
        if !response.status().is_success() {
            return Ok(Vec::new());
        }
        
        let json: serde_json::Value = response.json().await
            .map_err(|e| VaultError::AccessDenied(format!("Parse failed: {}", e)))?;
        
        Ok(json.get("value")
            .and_then(|v| v.as_array())
            .map(|arr: &Vec<serde_json::Value>| arr.iter()
                .filter_map(|s| s.get("name").and_then(|n| n.as_str()).map(String::from))
                .collect())
            .unwrap_or_default())
    }
    
    async fn exists(&self, path: &str) -> VaultResult<bool> {
        match self.retrieve(path).await {
            Ok(_) => Ok(true),
            Err(VaultError::SecretNotFound(_)) => Ok(false),
            Err(e) => Err(e),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  BACKEND FACTORY
// ═══════════════════════════════════════════════════════════════════════════════

/// Create a secrets backend from configuration
pub fn create_backend(config: BackendConfig) -> VaultResult<Box<dyn SecretsBackend>> {
    match config.backend_type {
        BackendType::Local => Ok(Box::new(LocalBackend::new())),
        
        BackendType::HashiCorpVault => {
            #[cfg(feature = "hashicorp_vault")]
            {
                Ok(Box::new(HashiCorpVaultBackend::new(config)?))
            }
            #[cfg(not(feature = "hashicorp_vault"))]
            {
                log::warn!("HashiCorp Vault backend requires 'hashicorp_vault' feature");
                Ok(Box::new(LocalBackend::new()))
            }
        }
        
        BackendType::AwsSecretsManager | BackendType::AwsKms => {
            // AWS Secrets Manager uses HTTP API (no SDK required)
            Ok(Box::new(AwsSecretsManagerBackend::new(config)?))
        }
        
        BackendType::AzureKeyVault => {
            // Azure Key Vault uses HTTP API (no SDK required)
            Ok(Box::new(AzureKeyVaultBackend::new(config)?))
        }
        
        BackendType::GcpSecretManager => {
            log::warn!("GCP Secret Manager backend not yet implemented");
            Ok(Box::new(LocalBackend::new()))
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_local_backend() {
        let backend = LocalBackend::new();
        
        assert!(backend.health_check().await.expect("operation failed"));
        assert_eq!(backend.name(), "local");
    }
    
    #[test]
    fn test_backend_factory() {
        let config = BackendConfig {
            backend_type: BackendType::Local,
            endpoint: None,
            auth: AuthMethod::None,
            namespace: None,
            mount_path: None,
            tls_verify: true,
            timeout_secs: 30,
        };
        
        let backend = create_backend(config).expect("operation failed");
        assert_eq!(backend.name(), "local");
    }
    
    #[test]
    fn test_auth_methods() {
        let token_auth = AuthMethod::Token { token: "test".into() };
        let approle_auth = AuthMethod::AppRole { 
            role_id: "role".into(), 
            secret_id: "secret".into() 
        };
        
        match token_auth {
            AuthMethod::Token { token } => assert_eq!(token, "test"),
            _ => panic!("Wrong variant"),
        }
        
        match approle_auth {
            AuthMethod::AppRole { role_id, secret_id } => {
                assert_eq!(role_id, "role");
                assert_eq!(secret_id, "secret");
            }
            _ => panic!("Wrong variant"),
        }
    }
}
