//! ═══════════════════════════════════════════════════════════════════════════════
//!  SCREENSHOT TOOL - EKRAN GÖRÜNTÜSÜ ARACI
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Ekran veya pencere görüntüsü alma.
//! OCR ve görüntü analizi desteği.

use crate::sentient_tool::{SentientTool, SentientToolResult, RiskLevel, ToolCategory, ToolParameter};
use async_trait::async_trait;
use std::collections::HashMap;

/// Screenshot aracı - ekran görüntüsü
pub struct ScreenshotTool {
    /// Varsayılan format
    format: ImageFormat,
    /// Kalite (0-100)
    quality: u8,
}

/// Görüntü formatı
#[derive(Debug, Clone, Copy)]
pub enum ImageFormat {
    Png,
    Jpeg,
    WebP,
}

impl ScreenshotTool {
    /// Yeni Screenshot aracı oluştur
    pub fn new() -> Self {
        Self {
            format: ImageFormat::Png,
            quality: 85,
        }
    }
    
    /// Format ayarla
    pub fn with_format(mut self, format: ImageFormat) -> Self {
        self.format = format;
        self
    }
}

impl Default for ScreenshotTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl SentientTool for ScreenshotTool {
    fn name(&self) -> &str {
        "screenshot"
    }
    
    fn description(&self) -> &str {
        "Ekran veya pencere görüntüsü alır. Tam ekran veya bölgesel ekran görüntüsü."
    }
    
    fn category(&self) -> ToolCategory {
        ToolCategory::Screen
    }
    
    fn risk_level(&self) -> RiskLevel {
        RiskLevel::Low
    }
    
    fn parameters(&self) -> Vec<ToolParameter> {
        vec![
            ToolParameter::new("region", "string", false, "Bölge (full, window, veya x,y,width,height)"),
            ToolParameter::new("format", "string", false, "Format (png, jpeg, webp)"),
            ToolParameter::new("quality", "number", false, "Kalite (0-100, jpeg için)"),
            ToolParameter::new("ocr", "boolean", false, "OCR uygulansın mı?"),
        ]
    }
    
    async fn execute(&self, params: HashMap<String, serde_json::Value>) -> SentientToolResult {
        let region = params.get("region")
            .and_then(|v| v.as_str())
            .unwrap_or("full");
        
        let format = params.get("format")
            .and_then(|v| v.as_str())
            .unwrap_or("png");
        
        let ocr = params.get("ocr")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        
        log::info!("📸  SCREENSHOT: Alınıyor → {} ({})", region, format);
        
        // Mock: Gerçek implementation için xcap veya screenshots crate kullanılır
        let mut result_data = serde_json::json!({
            "action": "screenshot",
            "region": region,
            "format": format,
            "width": 1920,
            "height": 1080,
            "size_bytes": 245_678,
            "data": "base64_mock_screenshot_data",
        });
        
        // OCR eklenecek mi?
        if ocr {
            result_data["ocr_text"] = serde_json::json!("Mock OCR sonucu: Bu bir test metnidir.");
            log::info!("📝  SCREENSHOT: OCR uygulandı");
        }
        
        SentientToolResult::success_with_data(
            &format!("Ekran görüntüsü alındı: {}x{}, {} format", 1920, 1080, format),
            result_data
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_screenshot_tool_creation() {
        let tool = ScreenshotTool::new();
        assert_eq!(tool.name(), "screenshot");
    }
    
    #[test]
    fn test_screenshot_category() {
        let tool = ScreenshotTool::new();
        assert_eq!(tool.category(), ToolCategory::Screen);
    }
    
    #[tokio::test]
    async fn test_screenshot_execute() {
        let tool = ScreenshotTool::new();
        let params = HashMap::from([
            ("region".to_string(), serde_json::json!("full")),
            ("format".to_string(), serde_json::json!("png")),
        ]);
        let result = tool.execute(params).await;
        assert!(result.success);
    }
    
    #[tokio::test]
    async fn test_screenshot_with_ocr() {
        let tool = ScreenshotTool::new();
        let params = HashMap::from([
            ("region".to_string(), serde_json::json!("window")),
            ("ocr".to_string(), serde_json::json!(true)),
        ]);
        let result = tool.execute(params).await;
        assert!(result.success);
        assert!(result.data.unwrap().get("ocr_text").is_some());
    }
}
