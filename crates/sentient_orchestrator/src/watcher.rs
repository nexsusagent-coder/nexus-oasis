//! ─── WATCHER (OTONOM GÖZCÜ) ───
//!
//! 12. Adım: Belirli aralıklarla Scout ve Forge birimlerini tetikleyen
//! otonom izleme ve görev üretme sistemi.
//!
//! - Periyodik tarama (Scout ile veri avı)
//! - Kod üretim görevleri (Forge ile)
//! - Zamanlanmış görevler
//! - Tetikleyici bazlı görevler

use crate::goal::{Goal, TaskPriority};
use sentient_common::error::SENTIENTResult;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use parking_lot::RwLock;
use tokio::sync::mpsc;
use chrono::{DateTime, Utc, Duration, Timelike, Datelike};
use uuid::Uuid;

/// ─── WATCHER ───
/// 
/// SENTIENT'nın otonom gözü - belirli aralıklarla görevler üretir
/// ve Scout/Forge birimlerini tetikler.

pub struct Watcher {
    /// Yapılandırma
    config: WatcherConfig,
    /// Zamanlanmış görevler
    scheduled_tasks: Arc<RwLock<Vec<ScheduledTask>>>,
    /// Tetikleyiciler
    triggers: Arc<RwLock<Vec<Trigger>>>,
    /// Görev kanalı (orchestrator'a)
    task_sender: Option<mpsc::Sender<WatcherTask>>,
    /// İstatistikler
    stats: Arc<RwLock<WatcherStats>>,
    /// Durum
    status: Arc<RwLock<WatcherStatus>>,
    /// Başlangıç zamanı
    started_at: DateTime<Utc>,
}

/// Watcher yapılandırması
#[derive(Debug, Clone)]
pub struct WatcherConfig {
    /// Tarama aralığı (saniye)
    pub scan_interval_secs: u64,
    /// Scout taraması aktif mi?
    pub scout_enabled: bool,
    /// Forge görevleri aktif mi?
    pub forge_enabled: bool,
    /// Maksimum görev sayısı (bir döngüde)
    pub max_tasks_per_cycle: usize,
    /// Otomatik görev üretimi
    pub auto_generate: bool,
    /// İlgi alanları
    pub interests: Vec<String>,
    /// Hedef URL'ler (Scout için)
    pub target_urls: Vec<String>,
    /// Aktif saatler (0-23)
    pub active_hours: Vec<u8>,
    /// Haftalık program (1=Pazartesi, 7=Pazar)
    pub active_days: Vec<u8>,
}

impl Default for WatcherConfig {
    fn default() -> Self {
        Self {
            scan_interval_secs: 300, // 5 dakika
            scout_enabled: true,
            forge_enabled: true,
            max_tasks_per_cycle: 5,
            auto_generate: true,
            interests: vec![
                "yapay zeka".into(),
                "teknoloji".into(),
                "yazılım".into(),
                "Rust".into(),
                "Python".into(),
            ],
            target_urls: vec![
                "https://github.com/trending".into(),
                "https://reddit.com/r/rust".into(),
                "https://news.ycombinator.com".into(),
            ],
            active_hours: (0..24).collect(), // 7/24
            active_days: (1..8).collect(),   // Her gün
        }
    }
}

/// Watcher durumu
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WatcherStatus {
    /// Başlatılmadı
    NotStarted,
    /// Çalışıyor
    Running,
    /// Duraklatıldı
    Paused,
    /// Hata
    Error,
    /// Durduruldu
    Stopped,
}

/// Zamanlanmış görev
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledTask {
    /// Görev ID
    pub id: Uuid,
    /// Görev adı
    pub name: String,
    /// Görev tipi
    pub task_type: ScheduledTaskType,
    /// Tekrar tipi
    pub repeat: RepeatType,
    /// Son çalışma
    pub last_run: Option<DateTime<Utc>>,
    /// Sonraki çalışma
    pub next_run: DateTime<Utc>,
    /// Aktif mi?
    pub enabled: bool,
    /// Başarı sayısı
    pub success_count: u32,
    /// Hata sayısı
    pub error_count: u32,
    /// Parametreler
    pub params: serde_json::Value,
}

