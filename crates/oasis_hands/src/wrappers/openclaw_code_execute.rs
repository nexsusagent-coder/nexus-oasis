//! ═══════════════════════════════════════════════════════════════════════════════
//!  SENTIENT TOOL WRAPPER: openclaw-code-execute
//! ═══════════════════════════════════════════════════════════════════════════════
//! 
//! Kaynak: openclaw
//! Risk Seviyesi: high
//! 
//! ═──────────────────────────────────────────────────────────────────────────────

/// openclaw_code_execute - Asimile edilmiş araç
/// 
/// SENTIENT Sovereign güvenlik kurallarına tabidir.
#[derive(Debug, Clone)]
pub struct openclaw_code_execute {
    config: openclaw_code_executeConfig,
}

#[derive(Debug, Clone)]
pub struct openclaw_code_executeConfig {
    /// Kaynak repo
    pub source_repo: String,
    /// Dil
    pub language: String,
    /// Risk seviyesi
    pub risk_level: String,
}

impl Default for openclaw_code_executeConfig {
    fn default() -> Self {
        Self {
            source_repo: "openclaw".to_string(),
            language: "typescript".to_string(),
            risk_level: "high".to_string(),
        }
    }
}

impl openclaw_code_execute {
    /// Yeni wrapper oluştur
    pub fn new() -> Self {
        Self::with_config(openclaw_code_executeConfig::default())
    }
    
    /// Yapılandırma ile oluştur
    pub fn with_config(config: openclaw_code_executeConfig) -> Self {
        Self { config }
    }
    
    /// Aracın adını döndür
    pub fn name(&self) -> &str {
        "openclaw-code-execute"
    }
    
    /// Aracın açıklamasını döndür
    pub fn description(&self) -> &str {
        "Sandbox içinde kod çalıştırma"
    }
    
    /// Risk seviyesini döndür
    pub fn risk_level(&self) -> &str {
        &self.config.risk_level
    }
    
    /// Parametreleri döndür
    pub fn parameters(&self) -> Vec<(&str, &str, bool)> {
        vec![
            ("code", "string", true),
            ("language", "string", true),
        ]
    }
}

impl Default for openclaw_code_execute {
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
        let wrapper = openclaw_code_execute::new();
        assert_eq!(wrapper.name(), "openclaw-code-execute");
    }
    
    #[test]
    fn test_config_default() {
        let config = openclaw_code_executeConfig::default();
        assert!(!config.source_repo.is_empty());
    }
}
