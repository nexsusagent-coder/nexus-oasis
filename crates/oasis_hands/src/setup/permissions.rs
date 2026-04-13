//! Permissions - İzin Yönetimi

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Permission {
    ScreenCapture,
    MouseControl,
    KeyboardControl,
    WindowManagement,
    FileAccess,
    ProcessSpawn,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PermissionStatus {
    Granted,
    Denied,
    NotRequested,
    Pending,
}

#[derive(Debug, Clone)]
pub struct PermissionManager {
    permissions: std::collections::HashMap<Permission, PermissionStatus>,
}

impl PermissionManager {
    pub fn new() -> Self {
        let mut perms = std::collections::HashMap::new();
        perms.insert(Permission::ScreenCapture, PermissionStatus::NotRequested);
        perms.insert(Permission::MouseControl, PermissionStatus::NotRequested);
        perms.insert(Permission::KeyboardControl, PermissionStatus::NotRequested);
        perms.insert(Permission::WindowManagement, PermissionStatus::NotRequested);
        perms.insert(Permission::FileAccess, PermissionStatus::NotRequested);
        perms.insert(Permission::ProcessSpawn, PermissionStatus::NotRequested);
        Self { permissions: perms }
    }
    
    pub fn grant(&mut self, perm: Permission) {
        self.permissions.insert(perm, PermissionStatus::Granted);
    }
    
    pub fn deny(&mut self, perm: Permission) {
        self.permissions.insert(perm, PermissionStatus::Denied);
    }
    
    pub fn status(&self, perm: Permission) -> PermissionStatus {
        self.permissions.get(&perm).copied().unwrap_or(PermissionStatus::NotRequested)
    }
    
    pub fn is_granted(&self, perm: Permission) -> bool {
        self.status(perm) == PermissionStatus::Granted
    }
    
    pub fn all_granted(&self) -> bool {
        self.permissions.values().all(|s| *s == PermissionStatus::Granted)
    }
}

impl Default for PermissionManager {
    fn default() -> Self { Self::new() }
}
