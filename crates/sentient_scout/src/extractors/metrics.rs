//! ─── METRIK CIKARICI ───

use super::{Extractor, ExtractedItem};
use crate::DataType;
use scraper::{Html, Selector};

/// Sosyal medya metrikleri cikarici
pub struct MetricsExtractor;

#[derive(Debug, serde::Serialize)]
struct Metrics {
    followers: u32,
    following: u32,
    posts: u32,
    likes: u32,
    engagement_rate: f32,
}

impl Extractor for MetricsExtractor {
    fn data_type(&self) -> DataType {
        DataType::Stats
    }
    
    fn extract(&self, html: &str, url: &str) -> Vec<ExtractedItem> {
        let document = Html::parse_document(html);
        let mut items = Vec::new();
        
        // Twitter/X metrikleri
        if url.contains("twitter.com") || url.contains("x.com") {
            items.extend(extract_twitter_metrics(&document, url));
        }
        
        // GitHub metrikleri
        if url.contains("github.com") {
            items.extend(extract_github_metrics(&document, url));
        }
        
        items
    }
}

fn extract_twitter_metrics(document: &Html, url: &str) -> Vec<ExtractedItem> {
    let mut items = Vec::new();
    
    // Takipci sayisi
    let followers_selector = Selector::parse("[href$=\"/followers\"]").ok();
    let followers = followers_selector
        .and_then(|s| document.select(&s).next())
        .and_then(|el| el.text().last())
        .and_then(|t| parse_count(t))
        .unwrap_or(0);
    
    // Takip edilen
    let following_selector = Selector::parse("[href$=\"/following\"]").ok();
    let following = following_selector
        .and_then(|s| document.select(&s).next())
        .and_then(|el| el.text().last())
        .and_then(|t| parse_count(t))
        .unwrap_or(0);
    
    items.push(ExtractedItem {
        data_type: DataType::Followers,
        content: serde_json::json!({
            "platform": "twitter",
            "followers": followers,
            "following": following,
        }),
        confidence: 0.8,
    });
    
    items
}

fn extract_github_metrics(document: &Html, url: &str) -> Vec<ExtractedItem> {
    let mut items = Vec::new();
    
    // Repository sayisi
    let repos_selector = Selector::parse("[itemprop=\"owns\"]").ok();
    // Parse edilecek...
    
    items.push(ExtractedItem {
        data_type: DataType::Stats,
        content: serde_json::json!({
            "platform": "github",
        }),
        confidence: 0.6,
    });
    
    items
}

fn parse_count(s: &str) -> Option<u32> {
    let s = s.trim().replace(',', "");
    if s.ends_with('K') {
        let num: f32 = s.trim_end_matches('K').parse().ok()?;
        Some((num * 1000.0) as u32)
    } else if s.ends_with('M') {
        let num: f32 = s.trim_end_matches('M').parse().ok()?;
        Some((num * 1_000_000.0) as u32)
    } else {
        s.parse().ok()
    }
}
