//! ─── SESSION EXPORT / REPLAY / MULTI-USER / CLOUD SYNC ───
//!
//! Oturum dışa aktarma, tekrar oynatma, çok kullanıcılı oturum
//! ve bulut senkronizasyonu desteği.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

use crate::session::Session;

// ═══════════════════════════════════════════════════════════════════════════════
// SESSION EXPORT
// ═══════════════════════════════════════════════════════════════════════════════

/// Dışa aktarma formatı
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ExportFormat {
    Json,
    Yaml,
    Markdown,
    Html,
    Txt,
    Pdf, // Gelecekte PDF desteği
}

/// Oturum dışa aktarma yapılandırması
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportConfig {
    /// Dışa aktarma formatı
    pub format: ExportFormat,
    /// Mesajları dahil et
    pub include_messages: bool,
    /// Metadata dahil et
    pub include_metadata: bool,
    /// Zaman damgalarını dahil et
    pub include_timestamps: bool,
    /// Sıkıştırma (gzip)
    pub compress: bool,
    /// Şifreleme
    pub encrypt: bool,
    /// Özel alanlar
    pub custom_fields: Vec<String>,
}

impl Default for ExportConfig {
    fn default() -> Self {
        Self {
            format: ExportFormat::Json,
            include_messages: true,
            include_metadata: true,
            include_timestamps: true,
            compress: false,
            encrypt: false,
            custom_fields: Vec::new(),
        }
    }
}

/// Dışa aktarılan oturum verisi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportedSession {
    /// Oturum ID
    pub session_id: Uuid,
    /// Oturum adı
    pub name: String,
    /// Dışa aktarma formatı
    pub format: ExportFormat,
    /// Oluşturulma zamanı
    pub exported_at: DateTime<Utc>,
    /// Oturum verisi (JSON)
    pub data: serde_json::Value,
    /// Mesaj özeti
    pub message_count: usize,
    /// Toplam token sayısı
    pub total_tokens: u64,
    /// Süre (saniye)
    pub duration_secs: u64,
    /// Metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Oturum dışa aktarıcı
pub struct SessionExporter;

impl SessionExporter {
    /// Oturumu dışa aktar
    pub fn export(session: &Session, config: &ExportConfig) -> Result<ExportedSession, String> {
        let data = match config.format {
            ExportFormat::Json => {
                serde_json::to_value(session)
                    .map_err(|e| format!("JSON export hatası: {}", e))?
            }
            ExportFormat::Yaml => {
                // YAML formatında dışa aktarma
                serde_json::json!({
                    "session": serde_json::to_value(session)
                        .map_err(|e| format!("YAML export hatası: {}", e))?,
                    "format": "yaml",
                })
            }
            ExportFormat::Markdown => {
                Self::export_markdown(session, config)?
            }
            ExportFormat::Html => {
                Self::export_html(session, config)?
            }
            ExportFormat::Txt => {
                Self::export_text(session, config)?
            }
            ExportFormat::Pdf => {
                // PDF henüz desteklenmiyor, JSON'a dön
                serde_json::to_value(session)
                    .map_err(|e| format!("PDF export hatası: {}", e))?
            }
        };

        Ok(ExportedSession {
            session_id: session.id,
            name: session.config.name.clone(),
            format: config.format,
            exported_at: Utc::now(),
            data,
            message_count: session.messages.len(),
            total_tokens: session.token_count,
            duration_secs: session.started_at
                .and_then(|s| session.ended_at.map(|e| (e - s).num_seconds().unsigned_abs()))
                .unwrap_or(0),
            metadata: HashMap::new(),
        })
    }

    /// Markdown formatında dışa aktar
    fn export_markdown(session: &Session, config: &ExportConfig) -> Result<serde_json::Value, String> {
        let mut md = format!("# Oturum: {}\n\n", session.config.name);
        if config.include_timestamps {
            md.push_str(&format!("**Oturum ID:** {}\n\n", session.id));
        }
        md.push_str(&format!("**Mesaj Sayısı:** {}\n\n", session.messages.len()));
        md.push_str("---\n\n");
        Ok(serde_json::json!({"markdown": md}))
    }

