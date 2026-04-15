//! ─── Home Assistant Client ───

use reqwest::Client;

use crate::models::*;
use crate::devices::DeviceCommand;
use crate::{HomeResult, HomeError};

/// Home Assistant client configuration
#[derive(Debug, Clone)]
pub struct HomeConfig {
    pub url: String,
    pub token: String,
    pub verify_ssl: bool,
}

impl HomeConfig {
    pub fn new(url: &str, token: &str) -> Self {
        Self {
            url: url.trim_end_matches('/').to_string(),
            token: token.to_string(),
            verify_ssl: true,
        }
    }
    
    pub fn from_env() -> HomeResult<Self> {
        let url = std::env::var("HOME_ASSISTANT_URL")
            .unwrap_or_else(|_| "http://homeassistant.local:8123".into());
        let token = std::env::var("HOME_ASSISTANT_TOKEN")
            .map_err(|_| HomeError::AuthFailed("HOME_ASSISTANT_TOKEN not set".into()))?;
        
        Ok(Self::new(&url, &token))
    }
}

/// Home Assistant client
pub struct HomeClient {
    config: HomeConfig,
    http: Client,
}

impl HomeClient {
    /// Create new client
    pub fn new(config: HomeConfig) -> HomeResult<Self> {
        let http = Client::builder()
            .danger_accept_invalid_certs(!config.verify_ssl)
            .build()
            .map_err(|e| HomeError::ConnectionFailed(e.to_string()))?;
        
        Ok(Self { config, http })
    }
    
    /// Connect with token
    pub async fn connect(url: &str, token: &str) -> HomeResult<Self> {
        Self::new(HomeConfig::new(url, token))
    }
    
    /// Get all devices
    pub async fn get_devices(&self) -> HomeResult<Vec<Device>> {
        let states = self.get_states().await?;
        Ok(states.into_iter()
            .map(|s| self.parse_entity_to_device(s))
            .collect())
    }
    
    /// Get devices by type
    pub async fn get_devices_by_type(&self, device_type: DeviceType) -> HomeResult<Vec<Device>> {
        let devices = self.get_devices().await?;
        Ok(devices.into_iter()
            .filter(|d| d.device_type == device_type)
            .collect())
    }
    
    /// Get device by entity_id
    pub async fn get_device(&self, entity_id: &str) -> HomeResult<Device> {
        let state = self.get_state(entity_id).await?;
        Ok(self.parse_entity_to_device(state))
    }
    
    /// Execute device command
    pub async fn execute_command(&self, command: DeviceCommand) -> HomeResult<()> {
        match command {
            DeviceCommand::TurnOn(entity_id) => self.call_service(&entity_id, "turn_on", serde_json::json!({})).await,
            DeviceCommand::TurnOff(entity_id) => self.call_service(&entity_id, "turn_off", serde_json::json!({})).await,
            DeviceCommand::Toggle(entity_id) => self.call_service(&entity_id, "toggle", serde_json::json!({})).await,
            DeviceCommand::SetBrightness(entity_id, brightness) => {
                self.call_service(&entity_id, "turn_on", serde_json::json!({
                    "brightness_pct": brightness
                })).await
            }
            DeviceCommand::SetColor(entity_id, r, g, b) => {
                self.call_service(&entity_id, "turn_on", serde_json::json!({
                    "rgb_color": [r, g, b]
                })).await
            }
            DeviceCommand::SetTemperature(entity_id, temp) => {
                self.call_service(&entity_id, "set_temperature", serde_json::json!({
                    "temperature": temp
                })).await
            }
            DeviceCommand::SetHvacMode(entity_id, mode) => {
                self.call_service(&entity_id, "set_hvac_mode", serde_json::json!({
                    "hvac_mode": mode
                })).await
            }
            DeviceCommand::Lock(entity_id) => self.call_service(&entity_id, "lock", serde_json::json!({})).await,
            DeviceCommand::Unlock(entity_id) => self.call_service(&entity_id, "unlock", serde_json::json!({})).await,
            DeviceCommand::OpenCover(entity_id) => self.call_service(&entity_id, "open_cover", serde_json::json!({})).await,
            DeviceCommand::CloseCover(entity_id) => self.call_service(&entity_id, "close_cover", serde_json::json!({})).await,
            DeviceCommand::SetCoverPosition(entity_id, pos) => {
                self.call_service(&entity_id, "set_cover_position", serde_json::json!({
                    "position": pos
                })).await
            }
            DeviceCommand::Play(entity_id) => self.call_service(&entity_id, "media_play", serde_json::json!({})).await,
            DeviceCommand::Pause(entity_id) => self.call_service(&entity_id, "media_pause", serde_json::json!({})).await,
            DeviceCommand::Stop(entity_id) => self.call_service(&entity_id, "media_stop", serde_json::json!({})).await,
            DeviceCommand::Next(entity_id) => self.call_service(&entity_id, "media_next_track", serde_json::json!({})).await,
            DeviceCommand::Previous(entity_id) => self.call_service(&entity_id, "media_previous_track", serde_json::json!({})).await,
            DeviceCommand::SetVolume(entity_id, level) => {
                self.call_service(&entity_id, "volume_set", serde_json::json!({
                    "volume_level": level
                })).await
            }
            DeviceCommand::Mute(entity_id) => self.call_service(&entity_id, "volume_mute", serde_json::json!({"is_volume_muted": true})).await,
            DeviceCommand::Unmute(entity_id) => self.call_service(&entity_id, "volume_mute", serde_json::json!({"is_volume_muted": false})).await,
            DeviceCommand::PlayMedia(entity_id, content_id, content_type) => {
                self.call_service(&entity_id, "play_media", serde_json::json!({
                    "media_content_id": content_id,
                    "media_content_type": content_type
                })).await
            }
            DeviceCommand::StartVacuum(entity_id) => self.call_service(&entity_id, "start", serde_json::json!({})).await,
            DeviceCommand::StopVacuum(entity_id) => self.call_service(&entity_id, "stop", serde_json::json!({})).await,
            DeviceCommand::ReturnToBase(entity_id) => self.call_service(&entity_id, "return_to_base", serde_json::json!({})).await,
            DeviceCommand::CleanSpot(entity_id) => self.call_service(&entity_id, "clean_spot", serde_json::json!({})).await,
            DeviceCommand::SetColorTemp(entity_id, temp) => {
                self.call_service(&entity_id, "turn_on", serde_json::json!({
                    "color_temp": temp
                })).await
            }
            DeviceCommand::SetEffect(entity_id, effect) => {
                self.call_service(&entity_id, "turn_on", serde_json::json!({
                    "effect": effect
                })).await
            }
            DeviceCommand::SetFanMode(entity_id, mode) => {
                self.call_service(&entity_id, "set_fan_mode", serde_json::json!({
                    "fan_mode": mode
                })).await
            }
            DeviceCommand::Play(entity_id) => self.call_service(&entity_id, "media_play", serde_json::json!({})).await,
            DeviceCommand::Pause(entity_id) => self.call_service(&entity_id, "media_pause", serde_json::json!({})).await,
            DeviceCommand::SetVolume(entity_id, level) => {
                self.call_service(&entity_id, "volume_set", serde_json::json!({
                    "volume_level": level
                })).await
            }
            DeviceCommand::Custom { entity_id, service, data } => {
                self.call_service(&entity_id, &service, data).await
            }
        }
    }
    
