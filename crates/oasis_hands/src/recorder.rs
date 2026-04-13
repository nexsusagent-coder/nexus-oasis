//! ═══════════════════════════════════════════════════════════════════════════════
//!  ACTION RECORDING - AKSİYON KAYIT VE TEKRAR SİSTEMİ
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Masaüstü kontrolü için makro kayıt ve oynatma sistemi.
//! 
//! ═──────────────────────────────────────────────────────────────────────────────
//!  ÖZELLİKLER:
//!  ────────────────
//!  Record          → Aksiyonları kaydet
//!  Playback        → Kayıtları oynat
//!  Edit            → Kayıtları düzenle
//!  Export/Import   → Dosya kayıt/yükleme
//!  Speed Control   → Oynatma hızı
//! ═──────────────────────────────────────────────────────────────────────────────

use crate::error::{HandsError, HandsResult};
use crate::history::UndoableActionType;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::time::{Duration, Instant};

// ───────────────────────────────────────────────────────────────────────────────
//  ZAMANLI AKSİYON
// ───────────────────────────────────────────────────────────────────────────────

/// Zamanlanmış aksiyon
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimedAction {
    /// Aksiyon türü
    pub action_type: UndoableActionType,
    /// Aksiyon parametreleri (JSON)
    pub params: String,
    /// Kayıt başlangıcından itibaren geçen süre (ms)
    pub delay_ms: u64,
    /// Açıklama
    pub description: String,
}

impl TimedAction {
    /// Yeni zamanlı aksiyon
    pub fn new(action_type: UndoableActionType, params: &str, delay_ms: u64, description: &str) -> Self {
        Self {
            action_type,
            params: params.to_string(),
            delay_ms,
            description: description.to_string(),
        }
    }
    
    /// Gecikme süresi (Duration)
    pub fn delay(&self) -> Duration {
        Duration::from_millis(self.delay_ms)
    }
}

// ───────────────────────────────────────────────────────────────────────────────
//  MAKRO
// ───────────────────────────────────────────────────────────────────────────────

/// Kayıtlı makro
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Macro {
    /// Makro ID
    pub id: u64,
    /// Makro adı
    pub name: String,
    /// Açıklama
    pub description: String,
    /// Kayıtlı aksiyonlar
    pub actions: Vec<TimedAction>,
    /// Oluşturulma zamanı
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Son kullanım
    pub last_used: Option<chrono::DateTime<chrono::Utc>>,
    /// Kullanım sayısı
    pub use_count: u64,
    /// Etiketler
    pub tags: Vec<String>,
    /// Toplam süre (ms)
    pub total_duration_ms: u64,
}

impl Macro {
    /// Yeni makro
    pub fn new(id: u64, name: &str, actions: Vec<TimedAction>) -> Self {
        let total_duration_ms = actions.last().map(|a| a.delay_ms).unwrap_or(0);
        
        Self {
            id,
            name: name.to_string(),
            description: String::new(),
            actions,
            created_at: chrono::Utc::now(),
            last_used: None,
            use_count: 0,
            tags: Vec::new(),
            total_duration_ms,
        }
    }
    
    /// Açıklama ile
    pub fn with_description(mut self, desc: &str) -> Self {
        self.description = desc.to_string();
        self
    }
    
    /// Etiket ekle
    pub fn add_tag(&mut self, tag: &str) {
        if !self.tags.contains(&tag.to_string()) {
            self.tags.push(tag.to_string());
        }
    }
    
    /// Kullanıldı işaretle
    pub fn mark_used(&mut self) {
        self.last_used = Some(chrono::Utc::now());
        self.use_count += 1;
    }
    
    /// Aksiyon sayısı
    pub fn action_count(&self) -> usize {
        self.actions.len()
    }
    
    /// Toplam süre
    pub fn total_duration(&self) -> Duration {
        Duration::from_millis(self.total_duration_ms)
    }
    
    /// Süre formatlı
    pub fn formatted_duration(&self) -> String {
        let secs = self.total_duration_ms / 1000;
        let ms = self.total_duration_ms % 1000;
        if secs > 60 {
            let mins = secs / 60;
            let secs = secs % 60;
            format!("{}m {}s {}ms", mins, secs, ms)
        } else if secs > 0 {
            format!("{}s {}ms", secs, ms)
        } else {
            format!("{}ms", ms)
        }
    }
    
    /// Özet
    pub fn summary(&self) -> MacroSummary {
        let mut mouse_count = 0;
        let mut keyboard_count = 0;
        let mut other_count = 0;
        
        for action in &self.actions {
            match action.action_type {
                UndoableActionType::MouseMove
                | UndoableActionType::MouseClick
                | UndoableActionType::MouseDrag
                | UndoableActionType::MouseScroll => mouse_count += 1,
                UndoableActionType::KeyPress
                | UndoableActionType::TypeText
                | UndoableActionType::Shortcut => keyboard_count += 1,
                _ => other_count += 1,
            }
        }
        
        MacroSummary {
            id: self.id,
            name: self.name.clone(),
            action_count: self.actions.len(),
            total_duration_ms: self.total_duration_ms,
            mouse_count,
            keyboard_count,
            other_count,
            use_count: self.use_count,
        }
    }
}

