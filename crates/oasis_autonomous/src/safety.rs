//! ═══════════════════════════════════════════════════════════════════════════════
//!  SAFETY SYSTEM - Güvenlik Katmanı
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Agent'ın güvenli çalışmasını sağlayan koruma sistemi.
//!
//! GÜVENLİK KATMANLARI:
//! ────────────────────
//! 1. Forbidden Regions   → Yasaklı ekran bölgeleri
//! 2. Action Validation   → Aksiyon doğrulama
//! 3. Rate Limiting       → Hız sınırlama
//! 4. Human Approval      → İnsan onayı
//! 5. Emergency Stop      → Acil durum durdurması
//! 6. Audit Logging       → Denetim kaydı
//!
//! ┌─────────────────────────────────────────────────────────────────────────┐
//! │                        SAFETY PIPELINE                                  │
//! │                                                                          │
//! │   Action ──► Validate ──► Rate Limit ──► Human Gate ──► Execute        │
//! │                 │              │              │                         │
//! │                 ▼              ▼              ▼                         │
//! │             [FORBIDDEN?]   [TOO FAST?]   [NEED APPROVAL?]              │
//! │                 │              │              │                         │
//! │                 └──────────────┴──────────────┘                         │
//! │                                │                                        │
//! │                                ▼                                        │
//! │                           [BLOCK]                                       │
//! └─────────────────────────────────────────────────────────────────────────┘

use crate::error::{AutonomousError, AutonomousResult};
use crate::Action;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::time::{Duration, Instant};

// ═══════════════════════════════════════════════════════════════════════════════
//  SAFETY CONFIG
// ═══════════════════════════════════════════════════════════════════════════════

/// Güvenlik yapılandırması
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyConfig {
    /// İnsan onayı gerekli mi?
    pub require_human_approval: bool,
    /// Kritik aksiyonlar için her zaman onay iste
    pub always_require_approval_for_critical: bool,
    /// Maksimum aksiyon/dakika
    pub max_actions_per_minute: u32,
    /// Maksimum hata sayısı
    pub max_errors_before_stop: usize,
    /// Yasaklı bölgeler
    pub forbidden_regions: Vec<ForbiddenRegion>,
    /// İzin verilen uygulamalar (boş = tümü)
    pub allowed_applications: Vec<String>,
    /// Yasaklı uygulamalar
    pub forbidden_applications: Vec<String>,
    /// Yasaklı URL pattern'leri
    pub forbidden_url_patterns: Vec<String>,
    /// Acil durum durdurması aktif mi?
    pub emergency_stop_enabled: bool,
    /// Audit log aktif mi?
    pub audit_logging: bool,
    /// Maksimum tek seferde çalışma süresi (saniye)
    pub max_session_duration_secs: u64,
    /// Auto-pause aktif mi?
    pub auto_pause_enabled: bool,
    /// Auto-pause süresi (saniye)
    pub auto_pause_interval_secs: u64,
}

impl Default for SafetyConfig {
    fn default() -> Self {
        Self {
            require_human_approval: false,
            always_require_approval_for_critical: true,
            max_actions_per_minute: 120,
            max_errors_before_stop: 10,
            forbidden_regions: vec![],
            allowed_applications: vec![],
            forbidden_applications: vec![],
            forbidden_url_patterns: vec![],
            emergency_stop_enabled: true,
            audit_logging: true,
            max_session_duration_secs: 3600, // 1 saat
            auto_pause_enabled: false,
            auto_pause_interval_secs: 300, // 5 dakika
        }
    }
}

impl SafetyConfig {
    /// Geliştirici modu (daha esnek)
    pub fn developer() -> Self {
        Self {
            require_human_approval: false,
            always_require_approval_for_critical: false,
            max_actions_per_minute: 300,
            max_errors_before_stop: 50,
            ..Default::default()
        }
    }
    
