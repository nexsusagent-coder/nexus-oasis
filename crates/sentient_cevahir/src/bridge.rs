//! Ana Cevahir Bridge - SENTIENT OS ile tam entegrasyon
//! 
//! Bu modül, tüm Cevahir bileşenlerini birleştiren ana API'yi sağlar.

use crate::{
    config::CevahirConfig,
    cognitive::CognitiveManager,
    tokenizer::TokenizerWrapper,
    model::ModelWrapper,
    tools::{ToolDefinition, ToolExecutor},
    memory::MemoryAdapter,
    types::{DecodingConfig, GenerationOutput, TokenizationResult, Strategy, CognitiveResult},
    error::{CevahirError, Result},
};
use std::sync::Arc;
use parking_lot::RwLock;

/// Ana Cevahir Bridge
/// 
/// SENTIENT OS ile Cevahir AI arasındaki ana köprü.
/// Tüm bileşenleri tek bir API altında birleştirir.
/// 
/// # Örnek
/// 
/// ```rust,no_run
/// use sentient_cevahir::{CevahirBridge, CevahirConfig, CognitiveStrategy};
/// 
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let bridge = CevahirBridge::new(CevahirConfig::default())?;
///     
///     // Basit üretim
///     let response = bridge.generate("Merhaba", 128).await?;
///     
///     // Cognitive strateji ile
///     let output = bridge.process_with_strategy(
///         "Bu kodu analiz et",
///         CognitiveStrategy::Think,
///     ).await?;
///     
///     Ok(())
/// }
/// ```
pub struct CevahirBridge {
    /// Yapılandırma
    config: CevahirConfig,
    
    /// Tokenizer
    tokenizer: TokenizerWrapper,
    
    /// Model
    model: ModelWrapper,
    
    /// Cognitive Manager
    cognitive: CognitiveManager,
    
    /// Tool Executor
    tools: ToolExecutor,
    
    /// Memory Adapter
    memory: Option<MemoryAdapter>,
    
    /// Başlatıldı mı?
    initialized: bool,
}

impl CevahirBridge {
    /// Yeni bridge oluştur
    pub fn new(config: CevahirConfig) -> Result<Self> {
        let mut bridge = Self {
            tokenizer: TokenizerWrapper::new(&config.vocab_path, &config.merges_path)?,
            model: ModelWrapper::new(config.clone())?,
            cognitive: CognitiveManager::new(config.clone())?,
            tools: ToolExecutor::new(),
            memory: None,
            config,
            initialized: false,
        };
        
        bridge.initialize()?;
        Ok(bridge)
    }
    
    /// Memory ile birlikte bridge oluştur
    pub fn with_memory(config: CevahirConfig, memory: MemoryAdapter) -> Result<Self> {
        let mut bridge = Self {
            tokenizer: TokenizerWrapper::new(&config.vocab_path, &config.merges_path)?,
            model: ModelWrapper::new(config.clone())?,
            cognitive: CognitiveManager::new(config.clone())?,
            tools: ToolExecutor::new(),
            memory: Some(memory),
            config,
            initialized: false,
        };
        
        bridge.initialize()?;
        Ok(bridge)
    }
    
    /// Tüm bileşenleri başlat
    pub fn initialize(&mut self) -> Result<()> {
        if self.initialized {
            return Ok(());
        }
        
        log::info!("[CevahirBridge] Starting initialization...");
        
        // Tokenizer başlat
        // (zaten constructor'da başlatıldı)
        
        // Model başlat
        self.model.initialize()?;
        
        // Cognitive manager başlat
        self.cognitive.initialize()?;
        
        self.initialized = true;
        
        log::info!("[CevahirBridge] Initialization complete");
        log::info!("  - Vocab size: {}", self.config.vocab_size);
        log::info!("  - Embed dim: {}", self.config.embed_dim);
        log::info!("  - Num layers: {}", self.config.num_layers);
        log::info!("  - Num heads: {}", self.config.num_heads);
        log::info!("  - Device: {}", self.config.device);
        
        Ok(())
    }
    
    /// Metin üret
    pub async fn generate(&self, prompt: &str, max_tokens: usize) -> Result<String> {
        let start = std::time::Instant::now();
        
        let text = self.model.generate(prompt, max_tokens)?;
        
        let duration = start.elapsed();
        log::debug!(
            "[CevahirBridge] Generated {} chars in {:?}",
            text.len(),
            duration
        );
        
        Ok(text)
    }
    
