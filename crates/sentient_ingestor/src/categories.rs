//! ═══════════════════════════════════════════════════════════════════════════════
//!  SKILL CATEGORIES - Kategori Yönetimi
//! ═══════════════════════════════════════════════════════════════════════════════

use std::fmt;

/// Skill Kategorileri
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SkillCategory {
    GitGithub,
    CodingAgentsIdes,
    BrowserAutomation,
    WebFrontendDev,
    DevopsCloud,
    ImageVideoGen,
    SearchResearch,
    CliUtilities,
    ProductivityTasks,
    Communication,
    MarketingSales,
    HealthFitness,
    MediaStreaming,
    PdfDocuments,
    CalendarScheduling,
    NotesPkm,
    SecurityPasswords,
    ShoppingEcommerce,
    PersonalDevelopment,
    SpeechTranscription,
    AppleAppsServices,
    SmartHomeIoT,
    Gaming,
    DataAnalytics,
    IosMacosDev,
    Moltbook,
    SelfHostedAutomation,
    ClawdbotTools,
    Transportation,
    Unknown,
}

impl SkillCategory {
    /// Dosya adından kategori belirle
    pub fn from_filename(filename: &str) -> Self {
        let filename = filename.to_lowercase();
        
        if filename.contains("git") && filename.contains("github") {
            Self::GitGithub
        } else if filename.contains("coding") && filename.contains("agent") {
            Self::CodingAgentsIdes
        } else if filename.contains("browser") || filename.contains("automation") {
            Self::BrowserAutomation
        } else if filename.contains("web") && filename.contains("frontend") {
            Self::WebFrontendDev
        } else if filename.contains("devops") || filename.contains("cloud") {
            Self::DevopsCloud
        } else if filename.contains("image") || filename.contains("video") {
            Self::ImageVideoGen
        } else if filename.contains("search") || filename.contains("research") {
            Self::SearchResearch
        } else if filename.contains("cli") {
            Self::CliUtilities
        } else if filename.contains("productivity") {
            Self::ProductivityTasks
        } else if filename.contains("communication") {
            Self::Communication
        } else if filename.contains("marketing") || filename.contains("sales") {
            Self::MarketingSales
        } else if filename.contains("health") || filename.contains("fitness") {
            Self::HealthFitness
        } else if filename.contains("media") || filename.contains("streaming") {
            Self::MediaStreaming
        } else if filename.contains("pdf") || filename.contains("document") {
            Self::PdfDocuments
        } else if filename.contains("calendar") {
            Self::CalendarScheduling
        } else if filename.contains("notes") || filename.contains("pkm") {
            Self::NotesPkm
        } else if filename.contains("security") || filename.contains("password") {
            Self::SecurityPasswords
        } else if filename.contains("shopping") || filename.contains("ecommerce") {
            Self::ShoppingEcommerce
        } else if filename.contains("personal") && filename.contains("development") {
            Self::PersonalDevelopment
        } else if filename.contains("speech") || filename.contains("transcription") {
            Self::SpeechTranscription
        } else if filename.contains("apple") {
            Self::AppleAppsServices
        } else if filename.contains("smart") && filename.contains("home") {
            Self::SmartHomeIoT
        } else if filename.contains("gaming") {
            Self::Gaming
        } else if filename.contains("data") && filename.contains("analytics") {
            Self::DataAnalytics
        } else if filename.contains("ios") || filename.contains("macos") {
            Self::IosMacosDev
        } else if filename.contains("moltbook") {
            Self::Moltbook
        } else if filename.contains("self") && filename.contains("hosted") {
            Self::SelfHostedAutomation
        } else if filename.contains("clawdbot") {
            Self::ClawdbotTools
        } else if filename.contains("transportation") {
            Self::Transportation
        } else {
            Self::Unknown
        }
    }
    
