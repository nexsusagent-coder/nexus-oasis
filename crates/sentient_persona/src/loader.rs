//! ─── PERSONA LOADER ───
//!
//! YAML/JSON persona dosyalarını yükleme

use crate::{Persona, PersonaError, PersonaResult};
use std::path::Path;
use std::collections::HashMap;

/// Persona formatı
#[derive(Debug, Clone, Copy)]
pub enum PersonaFormat {
    Yaml,
    Json,
}

/// Persona yükleyici
pub struct PersonaLoader {
    base_path: std::path::PathBuf,
}

impl PersonaLoader {
    /// Yeni yükleyici oluştur
    pub fn new(base_path: impl AsRef<Path>) -> Self {
        Self {
            base_path: base_path.as_ref().to_path_buf(),
        }
    }
    
    /// Persona dosyasını yükle
    pub fn load(&self, name: &str) -> PersonaResult<Persona> {
        // Önce YAML dene
        let yaml_path = self.base_path.join(format!("{}.yaml", name));
        if yaml_path.exists() {
            return self.load_from_path(&yaml_path, PersonaFormat::Yaml);
        }
        
        // Sonra JSON dene
        let json_path = self.base_path.join(format!("{}.json", name));
        if json_path.exists() {
            return self.load_from_path(&json_path, PersonaFormat::Json);
        }
        
        Err(PersonaError::NotFound(format!("Persona '{}' bulunamadı", name)))
    }
    
    /// Dosyadan yükle
    pub fn load_from_path(&self, path: &Path, format: PersonaFormat) -> PersonaResult<Persona> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| PersonaError::LoadError(e.to_string()))?;
        
        match format {
            PersonaFormat::Yaml => {
                serde_yaml::from_str(&content)
                    .map_err(|e| PersonaError::LoadError(e.to_string()))
            }
            PersonaFormat::Json => {
                serde_json::from_str(&content)
                    .map_err(|e| PersonaError::LoadError(e.to_string()))
            }
        }
    }
    
    /// Persona kaydet
    pub fn save(&self, persona: &Persona, format: PersonaFormat) -> PersonaResult<()> {
        let (path, content) = match format {
            PersonaFormat::Yaml => {
                let path = self.base_path.join(format!("{}.yaml", persona.name));
                let content = serde_yaml::to_string(persona)
                    .map_err(|e| PersonaError::SaveError(e.to_string()))?;
                (path, content)
            }
            PersonaFormat::Json => {
                let path = self.base_path.join(format!("{}.json", persona.name));
                let content = serde_json::to_string_pretty(persona)
                    .map_err(|e| PersonaError::SaveError(e.to_string()))?;
                (path, content)
            }
        };
        
        std::fs::write(&path, content)
            .map_err(|e| PersonaError::SaveError(e.to_string()))?;
        
        Ok(())
    }
    
    /// Tüm persona'ları listele
    pub fn list_available(&self) -> PersonaResult<Vec<String>> {
        let mut personas = Vec::new();
        
        if !self.base_path.exists() {
            return Ok(personas);
        }
        
        for entry in std::fs::read_dir(&self.base_path)
            .map_err(|e| PersonaError::LoadError(e.to_string()))?
        {
            let entry = entry.map_err(|e| PersonaError::LoadError(e.to_string()))?;
            let path = entry.path();
            
            if let Some(ext) = path.extension() {
                if ext == "yaml" || ext == "json" {
                    if let Some(stem) = path.file_stem() {
                        personas.push(stem.to_string_lossy().to_string());
                    }
                }
            }
        }
        
        Ok(personas)
    }
}

impl Default for PersonaLoader {
    fn default() -> Self {
        Self::new("personas")
    }
}
