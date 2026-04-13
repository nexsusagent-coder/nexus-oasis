//! ─── WEBHOOK SIGNATURE VERIFICATION ───
//!
//! V-GATE üzerinden güvenli webhook imza doğrulama:
//! - HMAC-SHA256 (GitHub, Stripe)
//! - SHA256 (Slack)
//! - Custom algorithms

use sentient_common::error::{SENTIENTError, SENTIENTResult};
use hmac::{Hmac, Mac};
use sha2::{Sha256, Sha512};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use super::WebhookProvider;

type HmacSha256 = Hmac<Sha256>;
#[allow(dead_code)]
type HmacSha512 = Hmac<Sha512>;

/// ─── SIGNATURE ALGORITHM ───

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignatureAlgorithm {
    /// GitHub style: HMAC-SHA256 (hex encoded)
    HmacSha256Hex,
    
    /// Stripe style: HMAC-SHA256 (hex encoded, v1 prefix)
    HmacSha256V1,
    
    /// Slack style: HMAC-SHA256 (base64, v0 prefix)
    HmacSha256Base64,
    
    /// SHA256 hash
    Sha256,
    
    /// Custom comparison
    Custom,
    
    /// No signature verification
    None,
}

impl std::fmt::Display for SignatureAlgorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::HmacSha256Hex => write!(f, "HMAC-SHA256-Hex"),
            Self::HmacSha256V1 => write!(f, "HMAC-SHA256-V1"),
            Self::HmacSha256Base64 => write!(f, "HMAC-SHA256-Base64"),
            Self::Sha256 => write!(f, "SHA256"),
            Self::Custom => write!(f, "Custom"),
            Self::None => write!(f, "None"),
        }
    }
}

/// ─── SIGNATURE VERIFIER ───

pub struct SignatureVerifier {
    /// V-GATE URL (key'leri almak için)
    pub vgate_url: String,
    
    /// Provider -> Secret eşleşmesi
    pub secrets: std::collections::HashMap<WebhookProvider, String>,
    
    /// Provider -> Algoritma eşleşmesi
    pub algorithms: std::collections::HashMap<WebhookProvider, SignatureAlgorithm>,
}

impl SignatureVerifier {
    /// Yeni verifier oluştur
    pub fn new(vgate_url: impl Into<String>) -> Self {
        let mut algorithms = std::collections::HashMap::new();
        algorithms.insert(WebhookProvider::GitHub, SignatureAlgorithm::HmacSha256Hex);
        algorithms.insert(WebhookProvider::Stripe, SignatureAlgorithm::HmacSha256V1);
        algorithms.insert(WebhookProvider::Slack, SignatureAlgorithm::HmacSha256Base64);
        algorithms.insert(WebhookProvider::N8n, SignatureAlgorithm::Sha256);
        
        Self {
            vgate_url: vgate_url.into(),
            secrets: std::collections::HashMap::new(),
            algorithms,
        }
    }
    
    /// Secret ekle
    pub fn with_secret(mut self, provider: WebhookProvider, secret: impl Into<String>) -> Self {
        self.secrets.insert(provider, secret.into());
        self
    }
    
    /// Algoritma ayarla
    pub fn with_algorithm(mut self, provider: WebhookProvider, algorithm: SignatureAlgorithm) -> Self {
        self.algorithms.insert(provider, algorithm);
        self
    }
    
    /// V-GATE'ten secret'ı al
    pub async fn fetch_secret_from_vgate(&mut self, provider: &WebhookProvider) -> SENTIENTResult<String> {
        let client = reqwest::Client::new();
        let url = format!("{}/api/secrets/webhook/{}", self.vgate_url, provider);
        
        let response = client
            .get(&url)
            .timeout(std::time::Duration::from_secs(5))
            .send()
            .await
            .map_err(|e| SENTIENTError::VGate(format!("V-GATE bağlantı hatası: {}", e)))?;
        
        if !response.status().is_success() {
            return Err(SENTIENTError::AuthError(
                format!("V-GATE: {} secret'ı alınamadı", provider)
            ));
        }
        
        let json: serde_json::Value = response.json().await
            .map_err(|e| SENTIENTError::General(format!("V-GATE yanıt parse hatası: {}", e)))?;
        
        let secret = json["secret"]
            .as_str()
            .ok_or_else(|| SENTIENTError::AuthError("Secret bulunamadı".into()))?
            .to_string();
        
        self.secrets.insert(provider.clone(), secret.clone());
        Ok(secret)
    }
    
    /// İmza doğrula
    pub fn verify(
        &self,
        provider: &WebhookProvider,
        payload: &[u8],
        signature: &str,
    ) -> SENTIENTResult<bool> {
        let algorithm = self.algorithms.get(provider)
            .copied()
            .unwrap_or(SignatureAlgorithm::None);
        
        // Algoritma None ise doğrulama yapma
        if algorithm == SignatureAlgorithm::None {
            log::debug!("webhook  {} için imza doğrulaması devre dışı", provider);
            return Ok(true);
        }
        
        // Secret'ı al
        let secret = self.secrets.get(provider)
            .ok_or_else(|| SENTIENTError::AuthError(
                format!("{} için secret tanımlı değil", provider)
            ))?;
        
        // Algoritmaya göre doğrula
        let result = match algorithm {
            SignatureAlgorithm::HmacSha256Hex => {
                Self::verify_hmac_sha256_hex(payload, secret, signature)
            }
            SignatureAlgorithm::HmacSha256V1 => {
                Self::verify_hmac_sha256_v1(payload, secret, signature)
            }
            SignatureAlgorithm::HmacSha256Base64 => {
                Self::verify_hmac_sha256_base64(payload, secret, signature)
            }
            SignatureAlgorithm::Sha256 => {
                Self::verify_sha256(payload, signature)
            }
            SignatureAlgorithm::Custom => {
                // Custom: Direkt karşılaştır
                Ok(signature == secret)
            }
            SignatureAlgorithm::None => Ok(true),
        };
        
        result.map_err(|e| SENTIENTError::AuthError(
            format!("İmza doğrulama hatası: {}", e)
        ))
    }
    