/// Makro özeti
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacroSummary {
    pub id: u64,
    pub name: String,
    pub action_count: usize,
    pub total_duration_ms: u64,
    pub mouse_count: u64,
    pub keyboard_count: u64,
    pub other_count: u64,
    pub use_count: u64,
}

// ───────────────────────────────────────────────────────────────────────────────
//  MAKRO KAYDEDİCİ YAPILANDIRMASI
// ───────────────────────────────────────────────────────────────────────────────

/// Kaydedici yapılandırması
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecorderConfig {
    /// Maksimum kayıt süresi (saniye)
    pub max_duration_secs: u64,
    /// Maksimum aksiyon sayısı
    pub max_actions: usize,
    /// Minimum gecikme (ms) - çok hızlı aksiyonları yavaşlat
    pub min_delay_ms: u64,
    /// Otomatik kaydetme
    pub auto_save: bool,
    /// Kayıt dosyası
    pub save_file: Option<String>,
    /// Gürültü filtresi (küçük hareketleri yok say)
    pub noise_filter: bool,
    /// Gürültü eşiği (piksel)
    pub noise_threshold: f64,
}

impl Default for RecorderConfig {
    fn default() -> Self {
        Self {
            max_duration_secs: 300, // 5 dakika
            max_actions: 1000,
            min_delay_ms: 10,
            auto_save: false,
            save_file: None,
            noise_filter: true,
            noise_threshold: 3.0,
        }
    }
}

impl RecorderConfig {
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Uzun kayıt
    pub fn long() -> Self {
        Self {
            max_duration_secs: 1800, // 30 dakika
            max_actions: 5000,
            ..Default::default()
        }
    }
    
    /// Hassas kayıt
    pub fn precise() -> Self {
        Self {
            noise_filter: false,
            min_delay_ms: 1,
            ..Default::default()
        }
    }
}

// ───────────────────────────────────────────────────────────────────────────────
//  OYNATMA AYARLARI
// ───────────────────────────────────────────────────────────────────────────────

/// Oynatma ayarları
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaybackSettings {
    /// Hız çarpanı (1.0 = normal, 2.0 = 2x hızlı)
    pub speed: f64,
    /// Döngü sayısı (0 = sonsuz)
    pub loop_count: u32,
    /// Döngüler arası bekleme (ms)
    pub loop_delay_ms: u64,
    /// Duraklatma noktaları (aksiyon indeksleri)
    pub breakpoints: Vec<usize>,
    /// Hata durumunda dur
    pub stop_on_error: bool,
    /// Gerçekçi rastgele gecikme ekle
    pub humanize: bool,
    /// Rastgele gecikme aralığı (±ms)
    pub humanize_variance_ms: u64,
}

impl Default for PlaybackSettings {
    fn default() -> Self {
        Self {
            speed: 1.0,
            loop_count: 1,
            loop_delay_ms: 1000,
            breakpoints: Vec::new(),
            stop_on_error: true,
            humanize: false,
            humanize_variance_ms: 50,
        }
    }
}

impl PlaybackSettings {
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Hızlı oynatma
    pub fn fast() -> Self {
        Self {
            speed: 2.0,
            ..Default::default()
        }
    }
    
    /// Yavaş oynatma
    pub fn slow() -> Self {
        Self {
            speed: 0.5,
            ..Default::default()
        }
    }
    
    /// İnsan gibi
    pub fn humanized() -> Self {
        Self {
            humanize: true,
            humanize_variance_ms: 100,
            ..Default::default()
        }
    }
    
    /// Sonsuz döngü
    pub fn infinite_loop() -> Self {
        Self {
            loop_count: 0,
            ..Default::default()
        }
    }
    
    /// Hız ayarla
    pub fn with_speed(mut self, speed: f64) -> Self {
        self.speed = speed.clamp(0.1, 10.0);
        self
    }
}

// ───────────────────────────────────────────────────────────────────────────────
//  KAYIT DURUMU
// ───────────────────────────────────────────────────────────────────────────────

/// Kayıt durumu
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecordingState {
    /// Boşta
    Idle,
    /// Kaydediyor
    Recording,
    /// Duraklatılmış
    Paused,
    /// Oynatıyor
    Playing,
    /// Oynatma duraklatılmış
    PlaybackPaused,
}

