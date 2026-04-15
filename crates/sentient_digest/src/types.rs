//! Core types for digest system

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ═══════════════════════════════════════════════════════════════════════════════
// DIGEST - Main output
// ═══════════════════════════════════════════════════════════════════════════════

/// A complete digest/briefing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Digest {
    pub id: String,
    pub title: String,
    pub language: String,
    pub created_at: DateTime<Utc>,
    pub sections: Vec<DigestSection>,
    pub full_text: String,
    pub html_content: Option<String>,
    pub audio_url: Option<String>,
    pub metadata: DigestMetadata,
}

impl Digest {
    pub fn new(title: &str, language: &str) -> Self {
        Self {
            id: generate_id(),
            title: title.to_string(),
            language: language.to_string(),
            created_at: Utc::now(),
            sections: Vec::new(),
            full_text: String::new(),
            html_content: None,
            audio_url: None,
            metadata: DigestMetadata::default(),
        }
    }

    pub fn with_section(mut self, section: DigestSection) -> Self {
        self.sections.push(section);
        self
    }

    pub fn build_full_text(&mut self) {
        self.full_text = self.sections
            .iter()
            .map(|s| format!("{}\n{}", s.title, s.content))
            .collect::<Vec<_>>()
            .join("\n\n");
    }
}

/// A section within a digest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DigestSection {
    pub section_type: SectionType,
    pub title: String,
    pub content: String,
    pub items: Vec<DigestItem>,
    pub priority: u8,
    pub icon: Option<String>,
}

impl DigestSection {
    pub fn new(section_type: SectionType, title: &str) -> Self {
        Self {
            section_type,
            title: title.to_string(),
            content: String::new(),
            items: Vec::new(),
            priority: 5,
            icon: None,
        }
    }

    pub fn with_content(mut self, content: &str) -> Self {
        self.content = content.to_string();
        self
    }

    pub fn with_item(mut self, item: DigestItem) -> Self {
        self.items.push(item);
        self
    }

    pub fn with_icon(mut self, icon: &str) -> Self {
        self.icon = Some(icon.to_string());
        self
    }
}

/// Section types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SectionType {
    Greeting,
    Weather,
    Calendar,
    Email,
    News,
    Tasks,
    Reminders,
    Health,
    Finance,
    Custom(String),
}

/// An item within a section
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DigestItem {
    pub title: String,
    pub content: String,
    pub source: Option<String>,
    pub url: Option<String>,
    pub timestamp: Option<DateTime<Utc>>,
    pub is_important: bool,
    pub metadata: HashMap<String, String>,
}

impl DigestItem {
    pub fn new(title: &str, content: &str) -> Self {
        Self {
            title: title.to_string(),
            content: content.to_string(),
            source: None,
            url: None,
            timestamp: None,
            is_important: false,
            metadata: HashMap::new(),
        }
    }

    pub fn with_source(mut self, source: &str) -> Self {
        self.source = Some(source.to_string());
        self
    }

    pub fn important(mut self) -> Self {
        self.is_important = true;
        self
    }
}

/// Digest metadata
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DigestMetadata {
    pub assistant_name: String,
    pub user_name: Option<String>,
    pub location: Option<String>,
    pub timezone: String,
    pub total_items: usize,
    pub generation_time_ms: u64,
    pub sources_used: Vec<String>,
}

/// Time of day
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum TimeOfDay {
    #[default]
    Morning,
    Afternoon,
    Evening,
}

/// Digest configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DigestConfig {
    pub language: String,
    pub time_of_day: TimeOfDay,
    pub include_sections: Vec<SectionType>,
    pub exclude_sections: Vec<SectionType>,
    pub max_items_per_section: usize,
    pub assistant_name: String,
    pub user_name: Option<String>,
    pub location: Option<String>,
    pub timezone: String,
}

impl Default for DigestConfig {
    fn default() -> Self {
        Self {
            language: "tr".to_string(),
            time_of_day: TimeOfDay::Morning,
            include_sections: vec![
                SectionType::Greeting,
                SectionType::Weather,
                SectionType::Calendar,
                SectionType::Email,
                SectionType::News,
            ],
            exclude_sections: Vec::new(),
            max_items_per_section: 5,
            assistant_name: "SENTIENT".to_string(),
            user_name: None,
            location: None,
            timezone: "Europe/Istanbul".to_string(),
        }
    }
}

fn generate_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0);
    format!("digest-{}", ts)
}
