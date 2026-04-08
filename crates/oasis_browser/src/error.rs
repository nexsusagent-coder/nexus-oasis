//! ═══════════════════════════════════════════════════════════════════════════════
//!  BROWSER ERROR - SENTIENT DİLİ HATA YÖNETİMİ
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Tüm ham hatalar SENTIENT'nın diline çevrilir.
//! Son kullanıcı asla Python traceback veya Rust panic görmez.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum BrowserError {
    // ─── Sovereign Sandbox Hataları ───
    #[error("OASIS-BROWSER: Güvenlik ihlali tespit edildi - {0}")]
    SandboxViolation(String),
    
    #[error("OASIS-BROWSER: Dosya sistemi erişimi reddedildi - {0}")]
    FileSystemBlocked(String),
    
    #[error("OASIS-BROWSER: Yerel ağ erişimi reddedildi - {0}")]
    LocalNetworkBlocked(String),
    
    // ─── Tarayıcı Hataları ───
    #[error("OASIS-BROWSER: Tarayıcı başlatılamadı - {0}")]
    BrowserInitFailed(String),
    
    #[error("OASIS-BROWSER: Sayfa yüklenemedi - {0}")]
    PageLoadFailed(String),
    
    #[error("OASIS-BROWSER: Element bulunamadı - {0}")]
    ElementNotFound(String),
    
    #[error("OASIS-BROWSER: Zaman aşımı - {0}")]
    Timeout(String),
    
    #[error("OASIS-BROWSER: Navigasyon hatası - {0}")]
    NavigationError(String),
    
    // ─── V-GATE Hataları ───
    #[error("V-GATE: LLM bağlantısı kesildi - {0}")]
    VGateConnectionFailed(String),
    
    #[error("V-GATE: Yanıt alınamadı - {0}")]
    VGateTimeout(String),
    
    #[error("V-GATE: Güvenlik filtresi uyarısı - {0}")]
    VGateGuardrails(String),
    
    // ─── Python Köprü Hataları ───
    #[error("KÖPRÜ: Python modülü yanıt vermiyor - {0}")]
    PythonBridgeError(String),
    
    #[error("KÖPRÜ: Browser-Use hatası - {0}")]
    BrowserUseError(String),
    
    // ─── Genel Hatalar ───
    #[error("OASIS-BROWSER: Başlatılmadı - {0}")]
    NotInitialized(String),
    
    #[error("OASIS-BROWSER: Geçersiz işlem - {0}")]
    InvalidOperation(String),
    
    #[error("OASIS-BROWSER: Kaynak tükeniyor - {0}")]
    ResourceExhausted(String),
    
    #[error("OASIS-BROWSER: {0}")]
    Other(String),
}

impl BrowserError {
    /// SENTIENT dilinde kullanıcı dostu mesaj
    pub fn to_user_message(&self) -> String {
        match self {
            Self::SandboxViolation(msg) => 
                format!("🔒 TARAYICI: Güvenlik engeli. {}", msg),
            Self::FileSystemBlocked(path) => 
                format!("🔒 TARAYICI: Dosya erişimi yasak. '{}' güvenli modda çalışmaz.", path),
            Self::LocalNetworkBlocked(addr) => 
                format!("🔒 TARAYICI: Yerel ağ erişimi yasak. '{}' adresine ulaşılamaz.", addr),
            Self::BrowserInitFailed(reason) => 
                format!("🌐 TARAYICI: Başlatılamadı. {} Sistem yeniden deneniyor.", reason),
            Self::PageLoadFailed(url) => 
                format!("🌐 TARAYICI: Sayfa açılamadı. '{}' adresi erişilemez.", url),
            Self::ElementNotFound(selector) => 
                format!("🔍 TARAYICI: Öğe bulunamadı. '{}' seçicisi geçersiz veya öğe yok.", selector),
            Self::Timeout(operation) => 
                format!("⏱️ TARAYICI: İşlem zaman aşımına uğradı. {} Yeniden deneniyor.", operation),
            Self::NavigationError(url) => 
                format!("🧭 TARAYICI: Yönlendirme başarısız. '{}' hedefine ulaşılamadı.", url),
            Self::VGateConnectionFailed(reason) => 
                format!("🚪 V-GATE: Bağlantı kopmuş. {} Yeniden bağlanılıyor.", reason),
            Self::VGateTimeout(details) => 
                format!("🚪 V-GATE: Yanıt bekleniyor. {} İşlem kuyruğa alındı.", details),
            Self::VGateGuardrails(warning) => 
                format!("🛡️ V-GATE: Güvenlik uyarısı. {} İstek reddedildi.", warning),
            Self::PythonBridgeError(module) => 
                format!("🔗 KÖPRÜ: Python modülü ile bağlantı koptu. '{}' yeniden yükleniyor.", module),
            Self::BrowserUseError(error) => 
                format!("🔗 KÖPRÜ: Browser-Use hatası. {}", error),
            Self::NotInitialized(msg) => 
                format!("⚠️ TARAYICI: Hazır değil. {}", msg),
            Self::InvalidOperation(op) => 
                format!("⚠️ TARAYICI: Geçersiz istek. {}", op),
            Self::ResourceExhausted(resource) => 
                format!("💾 TARAYICI: Kaynak tükeniyor. {} Temizlik yapılıyor.", resource),
            Self::Other(msg) => 
                format!("🌐 TARAYICI: {}", msg),
        }
    }
    
