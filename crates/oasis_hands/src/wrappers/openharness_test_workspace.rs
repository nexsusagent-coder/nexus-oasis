//! ═══════════════════════════════════════════════════════════════════════════════
//!  SENTIENT TOOL WRAPPER: openharness-test-workspace
//! ═══════════════════════════════════════════════════════════════════════════════
//! 
//! Kaynak: openharness
//! Risk Seviyesi: critical
//! 
//! ═──────────────────────────────────────────────────────────────────────────────

/// openharness_test_workspace - Asimile edilmiş araç
/// 
/// SENTIENT Sovereign güvenlik kurallarına tabidir.
#[derive(Debug, Clone)]
pub struct openharness_test_workspace {
    config: openharness_test_workspaceConfig,
}

#[derive(Debug, Clone)]
pub struct openharness_test_workspaceConfig {
    /// Kaynak repo
    pub source_repo: String,
    /// Dil
    pub language: String,
    /// Risk seviyesi
    pub risk_level: String,
}

impl Default for openharness_test_workspaceConfig {
    fn default() -> Self {
        Self {
            source_repo: "openharness".to_string(),
            language: "python".to_string(),
            risk_level: "critical".to_string(),
        }
    }
}

impl openharness_test_workspace {
    /// Yeni wrapper oluştur
    pub fn new() -> Self {
        Self::with_config(openharness_test_workspaceConfig::default())
    }
    
    /// Yapılandırma ile oluştur
    pub fn with_config(config: openharness_test_workspaceConfig) -> Self {
        Self { config }
    }
    
    /// Aracın adını döndür
    pub fn name(&self) -> &str {
        "openharness-test-workspace"
    }
    
    /// Aracın açıklamasını döndür
    pub fn description(&self) -> &str {
        "Auto-generated skill from test-workspace"
    }
    
    /// Risk seviyesini döndür
    pub fn risk_level(&self) -> &str {
        &self.config.risk_level
    }
    
    /// Parametreleri döndür
    pub fn parameters(&self) -> Vec<(&str, &str, bool)> {
        vec![
            ("tmp_path", "any", true),
        ]
    }
}

impl Default for openharness_test_workspace {
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
        let wrapper = openharness_test_workspace::new();
        assert_eq!(wrapper.name(), "openharness-test-workspace");
    }
    
    #[test]
    fn test_config_default() {
        let config = openharness_test_workspaceConfig::default();
        assert!(!config.source_repo.is_empty());
    }
}
