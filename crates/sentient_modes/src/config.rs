//! ─── MODE CONFIG ───
//!
//! Mod yapılandırması

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::ModeType;

/// Mod yapılandırması
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModeConfig {
    /// Varsayılan mod
    pub default_mode: ModeType,
    /// Mod ayarları
    pub mode_settings: HashMap<ModeType, ModeSettings>,
}

impl Default for ModeConfig {
    fn default() -> Self {
        Self {
            default_mode: ModeType::ReAct,
            mode_settings: HashMap::new(),
        }
    }
}

/// Mod ayarları
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModeSettings {
    /// Aktif mi?
    pub enabled: bool,
    /// Özel prompt
    pub custom_prompt: Option<String>,
    /// Ek parametreler
    pub extra_params: HashMap<String, serde_json::Value>,
}

impl Default for ModeSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            custom_prompt: None,
            extra_params: HashMap::new(),
        }
    }
}
