//! ─── SECURITY HEADERS MIDDLEWARE ───
//!
//! HTTP güvenlik başlıkları ekler ve tehlikeli istekleri engeller.

/// ─── Güvenlik Yapılandırması ───

#[derive(Debug, Clone)]
pub struct SecurityConfig {
    /// CORS izin verilen originler
    pub allowed_origins: Vec<String>,
    /// API anahtarını içeren istekleri engelle
    pub block_api_key_leak: bool,
    /// User-Agent zorunlu olsun mu
    pub require_user_agent: bool,
    /// Maksimum istek boyutu (bytes)
    pub max_request_size: usize,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            allowed_origins: vec!["http://localhost".into(), "http://127.0.0.1".into()],
            block_api_key_leak: true,
            require_user_agent: true,
            max_request_size: 1024 * 1024, // 1MB
        }
    }
}

/// ─── İstek Doğrulama ───

pub struct RequestValidator;

impl RequestValidator {
    /// Model adını doğrula
    pub fn validate_model(model: &str) -> Result<String, String> {
        if model.is_empty() {
            return Err("Model adı boş olamaz".into());
        }

        if model.len() > 100 {
            return Err("Model adı çok uzun".into());
        }

        if model.contains(|c: char| c.is_control()) {
            return Err("Model adı kontrol karakterleri içeremez".into());
        }

        Ok(model.to_string())
    }

    /// Mesaj içeriğini doğrula
    pub fn validate_message(content: &str) -> Result<String, String> {
        if content.trim().is_empty() {
            return Err("Mesaj içeriği boş olamaz".into());
        }

        if content.len() > 100_000 {
            return Err("Mesaj çok uzun (maks. 100K karakter)".into());
        }

        Ok(content.to_string())
    }

    /// Token limitini doğrula
    pub fn validate_max_tokens(max_tokens: Option<u32>) -> Result<u32, String> {
        match max_tokens {
            Some(tokens) if tokens == 0 => Err("max_tokens 0 olamaz".into()),
            Some(tokens) if tokens > 128_000 => Err("max_tokens 128000'den büyük olamaz".into()),
            Some(tokens) => Ok(tokens),
            None => Ok(4096),
        }
    }

    /// Sıcaklık değerini doğrula
    pub fn validate_temperature(temperature: Option<f32>) -> Result<f32, String> {
        match temperature {
            Some(temp) if temp < 0.0 => Err("temperature negatif olamaz".into()),
            Some(temp) if temp > 2.0 => Err("temperature 2.0'dan büyük olamaz".into()),
            Some(temp) => Ok(temp),
            None => Ok(0.7),
        }
    }
}

/// ─── CORS Headers ───

pub fn cors_headers(allowed_origins: &[String]) -> reqwest::header::HeaderMap {
    let mut headers = reqwest::header::HeaderMap::new();
    
    let default_origin = "*".to_string();
    let origin = allowed_origins.first().unwrap_or(&default_origin);
    headers.insert(
        "Access-Control-Allow-Origin",
        origin.parse().expect("Invalid CORS origin header"),
    );
    headers.insert(
        "Access-Control-Allow-Methods",
        "GET, POST, OPTIONS".parse().expect("Invalid CORS methods header"),
    );
    headers.insert(
        "Access-Control-Allow-Headers",
        "Content-Type, Authorization".parse().expect("Invalid CORS headers header"),
    );
    headers.insert(
        "Access-Control-Max-Age",
        "86400".parse().expect("Invalid CORS max-age header"),
    );
    
    headers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_model() {
        assert!(RequestValidator::validate_model("gpt-4").is_ok());
        assert!(RequestValidator::validate_model("").is_err());
        assert!(RequestValidator::validate_model(&"a".repeat(101)).is_err());
    }

    #[test]
    fn test_validate_message() {
        assert!(RequestValidator::validate_message("Merhaba").is_ok());
        assert!(RequestValidator::validate_message("").is_err());
        assert!(RequestValidator::validate_message(&"a".repeat(100_001)).is_err());
    }

    #[test]
    fn test_validate_temperature() {
        assert_eq!(RequestValidator::validate_temperature(None).expect("operation failed"), 0.7);
        assert_eq!(RequestValidator::validate_temperature(Some(1.5)).expect("operation failed"), 1.5);
        assert!(RequestValidator::validate_temperature(Some(-0.1)).is_err());
        assert!(RequestValidator::validate_temperature(Some(2.5)).is_err());
    }
}
