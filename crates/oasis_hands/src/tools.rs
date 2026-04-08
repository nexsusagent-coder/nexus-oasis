//! ═══════════════════════════════════════════════════════════════════════════════
//!  TOOLS - MASAÜSTÜ ARAÇLARI
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Dosya sistemi, process yönetimi ve diğer araçlar.

use crate::error::{HandsError, HandsResult};
use crate::sovereign::SovereignPolicy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ───────────────────────────────────────────────────────────────────────────────
//  ARAÇ TANIMLARI
// ───────────────────────────────────────────────────────────────────────────────

/// Masaüstü aracı
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesktopTool {
    /// Araç adı
    pub name: String,
    /// Açıklama
    pub description: String,
    /// Parametreler
    pub parameters: Vec<ToolParameter>,
    /// Kategori
    pub category: ToolCategory,
    /// Tehlike seviyesi
    pub risk_level: RiskLevel,
}

/// Araç parametresi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolParameter {
    pub name: String,
    pub param_type: String,
    pub required: bool,
    pub description: String,
}

/// Araç kategorisi
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ToolCategory {
    FileSystem,
    Process,
    Input,
    Screen,
    Network,
    System,
}

/// Risk seviyesi
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

/// Araç sonucu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    pub success: bool,
    pub output: String,
    pub data: Option<serde_json::Value>,
    pub error: Option<String>,
}

impl ToolResult {
    pub fn success(output: &str) -> Self {
        Self {
            success: true,
            output: output.into(),
            data: None,
            error: None,
        }
    }
    
    pub fn failure(error: &str) -> Self {
        Self {
            success: false,
            output: String::new(),
            data: None,
            error: Some(error.into()),
        }
    }
}

// ───────────────────────────────────────────────────────────────────────────────
//  ARAÇ KAYIT DEPOSU
// ───────────────────────────────────────────────────────────────────────────────

/// Araç kayıt deposu
pub struct ToolRegistry {
    /// Kayıtlı araçlar
    tools: HashMap<String, DesktopTool>,
    /// Sovereign policy
    policy: SovereignPolicy,
}

impl ToolRegistry {
    /// Yeni kayıt deposu oluştur
    pub fn new(policy: SovereignPolicy) -> Self {
        let mut registry = Self {
            tools: HashMap::new(),
            policy,
        };
        
        // Varsayılan araçları kaydet
        registry.register_default_tools();
        
        log::info!("🔧  TOOLS: {} araç kayıtlı", registry.tools.len());
        registry
    }
    
