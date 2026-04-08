//! ─── SENTIENT V-GATE ENVGUARD ───
//!
//! Ortam değişkenlerinin güvenli yüklenmesi ve yönetimi.
//! API anahtarları ASLA kod içinde sabitlenmez, her zaman
//! ortam değişkenlerinden okunur.
//!
//! ════════════════════════════════════════════════════════════════
//!  GÜVENLİK KURALLARI:
//!  1. API anahtarları ASLA kaynak kodunda yer alamaz
//!  2. API anahtarları ASLA log'a yazılamaz
//!  3. API anahtarları ASLA istemciye gönderilemez
//!  4. .env dosyası .gitignore'a eklenmelidir
//!  5. Production'da şifreli ortam değişkenleri kullanılır
//! ════════════════════════════════════════════════════════════════

use std::env;
use std::path::Path;
use log;

/// API anahtarı sağlayıcıları
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApiProvider {
    OpenRouter,
    OpenAI,
    Anthropic,
    Groq,
    Local,
}

impl ApiProvider {
    /// Ortam değişkeni adını döndür
    pub fn env_key(&self) -> &'static str {
        match self {
            Self::OpenRouter => "OPENROUTER_API_KEY",
            Self::OpenAI => "OPENAI_API_KEY",
            Self::Anthropic => "ANTHROPIC_API_KEY",
            Self::Groq => "GROQ_API_KEY",
            Self::Local => "LOCAL_API_KEY",
        }
    }

    /// Varsayılan base URL
    pub fn base_url(&self) -> &'static str {
        match self {
            Self::OpenRouter => "https://openrouter.ai/api/v1",
            Self::OpenAI => "https://api.openai.com/v1",
            Self::Anthropic => "https://api.anthropic.com/v1",
            Self::Groq => "https://api.groq.com/openai/v1",
            Self::Local => "http://localhost:11434/v1",
        }
    }
}

/// Envguard yapılandırması
#[derive(Debug, Clone)]
pub struct EnvguardConfig {
    /// .env dosyası yolu (opsiyonel)
    pub env_file: Option<String>,
    /// Gerekli ortam değişkenleri
    pub required_keys: Vec<String>,
    /// Hassas anahtarlar (log'a yazılmayacak)
    pub sensitive_keys: Vec<String>,
}

impl Default for EnvguardConfig {
    fn default() -> Self {
        Self {
            env_file: Some(".env".to_string()),
            required_keys: vec![],
            sensitive_keys: vec![
                "OPENROUTER_API_KEY".into(),
                "OPENAI_API_KEY".into(),
                "ANTHROPIC_API_KEY".into(),
                "GROQ_API_KEY".into(),
                "JWT_SECRET".into(),
                "GATEWAY_API_KEYS".into(),
            ],
        }
    }
}

/// Envguard - Güvenli ortam değişkeni yöneticisi
pub struct Envguard {
    config: EnvguardConfig,
    loaded: bool,
}

impl Envguard {
    /// Yeni Envguard oluştur
    pub fn new(config: EnvguardConfig) -> Self {
        Self {
            config,
            loaded: false,
        }
    }

    /// Ortam değişkenlerini yükle
    pub fn load(&mut self) -> EnvguardResult<()> {
        // 1) .env dosyasını yükle (varsa)
        if let Some(ref path) = self.config.env_file {
            self.load_env_file(path)?;
        }

        // 2) Gerekli anahtarları kontrol et
        for key in &self.config.required_keys {
            if env::var(key).is_err() {
                return Err(EnvguardError::MissingRequired(key.clone()));
            }
        }

        self.loaded = true;
        log::info!("🔐  ENVGUARD: Ortam değişkenleri yüklendi");
        Ok(())
    }

    /// .env dosyasını yükle
    fn load_env_file(&self, path: &str) -> EnvguardResult<()> {
        let path = Path::new(path);
        
        if !path.exists() {
            log::debug!("🔐  ENVGUARD: .env dosyası bulunamadı, ortam değişkenleri kullanılıyor");
            return Ok(());
        }

        let content = std::fs::read_to_string(path)
            .map_err(|e| EnvguardError::IoError(format!("Dosya okunamadı: {}", e)))?;

        for line in content.lines() {
            let line = line.trim();
            
            // Boş satırları ve yorumları atla
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            // KEY=VALUE ayrıştır
            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim();
                let value = value.trim();

                // Değerden tırnak işaretlerini kaldır
                let value = value
                    .strip_prefix('"')
                    .and_then(|v| v.strip_suffix('"'))
                    .unwrap_or(value)
                    .strip_prefix('\'')
                    .and_then(|v| v.strip_suffix('\''))
                    .unwrap_or(value);

                // Sadece ortam değişkeni ayarlanmamışsa ayarla
                if env::var(key).is_err() {
                    env::set_var(key, value);
                }
            }
        }

