//! Research hata yönetimi
//! Tüm Python hataları SENTIENT diline çevrilir

use thiserror::Error;

/// Research katmanı hataları
#[derive(Error, Debug)]
pub enum ResearchError {
    /// Python modülü yüklenemedi
    #[error("SENTIENT-RESEARCH: Python modülü yüklenemedi → {module}. {reason}")]
    ModuleLoadFailed {
        module: String,
        reason: String,
    },

    /// MindSearch hatası
    #[error("SENTIENT-RESEARCH: MindSearch arama hatası → {message}")]
    MindSearchError {
        message: String,
    },

    /// AutoResearch hatası
    #[error("SENTIENT-RESEARCH: AutoResearch planlama hatası → {message}")]
    AutoResearchError {
        message: String,
    },

    /// Python GIL hatası
    #[error("SENTIENT-RESEARCH: Python GIL alınamadı → {reason}")]
    GilError {
        reason: String,
    },

    /// Bellek hatası
    #[error("SENTIENT-RESEARCH: Bellek işlemi başarısız → {operation}")]
    MemoryError {
        operation: String,
    },

    /// V-GATE iletişim hatası
    #[error("SENTIENT-RESEARCH: V-GATE iletişim hatası → {status}: {message}")]
    VGateError {
        status: u16,
        message: String,
    },

    /// Zaman aşımı
    #[error("SENTIENT-RESEARCH: İşlem zaman aşımına uğradı → {operation} ({timeout_secs}s)")]
    Timeout {
        operation: String,
        timeout_secs: u64,
    },

    /// Başlatılmadı hatası
    #[error("SENTIENT-RESEARCH: Sistem başlatılmadı → {reason}")]
    NotInitialized {
        reason: String,
    },

    /// Geçersiz sorgu
    #[error("SENTIENT-RESEARCH: Geçersiz sorgu → {reason}")]
    InvalidQuery {
        reason: String,
    },

    /// Ağ hatası
    #[error("SENTIENT-RESEARCH: Ağ bağlantı hatası → {reason}")]
    NetworkError {
        reason: String,
    },

    /// Python exception
    #[error("SENTIENT-RESEARCH: Python exception → {exception_type}: {message}")]
    PythonException {
        exception_type: String,
        message: String,
    },

    /// Genel hata
    #[error("SENTIENT-RESEARCH: {message}")]
    Generic {
        message: String,
    },
}

/// Research result tipi
pub type ResearchResult<T> = Result<T, ResearchError>;

impl From<reqwest::Error> for ResearchError {
    fn from(err: reqwest::Error) -> Self {
        ResearchError::NetworkError {
            reason: err.to_string(),
        }
    }
}

impl From<std::io::Error> for ResearchError {
    fn from(err: std::io::Error) -> Self {
        ResearchError::NetworkError {
            reason: err.to_string(),
        }
    }
}

impl From<serde_json::Error> for ResearchError {
    fn from(err: serde_json::Error) -> Self {
        ResearchError::Generic {
            message: format!("JSON hatası: {}", err),
        }
    }
}

/// Python hata mesajlarını SENTIENT diline çevir
pub fn translate_python_error(message: String) -> String {
    // Yaygın Python hatalarını SENTIENT diline çevir
    let translated = message
        .replace("ModuleNotFoundError", "Modül bulunamadı")
        .replace("ImportError", "İçe aktarma hatası")
        .replace("AttributeError", "Özellik hatası")
        .replace("TypeError", "Tip hatası")
        .replace("ValueError", "Değer hatası")
        .replace("KeyError", "Anahtar hatası")
        .replace("IndexError", "İndeks hatası")
        .replace("RuntimeError", "Çalışma zamanı hatası")
        .replace("ConnectionError", "Bağlantı hatası")
        .replace("TimeoutError", "Zaman aşımı")
        .replace("HTTPError", "HTTP hatası");
    
    translated
}