    /// Varsayılan araçları kaydet
    fn register_default_tools(&mut self) {
        // Dosya sistemi araçları
        self.register(DesktopTool {
            name: "read_file".into(),
            description: "Dosya oku (whitelist kontrolü)".into(),
            parameters: vec![
                ToolParameter {
                    name: "path".into(),
                    param_type: "string".into(),
                    required: true,
                    description: "Dosya yolu".into(),
                },
            ],
            category: ToolCategory::FileSystem,
            risk_level: RiskLevel::Medium,
        });
        
        self.register(DesktopTool {
            name: "write_file".into(),
            description: "Dosya yaz (whitelist + güvenlik kontrolü)".into(),
            parameters: vec![
                ToolParameter {
                    name: "path".into(),
                    param_type: "string".into(),
                    required: true,
                    description: "Dosya yolu".into(),
                },
                ToolParameter {
                    name: "content".into(),
                    param_type: "string".into(),
                    required: true,
                    description: "Dosya içeriği".into(),
                },
            ],
            category: ToolCategory::FileSystem,
            risk_level: RiskLevel::High,
        });
        
        self.register(DesktopTool {
            name: "list_directory".into(),
            description: "Dizin içeriğini listele".into(),
            parameters: vec![
                ToolParameter {
                    name: "path".into(),
                    param_type: "string".into(),
                    required: true,
                    description: "Dizin yolu".into(),
                },
            ],
            category: ToolCategory::FileSystem,
            risk_level: RiskLevel::Low,
        });
        
        // Process araçları
        self.register(DesktopTool {
            name: "execute_command".into(),
            description: "Komut çalıştır (whitelist uygulama)".into(),
            parameters: vec![
                ToolParameter {
                    name: "command".into(),
                    param_type: "string".into(),
                    required: true,
                    description: "Çalıştırılacak komut".into(),
                },
            ],
            category: ToolCategory::Process,
            risk_level: RiskLevel::High,
        });
        
        self.register(DesktopTool {
            name: "list_processes".into(),
            description: "Çalışan processleri listele".into(),
            parameters: vec![],
            category: ToolCategory::Process,
            risk_level: RiskLevel::Low,
        });
        
        // Ekran araçları
        self.register(DesktopTool {
            name: "capture_screen".into(),
            description: "Ekran görüntüsü al".into(),
            parameters: vec![
                ToolParameter {
                    name: "region".into(),
                    param_type: "object".into(),
                    required: false,
                    description: "Kırpma bölgesi (opsiyonel)".into(),
                },
            ],
            category: ToolCategory::Screen,
            risk_level: RiskLevel::Low,
        });
        
        self.register(DesktopTool {
            name: "find_ui_element".into(),
            description: "UI element bul (OCR + Vision)".into(),
            parameters: vec![
                ToolParameter {
                    name: "description".into(),
                    param_type: "string".into(),
                    required: true,
                    description: "Element açıklaması".into(),
                },
            ],
            category: ToolCategory::Screen,
            risk_level: RiskLevel::Low,
        });
        
        // Input araçları
        self.register(DesktopTool {
            name: "click".into(),
            description: "Fare tıklama".into(),
            parameters: vec![
                ToolParameter {
                    name: "x".into(),
                    param_type: "number".into(),
                    required: true,
                    description: "X koordinatı".into(),
                },
                ToolParameter {
                    name: "y".into(),
                    param_type: "number".into(),
                    required: true,
                    description: "Y koordinatı".into(),
                },
                ToolParameter {
                    name: "button".into(),
                    param_type: "string".into(),
                    required: false,
                    description: "left/right/middle".into(),
                },
            ],
            category: ToolCategory::Input,
            risk_level: RiskLevel::Medium,
        });
        
        self.register(DesktopTool {
            name: "type_text".into(),
            description: "Metin yaz".into(),
            parameters: vec![
                ToolParameter {
                    name: "text".into(),
                    param_type: "string".into(),
                    required: true,
                    description: "Yazılacak metin".into(),
                },
            ],
            category: ToolCategory::Input,
            risk_level: RiskLevel::Medium,
        });
        
        self.register(DesktopTool {
            name: "press_key".into(),
            description: "Tuş bas".into(),
            parameters: vec![
                ToolParameter {
                    name: "key".into(),
                    param_type: "string".into(),
                    required: true,
                    description: "Tuş adı".into(),
                },
            ],
            category: ToolCategory::Input,
            risk_level: RiskLevel::Low,
        });
    }
    
    /// Araç kaydet
    pub fn register(&mut self, tool: DesktopTool) {
        log::debug!("🔧  TOOLS: Kayıt → {} ({:?})", tool.name, tool.category);
        self.tools.insert(tool.name.clone(), tool);
    }
    
    /// Araç getir
    pub fn get(&self, name: &str) -> Option<&DesktopTool> {
        self.tools.get(name)
    }
    