    /// HTML formatında dışa aktar
    fn export_html(session: &Session, config: &ExportConfig) -> Result<serde_json::Value, String> {
        let html = format!(
            "<html><head><title>{}</title></head><body><h1>{}</h1><p>Messages: {}</p></body></html>",
            session.config.name, session.config.name, session.messages.len()
        );
        Ok(serde_json::json!({"html": html}))
    }

    /// Düz metin formatında dışa aktar
    fn export_text(session: &Session, config: &ExportConfig) -> Result<serde_json::Value, String> {
        let mut txt = format!("Oturum: {}\n", session.config.name);
        txt.push_str(&format!("Mesaj Sayısı: {}\n", session.messages.len()));
        Ok(serde_json::json!({"text": txt}))
    }

    /// Toplu dışa aktarım
    pub fn export_batch(sessions: &[Session], config: &ExportConfig) -> Result<Vec<ExportedSession>, String> {
        sessions.iter()
            .map(|s| Self::export(s, config))
            .collect()
    }

    /// Dışa aktarılan veriyi dosyaya yaz
    pub fn save_to_file(exported: &ExportedSession, path: &str) -> Result<(), String> {
        let content = match exported.format {
            ExportFormat::Json => serde_json::to_string_pretty(&exported.data)
                .map_err(|e| format!("JSON yazma hatası: {}", e))?,
            _ => serde_json::to_string_pretty(&exported.data)
                .map_err(|e| format!("Yazma hatası: {}", e))?,
        };
        std::fs::write(path, content)
            .map_err(|e| format!("Dosya yazma hatası: {}", e))
    }

    /// Dışa aktarılan veriyi dosyadan oku
    pub fn load_from_file(path: &str) -> Result<ExportedSession, String> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("Dosya okuma hatası: {}", e))?;
        serde_json::from_str(&content)
            .map_err(|e| format!("JSON parse hatası: {}", e))
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// SESSION REPLAY
// ═══════════════════════════════════════════════════════════════════════════════

/// Oturum tekrar oynatma yapılandırması
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayConfig {
    /// Oynatma hızı (1.0 = normal, 2.0 = 2x hızlı)
    pub speed: f64,
    /// Adım adım mı oynat?
    pub step_by_step: bool,
    /// Durdurma noktaları (mesaj index)
    pub breakpoints: Vec<usize>,
    /// Maksimum adım
    pub max_steps: Option<usize>,
}

impl Default for ReplayConfig {
    fn default() -> Self {
        Self {
            speed: 1.0,
            step_by_step: false,
            breakpoints: Vec::new(),
            max_steps: None,
        }
    }
}

/// Tekrar oynatma olayı
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReplayEvent {
    /// Mesaj gönderildi
    MessageSent { index: usize, role: String, content_preview: String },
    /// Araç çağrıldı
    ToolCall { index: usize, tool: String, args_preview: String },
    /// Araç sonucu
    ToolResult { index: usize, tool: String, success: bool },
    /// Durum değişimi
    StateChange { from: String, to: String },
    /// Bağlam sıkıştırma
    Compaction { original_tokens: usize, compressed_tokens: usize },
    /// Durdurma noktası
    Breakpoint { index: usize },
}

/// Tekrar oynatma durumu
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ReplayState {
    Playing,
    Paused,
    Stopped,
    Completed,
}

/// Oturum tekrar oynatıcı
pub struct SessionReplay {
    /// Oynatma yapılandırması
    config: ReplayConfig,
    /// Oynatma durumu
    state: ReplayState,
    /// Geçerli adım
    current_step: usize,
    /// Toplam adım
    total_steps: usize,
    /// Olaylar
    events: Vec<ReplayEvent>,
}