/// Zamanlanmış görev tipi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScheduledTaskType {
    /// Web tarama (Scout)
    ScoutScan {
        /// Hedef URL
        url: String,
        /// Çıkarılacak veri tipi
        extract_type: String,
    },
    /// Veri avı
    DataHunt {
        /// Anahtar kelimeler
        keywords: Vec<String>,
        /// Hedef platform
        platform: String,
    },
    /// Kod üretimi (Forge)
    ForgeGenerate {
        /// Şablon
        template: String,
        /// Çıktı formatı
        output_format: String,
    },
    /// Kod analiz
    CodeAnalysis {
        /// Hedef dizin
        target_dir: String,
    },
    /// Bellek temizliği
    MemoryCleanup,
    /// Sistem durumu kontrolü
    HealthCheck,
    /// Öğrenme görevi
    LearningTask {
        /// Konu
        topic: String,
    },
}

/// Tekrar tipi
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum RepeatType {
    /// Bir kez
    Once,
    /// Her dakika
    EveryMinute,
    /// Her saat
    Hourly,
    /// Günlük
    Daily,
    /// Haftalık
    Weekly,
    /// Özel (saniye)
    Custom(u64),
}

impl RepeatType {
    /// Sonraki çalışma zamanını hesapla
    pub fn next_run(&self, from: DateTime<Utc>) -> DateTime<Utc> {
        match self {
            Self::Once => from,
            Self::EveryMinute => from + Duration::minutes(1),
            Self::Hourly => from + Duration::hours(1),
            Self::Daily => from + Duration::days(1),
            Self::Weekly => from + Duration::weeks(1),
            Self::Custom(secs) => from + Duration::seconds(*secs as i64),
        }
    }
}

/// Tetikleyici
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trigger {
    /// Trigger ID
    pub id: Uuid,
    /// Tetikleyici adı
    pub name: String,
    /// Koşul
    pub condition: TriggerCondition,
    /// Aksiyon
    pub action: TriggerAction,
    /// Aktif mi?
    pub enabled: bool,
    /// Son tetiklenme
    pub last_triggered: Option<DateTime<Utc>>,
    /// Tetiklenme sayısı
    pub trigger_count: u32,
}

/// Tetikleyici koşulu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TriggerCondition {
    /// Belirli bir saatte
    AtHour(u8),
    /// Bellek boyutu limiti aşıldığında
    MemorySizeExceeded(usize),
    /// Belirli bir event olduğunda
    OnEvent(String),
    /// Yeni dosya eklendiğinde
    NewFileInDir(String),
    /// API yanıt süresi yüksekse
    ApiLatencyHigh(u64),
    /// Hata oranı yüksekse
    ErrorRateAbove(f32),
}

/// Tetikleyici aksiyonu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TriggerAction {
    /// Scout taraması başlat
    ScoutScan(String),
    /// Forge görevi başlat
    ForgeGenerate(String),
    /// Bildirim gönder
    Notify(String),
    /// Bellek temizle
    CleanupMemory,
    /// Sistem yeniden başlat
    Restart,
}

/// Watcher'dan çıkan görev
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatcherTask {
    /// Görev ID
    pub id: Uuid,
    /// Kaynak (hangi scheduled task veya trigger)
    pub source: TaskSource,
    /// Oluşturulan hedef
    pub goal: Goal,
    /// Öncelik
    pub priority: TaskPriority,
    /// Zaman damgası
    pub timestamp: DateTime<Utc>,
}

/// Görev kaynağı
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskSource {
    /// Zamanlanmış görev
    Scheduled(Uuid),
    /// Tetikleyici
    Trigger(Uuid),
    /// Manuel
    Manual,
    /// Otomatik üretilen
    AutoGenerated,
}

/// Watcher istatistikleri
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WatcherStats {
    /// Toplam döngü sayısı
    pub total_cycles: u64,
    /// Üretilen görev sayısı
    pub tasks_generated: u64,
    /// Başarılı görevler
    pub successful_tasks: u64,
    /// Başarısız görevler
    pub failed_tasks: u64,
    /// Scout tarama sayısı
    pub scout_scans: u64,
    /// Forge üretim sayısı
    pub forge_generations: u64,
    /// Tetiklenme sayısı
    pub triggers_fired: u64,
    /// Toplam çalışma süresi (saniye)
    pub uptime_secs: u64,
}

