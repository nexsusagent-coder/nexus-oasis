//! ─── Global Hotkey Support ───

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Hotkey modifiers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Modifier {
    Ctrl,
    Alt,
    Shift,
    Super, // Windows key / Command / Meta
}

impl Modifier {
    pub fn as_str(&self) -> &'static str {
        match self {
            Modifier::Ctrl => "Ctrl",
            Modifier::Alt => "Alt",
            Modifier::Shift => "Shift",
            Modifier::Super => "Super",
        }
    }
}

/// Hotkey definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hotkey {
    pub id: String,
    pub modifiers: Vec<Modifier>,
    pub key: String,
    pub action: String,
    pub description: String,
    pub enabled: bool,
}

impl Hotkey {
    pub fn new(id: &str, modifiers: Vec<Modifier>, key: &str, action: &str) -> Self {
        Self {
            id: id.to_string(),
            modifiers,
            key: key.to_string(),
            action: action.to_string(),
            description: String::new(),
            enabled: true,
        }
    }
    
    pub fn with_description(mut self, desc: &str) -> Self {
        self.description = desc.to_string();
        self
    }
    
    pub fn to_string_repr(&self) -> String {
        let mods: Vec<&str> = self.modifiers.iter().map(|m| m.as_str()).collect();
        format!("{}+{}", mods.join("+"), self.key)
    }
    
    pub fn parse(s: &str) -> Option<Self> {
        let parts: Vec<&str> = s.split('+').collect();
        if parts.is_empty() { return None; }
        
        let key = parts.last()?.to_string();
        let modifiers = parts[..parts.len()-1].iter()
            .filter_map(|p| match *p {
                "Ctrl" | "ctrl" => Some(Modifier::Ctrl),
                "Alt" | "alt" => Some(Modifier::Alt),
                "Shift" | "shift" => Some(Modifier::Shift),
                "Super" | "super" | "Win" | "Cmd" | "Meta" => Some(Modifier::Super),
                _ => None,
            })
            .collect();
        
        Some(Self::new("", modifiers, &key, ""))
    }
}

/// Global hotkey manager
pub struct HotkeyManager {
    hotkeys: HashMap<String, Hotkey>,
    registered: bool,
}

impl HotkeyManager {
    pub fn new() -> Self {
        Self {
            hotkeys: HashMap::new(),
            registered: false,
        }
    }
    
    /// Register a hotkey
    pub fn register(&mut self, hotkey: Hotkey) -> crate::Result<()> {
        let id = hotkey.id.clone();
        tracing::info!("Registering hotkey: {} -> {}", hotkey.to_string_repr(), hotkey.action);
        self.hotkeys.insert(id, hotkey);
        Ok(())
    }
    
    /// Unregister a hotkey
    pub fn unregister(&mut self, id: &str) -> Option<Hotkey> {
        tracing::info!("Unregistering hotkey: {}", id);
        self.hotkeys.remove(id)
    }
    
    /// Register all hotkeys with the system
    pub async fn register_all(&mut self) -> crate::Result<()> {
        if self.registered { return Ok(()); }
        
        tracing::info!("Registering {} hotkeys with system", self.hotkeys.len());
        // TODO: Platform-specific global hotkey registration
        // Linux: XGrabKey (X11) or GNOME Shell extension
        // macOS: NSEvent addGlobalMonitorForEvents
        // Windows: RegisterHotKey
        
        self.registered = true;
        Ok(())
    }
    
    /// Unregister all hotkeys
    pub async fn unregister_all(&mut self) -> crate::Result<()> {
        if !self.registered { return Ok(()); }
        
        tracing::info!("Unregistering all hotkeys");
        // TODO: Platform-specific cleanup
        self.registered = false;
        Ok(())
    }
    
    /// Get hotkey by ID
    pub fn get(&self, id: &str) -> Option<&Hotkey> {
        self.hotkeys.get(id)
    }
    
    /// Get all hotkeys
    pub fn get_all(&self) -> Vec<&Hotkey> {
        self.hotkeys.values().collect()
    }
    
    /// Handle hotkey press
    pub async fn handle_press(&self, id: &str) -> crate::Result<()> {
        if let Some(hotkey) = self.hotkeys.get(id) {
            if !hotkey.enabled { return Ok(()); }
            
            tracing::info!("Hotkey pressed: {} ({})", hotkey.to_string_repr(), hotkey.action);
            // TODO: Execute action
        }
        Ok(())
    }
    
    /// Load default hotkeys
    pub fn load_defaults(&mut self) {
        let defaults = vec![
            Hotkey::new("voice_toggle", vec![Modifier::Super], "Space", "toggle_voice")
                .with_description("Toggle voice control"),
            Hotkey::new("screenshot", vec![Modifier::Ctrl, Modifier::Shift], "S", "screenshot")
                .with_description("Take screenshot"),
            Hotkey::new("quick_action", vec![Modifier::Super], "K", "quick_action")
                .with_description("Quick action palette"),
            Hotkey::new("dashboard", vec![Modifier::Super], "D", "open_dashboard")
                .with_description("Open dashboard"),
            Hotkey::new("lock", vec![Modifier::Super], "L", "lock_screen")
                .with_description("Lock screen"),
        ];
        
        for hotkey in defaults {
            let _ = self.register(hotkey);
        }
    }
}

impl Default for HotkeyManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_hotkey_creation() {
        let hk = Hotkey::new("test", vec![Modifier::Ctrl, Modifier::Alt], "T", "test_action");
        assert_eq!(hk.to_string_repr(), "Ctrl+Alt+T");
    }
    
    #[test]
    fn test_hotkey_parse() {
        let hk = Hotkey::parse("Ctrl+Shift+S").unwrap();
        assert_eq!(hk.key, "S");
        assert_eq!(hk.modifiers.len(), 2);
    }
}
