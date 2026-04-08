//! ═══════════════════════════════════════════════════════════════════════════════
//!  SENTIENT TOOL WRAPPER: openclaw-web-search
//! ═══════════════════════════════════════════════════════════════════════════════
//! 
//! Kaynak: openclaw
//! Risk Seviyesi: low
//! 
//! ═──────────────────────────────────────────────────────────────────────────────

/// openclaw_web_search - Asimile edilmiş araç
/// 
/// SENTIENT Sovereign güvenlik kurallarına tabidir.
#[derive(Debug, Clone)]
pub struct openclaw_web_search {
    config: openclaw_web_searchConfig,
}

#[derive(Debug, Clone)]
pub struct openclaw_web_searchConfig {
    /// Kaynak repo
    pub source_repo: String,
    /// Dil
    pub language: String,
    /// Risk seviyesi
    pub risk_level: String,
}

impl Default for openclaw_web_searchConfig {
    fn default() -> Self {
        Self {
            source_repo: "openclaw".to_string(),
            language: "typescript".to_string(),
            risk_level: "low".to_string(),
        }
    }
}

impl openclaw_web_search {
    /// Yeni wrapper oluştur
    pub fn new() -> Self {
        Self::with_config(openclaw_web_searchConfig::default())
    }
    
    /// Yapılandırma ile oluştur
    pub fn with_config(config: openclaw_web_searchConfig) -> Self {
        Self { config }
    }
    
    /// Aracın adını döndür
    pub fn name(&self) -> &str {
        "openclaw-web-search"
    }
    
    /// Aracın açıklamasını döndür
    pub fn description(&self) -> &str {
        "Web'de arama yapma ve sonuçları işleme skill'i"
    }
    
    /// Risk seviyesini döndür
    pub fn risk_level(&self) -> &str {
        &self.config.risk_level
    }
    
    /// Parametreleri döndür
    pub fn parameters(&self) -> Vec<(&str, &str, bool)> {
        vec![
            ("query", "string", true),
            ("max_results", "number", false),
        ]
    }
}

impl Default for openclaw_web_search {
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
        let wrapper = openclaw_web_search::new();
        assert_eq!(wrapper.name(), "openclaw-web-search");
    }
    
    #[test]
    fn test_config_default() {
        let config = openclaw_web_searchConfig::default();
        assert!(!config.source_repo.is_empty());
    }
}
