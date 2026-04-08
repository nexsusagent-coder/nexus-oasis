//! ═══════════════════════════════════════════════════════════════════════════════
//!  SENTIENT TOOL WRAPPER: openharness-types
//! ═══════════════════════════════════════════════════════════════════════════════
//! 
//! Kaynak: openharness
//! Risk Seviyesi: high
//! 
//! ═──────────────────────────────────────────────────────────────────────────────

/// openharness_types - Asimile edilmiş araç
/// 
/// SENTIENT Sovereign güvenlik kurallarına tabidir.
#[derive(Debug, Clone)]
pub struct openharness_types {
    config: openharness_typesConfig,
}

#[derive(Debug, Clone)]
pub struct openharness_typesConfig {
    /// Kaynak repo
    pub source_repo: String,
    /// Dil
    pub language: String,
    /// Risk seviyesi
    pub risk_level: String,
}

impl Default for openharness_typesConfig {
    fn default() -> Self {
        Self {
            source_repo: "openharness".to_string(),
            language: "typescript".to_string(),
            risk_level: "high".to_string(),
        }
    }
}

impl openharness_types {
    /// Yeni wrapper oluştur
    pub fn new() -> Self {
        Self::with_config(openharness_typesConfig::default())
    }
    
    /// Yapılandırma ile oluştur
    pub fn with_config(config: openharness_typesConfig) -> Self {
        Self { config }
    }
    
    /// Aracın adını döndür
    pub fn name(&self) -> &str {
        "openharness-types"
    }
    
    /// Aracın açıklamasını döndür
    pub fn description(&self) -> &str {
        "Auto-generated skill from types"
    }
    
    /// Risk seviyesini döndür
    pub fn risk_level(&self) -> &str {
        &self.config.risk_level
    }
    
    /// Parametreleri döndür
    pub fn parameters(&self) -> Vec<(&str, &str, bool)> {
        vec![
            // Parametre yok
        ]
    }
}

impl Default for openharness_types {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_wrapper_creation() {
        let wrapper = openharness_types::new();
        assert_eq!(wrapper.name(), "openharness-types");
    }
    
    #[test]
    fn test_config_default() {
        let config = openharness_typesConfig::default();
        assert!(!config.source_repo.is_empty());
    }
}
