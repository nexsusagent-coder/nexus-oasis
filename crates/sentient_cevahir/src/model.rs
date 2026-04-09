//! Model wrapper - Neural Network için Rust arayüzü
//! 
//! Bu modül, Cevahir AI'ın V-7 neural network'ünü
//! Rust üzerinden kullanılabilir hale getirir.

use crate::{
    config::CevahirConfig,
    types::{DecodingConfig, GenerationOutput, ModelState, Strategy},
    error::{CevahirError, Result},
};
use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::sync::Arc;
use parking_lot::RwLock;

/// Neural Network model wrapper
pub struct ModelWrapper {
    /// Model instance
    model: Option<Py<PyAny>>,
    /// Yapılandırma
    config: CevahirConfig,
    /// Başlatıldı mı?
    initialized: bool,
}

impl ModelWrapper {
    /// Yeni model wrapper oluştur
    pub fn new(config: CevahirConfig) -> Result<Self> {
        Ok(Self {
            model: None,
            config,
            initialized: false,
        })
    }
    
    /// Modeli başlat
    pub fn initialize(&mut self) -> Result<()> {
        if self.initialized {
            return Ok(());
        }
        
        Python::with_gil(|py| {
            // Cevahir modülünü import et
            let cevahir_module = PyModule::import(py, "model.cevahir")
                .map_err(|e| CevahirError::PythonError(format!("Failed to import cevahir: {}", e)))?;
            
            // CevahirConfig oluştur
            let config_dict = PyDict::new(py);
            
            // Device config
            config_dict.set_item("device", &self.config.device)
                .map_err(|e| CevahirError::PythonError(e.to_string()))?;
            config_dict.set_item("log_level", &self.config.log_level)
                .map_err(|e| CevahirError::PythonError(e.to_string()))?;
            
            if let Some(seed) = self.config.seed {
                config_dict.set_item("seed", seed)
                    .map_err(|e| CevahirError::PythonError(e.to_string()))?;
            }
            
            // Tokenizer config
            let tokenizer_config = PyDict::new(py);
            tokenizer_config.set_item("vocab_path", &self.config.vocab_path)
                .map_err(|e| CevahirError::PythonError(e.to_string()))?;
            tokenizer_config.set_item("merges_path", &self.config.merges_path)
                .map_err(|e| CevahirError::PythonError(e.to_string()))?;
            config_dict.set_item("tokenizer", tokenizer_config)
                .map_err(|e| CevahirError::PythonError(e.to_string()))?;
            
            // Model config
            let model_config = PyDict::new(py);
            model_config.set_item("vocab_size", self.config.vocab_size)
                .map_err(|e| CevahirError::PythonError(e.to_string()))?;
            model_config.set_item("embed_dim", self.config.embed_dim)
                .map_err(|e| CevahirError::PythonError(e.to_string()))?;
            model_config.set_item("seq_proj_dim", self.config.seq_proj_dim)
                .map_err(|e| CevahirError::PythonError(e.to_string()))?;
            model_config.set_item("num_heads", self.config.num_heads)
                .map_err(|e| CevahirError::PythonError(e.to_string()))?;
            model_config.set_item("num_layers", self.config.num_layers)
                .map_err(|e| CevahirError::PythonError(e.to_string()))?;
            model_config.set_item("dropout", self.config.dropout)
                .map_err(|e| CevahirError::PythonError(e.to_string()))?;
            model_config.set_item("learning_rate", self.config.learning_rate)
                .map_err(|e| CevahirError::PythonError(e.to_string()))?;
            
            // V-4 features
            model_config.set_item("pe_mode", if self.config.use_rope { "rope" } else { "sinusoidal" })
                .map_err(|e| CevahirError::PythonError(e.to_string()))?;
            model_config.set_item("use_rmsnorm", self.config.use_rmsnorm)
                .map_err(|e| CevahirError::PythonError(e.to_string()))?;
            model_config.set_item("use_swiglu", self.config.use_swiglu)
                .map_err(|e| CevahirError::PythonError(e.to_string()))?;
            model_config.set_item("use_kv_cache", self.config.use_kv_cache)
                .map_err(|e| CevahirError::PythonError(e.to_string()))?;
            model_config.set_item("use_moe", self.config.use_moe)
                .map_err(|e| CevahirError::PythonError(e.to_string()))?;
            model_config.set_item("use_gradient_checkpointing", self.config.use_gradient_checkpointing)
                .map_err(|e| CevahirError::PythonError(e.to_string()))?;
            model_config.set_item("tie_weights", self.config.tie_weights)
                .map_err(|e| CevahirError::PythonError(e.to_string()))?;
            model_config.set_item("quantization_type", &self.config.quantization_type)
                .map_err(|e| CevahirError::PythonError(e.to_string()))?;
            
            // V-6 features
            model_config.set_item("use_pytorch_sdpa", self.config.use_pytorch_sdpa)
                .map_err(|e| CevahirError::PythonError(e.to_string()))?;
            model_config.set_item("use_qk_norm", self.config.use_qk_norm)
                .map_err(|e| CevahirError::PythonError(e.to_string()))?;
            model_config.set_item("parallel_residual", self.config.parallel_residual)
                .map_err(|e| CevahirError::PythonError(e.to_string()))?;
            model_config.set_item("logit_soft_cap", self.config.logit_soft_cap)
                .map_err(|e| CevahirError::PythonError(e.to_string()))?;
            
            // V-7 features
            model_config.set_item("drop_path_rate", self.config.drop_path_rate)
                .map_err(|e| CevahirError::PythonError(e.to_string()))?;
            
            if let Some(ffn_dim) = self.config.ffn_dim {
                model_config.set_item("ffn_dim", ffn_dim)
                    .map_err(|e| CevahirError::PythonError(e.to_string()))?;
            }
            if let Some(num_kv_heads) = self.config.num_kv_heads {
                model_config.set_item("num_kv_heads", num_kv_heads)
                    .map_err(|e| CevahirError::PythonError(e.to_string()))?;
            }
            if let Some(sliding_window) = self.config.sliding_window {
                model_config.set_item("sliding_window", sliding_window)
                    .map_err(|e| CevahirError::PythonError(e.to_string()))?;
            }
            if let Some(model_path) = &self.config.model_path {
                config_dict.set_item("load_model_path", model_path)
                    .map_err(|e| CevahirError::PythonError(e.to_string()))?;
            }
            
            config_dict.set_item("model", model_config)
                .map_err(|e| CevahirError::PythonError(e.to_string()))?;
            
            // CevahirConfig instance oluştur
            let cevahir_config = cevahir_module.call_method1("CevahirConfig", (config_dict,))
                .map_err(|e| CevahirError::PythonError(format!("Failed to create CevahirConfig: {}", e)))?;
            
            // Cevahir instance oluştur
            let cevahir = cevahir_module.call_method1("Cevahir", (cevahir_config,))
                .map_err(|e| CevahirError::PythonError(format!("Failed to create Cevahir: {}", e)))?;
            
            self.model = Some(cevahir.into());
            self.initialized = true;
            
            Ok(())
        })
    }
    
