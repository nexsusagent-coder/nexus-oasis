//! ─── POST CIKARICI ───

use super::{Extractor, ExtractedItem};
use crate::DataType;
use scraper::{Html, Selector};

/// Sosyal medya postu cikarici
pub struct PostExtractor;

#[derive(Debug, serde::Serialize)]
struct PostData {
    text: String,
    author: Option<String>,
    timestamp: Option<String>,
    likes: u32,
    comments: u32,
    shares: u32,
    media_urls: Vec<String>,
}

impl Extractor for PostExtractor {
    fn data_type(&self) -> DataType {
        DataType::Post
    }
    
    fn extract(&self, html: &str, url: &str) -> Vec<ExtractedItem> {
        let document = Html::parse_document(html);
        let mut items = Vec::new();
        
        // Twitter tweet secici
        if url.contains("twitter.com") || url.contains("x.com") {
            items.extend(extract_twitter_posts(&document, url));
        }
        
        // Reddit post secici
        if url.contains("reddit.com") {
            items.extend(extract_reddit_posts(&document, url));
        }
        
        // Genel makale icerigi
        if items.is_empty() {
            items.extend(extract_article_content(&document, url));
        }
        
        items
    }
}

fn extract_twitter_posts(document: &Html, url: &str) -> Vec<ExtractedItem> {
    let mut items = Vec::new();
    
    // Tweet text
    let tweet_selector = Selector::parse("[data-testid=\"tweet\"]").ok();
    if let Some(selector) = tweet_selector {
        for tweet in document.select(&selector) {
            let text_selector = Selector::parse("[data-testid=\"tweetText\"]").ok();
            let text = text_selector
                .and_then(|s| tweet.select(&s).next())
                .map(|el| el.text().collect::<String>())
                .unwrap_or_default();
            
            if !text.is_empty() {
                items.push(ExtractedItem {
                    data_type: DataType::Post,
                    content: serde_json::json!({
                        "platform": "twitter",
                        "text": text,
                        "url": url,
                    }),
                    confidence: 0.9,
                });
            }
        }
    }
    
    items
}

fn extract_reddit_posts(document: &Html, url: &str) -> Vec<ExtractedItem> {
    let mut items = Vec::new();
    
    // Post title
    let title_selector = Selector::parse("shreddit-title").ok();
    let title = title_selector
        .and_then(|s| document.select(&s).next())
        .and_then(|el| el.value().attr("title"))
        .unwrap_or_default();
    
    // Post content
    let content_selector = Selector::parse("div[slot=\"text-body\"]").ok();
    let content = content_selector
        .and_then(|s| document.select(&s).next())
        .map(|el| el.text().collect::<String>())
        .unwrap_or_default();
    
    if !title.is_empty() {
        items.push(ExtractedItem {
            data_type: DataType::Post,
            content: serde_json::json!({
                "platform": "reddit",
                "title": title,
                "content": content,
                "url": url,
            }),
            confidence: 0.85,
        });
    }
    
    items
}

fn extract_article_content(document: &Html, url: &str) -> Vec<ExtractedItem> {
    let mut items = Vec::new();
    
    // Article tag
    let article_selector = Selector::parse("article").ok();
    let content = article_selector
        .and_then(|s| document.select(&s).next())
        .map(|el| el.text().collect::<String>())
        .or_else(|| {
            // Main content
            Selector::parse("main, [role=\"main\"]").ok()
                .and_then(|s| document.select(&s).next())
                .map(|el| el.text().collect::<String>())
        })
        .unwrap_or_default();
    
    if !content.is_empty() {
        items.push(ExtractedItem {
            data_type: DataType::Article,
            content: serde_json::json!({
                "platform": "web",
                "content": content.chars().take(5000).collect::<String>(),
                "url": url,
            }),
            confidence: 0.7,
        });
    }
    
    items
}