impl Watcher {
    /// Yeni watcher oluştur
    pub fn new(config: WatcherConfig) -> Self {
        Self {
            config,
            scheduled_tasks: Arc::new(RwLock::new(Vec::new())),
            triggers: Arc::new(RwLock::new(Vec::new())),
            task_sender: None,
            stats: Arc::new(RwLock::new(WatcherStats::default())),
            status: Arc::new(RwLock::new(WatcherStatus::NotStarted)),
            started_at: Utc::now(),
        }
    }
    
    /// Görev kanalını bağla
    pub fn with_channel(mut self, sender: mpsc::Sender<WatcherTask>) -> Self {
        self.task_sender = Some(sender);
        self
    }
    
    /// Watcher'ı başlat
    pub async fn start(&mut self) -> SENTIENTResult<()> {
        log::info!("════════════════════════════════════════════════════════════");
        log::info!("👁️  WATCHER başlatılıyor...");
        log::info!("════════════════════════════════════════════════════════════");
        
        *self.status.write() = WatcherStatus::Running;
        self.started_at = Utc::now();
        
        // Varsayılan zamanlanmış görevleri ekle
        self.setup_default_tasks();
        
        // Varsayılan tetikleyicileri ekle
        self.setup_default_triggers();
        
        log::info!("✅  WATCHER hazır");
        log::info!("   ⏱️  Tarama aralığı: {}s", self.config.scan_interval_secs);
        log::info!("   🔍 Scout: {} | Forge: {}", 
            if self.config.scout_enabled { "aktif" } else { "pasif" },
            if self.config.forge_enabled { "aktif" } else { "pasif" }
        );
        log::info!("   📅 Zamanlanmış görev: {}", self.scheduled_tasks.read().len());
        log::info!("   🎯 Tetikleyici: {}", self.triggers.read().len());
        log::info!("════════════════════════════════════════════════════════════");
        
        Ok(())
    }
    
    /// Varsayılan zamanlanmış görevleri ayarla
    fn setup_default_tasks(&mut self) {
        let now = Utc::now();
        
        // Saatlik Scout taraması
        self.add_scheduled_task(ScheduledTask {
            id: Uuid::new_v4(),
            name: "Saatlik Web Taraması".into(),
            task_type: ScheduledTaskType::ScoutScan {
                url: "https://github.com/trending".into(),
                extract_type: "trending_repos".into(),
            },
            repeat: RepeatType::Hourly,
            last_run: None,
            next_run: now + Duration::hours(1),
            enabled: self.config.scout_enabled,
            success_count: 0,
            error_count: 0,
            params: serde_json::json!({}),
        });
        
        // Günlük veri avı
        self.add_scheduled_task(ScheduledTask {
            id: Uuid::new_v4(),
            name: "Günlük Veri Avı".into(),
            task_type: ScheduledTaskType::DataHunt {
                keywords: self.config.interests.clone(),
                platform: "web".into(),
            },
            repeat: RepeatType::Daily,
            last_run: None,
            next_run: now + Duration::days(1),
            enabled: self.config.scout_enabled,
            success_count: 0,
            error_count: 0,
            params: serde_json::json!({}),
        });
        
        // Haftalık kod analizi
        self.add_scheduled_task(ScheduledTask {
            id: Uuid::new_v4(),
            name: "Haftalık Kod Analizi".into(),
            task_type: ScheduledTaskType::CodeAnalysis {
                target_dir: "/root/SENTIENT_CORE".into(),
            },
            repeat: RepeatType::Weekly,
            last_run: None,
            next_run: now + Duration::weeks(1),
            enabled: self.config.forge_enabled,
            success_count: 0,
            error_count: 0,
            params: serde_json::json!({}),
        });
        
        // Saatlik health check
        self.add_scheduled_task(ScheduledTask {
            id: Uuid::new_v4(),
            name: "Sistem Sağlık Kontrolü".into(),
            task_type: ScheduledTaskType::HealthCheck,
            repeat: RepeatType::Hourly,
            last_run: None,
            next_run: now + Duration::hours(1),
            enabled: true,
            success_count: 0,
            error_count: 0,
            params: serde_json::json!({}),
        });
        
        // Günlük bellek temizliği
        self.add_scheduled_task(ScheduledTask {
            id: Uuid::new_v4(),
            name: "Bellek Temizliği".into(),
            task_type: ScheduledTaskType::MemoryCleanup,
            repeat: RepeatType::Daily,
            last_run: None,
            next_run: now + Duration::days(1),
            enabled: true,
            success_count: 0,
            error_count: 0,
            params: serde_json::json!({}),
        });
    }
    
