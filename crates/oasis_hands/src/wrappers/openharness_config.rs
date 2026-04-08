//! ═══════════════════════════════════════════════════════════════════════════════
//!  SENTIENT TOOL WRAPPER: openharness-config
//! ═══════════════════════════════════════════════════════════════════════════════
//! 
//! Kaynak: openharness
//! Risk Seviyesi: low
//! 
//! ═──────────────────────────────────────────────────────────────────────────────

/// openharness_config - Asimile edilmiş araç
/// 
/// SENTIENT Sovereign güvenlik kurallarına tabidir.
#[derive(Debug, Clone)]
pub struct openharness_config {
    config: openharness_configConfig,
}

#[derive(Debug, Clone)]
pub struct openharness_configConfig {
    /// Kaynak repo
    pub source_repo: String,
    /// Dil
    pub language: String,
    /// Risk seviyesi
    pub risk_level: String,
}

impl Default for openharness_configConfig {
    fn default() -> Self {
        Self {
            source_repo: "openharness".to_string(),
            language: "python".to_string(),
            risk_level: "low".to_string(),
        }
    }
}

impl openharness_config {
    /// Yeni wrapper oluştur
    pub fn new() -> Self {
        Self::with_config(openharness_configConfig::default())
    }
    
    /// Yapılandırma ile oluştur
    pub fn with_config(config: openharness_configConfig) -> Self {
        Self { config }
    }
    
    /// Aracın adını döndür
    pub fn name(&self) -> &str {
        "openharness-config"
    }
    
    /// Aracın açıklamasını döndür
    pub fn description(&self) -> &str {
        "Auto-generated skill from config"
    }
    
    /// Risk seviyesini döndür
    pub fn risk_level(&self) -> &str {
        &self.config.risk_level
    }
    
    /// Parametreleri döndür
    pub fn parameters(&self) -> Vec<(&str, &str, bool)> {
        vec![
            ("settings", "any", true),
            ("plugins", "any", true),
        ]
    }
}

impl Default for openharness_config {
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
        let wrapper = openharness_config::new();
        assert_eq!(wrapper.name(), "openharness-config");
    }
    
    #[test]
    fn test_config_default() {
        let config = openharness_configConfig::default();
        assert!(!config.source_repo.is_empty());
    }
}
