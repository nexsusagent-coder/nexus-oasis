//! OCR (Optical Character Recognition) module

use crate::types::{BoundingBox, OcrPageResult};
use crate::{Result, VisionError};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// OCR provider trait
#[async_trait]
pub trait OcrProvider: Send + Sync {
    /// Get provider name
    fn name(&self) -> &str;

    /// Recognize text from image bytes
    async fn recognize(&self, image: &[u8], options: &OcrOptions) -> Result<OcrPageResult>;

    /// Check if provider is available
    fn is_available(&self) -> bool {
        true
    }
}

/// OCR options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OcrOptions {
    /// Language(s) to recognize
    #[serde(default)]
    pub languages: Vec<String>,
    /// Minimum confidence threshold
    #[serde(default = "default_min_confidence")]
    pub min_confidence: f32,
    /// Enable bounding box detection
    #[serde(default = "default_true")]
    pub detect_boxes: bool,
    /// DPI for PDF processing
    #[serde(default = "default_dpi")]
    pub dpi: u32,
    /// Page number (for multi-page documents)
    #[serde(default)]
    pub page: Option<u32>,
}

fn default_min_confidence() -> f32 { 0.5 }
fn default_true() -> bool { true }
fn default_dpi() -> u32 { 300 }

impl Default for OcrOptions {
    fn default() -> Self {
        Self {
            languages: vec!["en".to_string()],
            min_confidence: default_min_confidence(),
            detect_boxes: default_true(),
            dpi: default_dpi(),
            page: None,
        }
    }
}

impl OcrOptions {
    /// Create options for English
    pub fn english() -> Self {
        Self {
            languages: vec!["en".to_string()],
            ..Default::default()
        }
    }

    /// Create options for Turkish
    pub fn turkish() -> Self {
        Self {
            languages: vec!["tr".to_string()],
            ..Default::default()
        }
    }

    /// Create options for multiple languages
    pub fn multi(languages: Vec<&str>) -> Self {
        Self {
            languages: languages.into_iter().map(String::from).collect(),
            ..Default::default()
        }
    }

    /// Set minimum confidence
    pub fn with_min_confidence(mut self, confidence: f32) -> Self {
        self.min_confidence = confidence;
        self
    }
}

/// OCR engine using simple text extraction
pub struct SimpleOcrEngine {
    supported_languages: Vec<String>,
}

impl SimpleOcrEngine {
    pub fn new() -> Self {
        Self {
            supported_languages: vec!["en".to_string(), "tr".to_string()],
        }
    }
}

impl Default for SimpleOcrEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl OcrProvider for SimpleOcrEngine {
    fn name(&self) -> &str {
        "simple"
    }

    async fn recognize(&self, _image: &[u8], options: &OcrOptions) -> Result<OcrPageResult> {
        // This is a stub implementation
        // In a real implementation, this would use Tesseract or similar
        tracing::warn!("SimpleOcrEngine is a stub - returning empty result");

        Ok(OcrPageResult {
            lines: Vec::new(),
            full_text: String::new(),
            confidence: 0.0,
            languages: options.languages.clone(),
        })
    }

    fn is_available(&self) -> bool {
        // Always available but not functional
        false
    }
}

/// Tesseract OCR configuration
#[derive(Debug, Clone)]
pub struct TesseractConfig {
    /// Path to tessdata directory
    pub tessdata_path: Option<String>,
    /// Language
    pub language: String,
    /// OCR Engine Mode (0-3)
    pub oem: u32,
    /// Page Segmentation Mode (0-13)
    pub psm: u32,
}

impl Default for TesseractConfig {
    fn default() -> Self {
        Self {
            tessdata_path: None,
            language: "eng".to_string(),
            oem: 3, // Default/LSTM
            psm: 3, // Auto
        }
    }
}

/// OCR text region
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextRegion {
    /// Bounding box
    pub bbox: BoundingBox,
    /// Text content
    pub text: String,
    /// Confidence
    pub confidence: f32,
    /// Language detected
    pub language: Option<String>,
}

/// OCR result with layout analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuredOcrResult {
    /// All detected text regions
    pub regions: Vec<TextRegion>,
    /// Combined full text
    pub full_text: String,
    /// Detected languages
    pub languages: Vec<String>,
    /// Average confidence
    pub avg_confidence: f32,
    /// Page dimensions
    pub page_size: Option<(u32, u32)>,
}

