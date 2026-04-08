//! ─── SENTIENT SCOUT ───
//!
//! Sosyal medya ve iş platformlarından derin veri çekme modulu.
//! Anti-detection, rate limiting ve proxy yonetimi icerir.

pub mod platforms;
pub mod extractors;
pub mod anti_detect;
pub mod proxy;
mod config;
mod errors;
mod session;
mod rate_limiter;

// Re-exports
pub use config::*;
pub use errors::*;
pub use session::*;
pub use rate_limiter::*;

use sentient_common::error::{SENTIENTError, SENTIENTResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Scout agent - Ana veri toplama birimi
pub struct ScoutAgent {
    /// Agent kimligi
    id: uuid::Uuid,
    /// Yapilandirma
    config: ScoutConfig,
    /// HTTP istemcisi
    client: reqwest::Client,
    /// Oturum bilgileri
    session: Arc<RwLock<ScoutSession>>,
    /// Platform erisimleri
    platforms: HashMap<Platform, Box<dyn platforms::PlatformHandler + Send + Sync>>,
}

/// Platform turleri
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Platform {
    // Sosyal Medya
    Twitter,
    Instagram,
    Facebook,
    LinkedIn,
    TikTok,
    Reddit,
    
    // Is Platformlari
    GitHub,
    StackOverflow,
    Medium,
    DevTo,
    Kaggle,
    HuggingFace,
    
    // E-ticaret
    Amazon,
    eBay,
    Trendyol,
    
    // Haberler
    GoogleNews,
    HackerNews,
    ProductHunt,
    
    // Arama
    Google,
    DuckDuckGo,
    Bing,
}

impl Platform {
    pub fn domain(&self) -> &'static str {
        match self {
            Platform::Twitter => "twitter.com",
            Platform::Instagram => "instagram.com",
            Platform::Facebook => "facebook.com",
            Platform::LinkedIn => "linkedin.com",
            Platform::TikTok => "tiktok.com",
            Platform::Reddit => "reddit.com",
            Platform::GitHub => "github.com",
            Platform::StackOverflow => "stackoverflow.com",
            Platform::Medium => "medium.com",
            Platform::DevTo => "dev.to",
            Platform::Kaggle => "kaggle.com",
            Platform::HuggingFace => "huggingface.co",
            Platform::Amazon => "amazon.com",
            Platform::eBay => "ebay.com",
            Platform::Trendyol => "trendyol.com",
            Platform::GoogleNews => "news.google.com",
            Platform::HackerNews => "news.ycombinator.com",
            Platform::ProductHunt => "producthunt.com",
            Platform::Google => "google.com",
            Platform::DuckDuckGo => "duckduckgo.com",
            Platform::Bing => "bing.com",
        }
    }
    
    pub fn rate_limit(&self) -> u32 {
        match self {
            Platform::Twitter => 1,          // 1 istek/saniye
            Platform::Instagram => 1,
            Platform::LinkedIn => 1,
            Platform::GitHub => 10,
            Platform::Reddit => 2,
            _ => 5,
        }
    }
    
    pub fn requires_auth(&self) -> bool {
        matches!(self, Platform::LinkedIn | Platform::Instagram | Platform::Facebook)
    }
}

/// Platform kategorisi
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlatformCategory {
    SocialMedia,
    JobBoards,
    Developer,
    ECommerce,
    News,
    Search,
}

/// Toplanan veri turu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrapedData {
    /// Veri kimligi
    pub id: uuid::Uuid,
    /// Kaynak platform
    pub platform: Platform,
    /// Veri tipi
    pub data_type: DataType,
    /// Ham veri
    pub raw: String,
    /// Islemler
    pub metadata: HashMap<String, String>,
    /// Toplanma zamani
    pub scraped_at: chrono::DateTime<chrono::Utc>,
    /// Kaynak URL
    pub source_url: String,
}

/// Veri tipleri
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DataType {
    // Profil
    UserProfile,
    CompanyProfile,
    
    // Icerik
    Post,
    Comment,
    Article,
    Review,
    
    // Medya
    Image,
    Video,
    
    // Metrikler
    Followers,
    Engagement,
    Stats,
    
    // Arama
    SearchResult,
    Trending,
    
    // Diger
    Custom(String),
}

