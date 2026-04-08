//! ═══════════════════════════════════════════════════════════════════════════════
//!  SENTIENT TOOL WRAPPER: openharness-agent-definitions
//! ═══════════════════════════════════════════════════════════════════════════════
//! 
//! Kaynak: openharness
//! Risk Seviyesi: critical
//! 
//! ═──────────────────────────────────────────────────────────────────────────────

/// openharness_agent_definitions - Asimile edilmiş araç
/// 
/// SENTIENT Sovereign güvenlik kurallarına tabidir.
#[derive(Debug, Clone)]
pub struct openharness_agent_definitions {
    config: openharness_agent_definitionsConfig,
}

#[derive(Debug, Clone)]
pub struct openharness_agent_definitionsConfig {
    /// Kaynak repo
    pub source_repo: String,
    /// Dil
    pub language: String,
    /// Risk seviyesi
    pub risk_level: String,
}

impl Default for openharness_agent_definitionsConfig {
    fn default() -> Self {
        Self {
            source_repo: "openharness".to_string(),
            language: "python".to_string(),
            risk_level: "critical".to_string(),
        }
    }
}

impl openharness_agent_definitions {
    /// Yeni wrapper oluştur
    pub fn new() -> Self {
        Self::with_config(openharness_agent_definitionsConfig::default())
    }
    
    /// Yapılandırma ile oluştur
    pub fn with_config(config: openharness_agent_definitionsConfig) -> Self {
        Self { config }
    }
    
    /// Aracın adını döndür
    pub fn name(&self) -> &str {
        "openharness-agent-definitions"
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
            ("content", "any", true),
            ("raw", "any", true),
            ("raw", "any", true),
            ("directory", "any", true),
            ("name", "any", true),
            ("agent", "any", true),
            ("available_servers", "any", true),
        ]
    }
}

impl Default for openharness_agent_definitions {
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
        let wrapper = openharness_agent_definitions::new();
        assert_eq!(wrapper.name(), "openharness-agent-definitions");
    }
    
    #[test]
    fn test_config_default() {
        let config = openharness_agent_definitionsConfig::default();
        assert!(!config.source_repo.is_empty());
    }
}
