//! ═══════════════════════════════════════════════════════════════════════════════
//!  TIME-BASED RULES - ZAMAN BAZLI KURALLAR
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Masaüstü kontrolü için zaman bazlı kısıtlamalar.
//! 
//! ═──────────────────────────────────────────────────────────────────────────────
//!  KURALLAR:
//!  ────────────────
//!  Mesai Saatleri   → 09:00 - 18:00 (ayarlanabilir)
//!  Hafta İçi        → Pazartesi - Cuma
//!  Gece Modu        → 22:00 - 06:00 (düşük aktivite)
//!  Tatiller         → Özel gün listesi
//! ═──────────────────────────────────────────────────────────────────────────────

use crate::error::{HandsError, HandsResult};
use chrono::{DateTime, Utc, NaiveTime, NaiveDate, Weekday, Timelike, Datelike};
use serde::{Deserialize, Serialize};

// ───────────────────────────────────────────────────────────────────────────────
//  ÇALIŞMA MODU
// ───────────────────────────────────────────────────────────────────────────────

/// Çalışma modu
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkMode {
    /// Normal çalışma saati
    NormalWork,
    /// Mesai sonrası
    AfterHours,
    /// Gece modu (düşük aktivite)
    NightMode,
    /// Hafta sonu
    Weekend,
    /// Tatil
    Holiday,
    /// İzin verilmeyen zaman
    Blocked,
}

impl WorkMode {
    /// Mod açıklayıcı metni
    pub fn description(&self) -> &'static str {
        match self {
            WorkMode::NormalWork => "Normal çalışma saati",
            WorkMode::AfterHours => "Mesai sonrası",
            WorkMode::NightMode => "Gece modu - düşük aktivite",
            WorkMode::Weekend => "Hafta sonu",
            WorkMode::Holiday => "Tatil günü",
            WorkMode::Blocked => "İzin verilmeyen zaman",
        }
    }
    
    /// Bu modda çalışma izni var mı?
    pub fn is_allowed(&self) -> bool {
        matches!(self, WorkMode::NormalWork | WorkMode::AfterHours | WorkMode::NightMode)
    }
    
    /// Bu modda düşük aktivite gerekli mi?
    pub fn requires_reduced_activity(&self) -> bool {
        matches!(self, WorkMode::NightMode)
    }
}

// ───────────────────────────────────────────────────────────────────────────────
//  ZAMAN KURALLARI
// ───────────────────────────────────────────────────────────────────────────────

/// Zaman bazlı kurallar
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRules {
    /// Kurallar aktif mi?
    pub enabled: bool,
    
    // ─── Çalışma Saatleri ───
    /// Çalışma başlangıç saati (örn: 09:00)
    pub work_start: NaiveTime,
    /// Çalışma bitiş saati (örn: 18:00)
    pub work_end: NaiveTime,
    
    // ─── İzin Verilen Günler ───
    /// İzin verilen günler
    pub allowed_days: Vec<Weekday>,
    
    // ─── Gece Modu ───
    /// Gece modu başlangıç saati (örn: 22:00)
    pub night_mode_start: NaiveTime,
    /// Gece modu bitiş saati (örn: 06:00)
    pub night_mode_end: NaiveTime,
    /// Gece modunda düşük aktivite
    pub night_mode_reduced: bool,
    /// Gece modu izinli mi?
    pub night_mode_allowed: bool,
    
    // ─── Mesai Sonrası ───
    /// Mesai sonrası çalışma izni
    pub after_hours_allowed: bool,
    /// Mesai sonrası maksimum süre (saat)
    pub after_hours_max_hours: u32,
    
    // ─── Hafta Sonu ───
    /// Hafta sonu çalışma izni
    pub weekend_allowed: bool,
    
    // ─── Özel Günler ───
    /// Tatil günleri listesi
    pub holidays: Vec<NaiveDate>,
    /// Tatillerde çalışma izni
    pub holiday_allowed: bool,
    
    // ─── Tamamen Engelle ───
    /// Tamamen engellenen zaman aralıkları
    pub blocked_periods: Vec<BlockedPeriod>,
}

