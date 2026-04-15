//! ─── Voice Command Parser ───

use serde::{Deserialize, Serialize};

/// Parsed voice command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedCommand {
    pub intent: Intent,
    pub entity: Option<String>,
    pub value: Option<String>,
    pub confidence: f64,
}

/// Command intent
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Intent {
    TurnOn,
    TurnOff,
    Toggle,
    SetBrightness,
    SetColor,
    SetTemperature,
    Lock,
    Unlock,
    Open,
    Close,
    Play,
    Pause,
    SetVolume,
    ActivateScene,
    StatusCheck,
    Unknown,
}

/// Voice command parser
pub struct VoiceCommandParser {
    patterns: Vec<CommandPattern>,
}

struct CommandPattern {
    keywords: Vec<&'static str>,
    intent: Intent,
    extract_value: bool,
}

impl VoiceCommandParser {
    pub fn new() -> Self {
        Self {
            patterns: default_patterns(),
        }
    }
    
    /// Parse voice command text
    pub fn parse(&self, text: &str) -> ParsedCommand {
        let text_lower = text.to_lowercase();
        
        // Try each pattern
        for pattern in &self.patterns {
            if pattern.keywords.iter().any(|k| text_lower.contains(k)) {
                let entity = self.extract_entity(&text_lower);
                let value = if pattern.extract_value {
                    self.extract_value(&text_lower)
                } else {
                    None
                };
                
                return ParsedCommand {
                    intent: pattern.intent,
                    entity,
                    value,
                    confidence: 0.85,
                };
            }
        }
        
        // Check for scene activation
        if text_lower.contains("mod") || text_lower.contains("mode") || text_lower.contains("sahne") {
            return ParsedCommand {
                intent: Intent::ActivateScene,
                entity: self.extract_scene_name(&text_lower),
                value: None,
                confidence: 0.75,
            };
        }
        
        // Check for status
        if text_lower.contains("durum") || text_lower.contains("status") || text_lower.contains("kaç") || text_lower.contains("derece") {
            return ParsedCommand {
                intent: Intent::StatusCheck,
                entity: self.extract_entity(&text_lower),
                value: None,
                confidence: 0.80,
            };
        }
        
        ParsedCommand {
            intent: Intent::Unknown,
            entity: None,
            value: None,
            confidence: 0.0,
        }
    }
    
    fn extract_entity(&self, text: &str) -> Option<String> {
        // Room names
        let rooms = [
            ("salon", "living_room"),
            ("oturma", "living_room"),
            ("living", "living_room"),
            ("yatak", "bedroom"),
            ("yatak odası", "bedroom"),
            ("bedroom", "bedroom"),
            ("mutfak", "kitchen"),
            ("kitchen", "kitchen"),
            ("banyo", "bathroom"),
            ("bathroom", "bathroom"),
            ("ofis", "office"),
            ("office", "office"),
            ("çalışma", "office"),
            ("koridor", "hallway"),
            ("hallway", "hallway"),
            ("garaj", "garage"),
            ("garage", "garage"),
        ];
        
        for (tr, en) in rooms {
            if text.contains(tr) || text.contains(en) {
                return Some(en.to_string());
            }
        }
        
        // Device types
        let devices = [
            ("ışık", "light"),
            ("light", "light"),
            ("lamba", "light"),
            ("klima", "climate"),
            ("termostat", "climate"),
            ("ısıtıcı", "climate"),
            ("kilit", "lock"),
            ("lock", "lock"),
            ("kapı", "door"),
            ("perde", "cover"),
            ("curtain", "cover"),
            ("jaluzi", "cover"),
            ("televizyon", "media"),
            ("tv", "media"),
            ("müzik", "media"),
            ("speaker", "media"),
        ];
        
        for (tr, en) in devices {
            if text.contains(tr) {
                return Some(en.to_string());
            }
        }
        
        None
    }
    
    fn extract_value(&self, text: &str) -> Option<String> {
        // Numbers
        for word in text.split_whitespace() {
            if let Ok(n) = word.parse::<i32>() {
                return Some(n.to_string());
            }
        }
        
        // Color names
        let colors = [
            ("kırmızı", "red"),
            ("red", "red"),
            ("mavi", "blue"),
            ("blue", "blue"),
            ("yeşil", "green"),
            ("green", "green"),
            ("sarı", "yellow"),
            ("yellow", "yellow"),
            ("turuncu", "orange"),
            ("orange", "orange"),
            ("mor", "purple"),
            ("purple", "purple"),
            ("pembe", "pink"),
            ("pink", "pink"),
            ("beyaz", "white"),
            ("white", "white"),
        ];
        
        for (tr, en) in colors {
            if text.contains(tr) {
                return Some(en.to_string());
            }
        }
        
        None
    }
    