    /// Decoding config ile üret
    pub async fn generate_with_config(
        &self,
        prompt: &str,
        config: DecodingConfig,
    ) -> Result<GenerationOutput> {
        let start = std::time::Instant::now();
        
        let text = self.model.generate_with_config(prompt, config.clone())?;
        
        let token_count = self.tokenizer.encode(&text)?.ids.len();
        
        Ok(GenerationOutput {
            text,
            token_count,
            strategy: Strategy::Direct,
            reasoning: None,
            duration_ms: start.elapsed().as_millis() as u64,
        })
    }
    
    /// Cognitive strateji ile işlem
    pub async fn process_with_strategy(
        &self,
        input: &str,
        strategy: Strategy,
    ) -> Result<CognitiveResult> {
        let start = std::time::Instant::now();
        
        // Bellek konteksini al (varsa)
        let context = if let Some(memory) = &self.memory {
            memory.search(input, 5).await?
        } else {
            vec![]
        };
        
        // Cognitive işlem
        let model_ref = self.model.python_ref()
            .ok_or_else(|| CevahirError::ModelError("Model not initialized".into()))?;
        
        let mut result = self.cognitive.process(input, strategy, model_ref)?;
        
        // Belleğe kaydet (varsa)
        if let Some(memory) = &self.memory {
            memory.store(input, &result.response).await?;
        }
        
        let duration = start.elapsed();
        log::debug!(
            "[CevahirBridge] Processed with {:?} strategy in {:?}",
            strategy,
            duration
        );
        
        Ok(result)
    }
    
    /// Otomatik strateji seçimi ile işlem
    pub async fn process_auto(&self, input: &str) -> Result<CognitiveResult> {
        let strategy = Strategy::auto_select(input);
        self.process_with_strategy(input, strategy).await
    }
    
    /// Encode
    pub fn encode(&self, text: &str) -> Result<TokenizationResult> {
        self.tokenizer.encode(text)
    }
    
    /// Decode
    pub fn decode(&self, ids: &[u32]) -> Result<String> {
        self.tokenizer.decode(ids)
    }
    
    /// Tool kaydet
    pub fn register_tool(&self, tool: ToolDefinition) -> Result<()> {
        self.tools.register(tool);
        Ok(())
    }
    
    /// Tool çalıştır
    pub async fn execute_tool(&self, name: &str, args: &[String]) -> Result<String> {
        self.tools.execute(name, args).await
    }
    
    /// Belleğe kaydet
    pub async fn store_memory(&self, key: &str, value: &str) -> Result<()> {
        if let Some(memory) = &self.memory {
            memory.store(key, value).await?;
        }
        Ok(())
    }
    
    /// Bellekten ara
    pub async fn search_memory(&self, query: &str, limit: usize) -> Result<Vec<String>> {
        if let Some(memory) = &self.memory {
            memory.search(query, limit).await
        } else {
            Ok(vec![])
        }
    }
    
    /// Model state'i al
    pub fn model_state(&self) -> Result<crate::types::ModelState> {
        self.model.state()
    }
    
    /// Yapılandırmayı al
    pub fn config(&self) -> &CevahirConfig {
        &self.config
    }
    
    /// Başlatıldı mı?
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }
    
    /// Modeli eval moduna al
    pub fn eval_mode(&self) -> Result<()> {
        self.model.eval_mode()
    }
    
    /// Modeli train moduna al
    pub fn train_mode(&self) -> Result<()> {
        self.model.train_mode()
    }
    
    /// Modeli kaydet
    pub fn save_model(&self, path: &str) -> Result<()> {
        self.model.save(path)
    }
    
    /// Model yükle
    pub fn load_model(&mut self, path: &str) -> Result<()> {
        self.model.load(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = CevahirConfig::default();
        assert_eq!(config.device, "cpu");
        assert_eq!(config.vocab_size, 60000);
    }
    
    #[test]
    fn test_strategy_auto_select() {
        assert_eq!(Strategy::auto_select("Merhaba"), Strategy::Direct);
        assert_eq!(Strategy::auto_select("Bu nasıl çalışır?"), Strategy::Think);
    }
}
