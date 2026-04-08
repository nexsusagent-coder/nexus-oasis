//! ═══════════════════════════════════════════════════════════════════════════════
//!  SENTIENT TOOL WRAPPER: openharness-context
//! ═══════════════════════════════════════════════════════════════════════════════
//! 
//! Kaynak: openharness
//! Risk Seviyesi: low
//! 
//! ═──────────────────────────────────────────────────────────────────────────────

/// openharness_context - Asimile edilmiş araç
/// 
/// SENTIENT Sovereign güvenlik kurallarına tabidir.
#[derive(Debug, Clone)]
pub struct openharness_context {
    config: openharness_contextConfig,
}

#[derive(Debug, Clone)]
pub struct openharness_contextConfig {
    /// Kaynak repo
    pub source_repo: String,
    /// Dil
    pub language: String,
    /// Risk seviyesi
    pub risk_level: String,
}

impl Default for openharness_contextConfig {
    fn default() -> Self {
        Self {
            source_repo: "openharness".to_string(),
            language: "python".to_string(),
            risk_level: "low".to_string(),
        }
    }
}

impl openharness_context {
    /// Yeni wrapper oluştur
    pub fn new() -> Self {
        Self::with_config(openharness_contextConfig::default())
    }
    
    /// Yapılandırma ile oluştur
    pub fn with_config(config: openharness_contextConfig) -> Self {
        Self { config }
    }
    
    /// Aracın adını döndür
    pub fn name(&self) -> &str {
        "openharness-context"
    }
    
    /// Aracın açıklamasını döndür
    pub fn description(&self) -> &str {
        "Auto-generated skill from context"
    }
    
    /// Risk seviyesini döndür
    pub fn risk_level(&self) -> &str {
        &self.config.risk_level
    }
    
    /// Parametreleri döndür
    pub fn parameters(&self) -> Vec<(&str, &str, bool)> {
        vec![
            ("cwd", "any", true),
        ]
    }
}

impl Default for openharness_context {
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
        let wrapper = openharness_context::new();
        assert_eq!(wrapper.name(), "openharness-context");
    }
    
    #[test]
    fn test_config_default() {
        let config = openharness_contextConfig::default();
        assert!(!config.source_repo.is_empty());
    }
}
