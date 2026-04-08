//! ═══════════════════════════════════════════════════════════════════════════════
//!  MCP TOOL - MODEL CONTEXT PROTOCOL ARACI
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Anthropic MCP sunucularıyla iletişim.
//! Harici araç ve kaynak entegrasyonu.

use crate::sentient_tool::{SentientTool, SentientToolResult, RiskLevel, ToolCategory, ToolParameter};
use async_trait::async_trait;
use std::collections::HashMap;

/// MCP aracı - Model Context Protocol
pub struct McpTool {
    /// Bağlı sunucular
    servers: HashMap<String, McpServer>,
}

/// MCP sunucu bilgisi
#[derive(Debug, Clone)]
pub struct McpServer {
    /// Sunucu adı
    pub name: String,
    /// Sunucu türü
    pub server_type: McpServerType,
    /// Durum
    pub status: McpStatus,
}

/// MCP sunucu türü
#[derive(Debug, Clone, Copy)]
pub enum McpServerType {
    Stdio,
    Tcp,
    WebSocket,
}

/// MCP durumu
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum McpStatus {
    Disconnected,
    Connected,
    Error,
}

impl McpTool {
    /// Yeni MCP aracı oluştur
    pub fn new() -> Self {
        Self {
            servers: HashMap::new(),
        }
    }
    
    /// Sunucu ekle
    pub fn add_server(&mut self, name: &str, server_type: McpServerType) {
        self.servers.insert(name.to_string(), McpServer {
            name: name.to_string(),
            server_type,
            status: McpStatus::Disconnected,
        });
    }
}

impl Default for McpTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl SentientTool for McpTool {
    fn name(&self) -> &str {
        "mcp"
    }
    
    fn description(&self) -> &str {
        "Model Context Protocol sunucularıyla iletişim. Harici araç ve kaynak entegrasyonu."
    }
    
    fn category(&self) -> ToolCategory {
        ToolCategory::Integration
    }
    
    fn risk_level(&self) -> RiskLevel {
        RiskLevel::Medium
    }
    
    fn parameters(&self) -> Vec<ToolParameter> {
        vec![
            ToolParameter::new("action", "string", true, "Aksiyon (connect, disconnect, list, call)"),
            ToolParameter::new("server", "string", false, "Sunucu adı"),
            ToolParameter::new("tool", "string", false, "Çağrılacak araç (call için)"),
            ToolParameter::new("params", "object", false, "Araç parametreleri"),
        ]
    }
    
    async fn execute(&self, params: HashMap<String, serde_json::Value>) -> SentientToolResult {
        let action = params.get("action")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        
        match action {
            "list" | "servers" => {
                let servers: Vec<&str> = self.servers.keys().map(|s| s.as_str()).collect();
                SentientToolResult::success_with_data(
                    &format!("{} MCP sunucusu kayıtlı", servers.len()),
                    serde_json::json!({
                        "servers": servers,
                        "count": servers.len(),
                    })
                )
            }
            "connect" => {
                let server = params.get("server")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                
                log::info!("🔌  MCP: Sunucuya bağlanılıyor → {}", server);
                
                SentientToolResult::success(&format!("Sunucuya bağlanıldı: {}", server))
            }
            "call" => {
                let tool_name = params.get("tool")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                
                let tool_params = params.get("params").cloned().unwrap_or(serde_json::json!({}));
                
                log::info!("🔧  MCP: Araç çağrılıyor → {}", tool_name);
                
                SentientToolResult::success_with_data(
                    &format!("Araç çağrıldı: {}", tool_name),
                    serde_json::json!({
                        "tool": tool_name,
                        "params": tool_params,
                        "result": "Mock MCP araç sonucu",
                    })
                )
            }
            _ => {
                SentientToolResult::failure(&format!(
                    "Bilinmeyen MCP aksiyonu: '{}'. Kullanılabilir: list, connect, call",
                    action
                ))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_mcp_tool_creation() {
        let tool = McpTool::new();
        assert_eq!(tool.name(), "mcp");
    }
    
    #[test]
    fn test_mcp_add_server() {
        let mut tool = McpTool::new();
        tool.add_server("filesystem", McpServerType::Stdio);
        assert!(tool.servers.contains_key("filesystem"));
    }
    
    #[tokio::test]
    async fn test_mcp_list() {
        let tool = McpTool::new();
        let params = HashMap::from([
            ("action".to_string(), serde_json::json!("list")),
        ]);
        let result = tool.execute(params).await;
        assert!(result.success);
    }
}
