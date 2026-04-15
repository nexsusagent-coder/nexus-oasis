//! RSS/Atom feed connector

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

use crate::{
    Connector, ConnectorError, ConnectorResult, ConnectorStatus, Credentials,
    CredentialType, Document, DocumentType, FeedItem, SyncConfig, SyncResult,
};

/// RSS feed connector
pub struct RssConnector {
    feed_urls: Vec<String>,
    status: ConnectorStatus,
    last_sync: Option<DateTime<Utc>>,
    config: SyncConfig,
    client: reqwest::Client,
}

impl RssConnector {
    pub fn new() -> Self {
        Self {
            feed_urls: Vec::new(),
            status: ConnectorStatus::Disconnected,
            last_sync: None,
            config: SyncConfig::default(),
            client: reqwest::Client::new(),
        }
    }

    pub fn with_feeds(mut self, feeds: Vec<&str>) -> Self {
        self.feed_urls = feeds.iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn add_feed(&mut self, url: &str) {
        if !self.feed_urls.contains(&url.to_string()) {
            self.feed_urls.push(url.to_string());
        }
    }

    /// Fetch a single feed
    pub async fn fetch_feed(&self, url: &str) -> ConnectorResult<Vec<FeedItem>> {
        let response = self.client
            .get(url)
            .send()
            .await?
            .text()
            .await?;

        self.parse_feed(url, &response)
    }

    /// Parse RSS/Atom feed
    fn parse_feed(&self, feed_url: &str, content: &str) -> ConnectorResult<Vec<FeedItem>> {
        use feed_rs::parser;

        let feed = parser::parse(content.as_bytes())
            .map_err(|e| ConnectorError::ParseError(format!("Feed parse error: {}", e)))?;

        let feed_title = feed.title.as_ref().map(|t| t.content.clone()).unwrap_or_default();

        let items: Vec<FeedItem> = feed.entries.iter().map(|entry| {
            FeedItem {
                id: entry.id.clone(),
                feed_url: feed_url.to_string(),
                feed_title: feed_title.clone(),
                title: entry.title.as_ref().map(|t| t.content.clone()).unwrap_or_default(),
                description: entry.summary.as_ref().map(|s| s.content.clone()),
                content: entry.content.as_ref().and_then(|c| c.body.clone()),
                link: entry.links.first().map(|l| l.href.clone()).unwrap_or_default(),
                published: entry.published.or(entry.updated),
                author: entry.authors.first().map(|a| a.name.clone()),
                categories: entry.categories.iter().map(|c| c.term.clone()).collect(),
            }
        }).collect();

        Ok(items)
    }

    /// Fetch all feeds
    pub async fn fetch_all(&self) -> ConnectorResult<Vec<FeedItem>> {
        let mut all_items = Vec::new();
        
        for url in &self.feed_urls {
            match self.fetch_feed(url).await {
                Ok(items) => all_items.extend(items),
                Err(e) => log::warn!("Failed to fetch feed {}: {}", url, e),
            }
        }

        // Sort by published date (newest first)
        all_items.sort_by(|a, b| {
            b.published.cmp(&a.published)
        });

        Ok(all_items)
    }
}

impl Default for RssConnector {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Connector for RssConnector {
    fn connector_id(&self) -> &str { "rss" }
    fn connector_name(&self) -> &str { "RSS/Atom Feeds" }
    fn category(&self) -> &str { "news" }
    fn status(&self) -> ConnectorStatus { self.status.clone() }
    fn required_credentials(&self) -> Vec<String> { Vec::new() }

    async fn connect(&mut self, _credentials: Credentials) -> ConnectorResult<()> {
        self.status = ConnectorStatus::Connected;
        Ok(())
    }

    async fn disconnect(&mut self) -> ConnectorResult<()> {
        self.status = ConnectorStatus::Disconnected;
        Ok(())
    }

    async fn test_connection(&self) -> ConnectorResult<bool> {
        if self.feed_urls.is_empty() { return Ok(false); }
        self.fetch_feed(&self.feed_urls[0]).await.map(|_| true)
    }

    async fn sync(&self, since: Option<DateTime<Utc>>, config: &SyncConfig) -> ConnectorResult<SyncResult> {
        let mut result = SyncResult::new(self.connector_id());
        let items = self.fetch_all().await?;
        
        let filtered: Vec<_> = if let Some(since) = since {
            items.into_iter().filter(|i| i.published.map(|p| p > since).unwrap_or(false)).collect()
        } else {
            items
        };

        result.items_new = filtered.len();
        result.items_synced = filtered.len();
        Ok(result)
    }

    async fn fetch(&self, query: &str, limit: usize) -> ConnectorResult<Vec<Document>> {
        let items = self.fetch_all().await?;
        
        Ok(items.into_iter().take(limit).map(|item| {
            Document::new("rss", DocumentType::Article, &item.id, &item.title)
                .with_content(item.content.as_deref().unwrap_or_else(|| 
                    item.description.as_deref().unwrap_or("")))
                .with_author(item.author.as_deref().unwrap_or(&item.feed_title))
                .with_url(&item.link)
        }).collect())
    }

    async fn get_document(&self, id: &str) -> ConnectorResult<Option<Document>> {
        let docs = self.fetch("", 1000).await?;
        Ok(docs.into_iter().find(|d| d.id == id))
    }

    async fn search(&self, query: &str, limit: usize) -> ConnectorResult<Vec<Document>> {
        let docs = self.fetch("", 1000).await?;
        let query_lower = query.to_lowercase();
        
        Ok(docs.into_iter()
            .filter(|d| d.title.to_lowercase().contains(&query_lower) || 
                       d.content.to_lowercase().contains(&query_lower))
            .take(limit)
            .collect())
    }

    fn last_sync(&self) -> Option<DateTime<Utc>> { self.last_sync }
    fn set_config(&mut self, config: SyncConfig) { self.config = config; }
    fn config(&self) -> &SyncConfig { &self.config }
}