        Ok(())
    }

    /// Ortam değişkeni al (güvenli)
    pub fn get(&self, key: &str) -> EnvguardResult<String> {
        env::var(key).map_err(|_| EnvguardError::NotFound(key.to_string()))
    }

    /// Ortam değişkeni al (varsayılan değerli)
    pub fn get_or(&self, key: &str, default: &str) -> String {
        env::var(key).unwrap_or_else(|_| default.to_string())
    }

    /// API anahtarı al (ASLA log'a yazılmaz!)
    pub fn get_api_key(&self, provider: ApiProvider) -> EnvguardResult<String> {
        let key = env::var(provider.env_key()).map_err(|_| {
            EnvguardError::ApiKeyNotFound(provider.env_key().to_string())
        })?;

        // Anahtarın geçerli olduğunu kontrol et (boş olmamalı)
        if key.is_empty() {
            return Err(EnvguardError::InvalidApiKey(
                provider.env_key().to_string(),
                "Anahtar boş".to_string(),
            ));
        }

        // ASLA log'a yazma!
        Ok(key)
    }

    /// API anahtarı var mı kontrol et
    pub fn has_api_key(&self, provider: ApiProvider) -> bool {
        env::var(provider.env_key())
            .map(|k| !k.is_empty())
            .unwrap_or(false)
    }

    /// Kullanılabilir sağlayıcıları listele
    pub fn available_providers(&self) -> Vec<ApiProvider> {
        [
            ApiProvider::OpenRouter,
            ApiProvider::OpenAI,
            ApiProvider::Anthropic,
            ApiProvider::Groq,
            ApiProvider::Local,
        ]
        .into_iter()
        .filter(|p| self.has_api_key(*p))
        .collect()
    }

    /// Varsayılan sağlayıcıyı belirle
    pub fn default_provider(&self) -> ApiProvider {
        if self.has_api_key(ApiProvider::OpenRouter) {
            ApiProvider::OpenRouter
        } else if self.has_api_key(ApiProvider::Local) {
            ApiProvider::Local
        } else if self.has_api_key(ApiProvider::OpenAI) {
            ApiProvider::OpenAI
        } else if self.has_api_key(ApiProvider::Groq) {
            ApiProvider::Groq
        } else if self.has_api_key(ApiProvider::Anthropic) {
            ApiProvider::Anthropic
        } else {
            ApiProvider::Local
        }
    }

    /// Hassas anahtar mı kontrol et
    pub fn is_sensitive(&self, key: &str) -> bool {
        self.config.sensitive_keys.iter().any(|k| k == key)
    }

    /// Güvenli log (hassas değerleri maskele)
    pub fn safe_log(&self, key: &str, value: &str) -> String {
        if self.is_sensitive(key) {
            if value.is_empty() {
                "(boş)".to_string()
            } else {
                format!("{}***", &value[..value.len().min(8)])
            }
        } else {
            value.to_string()
        }
    }

    /// Yüklendi mi?
    pub fn is_loaded(&self) -> bool {
        self.loaded
    }
}

impl Default for Envguard {
    fn default() -> Self {
        Self::new(EnvguardConfig::default())
    }
}

// ─── Hata Tipleri ───

#[derive(Debug)]
pub enum EnvguardError {
    /// Ortam değişkeni bulunamadı
    NotFound(String),
    /// Gerekli ortam değişkeni eksik
    MissingRequired(String),
    /// API anahtarı bulunamadı
    ApiKeyNotFound(String),
    /// Geçersiz API anahtarı
    InvalidApiKey(String, String),
    /// IO hatası
    IoError(String),
}

impl std::fmt::Display for EnvguardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound(key) => write!(f, "Ortam değişkeni bulunamadı: {}", key),
            Self::MissingRequired(key) => write!(f, "Gerekli ortam değişkeni eksik: {}", key),
            Self::ApiKeyNotFound(provider) => {
                write!(f, "API anahtarı bulunamadı: {} ortam değişkenini ayarlayın", provider)
            }
            Self::InvalidApiKey(provider, reason) => {
                write!(f, "Geçersiz API anahtarı {}: {}", provider, reason)
            }
            Self::IoError(msg) => write!(f, "IO hatası: {}", msg),
        }
    }
}

impl std::error::Error for EnvguardError {}

pub type EnvguardResult<T> = Result<T, EnvguardError>;

// ─── Global Envguard ───

use std::sync::OnceLock;

static GLOBAL_ENVGUARD: OnceLock<Envguard> = OnceLock::new();

/// Global Envguard'ı başlat
pub fn init() {
    let mut guard = Envguard::default();
    if let Err(e) = guard.load() {
        log::warn!("🔐  ENVGUARD: Uyarı: {}", e);
    }
    let _ = GLOBAL_ENVGUARD.set(guard);
}

/// Global Envguard'ı al
pub fn global() -> &'static Envguard {
    GLOBAL_ENVGUARD.get_or_init(|| {
        let mut guard = Envguard::default();
        let _ = guard.load();
        guard
    })
}

/// Kısayol: Ortam değişkeni al
pub fn env(key: &str) -> EnvguardResult<String> {
    global().get(key)
}

/// Kısayol: Ortam değişkeni al (varsayılanlı)
pub fn env_or(key: &str, default: &str) -> String {
    global().get_or(key, default)
}

/// Kısayol: API anahtarı al
pub fn api_key(provider: ApiProvider) -> EnvguardResult<String> {
    global().get_api_key(provider)
}

// ─── Tests ───

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_env_key() {
        assert_eq!(ApiProvider::OpenRouter.env_key(), "OPENROUTER_API_KEY");
        assert_eq!(ApiProvider::OpenAI.env_key(), "OPENAI_API_KEY");
    }

    #[test]
    fn test_provider_base_url() {
        assert_eq!(
            ApiProvider::OpenRouter.base_url(),
            "https://openrouter.ai/api/v1"
        );
    }

    #[test]
    fn test_envguard_safe_log() {
        let guard = Envguard::default();
        let masked = guard.safe_log("OPENROUTER_API_KEY", "sk-or-v1-1234567890");
        assert!(masked.contains("***"));
        assert!(!masked.contains("1234567890"));
    }

    #[test]
    fn test_envguard_safe_log_non_sensitive() {
        let guard = Envguard::default();
        let value = guard.safe_log("RUST_LOG", "info");
        assert_eq!(value, "info");
    }
}