impl RecordingState {
    pub fn description(&self) -> &str {
        match self {
            RecordingState::Idle => "Boşta",
            RecordingState::Recording => "Kaydediyor",
            RecordingState::Paused => "Duraklatılmış",
            RecordingState::Playing => "Oynatıyor",
            RecordingState::PlaybackPaused => "Oynatma Duraklatılmış",
        }
    }
    
    pub fn icon(&self) -> &str {
        match self {
            RecordingState::Idle => "⏹️",
            RecordingState::Recording => "⏺️",
            RecordingState::Paused => "⏸️",
            RecordingState::Playing => "▶️",
            RecordingState::PlaybackPaused => "⏯️",
        }
    }
}

// ───────────────────────────────────────────────────────────────────────────────
//  OYNATMA SONUCU
// ───────────────────────────────────────────────────────────────────────────────

/// Oynatma sonucu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaybackResult {
    /// Başarılı mı?
    pub success: bool,
    /// Oynatılan aksiyon sayısı
    pub actions_played: usize,
    /// Toplam süre (ms)
    pub duration_ms: u64,
    /// Döngü sayısı
    pub loops_completed: u32,
    /// Hata mesajı (varsa)
    pub error: Option<String>,
    /// Durdurulma nedeni
    pub stop_reason: Option<StopReason>,
}

/// Durdurma nedeni
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StopReason {
    /// Normal tamamlandı
    Completed,
    /// Kullanıcı durdurdu
    UserStopped,
    /// Breakpoint'e ulaşıldı
    Breakpoint,
    /// Hata oluştu
    Error,
    /// Emergency stop
    EmergencyStop,
}

// ───────────────────────────────────────────────────────────────────────────────
//  MAKRO KAYDEDİCİ
// ───────────────────────────────────────────────────────────────────────────────

/// Makro kaydedici
pub struct MacroRecorder {
    /// Yapılandırma
    config: RecorderConfig,
    /// Kayıt durumu
    state: RecordingState,
    /// Kayıtlı aksiyonlar
    actions: Vec<TimedAction>,
    /// Kayıt başlangıç zamanı
    start_time: Option<Instant>,
    /// Toplam kayıt süresi (duraklamalar hariç)
    recorded_duration: Duration,
    /// Son aksiyon zamanı
    last_action_time: Option<Instant>,
    /// Son mouse pozisyonu (gürültü filtresi için)
    last_mouse_pos: Option<(f64, f64)>,
    /// Kaydedilmiş makrolar
    macros: Vec<Macro>,
    /// ID sayacı
    next_id: u64,
    /// İstatistikler
    stats: RecorderStats,
}

/// Kaydedici istatistikleri
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecorderStats {
    pub total_macros: u64,
    pub total_recordings: u64,
    pub total_playbacks: u64,
    pub total_actions_recorded: u64,
    pub total_actions_played: u64,
}

impl MacroRecorder {
    /// Yeni kaydedici
    pub fn new(config: RecorderConfig) -> Self {
        Self {
            config,
            state: RecordingState::Idle,
            actions: Vec::new(),
            start_time: None,
            recorded_duration: Duration::ZERO,
            last_action_time: None,
            last_mouse_pos: None,
            macros: Vec::new(),
            next_id: 1,
            stats: RecorderStats::default(),
        }
    }
    
    /// Varsayılan
    pub fn default_config() -> Self {
        Self::new(RecorderConfig::default())
    }
    
    // ─── KAYIT ───
    
    /// Kayda başla
    pub fn start_recording(&mut self) -> HandsResult<()> {
        if self.state != RecordingState::Idle {
            return Err(HandsError::RecordingError("Zaten kayıt yapılıyor".to_string()));
        }
        
        self.actions.clear();
        self.start_time = Some(Instant::now());
        self.last_action_time = Some(Instant::now());
        self.recorded_duration = Duration::ZERO;
        self.last_mouse_pos = None;
        self.state = RecordingState::Recording;
        
        log::info!("⏺️ RECORDING: Kayıt başladı");
        
        Ok(())
    }
    