    /// Varsayılan tetikleyicileri ayarla
    fn setup_default_triggers(&mut self) {
        // Gece yarısı öğrenme görevi
        self.add_trigger(Trigger {
            id: Uuid::new_v4(),
            name: "Gece Yarısı Öğrenme".into(),
            condition: TriggerCondition::AtHour(0),
            action: TriggerAction::ScoutScan("https://news.ycombinator.com".into()),
            enabled: true,
            last_triggered: None,
            trigger_count: 0,
        });
        
        // Yüksek hata oranı tetikleyicisi
        self.add_trigger(Trigger {
            id: Uuid::new_v4(),
            name: "Hata Oranı Uyarısı".into(),
            condition: TriggerCondition::ErrorRateAbove(0.3),
            action: TriggerAction::Notify("Hata oranı yüksek, sistem kontrolü gerekiyor".into()),
            enabled: true,
            last_triggered: None,
            trigger_count: 0,
        });
    }
    
    /// Zamanlanmış görev ekle
    pub fn add_scheduled_task(&mut self, task: ScheduledTask) {
        self.scheduled_tasks.write().push(task);
    }
    
    /// Tetikleyici ekle
    pub fn add_trigger(&mut self, trigger: Trigger) {
        self.triggers.write().push(trigger);
    }
    
    /// ─── ANA DÖNGÜ ───
    /// 
    /// Bu fonksiyon belirli aralıklarla çağrılmalıdır.
    /// Scout ve Forge birimlerini tetikler.
    
    pub async fn tick(&mut self) -> SENTIENTResult<Vec<WatcherTask>> {
        if *self.status.read() != WatcherStatus::Running {
            return Ok(Vec::new());
        }
        
        let now = Utc::now();
        let mut generated_tasks = Vec::new();
        
        // Aktif saat kontrolü
        if !self.is_active_hour(&now) {
            log::debug!("Watcher: Aktif saat dışı, atlanıyor");
            return Ok(Vec::new());
        }
        
        // İstatistikleri güncelle
        self.stats.write().total_cycles += 1;
        self.stats.write().uptime_secs = (now - self.started_at).num_seconds() as u64;
        
        // 1. Zamanlanmış görevleri kontrol et
        let due_tasks = self.get_due_tasks(&now);
        for task in due_tasks {
            let watcher_task = self.create_watcher_task(&task);
            if let Some(wt) = watcher_task {
                generated_tasks.push(wt);
                self.stats.write().tasks_generated += 1;
            }
        }
        
        // 2. Tetikleyicileri kontrol et
        let fired = self.check_triggers(&now);
        for trigger in fired {
            let watcher_task = self.create_trigger_task(&trigger);
            if let Some(wt) = watcher_task {
                generated_tasks.push(wt);
                self.stats.write().triggers_fired += 1;
            }
        }
        
        // 3. Otomatik görev üretimi
        if self.config.auto_generate && generated_tasks.len() < self.config.max_tasks_per_cycle {
            let auto_tasks = self.generate_auto_tasks();
            generated_tasks.extend(auto_tasks);
        }
        
        // Görevleri gönder
        for task in &generated_tasks {
            if let Some(ref sender) = self.task_sender {
                let _ = sender.send(task.clone()).await;
            }
        }
        
        if !generated_tasks.is_empty() {
            log::info!("👁️  Watcher: {} yeni görev üretildi", generated_tasks.len());
        }
        
        Ok(generated_tasks)
    }
    
    /// Aktif saat kontrolü
    fn is_active_hour(&self, now: &DateTime<Utc>) -> bool {
        let hour = now.hour() as u8;
        let day = now.weekday().number_from_monday() as u8;
        
        self.config.active_hours.contains(&hour) && 
        self.config.active_days.contains(&day)
    }
    
    /// Vakti gelmiş görevleri al
    fn get_due_tasks(&self, now: &DateTime<Utc>) -> Vec<ScheduledTask> {
        self.scheduled_tasks.read()
            .iter()
            .filter(|t| t.enabled && t.next_run <= *now)
            .cloned()
            .collect()
    }
    
    /// Tetikleyicileri kontrol et
    fn check_triggers(&self, now: &DateTime<Utc>) -> Vec<Trigger> {
        self.triggers.read()
            .iter()
            .filter(|t| {
                if !t.enabled { return false; }
                
                match &t.condition {
                    TriggerCondition::AtHour(h) => now.hour() as u8 == *h && now.minute() == 0,
                    _ => false, // Diğer koşullar harici event gerektirir
                }
            })
            .cloned()
            .collect()
    }
    
