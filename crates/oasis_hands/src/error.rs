//! ═══════════════════════════════════════════════════════════════════════════════
//!  OASIS HANDS - HATA YÖNETİMİ
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Tüm hatalar SENTIENT diline çevrilir - ham hatalar dışarı sızdırılmaz.

use thiserror::Error;

/// ─── ANA HATA TİPİ ───

#[derive(Debug, Error)]
pub enum HandsError {
    /// Başlatılmamış
    #[error("OASIS-HANDS: {0}")]
    NotInitialized(String),
    
    /// Sovereign ihlali
    #[error("OASIS-HANDS SOVEREIGN: {0}")]
    SovereignViolation(String),
    
    /// Dosya erişim hatası
    #[error("OASIS-HANDS DOSYA: {0}")]
    FileAccess(String),
    
    /// Komut engellendi
    #[error("OASIS-HANDS KOMUT: {0}")]
    CommandBlocked(String),
    
    /// Uygulama engellendi
    #[error("OASIS-HANDS UYGULAMA: {0}")]
    AppBlocked(String),
    
    /// Ekran hatası
    #[error("OASIS-HANDS EKRAN: {0}")]
    ScreenError(String),
    
    /// Input hatası
    #[error("OASIS-HANDS INPUT: {0}")]
    InputError(String),
    
    /// Vision hatası
    #[error("OASIS-HANDS VISION: {0}")]
    VisionError(String),
    
    /// V-GATE hatası
    #[error("OASIS-HANDS V-GATE: {0}")]
    VGateError(String),
    
    /// Timeout
    #[error("OASIS-HANDS ZAMAN AŞIMI: {0}")]
    Timeout(String),
    
    /// Agent hatası
    #[error("OASIS-HANDS AGENT: {0}")]
    AgentError(String),
    
    /// Config bulunamadı
    #[error("OASIS-HANDS CONFIG: Yapılandırma dosyası bulunamadı")]
    ConfigNotFound,
    
    /// Config hatası
    #[error("OASIS-HANDS CONFIG: {0}")]
    ConfigError(String),
    
    /// Acil durum hatası
    #[error("OASIS-HANDS EMERGENCY: {0}")]
    EmergencyError(String),
    
    /// Rate limit aşıldı
    #[error("OASIS-HANDS RATE LIMIT: {0}")]
    RateLimitExceeded(String),
    
    /// Yasaklı bölge erişimi
    #[error("OASIS-HANDS FORBIDDEN REGION: {0}")]
    ForbiddenRegion(String),
    
    /// Zaman kuralı ihlali
    #[error("OASIS-HANDS TIME RULE: {0}")]
    TimeRuleViolation(String),
    
    /// Sandbox hatası
    #[error("OASIS-HANDS SANDBOX: {0}")]
    SandboxError(String),
    
    /// Alert hatası
    #[error("OASIS-HANDS ALERT: {0}")]
    AlertError(String),
    
    /// History hatası
    #[error("OASIS-HANDS HISTORY: {0}")]
    HistoryError(String),
    
    /// Recording hatası
    #[error("OASIS-HANDS RECORDING: {0}")]
    RecordingError(String),
    
    /// Genel hata
    #[error("OASIS-HANDS: {0}")]
    Other(String),
    
    /// PyO3 hatası (gizli)
    #[error("OASIS-HANDS: Python modülünde iç hata")]
    PythonError(#[from] pyo3::PyErr),
    
    /// IO hatası
    #[error("OASIS-HANDS: {0}")]
    IoError(#[from] std::io::Error),
    
    /// String hatası
    #[error("OASIS-HANDS: {0}")]
    StringError(String),
    
    /// HTTP hatası
    #[error("OASIS-HANDS: {0}")]
    HttpError(#[from] reqwest::Error),
}

impl From<String> for HandsError {
    fn from(s: String) -> Self {
        HandsError::StringError(s)
    }
}

/// ─── SONUÇ TİPİ ───

pub type HandsResult<T> = std::result::Result<T, HandsError>;

// ───────────────────────────────────────────────────────────────────────────────
//  HATA ÇEVİRİSİ
// ─────────────────────────────────────────────────────────────────────────────--

/// Ham hatayı SENTIENT diline çevir
pub fn translate_error(raw: &str) -> String {
    // File system errors
    if raw.contains("Permission denied") || raw.contains("permission denied") {
        return "OASIS-HANDS: Dosya erişim izni reddedildi. Sovereign politika gereği bu kaynağa erişilemez.".into();
    }
    
    if raw.contains("No such file") || raw.contains("not found") {
        return "OASIS-HANDS: İstenen dosya veya dizin bulunamadı.".into();
    }
    
    if raw.contains("Is a directory") {
        return "OASIS-HANDS: Belirtilen yol bir dosya değil, bir dizin.".into();
    }
    
    // Network errors
    if raw.contains("Connection refused") || raw.contains("connection refused") {
        return "OASIS-HANDS: Bağlantı reddedildi. Hedef servis yanıt vermiyor.".into();
    }
    
    if raw.contains("timed out") || raw.contains("timeout") {
        return "OASIS-HANDS: İşlem zaman aşımına uğradı. Lütfen tekrar deneyin.".into();
    }
    
    // Python errors (gizle)
    if raw.contains("Traceback") || raw.contains("TypeError") || raw.contains("ValueError") {
        return "OASIS-HANDS: Python modülünde iç hata oluştu. Lütfen tekrar deneyin.".into();
    }
    
    if raw.contains("ModuleNotFoundError") || raw.contains("ImportError") {
        return "OASIS-HANDS: Gerekli modül yüklü değil. Sistem yöneticisine başvurun.".into();
    }
    
    // GUI errors
    if raw.contains("display") || raw.contains("X11") || raw.contains("DISPLAY") {
        return "OASIS-HANDS: Ekran bağlantısı kurulamadı. GUI ortamı gereklidir.".into();
    }
    
    // Default
    format!("OASIS-HANDS: İşlem sırasında bir hata oluştu.")
}

// ───────────────────────────────────────────────────────────────────────────────
//  TESTS
// ─────────────────────────────────────────────────────────────────────────────--

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_translate_permission_denied() {
        let translated = translate_error("Permission denied: /etc/shadow");
        assert!(translated.contains("izni") || translated.contains("izin"));
        assert!(!translated.contains("Permission"));
    }
    
    #[test]
    fn test_translate_not_found() {
        let translated = translate_error("No such file or directory: /tmp/test");
        assert!(translated.contains("bulunamadı"));
    }
    
    #[test]
    fn test_translate_python_error() {
        let translated = translate_error("Traceback (most recent call last):\n  TypeError: ...");
        assert!(translated.contains("iç hata"));
        assert!(!translated.contains("Traceback"));
    }
    
    #[test]
    fn test_translate_timeout() {
        let translated = translate_error("Connection timed out after 30s");
        assert!(translated.contains("zaman aşımı"));
    }
    
    #[test]
    fn test_error_display() {
        let err = HandsError::CommandBlocked("rm -rf /".into());
        let msg = err.to_string();
        assert!(msg.contains("KOMUT"));
        assert!(msg.contains("rm -rf"));
    }
    
    #[test]
    fn test_result_type() {
        fn test_fn() -> HandsResult<()> {
            Err(HandsError::Timeout("Test timeout".into()))
        }
        
        let result = test_fn();
        assert!(result.is_err());
    }
}
