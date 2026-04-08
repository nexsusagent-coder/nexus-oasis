//! ─── VERI CIKARICILAR ───
//!
//! Farkli veri tiplerini HTML'den cikarma

pub mod profile;
pub mod post;
pub mod metrics;
pub mod media;

use crate::{DataType, ScrapedData};
use serde::{Deserialize, Serialize};

/// Cikarici trait
pub trait Extractor: Send + Sync {
    /// Desteklenen veri tipi
    fn data_type(&self) -> DataType;
    
    /// HTML'den veri cikar
    fn extract(&self, html: &str, url: &str) -> Vec<ExtractedItem>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedItem {
    pub data_type: DataType,
    pub content: serde_json::Value,
    pub confidence: f32,
}

/// Cikarici factory
pub fn create_extractor(data_type: DataType) -> Box<dyn Extractor> {
    match data_type {
        DataType::UserProfile => Box::new(profile::ProfileExtractor),
        DataType::Post => Box::new(post::PostExtractor),
        DataType::Engagement | DataType::Followers | DataType::Stats => {
            Box::new(metrics::MetricsExtractor)
        }
        DataType::Image | DataType::Video => {
            Box::new(media::MediaExtractor)
        }
        _ => Box::new(generic_extractor::GenericExtractor(data_type)),
    }
}

mod generic_extractor {
    use super::*;
    
    pub struct GenericExtractor(pub DataType);
    
    impl Extractor for GenericExtractor {
        fn data_type(&self) -> DataType {
            self.0.clone()
        }
        
        fn extract(&self, html: &str, url: &str) -> Vec<ExtractedItem> {
            vec![ExtractedItem {
                data_type: self.0.clone(),
                content: serde_json::json!({"html": html, "url": url}),
                confidence: 0.5,
            }]
        }
    }
}
