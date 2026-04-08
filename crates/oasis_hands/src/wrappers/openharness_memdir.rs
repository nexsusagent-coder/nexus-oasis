//! ═══════════════════════════════════════════════════════════════════════════════
//!  SENTIENT TOOL WRAPPER: openharness-memdir
//! ═══════════════════════════════════════════════════════════════════════════════
//! 
//! Kaynak: openharness
//! Risk Seviyesi: medium
//! 
//! ═──────────────────────────────────────────────────────────────────────────────

/// openharness_memdir - Asimile edilmiş araç
/// 
/// SENTIENT Sovereign güvenlik kurallarına tabidir.
#[derive(Debug, Clone)]
pub struct openharness_memdir {
    config: openharness_memdirConfig,
}

#[derive(Debug, Clone)]
pub struct openharness_memdirConfig {
    /// Kaynak repo
    pub source_repo: String,
    /// Dil
    pub language: String,
    /// Risk seviyesi
    pub risk_level: String,
}

impl Default for openharness_memdirConfig {
    fn default() -> Self {
        Self {
            source_repo: "openharness".to_string(),
            language: "python".to_string(),
            risk_level: "medium".to_string(),
        }
    }
}

impl openharness_memdir {
    /// Yeni wrapper oluştur
    pub fn new() -> Self {
        Self::with_config(openharness_memdirConfig::default())
    }
    
    /// Yapılandırma ile oluştur
    pub fn with_config(config: openharness_memdirConfig) -> Self {
        Self { config }
    }
    
    /// Aracın adını döndür
    pub fn name(&self) -> &str {
        "openharness-memdir"
    }
    
    /// Aracın açıklamasını döndür
    pub fn description(&self) -> &str {
        "Auto-generated skill from memdir"
    }
    
    /// Risk seviyesini döndür
    pub fn risk_level(&self) -> &str {
        &self.config.risk_level
    }
    
    /// Parametreleri döndür
    pub fn parameters(&self) -> Vec<(&str, &str, bool)> {
        vec![
            ("cwd", "any", true),
            ("*", "any", false),
            ("max_entrypoint_lines", "any", true),
        ]
    }
}

impl Default for openharness_memdir {
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
        let wrapper = openharness_memdir::new();
        assert_eq!(wrapper.name(), "openharness-memdir");
    }
    
    #[test]
    fn test_config_default() {
        let config = openharness_memdirConfig::default();
        assert!(!config.source_repo.is_empty());
    }
}
