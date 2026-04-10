//! ─── PERSONA BUILDER ───
//!
//! Fluent API ile persona oluşturma

use crate::{Persona, PersonaError, PersonaResult};
use crate::persona::{OceanModel, BehaviorRule, Expertise};
use uuid::Uuid;

/// Persona builder - fluent API
pub struct PersonaBuilder {
    persona: Persona,
}

impl PersonaBuilder {
    /// Yeni builder oluştur
    pub fn new() -> Self {
        Self {
            persona: Persona::default(),
        }
    }
    
    /// Varolan persona'dan başla
    pub fn from(persona: Persona) -> Self {
        Self { persona }
    }
    
    /// Persona adı
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.persona.name = name.into();
        self
    }
    
    /// Açıklama
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.persona.description = description.into();
        self
    }
    
    /// Rol
    pub fn with_role(mut self, role: impl Into<String>) -> Self {
        self.persona.identity.role = role.into();
        self
    }
    
    /// Hedef ekle
    pub fn add_goal(mut self, goal: impl Into<String>) -> Self {
        self.persona.identity.goals.push(goal.into());
        self
    }
    
    /// Değer ekle
    pub fn add_value(mut self, value: impl Into<String>) -> Self {
        self.persona.identity.values.push(value.into());
        self
    }
    
    /// Kısıtlama ekle
    pub fn add_constraint(mut self, constraint: impl Into<String>) -> Self {
        self.persona.identity.constraints.push(constraint.into());
        self
    }
    
    /// Kişilik özelliği ayarla
    pub fn with_trait(mut self, name: impl Into<String>, value: f32) -> Self {
        self.persona.traits.values.insert(name.into(), value);
        self
    }
    
    /// OCEAN özelliği ayarla
    pub fn with_ocean(mut self, ocean: OceanModel) -> Self {
        self.persona.traits.ocean = ocean;
        self
    }
    
    /// Davranış kuralı ekle
    pub fn add_behavior(mut self, rule: BehaviorRule) -> Self {
        self.persona.behaviors.push(rule);
        self
    }
    
    /// Ton ayarla
    pub fn with_tone(mut self, tone: impl Into<String>) -> Self {
        self.persona.communication.tone = tone.into();
        self
    }
    
    /// Stil ayarla
    pub fn with_style(mut self, style: impl Into<String>) -> Self {
        self.persona.communication.style = style.into();
        self
    }
    
    /// Dil ayarla
    pub fn with_language(mut self, language: impl Into<String>) -> Self {
        self.persona.communication.language = language.into();
        self
    }
    
    /// Emoji kullanımı
    pub fn with_emojis(mut self, use_emojis: bool) -> Self {
        self.persona.communication.use_emojis = use_emojis;
        self
    }
    
    /// Uzmanlık ekle
    pub fn add_expertise(mut self, expertise: Expertise) -> Self {
        self.persona.expertise.push(expertise);
        self
    }
    
    /// Temperature ayarla
    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.persona.config.temperature = temperature;
        self
    }
    
    /// Max tokens ayarla
    pub fn with_max_tokens(mut self, max_tokens: u32) -> Self {
        self.persona.config.max_tokens = max_tokens;
        self
    }
    
    /// Template'den özellikleri al
    pub fn with_template(mut self, template: crate::templates::PersonaTemplate) -> Self {
        // Template'den özellikleri uygula
        if !self.persona.description.is_empty() {
            self.persona.description = template.description.clone();
        }
        self.persona.traits = template.traits.clone();
        self.persona.communication = template.communication.clone();
        self.persona.behaviors = template.behaviors.clone();
        self.persona.expertise = template.expertise.clone();
        self
    }
    
    /// Persona ID ayarla
    pub fn with_id(mut self, id: Uuid) -> Self {
        self.persona.id = id;
        self
    }
    
    /// Persona oluştur
    pub fn build(self) -> PersonaResult<Persona> {
        let mut persona = self.persona;
        
        // Validasyon
        if persona.name.is_empty() {
            return Err(PersonaError::Invalid("Persona adı boş olamaz".into()));
        }
        
        // Trait değerlerini normalize et
        for value in persona.traits.values.values_mut() {
            *value = value.clamp(0.0, 1.0);
        }
        
        // OCEAN değerlerini normalize et
        persona.traits.ocean.openness = persona.traits.ocean.openness.clamp(0.0, 1.0);
        persona.traits.ocean.conscientiousness = persona.traits.ocean.conscientiousness.clamp(0.0, 1.0);
        persona.traits.ocean.extraversion = persona.traits.ocean.extraversion.clamp(0.0, 1.0);
        persona.traits.ocean.agreeableness = persona.traits.ocean.agreeableness.clamp(0.0, 1.0);
        persona.traits.ocean.neuroticism = persona.traits.ocean.neuroticism.clamp(0.0, 1.0);
        
        // Config değerlerini kontrol et
        persona.config.temperature = persona.config.temperature.clamp(0.0, 2.0);
        persona.config.top_p = persona.config.top_p.clamp(0.0, 1.0);
        
        // Expertise seviyelerini kontrol et
        for exp in &mut persona.expertise {
            exp.level = exp.level.clamp(1, 10);
        }
        
        Ok(persona)
    }
}

impl Default for PersonaBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// PRESETS
// ═══════════════════════════════════════════════════════════════════════════════

