//! Voice Command Parser - Doğal dil → Aksiyon dönüşümü
//!
//! Desteklenen komutlar:
//! - "müzik aç" / "şarkı aç" → YouTube search
//! - "video aç" / "video izle" → YouTube search
//! - "ara" / "google'da ara" → Web search
//! - "dur" / "durdur" → Pause/Stop
//! - "kapat" → Close tab

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Parsed voice command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedCommand {
    /// Command intent
    pub intent: CommandIntent,
    /// Extracted entities (query, url, etc.)
    pub entities: HashMap<String, String>,
    /// Confidence (0-1)
    pub confidence: f32,
    /// Original text
    pub original: String,
}

/// Command intent type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommandIntent {
    /// Play music/song on YouTube
    PlayMusic,
    /// Play video on YouTube
    PlayVideo,
    /// Web search
    WebSearch,
    /// Pause/Stop playback
    Pause,
    /// Resume playback
    Resume,
    /// Close/Stop
    Close,
    /// Volume up
    VolumeUp,
    /// Volume down
    VolumeDown,
    /// Open website
    OpenWebsite,
    /// What time is it
    WhatTime,
    /// What's the weather
    Weather,
    /// Set reminder
    SetReminder,
    /// Smart home control (lights, climate, etc.)
    ControlHome,
    /// GitHub trending repos
    GitHubTrending,
    /// Open project and assign agents
    ProjectAssign,
    /// Unknown/unrecognized
    Unknown,
}

/// Voice Command Parser
pub struct CommandParser {
    /// Language for i18n
    language: String,
    /// Music keywords (Turkish + English)
    music_keywords: Vec<&'static str>,
    /// Video keywords
    video_keywords: Vec<&'static str>,
    /// Search keywords
    search_keywords: Vec<&'static str>,
    /// Pause keywords
    pause_keywords: Vec<&'static str>,
    /// Resume keywords
    resume_keywords: Vec<&'static str>,
    /// Close keywords
    close_keywords: Vec<&'static str>,
}

impl CommandParser {
    /// Create new parser
    pub fn new() -> Self {
        Self {
            language: "tr".to_string(),
            music_keywords: vec![
                // Turkish
                "müzik", "muzik", "şarkı", "sarki", "şarkıyı", "sarkiyi",
                "müziği", "muzigi", "melodi", "playlist", "listeyi",
                "rahatlatıcı", "rahatlatici", "sakin", "huzurlu",
                // English
                "music", "song", "play", "playlist", "relaxing", "calm",
            ],
            video_keywords: vec![
                // Turkish
                "video", "videoyu", "filmi", "film", "dizi", "klipleri",
                // English
                "video", "movie", "film", "watch",
            ],
            search_keywords: vec![
                // Turkish
                "ara", "arama", "google'da", "google da", "bul", "nedir",
                // English
                "search", "find", "look up", "google",
            ],
            pause_keywords: vec![
                // Turkish
                "dur", "durdur", "bekle", "pause",
                // English
                "pause", "stop", "wait",
            ],
            resume_keywords: vec![
                // Turkish
                "devam", "devam et", "başlat", "baslat", "oynat",
                // English
                "resume", "continue", "play",
            ],
            close_keywords: vec![
                // Turkish
                "kapat", "bitir", "çık", "cik",
                // English
                "close", "exit", "quit",
            ],
        }
    }

