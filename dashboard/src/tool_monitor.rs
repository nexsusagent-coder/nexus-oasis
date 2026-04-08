//! ═══════════════════════════════════════════════════════════════════════════════
//!  TOOL MONITOR - Araç Durum İzleme
//! ═══════════════════════════════════════════════════════════════════════════════

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Araç durumu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolStatus {
    pub name: String,
    pub category: String,
    pub available: bool,
    pub risk_level: String,
    pub last_used: Option<DateTime<Utc>>,
    pub execution_count: u64,
    pub error_count: u64,
}

/// Tool Monitor - Araçları izle
pub struct ToolMonitor {
    tools: Vec<ToolStatus>,
}

impl ToolMonitor {
    pub fn new() -> Self {
        Self {
            tools: vec![
                ToolStatus {
                    name: "bash".to_string(),
                    category: "System".to_string(),
                    available: true,
                    risk_level: "High".to_string(),
                    last_used: None,
                    execution_count: 0,
                    error_count: 0,
                },
                ToolStatus {
                    name: "read_file".to_string(),
                    category: "FileSystem".to_string(),
                    available: true,
                    risk_level: "Low".to_string(),
                    last_used: None,
                    execution_count: 0,
                    error_count: 0,
                },
                ToolStatus {
                    name: "write_file".to_string(),
                    category: "FileSystem".to_string(),
                    available: true,
                    risk_level: "Medium".to_string(),
                    last_used: None,
                    execution_count: 0,
                    error_count: 0,
                },
                ToolStatus {
                    name: "edit_file".to_string(),
                    category: "FileSystem".to_string(),
                    available: true,
                    risk_level: "Medium".to_string(),
                    last_used: None,
                    execution_count: 0,
                    error_count: 0,
                },
                ToolStatus {
                    name: "glob".to_string(),
                    category: "FileSystem".to_string(),
                    available: true,
                    risk_level: "Low".to_string(),
                    last_used: None,
                    execution_count: 0,
                    error_count: 0,
                },
                ToolStatus {
                    name: "web_search".to_string(),
                    category: "Web".to_string(),
                    available: true,
                    risk_level: "Low".to_string(),
                    last_used: None,
                    execution_count: 0,
                    error_count: 0,
                },
                ToolStatus {
                    name: "web_fetch".to_string(),
                    category: "Web".to_string(),
                    available: true,
                    risk_level: "Low".to_string(),
                    last_used: None,
                    execution_count: 0,
                    error_count: 0,
                },
            ],
        }
    }
    
    /// Tüm araçları listele
    pub fn list_tools(&self) -> &[ToolStatus] {
        &self.tools
    }
    
    /// Araç kullanımını kaydet
    pub fn record_usage(&mut self, tool_name: &str, success: bool) {
        if let Some(tool) = self.tools.iter_mut().find(|t| t.name == tool_name) {
            tool.last_used = Some(Utc::now());
            tool.execution_count += 1;
            if !success {
                tool.error_count += 1;
            }
        }
    }
    
    /// Toplam istatistik
    pub fn get_stats(&self) -> ToolStats {
        let total = self.tools.len() as u64;
        let available = self.tools.iter().filter(|t| t.available).count() as u64;
        let total_executions = self.tools.iter().map(|t| t.execution_count).sum();
        let total_errors = self.tools.iter().map(|t| t.error_count).sum();
        
        ToolStats {
            total_tools: total,
            available_tools: available,
            total_executions,
            total_errors,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolStats {
    pub total_tools: u64,
    pub available_tools: u64,
    pub total_executions: u64,
    pub total_errors: u64,
}

impl Default for ToolMonitor {
    fn default() -> Self {
        Self::new()
    }
}