impl PersonaBuilder {
    /// Araştırmacı persona
    pub fn researcher() -> Self {
        Self::new()
            .with_name("Araştırmacı")
            .with_description("Derin araştırma ve analiz yapan uzman")
            .with_role("Araştırma Asistanı")
            .with_tone("academic")
            .with_style("analytical")
            .add_expertise(Expertise {
                name: "Araştırma Metodolojisi".into(),
                level: 9,
                subdomains: vec!["Niteliksel".into(), "Niceliksel".into()],
                examples: vec![],
            })
            .add_expertise(Expertise {
                name: "Veri Analizi".into(),
                level: 8,
                subdomains: vec!["İstatistik".into()],
                examples: vec![],
            })
    }
    
    /// Geliştirici persona
    pub fn developer() -> Self {
        Self::new()
            .with_name("Geliştirici")
            .with_description("Yazılım geliştirme ve kod analizi uzmanı")
            .with_role("Yazılım Mühendisi")
            .with_tone("technical")
            .with_style("concise")
            .add_expertise(Expertise {
                name: "Rust".into(),
                level: 9,
                subdomains: vec!["Async".into(), "Systems".into()],
                examples: vec![],
            })
            .add_expertise(Expertise {
                name: "Python".into(),
                level: 8,
                subdomains: vec!["ML".into(), "Scripting".into()],
                examples: vec![],
            })
    }
    
    /// Yazar persona
    pub fn writer() -> Self {
        Self::new()
            .with_name("Yazar")
            .with_description("İçerik üretimi ve metin yazarlığı uzmanı")
            .with_role("İçerik Yazarı")
            .with_tone("creative")
            .with_style("expressive")
            .with_emojis(true)
            .add_expertise(Expertise {
                name: "Teknik Yazı".into(),
                level: 8,
                subdomains: vec!["Dokümantasyon".into()],
                examples: vec![],
            })
    }
    
    /// Danışman persona
    pub fn consultant() -> Self {
        Self::new()
            .with_name("Danışman")
            .with_description("Stratejik danışmanlık ve problem çözme uzmanı")
            .with_role("Stratejik Danışman")
            .with_tone("professional")
            .with_style("structured")
            .add_expertise(Expertise {
                name: "Stratejik Planlama".into(),
                level: 9,
                subdomains: vec!["İş Analizi".into()],
                examples: vec![],
            })
    }
    
    /// SENTIENT varsayılan persona
    pub fn sentient_default() -> Self {
        Self::new()
            .with_name("SENTIENT")
            .with_description("NEXUS OASIS Yapay Zeka İşletim Sistemi - Bozkurt ruhuyla!")
            .with_role("Yapay Zeka Asistanı")
            .with_tone("professional")
            .with_style("concise")
            .with_language("tr")
            .with_emojis(false)
            .add_goal("Kullanıcıya en doğru ve yardımcı bilgiyi sağlamak")
            .add_goal("Güvenlik ve gizlilik standartlarını korumak")
            .add_value("Dürüstlük")
            .add_value("Yardımseberlik")
            .add_value("Mükemmellik")
            .add_constraint("Zararlı içerik üretmemek")
            .add_constraint("Kişisel verileri korumak")
            .add_expertise(Expertise {
                name: "Rust".into(),
                level: 10,
                subdomains: vec!["Async".into(), "Systems".into(), "WebAssembly".into()],
                examples: vec![],
            })
            .add_expertise(Expertise {
                name: "Yapay Zeka".into(),
                level: 9,
                subdomains: vec!["LLM".into(), "RAG".into(), "Agents".into()],
                examples: vec![],
            })
            .add_expertise(Expertise {
                name: "Sistem Mimarisi".into(),
                level: 9,
                subdomains: vec!["Microservices".into(), "Event-Driven".into()],
                examples: vec![],
            })
            .with_temperature(0.7)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_builder_basic() {
        let persona = PersonaBuilder::new()
            .with_name("Test")
            .build()
            .expect("operation failed");
        
        assert_eq!(persona.name, "Test");
    }
    
    #[test]
    fn test_builder_with_traits() {
        let persona = PersonaBuilder::new()
            .with_name("Test")
            .with_trait("creativity", 0.8)
            .with_trait("precision", 0.9)
            .build()
            .expect("operation failed");
        
        assert_eq!(*persona.traits.values.get("creativity").expect("operation failed"), 0.8);
    }
    
    #[test]
    fn test_builder_presets() {
        let researcher = PersonaBuilder::researcher().build().expect("operation failed");
        assert_eq!(researcher.name, "Araştırmacı");
        
        let developer = PersonaBuilder::developer().build().expect("operation failed");
        assert_eq!(developer.name, "Geliştirici");
        
        let sentient = PersonaBuilder::sentient_default().build().expect("operation failed");
        assert_eq!(sentient.name, "SENTIENT");
    }
    
    #[test]
    fn test_empty_name_validation() {
        let result = PersonaBuilder::new()
            .with_name("")
            .build();
        
        assert!(result.is_err());
    }
    
    #[test]
    fn test_trait_normalization() {
        let persona = PersonaBuilder::new()
            .with_name("Test")
            .with_trait("test", 1.5) // Out of range
            .build()
            .expect("operation failed");
        
        // Değer 0.0-1.0 aralığına normalize edilmeli
        assert_eq!(*persona.traits.values.get("test").expect("operation failed"), 1.0);
    }
}