/// Engellenen zaman aralığı
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockedPeriod {
    /// Başlangıç zamanı
    pub start: NaiveTime,
    /// Bitiş zamanı
    pub end: NaiveTime,
    /// Neden
    pub reason: String,
    /// Aktif mi?
    pub enabled: bool,
}

impl BlockedPeriod {
    pub fn new(start: NaiveTime, end: NaiveTime, reason: &str) -> Self {
        Self {
            start,
            end,
            reason: reason.to_string(),
            enabled: true,
        }
    }
    
    /// Verilen zaman bu aralıkta mı?
    pub fn contains(&self, time: NaiveTime) -> bool {
        if !self.enabled {
            return false;
        }
        
        // Aynı gün içindeyse
        if self.start <= self.end {
            time >= self.start && time <= self.end
        } else {
            // Gece yarısını geçen aralık (örn: 23:00 - 02:00)
            time >= self.start || time <= self.end
        }
    }
}

impl Default for TimeRules {
    fn default() -> Self {
        Self {
            enabled: true,
            work_start: NaiveTime::from_hms_opt(9, 0, 0).unwrap(),
            work_end: NaiveTime::from_hms_opt(18, 0, 0).unwrap(),
            allowed_days: vec![
                Weekday::Mon,
                Weekday::Tue,
                Weekday::Wed,
                Weekday::Thu,
                Weekday::Fri,
            ],
            night_mode_start: NaiveTime::from_hms_opt(22, 0, 0).unwrap(),
            night_mode_end: NaiveTime::from_hms_opt(6, 0, 0).unwrap(),
            night_mode_reduced: true,
            night_mode_allowed: true,
            after_hours_allowed: true,
            after_hours_max_hours: 2,
            weekend_allowed: false,
            holidays: Vec::new(),
            holiday_allowed: false,
            blocked_periods: Vec::new(),
        }
    }
}

impl TimeRules {
    /// Yeni zaman kuralları oluştur
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Kuralları devre dışı bırak (her zaman izin ver)
    pub fn disabled() -> Self {
        let mut rules = Self::default();
        rules.enabled = false;
        rules
    }
    
    /// Katı mod - sadece çalışma saatleri
    pub fn strict() -> Self {
        Self::default()
    }
    
    /// Esnek mod - hafta sonu ve mesai sonrası da açık
    pub fn flexible() -> Self {
        Self {
            enabled: true,
            work_start: NaiveTime::from_hms_opt(8, 0, 0).unwrap(),
            work_end: NaiveTime::from_hms_opt(20, 0, 0).unwrap(),
            allowed_days: vec![
                Weekday::Mon, Weekday::Tue, Weekday::Wed, 
                Weekday::Thu, Weekday::Fri, Weekday::Sat,
            ],
            weekend_allowed: true,
            after_hours_allowed: true,
            after_hours_max_hours: 4,
            ..Default::default()
        }
    }
    
    // ─── KONTROL METOTLARI ───
    
    /// Şu an çalışma izni var mı?
    pub fn is_allowed_now(&self) -> bool {
        self.check_time(Utc::now()).is_allowed()
    }
    
    /// Verilen zamanda çalışma modunu belirle
    pub fn check_time(&self, datetime: DateTime<Utc>) -> WorkMode {
        if !self.enabled {
            return WorkMode::NormalWork;
        }
        
        let local_time = datetime.time();
        let weekday = datetime.weekday();
        let date = datetime.date_naive();
        
        // 1. Engellenen dönem kontrolü
        for period in &self.blocked_periods {
            if period.contains(local_time) {
                return WorkMode::Blocked;
            }
        }
        
        // 2. Tatil kontrolü
        if self.holidays.contains(&date) {
            if !self.holiday_allowed {
                return WorkMode::Holiday;
            }
            // Tatilde çalışma izni varsa devam et
        }
        
        // 3. Hafta sonu kontrolü
        if !self.allowed_days.contains(&weekday) {
            if !self.weekend_allowed {
                return WorkMode::Weekend;
            }
            // Hafta sonu çalışma izni varsa devam et
        }
        
        // 4. Gece modu kontrolü
        if self.is_night_mode(local_time) {
            if !self.night_mode_allowed {
                return WorkMode::Blocked;
            }
            return WorkMode::NightMode;
        }
        
        // 5. Çalışma saati kontrolü
        if self.is_work_hours(local_time) {
            return WorkMode::NormalWork;
        }
        
        // 6. Mesai sonrası
        if self.after_hours_allowed {
            return WorkMode::AfterHours;
        }
        
        WorkMode::Blocked
    }
    