    /// Açıklama metninden kategori tahmin et
    pub fn from_description(description: &str) -> Self {
        let desc = description.to_lowercase();
        
        // Git/GitHub
        if desc.contains("github") || desc.contains("git") || desc.contains("repository") 
            || desc.contains("pull request") || desc.contains("commit") {
            return Self::GitGithub;
        }
        
        // Browser/Automation
        if desc.contains("browser") || desc.contains("selenium") || desc.contains("puppeteer")
            || desc.contains("playwright") || desc.contains("headless") {
            return Self::BrowserAutomation;
        }
        
        // Web/Frontend
        if desc.contains("react") || desc.contains("vue") || desc.contains("angular")
            || desc.contains("frontend") || desc.contains("css") || desc.contains("html") {
            return Self::WebFrontendDev;
        }
        
        // DevOps/Cloud
        if desc.contains("docker") || desc.contains("kubernetes") || desc.contains("aws")
            || desc.contains("deploy") || desc.contains("ci/cd") || desc.contains("pipeline") {
            return Self::DevopsCloud;
        }
        
        // Search/Research
        if desc.contains("search") || desc.contains("research") || desc.contains("find")
            || desc.contains("query") || desc.contains("arxiv") {
            return Self::SearchResearch;
        }
        
        // CLI
        if desc.contains("cli") || desc.contains("terminal") || desc.contains("command line") {
            return Self::CliUtilities;
        }
        
        // Productivity
        if desc.contains("task") || desc.contains("todo") || desc.contains("productivity")
            || desc.contains("manage") || desc.contains("organize") {
            return Self::ProductivityTasks;
        }
        
        // Communication
        if desc.contains("slack") || desc.contains("discord") || desc.contains("email")
            || desc.contains("chat") || desc.contains("message") || desc.contains("telegram") {
            return Self::Communication;
        }
        
        // Marketing/Sales
        if desc.contains("marketing") || desc.contains("sales") || desc.contains("lead")
            || desc.contains("campaign") || desc.contains("analytics") {
            return Self::MarketingSales;
        }
        
        // Health/Fitness
        if desc.contains("health") || desc.contains("fitness") || desc.contains("workout")
            || desc.contains("exercise") || desc.contains("medical") {
            return Self::HealthFitness;
        }
        
        // Security
        if desc.contains("security") || desc.contains("password") || desc.contains("auth")
            || desc.contains("credential") || desc.contains("encrypt") {
            return Self::SecurityPasswords;
        }
        
        // Media/Streaming
        if desc.contains("video") || desc.contains("audio") || desc.contains("stream")
            || desc.contains("media") || desc.contains("youtube") || desc.contains("spotify") {
            return Self::MediaStreaming;
        }
        
        // Image/Video Generation
        if desc.contains("generate") && (desc.contains("image") || desc.contains("video"))
            || desc.contains("dalle") || desc.contains("midjourney") || desc.contains("stable diffusion") {
            return Self::ImageVideoGen;
        }
        
        Self::Unknown
    }
    
    /// Kategori rengi
    pub fn color(&self) -> &'static str {
        crate::CATEGORY_COLORS
            .iter()
            .find(|(cat, _)| *cat == self.to_string())
            .map(|(_, color)| *color)
            .unwrap_or("#808080")
    }
    
    /// Kategori ikonu (emoji)
    pub fn icon(&self) -> &'static str {
        match self {
            Self::GitGithub => "🐙",
            Self::CodingAgentsIdes => "💻",
            Self::BrowserAutomation => "🌐",
            Self::WebFrontendDev => "🎨",
            Self::DevopsCloud => "☁️",
            Self::ImageVideoGen => "🎬",
            Self::SearchResearch => "🔍",
            Self::CliUtilities => "⌨️",
            Self::ProductivityTasks => "✅",
            Self::Communication => "💬",
            Self::MarketingSales => "📈",
            Self::HealthFitness => "💪",
            Self::MediaStreaming => "🎵",
            Self::PdfDocuments => "📄",
            Self::CalendarScheduling => "📅",
            Self::NotesPkm => "📝",
            Self::SecurityPasswords => "🔒",
            Self::ShoppingEcommerce => "🛒",
            Self::PersonalDevelopment => "📚",
            Self::SpeechTranscription => "🎤",
            Self::AppleAppsServices => "",
            Self::SmartHomeIoT => "🏠",
            Self::Gaming => "🎮",
            Self::DataAnalytics => "📊",
            Self::IosMacosDev => "",
            Self::Moltbook => "📖",
            Self::SelfHostedAutomation => "🔧",
            Self::ClawdbotTools => "🤖",
            Self::Transportation => "🚗",
            Self::Unknown => "❓",
        }
    }
}

