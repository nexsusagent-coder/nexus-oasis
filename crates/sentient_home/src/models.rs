//! ─── Smart Home Models ───

use serde::{Deserialize, Serialize};

/// Device entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Device {
    pub entity_id: String,
    pub name: String,
    pub device_type: DeviceType,
    pub state: EntityState,
    pub area: Option<String>,
    pub capabilities: Vec<Capability>,
    pub attributes: serde_json::Value,
}

impl Device {
    /// Check if device is available
    pub fn is_available(&self) -> bool {
        self.state != EntityState::Unavailable
    }
    
    /// Check if device is on
    pub fn is_on(&self) -> bool {
        matches!(self.state, EntityState::On | EntityState::Open | EntityState::Home)
    }
    
    /// Get friendly name
    pub fn friendly_name(&self) -> &str {
        &self.name
    }
}

/// Device type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeviceType {
    Light,
    Switch,
    Climate,
    Lock,
    Cover,
    Camera,
    Media,
    Sensor,
    BinarySensor,
    Vacuum,
    Fan,
    Humidifier,
    WaterHeater,
    Alarm,
    Unknown,
}

impl DeviceType {
    /// Parse from entity_id prefix
    pub fn from_entity_id(id: &str) -> Self {
        let prefix = id.split('.').next().unwrap_or("");
        match prefix {
            "light" => Self::Light,
            "switch" => Self::Switch,
            "climate" => Self::Climate,
            "lock" => Self::Lock,
            "cover" => Self::Cover,
            "camera" => Self::Camera,
            "media_player" => Self::Media,
            "sensor" => Self::Sensor,
            "binary_sensor" => Self::BinarySensor,
            "vacuum" => Self::Vacuum,
            "fan" => Self::Fan,
            "humidifier" => Self::Humidifier,
            "water_heater" => Self::WaterHeater,
            "alarm_control_panel" => Self::Alarm,
            _ => Self::Unknown,
        }
    }
}

/// Device state
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntityState {
    On,
    Off,
    Open,
    Closed,
    Home,
    Away,
    Unavailable,
    Unknown,
    Custom(String),
}

impl std::fmt::Display for EntityState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::On => write!(f, "on"),
            Self::Off => write!(f, "off"),
            Self::Open => write!(f, "open"),
            Self::Closed => write!(f, "closed"),
            Self::Home => write!(f, "home"),
            Self::Away => write!(f, "away"),
            Self::Unavailable => write!(f, "unavailable"),
            Self::Unknown => write!(f, "unknown"),
            Self::Custom(s) => write!(f, "{}", s),
        }
    }
}

impl From<&str> for EntityState {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "on" => Self::On,
            "off" => Self::Off,
            "open" => Self::Open,
            "closed" => Self::Closed,
            "home" => Self::Home,
            "away" => Self::Away,
            "unavailable" => Self::Unavailable,
            other => Self::Custom(other.to_string()),
        }
    }
}

/// Device capability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Capability {
    Brightness,
    ColorTemp,
    RgbColor,
    Temperature,
    Humidity,
    Volume,
    Mute,
    Source,
    Position,
    Tilt,
    Speed,
    Preset,
    HvacMode,
    FanMode,
    SwingMode,
}

/// Area/room
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Area {
    pub area_id: String,
    pub name: String,
    pub icon: Option<String>,
    pub devices: Vec<String>,
}

/// Climate settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClimateSettings {
    pub temperature: Option<f32>,
    pub target_temp_high: Option<f32>,
    pub target_temp_low: Option<f32>,
    pub humidity: Option<f32>,
    pub hvac_mode: Option<HvacMode>,
    pub fan_mode: Option<String>,
    pub swing_mode: Option<String>,
}

/// HVAC mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HvacMode {
    Off,
    Heat,
    Cool,
    HeatCool,
    Auto,
    Dry,
    FanOnly,
}

/// Light settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightSettings {
    pub brightness: Option<u8>,
    pub color_temp: Option<u32>,
    pub rgb_color: Option<(u8, u8, u8)>,
    pub effect: Option<String>,
    pub flash: Option<String>,
    pub transition: Option<u32>,
}

/// Media player settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaSettings {
    pub volume_level: Option<f32>,
    pub is_volume_muted: Option<bool>,
    pub media_content_id: Option<String>,
    pub media_content_type: Option<String>,
    pub source: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_device_type_from_id() {
        assert_eq!(DeviceType::from_entity_id("light.living_room"), DeviceType::Light);
        assert_eq!(DeviceType::from_entity_id("climate.hallway"), DeviceType::Climate);
        assert_eq!(DeviceType::from_entity_id("unknown.thing"), DeviceType::Unknown);
    }
    
    #[test]
    fn test_entity_state_from_str() {
        assert_eq!(EntityState::from("on"), EntityState::On);
        assert_eq!(EntityState::from("OFF"), EntityState::Off);
        assert_eq!(EntityState::from("custom"), EntityState::Custom("custom".into()));
    }
}
