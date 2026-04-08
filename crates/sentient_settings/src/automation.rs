//! Automation Settings - Otomasyon ayarları

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationSettings {
    /// Otonom mod aktif
    pub autonomous_mode: bool,
    
    /// Ekran kaydı
    pub screen_recording: bool,
    
    /// Klavye kontrolü
    pub keyboard_control: bool,
    
    /// Mouse kontrolü
    pub mouse_control: bool,
    
    /// Dosya sistemi erişimi
    pub file_system_access: String, // "full", "read-only", "none"
    
    /// Maksimum işlem sayısı
    pub max_actions: u32,
    
    /// İşlem onayı gerektir
    pub require_confirmation: bool,
    
    /// Güvenli mod
    pub safe_mode: bool,
    
    /// Zaman aşımı (saniye)
    pub timeout: u32,
    
    /// Retry delay (ms)
    pub retry_delay: u32,
    
    /// Screenshot kaydet
    pub save_screenshots: bool,
    
    /// Screenshot klasörü
    pub screenshot_path: String,
    
    /// Macro kaydetme
    pub macro_recording: bool,
    
    /// Scheduled tasks aktif
    pub scheduled_tasks: bool,
}

impl Default for AutomationSettings {
    fn default() -> Self {
        Self {
            autonomous_mode: false,
            screen_recording: false,
            keyboard_control: false,
            mouse_control: false,
            file_system_access: "read-only".to_string(),
            max_actions: 100,
            require_confirmation: true,
            safe_mode: true,
            timeout: 300,
            retry_delay: 1000,
            save_screenshots: false,
            screenshot_path: "./screenshots".to_string(),
            macro_recording: false,
            scheduled_tasks: true,
        }
    }
}
