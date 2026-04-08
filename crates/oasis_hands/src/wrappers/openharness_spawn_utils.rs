//! ═══════════════════════════════════════════════════════════════════════════════
//!  SENTIENT TOOL WRAPPER: openharness-spawn-utils
//! ═══════════════════════════════════════════════════════════════════════════════
//! 
//! Kaynak: openharness
//! Risk Seviyesi: critical
//! 
//! ═──────────────────────────────────────────────────────────────────────────────

/// openharness_spawn_utils - Asimile edilmiş araç
/// 
/// SENTIENT Sovereign güvenlik kurallarına tabidir.
#[derive(Debug, Clone)]
pub struct openharness_spawn_utils {
    config: openharness_spawn_utilsConfig,
}

#[derive(Debug, Clone)]
pub struct openharness_spawn_utilsConfig {
    /// Kaynak repo
    pub source_repo: String,
    /// Dil
    pub language: String,
    /// Risk seviyesi
    pub risk_level: String,
}

impl Default for openharness_spawn_utilsConfig {
    fn default() -> Self {
        Self {
            source_repo: "openharness".to_string(),
            language: "python".to_string(),
            risk_level: "critical".to_string(),
        }
    }
}

impl openharness_spawn_utils {
    /// Yeni wrapper oluştur
    pub fn new() -> Self {
        Self::with_config(openharness_spawn_utilsConfig::default())
    }
    
    /// Yapılandırma ile oluştur
    pub fn with_config(config: openharness_spawn_utilsConfig) -> Self {
        Self { config }
    }
    
    /// Aracın adını döndür
    pub fn name(&self) -> &str {
        "openharness-spawn-utils"
    }
    
    /// Aracın açıklamasını döndür
    pub fn description(&self) -> &str {
        "Environment variable to override the teammate command"
    }
    
    /// Risk seviyesini döndür
    pub fn risk_level(&self) -> &str {
        &self.config.risk_level
    }
    
    /// Parametreleri döndür
    pub fn parameters(&self) -> Vec<(&str, &str, bool)> {
        vec![
            // Parametre yok
        ]
    }
}

impl Default for openharness_spawn_utils {
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
        let wrapper = openharness_spawn_utils::new();
        assert_eq!(wrapper.name(), "openharness-spawn-utils");
    }
    
    #[test]
    fn test_config_default() {
        let config = openharness_spawn_utilsConfig::default();
        assert!(!config.source_repo.is_empty());
    }
}
