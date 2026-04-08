//! ═══════════════════════════════════════════════════════════════════════════════
//!  SENTIENT TOOL WRAPPER: openharness-loader
//! ═══════════════════════════════════════════════════════════════════════════════
//! 
//! Kaynak: openharness
//! Risk Seviyesi: critical
//! 
//! ═──────────────────────────────────────────────────────────────────────────────

/// openharness_loader - Asimile edilmiş araç
/// 
/// SENTIENT Sovereign güvenlik kurallarına tabidir.
#[derive(Debug, Clone)]
pub struct openharness_loader {
    config: openharness_loaderConfig,
}

#[derive(Debug, Clone)]
pub struct openharness_loaderConfig {
    /// Kaynak repo
    pub source_repo: String,
    /// Dil
    pub language: String,
    /// Risk seviyesi
    pub risk_level: String,
}

impl Default for openharness_loaderConfig {
    fn default() -> Self {
        Self {
            source_repo: "openharness".to_string(),
            language: "python".to_string(),
            risk_level: "critical".to_string(),
        }
    }
}

impl openharness_loader {
    /// Yeni wrapper oluştur
    pub fn new() -> Self {
        Self::with_config(openharness_loaderConfig::default())
    }
    
    /// Yapılandırma ile oluştur
    pub fn with_config(config: openharness_loaderConfig) -> Self {
        Self { config }
    }
    
    /// Aracın adını döndür
    pub fn name(&self) -> &str {
        "openharness-loader"
    }
    
    /// Aracın açıklamasını döndür
    pub fn description(&self) -> &str {
        "Auto-generated skill from loader"
    }
    
    /// Risk seviyesini döndür
    pub fn risk_level(&self) -> &str {
        &self.config.risk_level
    }
    
    /// Parametreleri döndür
    pub fn parameters(&self) -> Vec<(&str, &str, bool)> {
        vec![
            ("cwd", "any", true),
            ("plugin_dir", "any", true),
            ("cwd", "any", true),
            ("settings", "any", true),
            ("cwd", "any", true),
            ("path", "any", true),
            ("enabled_plugins", "any", true),
            ("bool]", "any", true),
            ("path", "any", true),
            ("path", "any", true),
            ("path", "any", true),
            ("plugin_root", "any", true),
            ("path", "any", true),
        ]
    }
}

impl Default for openharness_loader {
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
        let wrapper = openharness_loader::new();
        assert_eq!(wrapper.name(), "openharness-loader");
    }
    
    #[test]
    fn test_config_default() {
        let config = openharness_loaderConfig::default();
        assert!(!config.source_repo.is_empty());
    }
}
