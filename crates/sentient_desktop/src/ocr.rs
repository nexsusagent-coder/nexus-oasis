//! ═══════════════════════════════════════════════════════════════════════════════
//!  Desktop OCR Engine
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Optical Character Recognition for desktop:
//! - Screen text extraction
//! - Document OCR
//! - Multi-language support
//! - Layout analysis

use serde::{Deserialize, Serialize};

// ═══════════════════════════════════════════════════════════════════════════════
//  OCR TYPES
// ═══════════════════════════════════════════════════════════════════════════════

/// OCR configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OcrConfig {
    /// Languages to recognize
    pub languages: Vec<String>,
    /// DPI for rendering
    pub dpi: u32,
    /// Enable deskew
    pub deskew: bool,
    /// Enable layout analysis
    pub analyze_layout: bool,
    /// Confidence threshold (0.0-1.0)
    pub min_confidence: f32,
    /// Character whitelist
    pub char_whitelist: Option<String>,
    /// Character blacklist
    pub char_blacklist: Option<String>,
}

impl Default for OcrConfig {
    fn default() -> Self {
        Self {
            languages: vec!["eng".to_string()],
            dpi: 300,
            deskew: true,
            analyze_layout: true,
            min_confidence: 0.6,
            char_whitelist: None,
            char_blacklist: None,
        }
    }
}

/// Recognized text block
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextBlock {
    /// Bounding box
    pub bbox: BoundingBox,
    /// Recognized text
    pub text: String,
    /// Confidence score
    pub confidence: f32,
    /// Text level
    pub level: TextLevel,
    /// Language detected
    pub language: Option<String>,
}

/// Text hierarchy level
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TextLevel {
    Page,
    Block,
    Paragraph,
    Line,
    Word,
    Character,
}

/// Bounding box
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BoundingBox {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl BoundingBox {
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        Self { x, y, width, height }
    }
    
    pub fn contains(&self, x: u32, y: u32) -> bool {
        x >= self.x && x < self.x + self.width &&
        y >= self.y && y < self.y + self.height
    }
    
    pub fn intersects(&self, other: &BoundingBox) -> bool {
        self.x < other.x + other.width &&
        self.x + self.width > other.x &&
        self.y < other.y + other.height &&
        self.y + self.height > other.y
    }
}

/// OCR result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OcrResult {
    /// All text blocks
    pub blocks: Vec<TextBlock>,
    /// Full text
    pub full_text: String,
    /// Detected layout
    pub layout: Option<LayoutInfo>,
    /// Processing time (ms)
    pub processing_time_ms: u64,
    /// Page dimensions
    pub page_width: u32,
    pub page_height: u32,
}

/// Layout analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutInfo {
    /// Detected regions
    pub regions: Vec<LayoutRegion>,
    /// Reading order
    pub reading_order: Vec<usize>,
    /// Text direction
    pub text_direction: TextDirection,
}

/// Layout region type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutRegion {
    pub region_type: RegionType,
    pub bbox: BoundingBox,
    pub confidence: f32,
}

/// Region type
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum RegionType {
    Text,
    Title,
    Heading,
    Paragraph,
    List,
    Table,
    Image,
    Formula,
    Footer,
    Header,
    Marginalia,
}

