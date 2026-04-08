//! General Settings - Genel ayarlar

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralSettings {
    /// Arayüz dili
    pub language: String,
    
    /// Tema
    pub theme: String,
    
    /// Otomatik başlat
    pub auto_start: bool,
    
    /// Bildirimler
    pub notifications: bool,
    
    /// Güncelleme kontrolü
    pub auto_update_check: bool,
    
    /// Ses efekleri
    pub sound_effects: bool,
    
    /// Animasyonlar
    pub animations: bool,
    
    /// Dashboard portu
    pub dashboard_port: u16,
}

impl Default for GeneralSettings {
    fn default() -> Self {
        Self {
            language: "tr".to_string(),
            theme: "dark".to_string(),
            auto_start: false,
            notifications: true,
            auto_update_check: true,
            sound_effects: false,
            animations: true,
            dashboard_port: 8080,
        }
    }
}
