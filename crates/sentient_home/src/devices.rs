//! ─── Device Controller ───

use serde::{Deserialize, Serialize};

/// Device command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeviceCommand {
    // General
    TurnOn(String),
    TurnOff(String),
    Toggle(String),
    
    // Light
    SetBrightness(String, u8),
    SetColor(String, u8, u8, u8),
    SetColorTemp(String, u32),
    SetEffect(String, String),
    
    // Climate
    SetTemperature(String, f32),
    SetHvacMode(String, String),
    SetFanMode(String, String),
    
    // Lock
    Lock(String),
    Unlock(String),
    
    // Cover
    OpenCover(String),
    CloseCover(String),
    SetCoverPosition(String, u8),
    
    // Media
    Play(String),
    Pause(String),
    Stop(String),
    Next(String),
    Previous(String),
    SetVolume(String, f32),
    Mute(String),
    Unmute(String),
    PlayMedia(String, String, String), // entity_id, content_id, content_type
    
    // Vacuum
    StartVacuum(String),
    StopVacuum(String),
    ReturnToBase(String),
    CleanSpot(String),
    
    // Custom
    Custom {
        entity_id: String,
        service: String,
        data: serde_json::Value,
    },
}

impl DeviceCommand {
    /// Create turn on command
    pub fn on(entity_id: &str) -> Self {
        Self::TurnOn(entity_id.to_string())
    }
    
    /// Create turn off command
    pub fn off(entity_id: &str) -> Self {
        Self::TurnOff(entity_id.to_string())
    }
    
    /// Create toggle command
    pub fn toggle(entity_id: &str) -> Self {
        Self::Toggle(entity_id.to_string())
    }
    
    /// Get entity ID
    pub fn entity_id(&self) -> &str {
        match self {
            Self::TurnOn(id) | Self::TurnOff(id) | Self::Toggle(id) => id,
            Self::SetBrightness(id, _) | Self::SetColorTemp(id, _) => id,
            Self::SetColor(id, _, _, _) => id,
            Self::SetEffect(id, _) => id,
            Self::SetTemperature(id, _) => id,
            Self::SetHvacMode(id, _) | Self::SetFanMode(id, _) => id,
            Self::Lock(id) | Self::Unlock(id) => id,
            Self::OpenCover(id) | Self::CloseCover(id) => id,
            Self::SetCoverPosition(id, _) => id,
            Self::Play(id) | Self::Pause(id) | Self::Stop(id) => id,
            Self::Next(id) | Self::Previous(id) => id,
            Self::SetVolume(id, _) => id,
            Self::Mute(id) | Self::Unmute(id) => id,
            Self::PlayMedia(id, _, _) => id,
            Self::StartVacuum(id) | Self::StopVacuum(id) => id,
            Self::ReturnToBase(id) | Self::CleanSpot(id) => id,
            Self::Custom { entity_id, .. } => entity_id,
        }
    }
}

/// Device controller
pub struct DeviceController;

impl DeviceController {
    /// Create new controller
    pub fn new() -> Self { Self }
    
