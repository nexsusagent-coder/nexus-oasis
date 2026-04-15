//! ─── System Tray Integration ───

use serde::{Deserialize, Serialize};

/// System tray configuration
#[derive(Debug, Clone)]
pub struct TrayConfig {
    pub icon_path: String,
    pub tooltip: String,
    pub menu_items: Vec<TrayMenuItem>,
}

impl Default for TrayConfig {
    fn default() -> Self {
        Self {
            icon_path: "icon.png".into(),
            tooltip: "SENTIENT OS".into(),
            menu_items: vec![
                TrayMenuItem::new("Open Dashboard", "open_dashboard"),
                TrayMenuItem::separator(),
                TrayMenuItem::new("Voice Control", "toggle_voice"),
                TrayMenuItem::new("Settings", "open_settings"),
                TrayMenuItem::separator(),
                TrayMenuItem::new("Quit", "quit"),
            ],
        }
    }
}

/// Tray menu item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrayMenuItem {
    pub label: String,
    pub action_id: String,
    pub enabled: bool,
    pub is_separator: bool,
}

impl TrayMenuItem {
    pub fn new(label: &str, action_id: &str) -> Self {
        Self { label: label.into(), action_id: action_id.into(), enabled: true, is_separator: false }
    }
    
    pub fn separator() -> Self {
        Self { label: String::new(), action_id: String::new(), enabled: false, is_separator: true }
    }
    
    pub fn disabled(mut self) -> Self {
        self.enabled = false;
        self
    }
}

/// System tray handler
pub struct SystemTray {
    config: TrayConfig,
    visible: bool,
}

impl SystemTray {
    pub fn new(config: TrayConfig) -> Self {
        Self { config, visible: false }
    }
    
    pub async fn initialize(&mut self) -> crate::Result<()> {
        tracing::info!("Initializing system tray");
        // TODO: Platform-specific tray implementation
        // Linux: GTK AppIndicator
        // macOS: NSStatusItem
        // Windows: Shell_NotifyIcon
        self.visible = true;
        Ok(())
    }
    
    pub fn show(&mut self) {
        self.visible = true;
    }
    
    pub fn hide(&mut self) {
        self.visible = false;
    }
    
    pub fn is_visible(&self) -> bool {
        self.visible
    }
    
    pub fn set_tooltip(&mut self, tooltip: &str) {
        self.config.tooltip = tooltip.into();
    }
    
    pub fn set_icon(&mut self, icon_path: &str) {
        self.config.icon_path = icon_path.into();
    }
    
    pub fn update_menu(&mut self, items: Vec<TrayMenuItem>) {
        self.config.menu_items = items;
    }
    
    pub fn get_menu(&self) -> &[TrayMenuItem] {
        &self.config.menu_items
    }
    
    pub async fn handle_menu_click(&self, action_id: &str) -> crate::Result<()> {
        tracing::info!("Tray menu clicked: {}", action_id);
        match action_id {
            "open_dashboard" => self.open_dashboard().await?,
            "toggle_voice" => self.toggle_voice().await?,
            "open_settings" => self.open_settings().await?,
            "quit" => self.quit().await?,
            _ => tracing::warn!("Unknown tray action: {}", action_id),
        }
        Ok(())
    }
    
    async fn open_dashboard(&self) -> crate::Result<()> {
        tracing::info!("Opening dashboard from tray");
        // TODO: Open browser to dashboard URL
        Ok(())
    }
    
    async fn toggle_voice(&self) -> crate::Result<()> {
        tracing::info!("Toggling voice from tray");
        // TODO: Toggle sentient_voice
        Ok(())
    }
    
    async fn open_settings(&self) -> crate::Result<()> {
        tracing::info!("Opening settings from tray");
        Ok(())
    }
    
    async fn quit(&self) -> crate::Result<()> {
        tracing::info!("Quit requested from tray");
        // TODO: Graceful shutdown
        Ok(())
    }
}

impl Default for SystemTray {
    fn default() -> Self {
        Self::new(TrayConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_tray_menu_item() {
        let item = TrayMenuItem::new("Test", "test_action");
        assert_eq!(item.label, "Test");
        assert!(item.enabled);
        assert!(!item.is_separator);
    }
    
    #[test]
    fn test_separator() {
        let sep = TrayMenuItem::separator();
        assert!(sep.is_separator);
    }
}
