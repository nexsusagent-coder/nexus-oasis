//! ═══════════════════════════════════════════════════════════════════════════════
//!  SENTIENT TOOL WRAPPER: openharness-output
//! ═══════════════════════════════════════════════════════════════════════════════
//! 
//! Kaynak: openharness
//! Risk Seviyesi: critical
//! 
//! ═──────────────────────────────────────────────────────────────────────────────

/// openharness_output - Asimile edilmiş araç
/// 
/// SENTIENT Sovereign güvenlik kurallarına tabidir.
#[derive(Debug, Clone)]
pub struct openharness_output {
    config: openharness_outputConfig,
}

#[derive(Debug, Clone)]
pub struct openharness_outputConfig {
    /// Kaynak repo
    pub source_repo: String,
    /// Dil
    pub language: String,
    /// Risk seviyesi
    pub risk_level: String,
}

impl Default for openharness_outputConfig {
    fn default() -> Self {
        Self {
            source_repo: "openharness".to_string(),
            language: "python".to_string(),
            risk_level: "critical".to_string(),
        }
    }
}

impl openharness_output {
    /// Yeni wrapper oluştur
    pub fn new() -> Self {
        Self::with_config(openharness_outputConfig::default())
    }
    
    /// Yapılandırma ile oluştur
    pub fn with_config(config: openharness_outputConfig) -> Self {
        Self { config }
    }
    
    /// Aracın adını döndür
    pub fn name(&self) -> &str {
        "openharness-output"
    }
    
    /// Aracın açıklamasını döndür
    pub fn description(&self) -> &str {
        "Auto-generated skill from output"
    }
    
    /// Risk seviyesini döndür
    pub fn risk_level(&self) -> &str {
        &self.config.risk_level
    }
    
    /// Parametreleri döndür
    pub fn parameters(&self) -> Vec<(&str, &str, bool)> {
        vec![
            ("text", "any", true),
            ("tool_name", "any", true),
            ("tool_input", "any", true),
            ("ext", "any", true),
            ("n", "any", true),
        ]
    }
}

impl Default for openharness_output {
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
        let wrapper = openharness_output::new();
        assert_eq!(wrapper.name(), "openharness-output");
    }
    
    #[test]
    fn test_config_default() {
        let config = openharness_outputConfig::default();
        assert!(!config.source_repo.is_empty());
    }
}