impl StructuredOcrResult {
    /// Create from regions
    pub fn from_regions(regions: Vec<TextRegion>) -> Self {
        let full_text: String = regions
            .iter()
            .map(|r| r.text.as_str())
            .collect::<Vec<_>>()
            .join("\n");

        let avg_confidence = if regions.is_empty() {
            0.0
        } else {
            regions.iter().map(|r| r.confidence).sum::<f32>() / regions.len() as f32
        };

        let languages: Vec<String> = regions
            .iter()
            .filter_map(|r| r.language.clone())
            .collect();

        Self {
            regions,
            full_text,
            languages,
            avg_confidence,
            page_size: None,
        }
    }

    /// Filter by confidence
    pub fn filter_by_confidence(&self, min_confidence: f32) -> Self {
        let regions: Vec<TextRegion> = self
            .regions
            .iter()
            .filter(|r| r.confidence >= min_confidence)
            .cloned()
            .collect();

        Self::from_regions(regions)
    }

    /// Get text in region
    pub fn text_in_region(&self, x: u32, y: u32, width: u32, height: u32) -> Vec<&TextRegion> {
        self.regions
            .iter()
            .filter(|r| {
                r.bbox.x >= x
                    && r.bbox.y >= y
                    && r.bbox.x + r.bbox.width <= x + width
                    && r.bbox.y + r.bbox.height <= y + height
            })
            .collect()
    }
}

/// OCR manager
pub struct OcrManager {
    providers: HashMap<String, Box<dyn OcrProvider>>,
    default_provider: Option<String>,
}

impl OcrManager {
    /// Create new OCR manager
    pub fn new() -> Self {
        let mut manager = Self {
            providers: HashMap::new(),
            default_provider: None,
        };

        // Register default simple engine
        manager.register("simple", Box::new(SimpleOcrEngine::new()));
        manager.default_provider = Some("simple".to_string());

        manager
    }

    /// Register a provider
    pub fn register(&mut self, name: &str, provider: Box<dyn OcrProvider>) {
        self.providers.insert(name.to_string(), provider);
    }

    /// Set default provider
    pub fn set_default(&mut self, name: &str) -> Result<()> {
        if self.providers.contains_key(name) {
            self.default_provider = Some(name.to_string());
            Ok(())
        } else {
            Err(VisionError::provider_not_available(name))
        }
    }

    /// Get provider by name
    pub fn get(&self, name: &str) -> Option<&dyn OcrProvider> {
        self.providers.get(name).map(|p| p.as_ref())
    }

    /// Recognize text using default provider
    pub async fn recognize(&self, image: &[u8], options: &OcrOptions) -> Result<OcrPageResult> {
        let provider_name = self.default_provider.as_ref()
            .ok_or_else(|| VisionError::config("No default OCR provider set"))?;

        self.recognize_with(provider_name, image, options).await
    }

    /// Recognize text using specific provider
    pub async fn recognize_with(&self, provider_name: &str, image: &[u8], options: &OcrOptions) -> Result<OcrPageResult> {
        let provider = self.providers.get(provider_name)
            .ok_or_else(|| VisionError::provider_not_available(provider_name))?;

        provider.recognize(image, options).await
    }

    /// List available providers
    pub fn list_providers(&self) -> Vec<&str> {
        self.providers.keys().map(String::as_str).collect()
    }
}

impl Default for OcrManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ocr_options_default() {
        let opts = OcrOptions::default();
        assert_eq!(opts.languages, vec!["en"]);
        assert_eq!(opts.min_confidence, 0.5);
    }

    #[test]
    fn test_ocr_options_turkish() {
        let opts = OcrOptions::turkish();
        assert_eq!(opts.languages, vec!["tr"]);
    }

    #[test]
    fn test_ocr_options_multi() {
        let opts = OcrOptions::multi(vec!["en", "tr", "de"]);
        assert_eq!(opts.languages, vec!["en", "tr", "de"]);
    }

    #[test]
    fn test_ocr_manager() {
        let manager = OcrManager::new();
        assert!(manager.list_providers().contains(&"simple"));
    }

    #[test]
    fn test_structured_ocr_result() {
        let regions = vec![
            TextRegion {
                bbox: BoundingBox::new(0, 0, 100, 20),
                text: "Hello".to_string(),
                confidence: 0.95,
                language: Some("en".to_string()),
            },
            TextRegion {
                bbox: BoundingBox::new(0, 25, 100, 20),
                text: "World".to_string(),
                confidence: 0.90,
                language: Some("en".to_string()),
            },
        ];

        let result = StructuredOcrResult::from_regions(regions);
        assert_eq!(result.full_text, "Hello\nWorld");
        assert!((result.avg_confidence - 0.925).abs() < 0.001);
    }
}