    /// Verilen zaman çalışma saati içinde mi?
    fn is_work_hours(&self, time: NaiveTime) -> bool {
        time >= self.work_start && time <= self.work_end
    }
    
    /// Verilen zaman gece modu içinde mi?
    fn is_night_mode(&self, time: NaiveTime) -> bool {
        // Gece modu gece yarısını geçebilir (örn: 22:00 - 06:00)
        if self.night_mode_start <= self.night_mode_end {
            time >= self.night_mode_start && time <= self.night_mode_end
        } else {
            time >= self.night_mode_start || time <= self.night_mode_end
        }
    }
    
    /// Şu anki çalışma modunu getir
    pub fn get_current_mode(&self) -> WorkMode {
        self.check_time(Utc::now())
    }
    
    /// Bir sonraki izin verilen zamanı getir
    pub fn get_next_allowed_time(&self) -> Option<DateTime<Utc>> {
        if !self.enabled {
            return None; // Her zaman izinli
        }
        
        let now = Utc::now();
        
        // Bugün için kontrol
        let today_work_start = now.with_hour(self.work_start.hour() as u32)?
            .with_minute(self.work_start.minute() as u32)?
            .with_second(0)?;
        
        if now < today_work_start && self.allowed_days.contains(&now.weekday()) {
            return Some(today_work_start);
        }
        
        // Sonraki izin verilen günü bul
        for days_ahead in 1..=7 {
            let next_day = now + chrono::Duration::days(days_ahead);
            if self.allowed_days.contains(&next_day.weekday()) {
                return next_day
                    .with_hour(self.work_start.hour() as u32)?
                    .with_minute(self.work_start.minute() as u32)?
                    .with_second(0)
                    .into();
            }
        }
        
        None
    }
    
    // ─── DÜZENLEME METOTLARI ───
    
    /// Çalışma saatlerini ayarla
    pub fn set_work_hours(&mut self, start: NaiveTime, end: NaiveTime) {
        self.work_start = start;
        self.work_end = end;
        log::info!(
            "⏰ TIME: Çalışma saatleri güncellendi → {} - {}",
            start.format("%H:%M"),
            end.format("%H:%M")
        );
    }
    
    /// Gece modu ayarla
    pub fn set_night_mode(&mut self, start: NaiveTime, end: NaiveTime, allowed: bool, reduced: bool) {
        self.night_mode_start = start;
        self.night_mode_end = end;
        self.night_mode_allowed = allowed;
        self.night_mode_reduced = reduced;
        log::info!(
            "⏰ TIME: Gece modu güncellendi → {} - {} (izinli: {}, düşük aktivite: {})",
            start.format("%H:%M"),
            end.format("%H:%M"),
            allowed,
            reduced
        );
    }
    
    /// Tatil ekle
    pub fn add_holiday(&mut self, date: NaiveDate) {
        if !self.holidays.contains(&date) {
            self.holidays.push(date);
            log::info!("⏰ TIME: Tatil eklendi → {}", date);
        }
    }
    
    /// Tatil kaldır
    pub fn remove_holiday(&mut self, date: NaiveDate) -> bool {
        if let Some(pos) = self.holidays.iter().position(|d| *d == date) {
            self.holidays.remove(pos);
            log::info!("⏰ TIME: Tatil kaldırıldı → {}", date);
            return true;
        }
        false
    }
    
