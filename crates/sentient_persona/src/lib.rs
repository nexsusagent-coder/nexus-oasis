//! ─── A1 PERSONA BUILDER ───
//!
//! SENTIENT'nın kişilik ve davranış sistemi.
//! Her ajan için özelleştirilebilir persona tanımları.
//!
//! Özellikler:
//! - Dinamik persona yükleme (YAML/JSON)
//! - Persona kalıpları ve miras alma
//! - Bağlamsal davranış ayarları
//! - Dil ve ton profilleri

pub mod persona;
pub mod builder;
pub mod traits;
pub mod loader;
pub mod templates;

pub use persona::{Persona, PersonaConfig, PersonaIdentity, PersonalityTraits};
pub use builder::PersonaBuilder;
pub use traits::{Trait, TraitValue, BehaviorPattern};
pub use loader::{PersonaLoader, PersonaFormat};
pub use templates::{PersonaTemplate, TemplateLibrary};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use chrono::{DateTime, Utc};

// ═══════════════════════════════════════════════════════════════════════════════
// PERSONA ERROR
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, thiserror::Error)]
pub enum PersonaError {
    #[error("Persona bulunamadı: {0}")]
    NotFound(String),
    
    #[error("Persona geçersiz: {0}")]
    Invalid(String),
    
    #[error("Persona yüklenemedi: {0}")]
    LoadError(String),
    
    #[error("Persona kaydedilemedi: {0}")]
    SaveError(String),
    
    #[error("Template hatası: {0}")]
    TemplateError(String),
}

pub type PersonaResult<T> = Result<T, PersonaError>;

// ═══════════════════════════════════════════════════════════════════════════════
// PERSONA REGISTRY
// ═══════════════════════════════════════════════════════════════════════════════

/// Persona kayıt merkezi - tüm persona'ları yönetir
pub struct PersonaRegistry {
    personas: Arc<RwLock<HashMap<Uuid, Persona>>>,
    active_persona: Arc<RwLock<Option<Uuid>>>,
    templates: TemplateLibrary,
}

impl PersonaRegistry {
    /// Yeni registry oluştur
    pub fn new() -> Self {
        Self {
            personas: Arc::new(RwLock::new(HashMap::new())),
            active_persona: Arc::new(RwLock::new(None)),
            templates: TemplateLibrary::default(),
        }
    }
    
    /// Persona ekle
    pub async fn register(&self, persona: Persona) -> PersonaResult<Uuid> {
        let id = persona.id;
        self.personas.write().await.insert(id, persona);
        Ok(id)
    }
    
    /// Persona getir
    pub async fn get(&self, id: &Uuid) -> PersonaResult<Persona> {
        self.personas.read().await.get(id).cloned()
            .ok_or_else(|| PersonaError::NotFound(id.to_string()))
    }
    
    /// Aktif persona'yı ayarla
    pub async fn set_active(&self, id: Uuid) -> PersonaResult<()> {
        if !self.personas.read().await.contains_key(&id) {
            return Err(PersonaError::NotFound(id.to_string()));
        }
        *self.active_persona.write().await = Some(id);
        Ok(())
    }
    
    /// Aktif persona'yı getir
    pub async fn get_active(&self) -> Option<Persona> {
        let active_id = self.active_persona.read().await;
        if let Some(id) = *active_id {
            self.personas.read().await.get(&id).cloned()
        } else {
            None
        }
    }
    
    /// Tüm persona'ları listele
    pub async fn list(&self) -> Vec<Persona> {
        self.personas.read().await.values().cloned().collect()
    }
    
    /// Template'den persona oluştur
    pub async fn from_template(&self, template_name: &str, name: &str) -> PersonaResult<Persona> {
        let template = self.templates.get(template_name)?;
        let persona = PersonaBuilder::new()
            .with_name(name)
            .with_template(template)
            .build()?;
        Ok(persona)
    }
}

impl Default for PersonaRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// PERSONA CONTEXT
// ═══════════════════════════════════════════════════════════════════════════════

/// Persona bağlamı - runtime davranış ayarları
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonaContext {
    /// Aktif persona
    pub persona_id: Option<Uuid>,
    /// Kullanıcı tercihleri
    pub user_preferences: HashMap<String, String>,
    /// Oturum bilgileri
    pub session_context: HashMap<String, serde_json::Value>,
    /// Son güncelleme
    pub last_updated: DateTime<Utc>,
}

impl Default for PersonaContext {
    fn default() -> Self {
        Self {
            persona_id: None,
            user_preferences: HashMap::new(),
            session_context: HashMap::new(),
            last_updated: Utc::now(),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_persona_registry() {
        let registry = PersonaRegistry::new();
        let persona = Persona::default();
        let id = persona.id;
        
        registry.register(persona).await.expect("operation failed");
        let retrieved = registry.get(&id).await.expect("operation failed");
        
        assert_eq!(retrieved.id, id);
    }
    
    #[tokio::test]
    async fn test_active_persona() {
        let registry = PersonaRegistry::new();
        let persona = Persona::default();
        let id = persona.id;
        
        registry.register(persona).await.expect("operation failed");
        registry.set_active(id).await.expect("operation failed");
        
        let active = registry.get_active().await;
        assert!(active.is_some());
    }
}
