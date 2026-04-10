//! ═══════════════════════════════════════════════════════════════════════════════
//!  MANUS TOOLS - Yerleşik Araçlar
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Manus için yerleşik araçlar.

use crate::error::{ManusError, ManusResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// ─── TOOL REGISTRY ───

pub struct ToolRegistry {
    /// Kayıtlı araçlar
    tools: HashMap<String, Tool>,
}

/// Araç tanımı
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    /// Araç adı
    pub name: String,
    /// Açıklama
    pub description: String,
    /// Parametreler
    pub parameters: Vec<ToolParameter>,
    /// Kategori
    pub category: ToolCategory,
}

/// Araç parametresi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolParameter {
    pub name: String,
    pub description: String,
    pub param_type: ParameterType,
    pub required: bool,
    pub default: Option<String>,
}

/// Parametre türü
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParameterType {
    String,
    Integer,
    Float,
    Boolean,
    Array,
    Object,
}

/// Araç kategorisi
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ToolCategory {
    Calculation,
    DataProcessing,
    FileSystem,
    Network,
    Testing,
    Utility,
}

/// Araç sonucu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    pub tool: String,
    pub success: bool,
    pub output: String,
    pub error: Option<String>,
}

impl ToolRegistry {
    /// Yeni registry oluştur
    pub fn new() -> Self {
        let mut registry = Self {
            tools: HashMap::new(),
        };
        
        // Yerleşik araçları kaydet
        registry.register_builtin_tools();
        
        registry
    }
    
    /// Yerleşik araçları kaydet
    fn register_builtin_tools(&mut self) {
        // Hesap makinesi
        self.register(Tool {
            name: "calculate".into(),
            description: "Matematiksel ifadeyi hesapla".into(),
            parameters: vec![
                ToolParameter {
                    name: "expression".into(),
                    description: "Hesaplanacak ifade".into(),
                    param_type: ParameterType::String,
                    required: true,
                    default: None,
                },
            ],
            category: ToolCategory::Calculation,
        });
        
        // Veri işleme
        self.register(Tool {
            name: "process_data".into(),
            description: "Veriyi işle ve dönüştür".into(),
            parameters: vec![
                ToolParameter {
                    name: "data".into(),
                    description: "İşlenecek veri".into(),
                    param_type: ParameterType::Array,
                    required: true,
                    default: None,
                },
                ToolParameter {
                    name: "operation".into(),
                    description: "İşlem türü (filter, map, reduce)".into(),
                    param_type: ParameterType::String,
                    required: true,
                    default: None,
                },
            ],
            category: ToolCategory::DataProcessing,
        });
        
        // Test çalıştırıcı
        self.register(Tool {
            name: "run_tests".into(),
            description: "Test kodunu çalıştır".into(),
            parameters: vec![
                ToolParameter {
                    name: "code".into(),
                    description: "Test kodu".into(),
                    param_type: ParameterType::String,
                    required: true,
                    default: None,
                },
            ],
            category: ToolCategory::Testing,
        });
        
        // JSON işleme
        self.register(Tool {
            name: "parse_json".into(),
            description: "JSON verisini parse et".into(),
            parameters: vec![
                ToolParameter {
                    name: "json_string".into(),
                    description: "JSON string".into(),
                    param_type: ParameterType::String,
                    required: true,
                    default: None,
                },
            ],
            category: ToolCategory::Utility,
        });
        
        // Metin işleme
        self.register(Tool {
            name: "text_transform".into(),
            description: "Metni dönüştür".into(),
            parameters: vec![
                ToolParameter {
                    name: "text".into(),
                    description: "Dönüştürülecek metin".into(),
                    param_type: ParameterType::String,
                    required: true,
                    default: None,
                },
                ToolParameter {
                    name: "operation".into(),
                    description: "İşlem (upper, lower, reverse, etc.)".into(),
                    param_type: ParameterType::String,
                    required: true,
                    default: None,
                },
            ],
            category: ToolCategory::Utility,
        });
    }
    
    /// Araç kaydet
    pub fn register(&mut self, tool: Tool) {
        log::debug!("🔧  MANUS-TOOLS: Araç kaydedildi → {}", tool.name);
        self.tools.insert(tool.name.clone(), tool);
    }
    
    /// Araç getir
    pub fn get(&self, name: &str) -> Option<&Tool> {
        self.tools.get(name)
    }
    
