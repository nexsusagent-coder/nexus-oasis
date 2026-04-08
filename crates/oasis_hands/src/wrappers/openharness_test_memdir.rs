//! ═══════════════════════════════════════════════════════════════════════════════
//!  SENTIENT TOOL WRAPPER: openharness-test-memdir
//! ═══════════════════════════════════════════════════════════════════════════════
//! 
//! Kaynak: openharness
//! Risk Seviyesi: medium
//! 
//! ═──────────────────────────────────────────────────────────────────────────────

/// openharness_test_memdir - Asimile edilmiş araç
/// 
/// SENTIENT Sovereign güvenlik kurallarına tabidir.
#[derive(Debug, Clone)]
pub struct openharness_test_memdir {
    config: openharness_test_memdirConfig,
}

#[derive(Debug, Clone)]
pub struct openharness_test_memdirConfig {
    /// Kaynak repo
    pub source_repo: String,
    /// Dil
    pub language: String,
    /// Risk seviyesi
    pub risk_level: String,
}

impl Default for openharness_test_memdirConfig {
    fn default() -> Self {
        Self {
            source_repo: "openharness".to_string(),
            language: "python".to_string(),
            risk_level: "medium".to_string(),
        }
    }
}

impl openharness_test_memdir {
    /// Yeni wrapper oluştur
    pub fn new() -> Self {
        Self::with_config(openharness_test_memdirConfig::default())
    }
    
    /// Yapılandırma ile oluştur
    pub fn with_config(config: openharness_test_memdirConfig) -> Self {
        Self { config }
    }
    
    /// Aracın adını döndür
    pub fn name(&self) -> &str {
        "openharness-test-memdir"
    }
    
    /// Aracın açıklamasını döndür
    pub fn description(&self) -> &str {
        "--- Frontmatter parsing tests ---"
    }
    
    /// Risk seviyesini döndür
    pub fn risk_level(&self) -> &str {
        &self.config.risk_level
    }
    
    /// Parametreleri döndür
    pub fn parameters(&self) -> Vec<(&str, &str, bool)> {
        vec![
            ("tmp_path", "any", true),
            ("monkeypatch", "any", true),
            ("tmp_path", "any", true),
            ("monkeypatch", "any", true),
            ("tmp_path", "any", true),
            ("monkeypatch", "any", true),
            ("tmp_path", "any", true),
            ("tmp_path", "any", true),
            ("tmp_path", "any", true),
            ("tmp_path", "any", true),
            ("tmp_path", "any", true),
            ("tmp_path", "any", true),
            ("monkeypatch", "any", true),
            ("tmp_path", "any", true),
            ("monkeypatch", "any", true),
            ("tmp_path", "any", true),
            ("monkeypatch", "any", true),
            ("tmp_path", "any", true),
            ("monkeypatch", "any", true),
        ]
    }
}

impl Default for openharness_test_memdir {
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
        let wrapper = openharness_test_memdir::new();
        assert_eq!(wrapper.name(), "openharness-test-memdir");
    }
    
    #[test]
    fn test_config_default() {
        let config = openharness_test_memdirConfig::default();
        assert!(!config.source_repo.is_empty());
    }
}