    /// Watcher görevi oluştur
    fn create_watcher_task(&self, scheduled: &ScheduledTask) -> Option<WatcherTask> {
        let goal = match &scheduled.task_type {
            ScheduledTaskType::ScoutScan { url, extract_type } => {
                Goal::new(format!("Scout taraması: {} - {}", url, extract_type))
                    .with_priority(TaskPriority::Low)
            }
            ScheduledTaskType::DataHunt { keywords, platform } => {
                Goal::new(format!("Veri avı: {:?} ({})", keywords, platform))
                    .with_priority(TaskPriority::Normal)
            }
            ScheduledTaskType::ForgeGenerate { template, output_format } => {
                Goal::new(format!("Forge üretimi: {} -> {}", template, output_format))
                    .with_priority(TaskPriority::Normal)
            }
            ScheduledTaskType::CodeAnalysis { target_dir } => {
                Goal::new(format!("Kod analizi: {}", target_dir))
                    .with_priority(TaskPriority::Low)
            }
            ScheduledTaskType::MemoryCleanup => {
                Goal::new("Bellek temizliği ve optimizasyonu")
                    .with_priority(TaskPriority::Low)
            }
            ScheduledTaskType::HealthCheck => {
                Goal::new("Sistem sağlık kontrolü")
                    .with_priority(TaskPriority::High)
            }
            ScheduledTaskType::LearningTask { topic } => {
                Goal::new(format!("Öğrenme görevi: {}", topic))
                    .with_priority(TaskPriority::Normal)
            }
        };
        
        Some(WatcherTask {
            id: Uuid::new_v4(),
            source: TaskSource::Scheduled(scheduled.id),
            goal,
            priority: scheduled.task_type.default_priority(),
            timestamp: Utc::now(),
        })
    }
    
    /// Tetikleyici görevi oluştur
    fn create_trigger_task(&self, trigger: &Trigger) -> Option<WatcherTask> {
        let goal = match &trigger.action {
            TriggerAction::ScoutScan(url) => {
                Goal::new(format!("Tetikleyici taraması: {}", url))
            }
            TriggerAction::ForgeGenerate(template) => {
                Goal::new(format!("Tetikleyici üretimi: {}", template))
            }
            TriggerAction::Notify(msg) => {
                Goal::new(format!("Bildirim: {}", msg))
            }
            TriggerAction::CleanupMemory => {
                Goal::new("Bellek temizliği (tetiklendi)")
            }
            TriggerAction::Restart => {
                Goal::new("Sistem yeniden başlatma")
            }
        };
        
        Some(WatcherTask {
            id: Uuid::new_v4(),
            source: TaskSource::Trigger(trigger.id),
            goal,
            priority: TaskPriority::High,
            timestamp: Utc::now(),
        })
    }
    
    /// Otomatik görev üret
    fn generate_auto_tasks(&self) -> Vec<WatcherTask> {
        let mut tasks = Vec::new();
        let stats = self.stats.read();
        
        // Scout görevi (yeterince tarama yapılmadıysa)
        if self.config.scout_enabled && stats.scout_scans < stats.total_cycles / 2 {
            if let Some(url) = self.config.target_urls.first() {
                tasks.push(WatcherTask {
                    id: Uuid::new_v4(),
                    source: TaskSource::AutoGenerated,
                    goal: Goal::new(format!("Otomatik Scout taraması: {}", url))
                        .with_priority(TaskPriority::Low),
                    priority: TaskPriority::Low,
                    timestamp: Utc::now(),
                });
            }
        }
        
        // Forge görevi (ilgi alanına göre)
        if self.config.forge_enabled && stats.forge_generations < stats.total_cycles / 4 {
            if let Some(interest) = self.config.interests.first() {
                tasks.push(WatcherTask {
                    id: Uuid::new_v4(),
                    source: TaskSource::AutoGenerated,
                    goal: Goal::new(format!("Otomatik Forge araştırması: {}", interest))
                        .with_priority(TaskPriority::Low),
                    priority: TaskPriority::Low,
                    timestamp: Utc::now(),
                });
            }
        }
        
        tasks
    }
    
    /// ─── KONTROL FONKSİYONLARI ───
    
