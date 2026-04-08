//! ═══════════════════════════════════════════════════════════════════════════════
//!  CONFIG TOOL - Yapılandırma Yönetimi
//! ═══════════════════════════════════════════════════════════════════════════════

use crate::sentient_tool::{SentientTool, SentientToolResult, RiskLevel, ToolCategory, ToolParameter};
use async_trait::async_trait;
use std::collections::HashMap;
use std::path::PathBuf;

/// Config Tool - SENTIENT yapılandırma yönetimi
pub struct ConfigTool {
    config_dir: PathBuf,
    config: HashMap<String, serde_json::Value>,
}

impl ConfigTool {
    pub fn new(config_dir: PathBuf) -> Self {
        let config = Self::load_config(&config_dir).unwrap_or_default();
        Self { config_dir, config }
    }
    
    pub fn default_tool() -> Self {
        Self::new(PathBuf::from("data"))
    }
    
    fn load_config(dir: &PathBuf) -> Option<HashMap<String, serde_json::Value>> {
        let config_file = dir.join("sentient_config.json");
        if config_file.exists() {
            let content = std::fs::read_to_string(config_file).ok()?;
            serde_json::from_str(&content).ok()
        } else {
            None
        }
    }
    
    fn save_config(&self) -> bool {
        let config_file = self.config_dir.join("sentient_config.json");
        if let Ok(content) = serde_json::to_string_pretty(&self.config) {
            std::fs::write(config_file, content).is_ok()
        } else {
            false
        }
    }
    
    /// Değer al
    pub fn get(&self, key: &str) -> Option<&serde_json::Value> {
        self.config.get(key)
    }
    
    /// Değer ayarla
    pub fn set(&mut self, key: &str, value: serde_json::Value) -> bool {
        self.config.insert(key.to_string(), value);
        self.save_config()
    }
    
    /// Anahtar sil
    pub fn delete(&mut self, key: &str) -> bool {
        self.config.remove(key).is_some() && self.save_config()
    }
    
    /// Tüm anahtarları listele
    pub fn list_keys(&self) -> Vec<String> {
        self.config.keys().cloned().collect()
    }
    
    /// Varsayılan yapılandırma
    pub fn defaults() -> HashMap<String, serde_json::Value> {
        let mut defaults = HashMap::new();
        
        // V-GATE
        defaults.insert("vgate.url".to_string(), serde_json::json!("http://127.0.0.1:1071"));
        defaults.insert("vgate.timeout_secs".to_string(), serde_json::json!(30));
        defaults.insert("vgate.max_retries".to_string(), serde_json::json!(3));
        
        // LLM
        defaults.insert("llm.default_provider".to_string(), serde_json::json!("openrouter"));
        defaults.insert("llm.default_model".to_string(), serde_json::json!("qwen/qwen3-coder:free"));
        defaults.insert("llm.max_tokens".to_string(), serde_json::json!(4096));
        defaults.insert("llm.temperature".to_string(), serde_json::json!(0.7));
        
        // Memory
        defaults.insert("memory.enabled".to_string(), serde_json::json!(true));
        defaults.insert("memory.max_entries".to_string(), serde_json::json!(10000));
        defaults.insert("memory.consolidation_interval_secs".to_string(), serde_json::json!(3600));
        
        // Tools
        defaults.insert("tools.bash_timeout_secs".to_string(), serde_json::json!(120));
        defaults.insert("tools.max_file_size_mb".to_string(), serde_json::json!(10));
        defaults.insert("tools.max_concurrent".to_string(), serde_json::json!(4));
        
        // Security
        defaults.insert("security.whitelist_mode".to_string(), serde_json::json!(true));
        defaults.insert("security.require_confirmation".to_string(), serde_json::json!(true));
        defaults.insert("security.audit_log".to_string(), serde_json::json!(true));
        
        // Dashboard
        defaults.insert("dashboard.port".to_string(), serde_json::json!(8080));
        defaults.insert("dashboard.theme".to_string(), serde_json::json!("dark"));
        
        defaults
    }
}

