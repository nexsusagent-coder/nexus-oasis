//! Tokenizer wrapper - BPE tokenizer için Rust arayüzü
//! 
//! Bu modül, Cevahir AI'ın Türkçe optimizasyonlu BPE tokenizer'ını
//! Rust üzerinden kullanılabilir hale getirir.

use crate::{
    types::TokenizationResult,
    error::{CevahirError, Result},
};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use std::path::Path;

/// BPE Tokenizer wrapper
pub struct TokenizerWrapper {
    /// Python tokenizer instance
    tokenizer: Option<Py<PyAny>>,
    /// Vocabulary yolu
    vocab_path: String,
    /// Merges yolu
    merges_path: String,
    /// GPU kullanımı
    use_gpu: bool,
    /// Batch size
    batch_size: usize,
}

impl TokenizerWrapper {
    /// Yeni tokenizer oluştur
    pub fn new(vocab_path: &str, merges_path: &str) -> Result<Self> {
        Self::with_options(vocab_path, merges_path, false, 32)
    }
    
    /// GPU desteği ile tokenizer oluştur
    pub fn new_gpu(vocab_path: &str, merges_path: &str) -> Result<Self> {
        Self::with_options(vocab_path, merges_path, true, 64)
    }
    
    /// Seçeneklerle tokenizer oluştur
    pub fn with_options(
        vocab_path: &str,
        merges_path: &str,
        use_gpu: bool,
        batch_size: usize,
    ) -> Result<Self> {
        let mut wrapper = Self {
            tokenizer: None,
            vocab_path: vocab_path.to_string(),
            merges_path: merges_path.to_string(),
            use_gpu,
            batch_size,
        };
        
        wrapper.initialize()?;
        Ok(wrapper)
    }
    
    /// Python tokenizer'ı başlat
    pub fn initialize(&mut self) -> Result<()> {
        // Dosya varlığını kontrol et
        if !Path::new(&self.vocab_path).exists() {
            return Err(CevahirError::TokenizerError(format!(
                "Vocab file not found: {}", self.vocab_path
            )));
        }
        if !Path::new(&self.merges_path).exists() {
            return Err(CevahirError::TokenizerError(format!(
                "Merges file not found: {}", self.merges_path
            )));
        }
        
        Python::with_gil(|py| {
            let module = PyModule::import(py, "tokenizer_management.core.tokenizer_core")
                .map_err(|e| CevahirError::PythonError(format!("Failed to import tokenizer_core: {}", e)))?;
            
            // Config dict oluştur
            let config = PyDict::new(py);
            config.set_item("vocab_path", &self.vocab_path)
                .map_err(|e| CevahirError::PythonError(e.to_string()))?;
            config.set_item("merges_path", &self.merges_path)
                .map_err(|e| CevahirError::PythonError(e.to_string()))?;
            config.set_item("use_gpu", self.use_gpu)
                .map_err(|e| CevahirError::PythonError(e.to_string()))?;
            config.set_item("batch_size", self.batch_size)
                .map_err(|e| CevahirError::PythonError(e.to_string()))?;
            
            // TokenizerCore instance oluştur
            let tokenizer = module.call_method1("TokenizerCore", (config,))
                .map_err(|e| CevahirError::PythonError(format!("Failed to create TokenizerCore: {}", e)))?;
            
            self.tokenizer = Some(tokenizer.into());
            Ok(())
        })
    }
    
    /// Metin encode et
    pub fn encode(&self, text: &str) -> Result<TokenizationResult> {
        self.encode_with_mode(text, "inference")
    }
    
    /// Belirli modda encode et
    pub fn encode_with_mode(&self, text: &str, mode: &str) -> Result<TokenizationResult> {
        Python::with_gil(|py| {
            let tokenizer = self.tokenizer.as_ref()
                .ok_or_else(|| CevahirError::TokenizerError("Tokenizer not initialized".into()))?;
            
            let tokenizer = tokenizer.as_ref(py);
            
            // encode() çağır
            let result = tokenizer.call_method1("encode", (text, mode))
                .map_err(|e| CevahirError::PythonError(format!("encode() failed: {}", e)))?;
            
            // Sonucu parse et (tuple: (tokens, ids))
            let tokens: Vec<String> = result.get_item(0)
                .and_then(|t| t.extract())
                .map_err(|e| CevahirError::PythonError(format!("Failed to extract tokens: {}", e)))?;
            
            let ids: Vec<u32> = result.get_item(1)
                .and_then(|i| i.extract())
                .map_err(|e| CevahirError::PythonError(format!("Failed to extract ids: {}", e)))?;
            
            // UNK ratio hesapla
            let unk_count = ids.iter().filter(|&&id| id == 0).count();
            let unk_ratio = if !ids.is_empty() {
                unk_count as f32 / ids.len() as f32
            } else {
                0.0
            };
            
            Ok(TokenizationResult {
                tokens,
                ids,
                unk_ratio,
            })
        })
    }
    
