//! ═══════════════════════════════════════════════════════════════════════════════
//!  SENTIENT TOOL WRAPPER: openharness-cli
//! ═══════════════════════════════════════════════════════════════════════════════
//! 
//! Kaynak: openharness
//! Risk Seviyesi: critical
//! 
//! ═──────────────────────────────────────────────────────────────────────────────

/// openharness_cli - Asimile edilmiş araç
/// 
/// SENTIENT Sovereign güvenlik kurallarına tabidir.
#[derive(Debug, Clone)]
pub struct openharness_cli {
    config: openharness_cliConfig,
}

#[derive(Debug, Clone)]
pub struct openharness_cliConfig {
    /// Kaynak repo
    pub source_repo: String,
    /// Dil
    pub language: String,
    /// Risk seviyesi
    pub risk_level: String,
}

impl Default for openharness_cliConfig {
    fn default() -> Self {
        Self {
            source_repo: "openharness".to_string(),
            language: "python".to_string(),
            risk_level: "critical".to_string(),
        }
    }
}

impl openharness_cli {
    /// Yeni wrapper oluştur
    pub fn new() -> Self {
        Self::with_config(openharness_cliConfig::default())
    }
    
    /// Yapılandırma ile oluştur
    pub fn with_config(config: openharness_cliConfig) -> Self {
        Self { config }
    }
    
    /// Aracın adını döndür
    pub fn name(&self) -> &str {
        "openharness-cli"
    }
    
    /// Aracın açıklamasını döndür
    pub fn description(&self) -> &str {
        "---------------------------------------------------------------------------"
    }
    
    /// Risk seviyesini döndür
    pub fn risk_level(&self) -> &str {
        &self.config.risk_level
    }
    
    /// Parametreleri döndür
    pub fn parameters(&self) -> Vec<(&str, &str, bool)> {
        vec![
            ("value", "any", true),
            ("message", "any", true),
            ("*", "any", false),
            ("default", "any", true),
            ("message", "any", true),
            ("profile", "any", true),
            ("info", "any", true),
            ("object]", "any", true),
            ("info", "any", true),
            ("object]", "any", true),
            ("name", "any", true),
            ("auth_source", "any", true),
            ("label", "any", true),
            ("manager", "any", true),
            ("manager", "any", true),
            ("target", "any", true),
            ("manager", "any", true),
            ("profile_name", "any", true),
            ("provider", "any", true),
            ("provider", "any", true),
            ("provider", "any", true),
        ]
    }
}

impl Default for openharness_cli {
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
        let wrapper = openharness_cli::new();
        assert_eq!(wrapper.name(), "openharness-cli");
    }
    
    #[test]
    fn test_config_default() {
        let config = openharness_cliConfig::default();
        assert!(!config.source_repo.is_empty());
    }
}