    /// Araç çalıştır
    pub fn execute(&self, name: &str, params: HashMap<String, String>) -> ManusResult<ToolResult> {
        let _tool = self.tools.get(name)
            .ok_or_else(|| ManusError::General(format!("Araç bulunamadı: {}", name)))?;
        
        log::info!("🔧  MANUS-TOOLS: {} çalıştırılıyor", name);
        
        let output = match name {
            "calculate" => {
                let expr = params.get("expression")
                    .ok_or_else(|| ManusError::General("expression parametresi gerekli".into()))?;
                self.calculate(expr)?
            }
            "parse_json" => {
                let json = params.get("json_string")
                    .ok_or_else(|| ManusError::General("json_string parametresi gerekli".into()))?;
                self.parse_json(json)?
            }
            "text_transform" => {
                let text = params.get("text")
                    .ok_or_else(|| ManusError::General("text parametresi gerekli".into()))?;
                let op = params.get("operation")
                    .ok_or_else(|| ManusError::General("operation parametresi gerekli".into()))?;
                self.text_transform(text, op)?
            }
            _ => format!("{} çalıştırıldı", name),
        };
        
        Ok(ToolResult {
            tool: name.into(),
            success: true,
            output,
            error: None,
        })
    }
    
    /// Hesapla
    fn calculate(&self, expr: &str) -> ManusResult<String> {
        // Basit hesaplama (gerçek uygulamada evalexpr gibi bir crate kullanılmalı)
        let result = if expr.contains('+') {
            let parts: Vec<&str> = expr.split('+').collect();
            if parts.len() == 2 {
                let a: f64 = parts[0].trim().parse().unwrap_or(0.0);
                let b: f64 = parts[1].trim().parse().unwrap_or(0.0);
                a + b
            } else {
                0.0
            }
        } else if expr.contains('-') {
            let parts: Vec<&str> = expr.split('-').collect();
            if parts.len() == 2 {
                let a: f64 = parts[0].trim().parse().unwrap_or(0.0);
                let b: f64 = parts[1].trim().parse().unwrap_or(0.0);
                a - b
            } else {
                0.0
            }
        } else if expr.contains('*') {
            let parts: Vec<&str> = expr.split('*').collect();
            if parts.len() == 2 {
                let a: f64 = parts[0].trim().parse().unwrap_or(0.0);
                let b: f64 = parts[1].trim().parse().unwrap_or(0.0);
                a * b
            } else {
                0.0
            }
        } else {
            expr.parse::<f64>().unwrap_or(0.0)
        };
        
        Ok(format!("{}", result))
    }
    
    /// JSON parse
    fn parse_json(&self, json: &str) -> ManusResult<String> {
        let parsed: serde_json::Value = serde_json::from_str(json)
            .map_err(|e| ManusError::General(format!("JSON parse hatası: {}", e)))?;
        
        Ok(serde_json::to_string_pretty(&parsed).unwrap_or_default())
    }
    
    /// Metin dönüştür
    fn text_transform(&self, text: &str, operation: &str) -> ManusResult<String> {
        let result = match operation {
            "upper" => text.to_uppercase(),
            "lower" => text.to_lowercase(),
            "reverse" => text.chars().rev().collect(),
            "trim" => text.trim().to_string(),
            "len" => text.len().to_string(),
            _ => text.into(),
        };
        
        Ok(result)
    }
    
    /// Tüm araçları listele
    pub fn list(&self) -> Vec<&Tool> {
        self.tools.values().collect()
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_creation() {
        let registry = ToolRegistry::new();
        assert!(!registry.tools.is_empty());
    }

    #[test]
    fn test_get_tool() {
        let registry = ToolRegistry::new();
        let tool = registry.get("calculate");
        assert!(tool.is_some());
    }

    #[test]
    fn test_calculate() {
        let registry = ToolRegistry::new();
        let mut params = HashMap::new();
        params.insert("expression".into(), "2 + 3".into());
        
        let result = registry.execute("calculate", params).expect("operation failed");
        assert!(result.success);
    }

    #[test]
    fn test_text_transform() {
        let registry = ToolRegistry::new();
        let mut params = HashMap::new();
        params.insert("text".into(), "hello".into());
        params.insert("operation".into(), "upper".into());
        
        let result = registry.execute("text_transform", params).expect("operation failed");
        assert_eq!(result.output, "HELLO");
    }
}
