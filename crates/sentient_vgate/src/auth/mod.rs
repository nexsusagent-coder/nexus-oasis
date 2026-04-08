//! ─── SENTIENT V-GATE AUTH (API ANAHTARI YÖNETİMİ) ───
//!
//! API anahtarları asla istemciye暴露 edilmez.
//! Tüm anahtarlar sunucu tarafında şifreli ortam değişkenlerinden okunur.

use sentient_common::error::{SENTIENTError, SENTIENTResult};
use log;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use tokio::sync::RwLock;

/// ─── API Sağlayıcı Tipleri ───

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Provider {
    OpenRouter,
    OpenAI,
    Anthropic,
    Groq,
    Local,
}

impl Provider {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::OpenRouter => "openrouter",
            Self::OpenAI => "openai",
            Self::Anthropic => "anthropic",
            Self::Groq => "groq",
            Self::Local => "local",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "openrouter" => Some(Self::OpenRouter),
            "openai" => Some(Self::OpenAI),
            "anthropic" => Some(Self::Anthropic),
            "groq" => Some(Self::Groq),
            "local" => Some(Self::Local),
            _ => None,
        }
    }

    pub fn default_base_url(&self) -> &'static str {
        match self {
            Self::OpenRouter => "https://openrouter.ai/api/v1",
            Self::OpenAI => "https://api.openai.com/v1",
            Self::Anthropic => "https://api.anthropic.com/v1",
            Self::Groq => "https://api.groq.com/openai/v1",
            Self::Local => "http://localhost:11434/v1",
        }
    }

    pub fn env_key_name(&self) -> &'static str {
        match self {
            Self::OpenRouter => "OPENROUTER_API_KEY",
            Self::OpenAI => "OPENAI_API_KEY",
            Self::Anthropic => "ANTHROPIC_API_KEY",
            Self::Groq => "GROQ_API_KEY",
            Self::Local => "LOCAL_API_KEY",
        }
    }
}

/// ─── API Anahtar Yöneticisi ───

pub struct ApiKeyManager {
    /// Sağlayıcı → API anahtarı eşlemesi (hafızada tutulmaz, her istekte okunur)
    providers: RwLock<HashMap<Provider, ProviderConfig>>,
    /// Varsayılan sağlayıcı
    default_provider: RwLock<Provider>,
    /// Sağlayıcıların etkin olup olmadığı
    enabled_providers: RwLock<Vec<Provider>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub provider: Provider,
    pub base_url: String,
    pub enabled: bool,
    pub rate_limit_per_minute: u32,
    pub max_tokens_per_request: u32,
}

impl Default for ProviderConfig {
    fn default() -> Self {
        Self {
            provider: Provider::OpenRouter,
            base_url: Provider::OpenRouter.default_base_url().to_string(),
            enabled: true,
            rate_limit_per_minute: 60,
            max_tokens_per_request: 8192,
        }
    }
}

impl ApiKeyManager {
    /// Yeni anahtar yöneticisi oluştur
    pub fn new() -> Self {
        log::info!("🔐  AUTH: API anahtar yöneticisi başlatılıyor...");

        let mut providers = HashMap::new();
        
        // Tüm sağlayıcıları varsayılan ayarlarla ekle
        for provider in [Provider::OpenRouter, Provider::OpenAI, Provider::Anthropic, Provider::Groq, Provider::Local] {
            let config = ProviderConfig {
                provider,
                base_url: provider.default_base_url().to_string(),
                enabled: Self::check_provider_available(&provider),
                rate_limit_per_minute: 60,
                max_tokens_per_request: 8192,
            };
            providers.insert(provider, config);
        }

        // Varsayılan sağlayıcıyı belirle
        let default_provider = if Self::check_provider_available(&Provider::OpenRouter) {
            Provider::OpenRouter
        } else if Self::check_provider_available(&Provider::Local) {
            Provider::Local
        } else {
            Provider::Local
        };

        let enabled: Vec<Provider> = providers
            .iter()
            .filter(|(_, c)| c.enabled)
            .map(|(p, _)| *p)
            .collect();

        log::info!("🔐  AUTH: Etkin sağlayıcılar: {:?}", enabled);

        Self {
            providers: RwLock::new(providers),
            default_provider: RwLock::new(default_provider),
            enabled_providers: RwLock::new(enabled),
        }
    }

    /// Sağlayıcının kullanılabilir olup olmadığını kontrol et
    fn check_provider_available(provider: &Provider) -> bool {
        env::var(provider.env_key_name()).is_ok()
    }

