//! Image processing utilities

use crate::types::{ImageFormat, ImageSize, ProcessedImage};
use crate::{Result, VisionError};
use image::{DynamicImage, GenericImageView, ImageFormat as ImgFormat};
use std::io::Cursor;

/// Image processor
pub struct ImageProcessor {
    /// Maximum dimension
    max_size: u32,
    /// Default output format
    default_format: ImageFormat,
}

impl Default for ImageProcessor {
    fn default() -> Self {
        Self {
            max_size: 2048,
            default_format: ImageFormat::Png,
        }
    }
}

impl ImageProcessor {
    /// Create new processor
    pub fn new(max_size: u32) -> Self {
        Self {
            max_size,
            default_format: ImageFormat::Png,
        }
    }

    /// Set default output format
    pub fn with_format(mut self, format: ImageFormat) -> Self {
        self.default_format = format;
        self
    }

    /// Load image from bytes
    pub fn load(&self, data: &[u8]) -> Result<DynamicImage> {
        let img = image::load_from_memory(data)
            .map_err(|e| VisionError::image(format!("Failed to load image: {}", e)))?;
        Ok(img)
    }

    /// Load image from file
    pub fn load_file(&self, path: &std::path::Path) -> Result<DynamicImage> {
        let img = image::open(path)
            .map_err(|e| VisionError::image(format!("Failed to load image from {:?}: {}", path, e)))?;
        Ok(img)
    }

    /// Load image from base64
    pub fn load_base64(&self, data: &str) -> Result<DynamicImage> {
        let bytes = base64::Engine::decode(&base64::engine::general_purpose::STANDARD, data)
            .map_err(|e| VisionError::Encoding(format!("Base64 decode error: {}", e)))?;
        self.load(&bytes)
    }

    /// Process image to target format
    pub fn process(&self, img: &DynamicImage) -> Result<ProcessedImage> {
        // Resize if needed
        let resized = self.resize_to_max(img);
        let size = ImageSize::new(resized.width(), resized.height());

        // Encode to bytes
        let data = self.encode(&resized, self.default_format)?;

        Ok(ProcessedImage::new(data, self.default_format, size))
    }

    /// Resize image to fit within max size
    pub fn resize_to_max(&self, img: &DynamicImage) -> DynamicImage {
        let (width, height) = img.dimensions();

        if width <= self.max_size && height <= self.max_size {
            return img.clone();
        }

        let ratio = (width as f64 / height as f64).min(1.0);
        let (new_width, new_height) = if width > height {
            (self.max_size, (self.max_size as f64 / ratio) as u32)
        } else {
            ((self.max_size as f64 * ratio) as u32, self.max_size)
        };

        img.resize(new_width, new_height, image::imageops::FilterType::Lanczos3)
    }

    /// Resize to exact dimensions
    pub fn resize(&self, img: &DynamicImage, width: u32, height: u32) -> DynamicImage {
        img.resize_exact(width, height, image::imageops::FilterType::Lanczos3)
    }

    /// Crop image
    pub fn crop(&self, img: &DynamicImage, x: u32, y: u32, width: u32, height: u32) -> DynamicImage {
        img.clone().crop(x, y, width, height)
    }

    /// Convert to grayscale
    pub fn to_grayscale(&self, img: &DynamicImage) -> DynamicImage {
        DynamicImage::ImageLuma8(img.to_luma8())
    }

    /// Convert to RGB
    pub fn to_rgb(&self, img: &DynamicImage) -> DynamicImage {
        DynamicImage::ImageRgb8(img.to_rgb8())
    }

    /// Encode image to bytes
    pub fn encode(&self, img: &DynamicImage, format: ImageFormat) -> Result<Vec<u8>> {
        let mut cursor = Cursor::new(Vec::new());

        let output_format = match format {
            ImageFormat::Png => ImgFormat::Png,
            ImageFormat::Jpeg => ImgFormat::Jpeg,
            ImageFormat::Gif => ImgFormat::Gif,
            ImageFormat::WebP => ImgFormat::WebP,
            ImageFormat::Bmp => ImgFormat::Bmp,
            ImageFormat::Tiff => ImgFormat::Tiff,
        };

        img.write_to(&mut cursor, output_format)
            .map_err(|e| VisionError::image(format!("Failed to encode image: {}", e)))?;

        Ok(cursor.into_inner())
    }