impl fmt::Display for SkillCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::GitGithub => write!(f, "git-github"),
            Self::CodingAgentsIdes => write!(f, "coding-agents-ides"),
            Self::BrowserAutomation => write!(f, "browser-automation"),
            Self::WebFrontendDev => write!(f, "web-frontend-dev"),
            Self::DevopsCloud => write!(f, "devops-cloud"),
            Self::ImageVideoGen => write!(f, "image-video-gen"),
            Self::SearchResearch => write!(f, "search-research"),
            Self::CliUtilities => write!(f, "cli-utilities"),
            Self::ProductivityTasks => write!(f, "productivity-tasks"),
            Self::Communication => write!(f, "communication"),
            Self::MarketingSales => write!(f, "marketing-sales"),
            Self::HealthFitness => write!(f, "health-fitness"),
            Self::MediaStreaming => write!(f, "media-streaming"),
            Self::PdfDocuments => write!(f, "pdf-documents"),
            Self::CalendarScheduling => write!(f, "calendar-scheduling"),
            Self::NotesPkm => write!(f, "notes-pkm"),
            Self::SecurityPasswords => write!(f, "security-passwords"),
            Self::ShoppingEcommerce => write!(f, "shopping-ecommerce"),
            Self::PersonalDevelopment => write!(f, "personal-development"),
            Self::SpeechTranscription => write!(f, "speech-transcription"),
            Self::AppleAppsServices => write!(f, "apple-apps-services"),
            Self::SmartHomeIoT => write!(f, "smart-home-iot"),
            Self::Gaming => write!(f, "gaming"),
            Self::DataAnalytics => write!(f, "data-analytics"),
            Self::IosMacosDev => write!(f, "ios-macos-dev"),
            Self::Moltbook => write!(f, "moltbook"),
            Self::SelfHostedAutomation => write!(f, "self-hosted-automation"),
            Self::ClawdbotTools => write!(f, "clawdbot-tools"),
            Self::Transportation => write!(f, "transportation"),
            Self::Unknown => write!(f, "unknown"),
        }
    }
}

/// Skill'i kategorize et
pub fn categorize_skill(name: &str, description: &str) -> SkillCategory {
    // Önce isimden dene
    let name_category = SkillCategory::from_description(name);
    if name_category != SkillCategory::Unknown {
        return name_category;
    }
    
    // Sonra açıklamadan
    SkillCategory::from_description(description)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_from_filename() {
        assert_eq!(
            SkillCategory::from_filename("git-and-github.md"),
            SkillCategory::GitGithub
        );
        assert_eq!(
            SkillCategory::from_filename("browser-and-automation.md"),
            SkillCategory::BrowserAutomation
        );
    }
    
    #[test]
    fn test_from_description() {
        assert_eq!(
            SkillCategory::from_description("Manage GitHub pull requests"),
            SkillCategory::GitGithub
        );
        assert_eq!(
            SkillCategory::from_description("Browser automation tool"),
            SkillCategory::BrowserAutomation
        );
    }
    
    #[test]
    fn test_display() {
        assert_eq!(format!("{}", SkillCategory::GitGithub), "git-github");
        assert_eq!(format!("{}", SkillCategory::Unknown), "unknown");
    }
}
