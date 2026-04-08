//! ═══════════════════════════════════════════════════════════════════════════════
//!  SENTIENT TOOL WRAPPER: openharness-system-prompt
//! ═══════════════════════════════════════════════════════════════════════════════
//! 
//! Kaynak: openharness
//! Risk Seviyesi: critical
//! 
//! ═──────────────────────────────────────────────────────────────────────────────

/// openharness_system_prompt - Asimile edilmiş araç
/// 
/// SENTIENT Sovereign güvenlik kurallarına tabidir.
#[derive(Debug, Clone)]
pub struct openharness_system_prompt {
    config: openharness_system_promptConfig,
}

#[derive(Debug, Clone)]
pub struct openharness_system_promptConfig {
    /// Kaynak repo
    pub source_repo: String,
    /// Dil
    pub language: String,
    /// Risk seviyesi
    pub risk_level: String,
}

impl Default for openharness_system_promptConfig {
    fn default() -> Self {
        Self {
            source_repo: "openharness".to_string(),
            language: "python".to_string(),
            risk_level: "critical".to_string(),
        }
    }
}

impl openharness_system_prompt {
    /// Yeni wrapper oluştur
    pub fn new() -> Self {
        Self::with_config(openharness_system_promptConfig::default())
    }
    
    /// Yapılandırma ile oluştur
    pub fn with_config(config: openharness_system_promptConfig) -> Self {
        Self { config }
    }
    
    /// Aracın adını döndür
    pub fn name(&self) -> &str {
        "openharness-system-prompt"
    }
    
    /// Aracın açıklamasını döndür
    pub fn description(&self) -> &str {
        "Doing tasks"
    }
    
    /// Risk seviyesini döndür
    pub fn risk_level(&self) -> &str {
        &self.config.risk_level
    }
    
    /// Parametreleri döndür
    pub fn parameters(&self) -> Vec<(&str, &str, bool)> {
        vec![
            ("env", "any", true),
        ]
    }
}

impl Default for openharness_system_prompt {
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
        let wrapper = openharness_system_prompt::new();
        assert_eq!(wrapper.name(), "openharness-system-prompt");
    }
    
    #[test]
    fn test_config_default() {
        let config = openharness_system_promptConfig::default();
        assert!(!config.source_repo.is_empty());
    }
}
