//! Digest templates - language-specific templates

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{DigestConfig, SectionType, TimeOfDay};

/// Template for digest generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DigestTemplate {
    /// Template name
    pub name: String,
    /// Language code
    pub language: String,
    /// Section order
    pub section_order: Vec<SectionType>,
    /// Greeting templates
    pub greetings: HashMap<String, String>,
    /// Section titles
    pub section_titles: HashMap<SectionType, String>,
    /// Section icons
    pub section_icons: HashMap<SectionType, String>,
    /// Footer text
    pub footer: String,
}

impl DigestTemplate {
    /// Turkish template
    pub fn turkish() -> Self {
        let mut greetings = HashMap::new();
        greetings.insert("morning".to_string(), "Günaydın".to_string());
        greetings.insert("afternoon".to_string(), "Tünaydın".to_string());
        greetings.insert("evening".to_string(), "İyi akşamlar".to_string());

        let mut section_titles = HashMap::new();
        section_titles.insert(SectionType::Weather, "Hava Durumu".to_string());
        section_titles.insert(SectionType::Calendar, "Bugünkü Etkinlikler".to_string());
        section_titles.insert(SectionType::Email, "E-postalar".to_string());
        section_titles.insert(SectionType::News, "Haberler".to_string());
        section_titles.insert(SectionType::Tasks, "Görevler".to_string());
        section_titles.insert(SectionType::Reminders, "Hatırlatmalar".to_string());

        let mut section_icons = HashMap::new();
        section_icons.insert(SectionType::Weather, "🌤️".to_string());
        section_icons.insert(SectionType::Calendar, "📅".to_string());
        section_icons.insert(SectionType::Email, "📧".to_string());
        section_icons.insert(SectionType::News, "📰".to_string());
        section_icons.insert(SectionType::Tasks, "✅".to_string());
        section_icons.insert(SectionType::Reminders, "🔔".to_string());

        Self {
            name: "tr-default".to_string(),
            language: "tr".to_string(),
            section_order: vec![
                SectionType::Greeting,
                SectionType::Weather,
                SectionType::Calendar,
                SectionType::Email,
                SectionType::News,
                SectionType::Tasks,
            ],
            greetings,
            section_titles,
            section_icons,
            footer: "{} tarafından hazırlandı".to_string(),
        }
    }

    /// English template
    pub fn english() -> Self {
        let mut greetings = HashMap::new();
        greetings.insert("morning".to_string(), "Good morning".to_string());
        greetings.insert("afternoon".to_string(), "Good afternoon".to_string());
        greetings.insert("evening".to_string(), "Good evening".to_string());

        let mut section_titles = HashMap::new();
        section_titles.insert(SectionType::Weather, "Weather".to_string());
        section_titles.insert(SectionType::Calendar, "Today's Events".to_string());
        section_titles.insert(SectionType::Email, "Emails".to_string());
        section_titles.insert(SectionType::News, "News".to_string());
        section_titles.insert(SectionType::Tasks, "Tasks".to_string());
        section_titles.insert(SectionType::Reminders, "Reminders".to_string());

        let mut section_icons = HashMap::new();
        section_icons.insert(SectionType::Weather, "🌤️".to_string());
        section_icons.insert(SectionType::Calendar, "📅".to_string());
        section_icons.insert(SectionType::Email, "📧".to_string());
        section_icons.insert(SectionType::News, "📰".to_string());
        section_icons.insert(SectionType::Tasks, "✅".to_string());
        section_icons.insert(SectionType::Reminders, "🔔".to_string());

        Self {
            name: "en-default".to_string(),
            language: "en".to_string(),
            section_order: vec![
                SectionType::Greeting,
                SectionType::Weather,
                SectionType::Calendar,
                SectionType::Email,
                SectionType::News,
                SectionType::Tasks,
            ],
            greetings,
            section_titles,
            section_icons,
            footer: "Prepared by {}".to_string(),
        }
    }

    /// German template
    pub fn german() -> Self {
        let mut greetings = HashMap::new();
        greetings.insert("morning".to_string(), "Guten Morgen".to_string());
        greetings.insert("afternoon".to_string(), "Guten Tag".to_string());
        greetings.insert("evening".to_string(), "Guten Abend".to_string());

        let mut section_titles = HashMap::new();
        section_titles.insert(SectionType::Weather, "Wetter".to_string());
        section_titles.insert(SectionType::Calendar, "Heutige Termine".to_string());
        section_titles.insert(SectionType::Email, "E-Mails".to_string());
        section_titles.insert(SectionType::News, "Nachrichten".to_string());
        section_titles.insert(SectionType::Tasks, "Aufgaben".to_string());

        let mut section_icons = HashMap::new();
        section_icons.insert(SectionType::Weather, "🌤️".to_string());
        section_icons.insert(SectionType::Calendar, "📅".to_string());
        section_icons.insert(SectionType::Email, "📧".to_string());
        section_icons.insert(SectionType::News, "📰".to_string());
        section_icons.insert(SectionType::Tasks, "✅".to_string());

        Self {
            name: "de-default".to_string(),
            language: "de".to_string(),
            section_order: vec![
                SectionType::Greeting,
                SectionType::Weather,
                SectionType::Calendar,
                SectionType::Email,
                SectionType::News,
                SectionType::Tasks,
            ],
            greetings,
            section_titles,
            section_icons,
            footer: "Erstellt von {}".to_string(),
        }
    }

    /// Get greeting for time of day
    pub fn get_greeting(&self, time_of_day: &TimeOfDay) -> &str {
        let key = match time_of_day {
            TimeOfDay::Morning => "morning",
            TimeOfDay::Afternoon => "afternoon",
            TimeOfDay::Evening => "evening",
        };
        self.greetings.get(key).map(|s| s.as_str()).unwrap_or("Hello")
    }

    /// Get section title
    pub fn get_section_title(&self, section_type: &SectionType) -> &str {
        self.section_titles.get(section_type).map(|s| s.as_str()).unwrap_or("")
    }

    /// Get section icon
    pub fn get_section_icon(&self, section_type: &SectionType) -> &str {
        self.section_icons.get(section_type).map(|s| s.as_str()).unwrap_or("📌")
    }
}

/// Template registry
pub struct TemplateRegistry {
    templates: HashMap<String, DigestTemplate>,
}

impl TemplateRegistry {
    pub fn new() -> Self {
        let mut templates = HashMap::new();
        
        // Add built-in templates
        let tr = DigestTemplate::turkish();
        templates.insert(tr.language.clone(), tr);
        
        let en = DigestTemplate::english();
        templates.insert(en.language.clone(), en);
        
        let de = DigestTemplate::german();
        templates.insert(de.language.clone(), de);

        Self { templates }
    }

    pub fn get(&self, language: &str) -> Option<&DigestTemplate> {
        self.templates.get(language)
    }

    pub fn register(&mut self, template: DigestTemplate) {
        self.templates.insert(template.language.clone(), template);
    }

    pub fn list(&self) -> Vec<&str> {
        self.templates.keys().map(|s| s.as_str()).collect()
    }
}

impl Default for TemplateRegistry {
    fn default() -> Self {
        Self::new()
    }
}