    /// Get image dimensions
    pub fn dimensions(&self, data: &[u8]) -> Result<ImageSize> {
        let img = self.load(data)?;
        Ok(ImageSize::new(img.width(), img.height()))
    }

    /// Detect image format from bytes
    pub fn detect_format(data: &[u8]) -> Option<ImageFormat> {
        if data.len() < 4 {
            return None;
        }

        // PNG: 89 50 4E 47
        if data[0..4] == [0x89, 0x50, 0x4E, 0x47] {
            return Some(ImageFormat::Png);
        }

        // JPEG: FF D8 FF
        if data[0..3] == [0xFF, 0xD8, 0xFF] {
            return Some(ImageFormat::Jpeg);
        }

        // GIF: 47 49 46 38
        if data[0..4] == [0x47, 0x49, 0x46, 0x38] {
            return Some(ImageFormat::Gif);
        }

        // WebP: 52 49 46 46 ... 57 45 42 50
        if data.len() >= 12 && data[0..4] == [0x52, 0x49, 0x46, 0x46] && data[8..12] == [0x57, 0x45, 0x42, 0x50] {
            return Some(ImageFormat::WebP);
        }

        // BMP: 42 4D
        if data[0..2] == [0x42, 0x4D] {
            return Some(ImageFormat::Bmp);
        }

        None
    }

    /// Extract dominant colors from image
    pub fn extract_colors(&self, img: &DynamicImage, num_colors: usize) -> Result<Vec<crate::types::DominantColor>> {
        use std::collections::HashMap;

        let mut color_counts: HashMap<(u8, u8, u8), usize> = HashMap::new();
        let rgb_img = img.to_rgb8();
        let total_pixels = rgb_img.width() * rgb_img.height();

        // Sample pixels
        for pixel in rgb_img.enumerate_pixels() {
            let r = pixel.2[0];
            let g = pixel.2[1];
            let b = pixel.2[2];

            // Quantize colors
            let qr = r / 32 * 32;
            let qg = g / 32 * 32;
            let qb = b / 32 * 32;

            *color_counts.entry((qr, qg, qb)).or_insert(0) += 1;
        }

        // Sort by count and take top N
        let mut colors: Vec<_> = color_counts.into_iter().collect();
        colors.sort_by(|a, b| b.1.cmp(&a.1));

        let result: Vec<crate::types::DominantColor> = colors
            .into_iter()
            .take(num_colors)
            .map(|((r, g, b), count)| {
                crate::types::DominantColor::new(
                    r,
                    g,
                    b,
                    count as f32 / total_pixels as f32,
                )
            })
            .collect();

        Ok(result)
    }

    /// Get image statistics
    pub fn statistics(&self, img: &DynamicImage) -> ImageStatistics {
        let (width, height) = img.dimensions();
        let luma = img.to_luma8();
        let mut histogram = [0u32; 256];

        for pixel in luma.pixels() {
            histogram[pixel[0] as usize] += 1;
        }

        // Calculate mean brightness
        let total_pixels = width * height;
        let mean_brightness: f32 = histogram
            .iter()
            .enumerate()
            .map(|(i, &count)| i as f32 * count as f32)
            .sum::<f32>()
            / total_pixels as f32;

        ImageStatistics {
            width,
            height,
            total_pixels,
            histogram,
            mean_brightness,
        }
    }
}

/// Image statistics
#[derive(Debug, Clone)]
pub struct ImageStatistics {
    pub width: u32,
    pub height: u32,
    pub total_pixels: u32,
    pub histogram: [u32; 256],
    pub mean_brightness: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_format_png() {
        let png_header = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
        assert_eq!(ImageProcessor::detect_format(&png_header), Some(ImageFormat::Png));
    }

    #[test]
    fn test_detect_format_jpeg() {
        let jpeg_header = vec![0xFF, 0xD8, 0xFF, 0xE0];
        assert_eq!(ImageProcessor::detect_format(&jpeg_header), Some(ImageFormat::Jpeg));
    }

    #[test]
    fn test_processor_creation() {
        let processor = ImageProcessor::new(1024);
        assert_eq!(processor.max_size, 1024);
    }

    #[test]
    fn test_image_size() {
        let processor = ImageProcessor::default();
        let size = ImageSize::new(1920, 1080);
        assert_eq!(size.width, 1920);
        assert_eq!(size.height, 1080);
    }
}
