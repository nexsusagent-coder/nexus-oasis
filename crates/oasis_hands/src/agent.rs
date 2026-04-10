//! ═══════════════════════════════════════════════════════════════════════════════
//!  DESKTOP AGENT - L6: ANA AJAN
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Masaüstü kontrolü için AI ajanı.

use crate::error::{HandsError, HandsResult};
use crate::sovereign::SovereignPolicy;
use crate::screen::{ScreenCapture, ScreenCapturer};
use crate::input::{InputController, MouseAction, KeyboardAction, MouseButton};
use crate::vision::{VisionEngine, UIElement};
use crate::vgate::HandsVGate;
use serde::{Deserialize, Serialize};

// ───────────────────────────────────────────────────────────────────────────────
//  AGENT DURUMU
// ───────────────────────────────────────────────────────────────────────────────

/// Ajan durumu
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentState {
    /// Boşta
    Idle,
    /// Görev hazırlanıyor
    Planning,
    /// Aksiyon alınıyor
    Acting,
    /// Gözlem yapıyor
    Observing,
    /// Hata durumu
    Error,
    /// Durduruldu
    Stopped,
}

impl Default for AgentState {
    fn default() -> Self {
        Self::Idle
    }
}

// ───────────────────────────────────────────────────────────────────────────────
//  AGENT YAPILANDIRMASI
// ───────────────────────────────────────────────────────────────────────────────

/// Ajan yapılandırması
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    /// Mouse hızı
    pub mouse_speed: f32,
    /// OCR aktif mi?
    pub ocr_enabled: bool,
    /// Güvenlik modu
    pub security_mode: crate::SecurityMode,
    /// Onay gerekli mi?
    pub require_confirmation: bool,
    /// Maksimum aksiyon süresi
    pub max_action_duration: u64,
    /// Maksimum iterasyon sayısı
    pub max_iterations: u32,
    /// Hedef çözünürlük
    pub target_resolution: Option<(u32, u32)>,
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            mouse_speed: crate::DEFAULT_MOUSE_SPEED,
            ocr_enabled: true,
            security_mode: crate::SecurityMode::Normal,
            require_confirmation: true,
            max_action_duration: crate::MAX_ACTION_DURATION_SECS,
            max_iterations: 50,
            target_resolution: None,
        }
    }
}

// ───────────────────────────────────────────────────────────────────────────────
//  MASAÜSTÜ GÖREVİ
// ───────────────────────────────────────────────────────────────────────────────

/// Masaüstü görevi
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesktopTask {
    /// Görev ID
    pub id: String,
    /// Görev açıklaması
    pub description: String,
    /// Görev tipi
    pub task_type: TaskType,
    /// Öncelik
    pub priority: u8,
    /// Zaman damgası
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Durum
    pub status: TaskStatus,
    /// Sonuç
    pub result: Option<String>,
}

/// Görev tipi
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskType {
    /// Genel görev
    General,
    /// Dosya işlemi
    FileOperation,
    /// Uygulama kontrolü
    AppControl,
    /// Web tarayıcı
    WebBrowsing,
    /// Metin işleme
    TextProcessing,
    /// Otomasyon
    Automation,
}

/// Görev durumu
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskStatus {
    /// Bekliyor
    Pending,
    /// Çalışıyor
    Running,
    /// Tamamlandı
    Completed,
    /// Başarısız
    Failed,
    /// İptal edildi
    Cancelled,
}

impl DesktopTask {
    /// Yeni görev oluştur
    pub fn new(description: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            description: description.into(),
            task_type: TaskType::General,
            priority: 5,
            created_at: chrono::Utc::now(),
            status: TaskStatus::Pending,
            result: None,
        }
    }
    
    /// Görev tipini ayarla
    pub fn with_type(mut self, task_type: TaskType) -> Self {
        self.task_type = task_type;
        self
    }
    
    /// Önceliği ayarla
    pub fn with_priority(mut self, priority: u8) -> Self {
        self.priority = priority.min(10);
        self
    }
}

// ───────────────────────────────────────────────────────────────────────────────
//  AKSİYON SONUCU
// ───────────────────────────────────────────────────────────────────────────────

/// Aksiyon sonucu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionResult {
    /// Başarılı mı?
    pub success: bool,
    /// Aksiyon tipi
    pub action_type: String,
    /// Mesaj
    pub message: String,
    /// Ek veri
    pub data: Option<serde_json::Value>,
    /// Zaman damgası
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl ActionResult {
    /// Başarılı sonuç
    pub fn success(action_type: &str, message: &str) -> Self {
        Self {
            success: true,
            action_type: action_type.into(),
            message: message.into(),
            data: None,
            timestamp: chrono::Utc::now(),
        }
    }
    
    /// Başarısız sonuç
    pub fn failure(action_type: &str, message: &str) -> Self {
        Self {
            success: false,
            action_type: action_type.into(),
            message: message.into(),
            data: None,
            timestamp: chrono::Utc::now(),
        }
    }
}

