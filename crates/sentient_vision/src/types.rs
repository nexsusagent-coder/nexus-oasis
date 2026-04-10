//! Vision type definitions

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Image format
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ImageFormat {
    Png,
    Jpeg,
    Gif,
    WebP,
    Bmp,
    Tiff,
}

impl ImageFormat {
    /// Get file extension
    pub fn extension(&self) -> &'static str {
        match self {
            Self::Png => "png",
            Self::Jpeg => "jpg",
            Self::Gif => "gif",
            Self::WebP => "webp",
            Self::Bmp => "bmp",
            Self::Tiff => "tiff",
        }
    }

    /// Get MIME type
    pub fn mime_type(&self) -> &'static str {
        match self {
            Self::Png => "image/png",
            Self::Jpeg => "image/jpeg",
            Self::Gif => "image/gif",
            Self::WebP => "image/webp",
            Self::Bmp => "image/bmp",
            Self::Tiff => "image/tiff",
        }
    }

    /// Parse from extension
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "png" => Some(Self::Png),
            "jpg" | "jpeg" => Some(Self::Jpeg),
            "gif" => Some(Self::Gif),
            "webp" => Some(Self::WebP),
            "bmp" => Some(Self::Bmp),
            "tiff" | "tif" => Some(Self::Tiff),
            _ => None,
        }
    }
}

/// Image size
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImageSize {
    pub width: u32,
    pub height: u32,
}

impl ImageSize {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    pub fn aspect_ratio(&self) -> f64 {
        self.width as f64 / self.height as f64
    }

    pub fn pixel_count(&self) -> u64 {
        self.width as u64 * self.height as u64
    }
}

/// Image source
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ImageSource {
    /// Image from URL
    Url { url: String },
    /// Image from base64 data
    Base64 { data: String, format: ImageFormat },
    /// Image from file path
    File { path: PathBuf },
    /// Image from bytes
    Bytes { data: Vec<u8>, format: ImageFormat },
}

impl ImageSource {
    /// Create URL source
    pub fn url(url: impl Into<String>) -> Self {
        Self::Url { url: url.into() }
    }

    /// Create file source
    pub fn file(path: impl Into<PathBuf>) -> Self {
        Self::File { path: path.into() }
    }

    /// Create base64 source
    pub fn base64(data: impl Into<String>, format: ImageFormat) -> Self {
        Self::Base64 {
            data: data.into(),
            format,
        }
    }

    /// Create bytes source
    pub fn bytes(data: Vec<u8>, format: ImageFormat) -> Self {
        Self::Bytes { data, format }
    }
}

/// Processed image
#[derive(Debug, Clone)]
pub struct ProcessedImage {
    /// Image data
    pub data: Vec<u8>,
    /// Image format
    pub format: ImageFormat,
    /// Image size
    pub size: ImageSize,
}

impl ProcessedImage {
    /// Create new processed image
    pub fn new(data: Vec<u8>, format: ImageFormat, size: ImageSize) -> Self {
        Self { data, format, size }
    }

    /// Get base64 encoding
    pub fn to_base64(&self) -> String {
        base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &self.data)
    }

    /// Get data URI
    pub fn to_data_uri(&self) -> String {
        format!("data:{};base64,{}", self.format.mime_type(), self.to_base64())
    }
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

    pub fn area(&self) -> u64 {
        self.width as u64 * self.height as u64
    }

    pub fn contains(&self, x: u32, y: u32) -> bool {
        x >= self.x && x < self.x + self.width && y >= self.y && y < self.y + self.height
    }
}

/// Detected object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedObject {
    /// Object label
    pub label: String,
    /// Confidence score (0.0 - 1.0)
    pub confidence: f32,
    /// Bounding box
    pub bbox: BoundingBox,
    /// Additional attributes
    #[serde(default)]
    pub attributes: std::collections::HashMap<String, String>,
}

impl DetectedObject {
    pub fn new(label: impl Into<String>, confidence: f32, bbox: BoundingBox) -> Self {
        Self {
            label: label.into(),
            confidence,
            bbox,
            attributes: std::collections::HashMap::new(),
        }
    }

