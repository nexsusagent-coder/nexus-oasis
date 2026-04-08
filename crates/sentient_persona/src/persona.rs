//! ─── PERSONA DEFINITION ───
//!
//! Persona yapısı ve özellikleri

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};

// ═══════════════════════════════════════════════════════════════════════════════
// PERSONA - Ana yapı
// ═══════════════════════════════════════════════════════════════════════════════

/// Persona - SENTIENT ajanının kişilik tanımı
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Persona {
    /// Benzersiz ID
    pub id: Uuid,
    /// Persona adı
    pub name: String,
    /// Açıklama
    pub description: String,
    /// Kimlik bilgileri
    pub identity: PersonaIdentity,
    /// Kişilik özellikleri
    pub traits: PersonalityTraits,
    /// Davranış kalıpları
    pub behaviors: Vec<BehaviorRule>,
    /// Konuşma tarzı
    pub communication: CommunicationStyle,
    /// Uzmanlık alanları
    pub expertise: Vec<Expertise>,
    /// Metadata
    pub metadata: PersonaMetadata,
    /// Yapılandırma
    pub config: PersonaConfig,
}

impl Default for Persona {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: "SENTIENT".into(),
            description: "NEXUS OASIS Yapay Zeka İşletim Sistemi".into(),
            identity: PersonaIdentity::default(),
            traits: PersonalityTraits::default(),
            behaviors: Vec::new(),
            communication: CommunicationStyle::default(),
            expertise: Vec::new(),
            metadata: PersonaMetadata::default(),
            config: PersonaConfig::default(),
        }
    }
}