// ───────────────────────────────────────────────────────────────────────────────
//  DESKTOP AGENT
// ───────────────────────────────────────────────────────────────────────────────

/// Masaüstü kontrol ajanı
pub struct DesktopAgent {
    /// Yapılandırma
    config: AgentConfig,
    /// Sovereign policy
    policy: SovereignPolicy,
    /// Durum
    state: AgentState,
    /// Ekran yakalayıcı
    screen_capturer: ScreenCapturer,
    /// Giriş kontrolü
    input_controller: InputController,
    /// Vision motoru
    vision_engine: VisionEngine,
    /// V-GATE köprüsü
    vgate: HandsVGate,
    /// Aktif görev
    current_task: Option<DesktopTask>,
    /// Görev geçmişi
    task_history: Vec<DesktopTask>,
    /// Aksiyon geçmişi
    action_history: Vec<ActionResult>,
}

impl DesktopAgent {
    /// Yeni agent oluştur
    pub async fn new(config: AgentConfig, policy: SovereignPolicy) -> HandsResult<Self> {
        log::info!("╔════════════════════════════════════════════════════════════════╗");
        log::info!("║  DESKTOP AGENT - L6: EXECUTION                                  ║");
        log::info!("╚════════════════════════════════════════════════════════════════╝");
        
        let screen_capturer = ScreenCapturer::new()?;
        let input_controller = InputController::new(policy.clone())?;
        let vision_engine = VisionEngine::new(config.ocr_enabled);
        let vgate = HandsVGate::new("http://127.0.0.1:1071");
        
        log::info!("🖐️  AGENT: Masaüstü kontrol ajanı hazır");
        
        Ok(Self {
            config,
            policy,
            state: AgentState::Idle,
            screen_capturer,
            input_controller,
            vision_engine,
            vgate,
            current_task: None,
            task_history: Vec::new(),
            action_history: Vec::new(),
        })
    }
    
    /// Görev çalıştır
    pub async fn execute_task(&mut self, task_description: &str) -> HandsResult<String> {
        // Durumu güncelle
        self.state = AgentState::Planning;
        
        let task = DesktopTask::new(task_description);
        log::info!("🎯  AGENT: Görev başlatılıyor → {}", 
            task_description.chars().take(60).collect::<String>());
        
        self.current_task = Some(task);
        
        // V-GATE üzerinden aksiyon planı al
        let action_plan = self.plan_actions(task_description).await?;
        
        // Aksiyonları uygula
        self.state = AgentState::Acting;
        
        let mut iteration = 0;
        let mut result = String::new();
        
        for action in action_plan {
            if iteration >= self.config.max_iterations {
                log::warn!("⚠️  AGENT: Maksimum iterasyon sayısına ulaşıldı");
                break;
            }
            
            // Her aksiyon öncesi sovereign kontrolü
            self.validate_action(&action)?;
            
            // Aksiyonu uygula
            let action_result = self.execute_action(&action).await?;
            
            // Sonucu kaydet
            self.action_history.push(action_result.clone());
            
            if !action_result.success {
                self.state = AgentState::Error;
                return Err(HandsError::AgentError(format!(
                    "OASIS-HANDS AGENT: Aksiyon başarısız → {}",
                    action_result.message
                )));
            }
            
            result = action_result.message;
            iteration += 1;
            
            // Kısa bekleme
            tokio::time::sleep(std::time::Duration::from_millis(200)).await;
        }
        
        // Görevi tamamlandı olarak işaretle
        if let Some(ref mut task) = self.current_task {
            task.status = TaskStatus::Completed;
            task.result = Some(result.clone());
            self.task_history.push(task.clone());
        }
        
        self.current_task = None;
        self.state = AgentState::Idle;
        
        log::info!("✅  AGENT: Görev tamamlandı");
        Ok(result)
    }
    
    /// Ekran görüntüsü al
    pub async fn capture_screen(&mut self) -> HandsResult<ScreenCapture> {
        self.screen_capturer.capture_full()
    }
    
    /// Element bul
    pub async fn find_element(&mut self, description: &str) -> HandsResult<Option<UIElement>> {
        let capture = self.capture_screen().await?;
        self.vision_engine.find_element(&capture, description).await
    }
    
    /// Fare aksiyonu
    pub async fn mouse_action(&mut self, action: MouseAction) -> HandsResult<()> {
        self.input_controller.execute_mouse(action).await
    }
    
    /// Klavye aksiyonu
    pub async fn keyboard_action(&mut self, action: KeyboardAction) -> HandsResult<()> {
        self.input_controller.execute_keyboard(action).await
    }
    
    /// Dosya oku
    pub async fn read_file(&mut self, path: &str) -> HandsResult<String> {
        // Sovereign kontrol
        self.policy.validate_file_access(path, false)?;
        
        // Mock dosya okuma
        log::info!("📄  AGENT: Dosya okunuyor → {}", path);
        Ok(format!("Mock içerik: {}", path))
    }
    
