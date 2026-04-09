//! Cognitive strateji yönetimi
//! 
//! Bu modül, Cevahir AI'ın bilişsel stratejilerini yönetir:
//! - Direct: Doğrudan yanıt
//! - Think: İç ses üretimi
//! - Debate: Çoklu perspektif
//! - TreeOfThoughts: Ağaç yapısında düşünme

use crate::{
    types::{CognitiveResult, DecodingConfig, Strategy, ThoughtStep, ToolCall},
    error::{CevahirError, Result},
    config::CevahirConfig,
};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};

/// Cognitive strateji yöneticisi
pub struct CognitiveManager {
    config: CevahirConfig,
    python_module: Option<Py<PyAny>>,
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
            python_module: None,
            strategies,
        })
    }
    
    /// Python modülünü başlat
    pub fn initialize(&mut self) -> Result<()> {
        Python::with_gil(|py| {
            let module = PyModule::import(py, "cognitive_management.cognitive_manager")
                .map_err(|e| CevahirError::PythonError(format!("Failed to import cognitive_manager: {}", e)))?;
            
            self.python_module = Some(module.into());
            Ok(())
        })
    }
    
    /// Mevcut stratejileri listele
    pub fn available_strategies(&self) -> &[Strategy] {
        &self.strategies
    }
    
    /// Strateji seç ve çalıştır
    pub fn process(
        &self,
        input: &str,
        strategy: Strategy,
        model_api: &Py<PyAny>,
    ) -> Result<CognitiveResult> {
        let start = std::time::Instant::now();
        
        Python::with_gil(|py| {
            let module = self.python_module.as_ref()
                .ok_or_else(|| CevahirError::CognitiveError("CognitiveManager not initialized".into()))?;
            
            let module = module.as_ref(py);
            
            // CognitiveManager instance oluştur
            let cfg = PyDict::new(py);
            cfg.set_item("policy.default_strategy", strategy.as_str())
                .map_err(|e| CevahirError::PythonError(e.to_string()))?;
            
            let manager = module.call_method1(
                "CognitiveManager",
                (model_api, cfg)
            ).map_err(|e| CevahirError::PythonError(format!("Failed to create CognitiveManager: {}", e)))?;
            
            // CognitiveInput oluştur
            let cognitive_input = PyDict::new(py);
            cognitive_input.set_item("message", input)
                .map_err(|e| CevahirError::PythonError(e.to_string()))?;
            cognitive_input.set_item("strategy", strategy.as_str())
                .map_err(|e| CevahirError::PythonError(e.to_string()))?;
            
            // DecodingConfig
            let decoding_config = PyDict::new(py);
            decoding_config.set_item("max_new_tokens", self.config.max_thinking_steps * 100)
                .map_err(|e| CevahirError::PythonError(e.to_string()))?;
            decoding_config.set_item("temperature", 0.7)
                .map_err(|e| CevahirError::PythonError(e.to_string()))?;
            
            cognitive_input.set_item("decoding_config", decoding_config)
                .map_err(|e| CevahirError::PythonError(e.to_string()))?;
            
            // handle() çağır
            let output = manager.call_method1("handle", (cognitive_input,))
                .map_err(|e| CevahirError::PythonError(format!("handle() failed: {}", e)))?;
            
            // Sonucu parse et
            let response = output.getattr("response")
                .and_then(|r| r.extract::<String>())
                .unwrap_or_default();
            
            let confidence = output.getattr("confidence")
                .and_then(|c| c.extract::<f32>())
                .unwrap_or(0.5);
            
            // Thought steps
            let thoughts = output.getattr("thoughts")
                .ok()
                .and_then(|t| {
                    t.downcast::<PyList>().ok().map(|list| {
                        list.iter()
                            .enumerate()
                            .filter_map(|(i, item)| {
                                let content = item.getattr("content")
                                    .and_then(|c| c.extract::<String>())
                                    .unwrap_or_default();
                                let result = item.getattr("result")
                                    .ok()
                                    .and_then(|r| r.extract::<String>().ok());
                                
                                Some(ThoughtStep {
                                    step: i,
                                    content,
                                    result,
                                })
                            })
                            .collect()
                    })
                });
            
            // Tool calls
            let tool_calls = output.getattr("tool_calls")
                .ok()
                .and_then(|t| {
                    t.downcast::<PyList>().ok().map(|list| {
                        list.iter()
                            .filter_map(|item| {
                                let tool = item.getattr("tool")
                                    .and_then(|t| t.extract::<String>())
                                    .unwrap_or_default();
                                let args = item.getattr("args")
                                    .and_then(|a| a.extract::<Vec<String>>())
                                    .unwrap_or_default();
                                let result = item.getattr("result")
                                    .and_then(|r| r.extract::<String>())
                                    .unwrap_or_default();
                                
                                Some(ToolCall { tool, args, result })
                            })
                            .collect()
                    })
                });
            
            Ok(CognitiveResult {
                response,
                strategy,
                thoughts,
                tool_calls,
                memory_access: None,
                confidence,
            })
        })
    }
    
    /// Otomatik strateji seçimi ile işlem
    pub fn process_auto(
        &self,
        input: &str,
        model_api: &Py<PyAny>,
    ) -> Result<CognitiveResult> {
        let strategy = Strategy::auto_select(input);
        self.process(input, strategy, model_api)
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strategy_auto_select() {
        assert_eq!(Strategy::auto_select("Merhaba"), Strategy::Direct);
        assert_eq!(Strategy::auto_select("Bu nasıl çalışır?"), Strategy::Think);
        assert_eq!(Strategy::auto_select("Avantaj ve dezavantajları neler?"), Strategy::Debate);
        assert_eq!(Strategy::auto_select("Bu hatayı çöz"), Strategy::TreeOfThoughts);
    }
    
    #[test]
    fn test_strategy_as_str() {
        assert_eq!(Strategy::Direct.as_str(), "direct");
        assert_eq!(Strategy::Think.as_str(), "think");
        assert_eq!(Strategy::Debate.as_str(), "debate");
        assert_eq!(Strategy::TreeOfThoughts.as_str(), "tree_of_thoughts");
    }
}
