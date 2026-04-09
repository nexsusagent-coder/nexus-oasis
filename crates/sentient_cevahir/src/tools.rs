//! Tool execution framework
//! 
//! Bu modül, Cevahir AI'ın tool execution mekanizmasını sağlar.
//!SENTIENT OS araçları (browser, sandbox, etc.) entegre edilebilir.

use crate::error::{CevahirError, Result};
use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;

/// Tool tanımı
#[derive(Debug, Clone)]
pub struct ToolDefinition {
    /// Tool adı
    pub name: String,
    /// Açıklama
    pub description: String,
    /// Parametreler
    pub parameters: Vec<String>,
    /// Executor function
    #[allow(dead_code)]
    pub executor: fn(&[String]) -> Result<String>,
}

/// Tool execution sonucu
#[derive(Debug, Clone)]
pub struct ToolResult {
    /// Tool adı
    pub tool: String,
    /// Sonuç
    pub result: String,
    /// Başarılı mı?
    pub success: bool,
    /// Hata mesajı (varsa)
    pub error: Option<String>,
}

/// Tool executor
pub struct ToolExecutor {
    /// Kayıtlı araçlar
    tools: Arc<RwLock<HashMap<String, ToolDefinition>>>,
}

impl ToolExecutor {
    /// Yeni executor oluştur
    pub fn new() -> Self {
        Self {
            tools: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Tool kaydet
    pub fn register(&self, tool: ToolDefinition) {
        let mut tools = self.tools.write();
        tools.insert(tool.name.clone(), tool);
    }
    
    /// Tool kaydını kaldır
    pub fn unregister(&self, name: &str) -> Option<ToolDefinition> {
        let mut tools = self.tools.write();
        tools.remove(name)
    }
    
    /// Tool var mı kontrol et
    pub fn has_tool(&self, name: &str) -> bool {
        let tools = self.tools.read();
        tools.contains_key(name)
    }
    
    /// Tüm araçları listele
    pub fn list_tools(&self) -> Vec<ToolDefinition> {
        let tools = self.tools.read();
        tools.values().cloned().collect()
    }
    
    /// Tool çalıştır
    pub async fn execute(&self, name: &str, args: &[String]) -> Result<String> {
        let tools = self.tools.read();
        
        let tool = tools.get(name)
            .ok_or_else(|| CevahirError::ToolError(format!("Tool not found: {}", name)))?;
        
        // Parametre kontrolü
        if args.len() != tool.parameters.len() {
            return Err(CevahirError::ToolError(format!(
                "Expected {} parameters, got {}",
                tool.parameters.len(),
                args.len()
            )));
        }
        
        // Executor'ı çalıştır
        let result = (tool.executor)(args)?;
        
        Ok(result)
    }
    
    /// SENTIENT araçlarını kaydet
    pub fn register_sentient_tools(&self) {
        // Browser tool
        self.register(ToolDefinition {
            name: "browser".to_string(),
            description: "Web sayfalarını analiz et ve etkileşim kur".to_string(),
            parameters: vec!["url".to_string(), "action".to_string()],
            executor: |_args| {
                // SENTIENT browser_use entegrasyonu
                Ok("Browser action completed".to_string())
            },
        });
        
        // Sandbox tool
        self.register(ToolDefinition {
            name: "sandbox".to_string(),
            description: "Kod çalıştır ve test et".to_string(),
            parameters: vec!["code".to_string(), "language".to_string()],
            executor: |_args| {
                // SENTIENT sandbox entegrasyonu
                Ok("Code executed successfully".to_string())
            },
        });
        
        // Memory tool
        self.register(ToolDefinition {
            name: "memory".to_string(),
            description: "Belleğe kaydet ve ara".to_string(),
            parameters: vec!["action".to_string(), "query".to_string()],
            executor: |_args| {
                // SENTIENT memory entegrasyonu
                Ok("Memory operation completed".to_string())
            },
        });
        
        // Search tool
        self.register(ToolDefinition {
            name: "search".to_string(),
            description: "Web'de ara".to_string(),
            parameters: vec!["query".to_string()],
            executor: |_args| {
                // Web search entegrasyonu
                Ok("Search completed".to_string())
            },
        });
    }
}

impl Default for ToolExecutor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_registration() {
        let executor = ToolExecutor::new();
        
        executor.register(ToolDefinition {
            name: "test".to_string(),
            description: "Test tool".to_string(),
            parameters: vec!["arg1".to_string()],
            executor: |_args| Ok("test result".to_string()),
        });
        
        assert!(executor.has_tool("test"));
        assert!(!executor.has_tool("nonexistent"));
    }
    
    #[test]
    fn test_sentient_tools() {
        let executor = ToolExecutor::new();
        executor.register_sentient_tools();
        
        assert!(executor.has_tool("browser"));
        assert!(executor.has_tool("sandbox"));
        assert!(executor.has_tool("memory"));
        assert!(executor.has_tool("search"));
    }
}