    /// Aksiyon kaydet
    pub fn record_action(
        &mut self,
        action_type: UndoableActionType,
        params: &str,
        description: &str,
    ) -> HandsResult<()> {
        if self.state != RecordingState::Recording {
            return Err(HandsError::RecordingError("Kayıt modunda değil".to_string()));
        }
        
        // Gürültü filtresi (mouse hareketleri için)
        if self.config.noise_filter && action_type == UndoableActionType::MouseMove {
            if let Some(params_value) = serde_json::from_str::<serde_json::Value>(params).ok() {
                if let (Some(x), Some(y)) = (params_value["x"].as_f64(), params_value["y"].as_f64()) {
                    if let Some((last_x, last_y)) = self.last_mouse_pos {
                        let dist = ((x - last_x).powi(2) + (y - last_y).powi(2)).sqrt();
                        if dist < self.config.noise_threshold {
                            return Ok(()); // Gürültü, yok say
                        }
                    }
                    self.last_mouse_pos = Some((x, y));
                }
            }
        }
        
        // Gecikme hesapla
        let now = Instant::now();
        let delay_ms = if let Some(last) = self.last_action_time {
            let elapsed = now.duration_since(last).as_millis() as u64;
            elapsed.max(self.config.min_delay_ms)
        } else {
            0
        };
        
        // Aksiyon ekle
        let action = TimedAction::new(action_type, params, delay_ms, description);
        self.actions.push(action);
        
        // Limit kontrolü
        if self.actions.len() >= self.config.max_actions {
            log::warn!("⏺️ RECORDING: Maksimum aksiyon sayısına ulaşıldı");
        }
        
        self.last_action_time = Some(now);
        self.stats.total_actions_recorded += 1;
        
        Ok(())
    }
    
    /// Kaydı duraklat
    pub fn pause_recording(&mut self) -> HandsResult<()> {
        if self.state != RecordingState::Recording {
            return Err(HandsError::RecordingError("Kayıt yapılıyor değil".to_string()));
        }
        
        self.state = RecordingState::Paused;
        log::info!("⏸️ RECORDING: Kayıt duraklatıldı");
        
        Ok(())
    }
    
    /// Kayda devam et
    pub fn resume_recording(&mut self) -> HandsResult<()> {
        if self.state != RecordingState::Paused {
            return Err(HandsError::RecordingError("Duraklatılmış değil".to_string()));
        }
        
        self.state = RecordingState::Recording;
        self.last_action_time = Some(Instant::now());
        log::info!("⏺️ RECORDING: Kayda devam ediliyor");
        
        Ok(())
    }
    
    /// Kaydı bitir
    pub fn stop_recording(&mut self, name: &str) -> HandsResult<Macro> {
        if self.state != RecordingState::Recording && self.state != RecordingState::Paused {
            return Err(HandsError::RecordingError("Kayıt yapılıyor değil".to_string()));
        }
        
        let id = self.next_id;
        self.next_id += 1;
        
        // Toplam süreyi hesapla
        if let Some(start) = self.start_time {
            self.recorded_duration = start.elapsed();
        }
        
        let macro_ = Macro::new(id, name, self.actions.clone());
        
        // Kaydet
        self.macros.push(macro_.clone());
        self.stats.total_macros += 1;
        self.stats.total_recordings += 1;
        
        // Durumu sıfırla
        self.state = RecordingState::Idle;
        self.actions.clear();
        self.start_time = None;
        
        log::info!("⏹️ RECORDING: Kayıt bitti - '{}' ({} aksiyon)", name, macro_.action_count());
        
        Ok(macro_)
    }
    
    /// Kaydı iptal et
    pub fn cancel_recording(&mut self) {
        self.state = RecordingState::Idle;
        self.actions.clear();
        self.start_time = None;
        log::info!("⏹️ RECORDING: Kayıt iptal edildi");
    }
    
    // ─── OYNATMA ───
    
    /// Makroyu oynat (simülasyon)
    pub fn playback(&mut self, macro_: &Macro, settings: &PlaybackSettings) -> PlaybackResult {
        if self.state != RecordingState::Idle {
            return PlaybackResult {
                success: false,
                actions_played: 0,
                duration_ms: 0,
                loops_completed: 0,
                error: Some("Kaydedici meşgul".to_string()),
                stop_reason: Some(StopReason::Error),
            };
        }
        
        self.state = RecordingState::Playing;
        let start = Instant::now();
        
        let mut actions_played = 0;
        let mut loops_completed = 0;
        let max_loops = if settings.loop_count == 0 { u32::MAX } else { settings.loop_count };
        
        for loop_idx in 0..max_loops {
            for (idx, action) in macro_.actions.iter().enumerate() {
                // Breakpoint kontrolü
                if settings.breakpoints.contains(&idx) {
                    log::info!("▶️ PLAYBACK: Breakpoint'e ulaşıldı (aksiyon {})", idx);
                    // Simülasyonda durmaz, sadece log
                }
                
                // Gecikme hesapla
                let base_delay = action.delay_ms as f64 / settings.speed;
                let delay_ms = if settings.humanize {
                    use rand::Rng;
                    let variance = settings.humanize_variance_ms as f64;
                    let jitter = rand::thread_rng().gen_range(-variance..variance);
                    (base_delay + jitter).max(0.0) as u64
                } else {
                    base_delay as u64
                };
                
                // Gerçek bekleme yapmıyoruz (simülasyon)
                // Gerçek uygulamada: tokio::time::sleep(Duration::from_millis(delay_ms)).await;
                let _ = delay_ms; // Uyarıyı bastır
                
                // Aksiyonu "çalıştır" (simülasyon)
                log::debug!(
                    "▶️ PLAYBACK: {} {} - {}ms",
                    action.action_type.icon(),
                    action.description,
                    delay_ms
                );
                
                actions_played += 1;
                self.stats.total_actions_played += 1;
            }
            
            loops_completed = loop_idx + 1;
            
            // Döngüler arası bekleme (simülasyon)
            if loop_idx + 1 < max_loops {
                let _ = settings.loop_delay_ms;
            }
        }
        
        let duration_ms = start.elapsed().as_millis() as u64;
        self.state = RecordingState::Idle;
        self.stats.total_playbacks += 1;
        
        log::info!(
            "▶️ PLAYBACK: Tamamlandı - {} aksiyon, {} döngü, {}ms",
            actions_played,
            loops_completed,
            duration_ms
        );
        
        PlaybackResult {
            success: true,
            actions_played,
            duration_ms,
            loops_completed,
            error: None,
            stop_reason: Some(StopReason::Completed),
        }
    }
    
