//! ═══════════════════════════════════════════════════════════════════════════════
//!  SENTIENT TOOL WRAPPER: openharness-lsp-tool
//! ═══════════════════════════════════════════════════════════════════════════════
//! 
//! Kaynak: openharness
//! Risk Seviyesi: critical
//! 
//! ═──────────────────────────────────────────────────────────────────────────────

/// openharness_lsp_tool - Asimile edilmiş araç
/// 
/// SENTIENT Sovereign güvenlik kurallarına tabidir.
#[derive(Debug, Clone)]
pub struct openharness_lsp_tool {
    config: openharness_lsp_toolConfig,
}

#[derive(Debug, Clone)]
pub struct openharness_lsp_toolConfig {
    /// Kaynak repo
    pub source_repo: String,
    /// Dil
    pub language: String,
    /// Risk seviyesi
    pub risk_level: String,
}

impl Default for openharness_lsp_toolConfig {
    fn default() -> Self {
        Self {
            source_repo: "openharness".to_string(),
            language: "python".to_string(),
            risk_level: "critical".to_string(),
        }
    }
}

impl openharness_lsp_tool {
    /// Yeni wrapper oluştur
    pub fn new() -> Self {
        Self::with_config(openharness_lsp_toolConfig::default())
    }
    
    /// Yapılandırma ile oluştur
    pub fn with_config(config: openharness_lsp_toolConfig) -> Self {
        Self { config }
    }
    
    /// Aracın adını döndür
    pub fn name(&self) -> &str {
        "openharness-lsp-tool"
    }
    
    /// Aracın açıklamasını döndür
    pub fn description(&self) -> &str {
        "Auto-generated skill from lsp-tool"
    }
    
    /// Risk seviyesini döndür
    pub fn risk_level(&self) -> &str {
        &self.config.risk_level
    }
    
    /// Parametreleri döndür
    pub fn parameters(&self) -> Vec<(&str, &str, bool)> {
        vec![
            ("base", "any", true),
            ("candidate", "any", true),
            ("path", "any", true),
            ("root", "any", true),
            ("results", "any", true),
            ("root", "any", true),
            ("results", "any", true),
            ("int", "any", true),
            ("str]]", "any", true),
            ("root", "any", true),
        ]
    }
}

impl Default for openharness_lsp_tool {
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
        let wrapper = openharness_lsp_tool::new();
        assert_eq!(wrapper.name(), "openharness-lsp-tool");
    }
    
    #[test]
    fn test_config_default() {
        let config = openharness_lsp_toolConfig::default();
        assert!(!config.source_repo.is_empty());
    }
}