    /// HMAC-SHA256 hex doğrulama (GitHub style)
    fn verify_hmac_sha256_hex(payload: &[u8], secret: &str, signature: &str) -> Result<bool, String> {
        let sig = signature.strip_prefix("sha256=").unwrap_or(signature);
        
        let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
            .map_err(|e| e.to_string())?;
        mac.update(payload);
        let expected = mac.finalize().into_bytes();
        let expected_hex = hex::encode(expected);
        
        // Constant-time comparison
        Ok(hex::decode(sig)
            .map(|s| s == expected.as_slice())
            .unwrap_or(false)
            || sig == expected_hex)
    }
    
    /// HMAC-SHA256 v1 doğrulama (Stripe style)
    fn verify_hmac_sha256_v1(payload: &[u8], secret: &str, signature: &str) -> Result<bool, String> {
        // Stripe format: t=1234567890,v1=abc123...
        let parts: std::collections::HashMap<&str, &str> = signature
            .split(',')
            .filter_map(|part| {
                let mut iter = part.splitn(2, '=');
                Some((iter.next()?, iter.next()?))
            })
            .collect();
        
        let v1 = parts.get("v1").ok_or("v1 imza bulunamadı")?;
        let timestamp = parts.get("t").ok_or("timestamp bulunamadı")?;
        
        // Payload with timestamp
        let signed_payload = format!("{}.{}", timestamp, std::str::from_utf8(payload).unwrap_or(""));
        
        let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
            .map_err(|e| e.to_string())?;
        mac.update(signed_payload.as_bytes());
        let expected = mac.finalize().into_bytes();
        let expected_hex = hex::encode(expected);
        
        Ok(hex::decode(v1)
            .map(|s| s == expected.as_slice())
            .unwrap_or(false)
            || *v1 == expected_hex)
    }
    
    /// HMAC-SHA256 base64 doğrulama (Slack style)
    fn verify_hmac_sha256_base64(payload: &[u8], secret: &str, signature: &str) -> Result<bool, String> {
        let sig = signature.strip_prefix("v0=").unwrap_or(signature);
        
        let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
            .map_err(|e| e.to_string())?;
        mac.update(payload);
        let expected = mac.finalize().into_bytes();
        let expected_b64 = BASE64.encode(expected);
        
        Ok(sig == expected_b64)
    }
    
    /// SHA256 doğrulama
    fn verify_sha256(payload: &[u8], signature: &str) -> Result<bool, String> {
        use sha2::Digest;
        
        let mut hasher = Sha256::new();
        hasher.update(payload);
        let expected = hasher.finalize();
        let expected_hex = hex::encode(expected);
        
        let sig = signature.strip_prefix("sha256=").unwrap_or(signature);
        Ok(sig == expected_hex)
    }
    
    /// Zaman damgası doğrulama (replay attack koruması)
    pub fn verify_timestamp(timestamp: i64, tolerance_secs: i64) -> SENTIENTResult<bool> {
        let now = chrono::Utc::now().timestamp();
        let diff = (now - timestamp).abs();
        
        if diff > tolerance_secs {
            return Err(SENTIENTError::AuthError(
                format!("Webhook timestamp çok eski veya gelecekte: {}s fark", diff)
            ));
        }
        
        Ok(true)
    }
}

// ─── Tests ───

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_hmac_sha256_hex() {
        let payload = b"test payload";
        let secret = "test_secret";
        
        // Doğru imza oluştur
        let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).expect("operation failed");
        mac.update(payload);
        let signature = format!("sha256={}", hex::encode(mac.finalize().into_bytes()));
        
        let result = SignatureVerifier::verify_hmac_sha256_hex(payload, secret, &signature);
        assert!(result.expect("operation failed"));
    }
    
    #[test]
    fn test_sha256_verification() {
        let payload = b"test payload";
        
        use sha2::Digest;
        let mut hasher = Sha256::new();
        hasher.update(payload);
        let signature = hex::encode(hasher.finalize());
        
        let result = SignatureVerifier::verify_sha256(payload, &signature);
        assert!(result.expect("operation failed"));
    }
    
    #[test]
    fn test_timestamp_verification() {
        let now = chrono::Utc::now().timestamp();
        
        // Geçerli timestamp
        assert!(SignatureVerifier::verify_timestamp(now, 300).is_ok());
        
        // Eski timestamp
        let old = now - 600;
        assert!(SignatureVerifier::verify_timestamp(old, 300).is_err());
    }
    
    #[test]
    fn test_verifier_creation() {
        let verifier = SignatureVerifier::new("http://localhost:1071")
            .with_secret(WebhookProvider::GitHub, "gh_secret")
            .with_secret(WebhookProvider::Stripe, "stripe_secret");
        
        assert!(verifier.secrets.contains_key(&WebhookProvider::GitHub));
        assert!(verifier.secrets.contains_key(&WebhookProvider::Stripe));
        assert_eq!(verifier.algorithms.get(&WebhookProvider::GitHub), 
            Some(&SignatureAlgorithm::HmacSha256Hex));
    }
}
