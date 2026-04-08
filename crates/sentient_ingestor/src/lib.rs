//! ═══════════════════════════════════════════════════════════════════════════════
//!  SENTIENT MASS INGESTOR - Ultimate Skill Assimilation Engine
//! ═══════════════════════════════════════════════════════════════════════════════
//! 
//!  Bu modül 5400+ OpenClaw skill'ini okur, parse eder ve Unified YAML formatına
//!  dönüştürerek SENTIENT'nın çelik çekirdeğine gömer.
//!
//!  Kategoriler:
//!  - Git & GitHub (167)
//!  - Coding Agents & IDEs (1184)
//!  - Browser & Automation (322)
//!  - Web & Frontend Dev (919)
//!  - DevOps & Cloud (393)
//!  - Image & Video Gen (170)
//!  - Search & Research (345)
//!  - CLI Utilities (180)
//!  - Productivity & Tasks (205)
//!  - Communication (146)
//!  - Marketing & Sales (102)
//!  - Health & Fitness (87)
//!  - Media & Streaming (85)
//!  - PDF & Documents (105)
//!  - Calendar & Scheduling (65)
//!  - Notes & PKM (69)
//!  - Security & Passwords (53)
//!  - Shopping & E-commerce (51)
//!  - Personal Development (50)
//!  - Speech & Transcription (45)
//!  - Apple Apps & Services (44)
//!  - Smart Home & IoT (41)
//!  - Gaming (35)
//!  - Data & Analytics (28)
//!  - iOS & macOS Dev (29)
//!  - Moltbook (29)
//!  - Self-Hosted & Automation (33)
//!  - Clawdbot Tools (37)
//!  - Transportation (110)
//!

pub mod error;
pub mod parser;
pub mod unified_yaml;
pub mod categories;
pub mod ingestor;
pub mod db;

pub use error::{IngestorError, IngestorResult};
pub use parser::{SkillParser, ParsedSkill};
pub use unified_yaml::{UnifiedSkill, SkillMetadata, SkillParameter, SkillExample};
pub use categories::{SkillCategory, categorize_skill};
pub use ingestor::{MassIngestor, IngestStats};
pub use db::{SkillDatabase, SkillRecord};

/// SENTIENT Skill versiyonu
pub const SKILL_SCHEMA_VERSION: &str = "1.0.0";

/// Maksimum paralel işlem sayısı
pub const MAX_PARALLEL_JOBS: usize = 32;

/// Skill hash'leme için prefix
pub const SKILL_HASH_PREFIX: &str = "sentient_skill_v1";

/// Desteklenen kategoriler
pub const CATEGORIES: &[&str] = &[
    "git-github",
    "coding-agents-ides",
    "browser-automation",
    "web-frontend-dev",
    "devops-cloud",
    "image-video-gen",
    "search-research",
    "cli-utilities",
    "productivity-tasks",
    "communication",
    "marketing-sales",
    "health-fitness",
    "media-streaming",
    "pdf-documents",
    "calendar-scheduling",
    "notes-pkm",
    "security-passwords",
    "shopping-ecommerce",
    "personal-development",
    "speech-transcription",
    "apple-apps-services",
    "smart-home-iot",
    "gaming",
    "data-analytics",
    "ios-macos-dev",
    "moltbook",
    "self-hosted-automation",
    "clawdbot-tools",
    "transportation",
];

/// Kategori renkleri (Dashboard için)
pub const CATEGORY_COLORS: &[(&str, &str)] = &[
    ("git-github", "#F05032"),       // Git turuncusu
    ("coding-agents-ides", "#007ACC"), // VS Code mavisi
    ("browser-automation", "#4285F4"), // Chrome mavisi
    ("web-frontend-dev", "#61DAFB"),   // React mavisi
    ("devops-cloud", "#FF6F00"),       // Kubernetes turuncusu
    ("image-video-gen", "#FF4081"),    // Pembe
    ("search-research", "#34A853"),    // Google yeşili
    ("cli-utilities", "#2E3440"),      // Nord koyu
    ("productivity-tasks", "#00C853"),  // Yeşil
    ("communication", "#7C4DFF"),       // Mor
    ("marketing-sales", "#FFD600"),     // Sarı
    ("health-fitness", "#00BFA5"),      // Teal
    ("media-streaming", "#E91E63"),     // Pembe
    ("pdf-documents", "#FF5722"),       // Turuncu
    ("calendar-scheduling", "#3F51B5"), // İndigo
    ("notes-pkm", "#9E9E9E"),           // Gri
    ("security-passwords", "#F44336"),  // Kırmızı
    ("shopping-ecommerce", "#4CAF50"),  // Yeşil
    ("personal-development", "#FF9800"),// Turuncu
    ("speech-transcription", "#673AB7"),// Derin mor
    ("apple-apps-services", "#A3AAAE"), // Apple gri
    ("smart-home-iot", "#00BCD4"),      // Cyan
    ("gaming", "#9C27B0"),              // Mor
    ("data-analytics", "#607D8B"),      // Blue grey
    ("ios-macos-dev", "#007AFF"),       // iOS mavisi
    ("moltbook", "#FF9800"),            // Turuncu
    ("self-hosted-automation", "#795548"),// Kahverengi
    ("clawdbot-tools", "#00E676"),      // Yeşil
    ("transportation", "#3D5AFE"),      // Mavi
];

/// SENTIENT Skill Prefix'i
pub fn sentient_skill_id(skill_name: &str, category: &str) -> String {
    format!("sentient_{}_{}", category.replace("-", "_"), 
            skill_name.to_lowercase()
                .replace(" ", "_")
                .replace("-", "_")
                .replace(".", "_")
                .replace("/", "_")
                .chars()
                .filter(|c| c.is_alphanumeric() || *c == '_')
                .take(48)
                .collect::<String>())
}

/// Skill hash hesapla (duplicate detection için)
pub fn skill_hash(name: &str, description: &str) -> String {
    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    hasher.update(SKILL_HASH_PREFIX.as_bytes());
    hasher.update(name.as_bytes());
    hasher.update(description.as_bytes());
    hex::encode(hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skill_id_generation() {
        let id = sentient_skill_id("GitHub PR Manager", "git-github");
        assert!(id.starts_with("sentient_git_github_"));
    }

    #[test]
    fn test_skill_hash_generation() {
        let hash1 = skill_hash("test", "description");
        let hash2 = skill_hash("test", "description");
        let hash3 = skill_hash("test", "different");
        
        assert_eq!(hash1, hash2);
        assert_ne!(hash1, hash3);
        assert_eq!(hash1.len(), 64); // SHA256 hex length
    }

    #[test]
    fn test_categories_count() {
        assert_eq!(CATEGORIES.len(), 29);
    }
}