    pub fn with_attribute(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.attributes.insert(key.into(), value.into());
        self
    }
}

/// Detected face
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedFace {
    /// Bounding box
    pub bbox: BoundingBox,
    /// Confidence score
    pub confidence: f32,
    /// Face landmarks (key points)
    #[serde(default)]
    pub landmarks: Vec<FaceLandmark>,
    /// Estimated age (if available)
    #[serde(default)]
    pub age: Option<u8>,
    /// Estimated gender (if available)
    #[serde(default)]
    pub gender: Option<String>,
    /// Embedding for face recognition
    #[serde(default)]
    pub embedding: Option<Vec<f32>>,
}

/// Face landmark point
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct FaceLandmark {
    pub point: LandmarkPoint,
    pub landmark_type: LandmarkType,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct LandmarkPoint {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LandmarkType {
    LeftEye,
    RightEye,
    Nose,
    LeftMouth,
    RightMouth,
    LeftEyebrow,
    RightEyebrow,
    Chin,
}

/// OCR result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OcrResult {
    /// Recognized text
    pub text: String,
    /// Confidence score
    pub confidence: f32,
    /// Bounding box
    pub bbox: Option<BoundingBox>,
    /// Language detected
    #[serde(default)]
    pub language: Option<String>,
}

impl OcrResult {
    pub fn text(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            confidence: 1.0,
            bbox: None,
            language: None,
        }
    }

    pub fn with_confidence(mut self, confidence: f32) -> Self {
        self.confidence = confidence;
        self
    }

    pub fn with_bbox(mut self, bbox: BoundingBox) -> Self {
        self.bbox = Some(bbox);
        self
    }

    pub fn with_language(mut self, lang: impl Into<String>) -> Self {
        self.language = Some(lang.into());
        self
    }
}

/// Full OCR page result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OcrPageResult {
    /// All detected text lines
    pub lines: Vec<OcrResult>,
    /// Combined text
    pub full_text: String,
    /// Average confidence
    pub confidence: f32,
    /// Detected languages
    pub languages: Vec<String>,
}

impl OcrPageResult {
    pub fn new(lines: Vec<OcrResult>) -> Self {
        let full_text: String = lines.iter().map(|l| l.text.as_str()).collect::<Vec<_>>().join("\n");
        let confidence = if lines.is_empty() {
            0.0
        } else {
            lines.iter().map(|l| l.confidence).sum::<f32>() / lines.len() as f32
        };
        let languages: Vec<String> = lines
            .iter()
            .filter_map(|l| l.language.clone())
            .collect();

        Self {
            lines,
            full_text,
            confidence,
            languages,
        }
    }
}

/// Image description result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageDescription {
    /// Generated description
    pub description: String,
    /// Confidence score
    #[serde(default)]
    pub confidence: f32,
    /// Tags extracted from the image
    #[serde(default)]
    pub tags: Vec<String>,
}

impl ImageDescription {
    pub fn new(description: impl Into<String>) -> Self {
        Self {
            description: description.into(),
            confidence: 1.0,
            tags: Vec::new(),
        }
    }

    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }

    pub fn with_confidence(mut self, confidence: f32) -> Self {
        self.confidence = confidence;
        self
    }
}

/// Image analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageAnalysis {
    /// Image description
    pub description: Option<ImageDescription>,
    /// Detected objects
    pub objects: Vec<DetectedObject>,
    /// Detected faces
    pub faces: Vec<DetectedFace>,
    /// OCR results
    pub text: Option<OcrPageResult>,
    /// Dominant colors
    #[serde(default)]
    pub colors: Vec<DominantColor>,
    /// Image category
    #[serde(default)]
    pub category: Option<String>,
}

impl Default for ImageAnalysis {
    fn default() -> Self {
        Self {
            description: None,
            objects: Vec::new(),
            faces: Vec::new(),
            text: None,
            colors: Vec::new(),
            category: None,
        }
    }
}

/// Dominant color
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct DominantColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub percentage: f32,
}

