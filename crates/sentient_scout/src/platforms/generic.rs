//! ─── GENERIC HANDLER ───
//!
//! Tum platformlar icin genel amacli handler

use super::PlatformHandler;
use crate::{ScrapedData, SearchParams, Platform, DataType};
use async_trait::async_trait;

/// Generic handler - HTML tabanli platformlar icin
pub struct GenericHandler {
    platform: Platform,
}

impl GenericHandler {
    pub fn new(platform: Platform) -> Self {
        Self { platform }
    }
}

#[async_trait]
impl PlatformHandler for GenericHandler {
    fn platform(&self) -> Platform {
        self.platform
    }
    
    async fn search(&self, params: SearchParams, client: &reqwest::Client) -> crate::Result<Vec<ScrapedData>> {
        log::debug!("[Generic/{}] Arama: {}", self.platform.domain(), params.query);
        
        let search_url = format!(
            "https://www.{}/search?q={}",
            self.platform.domain(),
            urlencoding::encode(&params.query)
        );
        
        let response = client
            .get(&search_url)
            .header("Accept", "text/html")
            .send()
            .await
            .map_err(|e| crate::ScoutError::Connection(e.to_string()))?;
        
        let html = response.text().await
            .map_err(|e| crate::ScoutError::ParseError(e.to_string()))?;
        
        // Basit link cikarma
        let links = extract_links(&html, self.platform.domain());
        
        let results: Vec<ScrapedData> = links
            .into_iter()
            .take(params.limit)
            .map(|link| ScrapedData {
                id: uuid::Uuid::new_v4(),
                platform: self.platform,
                data_type: DataType::SearchResult,
                raw: link.url.clone(),
                metadata: [
                    ("title".into(), link.title),
                    ("url".into(), link.url),
                ].into_iter().collect(),
                scraped_at: chrono::Utc::now(),
                source_url: search_url.clone(),
            })
            .collect();
        
        log::info!("[Generic/{}] {} sonuc bulundu", self.platform.domain(), results.len());
        Ok(results)
    }
    
    async fn get_profile(&self, username: &str, client: &reqwest::Client) -> crate::Result<ScrapedData> {
        let url = format!("https://www.{}/{}", self.platform.domain(), username);
        
        let response = client
            .get(&url)
            .send()
            .await
            .map_err(|e| crate::ScoutError::Connection(e.to_string()))?;
        
        let html = response.text().await
            .map_err(|e| crate::ScoutError::ParseError(e.to_string()))?;
        
        Ok(ScrapedData {
            id: uuid::Uuid::new_v4(),
            platform: self.platform,
            data_type: DataType::UserProfile,
            raw: html,
            metadata: [
                ("username".into(), username.into()),
            ].into_iter().collect(),
            scraped_at: chrono::Utc::now(),
            source_url: url,
        })
    }
    
    async fn get_trending(&self, client: &reqwest::Client) -> crate::Result<Vec<ScrapedData>> {
        // Her platform icin ozel trending endpoint'i gerekli
        Ok(vec![])
    }
    
    fn supported_data_types(&self) -> Vec<DataType> {
        vec![
            DataType::UserProfile,
            DataType::SearchResult,
        ]
    }
}

struct ExtractedLink {
    title: String,
    url: String,
}

fn extract_links(html: &str, domain: &str) -> Vec<ExtractedLink> {
    // Basit regex tabanli link cikarma
    let re = regex::Regex::new(r#"<a[^>]*href="([^"]*)"[^>]*>([^<]*)</a>"#).unwrap();
    
    re.captures_iter(html)
        .filter_map(|cap| {
            let url = cap.get(1)?.as_str().to_string();
            let title = cap.get(2)?.as_str().trim().to_string();
            
            // Sadece ayni domain'deki linkler
            if url.contains(domain) || url.starts_with('/') {
                Some(ExtractedLink { title, url })
            } else {
                None
            }
        })
        .collect()
}

mod urlencoding {
    pub fn encode(s: &str) -> String {
        url::form_urlencoded::byte_serialize(s.as_bytes()).collect()
    }
}