    /// Oynatmayı durdur
    pub fn stop_playback(&mut self) {
        if self.state == RecordingState::Playing {
            self.state = RecordingState::Idle;
            log::info!("⏹️ PLAYBACK: Durduruldu");
        }
    }
    
    // ─── MAKRO YÖNETİMİ ───
    
    /// Makroları listele
    pub fn list_macros(&self) -> &[Macro] {
        &self.macros
    }
    
    /// Makro getir
    pub fn get_macro(&self, id: u64) -> Option<&Macro> {
        self.macros.iter().find(|m| m.id == id)
    }
    
    /// Makro sil
    pub fn delete_macro(&mut self, id: u64) -> HandsResult<()> {
        if let Some(pos) = self.macros.iter().position(|m| m.id == id) {
            self.macros.remove(pos);
            log::info!("🗑️ MACRO: Makro {} silindi", id);
            Ok(())
        } else {
            Err(HandsError::RecordingError(format!("Makro bulunamadı: {}", id)))
        }
    }
    
    /// Makro yeniden adlandır
    pub fn rename_macro(&mut self, id: u64, new_name: &str) -> HandsResult<()> {
        if let Some(macro_) = self.macros.iter_mut().find(|m| m.id == id) {
            macro_.name = new_name.to_string();
            log::info!("📝 MACRO: Makro {} yeniden adlandırıldı → {}", id, new_name);
            Ok(())
        } else {
            Err(HandsError::RecordingError(format!("Makro bulunamadı: {}", id)))
        }
    }
    
    /// Makroyu kopyala
    pub fn duplicate_macro(&mut self, id: u64) -> HandsResult<u64> {
        // Önce makroyu klonla
        let macro_clone = if let Some(m) = self.macros.iter().find(|m| m.id == id) {
            m.clone()
        } else {
            return Err(HandsError::RecordingError(format!("Makro bulunamadı: {}", id)));
        };
        
        let new_id = self.next_id;
        self.next_id += 1;
        
        let mut new_macro = macro_clone.clone();
        new_macro.id = new_id;
        new_macro.name = format!("{} (kopya)", macro_clone.name);
        new_macro.created_at = chrono::Utc::now();
        new_macro.last_used = None;
        new_macro.use_count = 0;
        
        self.macros.push(new_macro);
        self.stats.total_macros += 1;
        
        log::info!("📋 MACRO: Makro {} kopyalandı → {}", id, new_id);
        
        Ok(new_id)
    }
    
    /// Dışa aktar (JSON)
    pub fn export_macro(&self, id: u64) -> HandsResult<String> {
        if let Some(macro_) = self.get_macro(id) {
            serde_json::to_string_pretty(macro_)
                .map_err(|e| HandsError::RecordingError(format!("Dışa aktarma hatası: {}", e)))
        } else {
            Err(HandsError::RecordingError(format!("Makro bulunamadı: {}", id)))
        }
    }
    
    /// İçe aktar (JSON)
    pub fn import_macro(&mut self, json: &str) -> HandsResult<u64> {
        let mut macro_: Macro = serde_json::from_str(json)
            .map_err(|e| HandsError::RecordingError(format!("İçe aktarma hatası: {}", e)))?;
        
        let id = self.next_id;
        self.next_id += 1;
        
        macro_.id = id;
        macro_.created_at = chrono::Utc::now();
        
        self.macros.push(macro_.clone());
        self.stats.total_macros += 1;
        
        log::info!("📥 MACRO: Makro içe aktarıldı → {}", id);
        
        Ok(id)
    }
    
    // ─── RAPORLAMA ───
    
    /// Durum
    pub fn state(&self) -> RecordingState {
        self.state
    }
    
    /// Kayıt yapılıyor mu?
    pub fn is_recording(&self) -> bool {
        self.state == RecordingState::Recording
    }
    
