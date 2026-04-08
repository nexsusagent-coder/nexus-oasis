//! ═══════════════════════════════════════════════════════════════════════════════
//!  SENTIENT TOOL WRAPPER: openharness-test-worktree
//! ═══════════════════════════════════════════════════════════════════════════════
//! 
//! Kaynak: openharness
//! Risk Seviyesi: low
//! 
//! ═──────────────────────────────────────────────────────────────────────────────

/// openharness_test_worktree - Asimile edilmiş araç
/// 
/// SENTIENT Sovereign güvenlik kurallarına tabidir.
#[derive(Debug, Clone)]
pub struct openharness_test_worktree {
    config: openharness_test_worktreeConfig,
}

#[derive(Debug, Clone)]
pub struct openharness_test_worktreeConfig {
    /// Kaynak repo
    pub source_repo: String,
    /// Dil
    pub language: String,
    /// Risk seviyesi
    pub risk_level: String,
}

impl Default for openharness_test_worktreeConfig {
    fn default() -> Self {
        Self {
            source_repo: "openharness".to_string(),
            language: "python".to_string(),
            risk_level: "low".to_string(),
        }
    }
}

impl openharness_test_worktree {
    /// Yeni wrapper oluştur
    pub fn new() -> Self {
        Self::with_config(openharness_test_worktreeConfig::default())
    }
    
    /// Yapılandırma ile oluştur
    pub fn with_config(config: openharness_test_worktreeConfig) -> Self {
        Self { config }
    }
    
    /// Aracın adını döndür
    pub fn name(&self) -> &str {
        "openharness-test-worktree"
    }
    
    /// Aracın açıklamasını döndür
    pub fn description(&self) -> &str {
        "---------------------------------------------------------------------------"
    }
    
    /// Risk seviyesini döndür
    pub fn risk_level(&self) -> &str {
        &self.config.risk_level
    }
    
    /// Parametreleri döndür
    pub fn parameters(&self) -> Vec<(&str, &str, bool)> {
        vec![
            ("slug", "any", true),
            ("slug", "any", true),
        ]
    }
}

impl Default for openharness_test_worktree {
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
        let wrapper = openharness_test_worktree::new();
        assert_eq!(wrapper.name(), "openharness-test-worktree");
    }
    
    #[test]
    fn test_config_default() {
        let config = openharness_test_worktreeConfig::default();
        assert!(!config.source_repo.is_empty());
    }
}
