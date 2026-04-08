//! ═══════════════════════════════════════════════════════════════════════════════
//!  SENTIENT TOOL WRAPPER: openharness-voice-mode
//! ═══════════════════════════════════════════════════════════════════════════════
//! 
//! Kaynak: openharness
//! Risk Seviyesi: high
//! 
//! ═──────────────────────────────────────────────────────────────────────────────

/// openharness_voice_mode - Asimile edilmiş araç
/// 
/// SENTIENT Sovereign güvenlik kurallarına tabidir.
#[derive(Debug, Clone)]
pub struct openharness_voice_mode {
    config: openharness_voice_modeConfig,
}

#[derive(Debug, Clone)]
pub struct openharness_voice_modeConfig {
    /// Kaynak repo
    pub source_repo: String,
    /// Dil
    pub language: String,
    /// Risk seviyesi
    pub risk_level: String,
}

impl Default for openharness_voice_modeConfig {
    fn default() -> Self {
        Self {
            source_repo: "openharness".to_string(),
            language: "python".to_string(),
            risk_level: "high".to_string(),
        }
    }
}

impl openharness_voice_mode {
    /// Yeni wrapper oluştur
    pub fn new() -> Self {
        Self::with_config(openharness_voice_modeConfig::default())
    }
    
    /// Yapılandırma ile oluştur
    pub fn with_config(config: openharness_voice_modeConfig) -> Self {
        Self { config }
    }
    
    /// Aracın adını döndür
    pub fn name(&self) -> &str {
        "openharness-voice-mode"
    }
    
    /// Aracın açıklamasını döndür
    pub fn description(&self) -> &str {
        "Auto-generated skill from voice-mode"
    }
    
    /// Risk seviyesini döndür
    pub fn risk_level(&self) -> &str {
        &self.config.risk_level
    }
    
    /// Parametreleri döndür
    pub fn parameters(&self) -> Vec<(&str, &str, bool)> {
        vec![
            ("enabled", "any", true),
            ("provider", "any", true),
        ]
    }
}

impl Default for openharness_voice_mode {
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
        let wrapper = openharness_voice_mode::new();
        assert_eq!(wrapper.name(), "openharness-voice-mode");
    }
    
    #[test]
    fn test_config_default() {
        let config = openharness_voice_modeConfig::default();
        assert!(!config.source_repo.is_empty());
    }
}