    /// API anahtarını güvenli şekilde al (asla log'a yazılmaz!)
    pub async fn get_api_key(&self, provider: &Provider) -> SENTIENTResult<String> {
        let key = env::var(provider.env_key_name()).map_err(|_| {
            SENTIENTError::Auth(format!(
                "{} API anahtarı bulunamadı. {} ortam değişkenini ayarlayın.",
                provider.as_str(),
                provider.env_key_name()
            ))
        })?;

        // Anahtarı asla log'a yazma!
        Ok(key)
    }

    /// Sağlayıcı yapılandırmasını al
    pub async fn get_config(&self, provider: &Provider) -> Option<ProviderConfig> {
        self.providers.read().await.get(provider).cloned()
    }

    /// Varsayılan sağlayıcıyı al
    pub async fn get_default_provider(&self) -> Provider {
        *self.default_provider.read().await
    }

    /// Varsayılan sağlayıcıyı ayarla
    pub async fn set_default_provider(&self, provider: Provider) -> SENTIENTResult<()> {
        if self.enabled_providers.read().await.contains(&provider) {
            *self.default_provider.write().await = provider;
            log::info!("🔐  AUTH: Varsayılan sağlayıcı → {}", provider.as_str());
            Ok(())
        } else {
            Err(SENTIENTError::Auth(format!(
                "{} sağlayıcısı etkin değil veya anahtar yok.",
                provider.as_str()
            )))
        }
    }

    /// Etkin sağlayıcıları listele
    pub async fn list_enabled(&self) -> Vec<Provider> {
        self.enabled_providers.read().await.clone()
    }

    /// Sağlayıcının temel URL'ini al
    pub async fn get_base_url(&self, provider: &Provider) -> String {
        self.providers
            .read()
            .await
            .get(provider)
            .map(|c| c.base_url.clone())
            .unwrap_or_else(|| provider.default_base_url().to_string())
    }

    /// Sağlayıcı yapılandırmasını güncelle
    pub async fn update_config(&self, provider: Provider, config: ProviderConfig) {
        self.providers.write().await.insert(provider, config);
        log::info!("🔐  AUTH: {} yapılandırması güncellendi", provider.as_str());
    }
}

impl Default for ApiKeyManager {
    fn default() -> Self {
        Self::new()
    }
}

/// ─── İstek Doğrulama ───

pub struct RequestValidator;

impl RequestValidator {
    /// Model adını doğrula ve sağlayıcıyı belirle
    pub fn parse_model(model: &str) -> Option<(Provider, String)> {
        // Format: "provider/model" veya direkt model adı
        if model.contains('/') {
            let parts: Vec<&str> = model.splitn(2, '/').collect();
            if parts.len() == 2 {
                let provider = Provider::from_str(parts[0])?;
                return Some((provider, parts[1].to_string()));
            }
        }

        // Model adından sağlayıcıyı tahmin et
        if model.starts_with("gpt") || model.starts_with("o1") {
            Some((Provider::OpenAI, model.to_string()))
        } else if model.starts_with("claude") {
            Some((Provider::Anthropic, model.to_string()))
        } else if model.starts_with("llama") || model.starts_with("mixtral") {
            Some((Provider::Groq, model.to_string()))
        } else if model.starts_with("qwen") {
            Some((Provider::OpenRouter, model.to_string()))
        } else {
            // Varsayılan olarak OpenRouter kullan
            Some((Provider::OpenRouter, model.to_string()))
        }
    }

    /// Token limitini kontrol et
    pub fn validate_token_limit(max_tokens: Option<u32>, limit: u32) -> SENTIENTResult<()> {
        if let Some(tokens) = max_tokens {
            if tokens > limit {
                return Err(SENTIENTError::Auth(format!(
                    "Token limiti aşıldı: {} > {}",
                    tokens, limit
                )));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_as_str() {
        assert_eq!(Provider::OpenRouter.as_str(), "openrouter");
        assert_eq!(Provider::OpenAI.as_str(), "openai");
    }

    #[test]
    fn test_parse_model() {
        let (provider, model) = RequestValidator::parse_model("openrouter/qwen/qwen3.6-plus:free").unwrap();
        assert_eq!(provider, Provider::OpenRouter);
        assert_eq!(model, "qwen/qwen3.6-plus:free");

        let (provider, _) = RequestValidator::parse_model("gpt-4").unwrap();
        assert_eq!(provider, Provider::OpenAI);
    }

    #[test]
    fn test_validate_token_limit() {
        assert!(RequestValidator::validate_token_limit(Some(100), 1000).is_ok());
        assert!(RequestValidator::validate_token_limit(Some(2000), 1000).is_err());
    }
}
