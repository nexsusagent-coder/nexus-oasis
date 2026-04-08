//! ═══════════════════════════════════════════════════════════════════════════════
//!  SENTIENT TOOL WRAPPER: openharness-enter-worktree-tool
//! ═══════════════════════════════════════════════════════════════════════════════
//! 
//! Kaynak: openharness
//! Risk Seviyesi: high
//! 
//! ═──────────────────────────────────────────────────────────────────────────────

/// openharness_enter_worktree_tool - Asimile edilmiş araç
/// 
/// SENTIENT Sovereign güvenlik kurallarına tabidir.
#[derive(Debug, Clone)]
pub struct openharness_enter_worktree_tool {
    config: openharness_enter_worktree_toolConfig,
}

#[derive(Debug, Clone)]
pub struct openharness_enter_worktree_toolConfig {
    /// Kaynak repo
    pub source_repo: String,
    /// Dil
    pub language: String,
    /// Risk seviyesi
    pub risk_level: String,
}

impl Default for openharness_enter_worktree_toolConfig {
    fn default() -> Self {
        Self {
            source_repo: "openharness".to_string(),
            language: "python".to_string(),
            risk_level: "high".to_string(),
        }
    }
}

impl openharness_enter_worktree_tool {
    /// Yeni wrapper oluştur
    pub fn new() -> Self {
        Self::with_config(openharness_enter_worktree_toolConfig::default())
    }
    
    /// Yapılandırma ile oluştur
    pub fn with_config(config: openharness_enter_worktree_toolConfig) -> Self {
        Self { config }
    }
    
    /// Aracın adını döndür
    pub fn name(&self) -> &str {
        "openharness-enter-worktree-tool"
    }
    
    /// Aracın açıklamasını döndür
    pub fn description(&self) -> &str {
        "Auto-generated skill from enter-worktree-tool"
    }
    
    /// Risk seviyesini döndür
    pub fn risk_level(&self) -> &str {
        &self.config.risk_level
    }
    
    /// Parametreleri döndür
    pub fn parameters(&self) -> Vec<(&str, &str, bool)> {
        vec![
            ("cwd", "any", true),
            ("*args", "any", false),
            ("repo_root", "any", true),
            ("branch", "any", true),
            ("path", "any", true),
        ]
    }
}

impl Default for openharness_enter_worktree_tool {
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
        let wrapper = openharness_enter_worktree_tool::new();
        assert_eq!(wrapper.name(), "openharness-enter-worktree-tool");
    }
    
    #[test]
    fn test_config_default() {
        let config = openharness_enter_worktree_toolConfig::default();
        assert!(!config.source_repo.is_empty());
    }
}
