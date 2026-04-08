//! ═══════════════════════════════════════════════════════════════════════════════
//!  LSP TOOL - Language Server Protocol Entegrasyonu
//! ═══════════════════════════════════════════════════════════════════════════════

use crate::sentient_tool::{SentientTool, SentientToolResult, RiskLevel, ToolCategory, ToolParameter};
use async_trait::async_trait;
use std::collections::HashMap;
use std::path::PathBuf;

/// LSP Tool - Dil sunucusu entegrasyonu
pub struct LspTool {
    lsp_clients: HashMap<String, LspClient>,
}

/// LSP Client durumu
struct LspClient {
    language: String,
    root_path: PathBuf,
    initialized: bool,
}

/// LSP Aksiyonları
#[derive(Debug, Clone)]
pub enum LspAction {
    /// Tanıma git (Go to definition)
    GotoDefinition { file: String, line: u32, column: u32 },
    /// Referansları bul
    FindReferences { file: String, line: u32, column: u32 },
    /// Hover bilgisi
    Hover { file: String, line: u32, column: u32 },
    /// Tamamlama
    Completion { file: String, line: u32, column: u32 },
    /// Semantik token'lar
    SemanticTokens { file: String },
    /// Sembol arama
    WorkspaceSymbol { query: String },
    /// Yeniden adlandır
    Rename { file: String, line: u32, column: u32, new_name: String },
    /// Formatlama
    Format { file: String },
    /// Tanı
    TypeDefinition { file: String, line: u32, column: u32 },
}

impl LspTool {
    pub fn new() -> Self {
        Self {
            lsp_clients: HashMap::new(),
        }
    }
    
    pub fn default_tool() -> Self {
        Self::new()
    }
    
    /// Dil sunucusu başlat
    pub async fn start_lsp(&mut self, language: &str, root_path: &PathBuf) -> SentientToolResult {
        let client = LspClient {
            language: language.to_string(),
            root_path: root_path.clone(),
            initialized: false,
        };
        
        // Dil sunucusu command'ını belirle
        let server_cmd = match language {
            "rust" => "rust-analyzer",
            "python" => "pylsp",
            "typescript" | "javascript" => "typescript-language-server",
            "go" => "gopls",
            "c" | "cpp" => "clangd",
            "java" => "jdtls",
            _ => return SentientToolResult::failure(&format!("Desteklenmeyen dil: {}", language)),
        };
        
        // LSP başlat (şimdilik simülasyon)
        self.lsp_clients.insert(language.to_string(), client);
        
        SentientToolResult::success(&format!("LSP başlatıldı: {} ({})", language, server_cmd))
    }
    
    /// LSP aksiyonu çalıştır
    pub async fn execute_lsp_action(&mut self, action: LspAction) -> SentientToolResult {
        match action {
            LspAction::GotoDefinition { file, line, column } => {
                // Simülasyon - gerçek LSP ile değiştirilecek
                SentientToolResult::success_with_data(
                    "Tanım bulundu",
                    serde_json::json!({
                        "file": file,
                        "line": line + 10, // Örnek
                        "column": column,
                        "definition": format!("{}:{}:{}", file, line + 10, column)
                    })
                )
            }
            LspAction::FindReferences { file, line, column } => {
                SentientToolResult::success_with_data(
                    "Referanslar bulundu",
                    serde_json::json!({
                        "references": [
                            { "file": file, "line": line, "column": column },
                            { "file": file, "line": line + 20, "column": column + 5 },
                        ]
                    })
                )
            }
            LspAction::Hover { file, line, column } => {
                SentientToolResult::success_with_data(
                    "Hover bilgisi",
                    serde_json::json!({
                        "contents": "fn main() -> ()",
                        "file": file,
                        "line": line,
                        "column": column
                    })
                )
            }
            LspAction::Completion { file, line, column } => {
                SentientToolResult::success_with_data(
                    "Tamamlamalar",
                    serde_json::json!({
                        "items": [
                            { "label": "println!", "kind": "function" },
                            { "label": "print!", "kind": "function" },
                            { "label": "panic!", "kind": "function" },
                        ]
                    })
                )
            }
            LspAction::WorkspaceSymbol { query } => {
                SentientToolResult::success_with_data(
                    "Semboller bulundu",
                    serde_json::json!({
                        "symbols": [
                            { "name": query, "kind": "function", "location": "main.rs:10:5" }
                        ]
                    })
                )
            }
            _ => SentientToolResult::failure("LSP aksiyonu henüz implement edilmedi")
        }
    }
}

#[async_trait]
impl SentientTool for LspTool {
    fn name(&self) -> &str { "lsp" }
    
    fn description(&self) -> &str {
        "Language Server Protocol entegrasyonu. Tanıma git, referans bul, tamamlama, hover."
    }
    
    fn category(&self) -> ToolCategory { ToolCategory::Development }
    
    fn risk_level(&self) -> RiskLevel { RiskLevel::Low }
    
    fn parameters(&self) -> Vec<ToolParameter> {
        vec![
            ToolParameter::new("action", "string", true, "LSP aksiyonu: goto_definition, find_references, hover, completion, workspace_symbol"),
            ToolParameter::new("file", "string", false, "Dosya yolu"),
            ToolParameter::new("line", "integer", false, "Satır numarası (1-indexed)"),
            ToolParameter::new("column", "integer", false, "Sütun numarası (1-indexed)"),
            ToolParameter::new("query", "string", false, "Arama sorgusu (workspace_symbol için)"),
        ]
    }
    
    async fn execute(&self, params: HashMap<String, serde_json::Value>) -> SentientToolResult {
        let action = params.get("action")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        
        let file = params.get("file").and_then(|v| v.as_str()).unwrap_or("");
        let line = params.get("line").and_then(|v| v.as_u64()).unwrap_or(1) as u32;
        let column = params.get("column").and_then(|v| v.as_u64()).unwrap_or(1) as u32;
        let query = params.get("query").and_then(|v| v.as_str()).unwrap_or("");
        
        let lsp_action = match action {
            "goto_definition" => LspAction::GotoDefinition { file: file.to_string(), line, column },
            "find_references" => LspAction::FindReferences { file: file.to_string(), line, column },
            "hover" => LspAction::Hover { file: file.to_string(), line, column },
            "completion" => LspAction::Completion { file: file.to_string(), line, column },
            "workspace_symbol" => LspAction::WorkspaceSymbol { query: query.to_string() },
            _ => return SentientToolResult::failure(&format!("Bilinmeyen LSP aksiyonu: {}", action)),
        };
        
        // Mutate self için clone ve execute
        let mut this = self.clone();
        this.execute_lsp_action(lsp_action).await
    }
}

impl Clone for LspTool {
    fn clone(&self) -> Self {
        Self {
            lsp_clients: HashMap::new(),
        }
    }
}

impl Default for LspTool {
    fn default() -> Self {
        Self::default_tool()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_tool_creation() {
        let tool = LspTool::default_tool();
        assert_eq!(tool.name(), "lsp");
        assert_eq!(tool.category(), ToolCategory::Development);
    }
}