/// Text direction
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TextDirection {
    LeftToRight,
    RightToLeft,
    TopToBottom,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  OCR ENGINE
// ═══════════════════════════════════════════════════════════════════════════════

/// OCR error
#[derive(Debug, thiserror::Error)]
pub enum OcrError {
    #[error("Image load error: {0}")]
    ImageLoad(String),
    
    #[error("Processing error: {0}")]
    Processing(String),
    
    #[error("Language not available: {0}")]
    LanguageNotAvailable(String),
    
    #[error("No text found")]
    NoTextFound,
}

/// OCR Engine
pub struct OcrEngine {
    config: OcrConfig,
}

impl OcrEngine {
    pub fn new(config: OcrConfig) -> Self {
        Self { config }
    }
    
    /// Recognize text from image bytes
    pub fn recognize(&self, image_data: &[u8]) -> Result<OcrResult, OcrError> {
        let start = std::time::Instant::now();
        
        // In production, use tesseract or similar
        // For now, simulate OCR
        
        let blocks = self.simulate_ocr(image_data)?;
        
        let full_text = blocks.iter()
            .map(|b| b.text.as_str())
            .collect::<Vec<_>>()
            .join("\n");
        
        Ok(OcrResult {
            blocks,
            full_text,
            layout: None,
            processing_time_ms: start.elapsed().as_millis() as u64,
            page_width: 800,
            page_height: 600,
        })
    }
    
    /// Recognize text from screen region
    pub fn recognize_screen_region(
        &self,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
    ) -> Result<OcrResult, OcrError> {
        // In production, capture screen and OCR
        // Simulate result
        Ok(OcrResult {
            blocks: vec![TextBlock {
                bbox: BoundingBox::new(x, y, width, height),
                text: "Sample screen text".to_string(),
                confidence: 0.95,
                level: TextLevel::Line,
                language: Some("eng".to_string()),
            }],
            full_text: "Sample screen text".to_string(),
            layout: None,
            processing_time_ms: 50,
            page_width: width,
            page_height: height,
        })
    }
    
    /// Extract text matching pattern
    pub fn extract_pattern(&self, image_data: &[u8], pattern: &str) -> Vec<String> {
        let result = self.recognize(image_data);
        
        match result {
            Ok(ocr) => {
                // Simple pattern matching
                ocr.full_text.lines()
                    .filter(|line| line.contains(pattern))
                    .map(|s| s.to_string())
                    .collect()
            }
            Err(_) => vec![],
        }
    }
    
    /// Find text position in image
    pub fn find_text(&self, image_data: &[u8], text: &str) -> Option<BoundingBox> {
        let result = self.recognize(image_data).ok()?;
        
        result.blocks.iter()
            .find(|b| b.text.contains(text))
            .map(|b| b.bbox)
    }
    
    fn simulate_ocr(&self, _image_data: &[u8]) -> Result<Vec<TextBlock>, OcrError> {
        // Simulated OCR blocks
        Ok(vec![
            TextBlock {
                bbox: BoundingBox::new(50, 50, 200, 30),
                text: "Hello World".to_string(),
                confidence: 0.98,
                level: TextLevel::Line,
                language: Some("eng".to_string()),
            },
            TextBlock {
                bbox: BoundingBox::new(50, 100, 300, 60),
                text: "This is a sample text extracted from the image.".to_string(),
                confidence: 0.95,
                level: TextLevel::Paragraph,
                language: Some("eng".to_string()),
            },
        ])
    }
}

impl Default for OcrEngine {
    fn default() -> Self {
        Self::new(OcrConfig::default())
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  SCREEN TEXT WATCHER
// ═══════════════════════════════════════════════════════════════════════════════

/// Screen text watcher for real-time text detection
pub struct ScreenTextWatcher {
    engine: OcrEngine,
    watch_regions: Vec<BoundingBox>,
    callbacks: Vec<Box<dyn Fn(&str) + Send + Sync>>,
}

impl ScreenTextWatcher {
    pub fn new(engine: OcrEngine) -> Self {
        Self {
            engine,
            watch_regions: vec![],
            callbacks: vec![],
        }
    }
    
    pub fn add_region(&mut self, bbox: BoundingBox) {
        self.watch_regions.push(bbox);
    }
    
    pub fn on_text_detected<F>(&mut self, callback: F)
    where
        F: Fn(&str) + Send + Sync + 'static,
    {
        self.callbacks.push(Box::new(callback));
    }
    
    pub fn scan(&self) -> Vec<(BoundingBox, String)> {
        self.watch_regions.iter()
            .filter_map(|region| {
                let result = self.engine.recognize_screen_region(
                    region.x, region.y, region.width, region.height
                ).ok()?;
                
                if !result.full_text.is_empty() {
                    Some((*region, result.full_text))
                } else {
                    None
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ocr_engine() {
        let engine = OcrEngine::default();
        let result = engine.recognize(&[]).unwrap();
        
        assert!(!result.blocks.is_empty());
        assert!(!result.full_text.is_empty());
    }
    
    #[test]
    fn test_bounding_box() {
        let bbox = BoundingBox::new(10, 20, 100, 50);
        
        assert!(bbox.contains(50, 40));
        assert!(!bbox.contains(5, 10));
        
        let other = BoundingBox::new(50, 40, 100, 50);
        assert!(bbox.intersects(&other));
    }
    
    #[test]
    fn test_screen_watcher() {
        let mut watcher = ScreenTextWatcher::new(OcrEngine::default());
        watcher.add_region(BoundingBox::new(0, 0, 800, 600));
        
        let results = watcher.scan();
        assert!(!results.is_empty());
    }
}
