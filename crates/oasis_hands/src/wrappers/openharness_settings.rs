//! ═══════════════════════════════════════════════════════════════════════════════
//!  SENTIENT TOOL WRAPPER: openharness-settings
//! ═══════════════════════════════════════════════════════════════════════════════
//! 
//! Kaynak: openharness
//! Risk Seviyesi: critical
//! 
//! ═──────────────────────────────────────────────────────────────────────────────

/// openharness_settings - Asimile edilmiş araç
/// 
/// SENTIENT Sovereign güvenlik kurallarına tabidir.
#[derive(Debug, Clone)]
pub struct openharness_settings {
    config: openharness_settingsConfig,
}

#[derive(Debug, Clone)]
pub struct openharness_settingsConfig {
    /// Kaynak repo
    pub source_repo: String,
    /// Dil
    pub language: String,
    /// Risk seviyesi
    pub risk_level: String,
}

impl Default for openharness_settingsConfig {
    fn default() -> Self {
        Self {
            source_repo: "openharness".to_string(),
            language: "python".to_string(),
            risk_level: "critical".to_string(),
        }
    }
}

impl openharness_settings {
    /// Yeni wrapper oluştur
    pub fn new() -> Self {
        Self::with_config(openharness_settingsConfig::default())
    }
    
    /// Yapılandırma ile oluştur
    pub fn with_config(config: openharness_settingsConfig) -> Self {
        Self { config }
    }
    
    /// Aracın adını döndür
    pub fn name(&self) -> &str {
        "openharness-settings"
    }
    
    /// Aracın açıklamasını döndür
    pub fn description(&self) -> &str {
        "Auto-generated skill from settings"
    }
    
    /// Risk seviyesini döndür
    pub fn risk_level(&self) -> &str {
        &self.config.risk_level
    }
    
    /// Parametreleri döndür
    pub fn parameters(&self) -> Vec<(&str, &str, bool)> {
        vec![
            ("model", "any", true),
            ("profile_name", "any", true),
            ("profile", "any", true),
            ("provider", "any", true),
            ("profile", "any", true),
            ("auth_source", "any", true),
            ("auth_source", "any", true),
            ("profile_name", "any", true),
            ("profile", "any", true),
            ("provider", "any", true),
            ("api_format", "any", true),
            ("value", "any", true),
            ("settings", "any", true),
            ("settings", "any", true),
            ("settings", "any", true),
            ("value", "any", true),
            ("config_path", "any", true),
            ("settings", "any", true),
            ("config_path", "any", true),
        ]
    }
}

impl Default for openharness_settings {
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
        let wrapper = openharness_settings::new();
        assert_eq!(wrapper.name(), "openharness-settings");
    }
    
    #[test]
    fn test_config_default() {
        let config = openharness_settingsConfig::default();
        assert!(!config.source_repo.is_empty());
    }
}
