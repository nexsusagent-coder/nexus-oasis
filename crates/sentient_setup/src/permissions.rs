//! Permission Management - Yetki yönetimi

use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Yetki seviyeleri
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum PermissionLevel {
    /// Sadece okuma
    ReadOnly = 1,
    /// Dosya işlemleri
    FileAccess = 2,
    /// Klavye/Mouse kontrolü
    GuiControl = 3,
    /// Tam otonom
    FullAuto = 4,
    /// Sistem yönetimi
    SystemAdmin = 5,
}

impl Default for PermissionLevel {
    fn default() -> Self {
        Self::FileAccess
    }
}

impl std::fmt::Display for PermissionLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ReadOnly => write!(f, "Level 1 - Sadece Okuma"),
            Self::FileAccess => write!(f, "Level 2 - Dosya Erişimi"),
            Self::GuiControl => write!(f, "Level 3 - GUI Kontrolü"),
            Self::FullAuto => write!(f, "Level 4 - Tam Otonom"),
            Self::SystemAdmin => write!(f, "Level 5 - Sistem Yönetimi"),
        }
    }
}

/// Yetki yöneticisi
#[derive(Debug, Clone)]
pub struct AuthManager {
    current_level: PermissionLevel,
    granted_permissions: HashSet<String>,
    require_confirmation: bool,
    safe_mode: bool,
}

impl AuthManager {
    pub fn new() -> Self {
        Self {
            current_level: PermissionLevel::FileAccess,
            granted_permissions: HashSet::new(),
            require_confirmation: true,
            safe_mode: true,
        }
    }
    
    /// Yetki seviyesini ayarla
    pub fn set_level(&mut self, level: PermissionLevel) -> bool {
        // Level 3+ için ek onay gerekebilir
        if level >= PermissionLevel::GuiControl && self.safe_mode {
            println!("⚠️  GUI kontrolü için onay gerekli!");
            println!("   Bu seviye klavye ve fare kontrolü içerir.");
            return false;
        }
        
        self.current_level = level;
        true
    }
    
    /// Mevcut yetki seviyesini al
    pub fn current_level(&self) -> PermissionLevel {
        self.current_level
    }
    
    /// GUI kontrolü izni var mı?
    pub fn can_control_gui(&self) -> bool {
        self.current_level >= PermissionLevel::GuiControl
    }
    
    /// Dosya erişimi var mı?
    pub fn can_access_files(&self) -> bool {
        self.current_level >= PermissionLevel::FileAccess
    }
    
    /// Ağ erişimi var mı?
    pub fn can_access_network(&self) -> bool {
        self.current_level >= PermissionLevel::FileAccess
    }
    
    /// Sistem komutları çalıştırabilir mi?
    pub fn can_execute_system(&self) -> bool {
        self.current_level >= PermissionLevel::FullAuto
    }
    
    /// İşlem için onay gerekiyor mu?
    pub fn needs_confirmation(&self, action: &str) -> bool {
        if !self.require_confirmation {
            return false;
        }
        
        // Tehlikeli işlemler için her zaman onay sor
        let dangerous_actions = [
            "delete", "remove", "rm", "format", "shutdown",
            "reboot", "kill", "terminate", "drop"
        ];
        
        dangerous_actions.iter().any(|d| action.to_lowercase().contains(d))
    }
    
    /// İzin ver
    pub fn grant(&mut self, permission: &str) {
        self.granted_permissions.insert(permission.to_string());
    }
    
    /// İzin kaldır
    pub fn revoke(&mut self, permission: &str) {
        self.granted_permissions.remove(permission);
    }
    
    /// İzin var mı?
    pub fn has_permission(&self, permission: &str) -> bool {
        self.granted_permissions.contains(permission)
    }
    
    /// Safe mode ayarla
    pub fn set_safe_mode(&mut self, enabled: bool) {
        self.safe_mode = enabled;
    }
    
    /// Onay gereksinimini ayarla
    pub fn set_require_confirmation(&mut self, required: bool) {
        self.require_confirmation = required;
    }
    
    /// Yetki özeti
    pub fn summary(&self) -> AuthSummary {
        AuthSummary {
            level: self.current_level,
            gui_enabled: self.can_control_gui(),
            file_enabled: self.can_access_files(),
            network_enabled: self.can_access_network(),
            system_enabled: self.can_execute_system(),
            safe_mode: self.safe_mode,
            granted_count: self.granted_permissions.len(),
        }
    }
}

impl Default for AuthManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Yetki özeti
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthSummary {
    pub level: PermissionLevel,
    pub gui_enabled: bool,
    pub file_enabled: bool,
    pub network_enabled: bool,
    pub system_enabled: bool,
    pub safe_mode: bool,
    pub granted_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_permission_levels() {
        let mut auth = AuthManager::new();
        
        // Varsayılan Level 2
        assert!(auth.can_access_files());
        assert!(!auth.can_control_gui());
        
        // Safe mode kapat (aksi takdirde level 3+ engellenir)
        auth.set_safe_mode(false);
        
        // Level 3'e geç
        assert!(auth.set_level(PermissionLevel::GuiControl));
        assert!(auth.can_control_gui());
        
        // Safe mode'da Level 4'e geçiş engellenmeli
        auth.set_safe_mode(true);
        assert!(!auth.set_level(PermissionLevel::FullAuto));
    }
    
    #[test]
    fn test_dangerous_actions() {
        let auth = AuthManager::new();
        
        assert!(auth.needs_confirmation("delete file"));
        assert!(auth.needs_confirmation("shutdown system"));
        assert!(!auth.needs_confirmation("read file"));
    }
}
