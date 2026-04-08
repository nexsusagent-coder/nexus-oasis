//! ═════════════════════════════════════════════════════════════════
//!  ERROR MODULE
//! ═════════════════════════════════════════════════════════════════

use thiserror::Error;

#[derive(Debug, Error)]
pub enum SENTIENTError {
    #[error("Genel SENTIENT hatasi: {0}")]
    General(String),
    #[error("Bellek (Memory Cube) hatasi: {0}")]
    Memory(String),
    #[error("V-GATE iletisim hatasi: {0}")]
    VGate(String),
    #[error("Guvenlik duvari (Guardrails) ihlali: {0}")]
    Guardrails(String),
    #[error("Python PyO3 koprusu hatasi: {0}")]
    PythonBridge(String),
    #[error("Kimlik dogrulama / yetkilendirme hatasi: {0}")]
    Auth(String),
    #[error("Gorev yurutme hatasi: {0}")]
    TaskExecution(String),
    #[error("Docker sandbox hatasi: {0}")]
    Docker(String),
    #[error("Sandbox zaman asimi: {0}")]
    SandboxTimeout(String),
    #[error("Dogrulama hatasi: {0}")]
    ValidationError(String),
    #[error("Hiz siniri asildi: {0}")]
    RateLimitError(String),
    #[error("Kimlik dogrulama hatasi: {0}")]
    AuthError(String),
    #[error("Veritabani hatasi: {0}")]
    Database(String),
    #[error("Arastirma modulu hatasi: {0}")]
    Research(String),
    #[error("Skill bulunamadi: {0}")]
    SkillNotFound(String),
    #[error("Cekirdek (Core) hatasi: {0}")]
    Core(String),
}

impl SENTIENTError {
    pub fn to_sentient_message(&self) -> String {
        match self {
            Self::General(msg) => format!("SENTIENT: Beklenmeyen bir durum olustu. Otomatik duzeltme deneniyor: {}", msg),
            Self::Memory(msg) => format!("BELLEK: Bilgi Kupu erisim sorunu. Yeniden baglaniliyor: {}", msg),
            Self::VGate(msg) => format!("V-GATE: Vekil sunucu katmaninda aksaklik. Veri akisi yenileniyor: {}", msg),
            Self::Guardrails(msg) => format!("GUARDRAILS: Guvenlik filtresi uyari tetiklendi: {}", msg),
            Self::PythonBridge(msg) => format!("KOPRU: Python modulu ile iletisim koptu. Modul yeniden baslatiliyor: {}", msg),
            Self::Auth(msg) => format!("KIMLIK: Dogrulama asamasindan gecilemedi: {}", msg),
            Self::TaskExecution(msg) => format!("GOREV: Yurutulemeyen operasyon tespit edildi: {}", msg),
            Self::Docker(msg) => format!("SANDBOX: Docker ortaminda sorun olustu. Konteyner yeniden baslatiliyor: {}", msg),
            Self::SandboxTimeout(msg) => format!("SANDBOX: Islem zaman asimina ugradi. Kaynaklar temizleniyor: {}", msg),
            Self::ValidationError(msg) => format!("DOGRULAMA: Gecersiz giris: {}", msg),
            Self::RateLimitError(msg) => format!("LIMIT: Hiz siniri asildi: {}", msg),
            Self::AuthError(msg) => format!("KIMLIK: Dogrulama hatasi: {}", msg),
            Self::Database(msg) => format!("VERITABANI: Veri erisim sorunu. Yeniden deneniyor: {}", msg),
            Self::Research(msg) => format!("ARASTIRMA: Arastirma modulu hatasi: {}", msg),
            Self::SkillNotFound(msg) => format!("SKILL: Istenen yetenek bulunamadi: {}", msg),
            Self::Core(msg) => format!("CEKIRDEK: Sistem katmaninda sorun olustu: {}", msg),
        }
    }

    pub fn summary(&self) -> &str {
        match self {
            Self::General(m)
            | Self::Memory(m)
            | Self::VGate(m)
            | Self::Guardrails(m)
            | Self::PythonBridge(m)
            | Self::Auth(m)
            | Self::TaskExecution(m)
            | Self::Docker(m)
            | Self::SandboxTimeout(m)
            | Self::ValidationError(m)
            | Self::RateLimitError(m)
            | Self::AuthError(m)
            | Self::Database(m)
            | Self::Research(m)
            | Self::SkillNotFound(m)
            | Self::Core(m) => m.as_str(),
        }
    }
}

pub type SENTIENTResult<T> = Result<T, SENTIENTError>;

// Otomatik donusum implementasyonlari
impl From<std::io::Error> for SENTIENTError {
    fn from(e: std::io::Error) -> Self {
        SENTIENTError::General(format!("IO hatasi: {}", e))
    }
}

impl From<serde_json::Error> for SENTIENTError {
    fn from(e: serde_json::Error) -> Self {
        SENTIENTError::General(format!("JSON hatasi: {}", e))
    }
}