    /// Oynatılıyor mu?
    pub fn is_playing(&self) -> bool {
        self.state == RecordingState::Playing
    }
    
    /// Mevcut kayıttaki aksiyon sayısı
    pub fn current_action_count(&self) -> usize {
        self.actions.len()
    }
    
    /// İstatistikler
    pub fn stats(&self) -> &RecorderStats {
        &self.stats
    }
    
    /// Rapor
    pub fn report(&self) -> RecorderReport {
        RecorderReport {
            state: self.state,
            current_actions: self.actions.len(),
            total_macros: self.macros.len(),
            stats: self.stats.clone(),
        }
    }
    
    /// Config
    pub fn config(&self) -> &RecorderConfig {
        &self.config
    }
}

/// Kaydedici raporu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecorderReport {
    pub state: RecordingState,
    pub current_actions: usize,
    pub total_macros: usize,
    pub stats: RecorderStats,
}

impl std::fmt::Display for RecorderReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "╔════════════════════════════════════════════╗")?;
        writeln!(f, "║         MACRO RECORDER RAPORU              ║")?;
        writeln!(f, "╠════════════════════════════════════════════╣")?;
        writeln!(f, "║ Durum: {:2} {:<29} ║", self.state.icon(), self.state.description())?;
        writeln!(f, "║ Mevcut Aksiyonlar: {:<23} ║", self.current_actions)?;
        writeln!(f, "║ Kayıtlı Makro: {:<28} ║", self.total_macros)?;
        writeln!(f, "╠════════════════════════════════════════════╣")?;
        writeln!(f, "║ İstatistikler:                               ║")?;
        writeln!(f, "║ ├─ Toplam Makro: {:<27} ║", self.stats.total_macros)?;
        writeln!(f, "║ ├─ Kayıtlar: {:<31} ║", self.stats.total_recordings)?;
        writeln!(f, "║ ├─ Oynatmalar: {:<30} ║", self.stats.total_playbacks)?;
        writeln!(f, "║ ├─ Kaydedilen Aksiyon: {:<22} ║", self.stats.total_actions_recorded)?;
        writeln!(f, "║ └─ Oynatılan Aksiyon: {:<22} ║", self.stats.total_actions_played)?;
        writeln!(f, "╚════════════════════════════════════════════╝")
    }
}