    /// Parse voice text to command
    pub fn parse(&self, text: &str) -> ParsedCommand {
        let text_lower = text.to_lowercase();
        let _words: Vec<&str> = text_lower.split_whitespace().collect();

        // === Priority 1: Smart home commands (must be before close/pause to avoid conflicts) ===
        // e.g. "salon ışığını kapat" should NOT match Close intent
        if self.matches_home_keywords(&text_lower) {
            return self.parse_home_command(text, &text_lower);
        }

        // === Priority 2: GitHub trending ===
        if text_lower.contains("github") && (text_lower.contains("trend") || text_lower.contains("trending") || text_lower.contains("popüler")) {
            return self.parse_github_trending(text, &text_lower);
        }

        // === Priority 3: Project assignment ===
        if self.matches_project_keywords(&text_lower) {
            return self.parse_project_command(text, &text_lower);
        }

        // === Priority 4: Media and general commands ===
        if self.matches_keywords(&text_lower, &self.music_keywords) {
            return self.parse_music_command(text, &text_lower);
        }

        if self.matches_keywords(&text_lower, &self.video_keywords) {
            return self.parse_video_command(text, &text_lower);
        }

        if self.matches_keywords(&text_lower, &self.search_keywords) {
            return self.parse_search_command(text, &text_lower);
        }

        if self.matches_keywords(&text_lower, &self.pause_keywords) {
            return ParsedCommand {
                intent: CommandIntent::Pause,
                entities: HashMap::new(),
                confidence: 0.95,
                original: text.to_string(),
            };
        }

        if self.matches_keywords(&text_lower, &self.resume_keywords) {
            return ParsedCommand {
                intent: CommandIntent::Resume,
                entities: HashMap::new(),
                confidence: 0.95,
                original: text.to_string(),
            };
        }

        if self.matches_keywords(&text_lower, &self.close_keywords) {
            return ParsedCommand {
                intent: CommandIntent::Close,
                entities: HashMap::new(),
                confidence: 0.95,
                original: text.to_string(),
            };
        }

        // Check for specific patterns
        if text_lower.contains("saat kaç") || text_lower.contains("what time") {
            return ParsedCommand {
                intent: CommandIntent::WhatTime,
                entities: HashMap::new(),
                confidence: 0.9,
                original: text.to_string(),
            };
        }

        if text_lower.contains("hava") || text_lower.contains("weather") {
            return ParsedCommand {
                intent: CommandIntent::Weather,
                entities: self.extract_location(&text_lower),
                confidence: 0.85,
                original: text.to_string(),
            };
        }

        // Unknown command
        ParsedCommand {
            intent: CommandIntent::Unknown,
            entities: HashMap::new(),
            confidence: 0.0,
            original: text.to_string(),
        }
    }

    /// Check if text matches any keyword
    fn matches_keywords(&self, text: &str, keywords: &[&str]) -> bool {
        keywords.iter().any(|k| text.contains(k))
    }

    /// Parse music command
    fn parse_music_command(&self, original: &str, text_lower: &str) -> ParsedCommand {
        let mut entities = HashMap::new();
        entities.insert("platform".to_string(), "youtube".to_string());

        // Extract search query
        let query = self.extract_music_query(text_lower);
        entities.insert("query".to_string(), query);

        ParsedCommand {
            intent: CommandIntent::PlayMusic,
            entities,
            confidence: 0.9,
            original: original.to_string(),
        }
    }

    /// Parse video command
    fn parse_video_command(&self, original: &str, text_lower: &str) -> ParsedCommand {
        let mut entities = HashMap::new();
        entities.insert("platform".to_string(), "youtube".to_string());

        let query = self.extract_video_query(text_lower);
        entities.insert("query".to_string(), query);

        ParsedCommand {
            intent: CommandIntent::PlayVideo,
            entities,
            confidence: 0.9,
            original: original.to_string(),
        }
    }

    /// Parse search command
    fn parse_search_command(&self, original: &str, text_lower: &str) -> ParsedCommand {
        let mut entities = HashMap::new();
        entities.insert("platform".to_string(), "google".to_string());

        let query = self.extract_search_query(text_lower);
        entities.insert("query".to_string(), query);

        ParsedCommand {
            intent: CommandIntent::WebSearch,
            entities,
            confidence: 0.85,
            original: original.to_string(),
        }
    }

    /// Extract music search query
    fn extract_music_query(&self, text: &str) -> String {
        // Remove command words, keep the query
        let stop_words = [
            "müzik", "muzik", "şarkı", "sarki", "şarkıyı", "sarkiyi",
            "müziği", "muzigi", "aç", "ac", "play", "music", "song",
            "bana", "bi", "bir", "lütfen", "lutfen",
        ];

        let words: Vec<&str> = text.split_whitespace().collect();
        let query_words: Vec<&str> = words
            .into_iter()
            .filter(|w| !stop_words.contains(w) && w.len() > 1)
            .collect();

        if query_words.is_empty() {
            "rahatlatıcı müzik".to_string()
        } else {
            query_words.join(" ")
        }
    }

    /// Extract video search query
    fn extract_video_query(&self, text: &str) -> String {
        let stop_words = [
            "video", "videoyu", "filmi", "film", "dizi", "aç", "ac",
            "izle", "watch", "play",
        ];

        let words: Vec<&str> = text.split_whitespace().collect();
        let query_words: Vec<&str> = words
            .into_iter()
            .filter(|w| !stop_words.contains(w) && w.len() > 1)
            .collect();

        if query_words.is_empty() {
            "video".to_string()
        } else {
            query_words.join(" ")
        }
    }

    /// Extract search query
    fn extract_search_query(&self, text: &str) -> String {
        let stop_words = ["ara", "arama", "google'da", "google", "da", "de", "search", "find"];

        let words: Vec<&str> = text.split_whitespace().collect();
        let query_words: Vec<&str> = words
            .into_iter()
            .filter(|w| !stop_words.contains(w) && w.len() > 1)
            .collect();

        query_words.join(" ")
    }