    /// Parse natural language command
    pub fn parse_command(&self, text: &str, devices: &[String]) -> Option<DeviceCommand> {
        let text_lower = text.to_lowercase();
        
        // Light commands
        if text_lower.contains("ışık") || text_lower.contains("ışı") || text_lower.contains("light") || text_lower.contains("lamba") || text_lower.contains("aydınlat") {
            let entity = self.find_device(&text_lower, devices, "light")?;
            
            if text_lower.contains("aç") || text_lower.contains("on") || text_lower.contains("yan") {
                return Some(DeviceCommand::TurnOn(entity));
            }
            if text_lower.contains("kapat") || text_lower.contains("off") || text_lower.contains("söndür") {
                return Some(DeviceCommand::TurnOff(entity));
            }
            if text_lower.contains("parla") || text_lower.contains("bright") {
                return Some(DeviceCommand::SetBrightness(entity, 100));
            }
            if text_lower.contains("kıs") || text_lower.contains("dim") {
                return Some(DeviceCommand::SetBrightness(entity, 30));
            }
        }
        
        // Climate commands
        if text_lower.contains("ısı") || text_lower.contains("klima") || text_lower.contains("termostat") || text_lower.contains("heat") || text_lower.contains("cool") {
            let entity = self.find_device(&text_lower, devices, "climate")?;
            
            if let Some(temp) = self.extract_number(&text_lower) {
                return Some(DeviceCommand::SetTemperature(entity, temp as f32));
            }
            
            if text_lower.contains("aç") || text_lower.contains("on") {
                return Some(DeviceCommand::SetHvacMode(entity, "auto".into()));
            }
            if text_lower.contains("kapat") || text_lower.contains("off") {
                return Some(DeviceCommand::SetHvacMode(entity, "off".into()));
            }
        }
        
        // Lock commands
        if text_lower.contains("kilit") || text_lower.contains("lock") || text_lower.contains("kapı") {
            let entity = self.find_device(&text_lower, devices, "lock")?;
            
            if text_lower.contains("kilitle") || text_lower.contains("lock") || text_lower.contains("kapat") {
                return Some(DeviceCommand::Lock(entity));
            }
            if text_lower.contains("aç") || text_lower.contains("unlock") {
                return Some(DeviceCommand::Unlock(entity));
            }
        }
        
        // Cover/curtain commands
        if text_lower.contains("perde") || text_lower.contains("curtain") || text_lower.contains("blind") || text_lower.contains("garaj") {
            let entity = self.find_device(&text_lower, devices, "cover")?;
            
            if text_lower.contains("aç") || text_lower.contains("open") || text_lower.contains("yukarı") {
                return Some(DeviceCommand::OpenCover(entity));
            }
            if text_lower.contains("kapat") || text_lower.contains("close") || text_lower.contains("aşağı") {
                return Some(DeviceCommand::CloseCover(entity));
            }
        }
        
        None
    }
    
    fn find_device(&self, text: &str, devices: &[String], device_type: &str) -> Option<String> {
        // Room name aliases: Turkish -> English/HA naming
        let room_aliases: &[(&[&str], &[&str])] = &[
            (&["salon", "oturma", "living"], &["salon", "living", "oturma"]),
            (&["yatak", "bedroom"], &["yatak", "bedroom"]),
            (&["mutfak", "kitchen"], &["mutfak", "kitchen"]),
            (&["banyo", "bathroom"], &["banyo", "bathroom"]),
            (&["ofis", "office"], &["ofis", "office"]),
            (&["koridor", "hallway", "hall"], &["koridor", "hallway", "hall"]),
        ];
        
        // Find which room group the text refers to
        for (triggers, search_terms) in room_aliases {
            let text_matches_trigger = triggers.iter().any(|t| text.contains(t));
            if text_matches_trigger {
                // Search devices using all terms in this group
                for device in devices {
                    let device_lower = device.to_lowercase();
                    if device_lower.starts_with(&format!("{}.", device_type)) {
                        if search_terms.iter().any(|term| device_lower.contains(term)) {
                            return Some(device.clone());
                        }
                    }
                }
            }
        }
        
        // Fallback: try simple matching
        let room_names = ["salon", "living", "oturma", "yatak", "bedroom", "mutfak", "kitchen", "banyo", "bathroom", "ofis", "office", "koridor", "hallway", "hall"];
        
        for room in room_names {
            if text.contains(room) {
                for device in devices {
                    let device_lower = device.to_lowercase();
                    if device_lower.starts_with(&format!("{}.", device_type)) && device_lower.contains(room) {
                        return Some(device.clone());
                    }
                }
            }
        }
        
        // Return first device of type if no room match
        for device in devices {
            if device.starts_with(&format!("{}.", device_type)) {
                return Some(device.clone());
            }
        }
        
        None
    }
    
    fn extract_number(&self, text: &str) -> Option<i32> {
        for word in text.split_whitespace() {
            if let Ok(n) = word.parse::<i32>() {
                return Some(n);
            }
        }
        None
    }
}

impl Default for DeviceController {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_light_on() {
        let controller = DeviceController::new();
        let devices = vec!["light.living_room".to_string(), "light.bedroom".to_string()];
        
        let cmd = controller.parse_command("Salon ışığını aç", &devices);
        assert!(matches!(cmd, Some(DeviceCommand::TurnOn(id)) if id == "light.living_room"));
    }
    
    #[test]
    fn test_parse_temperature() {
        let controller = DeviceController::new();
        let devices = vec!["climate.hallway".to_string()];
        
        let cmd = controller.parse_command("Koridor ısısı 22 derece yap", &devices);
        assert!(matches!(cmd, Some(DeviceCommand::SetTemperature(_, 22.0))));
    }
}
