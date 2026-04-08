//! ═══════════════════════════════════════════════════════════════════════════════
//!  SENTIENT TOOL WRAPPER: openharness-react-launcher
//! ═══════════════════════════════════════════════════════════════════════════════
//! 
//! Kaynak: openharness
//! Risk Seviyesi: critical
//! 
//! ═──────────────────────────────────────────────────────────────────────────────

/// openharness_react_launcher - Asimile edilmiş araç
/// 
/// SENTIENT Sovereign güvenlik kurallarına tabidir.
#[derive(Debug, Clone)]
pub struct openharness_react_launcher {
    config: openharness_react_launcherConfig,
}

#[derive(Debug, Clone)]
pub struct openharness_react_launcherConfig {
    /// Kaynak repo
    pub source_repo: String,
    /// Dil
    pub language: String,
    /// Risk seviyesi
    pub risk_level: String,
}

impl Default for openharness_react_launcherConfig {
    fn default() -> Self {
        Self {
            source_repo: "openharness".to_string(),
            language: "python".to_string(),
            risk_level: "critical".to_string(),
        }
    }
}

impl openharness_react_launcher {
    /// Yeni wrapper oluştur
    pub fn new() -> Self {
        Self::with_config(openharness_react_launcherConfig::default())
    }
    
    /// Yapılandırma ile oluştur
    pub fn with_config(config: openharness_react_launcherConfig) -> Self {
        Self { config }
    }
    
    /// Aracın adını döndür
    pub fn name(&self) -> &str {
        "openharness-react-launcher"
    }
    
    /// Aracın açıklamasını döndür
    pub fn description(&self) -> &str {
        "Auto-generated skill from react-launcher"
    }
    
    /// Risk seviyesini döndür
    pub fn risk_level(&self) -> &str {
        &self.config.risk_level
    }
    
    /// Parametreleri döndür
    pub fn parameters(&self) -> Vec<(&str, &str, bool)> {
        vec![
            ("frontend_dir", "any", true),
        ]
    }
}

impl Default for openharness_react_launcher {
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
        let wrapper = openharness_react_launcher::new();
        assert_eq!(wrapper.name(), "openharness-react-launcher");
    }
    
    #[test]
    fn test_config_default() {
        let config = openharness_react_launcherConfig::default();
        assert!(!config.source_repo.is_empty());
    }
}