    /// Check for smart home keywords
    fn matches_home_keywords(&self, text: &str) -> bool {
        // Exclude media commands first
        if text.contains("müzik") || text.contains("muzik") || text.contains("video") ||
            text.contains("şarkı") || text.contains("sarki") || text.contains("ara") ||
            text.contains("music") || text.contains("song") || text.contains("film") {
            return false;
        }

        // Device-specific keywords (strong indicators of smart home)
        let device_keywords = [
            "ışık", "lamba", "klima", "perde", "kilit", "kapı",
            "garaj", "akıllı ev", "smart home", "home assistant",
        ];

        // Direct device keyword match
        if device_keywords.iter().any(|k| text.contains(k)) {
            return true;
        }

        // Room + action pattern
        let rooms = ["salon", "yatak", "mutfak", "banyo", "ofis", "koridor"];
        let home_actions = ["söndür", "yak", "parla", "kıs", "ısıt", "soğut"];

        let has_room = rooms.iter().any(|r| text.contains(r));
        let has_home_action = home_actions.iter().any(|a| text.contains(a));

        if has_room && (has_home_action || text.contains("kapat") || text.contains("aç")) {
            return true;
        }

        // Scene keywords
        let scene_keywords = ["film modu", "uyku modu", "gece modu", "parti modu", "sabah modu"];
        if scene_keywords.iter().any(|k| text.contains(k)) {
            return true;
        }

        false
    }

    /// Check for project assignment keywords
    fn matches_project_keywords(&self, text: &str) -> bool {
        text.contains("proje") && (
            text.contains("aç") || text.contains("başlat") ||
            text.contains("ajan") || text.contains("yetkilendir") ||
            text.contains("assign") || text.contains("project")
        ) && !text.contains("müzik") && !text.contains("video")
    }

    /// Parse GitHub trending command
    fn parse_github_trending(&self, original: &str, text_lower: &str) -> ParsedCommand {
        let mut entities = HashMap::new();
        entities.insert("action".to_string(), "trending".to_string());
        // Extract language/category filter
        if text_lower.contains("rust") {
            entities.insert("language".to_string(), "rust".to_string());
        } else if text_lower.contains("python") {
            entities.insert("language".to_string(), "python".to_string());
        } else if text_lower.contains("javascript") || text_lower.contains("js") {
            entities.insert("language".to_string(), "javascript".to_string());
        }
        // Extract time range
        if text_lower.contains("günlük") || text_lower.contains("daily") || text_lower.contains("bugün") {
            entities.insert("since".to_string(), "daily".to_string());
        } else if text_lower.contains("haftalık") || text_lower.contains("weekly") || text_lower.contains("bu hafta") {
            entities.insert("since".to_string(), "weekly".to_string());
        } else if text_lower.contains("aylık") || text_lower.contains("monthly") || text_lower.contains("bu ay") {
            entities.insert("since".to_string(), "monthly".to_string());
        }
        ParsedCommand {
            intent: CommandIntent::GitHubTrending,
            entities,
            confidence: 0.9,
            original: original.to_string(),
        }
    }

    /// Parse home control command
    fn parse_home_command(&self, original: &str, text_lower: &str) -> ParsedCommand {
        let mut entities = HashMap::new();

        // Determine action
        if text_lower.contains(" aç") || text_lower.contains("yan") || text_lower.contains("on") {
            entities.insert("action".to_string(), "turn_on".to_string());
        } else if text_lower.contains("kapat") || text_lower.contains("söndür") || text_lower.contains("off") {
            entities.insert("action".to_string(), "turn_off".to_string());
        } else if text_lower.contains("kıs") || text_lower.contains("dim") {
            entities.insert("action".to_string(), "dim".to_string());
        } else if text_lower.contains("parla") || text_lower.contains("bright") {
            entities.insert("action".to_string(), "brighten".to_string());
        } else {
            entities.insert("action".to_string(), "toggle".to_string());
        }

        // Determine device/room
        let rooms = [("salon", "living_room"), ("yatak", "bedroom"), ("mutfak", "kitchen"), ("banyo", "bathroom"), ("ofis", "office")];
        for (tr, en) in rooms {
            if text_lower.contains(tr) {
                entities.insert("room".to_string(), en.to_string());
                break;
            }
        }

        let devices = [("ışık", "light"), ("lamba", "light"), ("klima", "climate"), ("perde", "cover"), ("kilit", "lock")];
        for (tr, en) in devices {
            if text_lower.contains(tr) {
                entities.insert("device_type".to_string(), en.to_string());
                break;
            }
        }

        // Extract numeric value (brightness, temperature)
        for word in text_lower.split_whitespace() {
            if let Ok(n) = word.parse::<i32>() {
                entities.insert("value".to_string(), n.to_string());
                break;
            }
        }

        // Scene detection
        let scenes = [("film", "movie"), ("uyku", "good_night"), ("sabah", "good_morning"), ("parti", "party")];
        for (keyword, scene) in scenes {
            if text_lower.contains(keyword) {
                entities.insert("scene".to_string(), scene.to_string());
                break;
            }
        }

        ParsedCommand {
            intent: CommandIntent::ControlHome,
            entities,
            confidence: 0.85,
            original: original.to_string(),
        }
    }