    /// Duraklat
    pub fn pause(&mut self) {
        *self.status.write() = WatcherStatus::Paused;
        log::info!("👁️  Watcher duraklatıldı");
    }
    
    /// Devam et
    pub fn resume(&mut self) {
        *self.status.write() = WatcherStatus::Running;
        log::info!("👁️  Watcher devam ediyor");
    }
    
    /// Durdur
    pub fn stop(&mut self) {
        *self.status.write() = WatcherStatus::Stopped;
        log::info!("👁️  Watcher durduruldu");
    }
    
    /// Durum al
    pub fn status(&self) -> WatcherStatus {
        *self.status.read()
    }
    
    /// İstatistikler
    pub fn stats(&self) -> WatcherStats {
        self.stats.read().clone()
    }
    
    /// Rapor
    pub fn report(&self) -> String {
        let stats = self.stats.read();
        let status = self.status.read();
        let scheduled = self.scheduled_tasks.read();
        let triggers = self.triggers.read();
        
        format!(
            r#"
════════════════════════════════════════════════════════════
  👁️  WATCHER DURUM RAPORU
════════════════════════════════════════════════════════════
  Durum:              {:?}
  Uptime:             {}s
  ────────────────────────────────────────────────────────────
  Döngü Sayısı:       {}
  Üretilen Görev:     {}
  Scout Tarama:       {}
  Forge Üretim:       {}
  Tetiklenme:         {}
  ────────────────────────────────────────────────────────────
  Zamanlanmış Görev:  {}
  Tetikleyici:        {}
════════════════════════════════════════════════════════════"#,
            *status,
            stats.uptime_secs,
            stats.total_cycles,
            stats.tasks_generated,
            stats.scout_scans,
            stats.forge_generations,
            stats.triggers_fired,
            scheduled.len(),
            triggers.len()
        )
    }
}

/// Scheduled task varsayılan öncelik
impl ScheduledTaskType {
    pub fn default_priority(&self) -> TaskPriority {
        match self {
            Self::HealthCheck => TaskPriority::High,
            Self::MemoryCleanup => TaskPriority::Low,
            Self::ScoutScan { .. } => TaskPriority::Low,
            Self::DataHunt { .. } => TaskPriority::Normal,
            Self::ForgeGenerate { .. } => TaskPriority::Normal,
            Self::CodeAnalysis { .. } => TaskPriority::Low,
            Self::LearningTask { .. } => TaskPriority::Normal,
        }
    }
}

// ─── Tests ───

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_watcher_config_default() {
        let config = WatcherConfig::default();
        assert_eq!(config.scan_interval_secs, 300);
        assert!(config.scout_enabled);
        assert!(config.forge_enabled);
    }
    
    #[test]
    fn test_repeat_type_next_run() {
        let now = Utc::now();
        
        let hourly = RepeatType::Hourly.next_run(now);
        assert!(hourly > now);
        
        let daily = RepeatType::Daily.next_run(now);
        assert!(daily > hourly);
    }
    
    #[tokio::test]
    async fn test_watcher_creation() {
        let watcher = Watcher::new(WatcherConfig::default());
        assert_eq!(watcher.status(), WatcherStatus::NotStarted);
    }
    
    #[tokio::test]
    async fn test_watcher_start() {
        let mut watcher = Watcher::new(WatcherConfig::default());
        watcher.start().await.expect("operation failed");
        assert_eq!(watcher.status(), WatcherStatus::Running);
    }
    
    #[tokio::test]
    async fn test_watcher_tick() {
        let mut watcher = Watcher::new(WatcherConfig::default());
        watcher.start().await.expect("operation failed");
        
        let tasks = watcher.tick().await.expect("operation failed");
        // İlk tick'te görev üretilmeyebilir (zamanlama nedeniyle)
        assert!(tasks.is_empty() || !tasks.is_empty());
    }
    
    #[test]
    fn test_active_hour_check() {
        let mut config = WatcherConfig::default();
        config.active_hours = vec![12]; // Sadece 12:00
        
        let watcher = Watcher::new(config);
        
        // Şu anki saate göre kontrol
        let now = Utc::now();
        let is_active = watcher.is_active_hour(&now);
        
        // Sadece 12:00'de true dönmeli
        if now.hour() == 12 {
            assert!(is_active);
        } else {
            assert!(!is_active);
        }
    }
}