// ───────────────────────────────────────────────────────────────────────────────
//  TESTS
// ───────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_timed_action_creation() {
        let action = TimedAction::new(
            UndoableActionType::MouseClick,
            r#"{"x":100,"y":200}"#,
            50,
            "Test click"
        );
        
        assert_eq!(action.delay_ms, 50);
        assert_eq!(action.description, "Test click");
    }
    
    #[test]
    fn test_timed_action_delay() {
        let action = TimedAction::new(UndoableActionType::MouseMove, "{}", 100, "");
        assert_eq!(action.delay(), Duration::from_millis(100));
    }
    
    #[test]
    fn test_macro_creation() {
        let actions = vec![
            TimedAction::new(UndoableActionType::MouseClick, "{}", 0, "Click 1"),
            TimedAction::new(UndoableActionType::KeyPress, "{}", 100, "Key 1"),
        ];
        
        let macro_ = Macro::new(1, "test-macro", actions);
        
        assert_eq!(macro_.id, 1);
        assert_eq!(macro_.name, "test-macro");
        assert_eq!(macro_.action_count(), 2);
        assert_eq!(macro_.total_duration_ms, 100);
    }
    
    #[test]
    fn test_macro_with_description() {
        let actions = vec![];
        let macro_ = Macro::new(1, "test", actions)
            .with_description("Test description");
        
        assert_eq!(macro_.description, "Test description");
    }
    
    #[test]
    fn test_macro_tags() {
        let mut macro_ = Macro::new(1, "test", vec![]);
        macro_.add_tag("important");
        macro_.add_tag("test");
        macro_.add_tag("important"); // Duplicate
        
        assert_eq!(macro_.tags.len(), 2);
    }
    
    #[test]
    fn test_macro_mark_used() {
        let mut macro_ = Macro::new(1, "test", vec![]);
        
        assert_eq!(macro_.use_count, 0);
        assert!(macro_.last_used.is_none());
        
        macro_.mark_used();
        
        assert_eq!(macro_.use_count, 1);
        assert!(macro_.last_used.is_some());
    }
    
    #[test]
    fn test_macro_formatted_duration() {
        let mut macro_ = Macro::new(1, "test", vec![]);
        
        macro_.total_duration_ms = 500;
        assert!(macro_.formatted_duration().contains("500ms"));
        
        macro_.total_duration_ms = 5000;
        assert!(macro_.formatted_duration().contains("5s"));
        
        macro_.total_duration_ms = 125000;
        assert!(macro_.formatted_duration().contains("2m"));
    }
    
    #[test]
    fn test_macro_summary() {
        let actions = vec![
            TimedAction::new(UndoableActionType::MouseClick, "{}", 0, ""),
            TimedAction::new(UndoableActionType::MouseMove, "{}", 10, ""),
            TimedAction::new(UndoableActionType::KeyPress, "{}", 20, ""),
            TimedAction::new(UndoableActionType::TypeText, "{}", 30, ""),
            TimedAction::new(UndoableActionType::Screenshot, "{}", 40, ""),
        ];
        
        let macro_ = Macro::new(1, "test", actions);
        let summary = macro_.summary();
        
        assert_eq!(summary.action_count, 5);
        assert_eq!(summary.mouse_count, 2);
        assert_eq!(summary.keyboard_count, 2);
        assert_eq!(summary.other_count, 1);
    }
    
    #[test]
    fn test_recorder_config_default() {
        let config = RecorderConfig::default();
        assert_eq!(config.max_duration_secs, 300);
        assert_eq!(config.max_actions, 1000);
        assert!(config.noise_filter);
    }
    
    #[test]
    fn test_recorder_config_presets() {
        let long = RecorderConfig::long();
        assert_eq!(long.max_duration_secs, 1800);
        
        let precise = RecorderConfig::precise();
        assert!(!precise.noise_filter);
        assert_eq!(precise.min_delay_ms, 1);
    }
    
    #[test]
    fn test_playback_settings_default() {
        let settings = PlaybackSettings::default();
        assert_eq!(settings.speed, 1.0);
        assert_eq!(settings.loop_count, 1);
        assert!(!settings.humanize);
    }
    
    #[test]
    fn test_playback_settings_presets() {
        let fast = PlaybackSettings::fast();
        assert_eq!(fast.speed, 2.0);
        
        let slow = PlaybackSettings::slow();
        assert_eq!(slow.speed, 0.5);
        
        let human = PlaybackSettings::humanized();
        assert!(human.humanize);
        
        let infinite = PlaybackSettings::infinite_loop();
        assert_eq!(infinite.loop_count, 0);
    }
    
    #[test]
    fn test_playback_settings_with_speed() {
        let settings = PlaybackSettings::new().with_speed(3.0);
        assert_eq!(settings.speed, 3.0);
        
        // Clamping test
        let settings = PlaybackSettings::new().with_speed(20.0);
        assert_eq!(settings.speed, 10.0);
        
        let settings = PlaybackSettings::new().with_speed(0.01);
        assert_eq!(settings.speed, 0.1);
    }
    
    #[test]
    fn test_recording_state() {
        assert_eq!(RecordingState::Idle.description(), "Boşta");
        assert_eq!(RecordingState::Recording.description(), "Kaydediyor");
        assert_eq!(RecordingState::Playing.description(), "Oynatıyor");
        
        assert_eq!(RecordingState::Idle.icon(), "⏹️");
        assert_eq!(RecordingState::Recording.icon(), "⏺️");
        assert_eq!(RecordingState::Playing.icon(), "▶️");
    }
    
    #[test]
    fn test_macro_recorder_creation() {
        let recorder = MacroRecorder::default_config();
        assert_eq!(recorder.state(), RecordingState::Idle);
        assert!(!recorder.is_recording());
        assert!(!recorder.is_playing());
    }
    
    #[test]
    fn test_macro_recorder_start_stop() {
        let mut recorder = MacroRecorder::default_config();
        
        recorder.start_recording().unwrap();
        assert!(recorder.is_recording());
        
        let macro_ = recorder.stop_recording("test").unwrap();
        assert_eq!(macro_.name, "test");
        assert_eq!(recorder.state(), RecordingState::Idle);
    }
    
    #[test]
    fn test_macro_recorder_record_action() {
        let mut recorder = MacroRecorder::default_config();
        recorder.start_recording().unwrap();
        
        recorder.record_action(UndoableActionType::MouseClick, r#"{"x":100}"#, "Click").unwrap();
        
        assert_eq!(recorder.current_action_count(), 1);
        
        recorder.stop_recording("test").unwrap();
    }
    
    #[test]
    fn test_macro_recorder_pause_resume() {
        let mut recorder = MacroRecorder::default_config();
        recorder.start_recording().unwrap();
        
        recorder.pause_recording().unwrap();
        assert_eq!(recorder.state(), RecordingState::Paused);
        
        recorder.resume_recording().unwrap();
        assert_eq!(recorder.state(), RecordingState::Recording);
        
        recorder.stop_recording("test").unwrap();
    }
    
    #[test]
    fn test_macro_recorder_cancel() {
        let mut recorder = MacroRecorder::default_config();
        recorder.start_recording().unwrap();
        recorder.record_action(UndoableActionType::MouseClick, "{}", "").unwrap();
        
        recorder.cancel_recording();
        
        assert_eq!(recorder.state(), RecordingState::Idle);
        assert_eq!(recorder.current_action_count(), 0);
    }
    
    #[test]
    fn test_macro_recorder_playback() {
        let mut recorder = MacroRecorder::default_config();
        
        // Kayıt
        recorder.start_recording().unwrap();
        recorder.record_action(UndoableActionType::MouseClick, "{}", "Click").unwrap();
        let macro_ = recorder.stop_recording("test").unwrap();
        
        // Oynat
        let settings = PlaybackSettings::default();
        let result = recorder.playback(&macro_, &settings);
        
        assert!(result.success);
        assert_eq!(result.actions_played, 1);
        assert_eq!(result.loops_completed, 1);
    }
    
    #[test]
    fn test_macro_recorder_playback_with_loops() {
        let mut recorder = MacroRecorder::default_config();
        
        recorder.start_recording().unwrap();
        recorder.record_action(UndoableActionType::MouseClick, "{}", "Click").unwrap();
        let macro_ = recorder.stop_recording("test").unwrap();
        
        let settings = PlaybackSettings {
            loop_count: 3,
            ..Default::default()
        };
        
        let result = recorder.playback(&macro_, &settings);
        
        assert_eq!(result.actions_played, 3);
        assert_eq!(result.loops_completed, 3);
    }
    
    #[test]
    fn test_macro_recorder_macro_management() {
        let mut recorder = MacroRecorder::default_config();
        
        recorder.start_recording().unwrap();
        recorder.record_action(UndoableActionType::MouseClick, "{}", "Click").unwrap();
        let macro_ = recorder.stop_recording("test").unwrap();
        
        // List
        assert_eq!(recorder.list_macros().len(), 1);
        
        // Get
        let found = recorder.get_macro(macro_.id).unwrap();
        assert_eq!(found.name, "test");
        
        // Rename
        recorder.rename_macro(macro_.id, "renamed").unwrap();
        let found = recorder.get_macro(macro_.id).unwrap();
        assert_eq!(found.name, "renamed");
        
        // Duplicate
        let new_id = recorder.duplicate_macro(macro_.id).unwrap();
        assert_eq!(recorder.list_macros().len(), 2);
        
        // Delete
        recorder.delete_macro(macro_.id).unwrap();
        assert_eq!(recorder.list_macros().len(), 1);
        
        recorder.delete_macro(new_id).unwrap();
        assert_eq!(recorder.list_macros().len(), 0);
    }
    
    #[test]
    fn test_macro_recorder_export_import() {
        let mut recorder = MacroRecorder::default_config();
        
        recorder.start_recording().unwrap();
        recorder.record_action(UndoableActionType::MouseClick, "{}", "Click").unwrap();
        let macro_ = recorder.stop_recording("test").unwrap();
        
        // Export
        let json = recorder.export_macro(macro_.id).unwrap();
        assert!(json.contains("test"));
        
        // Delete
        recorder.delete_macro(macro_.id).unwrap();
        assert_eq!(recorder.list_macros().len(), 0);
        
        // Import
        let new_id = recorder.import_macro(&json).unwrap();
        let imported = recorder.get_macro(new_id).unwrap();
        assert_eq!(imported.name, "test");
    }
    
    #[test]
    fn test_macro_recorder_stats() {
        let mut recorder = MacroRecorder::default_config();
        
        recorder.start_recording().unwrap();
        recorder.record_action(UndoableActionType::MouseClick, "{}", "Click").unwrap();
        let macro_ = recorder.stop_recording("test").unwrap();
        
        recorder.playback(&macro_, &PlaybackSettings::default());
        
        assert_eq!(recorder.stats().total_macros, 1);
        assert_eq!(recorder.stats().total_recordings, 1);
        assert_eq!(recorder.stats().total_playbacks, 1);
        assert_eq!(recorder.stats().total_actions_recorded, 1);
        assert_eq!(recorder.stats().total_actions_played, 1);
    }
    
    #[test]
    fn test_macro_recorder_report() {
        let recorder = MacroRecorder::default_config();
        let report = recorder.report();
        let output = format!("{}", report);
        
        assert!(output.contains("MACRO RECORDER RAPORU"));
    }
    
    #[test]
    fn test_playback_result() {
        let result = PlaybackResult {
            success: true,
            actions_played: 10,
            duration_ms: 1000,
            loops_completed: 2,
            error: None,
            stop_reason: Some(StopReason::Completed),
        };
        
        assert!(result.success);
        assert_eq!(result.actions_played, 10);
    }
    
    #[test]
    fn test_stop_reason() {
        assert_eq!(StopReason::Completed, StopReason::Completed);
        assert_ne!(StopReason::Completed, StopReason::Error);
    }
}