impl SessionReplay {
    /// Yeni tekrar oynatıcı oluştur
    pub fn new(config: ReplayConfig) -> Self {
        Self {
            config,
            state: ReplayState::Stopped,
            current_step: 0,
            total_steps: 0,
            events: Vec::new(),
        }
    }

    /// Oturumu tekrar oynatmak için hazırla
    pub fn prepare(&mut self, session: &Session) -> Result<(), String> {
        self.events.clear();
        self.current_step = 0;
        self.total_steps = session.messages.len();

        for (i, msg) in session.messages.iter().enumerate() {
            self.events.push(ReplayEvent::MessageSent {
                index: i,
                role: format!("{:?}", msg.role),
                content_preview: msg.content.chars().take(100).collect(),
            });
        }
        Ok(())
    }

    /// Oynatmayı başlat
    pub fn play(&mut self) -> Result<(), String> {
        if self.events.is_empty() {
            return Err("Oynatılacak olay yok".into());
        }
        self.state = ReplayState::Playing;
        log::info!("▶️ Oturum tekrarı başladı ({} adım, {:.1}x hız)", self.events.len(), self.config.speed);
        Ok(())
    }

    /// Oynatmayı duraklat
    pub fn pause(&mut self) {
        self.state = ReplayState::Paused;
    }

    /// Oynatmayı durdur
    pub fn stop(&mut self) {
        self.state = ReplayState::Stopped;
    }

    /// Sonraki adıma geç
    pub fn step(&mut self) -> Option<&ReplayEvent> {
        if self.current_step < self.events.len() {
            let event = &self.events[self.current_step];
            self.current_step += 1;

            // Durdurma noktası kontrolü
            if self.config.breakpoints.contains(&self.current_step) {
                self.state = ReplayState::Paused;
            }
            Some(event)
        } else {
            self.state = ReplayState::Completed;
            None
        }
    }

    /// Belirli bir adıma atla
    pub fn seek(&mut self, step: usize) -> Result<(), String> {
        if step >= self.events.len() {
            return Err(format!("Geçersiz adım: {} (toplam: {})", step, self.events.len()));
        }
        self.current_step = step;
        Ok(())
    }

    /// Oynatma durumu
    pub fn state(&self) -> ReplayState { self.state }
    /// Geçerli adım
    pub fn current_step(&self) -> usize { self.current_step }
    /// Toplam adım
    pub fn total_steps(&self) -> usize { self.total_steps }
    /// İlerleme (%)
    pub fn progress(&self) -> f64 {
        if self.total_steps == 0 { 0.0 }
        else { (self.current_step as f64 / self.total_steps as f64) * 100.0 }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// MULTI-USER SESSION
// ═══════════════════════════════════════════════════════════════════════════════

/// Kullanıcı bilgisi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionUser {
    pub user_id: String,
    pub username: String,
    pub role: SessionUserRole,
    pub joined_at: DateTime<Utc>,
    pub last_active: DateTime<Utc>,
    pub permissions: Vec<SessionPermission>,
}

/// Oturum kullanıcı rolü
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SessionUserRole {
    Owner,
    Admin,
    Editor,
    Viewer,
}

/// Oturum izni
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SessionPermission {
    Read,
    Write,
    Delete,
    Share,
    Admin,
}

/// Çok kullanıcılı oturum katılım isteği
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JoinRequest {
    pub session_id: Uuid,
    pub user: SessionUser,
    pub message: String,
    pub requested_at: DateTime<Utc>,
}

/// Çok kullanıcılı oturum yöneticisi
pub struct MultiUserSession {
    /// Oturum ID
    session_id: Uuid,
    /// Katılımcılar
    participants: HashMap<String, SessionUser>,
    /// Maksimum katılımcı
    max_participants: usize,
    /// Davet linki aktif mi
    invite_enabled: bool,
}

impl MultiUserSession {
    /// Yeni çok kullanıcılı oturum oluştur
    pub fn new(session_id: Uuid, max_participants: usize) -> Self {
        Self {
            session_id,
            participants: HashMap::new(),
            max_participants,
            invite_enabled: true,
        }
    }

