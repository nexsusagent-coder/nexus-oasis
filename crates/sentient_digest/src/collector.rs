//! Data collectors for digest sections

use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{DigestConfig, DigestError, DigestResult, DigestSection, SectionType};

/// Collector trait - gather data for a section
#[async_trait]
pub trait Collector: Send + Sync {
    /// Section type this collector handles
    fn section_type(&self) -> SectionType;

    /// Collect data
    async fn collect(&self, config: &DigestConfig) -> DigestResult<Option<DigestSection>>;
}

/// Registry of collectors
pub struct CollectorRegistry {
    collectors: HashMap<SectionType, Arc<RwLock<Box<dyn Collector>>>>,
}

impl CollectorRegistry {
    pub fn new() -> Self {
        Self {
            collectors: HashMap::new(),
        }
    }

    pub fn register(&mut self, collector: Box<dyn Collector>) {
        let section_type = collector.section_type();
        self.collectors.insert(section_type, Arc::new(RwLock::new(collector)));
    }

    pub async fn collect(&self, section_type: &SectionType, config: &DigestConfig) -> DigestResult<Option<DigestSection>> {
        if let Some(collector) = self.collectors.get(section_type) {
            let guard = collector.read().await;
            guard.collect(config).await
        } else {
            Ok(None)
        }
    }

    pub fn list(&self) -> Vec<&SectionType> {
        self.collectors.keys().collect()
    }
}

impl Default for CollectorRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// BUILT-IN COLLECTORS
// ═══════════════════════════════════════════════════════════════════════════════

/// Weather collector
pub struct WeatherCollector {
    location: String,
}

impl WeatherCollector {
    pub fn new(location: &str) -> Self {
        Self {
            location: location.to_string(),
        }
    }
}

#[async_trait]
impl Collector for WeatherCollector {
    fn section_type(&self) -> SectionType {
        SectionType::Weather
    }

    async fn collect(&self, _config: &DigestConfig) -> DigestResult<Option<DigestSection>> {
        // In a real implementation, this would call the weather connector
        let section = DigestSection::new(SectionType::Weather, "🌤️ Hava Durumu")
            .with_content(&format!("{} için hava durumu bilgisi", self.location))
            .with_icon("🌤️");

        Ok(Some(section))
    }
}

/// Calendar collector
pub struct CalendarCollector;

impl CalendarCollector {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Collector for CalendarCollector {
    fn section_type(&self) -> SectionType {
        SectionType::Calendar
    }

    async fn collect(&self, _config: &DigestConfig) -> DigestResult<Option<DigestSection>> {
        let section = DigestSection::new(SectionType::Calendar, "📅 Bugünkü Etkinlikler")
            .with_content("Bugün için planlanmış etkinlikleriniz:")
            .with_icon("📅");

        Ok(Some(section))
    }
}

impl Default for CalendarCollector {
    fn default() -> Self {
        Self::new()
    }
}

/// Email collector
pub struct EmailCollector {
    unread_count: usize,
}

impl EmailCollector {
    pub fn new(unread_count: usize) -> Self {
        Self { unread_count }
    }
}

#[async_trait]
impl Collector for EmailCollector {
    fn section_type(&self) -> SectionType {
        SectionType::Email
    }

    async fn collect(&self, _config: &DigestConfig) -> DigestResult<Option<DigestSection>> {
        let content = if self.unread_count > 0 {
            format!("{} okunmamış e-postanız var.", self.unread_count)
        } else {
            "Okunmamış e-postanız yok.".to_string()
        };

        let section = DigestSection::new(SectionType::Email, "📧 E-postalar")
            .with_content(&content)
            .with_icon("📧");

        Ok(Some(section))
    }
}

impl Default for EmailCollector {
    fn default() -> Self {
        Self::new(0)
    }
}

/// News collector
pub struct NewsCollector {
    feeds: Vec<String>,
}

impl NewsCollector {
    pub fn new(feeds: Vec<&str>) -> Self {
        Self {
            feeds: feeds.into_iter().map(|s| s.to_string()).collect(),
        }
    }
}

#[async_trait]
impl Collector for NewsCollector {
    fn section_type(&self) -> SectionType {
        SectionType::News
    }

    async fn collect(&self, _config: &DigestConfig) -> DigestResult<Option<DigestSection>> {
        let section = DigestSection::new(SectionType::News, "📰 Haberler")
            .with_content(&format!("{} kaynaktan haber başlıkları", self.feeds.len()))
            .with_icon("📰");

        Ok(Some(section))
    }
}

impl Default for NewsCollector {
    fn default() -> Self {
        Self::new(vec![])
    }
}

/// Greeting collector
pub struct GreetingCollector;

impl GreetingCollector {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Collector for GreetingCollector {
    fn section_type(&self) -> SectionType {
        SectionType::Greeting
    }

    async fn collect(&self, config: &DigestConfig) -> DigestResult<Option<DigestSection>> {
        let greeting = match config.time_of_day {
            crate::TimeOfDay::Morning => "Günaydın",
            crate::TimeOfDay::Afternoon => "Tünaydın",
            crate::TimeOfDay::Evening => "İyi akşamlar",
        };

        let content = if let Some(ref user) = config.user_name {
            format!("{}, {}! Size bugün nasıl yardımcı olabilirim?", greeting, user)
        } else {
            format!("{}! Size bugün nasıl yardımcı olabilirim?", greeting)
        };

        let section = DigestSection::new(SectionType::Greeting, "")
            .with_content(&content)
            .with_icon("👋");

        Ok(Some(section))
    }
}

impl Default for GreetingCollector {
    fn default() -> Self {
        Self::new()
    }
}

/// Tasks collector
pub struct TasksCollector;

impl TasksCollector {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Collector for TasksCollector {
    fn section_type(&self) -> SectionType {
        SectionType::Tasks
    }

    async fn collect(&self, _config: &DigestConfig) -> DigestResult<Option<DigestSection>> {
        let section = DigestSection::new(SectionType::Tasks, "✅ Görevler")
            .with_content("Bugün için görevleriniz:")
            .with_icon("✅");

        Ok(Some(section))
    }
}

impl Default for TasksCollector {
    fn default() -> Self {
        Self::new()
    }
}