    /// Token ID'leri decode et
    pub fn decode(&self, ids: &[u32]) -> Result<String> {
        self.decode_with_options(ids, "bpe", true)
    }
    
    /// Seçeneklerle decode et
    pub fn decode_with_options(
        &self,
        ids: &[u32],
        method: &str,
        remove_specials: bool,
    ) -> Result<String> {
        Python::with_gil(|py| {
            let tokenizer = self.tokenizer.as_ref()
                .ok_or_else(|| CevahirError::TokenizerError("Tokenizer not initialized".into()))?;
            
            let tokenizer = tokenizer.as_ref(py);
            
            // decode() çağır
            let result = tokenizer.call_method1(
                "decode",
                (ids.to_vec(), method, remove_specials)
            ).map_err(|e| CevahirError::PythonError(format!("decode() failed: {}", e)))?;
            
            let text: String = result.extract()
                .map_err(|e| CevahirError::PythonError(format!("Failed to extract text: {}", e)))?;
            
            Ok(text)
        })
    }
    
    /// Batch encode
    pub fn batch_encode(&self, texts: &[&str]) -> Result<Vec<TokenizationResult>> {
        Python::with_gil(|py| {
            let tokenizer = self.tokenizer.as_ref()
                .ok_or_else(|| CevahirError::TokenizerError("Tokenizer not initialized".into()))?;
            
            let tokenizer = tokenizer.as_ref(py);
            
            // batch_encode() çağır
            let result = tokenizer.call_method1("batch_encode", (texts.to_vec(),))
                .map_err(|e| CevahirError::PythonError(format!("batch_encode() failed: {}", e)))?;
            
            let list: &PyList = result.downcast()
                .map_err(|e| CevahirError::PythonError(format!("Result is not a list: {}", e)))?;
            
            let mut results = Vec::with_capacity(list.len());
            for item in list.iter() {
                let tokens: Vec<String> = item.get_item(0)
                    .and_then(|t| t.extract())
                    .map_err(|e| CevahirError::PythonError(e.to_string()))?;
                
                let ids: Vec<u32> = item.get_item(1)
                    .and_then(|i| i.extract())
                    .map_err(|e| CevahirError::PythonError(e.to_string()))?;
                
                let unk_count = ids.iter().filter(|&&id| id == 0).count();
                let unk_ratio = if !ids.is_empty() {
                    unk_count as f32 / ids.len() as f32
                } else {
                    0.0
                };
                
                results.push(TokenizationResult {
                    tokens,
                    ids,
                    unk_ratio,
                });
            }
            
            Ok(results)
        })
    }
    
    /// Vocabulary boyutunu al
    pub fn vocab_size(&self) -> Result<usize> {
        Python::with_gil(|py| {
            let tokenizer = self.tokenizer.as_ref()
                .ok_or_else(|| CevahirError::TokenizerError("Tokenizer not initialized".into()))?;
            
            let tokenizer = tokenizer.as_ref(py);
            let vocab = tokenizer.call_method0("get_vocab")
                .map_err(|e| CevahirError::PythonError(format!("get_vocab() failed: {}", e)))?;
            
            let size: usize = vocab.len();
            Ok(size)
        })
    }
    
    /// Vocabulary'i al
    pub fn get_vocab(&self) -> Result<std::collections::HashMap<String, u32>> {
        Python::with_gil(|py| {
            let tokenizer = self.tokenizer.as_ref()
                .ok_or_else(|| CevahirError::TokenizerError("Tokenizer not initialized".into()))?;
            
            let tokenizer = tokenizer.as_ref(py);
            let vocab = tokenizer.call_method0("get_vocab")
                .map_err(|e| CevahirError::PythonError(format!("get_vocab() failed: {}", e)))?;
            
            let dict: &PyDict = vocab.downcast()
                .map_err(|e| CevahirError::PythonError(format!("Vocab is not a dict: {}", e)))?;
            
            let mut result = std::collections::HashMap::new();
            for (key, value) in dict.iter() {
                let token: String = key.extract()
                    .map_err(|e| CevahirError::PythonError(e.to_string()))?;
                let id: u32 = value.extract()
                    .map_err(|e| CevahirError::PythonError(e.to_string()))?;
                result.insert(token, id);
            }
            
            Ok(result)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenizer_new() {
        // Dosya yoksa hata vermeli
        let result = TokenizerWrapper::new("nonexistent.json", "nonexistent.txt");
        assert!(result.is_err());
    }
}