impl DominantColor {
    pub fn new(r: u8, g: u8, b: u8, percentage: f32) -> Self {
        Self {
            r,
            g,
            b,
            percentage,
        }
    }

    pub fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}

/// Vision request options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisionOptions {
    /// Maximum image size (pixels on longest side)
    #[serde(default = "default_max_size")]
    pub max_size: u32,
    /// Enable object detection
    #[serde(default)]
    pub detect_objects: bool,
    /// Enable face detection
    #[serde(default)]
    pub detect_faces: bool,
    /// Enable OCR
    #[serde(default)]
    pub extract_text: bool,
    /// Enable color extraction
    #[serde(default)]
    pub extract_colors: bool,
    /// Generate description
    #[serde(default = "default_true")]
    pub describe: bool,
    /// Custom prompt for description
    #[serde(default)]
    pub prompt: Option<String>,
    /// Language for OCR and description
    #[serde(default = "default_language")]
    pub language: String,
    /// Minimum confidence threshold
    #[serde(default = "default_confidence")]
    pub min_confidence: f32,
}

fn default_max_size() -> u32 { 2048 }
fn default_true() -> bool { true }
fn default_language() -> String { "en".to_string() }
fn default_confidence() -> f32 { 0.5 }

impl Default for VisionOptions {
    fn default() -> Self {
        Self {
            max_size: default_max_size(),
            detect_objects: false,
            detect_faces: false,
            extract_text: false,
            extract_colors: false,
            describe: true,
            prompt: None,
            language: default_language(),
            min_confidence: default_confidence(),
        }
    }
}

/// Multimodal embedding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultimodalEmbedding {
    /// Embedding vector
    pub vector: Vec<f32>,
    /// Dimension
    pub dimension: usize,
    /// Model used
    pub model: String,
}

impl MultimodalEmbedding {
    pub fn new(vector: Vec<f32>, model: impl Into<String>) -> Self {
        let dimension = vector.len();
        Self {
            vector,
            dimension,
            model: model.into(),
        }
    }

    pub fn cosine_similarity(&self, other: &MultimodalEmbedding) -> f32 {
        if self.dimension != other.dimension {
            return 0.0;
        }

        let dot: f32 = self.vector.iter()
            .zip(other.vector.iter())
            .map(|(a, b)| a * b)
            .sum();

        let mag_a: f32 = self.vector.iter().map(|x| x * x).sum::<f32>().sqrt();
        let mag_b: f32 = other.vector.iter().map(|x| x * x).sum::<f32>().sqrt();

        if mag_a == 0.0 || mag_b == 0.0 {
            return 0.0;
        }

        dot / (mag_a * mag_b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_format() {
        assert_eq!(ImageFormat::Png.extension(), "png");
        assert_eq!(ImageFormat::Jpeg.mime_type(), "image/jpeg");
        assert_eq!(ImageFormat::from_extension("jpg"), Some(ImageFormat::Jpeg));
    }

    #[test]
    fn test_image_size() {
        let size = ImageSize::new(1920, 1080);
        assert_eq!(size.aspect_ratio(), 16.0 / 9.0);
        assert_eq!(size.pixel_count(), 2_073_600);
    }

    #[test]
    fn test_bounding_box() {
        let bbox = BoundingBox::new(10, 20, 100, 50);
        assert_eq!(bbox.area(), 5000);
        assert!(bbox.contains(50, 40));
        assert!(!bbox.contains(5, 10));
    }

    #[test]
    fn test_ocr_result() {
        let result = OcrResult::text("Hello World")
            .with_confidence(0.95)
            .with_language("en");
        assert_eq!(result.text, "Hello World");
        assert_eq!(result.confidence, 0.95);
    }

    #[test]
    fn test_embedding_similarity() {
        let emb1 = MultimodalEmbedding::new(vec![1.0, 0.0, 0.0], "test");
        let emb2 = MultimodalEmbedding::new(vec![1.0, 0.0, 0.0], "test");
        assert!((emb1.cosine_similarity(&emb2) - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_dominant_color() {
        let color = DominantColor::new(255, 128, 0, 0.5);
        assert_eq!(color.to_hex(), "#FF8000");
    }
}
