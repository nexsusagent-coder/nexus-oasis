//! Profiles - Profil Yönetimi

use serde::{Deserialize, Serialize};
use crate::setup::config::SetupConfig;
use crate::setup::profiles_dir;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProfileType {
    Default,
    Strict,
    Developer,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetupProfile {
    pub name: String,
    pub profile_type: ProfileType,
    pub config: SetupConfig,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl SetupProfile {
    pub fn from_config(config: &SetupConfig, name: String, ptype: ProfileType) -> Self {
        Self {
            name,
            profile_type: ptype,
            config: config.clone(),
            created_at: chrono::Utc::now(),
        }
    }
}

#[derive(Debug)]
pub struct ProfileManager {
    profiles: Vec<SetupProfile>,
}

impl ProfileManager {
    pub fn new() -> Self {
        Self { profiles: vec![] }
    }
    
    pub fn save(&self, profile: &SetupProfile) -> Result<(), std::io::Error> {
        let dir = profiles_dir();
        std::fs::create_dir_all(&dir)?;
        let path = dir.join(format!("{}.toml", profile.name));
        let s = toml::to_string_pretty(profile)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        std::fs::write(&path, s)
    }
    
    pub fn load(&mut self, name: &str) -> Result<SetupProfile, std::io::Error> {
        let path = profiles_dir().join(format!("{}.toml", name));
        let s = std::fs::read_to_string(&path)?;
        toml::from_str(&s).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
    }
    
    pub fn list(&self) -> &[SetupProfile] {
        &self.profiles
    }
}

impl Default for ProfileManager {
    fn default() -> Self { Self::new() }
}
