//! ═══════════════════════════════════════════════════════════════════════════════
//!  SENTIENT TOOL WRAPPER: openharness-scan
//! ═══════════════════════════════════════════════════════════════════════════════
//! 
//! Kaynak: openharness
//! Risk Seviyesi: low
//! 
//! ═──────────────────────────────────────────────────────────────────────────────

/// openharness_scan - Asimile edilmiş araç
/// 
/// SENTIENT Sovereign güvenlik kurallarına tabidir.
#[derive(Debug, Clone)]
pub struct openharness_scan {
    config: openharness_scanConfig,
}

#[derive(Debug, Clone)]
pub struct openharness_scanConfig {
    /// Kaynak repo
    pub source_repo: String,
    /// Dil
    pub language: String,
    /// Risk seviyesi
    pub risk_level: String,
}

impl Default for openharness_scanConfig {
    fn default() -> Self {
        Self {
            source_repo: "openharness".to_string(),
            language: "python".to_string(),
            risk_level: "low".to_string(),
        }
    }
}

impl openharness_scan {
    /// Yeni wrapper oluştur
    pub fn new() -> Self {
        Self::with_config(openharness_scanConfig::default())
    }
    
    /// Yapılandırma ile oluştur
    pub fn with_config(config: openharness_scanConfig) -> Self {
        Self { config }
    }
    
    /// Aracın adını döndür
    pub fn name(&self) -> &str {
        "openharness-scan"
    }
    
    /// Aracın açıklamasını döndür
    pub fn description(&self) -> &str {
        "Auto-generated skill from scan"
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
            ("max_files", "any", true),
            ("path", "any", true),
            ("content", "any", true),
        ]
    }
}

impl Default for openharness_scan {
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
        let wrapper = openharness_scan::new();
        assert_eq!(wrapper.name(), "openharness-scan");
    }
    
    #[test]
    fn test_config_default() {
        let config = openharness_scanConfig::default();
        assert!(!config.source_repo.is_empty());
    }
}