#[async_trait]
impl SentientTool for ConfigTool {
    fn name(&self) -> &str { "config" }
    
    fn description(&self) -> &str {
        "SENTIENT yapılandırma yönetimi. Ayarları oku, yaz, sil, listele."
    }
    
    fn category(&self) -> ToolCategory { ToolCategory::System }
    
    fn risk_level(&self) -> RiskLevel { RiskLevel::Medium }
    
    fn parameters(&self) -> Vec<ToolParameter> {
        vec![
            ToolParameter::new("action", "string", true, "Aksiyon: get, set, delete, list, defaults, reset"),
            ToolParameter::new("key", "string", false, "Yapılandırma anahtarı"),
            ToolParameter::new("value", "any", false, "Değer (set için)"),
        ]
    }
    
    async fn execute(&self, params: HashMap<String, serde_json::Value>) -> SentientToolResult {
        let action = params.get("action")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        
        match action {
            "get" => {
                let key = params.get("key").and_then(|v| v.as_str()).unwrap_or("");
                
                if key.is_empty() {
                    return SentientToolResult::failure("Anahtar belirtilmedi");
                }
                
                // Nokta ile ayrılmış key'leri destekle
                if let Some(value) = self.config.get(key) {
                    SentientToolResult::success_with_data(
                        &format!("{} = {:?}", key, value),
                        serde_json::json!({
                            "key": key,
                            "value": value
                        })
                    )
                } else {
                    SentientToolResult::failure(&format!("Anahtar bulunamadı: {}", key))
                }
            }
            "set" => {
                let key = params.get("key").and_then(|v| v.as_str()).unwrap_or("");
                let value = params.get("value").cloned().unwrap_or(serde_json::json!(null));
                
                if key.is_empty() {
                    return SentientToolResult::failure("Anahtar belirtilmedi");
                }
                
                // Self'i mutate edemiyoruz, clone yapalım
                let mut this = self.clone();
                if this.set(key, value.clone()) {
                    SentientToolResult::success_with_data(
                        &format!("{} ayarlandı", key),
                        serde_json::json!({
                            "key": key,
                            "value": value
                        })
                    )
                } else {
                    SentientToolResult::failure("Yapılandırma kaydedilemedi")
                }
            }
            "delete" => {
                let key = params.get("key").and_then(|v| v.as_str()).unwrap_or("");
                
                if key.is_empty() {
                    return SentientToolResult::failure("Anahtar belirtilmedi");
                }
                
                let mut this = self.clone();
                if this.delete(key) {
                    SentientToolResult::success(&format!("Anahtar silindi: {}", key))
                } else {
                    SentientToolResult::failure(&format!("Anahtar bulunamadı: {}", key))
                }
            }
            "list" => {
                let keys = self.list_keys();
                SentientToolResult::success_with_data(
                    &format!("{} yapılandırma anahtarı", keys.len()),
                    serde_json::json!({
                        "count": keys.len(),
                        "keys": keys,
                        "config": self.config
                    })
                )
            }
            "defaults" => {
                let defaults = Self::defaults();
                SentientToolResult::success_with_data(
                    "Varsayılan yapılandırma",
                    serde_json::json!({
                        "defaults": defaults
                    })
                )
            }
            "reset" => {
                let mut this = self.clone();
                this.config = Self::defaults();
                if this.save_config() {
                    SentientToolResult::success("Yapılandırma sıfırlandı")
                } else {
                    SentientToolResult::failure("Yapılandırma sıfırlanamadı")
                }
            }
            _ => SentientToolResult::failure(&format!("Bilinmeyen aksiyon: {}", action))
        }
    }
}

impl Clone for ConfigTool {
    fn clone(&self) -> Self {
        Self {
            config_dir: self.config_dir.clone(),
            config: self.config.clone(),
        }
    }
}

impl Default for ConfigTool {
    fn default() -> Self {
        Self::default_tool()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_defaults() {
        let defaults = ConfigTool::defaults();
        assert!(defaults.contains_key("vgate.url"));
        assert!(defaults.contains_key("llm.default_model"));
    }
}
