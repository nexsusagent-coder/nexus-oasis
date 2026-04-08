//! ═══════════════════════════════════════════════════════════════════════════════
//!  SENTIENT TOOL WRAPPER: openharness-test-prompts
//! ═══════════════════════════════════════════════════════════════════════════════
//! 
//! Kaynak: openharness
//! Risk Seviyesi: medium
//! 
//! ═──────────────────────────────────────────────────────────────────────────────

/// openharness_test_prompts - Asimile edilmiş araç
/// 
/// SENTIENT Sovereign güvenlik kurallarına tabidir.
#[derive(Debug, Clone)]
pub struct openharness_test_prompts {
    config: openharness_test_promptsConfig,
}

#[derive(Debug, Clone)]
pub struct openharness_test_promptsConfig {
    /// Kaynak repo
    pub source_repo: String,
    /// Dil
    pub language: String,
    /// Risk seviyesi
    pub risk_level: String,
}

impl Default for openharness_test_promptsConfig {
    fn default() -> Self {
        Self {
            source_repo: "openharness".to_string(),
            language: "python".to_string(),
            risk_level: "medium".to_string(),
        }
    }
}

impl openharness_test_prompts {
    /// Yeni wrapper oluştur
    pub fn new() -> Self {
        Self::with_config(openharness_test_promptsConfig::default())
    }
    
    /// Yapılandırma ile oluştur
    pub fn with_config(config: openharness_test_promptsConfig) -> Self {
        Self { config }
    }
    
    /// Aracın adını döndür
    pub fn name(&self) -> &str {
        "openharness-test-prompts"
    }
    
    /// Aracın açıklamasını döndür
    pub fn description(&self) -> &str {
        "Auto-generated skill from test-prompts"
    }
    
    /// Risk seviyesini döndür
    pub fn risk_level(&self) -> &str {
        &self.config.risk_level
    }
    
    /// Parametreleri döndür
    pub fn parameters(&self) -> Vec<(&str, &str, bool)> {
        vec![
            ("tmp_path", "any", true),
        ]
    }
}

impl Default for openharness_test_prompts {
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
        let wrapper = openharness_test_prompts::new();
        assert_eq!(wrapper.name(), "openharness-test-prompts");
    }
    
    #[test]
    fn test_config_default() {
        let config = openharness_test_promptsConfig::default();
        assert!(!config.source_repo.is_empty());
    }
}
