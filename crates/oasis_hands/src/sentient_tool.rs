//! ═══════════════════════════════════════════════════════════════════════════════
//!  SENTIENT TOOL TRAIT - TOOL INTERFACE
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! All SENTIENT tools implement this trait.
//! The Operating System That Thinks - intelligent and powerful tools.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ───────────────────────────────────────────────────────────────────────────────
//  SENTIENT TOOL TRAIT
// ───────────────────────────────────────────────────────────────────────────────

/// SentientTool - All tools must implement this trait
///
/// This trait defines the standard interface for all tools in the SENTIENT system.
/// Each tool has a name, description, and parameter schema.
///
/// # Security
/// - `unsafe` Rust usage is prohibited
/// - All actions are logged
/// - Errors are converted to SENTIENT format
#[async_trait]
pub trait SentientTool: Send + Sync {
    /// Aracın benzersiz tanımlayıcısı
    fn name(&self) -> &str;
    
    /// Aracın açıklaması (Türkçe)
    fn description(&self) -> &str;
    
    /// Aracın kategorisi
    fn category(&self) -> ToolCategory;
    
    /// Risk seviyesi
    fn risk_level(&self) -> RiskLevel;
    
    /// Parametre şeması
    fn parameters(&self) -> Vec<ToolParameter>;
    
    /// Aracı çalıştır
    async fn execute(&self, params: HashMap<String, serde_json::Value>) -> SentientToolResult;
    
    /// Aracın kullanılabilir olup olmadığını kontrol et
    fn is_available(&self) -> bool {
        true
    }
    
    /// Aracın versiyonu
    fn version(&self) -> &str {
        "0.1.0"
    }
}

// ───────────────────────────────────────────────────────────────────────────────
//  TİPLER
// ───────────────────────────────────────────────────────────────────────────────

/// Araç kategorisi
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ToolCategory {
    /// Dosya sistemi işlemleri
    FileSystem,
    /// Process/komut çalıştırma
    Process,
    /// Tarayıcı ve web işlemleri
    Browser,
    /// Web araması ve içerik çekme
    Web,
    /// Ekran ve görsel işlemler
    Screen,
    /// Ağ iletişimi
    Network,
    /// Sistem bilgisi
    System,
    /// Veri işleme
    Data,
    /// İletişim araçları
    Communication,
    /// Zamanlama ve planlama
    Scheduling,
    /// Yapay zeka ve LLM
    Intelligence,
    /// Entegrasyon araçları
    Integration,
    /// Bellek ve depolama
    Memory,
    /// Ajan yönetimi
    Agent,
    /// Kullanıcı etkileşimi
    Interaction,
    /// Geliştirme araçları
    Development,
    /// Verimlilik araçları
    Productivity,
    /// Analiz araçları
    Analysis,
}

impl ToolCategory {
    /// Kategorinin Türkçe adını döndürür
    pub fn to_turkish(&self) -> &'static str {
        match self {
            Self::FileSystem => "Dosya Sistemi",
            Self::Process => "İşlem",
            Self::Browser => "Tarayıcı",
            Self::Screen => "Ekran",
            Self::Network => "Ağ",
            Self::System => "Sistem",
            Self::Data => "Veri",
            Self::Communication => "İletişim",
            Self::Scheduling => "Zamanlama",
            Self::Intelligence => "Yapay Zeka",
            Self::Integration => "Entegrasyon",
            Self::Memory => "Bellek",
            Self::Agent => "Ajan",
            Self::Web => "Web",
            Self::Interaction => "Etkileşim",
            Self::Development => "Geliştirme",
            Self::Productivity => "Verimlilik",
            Self::Analysis => "Analiz",
        }
    }
}

/// Risk seviyesi
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskLevel {
    /// Düşük risk - güvenli işlemler
    Low,
    /// Orta risk - dikkatli kullanım
    Medium,
    /// Yüksek risk - onay gerekebilir
    High,
    /// Kritik risk - mutlak onay gerekir
    Critical,
}

impl RiskLevel {
    /// Risk seviyesinin Türkçe adını döndürür
    pub fn to_turkish(&self) -> &'static str {
        match self {
            Self::Low => "Düşük",
            Self::Medium => "Orta",
            Self::High => "Yüksek",
            Self::Critical => "Kritik",
        }
    }
}

