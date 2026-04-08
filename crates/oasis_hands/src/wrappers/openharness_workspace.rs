//! ═══════════════════════════════════════════════════════════════════════════════
//!  SENTIENT TOOL WRAPPER: openharness-workspace
//! ═══════════════════════════════════════════════════════════════════════════════
//! 
//! Kaynak: openharness
//! Risk Seviyesi: critical
//! 
//! ═──────────────────────────────────────────────────────────────────────────────

/// openharness_workspace - Asimile edilmiş araç
/// 
/// SENTIENT Sovereign güvenlik kurallarına tabidir.
#[derive(Debug, Clone)]
pub struct openharness_workspace {
    config: openharness_workspaceConfig,
}

#[derive(Debug, Clone)]
pub struct openharness_workspaceConfig {
    /// Kaynak repo
    pub source_repo: String,
    /// Dil
    pub language: String,
    /// Risk seviyesi
    pub risk_level: String,
}

impl Default for openharness_workspaceConfig {
    fn default() -> Self {
        Self {
            source_repo: "openharness".to_string(),
            language: "python".to_string(),
            risk_level: "critical".to_string(),
        }
    }
}

impl openharness_workspace {
    /// Yeni wrapper oluştur
    pub fn new() -> Self {
        Self::with_config(openharness_workspaceConfig::default())
    }
    
    /// Yapılandırma ile oluştur
    pub fn with_config(config: openharness_workspaceConfig) -> Self {
        Self { config }
    }
    
    /// Aracın adını döndür
    pub fn name(&self) -> &str {
        "openharness-workspace"
    }
    
    /// Aracın açıklamasını döndür
    pub fn description(&self) -> &str {
        "Auto-generated skill from workspace"
    }
    
    /// Risk seviyesini döndür
    pub fn risk_level(&self) -> &str {
        &self.config.risk_level
    }
    
    /// Parametreleri döndür
    pub fn parameters(&self) -> Vec<(&str, &str, bool)> {
        vec![
            ("workspace", "any", true),
            ("workspace", "any", true),
            ("workspace", "any", true),
            ("workspace", "any", true),
            ("workspace", "any", true),
            ("workspace", "any", true),
            ("workspace", "any", true),
            ("workspace", "any", true),
            ("workspace", "any", true),
            ("workspace", "any", true),
            ("workspace", "any", true),
            ("workspace", "any", true),
            ("workspace", "any", true),
            ("workspace", "any", true),
            ("workspace", "any", true),
        ]
    }
}

impl Default for openharness_workspace {
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
        let wrapper = openharness_workspace::new();
        assert_eq!(wrapper.name(), "openharness-workspace");
    }
    
    #[test]
    fn test_config_default() {
        let config = openharness_workspaceConfig::default();
        assert!(!config.source_repo.is_empty());
    }
}
