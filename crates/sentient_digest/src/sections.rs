//! Digest sections - section builders and utilities

use crate::{DigestConfig, DigestItem, DigestResult, DigestSection, SectionType};

/// Section builder trait
pub trait Section: Send + Sync {
    fn section_type(&self) -> SectionType;
    fn title(&self, config: &DigestConfig) -> String;
    fn icon(&self) -> &'static str;
    fn build(&self, config: &DigestConfig) -> DigestResult<DigestSection>;
}

/// Default section implementations
pub struct WeatherSection;
pub struct CalendarSection;
pub struct EmailSection;
pub struct NewsSection;
pub struct TasksSection;
pub struct GreetingSection;

impl Section for WeatherSection {
    fn section_type(&self) -> SectionType { SectionType::Weather }
    fn title(&self, _config: &DigestConfig) -> String { "Hava Durumu".to_string() }
    fn icon(&self) -> &'static str { "🌤️" }
    
    fn build(&self, config: &DigestConfig) -> DigestResult<DigestSection> {
        let location = config.location.as_deref().unwrap_or("konum");
        Ok(DigestSection::new(SectionType::Weather, &self.title(config))
            .with_content(&format!("{} için hava durumu", location))
            .with_icon(self.icon()))
    }
}

impl Section for CalendarSection {
    fn section_type(&self) -> SectionType { SectionType::Calendar }
    fn title(&self, _config: &DigestConfig) -> String { "Bugünkü Etkinlikler".to_string() }
    fn icon(&self) -> &'static str { "📅" }
    
    fn build(&self, config: &DigestConfig) -> DigestResult<DigestSection> {
        Ok(DigestSection::new(SectionType::Calendar, &self.title(config))
            .with_content("Bugün için planlanmış etkinlikleriniz:")
            .with_icon(self.icon()))
    }
}

impl Section for EmailSection {
    fn section_type(&self) -> SectionType { SectionType::Email }
    fn title(&self, _config: &DigestConfig) -> String { "E-postalar".to_string() }
    fn icon(&self) -> &'static str { "📧" }
    
    fn build(&self, _config: &DigestConfig) -> DigestResult<DigestSection> {
        Ok(DigestSection::new(SectionType::Email, &self.title(_config))
            .with_content("Okunmamış e-postalarınız:")
            .with_icon(self.icon()))
    }
}

impl Section for NewsSection {
    fn section_type(&self) -> SectionType { SectionType::News }
    fn title(&self, _config: &DigestConfig) -> String { "Haberler".to_string() }
    fn icon(&self) -> &'static str { "📰" }
    
    fn build(&self, _config: &DigestConfig) -> DigestResult<DigestSection> {
        Ok(DigestSection::new(SectionType::News, &self.title(_config))
            .with_content("Güncel haber başlıkları:")
            .with_icon(self.icon()))
    }
}

impl Section for TasksSection {
    fn section_type(&self) -> SectionType { SectionType::Tasks }
    fn title(&self, _config: &DigestConfig) -> String { "Görevler".to_string() }
    fn icon(&self) -> &'static str { "✅" }
    
    fn build(&self, _config: &DigestConfig) -> DigestResult<DigestSection> {
        Ok(DigestSection::new(SectionType::Tasks, &self.title(_config))
            .with_content("Bugün için görevleriniz:")
            .with_icon(self.icon()))
    }
}

impl Section for GreetingSection {
    fn section_type(&self) -> SectionType { SectionType::Greeting }
    fn title(&self, _config: &DigestConfig) -> String { "".to_string() }
    fn icon(&self) -> &'static str { "👋" }
    
    fn build(&self, config: &DigestConfig) -> DigestResult<DigestSection> {
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

        Ok(DigestSection::new(SectionType::Greeting, "")
            .with_content(&content)
            .with_icon(self.icon()))
    }
}
