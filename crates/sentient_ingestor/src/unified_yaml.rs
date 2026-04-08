//! ═══════════════════════════════════════════════════════════════════════════════
//!  UNIFIED YAML FORMAT - SENTIENT Skill Standard
//! ═══════════════════════════════════════════════════════════════════════════════

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Unified Skill Format - Tüm skill'ler bu formata dönüştürülür
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedSkill {
    /// Schema versiyonu
    pub schema_version: String,
    
    /// Benzersiz SENTIENT skill ID'si
    pub id: String,
    
    /// Orijinal skill adı
    pub name: String,
    
    /// Slug (URL-friendly isim)
    pub slug: String,
    
    /// Açıklama
    pub description: String,
    
    /// Kategori
    pub category: String,
    
    /// Alt kategori (opsiyonel)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subcategory: Option<String>,
    
    /// Metadata
    pub metadata: SkillMetadata,
    
    /// Parametreler
    #[serde(default)]
    pub parameters: Vec<SkillParameter>,
    
    /// Örnekler
    #[serde(default)]
    pub examples: Vec<SkillExample>,
    
    /// Etiketler
    #[serde(default)]
    pub tags: Vec<String>,
    
    /// Bağımlılıklar (diğer skill'ler)
    #[serde(default)]
    pub dependencies: Vec<String>,
    
    /// Kaynak URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<String>,
    
    /// GitHub URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub github_url: Option<String>,
    
    /// Oluşturulma tarihi
    pub created_at: String,
    
    /// Güncelleme tarihi
    pub updated_at: String,
    
    /// Hash (duplicate detection için)
    pub hash: String,
    
    /// SENTIENT özgü extensions
    #[serde(default)]
    pub sentient_extensions: SENTIENTExtensions,
}

/// Skill Metadata
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SkillMetadata {
    /// Yazar
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    
    /// Versiyon
    #[serde(default = "default_version")]
    pub version: String,
    
    /// Lisans
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
    
    /// Minimum SENTIENT versiyonu
    #[serde(default = "default_min_version")]
    pub min_sentient_version: String,
    
    /// Platformlar (linux, macos, windows)
    #[serde(default)]
    pub platforms: Vec<String>,
    
    /// Dil desteği
    #[serde(default)]
    pub languages: Vec<String>,
    
    /// Tahmini kullanım sıklığı (1-10)
    #[serde(default = "default_frequency")]
    pub estimated_frequency: u8,
    
    /// Güvenilirlik skoru (0.0-1.0)
    #[serde(default = "default_reliability")]
    pub reliability_score: f32,
    
    /// Risk seviyesi (low, medium, high)
    #[serde(default = "default_risk")]
    pub risk_level: String,
}

fn default_version() -> String { "1.0.0".to_string() }
fn default_min_version() -> String { "0.1.0".to_string() }
fn default_frequency() -> u8 { 5 }
fn default_reliability() -> f32 { 0.8 }
fn default_risk() -> String { "low".to_string() }

/// Skill Parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillParameter {
    /// Parametre adı
    pub name: String,
    
    /// Açıklama
    pub description: String,
    
    /// Veri tipi (string, number, boolean, array, object)
    #[serde(rename = "type")]
    pub param_type: String,
    
    /// Zorunlu mu?
    #[serde(default)]
    pub required: bool,
    
    /// Varsayılan değer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<serde_json::Value>,
    
    /// Enum değerleri (opsiyonel)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enum_values: Option<Vec<String>>,
    
    /// Min/max (sayılar için)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
}

/// Skill Example
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillExample {
    /// Örnek başlığı
    pub title: String,
    
    /// Açıklama
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    
    /// Girdi
    pub input: String,
    
    /// Beklenen çıktı (opsiyonel)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_output: Option<String>,
}

/// SENTIENT Extensions - SENTIENT'ya özgü özellikler
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SENTIENTExtensions {
    /// V-GATE üzerinden erişim gerekiyor mu?
    #[serde(default)]
    pub requires_vgate: bool,
    
    /// Bellek entegrasyonu var mı?
    #[serde(default)]
    pub uses_memory: bool,
    
    /// Browser erişimi gerekiyor mu?
    #[serde(default)]
    pub requires_browser: bool,
    
    /// Dosya sistemi erişimi
    #[serde(default)]
    pub requires_filesystem: bool,
    
    /// Ağ erişimi
    #[serde(default)]
    pub requires_network: bool,
    
    /// Docker sandbox gerekiyor mu?
    #[serde(default)]
    pub requires_sandbox: bool,
    
    /// Tool mapping (hangi Rust tool'una map edilecek)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_mapping: Option<String>,
    
    /// Timeout (saniye)
    #[serde(default = "default_timeout")]
    pub timeout_secs: u64,
    
    /// Retry count
    #[serde(default)]
    pub max_retries: u8,
    
    /// Rate limit (requests per minute)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit: Option<u32>,
    
    /// Custom config
    #[serde(default)]
    pub custom_config: HashMap<String, serde_json::Value>,
}

fn default_timeout() -> u64 { 30 }

impl UnifiedSkill {
    /// Yeni UnifiedSkill oluştur
    pub fn new(name: &str, description: &str, category: &str) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        let slug = name.to_lowercase()
            .replace(" ", "-")
            .replace("_", "-")
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '-')
            .collect();
        
        let id = crate::sentient_skill_id(name, category);
        let hash = crate::skill_hash(name, description);
        
        Self {
            schema_version: crate::SKILL_SCHEMA_VERSION.to_string(),
            id,
            name: name.to_string(),
            slug,
            description: description.to_string(),
            category: category.to_string(),
            subcategory: None,
            metadata: SkillMetadata::default(),
            parameters: Vec::new(),
            examples: Vec::new(),
            tags: Vec::new(),
            dependencies: Vec::new(),
            source_url: None,
            github_url: None,
            created_at: now.clone(),
            updated_at: now,
            hash,
            sentient_extensions: SENTIENTExtensions::default(),
        }
    }
    
    /// YAML'e dönüştür
    pub fn to_yaml(&self) -> Result<String, serde_yaml::Error> {
        serde_yaml::to_string(self)
    }
    
    /// YAML'den oku
    pub fn from_yaml(yaml: &str) -> Result<Self, serde_yaml::Error> {
        serde_yaml::from_str(yaml)
    }
    
    /// JSON'e dönüştür
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}
