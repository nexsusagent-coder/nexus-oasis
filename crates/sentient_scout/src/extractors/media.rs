//! ─── MEDYA CIKARICI ───

use super::{Extractor, ExtractedItem};
use crate::DataType;
use scraper::{Html, Selector};

/// Medya dosyaları cikarici
pub struct MediaExtractor;

impl Extractor for MediaExtractor {
    fn data_type(&self) -> DataType {
        DataType::Image
    }
    
    fn extract(&self, html: &str, url: &str) -> Vec<ExtractedItem> {
        let document = Html::parse_document(html);
        let mut items = Vec::new();
        
        // Resimler
        let img_selector = Selector::parse("img").unwrap();
        for img in document.select(&img_selector) {
            if let Some(src) = img.value().attr("src") {
                if let Some(alt) = img.value().attr("alt") {
                    items.push(ExtractedItem {
                        data_type: DataType::Image,
                        content: serde_json::json!({
                            "url": src,
                            "alt": alt,
                        }),
                        confidence: 0.7,
                    });
                }
            }
        }
        
        // Videolar
        let video_selector = Selector::parse("video source, video").unwrap();
        for video in document.select(&video_selector) {
            if let Some(src) = video.value().attr("src").or_else(|| video.value().attr("poster")) {
                items.push(ExtractedItem {
                    data_type: DataType::Video,
                    content: serde_json::json!({
                        "url": src,
                    }),
                    confidence: 0.8,
                });
            }
        }
        
        items
    }
}
