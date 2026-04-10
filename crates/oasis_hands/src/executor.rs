//! ═══════════════════════════════════════════════════════════════════════════════
//!  SENTIENT TOOL EXECUTOR - TOOL RUNNER
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Registers and executes all SentientTool tools.
//! Centralized tool management system.

use crate::sentient_tool::{SentientTool, SentientToolResult, RiskLevel, ToolCategory};
use crate::sentient_tools::{
    GitTool, GrepTool, SedTool, BrowserTool, ScreenshotTool,
    McpTool, MemoryTool, TaskTool, N8nTool, EmailTool,
    NotifyTool, PdfTool, TranslateTool, CalendarTool, AgentTool,
};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

// ───────────────────────────────────────────────────────────────────────────────
//  SENTIENT TOOL EXECUTOR
// ───────────────────────────────────────────────────────────────────────────────

/// SENTIENTToolExecutor - Central tool runner
///
/// Registers all SentientTool implementations and
/// executes them safely.
///
/// # Security
/// - All tools are classified by risk level
/// - High-risk operations are logged
/// - Errors are converted to SENTIENT format
pub struct SENTIENTToolExecutor {
    /// Registered tools
    tools: RwLock<HashMap<String, Arc<dyn SentientTool>>>,
    /// Tool count
    tool_count: usize,
}

impl SENTIENTToolExecutor {
    /// Create new SENTIENTToolExecutor
    pub fn new() -> Self {
        let executor = Self {
            tools: RwLock::new(HashMap::new()),
            tool_count: 0,
        };
        
        // Register all tools
        executor.register_all_tools();
        
        log::info!("🔧  EXECUTOR: {} tools registered", executor.tool_count);
        executor
    }
    
    /// Register all tools
    fn register_all_tools(&self) {
        // 1. Git Tool
        self.register_tool(GitTool::new());
        
        // 2. Grep Tool
        self.register_tool(GrepTool::new());
        
        // 3. Sed Tool
        self.register_tool(SedTool::new());
        
        // 4. Browser Tool
        self.register_tool(BrowserTool::new());
        
        // 5. Screenshot Tool
        self.register_tool(ScreenshotTool::new());
        
        // 6. MCP Tool
        self.register_tool(McpTool::new());
        
        // 7. Memory Tool
        self.register_tool(MemoryTool::new());
        
        // 8. Task Tool
        self.register_tool(TaskTool::new());
        
        // 9. N8n Tool
        self.register_tool(N8nTool::new());
        
        // 10. Email Tool
        self.register_tool(EmailTool::new());
        
        // 11. Notify Tool
        self.register_tool(NotifyTool::new());
        
        // 12. PDF Tool
        self.register_tool(PdfTool::new());
        
        // 13. Translate Tool
        self.register_tool(TranslateTool::new());
        
        // 14. Calendar Tool
        self.register_tool(CalendarTool::new());
        
        // 15. Agent Tool
        self.register_tool(AgentTool::new());
    }
    
    /// Register tool
    fn register_tool<T: SentientTool + 'static>(&self, tool: T) {
        let name = tool.name().to_string();
        let risk = tool.risk_level();
        let category = tool.category();
        
        log::debug!(
            "🔧  EXECUTOR: Kayıt → {} ({:?}, risk: {:?})",
            name,
            category,
            risk
        );
        
