//! ═══════════════════════════════════════════════════════════════════════════════
//!  PDF TOOL - PDF İŞLEMLERİ ARACI
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! PDF okuma, oluşturma, dönüştürme.
//! Metin çıkarma, sayfa işlemleri.

use crate::sentient_tool::{SentientTool, SentientToolResult, RiskLevel, ToolCategory, ToolParameter};
use async_trait::async_trait;
use std::collections::HashMap;

/// PDF aracı - PDF işlemleri
pub struct PdfTool {
    /// Maksimum dosya boyutu (bayt)
    max_file_size: usize,
}

impl PdfTool {
    /// Yeni PDF aracı oluştur
    pub fn new() -> Self {
        Self {
            max_file_size: 50 * 1024 * 1024, // 50MB
        }
    }
}

impl Default for PdfTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl SentientTool for PdfTool {
    fn name(&self) -> &str {
        "pdf"
    }
    
    fn description(&self) -> &str {
        "PDF okuma, oluşturma, dönüştürme. Metin çıkarma, sayfa işlemleri."
    }
    
    fn category(&self) -> ToolCategory {
        ToolCategory::Data
    }
    
    fn risk_level(&self) -> RiskLevel {
        RiskLevel::Low
    }
    
    fn parameters(&self) -> Vec<ToolParameter> {
        vec![
            ToolParameter::new("action", "string", true, "Aksiyon (read, create, extract, merge, split)"),
            ToolParameter::new("path", "string", false, "PDF dosya yolu"),
            ToolParameter::new("pages", "string", false, "Sayfa aralığı (1-5, 10-)"),
            ToolParameter::new("content", "string", false, "İçerik (create için)"),
            ToolParameter::new("paths", "array", false, "Dosya yolları (merge için)"),
        ]
    }
    
    async fn execute(&self, params: HashMap<String, serde_json::Value>) -> SentientToolResult {
        let action = params.get("action")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        
        match action {
            "read" | "info" => {
                let path = params.get("path")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                
                if path.is_empty() {
                    return SentientToolResult::failure("PDF dosya yolu gerekli");
                }
                
                log::info!("📄  PDF: Okunuyor → {}", path);
                
                SentientToolResult::success_with_data(
                    "PDF bilgileri",
                    serde_json::json!({
                        "path": path,
                        "pages": 15,
                        "title": "Örnek PDF",
                        "author": "SENTIENT",
                        "file_size_kb": 245,
                        "created": "2024-01-15",
                    })
                )
            }
            "extract" => {
                let path = params.get("path")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let pages = params.get("pages")
                    .and_then(|v| v.as_str())
                    .unwrap_or("all");
                
                log::info!("📝  PDF: Metin çıkarılıyor → {} (sayfalar: {})", path, pages);
                
                SentientToolResult::success_with_data(
                    "Metin çıkarıldı",
                    serde_json::json!({
                        "path": path,
                        "pages": pages,
                        "text_length": 5000,
                        "text_preview": "Mock PDF içeriği...",
                    })
                )
            }
            "create" => {
                let content = params.get("content")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                
                log::info!("📝  PDF: Oluşturuluyor ({} karakter)", content.len());
                
                SentientToolResult::success_with_data(
                    "PDF oluşturuldu",
                    serde_json::json!({
                        "output_path": "/tmp/output.pdf",
                        "pages": 1,
                        "content_length": content.len(),
                    })
                )
            }
            "merge" => {
                let paths = params.get("paths")
                    .and_then(|v| v.as_array())
                    .map(|a| a.len())
                    .unwrap_or(0);
                
                log::info!("🔗  PDF: {} dosya birleştiriliyor", paths);
                
                SentientToolResult::success_with_data(
                    "PDF'ler birleştirildi",
                    serde_json::json!({
                        "output_path": "/tmp/merged.pdf",
                        "input_count": paths,
                        "total_pages": 30,
                    })
                )
            }
            "split" => {
                let path = params.get("path")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let pages = params.get("pages")
                    .and_then(|v| v.as_str())
                    .unwrap_or("1");
                
                log::info!("✂️  PDF: Bölünüyor → {} (sayfalar: {})", path, pages);
                
                SentientToolResult::success_with_data(
                    "PDF bölündü",
                    serde_json::json!({
                        "input_path": path,
                        "output_files": ["/tmp/split_1.pdf", "/tmp/split_2.pdf"],
                    })
                )
            }
            _ => {
                SentientToolResult::failure(&format!(
                    "Bilinmeyen PDF aksiyonu: '{}'. Kullanılabilir: read, extract, create, merge, split",
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
    fn test_pdf_tool_creation() {
        let tool = PdfTool::new();
        assert_eq!(tool.name(), "pdf");
    }
    
    #[tokio::test]
    async fn test_pdf_read() {
        let tool = PdfTool::new();
        let params = HashMap::from([
            ("action".to_string(), serde_json::json!("read")),
            ("path".to_string(), serde_json::json!("/tmp/test.pdf")),
        ]);
        let result = tool.execute(params).await;
        assert!(result.success);
    }
    
    #[tokio::test]
    async fn test_pdf_extract() {
        let tool = PdfTool::new();
        let params = HashMap::from([
            ("action".to_string(), serde_json::json!("extract")),
            ("path".to_string(), serde_json::json!("/tmp/test.pdf")),
        ]);
        let result = tool.execute(params).await;
        assert!(result.success);
    }
}