    /// Araç çalıştır
    pub async fn execute(&self, name: &str, params: HashMap<String, serde_json::Value>) -> HandsResult<ToolResult> {
        let tool = self.tools.get(name).ok_or_else(|| {
            HandsError::Other(format!("OASIS-HANDS TOOLS: '{}' aracı bulunamadı", name))
        })?;
        
        log::info!("🔧  TOOLS: Çalıştır → {} (risk: {:?})", name, tool.risk_level);
        
        // Sovereign kontrolü
        self.validate_tool_execution(tool, &params)?;
        
        // Aracı çalıştır
        match name {
            "read_file" => self.execute_read_file(params).await,
            "write_file" => self.execute_write_file(params).await,
            "list_directory" => self.execute_list_directory(params).await,
            "execute_command" => self.execute_command(params).await,
            "list_processes" => self.execute_list_processes(params).await,
            "capture_screen" => self.execute_capture_screen(params).await,
            "find_ui_element" => self.execute_find_element(params).await,
            "click" => self.execute_click(params).await,
            "type_text" => self.execute_type_text(params).await,
            "press_key" => self.execute_press_key(params).await,
            _ => Ok(ToolResult::failure(&format!("Bilinmeyen araç: {}", name))),
        }
    }
    
    /// Araç doğrulama
    fn validate_tool_execution(&self, tool: &DesktopTool, params: &HashMap<String, serde_json::Value>) -> HandsResult<()> {
        // Risk seviyesi kontrolü
        match tool.risk_level {
            RiskLevel::Critical => {
                log::warn!("⚠️  OASIS-HANDS TOOLS: KRİTİK riskli araç → {}", tool.name);
                // Ek onay gerekebilir
            }
            RiskLevel::High => {
                log::info!("⚠️  OASIS-HANDS TOOLS: Yüksek riskli araç → {}", tool.name);
            }
            _ => {}
        }
        
        // Kategori bazlı kontroller
        match tool.category {
            ToolCategory::FileSystem => {
                if let Some(path) = params.get("path").and_then(|p| p.as_str()) {
                    let write = tool.name == "write_file";
                    self.policy.validate_file_access(path, write)?;
                }
            }
            ToolCategory::Process => {
                if let Some(cmd) = params.get("command").and_then(|c| c.as_str()) {
                    self.policy.validate_command(cmd)?;
                }
            }
            _ => {}
        }
        
        Ok(())
    }
    
    // ─── Araç Implementasyonları ───
    
    async fn execute_read_file(&self, params: HashMap<String, serde_json::Value>) -> HandsResult<ToolResult> {
        let path = params.get("path").and_then(|p| p.as_str()).unwrap_or("");
        log::info!("📄  TOOLS: Dosya okunuyor → {}", path);
        Ok(ToolResult::success(&format!("Mock dosya içeriği: {}", path)))
    }
    
    async fn execute_write_file(&self, params: HashMap<String, serde_json::Value>) -> HandsResult<ToolResult> {
        let path = params.get("path").and_then(|p| p.as_str()).unwrap_or("");
        log::info!("📝  TOOLS: Dosya yazılıyor → {}", path);
        Ok(ToolResult::success("Dosya başarıyla yazıldı"))
    }
    
    async fn execute_list_directory(&self, params: HashMap<String, serde_json::Value>) -> HandsResult<ToolResult> {
        let path = params.get("path").and_then(|p| p.as_str()).unwrap_or("");
        log::info!("📁  TOOLS: Dizin listeleniyor → {}", path);
        Ok(ToolResult::success("Mock dizin içeriği"))
    }
    
    async fn execute_command(&self, params: HashMap<String, serde_json::Value>) -> HandsResult<ToolResult> {
        let command = params.get("command").and_then(|c| c.as_str()).unwrap_or("");
        
        // Uygulama whitelist kontrolü
        let app = command.split_whitespace().next().unwrap_or("");
        self.policy.validate_application(app)?;
        
        log::info!("⚡  TOOLS: Komut çalıştırılıyor → {}", command);
        Ok(ToolResult::success("Mock komut çıktısı"))
    }
    
