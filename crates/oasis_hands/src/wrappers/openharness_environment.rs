//! ═══════════════════════════════════════════════════════════════════════════════
//!  SENTIENT TOOL WRAPPER: openharness-environment
//! ═══════════════════════════════════════════════════════════════════════════════
//! 
//! Kaynak: openharness
//! Risk Seviyesi: critical
//! 
//! ═──────────────────────────────────────────────────────────────────────────────

/// openharness_environment - Asimile edilmiş araç
/// 
/// SENTIENT Sovereign güvenlik kurallarına tabidir.
#[derive(Debug, Clone)]
pub struct openharness_environment {
    config: openharness_environmentConfig,
}

#[derive(Debug, Clone)]
pub struct openharness_environmentConfig {
    /// Kaynak repo
    pub source_repo: String,
    /// Dil
    pub language: String,
    /// Risk seviyesi
    pub risk_level: String,
}

impl Default for openharness_environmentConfig {
    fn default() -> Self {
        Self {
            source_repo: "openharness".to_string(),
            language: "python".to_string(),
            risk_level: "critical".to_string(),
        }
    }
}

impl openharness_environment {
    /// Yeni wrapper oluştur
    pub fn new() -> Self {
        Self::with_config(openharness_environmentConfig::default())
    }
    
    /// Yapılandırma ile oluştur
    pub fn with_config(config: openharness_environmentConfig) -> Self {
        Self { config }
    }
    
    /// Aracın adını döndür
    pub fn name(&self) -> &str {
        "openharness-environment"
    }
    
    /// Aracın açıklamasını döndür
    pub fn description(&self) -> &str {
        "Auto-generated skill from environment"
    }
    
    /// Risk seviyesini döndür
    pub fn risk_level(&self) -> &str {
        &self.config.risk_level
    }
    
    /// Parametreleri döndür
    pub fn parameters(&self) -> Vec<(&str, &str, bool)> {
        vec![
            ("cwd", "any", true),
            ("cwd", "any", true),
        ]
    }
}

impl Default for openharness_environment {
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
        let wrapper = openharness_environment::new();
        assert_eq!(wrapper.name(), "openharness-environment");
    }
    
    #[test]
    fn test_config_default() {
        let config = openharness_environmentConfig::default();
        assert!(!config.source_repo.is_empty());
    }
}