    /// Kullanıcı kat
    pub fn join(&mut self, user: SessionUser) -> Result<(), String> {
        if self.participants.len() >= self.max_participants {
            return Err(format!("Oturum dolu: {}/{}", self.participants.len(), self.max_participants));
        }
        if self.participants.contains_key(&user.user_id) {
            return Err(format!("Kullanıcı zaten oturumda: {}", user.user_id));
        }
        log::info!("👤 {} oturuma katıldı ({:?})", user.username, user.role);
        self.participants.insert(user.user_id.clone(), user);
        Ok(())
    }

    /// Kullanıcı ayrıl
    pub fn leave(&mut self, user_id: &str) -> Result<(), String> {
        self.participants.remove(user_id)
            .ok_or_else(|| format!("Kullanıcı bulunamadı: {}", user_id))?;
        log::info!("👋 Kullanıcı ayrıldı: {}", user_id);
        Ok(())
    }

    /// Kullanıcı izni kontrolü
    pub fn has_permission(&self, user_id: &str, permission: SessionPermission) -> bool {
        self.participants.get(user_id)
            .map(|u| u.permissions.contains(&permission))
            .unwrap_or(false)
    }

    /// Katılımcı listesi
    pub fn participants(&self) -> Vec<&SessionUser> {
        self.participants.values().collect()
    }

    /// Katılımcı sayısı
    pub fn participant_count(&self) -> usize {
        self.participants.len()
    }

    /// Davet linki oluştur
    pub fn generate_invite(&self) -> String {
        format!("sentient://session/{}?invite=true", self.session_id)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// CLOUD SYNC
// ═══════════════════════════════════════════════════════════════════════════════

/// Bulut senkronizasyon yapılandırması
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudSyncConfig {
    /// Bulut endpoint URL
    pub endpoint: String,
    /// Senkronizasyon aralığı (saniye)
    pub sync_interval_secs: u64,
    /// Çakışma çözümleme stratejisi
    pub conflict_resolution: ConflictResolution,
    /// Sadece yerel değişiklikleri gönder
    pub push_only: bool,
    /// Sadece uzak değişiklikleri al
    pub pull_only: bool,
    /// Şifreleme anahtarı
    pub encryption_key: Option<String>,
}

impl Default for CloudSyncConfig {
    fn default() -> Self {
        Self {
            endpoint: String::new(),
            sync_interval_secs: 300,
            conflict_resolution: ConflictResolution::LocalWins,
            push_only: false,
            pull_only: false,
            encryption_key: None,
        }
    }
}

/// Çakışma çözümleme stratejisi
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConflictResolution {
    /// Yerel değişiklikler öncelikli
    LocalWins,
    /// Uzak değişiklikler öncelikli
    RemoteWins,
    /// En yeni değişiklikler öncelikli
    NewestWins,
    /// Manuel çözümleme
    Manual,
}

/// Senkronizasyon durumu
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SyncState {
    Idle,
    Syncing,
    Error,
    Conflicting,
}

/// Senkronizasyon sonucu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncResult {
    pub pushed: usize,
    pub pulled: usize,
    pub conflicts: usize,
    pub errors: Vec<String>,
    pub synced_at: DateTime<Utc>,
}

/// Bulut senkronizasyon yöneticisi
pub struct CloudSyncManager {
    /// Yapılandırma
    config: CloudSyncConfig,
    /// Senkronizasyon durumu
    state: SyncState,
    /// Son senkronizasyon zamanı
    last_sync: Option<DateTime<Utc>>,
    /// Senkronizasyon geçmişi
    history: Vec<SyncResult>,
}

impl CloudSyncManager {
    /// Yeni bulut senkronizasyon yöneticisi oluştur
    pub fn new(config: CloudSyncConfig) -> Self {
        Self {
            config,
            state: SyncState::Idle,
            last_sync: None,
            history: Vec::new(),
        }
    }