    /// Kısa özet
    pub fn summary(&self) -> String {
        match self {
            Self::SandboxViolation(_) => "Güvenlik ihlali".into(),
            Self::FileSystemBlocked(_) => "Dosya sistemi engelli".into(),
            Self::LocalNetworkBlocked(_) => "Yerel ağ engelli".into(),
            Self::BrowserInitFailed(_) => "Tarayıcı başlatma hatası".into(),
            Self::PageLoadFailed(_) => "Sayfa yükleme hatası".into(),
            Self::ElementNotFound(_) => "Element bulunamadı".into(),
            Self::Timeout(_) => "Zaman aşımı".into(),
            Self::NavigationError(_) => "Navigasyon hatası".into(),
            Self::VGateConnectionFailed(_) => "V-GATE bağlantı hatası".into(),
            Self::VGateTimeout(_) => "V-GATE zaman aşımı".into(),
            Self::VGateGuardrails(_) => "V-GATE güvenlik uyarısı".into(),
            Self::PythonBridgeError(_) => "Python köprü hatası".into(),
            Self::BrowserUseError(_) => "Browser-Use hatası".into(),
            Self::NotInitialized(_) => "Başlatılmadı".into(),
            Self::InvalidOperation(_) => "Geçersiz işlem".into(),
            Self::ResourceExhausted(_) => "Kaynak tükeniyor".into(),
            Self::Other(msg) => msg.clone(),
        }
    }
    
    /// Yeniden denenebilir mi?
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            Self::Timeout(_) |
            Self::VGateConnectionFailed(_) |
            Self::VGateTimeout(_) |
            Self::PythonBridgeError(_) |
            Self::PageLoadFailed(_)
        )
    }
}

pub type BrowserResult<T> = Result<T, BrowserError>;

// ─── Dış Modül Hatalarından Dönüşüm ───

impl From<url::ParseError> for BrowserError {
    fn from(e: url::ParseError) -> Self {
        BrowserError::InvalidOperation(format!("URL ayrıştırma hatası: {}", e))
    }
}

impl From<std::io::Error> for BrowserError {
    fn from(e: std::io::Error) -> Self {
        BrowserError::Other(format!("I/O hatası: {}", e))
    }
}

impl From<serde_json::Error> for BrowserError {
    fn from(e: serde_json::Error) -> Self {
        BrowserError::Other(format!("JSON hatası: {}", e))
    }
}

// ─── SENTIENTError Entegrasyonu ───

impl From<BrowserError> for sentient_common::error::SENTIENTError {
    fn from(e: BrowserError) -> Self {
        match e {
            BrowserError::SandboxViolation(msg) => 
                sentient_common::error::SENTIENTError::Guardrails(msg),
            BrowserError::VGateConnectionFailed(msg) => 
                sentient_common::error::SENTIENTError::VGate(msg),
            BrowserError::PythonBridgeError(msg) => 
                sentient_common::error::SENTIENTError::PythonBridge(msg),
            _ => sentient_common::error::SENTIENTError::TaskExecution(e.to_user_message()),
        }
    }
}