/// Arama parametreleri
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchParams {
    /// Arama sorgusu
    pub query: String,
    /// Maksimum sonuc sayisi
    pub limit: usize,
    /// Zaman araligi
    pub time_range: Option<TimeRange>,
    /// Siralama
    pub sort: SortOrder,
    /// Filtreler
    pub filters: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TimeRange {
    Hour,
    Day,
    Week,
    Month,
    Year,
    Custom { start: chrono::DateTime<chrono::Utc>, end: chrono::DateTime<chrono::Utc> },
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SortOrder {
    Relevance,
    Recent,
    Popular,
}

impl Default for SearchParams {
    fn default() -> Self {
        Self {
            query: String::new(),
            limit: 50,
            time_range: None,
            sort: SortOrder::Relevance,
            filters: HashMap::new(),
        }
    }
}

impl ScoutAgent {
    /// Yeni Scout agent olustur
    pub async fn new(config: ScoutConfig) -> SENTIENTResult<Self> {
        let client = Self::build_client(&config)?;
        
        Ok(Self {
            id: uuid::Uuid::new_v4(),
            config,
            client,
            session: Arc::new(RwLock::new(ScoutSession::new())),
            platforms: HashMap::new(),
        })
    }
    
    /// HTTP istemcisi olustur
    fn build_client(config: &ScoutConfig) -> SENTIENTResult<reqwest::Client> {
        let mut builder = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeout_secs))
            .user_agent(&config.user_agent);
        
        // Proxy yapilandirmasi
        if let Some(ref proxy) = config.proxy {
            let proxy_url = format!("{}://{}:{}", proxy.protocol, proxy.host, proxy.port);
            let req_proxy = match proxy.protocol.as_str() {
                "http" => reqwest::Proxy::http(&proxy_url).map_err(|e| SENTIENTError::VGate(e.to_string()))?,
                "https" => reqwest::Proxy::https(&proxy_url).map_err(|e| SENTIENTError::VGate(e.to_string()))?,
                "socks5" => reqwest::Proxy::all(&proxy_url).map_err(|e| SENTIENTError::VGate(e.to_string()))?,
                _ => return Err(SENTIENTError::General("Desteklenmeyen proxy protokolu".into())),
            };
            builder = builder.proxy(req_proxy);
        }
        
        builder.build().map_err(|e| SENTIENTError::General(format!("HTTP istemci hatasi: {}", e)))
    }
    
    /// Platformdan veri topla
    pub async fn scrape(&self, platform: Platform, params: SearchParams) -> SENTIENTResult<Vec<ScrapedData>> {
        log::info!("[Scout] {} uzerinden '{}' araniyor...", platform.domain(), params.query);
        
        // Rate limit kontrolu
        self.check_rate_limit(platform).await?;
        
        // Platform handler'i al
        let handler = self.platforms.get(&platform)
            .ok_or_else(|| SENTIENTError::General(format!("Platform destegi yok: {:?}", platform)))?;
        
        // Anti-detection uygulamasi
        let mut session = self.session.write().await;
        session.apply_stealth(&self.config.stealth);
        
        // Veri toplama
        let results = handler.search(params, &self.client).await.map_err(SENTIENTError::from)?;
        
        // Oturum istatistikleri guncelle
        session.record_request(platform, results.len());
        
        Ok(results)
    }
    
    /// Profil bilgisi cek
    pub async fn get_profile(&self, platform: Platform, username: &str) -> SENTIENTResult<ScrapedData> {
        log::info!("[Scout] {} profil cekiliyor: {}", platform.domain(), username);
        
        self.check_rate_limit(platform).await?;
        
        let handler = self.platforms.get(&platform)
            .ok_or_else(|| SENTIENTError::General(format!("Platform destegi yok: {:?}", platform)))?;
        
        handler.get_profile(username, &self.client).await.map_err(SENTIENTError::from)
    }
    
    /// Trend topicleri cek
    pub async fn get_trending(&self, platform: Platform) -> SENTIENTResult<Vec<ScrapedData>> {
        log::info!("[Scout] {} trendleri cekiliyor...", platform.domain());
        
        self.check_rate_limit(platform).await?;
        
        let handler = self.platforms.get(&platform)
            .ok_or_else(|| SENTIENTError::General(format!("Platform destegi yok: {:?}", platform)))?;
        
        handler.get_trending(&self.client).await.map_err(SENTIENTError::from)
    }
    
    /// Rate limit kontrolu
    async fn check_rate_limit(&self, platform: Platform) -> SENTIENTResult<()> {
        let session = self.session.read().await;
        let rate_limiter = session.rate_limiter(platform);
        
        // TODO: Governor rate limiting implementasyonu
        
        Ok(())
    }
    
    /// Oturum durumu
    pub async fn status(&self) -> ScoutStatus {
        let session = self.session.read().await;
        ScoutStatus {
            id: self.id,
            total_requests: session.total_requests(),
            success_rate: session.success_rate(),
            active_platforms: session.active_platforms(),
        }
    }
}

/// Scout durumu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoutStatus {
    pub id: uuid::Uuid,
    pub total_requests: u64,
    pub success_rate: f64,
    pub active_platforms: Vec<Platform>,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_platform_domain() {
        assert_eq!(Platform::Twitter.domain(), "twitter.com");
        assert_eq!(Platform::GitHub.domain(), "github.com");
    }
    
    #[test]
    fn test_platform_rate_limit() {
        assert_eq!(Platform::Twitter.rate_limit(), 1);
        assert_eq!(Platform::GitHub.rate_limit(), 10);
    }
    
    #[test]
    fn test_search_params_default() {
        let params = SearchParams::default();
        assert_eq!(params.limit, 50);
    }
}