    /// Dosya yaz
    pub async fn write_file(&mut self, path: &str, content: &str) -> HandsResult<()> {
        // Sovereign kontrol - yazma daha katı
        self.policy.validate_file_access(path, true)?;
        
        log::info!("📝  AGENT: Dosya yazılıyor → {} ({} bytes)", path, content.len());
        Ok(())
    }
    
    /// Komut çalıştır
    pub async fn execute_command(&mut self, command: &str) -> HandsResult<String> {
        // Sovereign kontrol - tehlikeli komutlar engellenir
        self.policy.validate_command(command)?;
        
        // Whitelist uygulama kontrolü
        let app_name = command.split_whitespace().next().unwrap_or("");
        self.policy.validate_application(app_name)?;
        
        log::info!("⚡  AGENT: Komut çalıştırılıyor → {}", command);
        Ok(format!("Komut çıktısı: {}", command))
    }
    
    /// Agent'ı kapat
    pub async fn close(&mut self) -> HandsResult<()> {
        self.state = AgentState::Stopped;
        self.input_controller.stop();
        log::info!("🖐️  AGENT: Kapatıldı");
        Ok(())
    }
    
    /// Durum getir
    pub fn state(&self) -> AgentState {
        self.state
    }
    
    /// İstatistikler
    pub fn stats(&self) -> AgentStats {
        AgentStats {
            tasks_completed: self.task_history.len(),
            actions_executed: self.action_history.len(),
            current_state: self.state,
        }
    }
    
    // ─── İç metodlar ───
    
    /// Aksiyon planla
    async fn plan_actions(&self, _task: &str) -> HandsResult<Vec<String>> {
        // V-GATE üzerinden LLM'den aksiyon planı al
        // Mock plan döndür
        Ok(vec![
            "observe".into(),
            "click_button".into(),
            "type_text".into(),
        ])
    }
    
    /// Aksiyon doğrula
    fn validate_action(&self, action: &str) -> HandsResult<()> {
        // Tehlikeli aksiyonları kontrol et
        let blocked_actions = ["delete_system", "format_drive", "rm_rf"];
        
        if blocked_actions.iter().any(|b| action.contains(b)) {
            return Err(HandsError::SovereignViolation(format!(
                "OASIS-HANDS AGENT: '{}' aksiyonu güvenlik nedeniyle engellendi!",
                action
            )));
        }
        
        Ok(())
    }
    
    /// Aksiyon uygula
    async fn execute_action(&mut self, action: &str) -> HandsResult<ActionResult> {
        log::debug!("🎬  AGENT: Aksiyon → {}", action);
        
        match action {
            "observe" => {
                let capture = self.capture_screen().await?;
                let result = self.vision_engine.analyze(&capture).await?;
                Ok(ActionResult::success("observe", 
                    &format!("{} element tespit edildi", result.elements.len())))
            }
            "click_button" => {
                self.mouse_action(MouseAction::Click { button: MouseButton::Left }).await?;
                Ok(ActionResult::success("click", "Tıklama başarılı"))
            }
            "type_text" => {
                self.keyboard_action(KeyboardAction::TypeText { 
                    text: "Örnek metin".into(), 
                    typing_speed_ms: Some(50) 
                }).await?;
                Ok(ActionResult::success("type", "Metin yazıldı"))
            }
            _ => {
                Ok(ActionResult::success("generic", &format!("Aksiyon tamamlandı: {}", action)))
            }
        }
    }
}

/// Ajan istatistikleri
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentStats {
    pub tasks_completed: usize,
    pub actions_executed: usize,
    pub current_state: AgentState,
}

// ───────────────────────────────────────────────────────────────────────────────
//  TESTS
// ───────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_agent_state_default() {
        assert_eq!(AgentState::default(), AgentState::Idle);
    }
    
    #[test]
    fn test_agent_config_default() {
        let config = AgentConfig::default();
        assert!(config.ocr_enabled);
        assert!(config.require_confirmation);
    }
    
    #[test]
    fn test_desktop_task_creation() {
        let task = DesktopTask::new("Test görevi");
        assert_eq!(task.status, TaskStatus::Pending);
        assert!(task.result.is_none());
    }
    
    #[test]
    fn test_desktop_task_with_type() {
        let task = DesktopTask::new("Test")
            .with_type(TaskType::FileOperation);
        assert_eq!(task.task_type, TaskType::FileOperation);
    }
    
    #[test]
    fn test_action_result_success() {
        let result = ActionResult::success("test", "Başarılı");
        assert!(result.success);
    }
    
    #[test]
    fn test_action_result_failure() {
        let result = ActionResult::failure("test", "Başarısız");
        assert!(!result.success);
    }
    
    #[tokio::test]
    async fn test_agent_creation() {
        let config = AgentConfig::default();
        let policy = SovereignPolicy::strict();
        let agent = DesktopAgent::new(config, policy).await.expect("operation failed");
        assert_eq!(agent.state(), AgentState::Idle);
    }
}