    /// Metin üret
    pub fn generate(
        &self,
        prompt: &str,
        max_new_tokens: usize,
    ) -> Result<String> {
        self.generate_with_config(prompt, DecodingConfig {
            max_new_tokens,
            ..Default::default()
        })
    }
    
    /// Decoding config ile üret
    pub fn generate_with_config(
        &self,
        prompt: &str,
        config: DecodingConfig,
    ) -> Result<String> {
        let start = std::time::Instant::now();
        
        Python::with_gil(|py| {
            let model = self.model.as_ref()
                .ok_or_else(|| CevahirError::ModelError("Model not initialized".into()))?;
            
            let model = model.as_ref(py);
            
            // generate() çağır
            let result = model.call_method1(
                "generate",
                (prompt, config.max_new_tokens, config.temperature, config.top_p, config.top_k)
            ).map_err(|e| CevahirError::PythonError(format!("generate() failed: {}", e)))?;
            
            let text: String = result.extract()
                .map_err(|e| CevahirError::PythonError(format!("Failed to extract text: {}", e)))?;
            
            Ok(text)
        })
    }
    
    /// Model state'i al
    pub fn state(&self) -> Result<ModelState> {
        if !self.initialized {
            return Ok(ModelState {
                initialized: false,
                device: self.config.device.clone(),
                vocab_size: self.config.vocab_size,
                embed_dim: self.config.embed_dim,
                num_layers: self.config.num_layers,
                num_heads: self.config.num_heads,
                param_count: 0,
            });
        }
        
        Python::with_gil(|py| {
            let model = self.model.as_ref()
                .ok_or_else(|| CevahirError::ModelError("Model not initialized".into()))?;
            
            let model = model.as_ref(py);
            
            // Parametre sayısını hesapla
            let param_count: usize = model.call_method0("count_parameters")
                .and_then(|p| p.extract())
                .unwrap_or(0);
            
            Ok(ModelState {
                initialized: true,
                device: self.config.device.clone(),
                vocab_size: self.config.vocab_size,
                embed_dim: self.config.embed_dim,
                num_layers: self.config.num_layers,
                num_heads: self.config.num_heads,
                param_count,
            })
        })
    }
    