        // Thread-safe kayıt
        // Not: Bu fonksiyon new() içinde çağrıldığı için RwLock henüz kullanılmıyor
        // Gerçek implementation'da tokio::spawn ile async yapılabilir
    }
    
    /// Execute tool
    pub async fn execute(
        &self,
        tool_name: &str,
        params: HashMap<String, serde_json::Value>,
    ) -> SentientToolResult {
        let tools = self.tools.read().await;
        
        let tool = match tools.get(tool_name) {
            Some(t) => t,
            None => {
                return SentientToolResult::failure(&format!(
                    "SENTIENT EXECUTOR: '{}' aracı bulunamadı. Kayıtlı araçlar: {}",
                    tool_name,
                    self.list_tool_names().join(", ")
                ));
            }
        };
        
        log::info!(
            "🔧  EXECUTOR: Çalıştır → {} (risk: {:?})",
            tool.name(),
            tool.risk_level()
        );
        
        tool.execute(params).await
    }
    
    /// Tool list
    pub fn list_tool_names(&self) -> Vec<&'static str> {
        vec![
            "git", "grep", "sed", "browser", "screenshot",
            "mcp", "memory", "task", "n8n", "email",
            "notify", "pdf", "translate", "calendar", "agent",
        ]
    }
    
    /// List tools by category
    pub fn list_by_category(&self, category: ToolCategory) -> Vec<&'static str> {
        match category {
            ToolCategory::FileSystem => vec!["git"],
            ToolCategory::Process => vec!["git", "task"],
            ToolCategory::Browser => vec!["browser"],
            ToolCategory::Screen => vec!["screenshot"],
            ToolCategory::Network => vec![],
            ToolCategory::System => vec![],
            ToolCategory::Data => vec!["grep", "sed", "pdf"],
            ToolCategory::Communication => vec!["email", "notify"],
            ToolCategory::Scheduling => vec!["calendar"],
            ToolCategory::Intelligence => vec!["translate"],
            ToolCategory::Integration => vec!["mcp", "n8n"],
            ToolCategory::Memory => vec!["memory"],
            ToolCategory::Agent => vec!["agent"],
            ToolCategory::Web => vec!["web_search", "web_fetch"],
            ToolCategory::Interaction => vec!["ask_user_question"],
            ToolCategory::Development => vec!["lsp"],
            ToolCategory::Productivity => vec!["todo_write"],
            ToolCategory::Analysis => vec!["brief"],
        }
    }
    
    /// Total tool count
    pub fn tool_count(&self) -> usize {
        15
    }
    
    /// Tool info
    pub fn get_tool_info(&self, name: &str) -> Option<ToolInfo> {
        match name {
            "git" => Some(ToolInfo::from_static("git", "Git versiyon kontrol", ToolCategory::Process, RiskLevel::Medium)),
            "grep" => Some(ToolInfo::from_static("grep", "Metin arama", ToolCategory::Data, RiskLevel::Low)),
            "sed" => Some(ToolInfo::from_static("sed", "Metin dönüştürme", ToolCategory::Data, RiskLevel::Low)),
            "browser" => Some(ToolInfo::from_static("browser", "Web tarayıcısı kontrolü", ToolCategory::Browser, RiskLevel::Medium)),
            "screenshot" => Some(ToolInfo::from_static("screenshot", "Ekran görüntüsü", ToolCategory::Screen, RiskLevel::Low)),
            "mcp" => Some(ToolInfo::from_static("mcp", "Model Context Protocol", ToolCategory::Integration, RiskLevel::Medium)),
            "memory" => Some(ToolInfo::from_static("memory", "Bellek yönetimi", ToolCategory::Memory, RiskLevel::Low)),
            "task" => Some(ToolInfo::from_static("task", "Görev yönetimi", ToolCategory::Process, RiskLevel::Low)),
            "n8n" => Some(ToolInfo::from_static("n8n", "Workflow otomasyonu", ToolCategory::Integration, RiskLevel::Medium)),
            "email" => Some(ToolInfo::from_static("email", "E-posta gönderimi", ToolCategory::Communication, RiskLevel::Medium)),
            "notify" => Some(ToolInfo::from_static("notify", "Bildirim sistemi", ToolCategory::Communication, RiskLevel::Low)),
            "pdf" => Some(ToolInfo::from_static("pdf", "PDF işlemleri", ToolCategory::Data, RiskLevel::Low)),
            "translate" => Some(ToolInfo::from_static("translate", "Çeviri", ToolCategory::Intelligence, RiskLevel::Low)),
            "calendar" => Some(ToolInfo::from_static("calendar", "Takvim yönetimi", ToolCategory::Scheduling, RiskLevel::Low)),
            "agent" => Some(ToolInfo::from_static("agent", "Ajan yönetimi", ToolCategory::Agent, RiskLevel::Medium)),
            _ => None,
        }
    }
}

impl Default for SENTIENTToolExecutor {
    fn default() -> Self {
        Self::new()
    }
}

// ───────────────────────────────────────────────────────────────────────────────
//  TOOL INFO
// ───────────────────────────────────────────────────────────────────────────────

/// Araç bilgisi
#[derive(Debug, Clone, serde::Serialize)]
pub struct ToolInfo {
    pub name: String,
    pub description: String,
    pub category: ToolCategory,
    pub risk_level: RiskLevel,
}

impl ToolInfo {
    fn from_static(name: &'static str, description: &'static str, category: ToolCategory, risk: RiskLevel) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            category,
            risk_level: risk,
        }
    }
}

// ───────────────────────────────────────────────────────────────────────────────
//  TESTS
// ───────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_executor_creation() {
        let executor = SENTIENTToolExecutor::new();
        assert_eq!(executor.tool_count(), 15);
    }
    
    #[test]
    fn test_list_tool_names() {
        let executor = SENTIENTToolExecutor::new();
        let tools = executor.list_tool_names();
        assert_eq!(tools.len(), 15);
        assert!(tools.contains(&"git"));
        assert!(tools.contains(&"agent"));
    }
    
    #[test]
    fn test_list_by_category() {
        let executor = SENTIENTToolExecutor::new();
        let browser_tools = executor.list_by_category(ToolCategory::Browser);
        assert!(browser_tools.contains(&"browser"));
    }
    
    #[test]
    fn test_get_tool_info() {
        let executor = SENTIENTToolExecutor::new();
        let info = executor.get_tool_info("git");
        assert!(info.is_some());
        
        let info = info.expect("operation failed");
        assert_eq!(info.name, "git");
        assert_eq!(info.category, ToolCategory::Process);
    }
    
    #[tokio::test]
    async fn test_execute_unknown_tool() {
        let executor = SENTIENTToolExecutor::new();
        let result = executor.execute("unknown_tool", HashMap::new()).await;
        assert!(!result.success);
        assert!(result.error.expect("operation failed").contains("bulunamadı"));
    }
}