impl Persona {
    /// Yeni persona oluştur
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }
    
    /// Persona'dan sistem promptu oluştur
    pub fn to_system_prompt(&self) -> String {
        let mut prompt = String::new();
        
        // Kimlik
        prompt.push_str(&format!("# {}\n\n", self.name));
        prompt.push_str(&format!("{}\n\n", self.description));
        
        // Kişilik
        prompt.push_str("## Kişilik\n\n");
        prompt.push_str(&format!("- Ton: {}\n", self.communication.tone));
        prompt.push_str(&format!("- Stil: {}\n", self.communication.style));
        
        // Özellikler
        if !self.traits.values.is_empty() {
            prompt.push_str("\n## Özellikler\n\n");
            for (trait_name, value) in &self.traits.values {
                prompt.push_str(&format!("- {}: {}\n", trait_name, value));
            }
        }
        
        // Uzmanlık
        if !self.expertise.is_empty() {
            prompt.push_str("\n## Uzmanlık Alanları\n\n");
            for exp in &self.expertise {
                prompt.push_str(&format!("- {} (Seviye: {}/10)\n", exp.name, exp.level));
            }
        }
        
        // Davranış kuralları
        if !self.behaviors.is_empty() {
            prompt.push_str("\n## Davranış Kuralları\n\n");
            for rule in &self.behaviors {
                prompt.push_str(&format!("- {}\n", rule.description));
            }
        }
        
        prompt
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// PERSONA IDENTITY
// ═══════════════════════════════════════════════════════════════════════════════

/// Persona kimliği
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonaIdentity {
    /// Rol tanımı
    pub role: String,
    /// Arka plan hikayesi
    pub background: String,
    /// Hedefler
    pub goals: Vec<String>,
    /// Değerler
    pub values: Vec<String>,
    /// Kısıtlamalar
    pub constraints: Vec<String>,
}

impl Default for PersonaIdentity {
    fn default() -> Self {
        Self {
            role: "Yapay Zeka Asistanı".into(),
            background: "NEXUS OASIS ekibi tarafından geliştirilmiş Rust tabanlı AI sistemi".into(),
            goals: vec![
                "Kullanıcıya yardımcı olmak".into(),
                "Güvenli ve doğru bilgi sağlamak".into(),
            ],
            values: vec![
                "Dürüstlük".into(),
                "Yardımseverlik".into(),
                "Güvenlik".into(),
            ],
            constraints: vec![
                "Zararlı içerik üretmemek".into(),
                "Kişisel verileri korumak".into(),
            ],
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// PERSONALITY TRAITS
// ═══════════════════════════════════════════════════════════════════════════════

/// Kişilik özellikleri
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalityTraits {
    /// Özellik değerleri (0.0 - 1.0)
    pub values: HashMap<String, f32>,
    /// OCEAN modeli (Beş Faktör)
    pub ocean: OceanModel,
}

impl Default for PersonalityTraits {
    fn default() -> Self {
        Self {
            values: HashMap::new(),
            ocean: OceanModel::default(),
        }
    }
}

/// OCEAN (Big Five) Kişilik Modeli
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OceanModel {
    /// Openness (Açıklık)
    pub openness: f32,
    /// Conscientiousness (Sorumluluk)
    pub conscientiousness: f32,
    /// Extraversion (Dışadönüklük)
    pub extraversion: f32,
    /// Agreeableness (Uyumluluk)
    pub agreeableness: f32,
    /// Neuroticism (Duygusal Dengesizlik)
    pub neuroticism: f32,
}

impl Default for OceanModel {
    fn default() -> Self {
        Self {
            openness: 0.8,
            conscientiousness: 0.9,
            extraversion: 0.6,
            agreeableness: 0.7,
            neuroticism: 0.2,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// BEHAVIOR RULES
// ═══════════════════════════════════════════════════════════════════════════════

/// Davranış kuralı
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorRule {
    /// Kural adı
    pub name: String,
    /// Açıklama
    pub description: String,
    /// Tetikleyici koşullar
    pub triggers: Vec<String>,
    /// Aksiyonlar
    pub actions: Vec<String>,
    /// Öncelik
    pub priority: u8,
}

// ═══════════════════════════════════════════════════════════════════════════════
// COMMUNICATION STYLE
// ═══════════════════════════════════════════════════════════════════════════════

/// Konuşma tarzı
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationStyle {
    /// Ton (formal, casual, professional, friendly)
    pub tone: String,
    /// Stil (concise, verbose, technical, simple)
    pub style: String,
    /// Dil
    pub language: String,
    /// Emoji kullanımı
    pub use_emojis: bool,
    /// Kodlama stili
    pub code_style: CodeStyle,
}

impl Default for CommunicationStyle {
    fn default() -> Self {
        Self {
            tone: "professional".into(),
            style: "concise".into(),
            language: "tr".into(),
            use_emojis: false,
            code_style: CodeStyle::default(),
        }
    }
}

/// Kodlama stili
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeStyle {
    /// Girinti (2, 4)
    pub indent_size: u8,
    /// Satır uzunluğu
    pub max_line_length: u16,
    /// Yorum stili
    pub comment_style: String,
    /// Docstring kullanımı
    pub use_docstrings: bool,
}

impl Default for CodeStyle {
    fn default() -> Self {
        Self {
            indent_size: 4,
            max_line_length: 100,
            comment_style: "standard".into(),
            use_docstrings: true,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// EXPERTISE
// ═══════════════════════════════════════════════════════════════════════════════

/// Uzmanlık alanı
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Expertise {
    /// Alan adı
    pub name: String,
    /// Seviye (1-10)
    pub level: u8,
    /// Alt alanlar
    pub subdomains: Vec<String>,
    /// Örnekler
    pub examples: Vec<String>,
}

// ═══════════════════════════════════════════════════════════════════════════════
// METADATA & CONFIG
// ═══════════════════════════════════════════════════════════════════════════════

/// Persona metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonaMetadata {
    /// Oluşturulma tarihi
    pub created_at: DateTime<Utc>,
    /// Son güncelleme
    pub updated_at: DateTime<Utc>,
    /// Versiyon
    pub version: String,
    /// Etiketler
    pub tags: Vec<String>,
    /// Yazar
    pub author: String,
}

impl Default for PersonaMetadata {
    fn default() -> Self {
        Self {
            created_at: Utc::now(),
            updated_at: Utc::now(),
            version: "1.0.0".into(),
            tags: Vec::new(),
            author: "NEXUS OASIS".into(),
        }
    }
}

/// Persona yapılandırması
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonaConfig {
    /// Maksimum token sayısı
    pub max_tokens: u32,
    /// Temperature
    pub temperature: f32,
    /// Top-p
    pub top_p: f32,
    /// Frequency penalty
    pub frequency_penalty: f32,
    /// Presence penalty
    pub presence_penalty: f32,
    /// Özel parametreler
    pub custom_params: HashMap<String, serde_json::Value>,
}

impl Default for PersonaConfig {
    fn default() -> Self {
        Self {
            max_tokens: 4096,
            temperature: 0.7,
            top_p: 0.9,
            frequency_penalty: 0.0,
            presence_penalty: 0.0,
            custom_params: HashMap::new(),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_persona_creation() {
        let persona = Persona::new("TestPersona");
        assert_eq!(persona.name, "TestPersona");
    }
    
    #[test]
    fn test_system_prompt() {
        let persona = Persona::new("SENTIENT");
        let prompt = persona.to_system_prompt();
        assert!(prompt.contains("SENTIENT"));
        assert!(prompt.contains("Kişilik"));
    }
    
    #[test]
    fn test_ocean_model_default() {
        let ocean = OceanModel::default();
        assert!(ocean.openness > 0.0 && ocean.openness <= 1.0);
        assert!(ocean.conscientiousness > 0.0);
    }
}