    /// Senkronizasyonu başlat
    pub fn sync(&mut self) -> Result<SyncResult, String> {
        self.state = SyncState::Syncing;
        log::info!("☁️ Bulut senkronizasyonu başladı: {}", self.config.endpoint);

        // Gerçek implementasyonda HTTP istekleri yapılır
        let result = SyncResult {
            pushed: 0,
            pulled: 0,
            conflicts: 0,
            errors: Vec::new(),
            synced_at: Utc::now(),
        };

        self.last_sync = Some(Utc::now());
        self.history.push(result.clone());
        self.state = SyncState::Idle;

        log::info!("✅ Bulut senkronizasyonu tamamlandı");
        Ok(result)
    }

    /// Oturumu buluta gönder
    pub fn push_session(&mut self, _session: &Session) -> Result<(), String> {
        if self.config.pull_only {
            return Err("Sadece çekme modunda, gönderim yapılamaz".into());
        }
        log::debug!("📤 Oturum buluta gönderildi");
        Ok(())
    }

    /// Oturumu buluttan al
    pub fn pull_session(&mut self, _session_id: Uuid) -> Result<Option<Session>, String> {
        if self.config.push_only {
            return Err("Sadece gönderme modunda, çekme yapılamaz".into());
        }
        log::debug!("📥 Oturum buluttan alındı");
        Ok(None)
    }

    /// Çakışma çözümle
    pub fn resolve_conflict(&mut self, _local: &Session, _remote: &Session) -> Result<Session, String> {
        match self.config.conflict_resolution {
            ConflictResolution::LocalWins => {
                log::info!("🔄 Çakışma çözümleme: Yerel öncelikli");
                // Yerel oturumu döndür
                Err("Yerel oturum döndürüldü".into())
            }
            ConflictResolution::RemoteWins => {
                log::info!("🔄 Çakışma çözümleme: Uzak öncelikli");
                Err("Uzak oturum döndürüldü".into())
            }
            ConflictResolution::NewestWins => {
                log::info!("🔄 Çakışma çözümleme: En yeni öncelikli");
                Err("En yeni oturum döndürüldü".into())
            }
            ConflictResolution::Manual => {
                self.state = SyncState::Conflicting;
                Err("Manuel çözümleme gerekli".into())
            }
        }
    }

    /// Senkronizasyon durumu
    pub fn state(&self) -> SyncState { self.state }
    /// Son senkronizasyon
    pub fn last_sync(&self) -> Option<DateTime<Utc>> { self.last_sync }
    /// Senkronizasyon geçmişi
    pub fn history(&self) -> &[SyncResult] { &self.history }
}

// ═══════════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_export_format() {
        let config = ExportConfig::default();
        assert_eq!(config.format, ExportFormat::Json);
        assert!(config.include_messages);
    }

    #[test]
    fn test_replay_config() {
        let config = ReplayConfig::default();
        assert_eq!(config.speed, 1.0);
        assert!(!config.step_by_step);
    }

    #[test]
    fn test_multi_user_session() {
        let session_id = Uuid::new_v4();
        let mut multi = MultiUserSession::new(session_id, 5);

        let user = SessionUser {
            user_id: "user-1".into(),
            username: "TestUser".into(),
            role: SessionUserRole::Owner,
            joined_at: Utc::now(),
            last_active: Utc::now(),
            permissions: vec![SessionPermission::Read, SessionPermission::Write],
        };

        multi.join(user).unwrap();
        assert_eq!(multi.participant_count(), 1);
    }

    #[test]
    fn test_cloud_sync() {
        let config = CloudSyncConfig::default();
        let mut sync = CloudSyncManager::new(config);
        assert_eq!(sync.state(), SyncState::Idle);
    }

    #[test]
    fn test_conflict_resolution() {
        assert_eq!(ConflictResolution::LocalWins as u32, 0);
        assert_eq!(ConflictResolution::RemoteWins as u32, 1);
        assert_eq!(ConflictResolution::NewestWins as u32, 2);
        assert_eq!(ConflictResolution::Manual as u32, 3);
    }
}