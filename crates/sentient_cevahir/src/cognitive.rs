//! Cognitive strateji yönetimi

use crate::{
    types::{CognitiveResult, Strategy},
    error::Result,
    config::CevahirConfig,
};

/// Cognitive strateji yöneticisi
pub struct CognitiveManager {
    config: CevahirConfig,
    strategies: Vec<Strategy>,
}

impl CognitiveManager {
    /// Yeni cognitive manager oluştur
    pub fn new(config: CevahirConfig) -> Result<Self> {
        let strategies = vec![
            Strategy::Direct,
            Strategy::Think,
            Strategy::Debate,
            Strategy::TreeOfThoughts,
        ];
        
        Ok(Self {
            config,
            strategies,
        })
    }
    
    /// Python modülünü başlat (stub)
    pub fn initialize(&mut self) -> Result<()> {
        Ok(())
    }
    
    /// Mevcut stratejileri listele
    pub fn available_strategies(&self) -> &[Strategy] {
        &self.strategies
    }
    
    /// Strateji seç ve çalıştır (stub)
    pub fn process(
        &self,
        input: &str,
        strategy: Strategy,
    ) -> Result<CognitiveResult> {
        Ok(CognitiveResult {
            response: format!("[Cevahir] Processed with {:?}: {}", strategy, input),
            strategy,
            thoughts: None,
            tool_calls: None,
            memory_access: None,
            confidence: 0.85,
        })
    }
    
    /// Otomatik strateji seçimi ile işlem
    pub fn process_auto(&self, input: &str) -> Result<CognitiveResult> {
        let strategy = Strategy::auto_select(input);
        self.process(input, strategy)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strategy_auto_select() {
        assert_eq!(Strategy::auto_select("Merhaba"), Strategy::Direct);
        assert_eq!(Strategy::auto_select("Bu nasıl çalışır?"), Strategy::Think);
    }
}