    /// Activate scene
    pub async fn activate_scene(&self, scene_id: &str) -> HomeResult<()> {
        self.call_service(&format!("scene.{}", scene_id), "turn_on", serde_json::json!({})).await
    }
    
    /// Get areas
    pub async fn get_areas(&self) -> HomeResult<Vec<Area>> {
        // TODO: Implement with Home Assistant API
        Ok(vec![])
    }
    
    // Internal API methods
    
    async fn get_states(&self) -> HomeResult<Vec<serde_json::Value>> {
        let url = format!("{}/api/states", self.config.url);
        
        let response = self.http
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.config.token))
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(HomeError::ConnectionFailed(format!("API error: {}", response.status())));
        }
        
        Ok(response.json().await?)
    }
    
    async fn get_state(&self, entity_id: &str) -> HomeResult<serde_json::Value> {
        let url = format!("{}/api/states/{}", self.config.url, entity_id);
        
        let response = self.http
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.config.token))
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(HomeError::DeviceNotFound(entity_id.to_string()));
        }
        
        Ok(response.json().await?)
    }
    
    async fn call_service(&self, entity_id: &str, service: &str, data: serde_json::Value) -> HomeResult<()> {
        let (domain, _) = entity_id.split_once('.').unwrap_or((entity_id, ""));
        let url = format!("{}/api/services/{}/{}", self.config.url, domain, service);
        
        let mut body = data;
        if let Some(obj) = body.as_object_mut() {
            obj.insert("entity_id".to_string(), serde_json::json!(entity_id));
        }
        
        let response = self.http
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.config.token))
            .json(&body)
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(HomeError::CommandFailed(format!("Service call failed: {}", response.status())));
        }
        
        Ok(())
    }
    
    fn parse_entity_to_device(&self, state: serde_json::Value) -> Device {
        let entity_id = state["entity_id"].as_str().unwrap_or("unknown").to_string();
        let name = state["attributes"]["friendly_name"]
            .as_str()
            .unwrap_or(&entity_id)
            .to_string();
        
        Device {
            entity_id: entity_id.clone(),
            name,
            device_type: DeviceType::from_entity_id(&entity_id),
            state: EntityState::from(state["state"].as_str().unwrap_or("unknown")),
            area: state["attributes"]["area"].as_str().map(|s| s.to_string()),
            capabilities: vec![],
            attributes: state["attributes"].clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_config_creation() {
        let config = HomeConfig::new("http://localhost:8123", "test_token");
        assert_eq!(config.url, "http://localhost:8123");
        assert!(!config.url.ends_with('/'));
    }
}