    fn extract_scene_name(&self, text: &str) -> Option<String> {
        let scenes = [
            ("film", "movie"),
            ("movie", "movie"),
            ("sinema", "movie"),
            ("uyku", "good_night"),
            ("gece", "good_night"),
            ("night", "good_night"),
            ("sabah", "good_morning"),
            ("morning", "good_morning"),
            ("günaydın", "good_morning"),
            ("parti", "party"),
            ("party", "party"),
            ("odaklan", "focus"),
            ("focus", "focus"),
            ("çalışma", "focus"),
            ("okuma", "reading"),
            ("reading", "reading"),
            ("rahatla", "relax"),
            ("relax", "relax"),
            ("away", "away"),
            ("dışarı", "away"),
        ];
        
        for (tr, en) in scenes {
            if text.contains(tr) {
                return Some(en.to_string());
            }
        }
        
        None
    }
}

impl Default for VoiceCommandParser {
    fn default() -> Self {
        Self::new()
    }
}

fn default_patterns() -> Vec<CommandPattern> {
    vec![
        // Turn on
        CommandPattern {
            keywords: vec!["aç", "turn on", "yan", "on"],
            intent: Intent::TurnOn,
            extract_value: false,
        },
        // Turn off
        CommandPattern {
            keywords: vec!["kapat", "turn off", "söndür", "off", "kapa"],
            intent: Intent::TurnOff,
            extract_value: false,
        },
        // Toggle
        CommandPattern {
            keywords: vec!["değiştir", "toggle", "switch"],
            intent: Intent::Toggle,
            extract_value: false,
        },
        // Brightness
        CommandPattern {
            keywords: vec!["parlaklık", "brightness", "kıs", "dim", "yükselt", "parla"],
            intent: Intent::SetBrightness,
            extract_value: true,
        },
        // Color
        CommandPattern {
            keywords: vec!["renk", "color", "rengini"],
            intent: Intent::SetColor,
            extract_value: true,
        },
        // Temperature
        CommandPattern {
            keywords: vec!["derece", "temperature", "ısı", "ısıt", "soğut", "sıcaklık"],
            intent: Intent::SetTemperature,
            extract_value: true,
        },
        // Lock
        CommandPattern {
            keywords: vec!["kilitle", "lock", "kilit"],
            intent: Intent::Lock,
            extract_value: false,
        },
        // Unlock
        CommandPattern {
            keywords: vec!["kilit aç", "unlock", "kilidi aç"],
            intent: Intent::Unlock,
            extract_value: false,
        },
        // Open
        CommandPattern {
            keywords: vec!["aç", "open", "yukarı"],
            intent: Intent::Open,
            extract_value: false,
        },
        // Close
        CommandPattern {
            keywords: vec!["kapat", "close", "aşağı"],
            intent: Intent::Close,
            extract_value: false,
        },
        // Play
        CommandPattern {
            keywords: vec!["oynat", "play", "başlat"],
            intent: Intent::Play,
            extract_value: false,
        },
        // Pause
        CommandPattern {
            keywords: vec!["durdur", "pause", "dur"],
            intent: Intent::Pause,
            extract_value: false,
        },
        // Volume
        CommandPattern {
            keywords: vec!["ses", "volume", "sesi"],
            intent: Intent::SetVolume,
            extract_value: true,
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_turn_on() {
        let parser = VoiceCommandParser::new();
        
        let cmd = parser.parse("Salon ışığını aç");
        assert_eq!(cmd.intent, Intent::TurnOn);
        assert_eq!(cmd.entity, Some("living_room".to_string()));
    }
    
    #[test]
    fn test_parse_temperature() {
        let parser = VoiceCommandParser::new();
        
        let cmd = parser.parse("Isıyı 22 derece yap");
        assert_eq!(cmd.intent, Intent::SetTemperature);
        assert_eq!(cmd.value, Some("22".to_string()));
    }
    
    #[test]
    fn test_parse_scene() {
        let parser = VoiceCommandParser::new();
        
        let cmd = parser.parse("Film modunu aktifleştir");
        assert_eq!(cmd.intent, Intent::ActivateScene);
        assert_eq!(cmd.entity, Some("movie".to_string()));
    }
}