    /// Engellenen dönem ekle
    pub fn add_blocked_period(&mut self, period: BlockedPeriod) {
        log::info!(
            "⏰ TIME: Engellenen dönem eklendi → {} - {} ({})",
            period.start.format("%H:%M"),
            period.end.format("%H:%M"),
            period.reason
        );
        self.blocked_periods.push(period);
    }
    
    /// İzin verilen günleri ayarla
    pub fn set_allowed_days(&mut self, days: Vec<Weekday>) {
        self.allowed_days = days;
        log::info!("⏰ TIME: İzin verilen günler güncellendi");
    }
    
    /// Hafta sonu iznini ayarla
    pub fn set_weekend_allowed(&mut self, allowed: bool) {
        self.weekend_allowed = allowed;
        log::info!("⏰ TIME: Hafta sonu çalışma → {}", if allowed { "izni verildi" } else { "engellendi" });
    }
}

// ───────────────────────────────────────────────────────────────────────────────
//  TIME RULES MANAGER
// ───────────────────────────────────────────────────────────────────────────────

/// Zaman kuralları yöneticisi
#[derive(Debug, Clone)]
pub struct TimeRulesManager {
    rules: TimeRules,
    /// Son kontrol sonucu
    last_check: Option<(DateTime<Utc>, WorkMode)>,
    /// İhlal sayısı
    violation_count: u64,
}

impl TimeRulesManager {
    /// Yeni yönetici oluştur
    pub fn new(rules: TimeRules) -> Self {
        Self {
            rules,
            last_check: None,
            violation_count: 0,
        }
    }
    
    /// Varsayılan kurallarla yönetici oluştur
    pub fn default_rules() -> Self {
        Self::new(TimeRules::default())
    }
    
    /// Şu anki zamanı kontrol et
    pub fn check(&mut self) -> HandsResult<WorkMode> {
        let now = Utc::now();
        let mode = self.rules.check_time(now);
        
        self.last_check = Some((now, mode));
        
        if !mode.is_allowed() {
            self.violation_count += 1;
            log::warn!(
                "⏰ TIME: Zaman kuralı ihlali → {} (#{})",
                mode.description(),
                self.violation_count
            );
            return Err(HandsError::TimeRuleViolation(format!(
                "OASIS-HANDS TIME: {} - Çalışma izni yok",
                mode.description()
            )));
        }
        
        Ok(mode)
    }
    
    /// Zaman kurallarını getir
    pub fn rules(&self) -> &TimeRules {
        &self.rules
    }
    
    /// Zaman kurallarını değiştir
    pub fn rules_mut(&mut self) -> &mut TimeRules {
        &mut self.rules
    }
    
    /// Son kontrol sonucunu getir
    pub fn last_check(&self) -> Option<(DateTime<Utc>, WorkMode)> {
        self.last_check
    }
    
    /// İhlal sayısını getir
    pub fn violation_count(&self) -> u64 {
        self.violation_count
    }
    
    /// İstatistikleri sıfırla
    pub fn reset_stats(&mut self) {
        self.violation_count = 0;
        self.last_check = None;
        log::info!("⏰ TIME: İstatistikler sıfırlandı");
    }
    
    /// Zaman raporu oluştur
    pub fn report(&self) -> TimeRulesReport {
        let current_mode = self.rules.get_current_mode();
        let next_allowed = self.rules.get_next_allowed_time();
        
        TimeRulesReport {
            enabled: self.rules.enabled,
            current_mode,
            is_allowed_now: current_mode.is_allowed(),
            next_allowed_time: next_allowed,
            violation_count: self.violation_count,
            work_hours: format!(
                "{} - {}",
                self.rules.work_start.format("%H:%M"),
                self.rules.work_end.format("%H:%M")
            ),
            allowed_days: self.rules.allowed_days.len(),
            holidays_count: self.rules.holidays.len(),
            blocked_periods_count: self.rules.blocked_periods.len(),
        }
    }
}

/// Zaman kuralları raporu
#[derive(Debug, Clone)]
pub struct TimeRulesReport {
    pub enabled: bool,
    pub current_mode: WorkMode,
    pub is_allowed_now: bool,
    pub next_allowed_time: Option<DateTime<Utc>>,
    pub violation_count: u64,
    pub work_hours: String,
    pub allowed_days: usize,
    pub holidays_count: usize,
    pub blocked_periods_count: usize,
}

