//! # Sentient Vision
//!
//! Vision and multimodal AI capabilities for SENTIENT OS.
//!
//! ## Features
//!
//! - **Image Processing**: Load, resize, crop, convert images
//! - **OCR**: Extract text from images
//! - **Vision Models**: Integration with OpenAI GPT-4V, Claude 3, etc.
//! - **Multimodal Embeddings**: CLIP-style image/text embeddings
//!
//! ## Example
//!
//! ```rust
//! use sentient_vision::{ImageProcessor, VisionOptions, VisionManager};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Create processor
//! let processor = ImageProcessor::new(1024);
//!
//! // Process an image (in real usage, load from file)
//! // let image_data = std::fs::read("photo.jpg")?;
//! // let img = processor.load(&image_data)?;
//! // let processed = processor.process(&img)?;
//!
//! // Analyze with vision model
//! let manager = VisionManager::new();
//! let options = VisionOptions::default();
//! // let analysis = manager.analyze(&processed.data, &options).await?;
//!
//! // if let Some(desc) = analysis.description {
//! //     println!("Description: {}", desc.description);
//! // }
//! # Ok(())
//! # }
//! ```

pub mod error;
pub mod types;
pub mod image;
pub mod ocr;
pub mod provider;
pub mod embedding;

pub use error::{VisionError, Result};
pub use types::*;
pub use image::{ImageProcessor, ImageStatistics};
pub use ocr::{OcrManager, OcrOptions, OcrProvider};
pub use provider::{VisionManager, VisionProvider, Feature};
pub use embedding::{EmbeddingManager, EmbeddingProvider};

/// Vision version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Default maximum image size
pub const DEFAULT_MAX_SIZE: u32 = 2048;

/// Default minimum confidence
pub const DEFAULT_MIN_CONFIDENCE: f32 = 0.5;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn test_constants() {
        assert_eq!(DEFAULT_MAX_SIZE, 2048);
        assert!((DEFAULT_MIN_CONFIDENCE - 0.5).abs() < 0.001);
    }
}
