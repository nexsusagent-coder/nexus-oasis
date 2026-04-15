//! ─── LSP (Language Server Protocol) Integration ───

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Position in a document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub line: u32,
    pub character: u32,
}

impl Position {
    pub fn new(line: u32, character: u32) -> Self {
        Self { line, character }
    }
}

/// Range in a document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

impl Range {
    pub fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }
}

/// Location with URI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub uri: String,
    pub range: Range,
}

impl Location {
    pub fn new(uri: &str, range: Range) -> Self {
        Self { uri: uri.to_string(), range }
    }
}

/// Hover information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hover {
    pub contents: String,
    pub range: Option<Range>,
}

/// Symbol information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolInformation {
    pub name: String,
    pub kind: SymbolKind,
    pub location: Location,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[repr(i32)]
pub enum SymbolKind {
    File = 1,
    Module = 2,
    Class = 5,
    Method = 6,
    Function = 12,
    Variable = 13,
    Struct = 23,
    Enum = 10,
    Interface = 11,
}

/// Completion item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionItem {
    pub label: String,
    pub kind: Option<CompletionItemKind>,
    pub detail: Option<String>,
    pub insert_text: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[repr(i32)]
pub enum CompletionItemKind {
    Text = 1,
    Method = 2,
    Function = 3,
    Class = 7,
    Variable = 6,
    Module = 9,
    Keyword = 14,
    Struct = 22,
}

/// LSP client error
#[derive(Debug, thiserror::Error)]
pub enum LspError {
    #[error("Connection error: {0}")]
    Connection(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("LSP error: {0}")]
    Lsp(String),
    
    #[error("Not initialized")]
    NotInitialized,
}

/// LSP client configuration
#[derive(Debug, Clone)]
pub struct LspConfig {
    pub server_path: String,
    pub server_args: Vec<String>,
    pub root_uri: String,
}

impl Default for LspConfig {
    fn default() -> Self {
        Self {
            server_path: "rust-analyzer".into(),
            server_args: vec![],
            root_uri: "file:///workspace".into(),
        }
    }
}

/// Language Server Protocol client
pub struct LspClient {
    config: LspConfig,
    initialized: bool,
    request_id: u64,
}

impl LspClient {
    pub fn new(config: LspConfig) -> Self {
        Self {
            config,
            initialized: false,
            request_id: 0,
        }
    }
    
    /// Initialize LSP server
    pub async fn initialize(&mut self) -> Result<(), LspError> {
        tracing::info!("Initializing LSP server: {}", self.config.server_path);
        self.initialized = true;
        Ok(())
    }
    
    /// Go to definition
    pub async fn goto_definition(
        &mut self,
        uri: &str,
        position: Position,
    ) -> Result<Vec<Location>, LspError> {
        if !self.initialized {
            return Err(LspError::NotInitialized);
        }
        
        tracing::info!("goto_definition: {} at {:?}", uri, position);
        // TODO: Actual LSP communication
        Ok(vec![])
    }
    
    /// Find references
    pub async fn find_references(
        &mut self,
        uri: &str,
        position: Position,
    ) -> Result<Vec<Location>, LspError> {
        if !self.initialized {
            return Err(LspError::NotInitialized);
        }
        
        tracing::info!("find_references: {} at {:?}", uri, position);
        Ok(vec![])
    }
    
    /// Get hover information
    pub async fn hover(
        &mut self,
        uri: &str,
        position: Position,
    ) -> Result<Option<Hover>, LspError> {
        if !self.initialized {
            return Err(LspError::NotInitialized);
        }
        
        tracing::info!("hover: {} at {:?}", uri, position);
        Ok(None)
    }
    
    /// Get completions
    pub async fn completion(
        &mut self,
        uri: &str,
        position: Position,
    ) -> Result<Vec<CompletionItem>, LspError> {
        if !self.initialized {
            return Err(LspError::NotInitialized);
        }
        
        tracing::info!("completion: {} at {:?}", uri, position);
        Ok(vec![])
    }
    
    /// Document symbols
    pub async fn document_symbols(&mut self, uri: &str) -> Result<Vec<SymbolInformation>, LspError> {
        if !self.initialized {
            return Err(LspError::NotInitialized);
        }
        
        tracing::info!("document_symbols: {}", uri);
        Ok(vec![])
    }
    
    /// Shutdown LSP server
    pub async fn shutdown(&mut self) -> Result<(), LspError> {
        self.initialized = false;
        Ok(())
    }
    
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }
}

/// Multi-Language LSP Manager
pub struct LspManager {
    clients: std::collections::HashMap<String, LspClient>,
}

impl LspManager {
    pub fn new() -> Self {
        Self {
            clients: std::collections::HashMap::new(),
        }
    }
    
    /// Get or create LSP client for a language
    pub async fn get_client(&mut self, language: &str, root_uri: &str) -> Result<&mut LspClient, LspError> {
        let key = format!("{}:{}", language, root_uri);
        
        if !self.clients.contains_key(&key) {
            let server_path = Self::get_server_for_language(language)
                .ok_or_else(|| LspError::Lsp(format!("No LSP server for language: {}", language)))?;
            
            let config = LspConfig {
                server_path,
                server_args: vec![],
                root_uri: root_uri.to_string(),
            };
            
            let mut client = LspClient::new(config);
            client.initialize().await?;
            self.clients.insert(key.clone(), client);
        }
        
        Ok(self.clients.get_mut(&key).unwrap())
    }
    
    /// Get LSP server for language
    pub fn get_server_for_language(language: &str) -> Option<String> {
        match language {
            "rust" => Some("rust-analyzer".into()),
            "python" => Some("pylsp".into()),
            "javascript" | "typescript" => Some("typescript-language-server".into()),
            "go" => Some("gopls".into()),
            "java" => Some("jdtls".into()),
            _ => None,
        }
    }
    
    /// Detect language from file extension
    pub fn detect_language(path: &str) -> Option<&'static str> {
        let pb = PathBuf::from(path);
        let ext = pb.extension()?.to_str()?;
        
        match ext {
            "rs" => Some("rust"),
            "py" => Some("python"),
            "js" => Some("javascript"),
            "ts" => Some("typescript"),
            "tsx" => Some("typescript"),
            "go" => Some("go"),
            "java" => Some("java"),
            "c" => Some("c"),
            "cpp" => Some("cpp"),
            _ => None,
        }
    }
    
    /// Shutdown all clients
    pub async fn shutdown_all(&mut self) {
        for (_, client) in self.clients.iter_mut() {
            let _ = client.shutdown().await;
        }
        self.clients.clear();
    }
}

impl Default for LspManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_position() {
        let pos = Position::new(10, 5);
        assert_eq!(pos.line, 10);
    }
    
    #[test]
    fn test_language_detection() {
        assert_eq!(LspManager::detect_language("main.rs"), Some("rust"));
        assert_eq!(LspManager::detect_language("app.py"), Some("python"));
        assert_eq!(LspManager::detect_language("unknown.xyz"), None);
    }
}