impl std::fmt::Display for TimeRulesReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "╔════════════════════════════════════════════╗")?;
        writeln!(f, "║         TIME RULES DURUM RAPORU            ║")?;
        writeln!(f, "╠════════════════════════════════════════════╣")?;
        writeln!(f, "║ Aktif: {:<36} ║", if self.enabled { "Evet" } else { "Hayır" })?;
        writeln!(f, "║ Mevcut Mod: {:<31} ║", self.current_mode.description())?;
        writeln!(f, "║ İzin: {:<37} ║", if self.is_allowed_now { "✅ İzinli" } else { "❌ Yasak" })?;
        writeln!(f, "║ Çalışma Saatleri: {:<23} ║", self.work_hours)?;
        writeln!(f, "║ İzinli Günler: {:<27} ║", self.allowed_days)?;
        writeln!(f, "║ Tatiller: {:<34} ║", self.holidays_count)?;
        writeln!(f, "║ Engellenen Dönemler: {:<21} ║", self.blocked_periods_count)?;
        writeln!(f, "║ İhlal Sayısı: {:<29} ║", self.violation_count)?;
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
    fn test_work_mode_is_allowed() {
        assert!(WorkMode::NormalWork.is_allowed());
        assert!(WorkMode::AfterHours.is_allowed());
        assert!(WorkMode::NightMode.is_allowed());
        assert!(!WorkMode::Weekend.is_allowed());
        assert!(!WorkMode::Holiday.is_allowed());
        assert!(!WorkMode::Blocked.is_allowed());
    }
    
    #[test]
    fn test_work_mode_reduced_activity() {
        assert!(WorkMode::NightMode.requires_reduced_activity());
        assert!(!WorkMode::NormalWork.requires_reduced_activity());
    }
    
    #[test]
    fn test_time_rules_default() {
        let rules = TimeRules::default();
        assert!(rules.enabled);
        assert_eq!(rules.allowed_days.len(), 5);
        assert!(!rules.weekend_allowed);
    }
    
    #[test]
    fn test_time_rules_disabled() {
        let rules = TimeRules::disabled();
        assert!(!rules.enabled);
        
        // Devre dışı iken her zaman izinli
        let mode = rules.check_time(Utc::now());
        assert_eq!(mode, WorkMode::NormalWork);
    }
    
    #[test]
    fn test_time_rules_flexible() {
        let rules = TimeRules::flexible();
        assert!(rules.weekend_allowed);
        assert!(rules.after_hours_allowed);
        assert_eq!(rules.allowed_days.len(), 6); // Mon-Sat
    }
    
    #[test]
    fn test_blocked_period_creation() {
        let period = BlockedPeriod::new(
            NaiveTime::from_hms_opt(12, 0, 0).unwrap(),
            NaiveTime::from_hms_opt(13, 0, 0).unwrap(),
            "Öğle tatili"
        );
        
        assert!(period.enabled);
        assert_eq!(period.reason, "Öğle tatili");
    }
    
    #[test]
    fn test_blocked_period_contains() {
        let period = BlockedPeriod::new(
            NaiveTime::from_hms_opt(12, 0, 0).unwrap(),
            NaiveTime::from_hms_opt(13, 0, 0).unwrap(),
            "Test"
        );
        
        // İçeride
        assert!(period.contains(NaiveTime::from_hms_opt(12, 30, 0).unwrap()));
        
        // Dışarıda
        assert!(!period.contains(NaiveTime::from_hms_opt(11, 0, 0).unwrap()));
        assert!(!period.contains(NaiveTime::from_hms_opt(14, 0, 0).unwrap()));
    }
    
    #[test]
    fn test_blocked_period_midnight_crossing() {
        // 23:00 - 02:00 arası
        let period = BlockedPeriod::new(
            NaiveTime::from_hms_opt(23, 0, 0).unwrap(),
            NaiveTime::from_hms_opt(2, 0, 0).unwrap(),
            "Gece"
        );
        
        // İçeride
        assert!(period.contains(NaiveTime::from_hms_opt(23, 30, 0).unwrap()));
        assert!(period.contains(NaiveTime::from_hms_opt(0, 30, 0).unwrap()));
        assert!(period.contains(NaiveTime::from_hms_opt(1, 30, 0).unwrap()));
        
        // Dışarıda
        assert!(!period.contains(NaiveTime::from_hms_opt(10, 0, 0).unwrap()));
        assert!(!period.contains(NaiveTime::from_hms_opt(22, 0, 0).unwrap()));
    }
    
    #[test]
    fn test_set_work_hours() {
        let mut rules = TimeRules::default();
        rules.set_work_hours(
            NaiveTime::from_hms_opt(8, 0, 0).unwrap(),
            NaiveTime::from_hms_opt(17, 0, 0).unwrap()
        );
        
        assert_eq!(rules.work_start, NaiveTime::from_hms_opt(8, 0, 0).unwrap());
        assert_eq!(rules.work_end, NaiveTime::from_hms_opt(17, 0, 0).unwrap());
    }
    
    #[test]
    fn test_add_holiday() {
        let mut rules = TimeRules::default();
        let date = NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();
        
        rules.add_holiday(date);
        assert_eq!(rules.holidays.len(), 1);
        
        // Aynı tarihi tekrar ekleme
        rules.add_holiday(date);
        assert_eq!(rules.holidays.len(), 1);
    }
    
    #[test]
    fn test_remove_holiday() {
        let mut rules = TimeRules::default();
        let date = NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();
        
        rules.add_holiday(date);
        assert_eq!(rules.holidays.len(), 1);
        
        let removed = rules.remove_holiday(date);
        assert!(removed);
        assert_eq!(rules.holidays.len(), 0);
        
        // Tekrar kaldırmayı dene
        let removed_again = rules.remove_holiday(date);
        assert!(!removed_again);
    }
    
    #[test]
    fn test_add_blocked_period() {
        let mut rules = TimeRules::default();
        let period = BlockedPeriod::new(
            NaiveTime::from_hms_opt(12, 0, 0).unwrap(),
            NaiveTime::from_hms_opt(13, 0, 0).unwrap(),
            "Öğle"
        );
        
        rules.add_blocked_period(period);
        assert_eq!(rules.blocked_periods.len(), 1);
    }
    
    #[test]
    fn test_time_rules_manager_creation() {
        let manager = TimeRulesManager::default_rules();
        assert!(manager.rules().enabled);
        assert_eq!(manager.violation_count(), 0);
    }
    
    #[test]
    fn test_time_rules_manager_report() {
        let manager = TimeRulesManager::default_rules();
        let report = manager.report();
        
        assert!(report.enabled);
        assert_eq!(report.violation_count, 0);
    }
    
    #[test]
    fn test_time_rules_manager_reset() {
        let mut manager = TimeRulesManager::default_rules();
        manager.violation_count = 5;
        
        manager.reset_stats();
        assert_eq!(manager.violation_count(), 0);
    }
    
    #[test]
    fn test_set_allowed_days() {
        let mut rules = TimeRules::default();
        rules.set_allowed_days(vec![Weekday::Mon, Weekday::Wed, Weekday::Fri]);
        
        assert_eq!(rules.allowed_days.len(), 3);
        assert!(rules.allowed_days.contains(&Weekday::Mon));
        assert!(!rules.allowed_days.contains(&Weekday::Tue));
    }
    
    #[test]
    fn test_set_weekend_allowed() {
        let mut rules = TimeRules::default();
        assert!(!rules.weekend_allowed);
        
        rules.set_weekend_allowed(true);
        assert!(rules.weekend_allowed);
    }
    
    #[test]
    fn test_time_rules_report_display() {
        let manager = TimeRulesManager::default_rules();
        let report = manager.report();
        let output = format!("{}", report);
        
        assert!(output.contains("TIME RULES DURUM RAPORU"));
        assert!(output.contains("Çalışma Saatleri"));
    }
}