    async fn execute_list_processes(&self, _params: HashMap<String, serde_json::Value>) -> HandsResult<ToolResult> {
        log::info!("📊  TOOLS: Processler listeleniyor");
        Ok(ToolResult::success("Mock process listesi"))
    }
    
    async fn execute_capture_screen(&self, _params: HashMap<String, serde_json::Value>) -> HandsResult<ToolResult> {
        log::info!("🖼️  TOOLS: Ekran görüntüsü alınıyor");
        Ok(ToolResult::success("base64_mock_image_data"))
    }
    
    async fn execute_find_element(&self, params: HashMap<String, serde_json::Value>) -> HandsResult<ToolResult> {
        let desc = params.get("description").and_then(|d| d.as_str()).unwrap_or("");
        log::info!("🔍  TOOLS: Element aranıyor → {}", desc);
        Ok(ToolResult::success("Element bulundu: (100, 200)"))
    }
    
    async fn execute_click(&self, params: HashMap<String, serde_json::Value>) -> HandsResult<ToolResult> {
        let x = params.get("x").and_then(|v| v.as_i64()).unwrap_or(0);
        let y = params.get("y").and_then(|v| v.as_i64()).unwrap_or(0);
        log::info!("🖱️  TOOLS: Tıklama → ({}, {})", x, y);
        Ok(ToolResult::success("Tıklama başarılı"))
    }
    
    async fn execute_type_text(&self, params: HashMap<String, serde_json::Value>) -> HandsResult<ToolResult> {
        let text = params.get("text").and_then(|t| t.as_str()).unwrap_or("");
        log::info!("⌨️  TOOLS: Metin yazılıyor ({} karakter)", text.len());
        Ok(ToolResult::success("Metin yazıldı"))
    }
    
    async fn execute_press_key(&self, params: HashMap<String, serde_json::Value>) -> HandsResult<ToolResult> {
        let key = params.get("key").and_then(|k| k.as_str()).unwrap_or("");
        log::info!("⌨️  TOOLS: Tuş bas → {}", key);
        Ok(ToolResult::success("Tuş basıldı"))
    }
    
    /// Tüm araçları listele
    pub fn list(&self) -> Vec<&DesktopTool> {
        self.tools.values().collect()
    }
    
    /// Kategoriye göre araçları listele
    pub fn list_by_category(&self, category: ToolCategory) -> Vec<&DesktopTool> {
        self.tools.values().filter(|t| t.category == category).collect()
    }
    
    /// Toplam araç sayısı
    pub fn count(&self) -> usize {
        self.tools.len()
    }
}

// ───────────────────────────────────────────────────────────────────────────────
//  TESTS
// ───────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_tool_registry_creation() {
        let policy = SovereignPolicy::strict();
        let registry = ToolRegistry::new(policy);
        assert!(registry.count() > 0);
    }
    
    #[test]
    fn test_tool_result_success() {
        let result = ToolResult::success("Test çıktısı");
        assert!(result.success);
        assert!(result.error.is_none());
    }
    
    #[test]
    fn test_tool_result_failure() {
        let result = ToolResult::failure("Hata mesajı");
        assert!(!result.success);
        assert!(result.error.is_some());
    }
    
    #[test]
    fn test_get_tool() {
        let policy = SovereignPolicy::strict();
        let registry = ToolRegistry::new(policy);
        assert!(registry.get("read_file").is_some());
        assert!(registry.get("unknown_tool").is_none());
    }
    
    #[test]
    fn test_list_by_category() {
        let policy = SovereignPolicy::strict();
        let registry = ToolRegistry::new(policy);
        let fs_tools = registry.list_by_category(ToolCategory::FileSystem);
        assert!(fs_tools.len() > 0);
    }
    
    #[tokio::test]
    async fn test_execute_unknown_tool() {
        let policy = SovereignPolicy::strict();
        let registry = ToolRegistry::new(policy);
        let result = registry.execute("unknown", HashMap::new()).await;
        assert!(result.is_err());
    }
}
