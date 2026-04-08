//! ═══════════════════════════════════════════════════════════════════════════════
//!  SENTIENT TOOL WRAPPER: openclaw-file-read
//! ═══════════════════════════════════════════════════════════════════════════════
//! 
//! Kaynak: openclaw
//! Risk Seviyesi: medium
//! 
//! ═──────────────────────────────────────────────────────────────────────────────

/// openclaw_file_read - Asimile edilmiş araç
/// 
/// SENTIENT Sovereign güvenlik kurallarına tabidir.
#[derive(Debug, Clone)]
pub struct openclaw_file_read {
    config: openclaw_file_readConfig,
}

#[derive(Debug, Clone)]
pub struct openclaw_file_readConfig {
    /// Kaynak repo
    pub source_repo: String,
    /// Dil
    pub language: String,
    /// Risk seviyesi
    pub risk_level: String,
}

impl Default for openclaw_file_readConfig {
    fn default() -> Self {
        Self {
            source_repo: "openclaw".to_string(),
            language: "typescript".to_string(),
            risk_level: "medium".to_string(),
        }
    }
}

impl openclaw_file_read {
    /// Yeni wrapper oluştur
    pub fn new() -> Self {
        Self::with_config(openclaw_file_readConfig::default())
    }
    
    /// Yapılandırma ile oluştur
    pub fn with_config(config: openclaw_file_readConfig) -> Self {
        Self { config }
    }
    
    /// Aracın adını döndür
    pub fn name(&self) -> &str {
        "openclaw-file-read"
    }
    
    /// Aracın açıklamasını döndür
    pub fn description(&self) -> &str {
        "Dosya okuma ve içerik çıkarma"
    }
    
    /// Risk seviyesini döndür
    pub fn risk_level(&self) -> &str {
        &self.config.risk_level
    }
    
    /// Parametreleri döndür
    pub fn parameters(&self) -> Vec<(&str, &str, bool)> {
        vec![
            ("path", "string", true),
        ]
    }
}

impl Default for openclaw_file_read {
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
        let wrapper = openclaw_file_read::new();
        assert_eq!(wrapper.name(), "openclaw-file-read");
    }
    
    #[test]
    fn test_config_default() {
        let config = openclaw_file_readConfig::default();
        assert!(!config.source_repo.is_empty());
    }
}