/// Araç parametresi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolParameter {
    /// Parametre adı
    pub name: String,
    /// Parametre tipi (string, number, boolean, object, array)
    pub param_type: String,
    /// Zorunlu mu?
    pub required: bool,
    /// Türkçe açıklama
    pub description: String,
    /// Varsayılan değer (opsiyonel)
    pub default: Option<serde_json::Value>,
}

impl ToolParameter {
    /// Yeni parametre oluştur
    pub fn new(name: &str, param_type: &str, required: bool, description: &str) -> Self {
        Self {
            name: name.to_string(),
            param_type: param_type.to_string(),
            required,
            description: description.to_string(),
            default: None,
        }
    }
    
    /// Varsayılan değerli parametre oluştur
    pub fn with_default(name: &str, param_type: &str, description: &str, default: serde_json::Value) -> Self {
        Self {
            name: name.to_string(),
            param_type: param_type.to_string(),
            required: false,
            description: description.to_string(),
            default: Some(default),
        }
    }
}

/// Tool result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentientToolResult {
    /// İşlem başarılı mı?
    pub success: bool,
    /// Çıktı mesajı
    pub output: String,
    /// Ek veri (opsiyonel)
    pub data: Option<serde_json::Value>,
    /// Hata mesajı (başarısızsa)
    pub error: Option<String>,
    /// Çalıştırma süresi (ms)
    pub duration_ms: u64,
}

impl SentientToolResult {
    /// Create successful result
    pub fn success(output: &str) -> Self {
        Self {
            success: true,
            output: output.to_string(),
            data: None,
            error: None,
            duration_ms: 0,
        }
    }
    
    /// Success with data
    pub fn success_with_data(output: &str, data: serde_json::Value) -> Self {
        Self {
            success: true,
            output: output.to_string(),
            data: Some(data),
            error: None,
            duration_ms: 0,
        }
    }
    
    /// Create failure result
    pub fn failure(error: &str) -> Self {
        Self {
            success: false,
            output: String::new(),
            data: None,
            error: Some(error.to_string()),
            duration_ms: 0,
        }
    }
    
    /// Add duration info
    pub fn with_duration(mut self, duration_ms: u64) -> Self {
        self.duration_ms = duration_ms;
        self
    }
}

// ───────────────────────────────────────────────────────────────────────────────
//  TESTS
// ───────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_tool_parameter_new() {
        let param = ToolParameter::new("path", "string", true, "Dosya yolu");
        assert_eq!(param.name, "path");
        assert!(param.required);
        assert!(param.default.is_none());
    }
    
    #[test]
    fn test_tool_parameter_with_default() {
        let param = ToolParameter::with_default(
            "verbose",
            "boolean",
            "Ayrıntılı çıktı",
            serde_json::json!(false)
        );
        assert!(!param.required);
        assert!(param.default.is_some());
    }
    
    #[test]
    fn test_tool_result_success() {
        let result = SentientToolResult::success("Islem basarili");
        assert!(result.success);
        assert!(result.error.is_none());
    }

    #[test]
    fn test_tool_result_failure() {
        let result = SentientToolResult::failure("Bir hata olustu");
        assert!(!result.success);
        assert!(result.error.is_some());
    }

    #[test]
    fn test_tool_result_with_data() {
        let data = serde_json::json!({"count": 42});
        let result = SentientToolResult::success_with_data("Tamamlandi", data.clone());
        assert!(result.data.is_some());
        assert_eq!(result.data.expect("operation failed")["count"], 42);
    }

    #[test]
    fn test_tool_result_with_duration() {
        let result = SentientToolResult::success("Test")
            .with_duration(150);
        assert_eq!(result.duration_ms, 150);
    }
    
    #[test]
    fn test_tool_category_to_turkish() {
        assert_eq!(ToolCategory::FileSystem.to_turkish(), "Dosya Sistemi");
        assert_eq!(ToolCategory::Browser.to_turkish(), "Tarayıcı");
        assert_eq!(ToolCategory::Intelligence.to_turkish(), "Yapay Zeka");
    }
    
    #[test]
    fn test_risk_level_to_turkish() {
        assert_eq!(RiskLevel::Low.to_turkish(), "Düşük");
        assert_eq!(RiskLevel::Critical.to_turkish(), "Kritik");
    }
}
