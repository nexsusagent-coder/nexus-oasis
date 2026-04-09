//! ─── SENTIENT Desktop Application ───
//!
//! Cross-platform desktop app using Tauri (Rust + Web)
//! Supports: macOS, Windows, Linux

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use tauri_plugin_notification::NotificationExt;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

mod commands;
mod tray;
mod voice;

pub use commands::*;
pub use tray::*;
pub use voice::*;

/// Application state
pub struct AppState {
    pub config: Arc<Mutex<AppConfig>>,
    pub voice_active: Arc<Mutex<bool>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub api_key: Option<String>,
    pub model: String,
    pub voice_enabled: bool,
    pub wake_word: String,
    pub channels: Vec<String>,
    pub theme: String,
    pub language: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            api_key: None,
            model: "gpt-4o".into(),
            voice_enabled: true,
            wake_word: "sentient".into(),
            channels: vec!["telegram".into(), "discord".into()],
            theme: "dark".into(),
            language: "tr".into(),
        }
    }
}

fn main() {
    env_logger::init();
    
    let state = AppState {
        config: Arc::new(Mutex::new(AppConfig::default())),
        voice_active: Arc::new(Mutex::new(false)),
    };
    
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_websocket::init())
        .manage(state)
        .setup(|app| {
            // Setup system tray
            setup_tray(app)?;
            
            // Setup voice listener
            #[cfg(feature = "voice")]
            {
                let handle = app.handle().clone();
                tokio::spawn(async move {
                    if let Err(e) = voice::start_voice_listener(handle).await {
                        log::error!("Voice listener error: {}", e);
                    }
                });
            }
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Core commands
            commands::get_config,
            commands::set_config,
            commands::chat,
            commands::chat_stream,
            commands::stop_generation,
            
            // Voice commands
            commands::start_voice,
            commands::stop_voice,
            commands::get_voice_status,
            
            // Channel commands
            commands::send_message,
            commands::get_channels,
            commands::connect_channel,
            commands::disconnect_channel,
            
            // Skills commands
            commands::search_skills,
            commands::install_skill,
            commands::list_installed_skills,
            
            // System commands
            commands::get_system_info,
            commands::open_logs,
            commands::check_updates,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
