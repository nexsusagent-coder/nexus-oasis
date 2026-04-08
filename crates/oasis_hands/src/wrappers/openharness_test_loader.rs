//! ═══════════════════════════════════════════════════════════════════════════════
//!  SENTIENT TOOL WRAPPER: openharness-test-loader
//! ═══════════════════════════════════════════════════════════════════════════════
//! 
//! Kaynak: openharness
//! Risk Seviyesi: medium
//! 
//! ═──────────────────────────────────────────────────────────────────────────────

/// openharness_test_loader - Asimile edilmiş araç
/// 
/// SENTIENT Sovereign güvenlik kurallarına tabidir.
#[derive(Debug, Clone)]
pub struct openharness_test_loader {
    config: openharness_test_loaderConfig,
}

#[derive(Debug, Clone)]
pub struct openharness_test_loaderConfig {
    /// Kaynak repo
    pub source_repo: String,
    /// Dil
    pub language: String,
    /// Risk seviyesi
    pub risk_level: String,
}

impl Default for openharness_test_loaderConfig {
    fn default() -> Self {
        Self {
            source_repo: "openharness".to_string(),
            language: "python".to_string(),
            risk_level: "medium".to_string(),
        }
    }
}

impl openharness_test_loader {
    /// Yeni wrapper oluştur
    pub fn new() -> Self {
        Self::with_config(openharness_test_loaderConfig::default())
    }
    
    /// Yapılandırma ile oluştur
    pub fn with_config(config: openharness_test_loaderConfig) -> Self {
        Self { config }
    }
    
    /// Aracın adını döndür
    pub fn name(&self) -> &str {
        "openharness-test-loader"
    }
    
    /// Aracın açıklamasını döndür
    pub fn description(&self) -> &str {
        "Auto-generated skill from test-loader"
    }
    
    /// Risk seviyesini döndür
    pub fn risk_level(&self) -> &str {
        &self.config.risk_level
    }
    
    /// Parametreleri döndür
    pub fn parameters(&self) -> Vec<(&str, &str, bool)> {
        vec![
            ("tmp_path", "any", true),
            ("monkeypatch", "any", true),
            ("tmp_path", "any", true),
            ("monkeypatch", "any", true),
        ]
    }
}

impl Default for openharness_test_loader {
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
        let wrapper = openharness_test_loader::new();
        assert_eq!(wrapper.name(), "openharness-test-loader");
    }
    
    #[test]
    fn test_config_default() {
        let config = openharness_test_loaderConfig::default();
        assert!(!config.source_repo.is_empty());
    }
}
