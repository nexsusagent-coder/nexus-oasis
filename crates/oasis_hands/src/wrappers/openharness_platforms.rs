//! ═══════════════════════════════════════════════════════════════════════════════
//!  SENTIENT TOOL WRAPPER: openharness-platforms
//! ═══════════════════════════════════════════════════════════════════════════════
//! 
//! Kaynak: openharness
//! Risk Seviyesi: high
//! 
//! ═──────────────────────────────────────────────────────────────────────────────

/// openharness_platforms - Asimile edilmiş araç
/// 
/// SENTIENT Sovereign güvenlik kurallarına tabidir.
#[derive(Debug, Clone)]
pub struct openharness_platforms {
    config: openharness_platformsConfig,
}

#[derive(Debug, Clone)]
pub struct openharness_platformsConfig {
    /// Kaynak repo
    pub source_repo: String,
    /// Dil
    pub language: String,
    /// Risk seviyesi
    pub risk_level: String,
}

impl Default for openharness_platformsConfig {
    fn default() -> Self {
        Self {
            source_repo: "openharness".to_string(),
            language: "python".to_string(),
            risk_level: "high".to_string(),
        }
    }
}

impl openharness_platforms {
    /// Yeni wrapper oluştur
    pub fn new() -> Self {
        Self::with_config(openharness_platformsConfig::default())
    }
    
    /// Yapılandırma ile oluştur
    pub fn with_config(config: openharness_platformsConfig) -> Self {
        Self { config }
    }
    
    /// Aracın adını döndür
    pub fn name(&self) -> &str {
        "openharness-platforms"
    }
    
    /// Aracın açıklamasını döndür
    pub fn description(&self) -> &str {
        "Auto-generated skill from platforms"
    }
    
    /// Risk seviyesini döndür
    pub fn risk_level(&self) -> &str {
        &self.config.risk_level
    }
    
    /// Parametreleri döndür
    pub fn parameters(&self) -> Vec<(&str, &str, bool)> {
        vec![
            ("platform_name", "any", true),
        ]
    }
}

impl Default for openharness_platforms {
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
        let wrapper = openharness_platforms::new();
        assert_eq!(wrapper.name(), "openharness-platforms");
    }
    
    #[test]
    fn test_config_default() {
        let config = openharness_platformsConfig::default();
        assert!(!config.source_repo.is_empty());
    }
}
