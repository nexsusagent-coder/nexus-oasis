//! ─── SCOUT HATALARI ───

use sentient_common::error::SENTIENTError;
use thiserror::Error;

/// Scout hata turleri
#[derive(Debug, Error)]
pub enum ScoutError {
    #[error("Baglanti hatasi: {0}")]
    Connection(String),
    
    #[error("Rate limit asildi: {0}")]
    RateLimitExceeded(String),
    
    #[error("Platform hatasi: {0}")]
    PlatformError(String),
    
    #[error("Anti-detection basarisiz: {0}")]
    StealthFailed(String),
    
    #[error("Parsing hatasi: {0}")]
    ParseError(String),
    
    #[error("Proxy hatasi: {0}")]
    ProxyError(String),
    
    #[error("Yetkilendirme hatasi: {0}")]
    AuthError(String),
    
    #[error("Timeout: {0}")]
    Timeout(String),
    
    #[error("Veri bulunamadi: {0}")]
    NotFound(String),
    
    #[error("Bilinmeyen hata: {0}")]
    Unknown(String),
}

/// Scout Result tipi
pub type Result<T> = std::result::Result<T, ScoutError>;

impl ScoutError {
    /// SENTIENT diline cevir
    pub fn to_sentient_message(&self) -> String {
        match self {
            ScoutError::Connection(msg) => format!("[Scout] Baglanti saglanamadi: {}", msg),
            ScoutError::RateLimitExceeded(msg) => format!("[Scout] Hiz siniri asildi, bekleniyor: {}", msg),
            ScoutError::PlatformError(msg) => format!("[Scout] Platform yanit vermiyor: {}", msg),
            ScoutError::StealthFailed(msg) => format!("[Scout] Gizlilik katmani hatasi: {}", msg),
            ScoutError::ParseError(msg) => format!("[Scout] Veri islenemedi: {}", msg),
            ScoutError::ProxyError(msg) => format!("[Scout] Vekil sunucu hatasi: {}", msg),
            ScoutError::AuthError(msg) => format!("[Scout] Kimlik dogrulama basarisiz: {}", msg),
            ScoutError::Timeout(msg) => format!("[Scout] Zaman asimi: {}", msg),
            ScoutError::NotFound(msg) => format!("[Scout] Istenen veri bulunamadi: {}", msg),
            ScoutError::Unknown(msg) => format!("[Scout] Bilinmeyen hata: {}", msg),
        }
    }
}

impl From<ScoutError> for SENTIENTError {
    fn from(e: ScoutError) -> Self {
        SENTIENTError::General(e.to_sentient_message())
    }
}