    /// Sıkı mod (daha güvenli)
    pub fn strict() -> Self {
        Self {
            require_human_approval: true,
            always_require_approval_for_critical: true,
            max_actions_per_minute: 30,
            max_errors_before_stop: 3,
            emergency_stop_enabled: true,
            audit_logging: true,
            ..Default::default()
        }
    }
    
    /// Production modu
    pub fn production() -> Self {
        Self {
            require_human_approval: true,
            always_require_approval_for_critical: true,
            max_actions_per_minute: 60,
            max_errors_before_stop: 5,
            emergency_stop_enabled: true,
            audit_logging: true,
            max_session_duration_secs: 1800, // 30 dakika
            auto_pause_enabled: true,
            auto_pause_interval_secs: 600, // 10 dakika
            ..Default::default()
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  FORBIDDEN REGION
// ═══════════════════════════════════════════════════════════════════════════════

/// Yasaklı bölge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForbiddenRegion {
    /// Bölge adı
    pub name: String,
    /// Bölge sınırları
    pub bounds: crate::screen::Rectangle,
    /// Neden yasaklı?
    pub reason: String,
    /// Seviye
    pub level: ForbiddenLevel,
}

/// Yasak seviyesi
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ForbiddenLevel {
    /// Uyarı ver ama izin ver
    Warning,
    /// Tamamen yasak
    Block,
    /// Onay gerektir
    RequiresApproval,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  STOP CONDITION
// ═══════════════════════════════════════════════════════════════════════════════

/// Durdurma koşulu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StopCondition {
    /// Hata sayısı aşıldı
    ErrorCountExceeded { max: usize, current: usize },
    /// Timeout aşıldı
    TimeoutExceeded { max_secs: u64, elapsed_secs: u64 },
    /// Döngü tespit edildi
    LoopDetected { pattern: String, repeat_count: usize },
    /// Yasak bölgeye erişim
    ForbiddenRegionAccess { region: String },
    /// Kritik aksiyon onayı gerekli
    CriticalActionNeedsApproval { action: String },
    /// Kullanıcı durdurdu
    UserInterrupted,
    /// Maksimum aksiyon sayısı aşıldı
    MaxActionsExceeded { max: usize, current: usize },
    /// Rate limit aşıldı
    RateLimitExceeded { max_per_minute: u32 },
    /// Session süresi aşıldı
    SessionDurationExceeded { max_secs: u64 },
    /// Sistem kaynak yetersiz
    SystemResourceLow { resource: String },
}

impl std::fmt::Display for StopCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StopCondition::ErrorCountExceeded { max, current } => {
                write!(f, "Hata sayısı aşıldı: {}/{}", current, max)
            }
            StopCondition::TimeoutExceeded { max_secs, elapsed_secs } => {
                write!(f, "Timeout: {}s/{}s", elapsed_secs, max_secs)
            }
            StopCondition::LoopDetected { pattern, repeat_count } => {
                write!(f, "Döngü tespit: '{}' x{}", pattern, repeat_count)
            }
            StopCondition::ForbiddenRegionAccess { region } => {
                write!(f, "Yasak bölge: {}", region)
            }
            StopCondition::CriticalActionNeedsApproval { action } => {
                write!(f, "Onay gerekli: {}", action)
            }
            StopCondition::UserInterrupted => {
                write!(f, "Kullanıcı durdurdu")
            }
            StopCondition::MaxActionsExceeded { max, current } => {
                write!(f, "Maks aksiyon: {}/{}", current, max)
            }
            StopCondition::RateLimitExceeded { max_per_minute } => {
                write!(f, "Rate limit: {}/dk", max_per_minute)
            }
            StopCondition::SessionDurationExceeded { max_secs } => {
                write!(f, "Session süresi: {}s", max_secs)
            }
            StopCondition::SystemResourceLow { resource } => {
                write!(f, "Düşük kaynak: {}", resource)
            }
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  ACTION TYPE CLASSIFICATION
// ═══════════════════════════════════════════════════════════════════════════════

/// Aksiyon kritiklik seviyesi
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActionCriticality {
    /// Normal
    Normal,
    /// Dikkatli
    Moderate,
    /// Kritik
    Critical,
    /// Çok kritik
    VeryCritical,
}

impl Action {
    /// Aksiyon kritikliğini belirle
    pub fn criticality(&self) -> ActionCriticality {
        match self {
            // Çok kritik
            Action::Custom { name, .. } if name == "delete_files" => ActionCriticality::VeryCritical,
            Action::Custom { name, .. } if name == "send_email" => ActionCriticality::VeryCritical,
            Action::Custom { name, .. } if name == "execute_shell" => ActionCriticality::VeryCritical,
            
            // Kritik
            Action::KeyShortcut { modifiers, .. } if modifiers.contains(&crate::Key::Ctrl) => {
                ActionCriticality::Critical // Ctrl+something shortcuts
            }
            Action::MouseDrag { .. } => ActionCriticality::Critical,
            Action::TypeText { text, .. } if text.len() > 100 => ActionCriticality::Critical,
            
            // Orta
            Action::MouseClick { .. } => ActionCriticality::Moderate,
            Action::TypeText { .. } => ActionCriticality::Moderate,
            Action::BrowserClick { .. } => ActionCriticality::Moderate,
            
            // Normal
            Action::MouseMove { .. } => ActionCriticality::Normal,
            Action::MouseScroll { .. } => ActionCriticality::Normal,
            Action::KeyPress { key } if matches!(key, crate::Key::ArrowUp | crate::Key::ArrowDown) => {
                ActionCriticality::Normal
            }
            Action::NoOp => ActionCriticality::Normal,
            
            _ => ActionCriticality::Moderate,
        }
    }
    
    /// Açıklama üret
    pub fn describe(&self) -> String {
        match self {
            Action::MouseMove { x, y } => format!("Fare hareket: ({}, {})", x, y),
            Action::MouseClick { button, x, y } => format!("Tıklama {:?}: ({}, {})", button, x, y),
            Action::MouseDrag { from, to } => format!("Sürükle: {:?} → {:?}", from, to),
            Action::MouseScroll { delta_x, delta_y } => format!("Kaydır: x={}, y={}", delta_x, delta_y),
            Action::KeyPress { key } => format!("Tuş: {:?}", key),
            Action::KeyShortcut { modifiers, key } => {
                let mods: String = modifiers.iter()
                    .map(|k| format!("{:?}", k))
                    .collect::<Vec<_>>()
                    .join("+");
                format!("Kısayol: {}+{:?}", mods, key)
            }
            Action::TypeText { text, .. } => {
                let preview = text.chars().take(30).collect::<String>();
                if text.len() > 30 {
                    format!("Yaz: \"{}...\"", preview)
                } else {
                    format!("Yaz: \"{}\"", preview)
                }
            }
            Action::BrowserNavigate { url } => format!("URL: {}", url),
            Action::BrowserClick { selector } => format!("Click: {}", selector),
            Action::BrowserType { selector, text } => format!("Type '{}' → {}", text, selector),
            Action::Composite { actions } => format!("Kompozit: {} aksiyon", actions.len()),
            Action::Custom { name, params } => format!("Özel: {} ({} param)", name, params.len()),
            Action::NoOp => "No-op".into(),
            Action::Stop { reason } => format!("Durdur: {}", reason),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  AUDIT LOG
// ═══════════════════════════════════════════════════════════════════════════════

/// Denetim kaydı
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLog {
    /// Kayıt ID
    pub id: String,
    /// Zaman damgası
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Agent ID
    pub agent_id: String,
    /// Aksiyon
    pub action: String,
    /// Sonuç
    pub result: String,
    /// Onaylandı mı?
    pub approved: bool,
    /// Onaylayan (varsa)
    pub approver: Option<String>,
    /// Risk seviyesi
    pub risk_level: ActionCriticality,
    /// Ek veri
    pub metadata: HashMap<String, String>,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  HUMAN APPROVAL GATE
// ═══════════════════════════════════════════════════════════════════════════════

/// İnsan onay kapısı
pub struct HumanApprovalGate {
    /// Bekleyen onaylar
    pending: Arc<RwLock<Vec<PendingApproval>>>,
    /// Onay geçmişi
    history: Vec<ApprovalDecision>,
}

/// Bekleyen onay
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingApproval {
    /// ID
    pub id: String,
    /// Aksiyon açıklaması
    pub action_description: String,
    /// İstek zamanı
    pub requested_at: chrono::DateTime<chrono::Utc>,
    /// Timeout (saniye)
    pub timeout_secs: u64,
    /// Bağlam
    pub context: String,
}

/// Onay kararı
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalDecision {
    /// ID
    pub id: String,
    /// Onaylandı mı?
    pub approved: bool,
    /// Karar zamanı
    pub decided_at: chrono::DateTime<chrono::Utc>,
    /// Red nedeni (varsa)
    pub rejection_reason: Option<String>,
}

impl HumanApprovalGate {
    pub fn new() -> Self {
        Self {
            pending: Arc::new(RwLock::new(Vec::new())),
            history: Vec::new(),
        }
    }
    
    /// Onay iste
    pub async fn request_approval(&mut self, action: &Action, context: &str, timeout_secs: u64) -> AutonomousResult<String> {
        let pending = PendingApproval {
            id: uuid::Uuid::new_v4().to_string(),
            action_description: action.describe(),
            requested_at: chrono::Utc::now(),
            timeout_secs,
            context: context.into(),
        };
        
        let id = pending.id.clone();
        
        self.pending.write().await.push(pending);
        
        log::warn!("🚨 SAFETY: İnsan onayı gerekli → {}", action.describe());
        
        Ok(id)
    }
    
    /// Onay ver
    pub async fn approve(&mut self, id: &str) -> AutonomousResult<()> {
        let mut pending = self.pending.write().await;
        
        if let Some(idx) = pending.iter().position(|p| p.id == id) {
            let _ = pending.remove(idx);
            
            self.history.push(ApprovalDecision {
                id: id.into(),
                approved: true,
                decided_at: chrono::Utc::now(),
                rejection_reason: None,
            });
            
            log::info!("✅ SAFETY: Aksiyon onaylandı → {}", id);
        }
        
        Ok(())
    }
    
    /// Reddet
    pub async fn reject(&mut self, id: &str, reason: &str) -> AutonomousResult<()> {
        let mut pending = self.pending.write().await;
        
        if let Some(idx) = pending.iter().position(|p| p.id == id) {
            let _ = pending.remove(idx);
            
            self.history.push(ApprovalDecision {
                id: id.into(),
                approved: false,
                decided_at: chrono::Utc::now(),
                rejection_reason: Some(reason.into()),
            });
            
            log::warn!("❌ SAFETY: Aksiyon reddedildi → {} ({})", id, reason);
        }
        
        Ok(())
    }
    
    /// Bekleyen onayları kontrol et (timeout için)
    pub async fn check_timeouts(&mut self) -> Vec<String> {
        let mut pending = self.pending.write().await;
        let now = chrono::Utc::now();
        
        let timed_out: Vec<_> = pending.iter()
            .filter(|p| {
                let elapsed = (now - p.requested_at).num_seconds() as u64;
                elapsed > p.timeout_secs
            })
            .map(|p| p.id.clone())
            .collect();
        
        pending.retain(|p| !timed_out.contains(&p.id));
        
        timed_out
    }
    
    /// Bekleyen var mı?
    pub async fn has_pending(&self) -> bool {
        !self.pending.read().await.is_empty()
    }
}

impl Default for HumanApprovalGate {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  SAFETY SYSTEM
// ═══════════════════════════════════════════════════════════════════════════════

/// Güvenlik sistemi
pub struct SafetySystem {
    /// Yapılandırma
    config: SafetyConfig,
    /// İnsan onay kapısı
    approval_gate: HumanApprovalGate,
    /// Rate limiter
    rate_limiter: RateLimiter,
    /// Audit logları
    audit_log: Vec<AuditLog>,
    /// Session başlangıç
    session_start: Option<Instant>,
    /// Aksiyon sayacı
    action_counter: usize,
    /// Hata sayacı
    error_counter: usize,
    /// Emergency stop flag
    emergency_stop: bool,
}

impl SafetySystem {
    pub fn new(config: SafetyConfig) -> Self {
        log::info!("🛡️ SAFETY: Güvenlik sistemi başlatılıyor...");
        
        Self {
            rate_limiter: RateLimiter::new(config.max_actions_per_minute),
            approval_gate: HumanApprovalGate::new(),
            config,
            audit_log: Vec::new(),
            session_start: None,
            action_counter: 0,
            error_counter: 0,
            emergency_stop: false,
        }
    }
    
    /// Session başlat
    pub fn start_session(&mut self) {
        self.session_start = Some(Instant::now());
        self.action_counter = 0;
        self.error_counter = 0;
        self.emergency_stop = false;
        
        log::info!("🛡️ SAFETY: Session başlatıldı");
    }
    
    /// Session sonlandır
    pub fn end_session(&mut self) {
        self.session_start = None;
        log::info!("🛡️ SAFETY: Session sonlandırıldı");
    }
    
    /// Aksiyonu kontrol et
    pub async fn check_action(&mut self, action: &Action) -> AutonomousResult<bool> {
        // 1. Emergency stop kontrolü
        if self.emergency_stop {
            return Err(AutonomousError::SafetyViolation("Emergency stop aktif".into()));
        }
        
        // 2. Session kontrolü
        if let Some(start) = self.session_start {
            let elapsed = start.elapsed().as_secs();
            if elapsed > self.config.max_session_duration_secs {
                return Err(AutonomousError::SafetyViolation(
                    format!("Session süresi aşıldı: {}s", elapsed)
                ));
            }
        }
        
        // 3. Rate limit kontrolü
        if !self.rate_limiter.check() {
            return Err(AutonomousError::SafetyViolation(
                format!("Rate limit aşıldı: {}/dk", self.config.max_actions_per_minute)
            ));
        }
        
        // 4. Yasaklı bölge kontrolü
        self.check_forbidden_regions(action)?;
        
        // 5. Yasaklı uygulama kontrolü
        self.check_forbidden_applications(action)?;
        
        // 6. Yasaklı URL kontrolü
        self.check_forbidden_urls(action)?;
        
        // 7. Kritik aksiyon kontrolü
        if self.config.always_require_approval_for_critical {
            let criticality = action.criticality();
            if matches!(criticality, ActionCriticality::Critical | ActionCriticality::VeryCritical) {
                let id = self.approval_gate.request_approval(
                    action, 
                    "Kritik aksiyon",
                    60
                ).await?;
                
                // Bu gerçek bir sistemde UI'dan gelecek
                // Şimdilik otomatik onayla (geliştirme modu)
                if !self.config.require_human_approval {
                    self.approval_gate.approve(&id).await?;
                } else {
                    return Err(AutonomousError::HumanApprovalRequired(
                        action.describe()
                    ));
                }
            }
        }
        
        // 8. Audit log
        if self.config.audit_logging {
            self.log_action(action, true);
        }
        
        self.action_counter += 1;
        
        Ok(true)
    }
    
    /// Yasaklı bölge kontrolü
    fn check_forbidden_regions(&self, action: &Action) -> AutonomousResult<()> {
        if self.config.forbidden_regions.is_empty() {
            return Ok(());
        }
        
        // Aksiyondan koordinatları çıkar
        let coords = match action {
            Action::MouseMove { x, y } => Some((*x, *y)),
            Action::MouseClick { x, y, .. } => Some((*x, *y)),
            Action::MouseDrag { to, .. } => Some((to.0, to.1)),
            _ => None,
        };
        
        if let Some((x, y)) = coords {
            for region in &self.config.forbidden_regions {
                if region.bounds.contains(x, y) {
                    match region.level {
                        ForbiddenLevel::Block => {
                            return Err(AutonomousError::ForbiddenRegion(x, y));
                        }
                        ForbiddenLevel::Warning => {
                            log::warn!("🛡️ SAFETY: Uyarı - Yasaklı bölgeye yakın: {}", region.name);
                        }
                        ForbiddenLevel::RequiresApproval => {
                            // Bu durumda human approval gerekli
                            return Err(AutonomousError::HumanApprovalRequired(
                                format!("Yasaklı bölge: {}", region.name)
                            ));
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// Yasaklı uygulama kontrolü
    fn check_forbidden_applications(&self, _action: &Action) -> AutonomousResult<()> {
        // Aktif pencere kontrolü
        // Not: Gerçek implementation için platform-specific API gerekir
        // Linux: xdotool/getactivewindow
        // Windows: GetForegroundWindow
        // macOS: NSWorkspace.frontmostApplication
        
        // Simulated check - production'da gerçek API kullanılmalı
        log::debug!("🛡️ SAFETY: Checking forbidden applications (simulated)");
        Ok(())
    }
    
    /// Yasaklı URL kontrolü
    fn check_forbidden_urls(&self, action: &Action) -> AutonomousResult<()> {
        if let Action::BrowserNavigate { url } = action {
            for pattern in &self.config.forbidden_url_patterns {
                if url.contains(pattern) {
                    return Err(AutonomousError::SafetyViolation(
                        format!("Yasaklı URL pattern: {}", pattern)
                    ));
                }
            }
        }
        Ok(())
    }
    
    /// Audit log kaydı
    fn log_action(&mut self, action: &Action, approved: bool) {
        self.audit_log.push(AuditLog {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: chrono::Utc::now(),
            agent_id: "default".into(),
            action: action.describe(),
            result: if approved { "approved" } else { "blocked" }.into(),
            approved,
            approver: None,
            risk_level: action.criticality(),
            metadata: HashMap::new(),
        });
    }
    
    /// Hata kaydet
    pub fn record_error(&mut self) -> AutonomousResult<()> {
        self.error_counter += 1;
        
        if self.error_counter >= self.config.max_errors_before_stop {
            return Err(AutonomousError::MaxErrorsExceeded(self.config.max_errors_before_stop));
        }
        
        Ok(())
    }
    
    /// Emergency stop
    pub fn emergency_stop(&mut self) {
        self.emergency_stop = true;
        log::error!("🚨 SAFETY: EMERGENCY STOP!");
    }
    
    /// Emergency stop'ı kaldır
    pub fn clear_emergency_stop(&mut self) {
        self.emergency_stop = false;
        log::info!("🛡️ SAFETY: Emergency stop kaldırıldı");
    }
    
    /// Durdurma koşullarını kontrol et
    pub fn check_stop_conditions(&self) -> Vec<StopCondition> {
        let mut conditions = Vec::new();
        
        // Hata sayısı
        if self.error_counter >= self.config.max_errors_before_stop {
            conditions.push(StopCondition::ErrorCountExceeded {
                max: self.config.max_errors_before_stop,
                current: self.error_counter,
            });
        }
        
        // Session süresi
        if let Some(start) = self.session_start {
            let elapsed = start.elapsed().as_secs();
            if elapsed > self.config.max_session_duration_secs {
                conditions.push(StopCondition::SessionDurationExceeded {
                    max_secs: self.config.max_session_duration_secs,
                });
            }
        }
        
        conditions
    }
    
    /// Config'i al
    pub fn config(&self) -> &SafetyConfig {
        &self.config
    }
    
    /// Config'i güncelle
    pub fn update_config(&mut self, config: SafetyConfig) {
        self.config = config;
        log::info!("🛡️ SAFETY: Config güncellendi");
    }
    
    /// Audit log'u al
    pub fn audit_log(&self) -> &[AuditLog] {
        &self.audit_log
    }
    
    /// İstatistikler
    pub fn stats(&self) -> SafetyStats {
        SafetyStats {
            total_actions: self.action_counter,
            total_errors: self.error_counter,
            session_duration: self.session_start.map(|s| s.elapsed().as_secs()).unwrap_or(0),
            emergency_stops: if self.emergency_stop { 1 } else { 0 },
        }
    }
}

/// Güvenlik istatistikleri
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyStats {
    pub total_actions: usize,
    pub total_errors: usize,
    pub session_duration: u64,
    pub emergency_stops: u32,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  RATE LIMITER
// ═══════════════════════════════════════════════════════════════════════════════

/// Rate limiter
struct RateLimiter {
    max_per_minute: u32,
    timestamps: Vec<Instant>,
}

impl RateLimiter {
    fn new(max_per_minute: u32) -> Self {
        Self {
            max_per_minute,
            timestamps: Vec::with_capacity(max_per_minute as usize + 10),
        }
    }
    
    fn check(&mut self) -> bool {
        let now = Instant::now();
        let one_minute_ago = now - Duration::from_secs(60);
        
        // Eski kayıtları temizle
        self.timestamps.retain(|&t| t > one_minute_ago);
        
        // Limit kontrolü
        if self.timestamps.len() >= self.max_per_minute as usize {
            return false;
        }
        
        self.timestamps.push(now);
        true
    }
}

impl Default for SafetySystem {
    fn default() -> Self {
        Self::new(SafetyConfig::default())
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_safety_config_default() {
        let config = SafetyConfig::default();
        assert_eq!(config.max_actions_per_minute, 120);
        assert!(config.emergency_stop_enabled);
    }
    
    #[test]
    fn test_safety_config_strict() {
        let config = SafetyConfig::strict();
        assert!(config.require_human_approval);
        assert_eq!(config.max_errors_before_stop, 3);
    }
    
    #[test]
    fn test_action_criticality() {
        let action = Action::MouseMove { x: 100, y: 100 };
        assert_eq!(action.criticality(), ActionCriticality::Normal);
        
        let action = Action::MouseClick { button: crate::MouseButton::Left, x: 100, y: 100 };
        assert_eq!(action.criticality(), ActionCriticality::Moderate);
    }
    
    #[test]
    fn test_action_describe() {
        let action = Action::MouseMove { x: 100, y: 200 };
        assert!(action.describe().contains("100"));
    }
    
    #[tokio::test]
    async fn test_safety_system_creation() {
        let safety = SafetySystem::default();
        assert!(!safety.emergency_stop);
    }
    
    #[tokio::test]
    async fn test_check_action_normal() {
        let mut safety = SafetySystem::default();
        safety.start_session();
        
        let action = Action::MouseMove { x: 100, y: 100 };
        let result = safety.check_action(&action).await;
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_emergency_stop() {
        let mut safety = SafetySystem::default();
        safety.start_session();
        safety.emergency_stop();
        
        let action = Action::MouseMove { x: 100, y: 100 };
        let result = safety.check_action(&action).await;
        assert!(result.is_err());
    }
    
    #[tokio::test]
    async fn test_rate_limiter() {
        let mut limiter = RateLimiter::new(2);
        
        assert!(limiter.check());
        assert!(limiter.check());
        assert!(!limiter.check()); // Limit aşıldı
    }
}
