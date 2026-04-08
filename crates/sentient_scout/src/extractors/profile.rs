//! ─── PROFIL CIKARICI ───

use super::{Extractor, ExtractedItem};
use crate::DataType;
use scraper::{Html, Selector};

/// Profil bilgileri cikarici
pub struct ProfileExtractor;

impl Extractor for ProfileExtractor {
    fn data_type(&self) -> DataType {
        DataType::UserProfile
    }
    
    fn extract(&self, html: &str, url: &str) -> Vec<ExtractedItem> {
        let document = Html::parse_document(html);
        let mut items = Vec::new();
        
        // Open Graph meta etiketleri
        let title = extract_meta_content(&document, "og:title");
        let description = extract_meta_content(&document, "og:description");
        let image = extract_meta_content(&document, "og:image");
        
        // Schema.org JSON-LD
        let json_ld = extract_json_ld(&document);
        
        // Temel profil bilgileri
        let profile_data = serde_json::json!({
            "title": title,
            "description": description,
            "image": image,
            "url": url,
            "schema_org": json_ld,
        });
        
        items.push(ExtractedItem {
            data_type: DataType::UserProfile,
            content: profile_data,
            confidence: if title.is_some() { 0.8 } else { 0.5 },
        });
        
        items
    }
}

fn extract_meta_content(document: &Html, property: &str) -> Option<String> {
    let selector = Selector::parse(&format!("meta[property=\"{}\"]", property)).ok()?;
    document.select(&selector).next()?.value().attr("content").map(|s| s.to_string())
}

fn extract_json_ld(document: &Html) -> Option<serde_json::Value> {
    let selector = Selector::parse("script[type=\"application/ld+json\"]").ok()?;
    let script = document.select(&selector).next()?;
    let text = script.text().collect::<String>();
    serde_json::from_str(&text).ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_extract_profile() {
        let html = r#"
        <html>
            <head>
                <meta property="og:title" content="Test User">
                <meta property="og:description" content="Test bio">
            </head>
        </html>
        "#;
        
        let extractor = ProfileExtractor;
        let items = extractor.extract(html, "https://example.com/user");
        
        assert!(!items.is_empty());
        assert_eq!(items[0].data_type, DataType::UserProfile);
    }
}
