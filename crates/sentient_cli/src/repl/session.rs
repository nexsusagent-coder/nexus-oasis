//! ─── SESSION MODULU ───
//!
//! REPL oturum yonetimi ve durumu

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::collections::HashMap;

/// Aktif REPL oturumu
#[derive(Debug)]
pub struct ReplSession {
    /// Oturum ID
    pub id: Uuid,
    /// Baslangic zamani
    pub started_at: DateTime<Utc>,
    /// Mevcut mod
    pub mode: SessionMode,
    /// Aktif modul
    pub active_module: Option<String>,
    /// Kullanici tarafindan belirlenen degiskenler
    pub variables: HashMap<String, String>,
    /// Istatisikler
    pub stats: SessionStats,
    /// Debug modu
    pub debug: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum SessionMode {
    /// Normal interaktif mod
    Interactive,
    /// Swarm orkestrasyon modu
    Swarm,
    /// Debug/hata ayiklama modu
    Debug,
    /// Script modu (dosyadan okuma)
    Script,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SessionStats {
    /// Toplam komut sayisi
    pub commands_executed: u64,
    /// Basarili komutlar
    pub successful_commands: u64,
    /// Basarisiz komutlar
    pub failed_commands: u64,
    /// LLM sorgulari
    pub llm_queries: u64,
    /// Toplam token kullanimi
    pub total_tokens: u64,
    /// Toplam sure (ms)
    pub total_duration_ms: u64,
}

impl ReplSession {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            started_at: Utc::now(),
            mode: SessionMode::Interactive,
            active_module: None,
            variables: HashMap::new(),
            stats: SessionStats::default(),
            debug: false,
        }
    }

    /// Mod degistir
    pub fn set_mode(&mut self, mode: SessionMode) {
        self.mode = mode;
    }

    /// Modul gir
    pub fn enter_module(&mut self, module: &str) {
        self.active_module = Some(module.to_string());
    }

    /// Modul cik
    pub fn exit_module(&mut self) {
        self.active_module = None;
    }

    /// Degisken ayarla
    pub fn set_var(&mut self, key: &str, value: &str) {
        self.variables.insert(key.to_string(), value.to_string());
    }

    /// Degisken al
    pub fn get_var(&self, key: &str) -> Option<&str> {
        self.variables.get(key).map(|s| s.as_str())
    }

    /// Komut kaydi ekle
    pub fn record_command(&mut self, success: bool, duration_ms: u64, tokens: u64) {
        self.stats.commands_executed += 1;
        if success {
            self.stats.successful_commands += 1;
        } else {
            self.stats.failed_commands += 1;
        }
        self.stats.total_duration_ms += duration_ms;
        self.stats.total_tokens += tokens;
    }

    /// LLM sorgusu kaydi ekle
    pub fn record_llm_query(&mut self, tokens: u64) {
        self.stats.llm_queries += 1;
        self.stats.total_tokens += tokens;
    }

    /// Oturum suresi
    pub fn duration_secs(&self) -> i64 {
        (Utc::now() - self.started_at).num_seconds()
    }

    /// Oturum raporu
    pub fn report(&self) -> SessionReport {
        SessionReport {
            session_id: self.id,
            duration_secs: self.duration_secs(),
            mode: self.mode,
            stats: self.stats.clone(),
        }
    }

    /// Debug modunu ayarla
    pub fn set_debug(&mut self, debug: bool) {
        self.debug = debug;
    }
}

impl Default for ReplSession {
    fn default() -> Self {
        Self::new()
    }
}

/// Oturum raporu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionReport {
    pub session_id: Uuid,
    pub duration_secs: i64,
    pub mode: SessionMode,
    pub stats: SessionStats,
}

impl std::fmt::Display for SessionReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let success_rate = if self.stats.commands_executed > 0 {
            (self.stats.successful_commands as f64 / self.stats.commands_executed as f64 * 100.0) as u32
        } else {
            100
        };

        write!(
            f,
            r#"
════════════════════════════════════════════════════════════
  📊 OTURUM RAPORU
════════════════════════════════════════════════════════════
  Oturum ID     : {}
  Sure          : {} saniye
  Mod           : {:?}
────────────────────────────────────────────────────────────
  Komutlar      : {} toplam
                 : {} basarili / {} basarisiz (%{} basari)
  LLM Sorgulari : {}
  Token Kullanimi: {}
  Ort. Sure     : {} ms/komut
════════════════════════════════════════════════════════════
"#,
            self.session_id,
            self.duration_secs,
            self.mode,
            self.stats.commands_executed,
            self.stats.successful_commands,
            self.stats.failed_commands,
            success_rate,
            self.stats.llm_queries,
            self.stats.total_tokens,
            if self.stats.commands_executed > 0 {
                self.stats.total_duration_ms / self.stats.commands_executed
            } else {
                0
            }
        )
    }
}

/// Kullanici tercihleri
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    /// Varsayilan model
    pub default_model: String,
    /// Varsayilan vgate URL
    pub vgate_url: String,
    /// Otomatik kayit
    pub auto_save: bool,
    /// Renkli cikti
    pub color_output: bool,
    /// Komut onerileri
    pub show_suggestions: bool,
    /// Detayli log
    pub verbose: bool,
}

impl Default for UserPreferences {
    fn default() -> Self {
        Self {
            default_model: "qwen/qwen3-1.7b:free".into(),
            vgate_url: "http://127.0.0.1:1071".into(),
            auto_save: true,
            color_output: true,
            show_suggestions: true,
            verbose: false,
        }
    }
}

impl UserPreferences {
    /// Dosyadan yukle
    pub fn load() -> Self {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
        let config_path = std::path::PathBuf::from(home).join(".sentient_config.json");

        if config_path.exists() {
            if let Ok(content) = std::fs::read_to_string(&config_path) {
                if let Ok(prefs) = serde_json::from_str(&content) {
                    return prefs;
                }
            }
        }

        Self::default()
    }

    /// Dosyaya kaydet
    pub fn save(&self) {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
        let config_path = std::path::PathBuf::from(home).join(".sentient_config.json");

        if let Ok(json) = serde_json::to_string_pretty(self) {
            let _ = std::fs::write(&config_path, json);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_creation() {
        let session = ReplSession::new();
        assert!(!session.debug);
        assert!(session.active_module.is_none());
    }

    #[test]
    fn test_session_variables() {
        let mut session = ReplSession::new();
        session.set_var("model", "qwen/test");
        assert_eq!(session.get_var("model"), Some("qwen/test"));
    }

    #[test]
    fn test_session_stats() {
        let mut session = ReplSession::new();
        session.record_command(true, 100, 50);
        session.record_command(false, 200, 100);
        
        assert_eq!(session.stats.commands_executed, 2);
        assert_eq!(session.stats.successful_commands, 1);
        assert_eq!(session.stats.failed_commands, 1);
    }

    #[test]
    fn test_session_module_navigation() {
        let mut session = ReplSession::new();
        session.enter_module("memory");
        assert_eq!(session.active_module, Some("memory".to_string()));
        session.exit_module();
        assert!(session.active_module.is_none());
    }
}
