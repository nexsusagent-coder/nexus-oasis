//! ═══════════════════════════════════════════════════════════════════════════════
//!  SENTIENT TOOL WRAPPER: openharness-claudemd
//! ═══════════════════════════════════════════════════════════════════════════════
//! 
//! Kaynak: openharness
//! Risk Seviyesi: critical
//! 
//! ═──────────────────────────────────────────────────────────────────────────────

/// openharness_claudemd - Asimile edilmiş araç
/// 
/// SENTIENT Sovereign güvenlik kurallarına tabidir.
#[derive(Debug, Clone)]
pub struct openharness_claudemd {
    config: openharness_claudemdConfig,
}

#[derive(Debug, Clone)]
pub struct openharness_claudemdConfig {
    /// Kaynak repo
    pub source_repo: String,
    /// Dil
    pub language: String,
    /// Risk seviyesi
    pub risk_level: String,
}

impl Default for openharness_claudemdConfig {
    fn default() -> Self {
        Self {
            source_repo: "openharness".to_string(),
            language: "python".to_string(),
            risk_level: "critical".to_string(),
        }
    }
}

impl openharness_claudemd {
    /// Yeni wrapper oluştur
    pub fn new() -> Self {
        Self::with_config(openharness_claudemdConfig::default())
    }
    
    /// Yapılandırma ile oluştur
    pub fn with_config(config: openharness_claudemdConfig) -> Self {
        Self { config }
    }
    
    /// Aracın adını döndür
    pub fn name(&self) -> &str {
        "openharness-claudemd"
    }
    
    /// Aracın açıklamasını döndür
    pub fn description(&self) -> &str {
        "Auto-generated skill from claudemd"
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
            ("*", "any", false),
            ("max_chars_per_file", "any", true),
        ]
    }
}

impl Default for openharness_claudemd {
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
        let wrapper = openharness_claudemd::new();
        assert_eq!(wrapper.name(), "openharness-claudemd");
    }
    
    #[test]
    fn test_config_default() {
        let config = openharness_claudemdConfig::default();
        assert!(!config.source_repo.is_empty());
    }
}
