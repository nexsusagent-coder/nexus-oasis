//! ═══════════════════════════════════════════════════════════════════════════════
//!  SENTIENT TOOL WRAPPER: openharness-matrix
//! ═══════════════════════════════════════════════════════════════════════════════
//! 
//! Kaynak: openharness
//! Risk Seviyesi: critical
//! 
//! ═──────────────────────────────────────────────────────────────────────────────

/// openharness_matrix - Asimile edilmiş araç
/// 
/// SENTIENT Sovereign güvenlik kurallarına tabidir.
#[derive(Debug, Clone)]
pub struct openharness_matrix {
    config: openharness_matrixConfig,
}

#[derive(Debug, Clone)]
pub struct openharness_matrixConfig {
    /// Kaynak repo
    pub source_repo: String,
    /// Dil
    pub language: String,
    /// Risk seviyesi
    pub risk_level: String,
}

impl Default for openharness_matrixConfig {
    fn default() -> Self {
        Self {
            source_repo: "openharness".to_string(),
            language: "python".to_string(),
            risk_level: "critical".to_string(),
        }
    }
}

impl openharness_matrix {
    /// Yeni wrapper oluştur
    pub fn new() -> Self {
        Self::with_config(openharness_matrixConfig::default())
    }
    
    /// Yapılandırma ile oluştur
    pub fn with_config(config: openharness_matrixConfig) -> Self {
        Self { config }
    }
    
    /// Aracın adını döndür
    pub fn name(&self) -> &str {
        "openharness-matrix"
    }
    
    /// Aracın açıklamasını döndür
    pub fn description(&self) -> &str {
        "Must stay below TYPING_NOTICE_TIMEOUT_MS so the indicator doesn't expire mid-processing."
    }
    
    /// Risk seviyesini döndür
    pub fn risk_level(&self) -> &str {
        &self.config.risk_level
    }
    
    /// Parametreleri döndür
    pub fn parameters(&self) -> Vec<(&str, &str, bool)> {
        vec![
            ("tag", "any", true),
            ("attr", "any", true),
            ("value", "any", true),
            ("text", "any", true),
            ("text", "any", true),
        ]
    }
}

impl Default for openharness_matrix {
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
        let wrapper = openharness_matrix::new();
        assert_eq!(wrapper.name(), "openharness-matrix");
    }
    
    #[test]
    fn test_config_default() {
        let config = openharness_matrixConfig::default();
        assert!(!config.source_repo.is_empty());
    }
}