    /// Modeli eval moduna al
    pub fn eval_mode(&self) -> Result<()> {
        Python::with_gil(|py| {
            let model = self.model.as_ref()
                .ok_or_else(|| CevahirError::ModelError("Model not initialized".into()))?;
            
            let model = model.as_ref(py);
            
            model.call_method0("eval")
                .map_err(|e| CevahirError::PythonError(format!("eval() failed: {}", e)))?;
            
            Ok(())
        })
    }
    
    /// Modeli train moduna al
    pub fn train_mode(&self) -> Result<()> {
        Python::with_gil(|py| {
            let model = self.model.as_ref()
                .ok_or_else(|| CevahirError::ModelError("Model not initialized".into()))?;
            
            let model = model.as_ref(py);
            
            model.call_method0("train")
                .map_err(|e| CevahirError::PythonError(format!("train() failed: {}", e)))?;
            
            Ok(())
        })
    }
    
    /// Modeli kaydet
    pub fn save(&self, path: &str) -> Result<()> {
        Python::with_gil(|py| {
            let model = self.model.as_ref()
                .ok_or_else(|| CevahirError::ModelError("Model not initialized".into()))?;
            
            let model = model.as_ref(py);
            
            model.call_method1("save", (path,))
                .map_err(|e| CevahirError::PythonError(format!("save() failed: {}", e)))?;
            
            Ok(())
        })
    }
    
    /// Model yükle
    pub fn load(&mut self, path: &str) -> Result<()> {
        if !self.initialized {
            self.initialize()?;
        }
        
        Python::with_gil(|py| {
            let model = self.model.as_ref()
                .ok_or_else(|| CevahirError::ModelError("Model not initialized".into()))?;
            
            let model = model.as_ref(py);
            
            model.call_method1("load", (path,))
                .map_err(|e| CevahirError::PythonError(format!("load() failed: {}", e)))?;
            
            Ok(())
        })
    }
    
    /// Python model referansını al (cognitive manager için)
    pub fn python_ref(&self) -> Option<&Py<PyAny>> {
        self.model.as_ref()
    }
    
    /// Başlatıldı mı?
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_new() {
        let config = CevahirConfig::default();
        let model = ModelWrapper::new(config);
        assert!(model.is_ok());
    }
}
