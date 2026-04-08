//! ═══════════════════════════════════════════════════════════════════════════════
//!  ASK USER QUESTION TOOL - Kullanıcı Etkileşimi
//! ═══════════════════════════════════════════════════════════════════════════════

use crate::sentient_tool::{SentientTool, SentientToolResult, RiskLevel, ToolCategory, ToolParameter};
use async_trait::async_trait;
use std::collections::HashMap;

/// Kullanıcıya soru sor ve cevap bekle
pub struct AskUserQuestionTool {
    timeout_secs: u64,
}

impl AskUserQuestionTool {
    pub fn new() -> Self {
        Self { timeout_secs: 300 }
    }
    
    pub fn default_tool() -> Self {
        Self::new()
    }
}

#[async_trait]
impl SentientTool for AskUserQuestionTool {
    fn name(&self) -> &str { "ask_user_question" }
    
    fn description(&self) -> &str {
        "Kullanıcıya soru sor ve cevap bekle. Onay, seçim veya bilgi almak için kullanılır."
    }
    
    fn category(&self) -> ToolCategory { ToolCategory::Interaction }
    
    fn risk_level(&self) -> RiskLevel { RiskLevel::Low }
    
    fn parameters(&self) -> Vec<ToolParameter> {
        vec![
            ToolParameter::new("question", "string", true, "Sorulacak soru"),
            ToolParameter::new("type", "string", false, "Soru tipi: 'text', 'confirm', 'select'"),
            ToolParameter::new("options", "array", false, "Seçenekler (select tipi için)"),
            ToolParameter::new("default", "string", false, "Varsayılan cevap"),
        ]
    }
    
    async fn execute(&self, params: HashMap<String, serde_json::Value>) -> SentientToolResult {
        let question = params.get("question")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        
        let q_type = params.get("type")
            .and_then(|v| v.as_str())
            .unwrap_or("text");
        
        let options = params.get("options")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect::<Vec<_>>())
            .unwrap_or_default();
        
        let default = params.get("default").and_then(|v| v.as_str());
        
        // Interactive mod kontrolü - her zaman interactive modda çalış
        // (Non-interactive modda tool çağrılmamalı)
        
        // Soruyu göster
        println!("\n❓ {}", question);
        
        if q_type == "select" && !options.is_empty() {
            println!("Seçenekler:");
            for (i, opt) in options.iter().enumerate() {
                println!("  {}. {}", i + 1, opt);
            }
            print!("Seçiminiz (1-{}): ", options.len());
        } else if q_type == "confirm" {
            print!("[y/N]: ");
        } else {
            print!("> ");
        }
        
        // Cevabı oku
        let mut input = String::new();
        use std::io::{self, Write};
        io::stdout().flush().ok();
        
        if io::stdin().read_line(&mut input).is_ok() {
            let answer = input.trim();
            
            // Tip bazlı işleme
            let result = match q_type {
                "confirm" => {
                    let confirmed = answer.to_lowercase() == "y" || answer.to_lowercase() == "yes";
                    serde_json::json!({
                        "answer": confirmed,
                        "raw": answer
                    })
                }
                "select" => {
                    if let Ok(idx) = answer.parse::<usize>() {
                        if idx > 0 && idx <= options.len() {
                            serde_json::json!({
                                "answer": options[idx - 1],
                                "index": idx - 1
                            })
                        } else {
                            serde_json::json!({ "answer": answer, "error": "Geçersiz seçim" })
                        }
                    } else if default.is_some() {
                        serde_json::json!({ "answer": default })
                    } else {
                        serde_json::json!({ "answer": answer })
                    }
                }
                _ => {
                    if answer.is_empty() && default.is_some() {
                        serde_json::json!({ "answer": default })
                    } else {
                        serde_json::json!({ "answer": answer })
                    }
                }
            };
            
            return SentientToolResult::success_with_data("Cevap alındı", result);
        }
        
        SentientToolResult::failure("Cevap okunamadı")
    }
}

impl Default for AskUserQuestionTool {
    fn default() -> Self {
        Self::default_tool()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_tool_creation() {
        let tool = AskUserQuestionTool::default_tool();
        assert_eq!(tool.name(), "ask_user_question");
        assert_eq!(tool.risk_level(), RiskLevel::Low);
    }
}