    /// Parse project assignment command
    fn parse_project_command(&self, original: &str, text_lower: &str) -> ParsedCommand {
        let mut entities = HashMap::new();

        // Extract project name
        let stop_words = ["proje", "projesini", "projesi", "aç", "başlat", "ajanları", "ajan", "yetkilendir", "ata", "project", "open", "assign", "agents"];
        let query_words: Vec<&str> = text_lower.split_whitespace()
            .filter(|w| !stop_words.contains(w) && w.len() > 1)
            .collect();

        if !query_words.is_empty() {
            entities.insert("project".to_string(), query_words.join(" "));
        }

        // Determine agent type
        if text_lower.contains("araştır") || text_lower.contains("research") {
            entities.insert("agent_type".to_string(), "researcher".to_string());
        } else if text_lower.contains("kod") || text_lower.contains("code") || text_lower.contains("yaz") {
            entities.insert("agent_type".to_string(), "coder".to_string());
        } else if text_lower.contains("test") {
            entities.insert("agent_type".to_string(), "tester".to_string());
        } else if text_lower.contains("tasarla") || text_lower.contains("design") {
            entities.insert("agent_type".to_string(), "designer".to_string());
        }

        // Determine framework
        if text_lower.contains("crew") || text_lower.contains("crewai") {
            entities.insert("framework".to_string(), "crewai".to_string());
        } else if text_lower.contains("swarm") {
            entities.insert("framework".to_string(), "swarm".to_string());
        } else if text_lower.contains("autogen") {
            entities.insert("framework".to_string(), "autogen".to_string());
        }

        ParsedCommand {
            intent: CommandIntent::ProjectAssign,
            entities,
            confidence: 0.8,
            original: original.to_string(),
        }
    }

    /// Extract location for weather
    fn extract_location(&self, text: &str) -> HashMap<String, String> {
        let mut entities = HashMap::new();

        // Look for "in [city]" or "[şehir]'da"
        let words: Vec<&str> = text.split_whitespace().collect();
        for (i, word) in words.iter().enumerate() {
            if *word == "in" && i + 1 < words.len() {
                entities.insert("location".to_string(), words[i + 1].to_string());
                break;
            }
        }

        if entities.is_empty() {
            entities.insert("location".to_string(), "İstanbul".to_string());
        }

        entities
    }
}

impl Default for CommandParser {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_music_command() {
        let parser = CommandParser::new();

        let cmd = parser.parse("bana rahatlatıcı müzik aç");
        assert_eq!(cmd.intent, CommandIntent::PlayMusic);
        assert_eq!(cmd.entities.get("platform").expect("operation failed"), "youtube");
        assert!(cmd.confidence > 0.8);
    }

    #[test]
    fn test_parse_song_command() {
        let parser = CommandParser::new();

        let cmd = parser.parse("sezen aksu gidiyorum şarkısını aç");
        assert_eq!(cmd.intent, CommandIntent::PlayMusic);
        assert!(cmd.entities.get("query").expect("operation failed").contains("sezen"));
    }

    #[test]
    fn test_parse_pause() {
        let parser = CommandParser::new();

        let cmd = parser.parse("durdur");
        assert_eq!(cmd.intent, CommandIntent::Pause);
    }

    #[test]
    fn test_parse_video() {
        let parser = CommandParser::new();

        let cmd = parser.parse("yapay zeka hakkında video aç");
        assert_eq!(cmd.intent, CommandIntent::PlayVideo);
    }

    #[test]
    fn test_parse_search() {
        let parser = CommandParser::new();

        let cmd = parser.parse("google'da rust programlama ara");
        assert_eq!(cmd.intent, CommandIntent::WebSearch);
    }

    #[test]
    fn test_parse_english() {
        let parser = CommandParser::new();

        let cmd = parser.parse("play relaxing music");
        assert_eq!(cmd.intent, CommandIntent::PlayMusic);
    }

    #[test]
    fn test_parse_unknown() {
        let parser = CommandParser::new();

        let cmd = parser.parse("bu anlamsız bir cümle xyz");
        assert_eq!(cmd.intent, CommandIntent::Unknown);
    }
}
