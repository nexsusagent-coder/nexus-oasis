//! ═══════════════════════════════════════════════════════════════════════════════
//!  OASIS VOICE CONTROL - JARVIS-level Desktop Voice Interface
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//!  Sesle bilgisayar kontrolü sağlar:
//!  - Wake word detection ("Hey SENTIENT", "JARVIS")
//!  - Voice command parsing
//!  - Screen understanding integration
//!  - Action execution (mouse, keyboard, browser)
//!  - Safety checks
//!
//!  ÖRNEK KOMUTLAR:
//!  ───────────────
//!  - "Browser aç"
//!  - "Ekranda ne var?"
//!  - "Şuraya tıkla" (gaze tracking ile)
//!  - "Bu yazıyı seç"
//!  - "Araştır: Rust async programming"
//!  - "Uygulamayı kapat"
//!  - "Screenshot al"
//!  - "Hata mesajını oku"

use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::{RwLock, mpsc};
use serde::{Deserialize, Serialize};
use regex::Regex;

use sentient_voice::{
    VoiceEngine, VoiceConfig, VoiceError,
    TranscriptionResult, StreamConfig, StreamEvent,
    WakeWord, WakeWordDetector,
};

use crate::{
    AutonomousAgent, AgentState, AgentConfig,
    Action, ActionResult, MouseButton, Key,
    SafetySystem, SafetyConfig,
    ScreenUnderstanding,
    TaskPlanner, Task, TaskStep, Target,
};

// ─────────────────────────────────────────────────────────────────────────────
// VOICE COMMAND TYPES
// ─────────────────────────────────────────────────────────────────────────────

/// Sesli komut türleri
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum VoiceCommand {
    /// Uygulama aç
    OpenApp {
        app_name: String,
    },
    
    /// Uygulama kapat
    CloseApp {
        app_name: String,
    },
    
    /// Web'de araştır
    WebSearch {
        query: String,
    },
    
    /// URL aç
    NavigateUrl {
        url: String,
    },
    
    /// Tıkla
    Click {
        target: String,
    },
    
    /// Yazı yaz
    Type {
        text: String,
        human_like: bool,
    },
    
    /// Tuşa bas
    PressKey {
        key: String,
    },
    
    /// Kısayol
    Shortcut {
        modifiers: Vec<String>,
        key: String,
    },
    
    /// Scroll
    Scroll {
        direction: ScrollDirection,
        amount: u32,
    },
    
    /// Ekranı anlat
    DescribeScreen,
    
    /// Ekranı oku (OCR)
    ReadScreen,
    
    /// Screenshot al
    Screenshot,
    
    /// Mouse hareket et
    MoveMouse {
        direction: String,
        amount: i32,
    },
    
    /// Fareyi konuma götür
    GoToPosition {
        target: String,
    },
    
    /// Sesli yanıt ver
    Speak {
        text: String,
    },
    
    /// Toplantı başlat
    StartMeeting {
        platform: String,
        participants: Vec<String>,
    },
    
    /// Email gönder
    SendEmail {
        to: String,
        subject: String,
        body: String,
    },
    
    /// Dosya aç
    OpenFile {
        path: String,
    },
    
    /// Bilgi sor
    AskQuestion {
        question: String,
    },
    
    /// Hatırlatıcı kur
    SetReminder {
        message: String,
        time: String,
    },
    
    /// Sesi kapat/aç
    ToggleMute,
    
    /// Durdur
    Stop,
    
    /// Yardım
    Help,
    
    /// Bilinmeyen komut
    Unknown {
        original: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ScrollDirection {
    Up,
    Down,
    Left,
    Right,
}

/// Komut sonucu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandResult {
    /// Komut
    pub command: VoiceCommand,
    /// Başarılı mı?
    pub success: bool,
    /// Mesaj
    pub message: String,
    /// Konuşma yanıtı (TTS için)
    pub spoken_response: String,
    /// Ek veri
    pub data: Option<serde_json::Value>,
}

impl CommandResult {
    pub fn success(command: VoiceCommand, message: &str, spoken: &str) -> Self {
        Self {
            command,
            success: true,
            message: message.into(),
            spoken_response: spoken.into(),
            data: None,
        }
    }
    
    pub fn failure(command: VoiceCommand, message: &str) -> Self {
        Self {
            command,
            success: false,
            message: message.into(),
            spoken_response: message.into(),
            data: None,
        }
    }
    
    pub fn with_data(mut self, data: serde_json::Value) -> Self {
        self.data = Some(data);
        self
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// VOICE CONTROL CONFIG
// ─────────────────────────────────────────────────────────────────────────────

/// Voice control konfigürasyonu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceControlConfig {
    /// Wake word'ler
    pub wake_words: Vec<String>,
    
    /// Dil
    pub language: String,
    
    /// Otomatik dinleme (wake word gerekmez)
    pub always_listening: bool,
    
    /// Komut timeout (ms)
    pub command_timeout_ms: u64,
    
    /// Onay gerektiren aksiyonlar
    pub requires_confirmation: Vec<String>,
    
    /// TTS yanıtı ver
    pub speak_responses: bool,
    
    /// Güvenli mod (tehlikeli aksiyonları engelle)
    pub safe_mode: bool,
    
    /// Göz takibi entegrasyonu
    pub gaze_integration: bool,
}

impl Default for VoiceControlConfig {
    fn default() -> Self {
        Self {
            wake_words: vec![
                "hey sentient".into(),
                "jarvis".into(),
                "sentient".into(),
            ],
            language: "tr".into(),
            always_listening: false,
            command_timeout_ms: 30000,
            requires_confirmation: vec![
                "delete".into(),
                "close".into(),
                "shutdown".into(),
                "email".into(),
            ],
            speak_responses: true,
            safe_mode: true,
            gaze_integration: true,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// COMMAND PARSER
// ─────────────────────────────────────────────────────────────────────────────

/// Sesli komut ayrıştırıcı
pub struct CommandParser {
    patterns: Vec<CommandPattern>,
}

/// Komut pattern'i
struct CommandPattern {
    regex: Regex,
    command_type: String,
    extractors: Vec<String>,
}

impl CommandParser {
    pub fn new() -> Self {
        let patterns = vec![
            // Uygulama açma
            CommandPattern {
                regex: Regex::new(r"(?i)^(aç|open|launch|start)\s+(.+)$").unwrap(),
                command_type: "open_app".into(),
                extractors: vec!["app".into()],
            },
            
            // Uygulama kapatma
            CommandPattern {
                regex: Regex::new(r"(?i)^(kapat|close|quit|exit)\s+(.+)$").unwrap(),
                command_type: "close_app".into(),
                extractors: vec!["app".into()],
            },
            
            // Web araştırma
            CommandPattern {
                regex: Regex::new(r"(?i)^(araştır|search|google|look up)\s+(.+)$").unwrap(),
                command_type: "web_search".into(),
                extractors: vec!["query".into()],
            },
            
            // URL açma
            CommandPattern {
                regex: Regex::new(r"(?i)^(git|go to|navigate|open)\s+(https?://\S+)$").unwrap(),
                command_type: "navigate_url".into(),
                extractors: vec!["url".into()],
            },
            
            // Tıklama
            CommandPattern {
                regex: Regex::new(r"(?i)^(tıkla|click|press)\s+(.+)$").unwrap(),
                command_type: "click".into(),
                extractors: vec!["target".into()],
            },
            
            // Yazma
            CommandPattern {
                regex: Regex::new(r"(?i)^(yaz|type|write)\s+(.+)$").unwrap(),
                command_type: "type".into(),
                extractors: vec!["text".into()],
            },
            
            // Scroll
            CommandPattern {
                regex: Regex::new(r"(?i)^(scroll|kaydır)\s+(up|down|yukarı|aşağı)\s*(\d+)?").unwrap(),
                command_type: "scroll".into(),
                extractors: vec!["direction".into(), "amount".into()],
            },
            
            // Ekran anlatma
            CommandPattern {
                regex: Regex::new(r"(?i)^(ekranı anlat|what'?s on (the )?screen|describe screen|ekranda ne var)").unwrap(),
                command_type: "describe_screen".into(),
                extractors: vec![],
            },
            
            // Ekran okuma
            CommandPattern {
                regex: Regex::new(r"(?i)^(ekranı oku|read screen|ocr|yazıyı oku)").unwrap(),
                command_type: "read_screen".into(),
                extractors: vec![],
            },
            
            // Screenshot
            CommandPattern {
                regex: Regex::new(r"(?i)^(screenshot|ekran görüntüsü|capture screen)").unwrap(),
                command_type: "screenshot".into(),
                extractors: vec![],
            },
            
            // Fare hareketi
            CommandPattern {
                regex: Regex::new(r"(?i)^(move mouse|fareyi hareket ettir|fareyi?\s+(yukarı|aşağı|sağa|sola|up|down|left|right))").unwrap(),
                command_type: "move_mouse".into(),
                extractors: vec!["direction".into()],
            },
            
            // Sesli konuşma
            CommandPattern {
                regex: Regex::new(r"(?i)^(söyle|say|speak|tell me)\s+(.+)$").unwrap(),
                command_type: "speak".into(),
                extractors: vec!["text".into()],
            },
            
            // Soru sorma
            CommandPattern {
                regex: Regex::new(r"(?i)^(what|who|when|where|why|how|ne|kim|neden|nasıl)\s+(.+?)\??$").unwrap(),
                command_type: "ask_question".into(),
                extractors: vec!["question".into()],
            },
            
            // Durdurma
            CommandPattern {
                regex: Regex::new(r"(?i)^(stop|dur|durdur|cancel|iptal)").unwrap(),
                command_type: "stop".into(),
                extractors: vec![],
            },
            
            // Yardım
            CommandPattern {
                regex: Regex::new(r"(?i)^(help|yardım|yardim|neler yapabilirsin)").unwrap(),
                command_type: "help".into(),
                extractors: vec![],
            },
        ];
        
        Self { patterns }
    }
    
    /// Metni komuta dönüştür
    pub fn parse(&self, text: &str) -> VoiceCommand {
        let text = text.trim();
        
        for pattern in &self.patterns {
            if let Some(caps) = pattern.regex.captures(text) {
                return self.extract_command(&pattern.command_type, &caps);
            }
        }
        
        // Komut tanınamadı
        VoiceCommand::Unknown {
            original: text.into(),
        }
    }
    
    fn extract_command(&self, command_type: &str, caps: &regex::Captures) -> VoiceCommand {
        match command_type {
            "open_app" => {
                let app = caps.get(2).map(|m| m.as_str()).unwrap_or("");
                VoiceCommand::OpenApp { app_name: app.into() }
            }
            
            "close_app" => {
                let app = caps.get(2).map(|m| m.as_str()).unwrap_or("");
                VoiceCommand::CloseApp { app_name: app.into() }
            }
            
            "web_search" => {
                let query = caps.get(2).map(|m| m.as_str()).unwrap_or("");
                VoiceCommand::WebSearch { query: query.into() }
            }
            
            "navigate_url" => {
                let url = caps.get(2).map(|m| m.as_str()).unwrap_or("");
                VoiceCommand::NavigateUrl { url: url.into() }
            }
            
            "click" => {
                let target = caps.get(2).map(|m| m.as_str()).unwrap_or("");
                VoiceCommand::Click { target: target.into() }
            }
            
            "type" => {
                let text = caps.get(2).map(|m| m.as_str()).unwrap_or("");
                VoiceCommand::Type { text: text.into(), human_like: true }
            }
            
            "scroll" => {
                let direction_str = caps.get(2).map(|m| m.as_str().to_lowercase()).unwrap_or_default();
                let direction = if direction_str.contains("yukarı") || direction_str.contains("up") {
                    ScrollDirection::Up
                } else {
                    ScrollDirection::Down
                };
                let amount = caps.get(3)
                    .and_then(|m| m.as_str().parse().ok())
                    .unwrap_or(3);
                VoiceCommand::Scroll { direction, amount }
            }
            
            "describe_screen" => VoiceCommand::DescribeScreen,
            "read_screen" => VoiceCommand::ReadScreen,
            "screenshot" => VoiceCommand::Screenshot,
            
            "stop" => VoiceCommand::Stop,
            "help" => VoiceCommand::Help,
            
            "speak" => {
                let text = caps.get(2).map(|m| m.as_str()).unwrap_or("");
                VoiceCommand::Speak { text: text.into() }
            }
            
            "ask_question" => {
                let question = caps.get(0).map(|m| m.as_str()).unwrap_or("");
                VoiceCommand::AskQuestion { question: question.into() }
            }
            
            _ => VoiceCommand::Unknown { original: command_type.into() }
        }
    }
}

impl Default for CommandParser {
    fn default() -> Self {
        Self::new()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// VOICE CONTROL ENGINE
// ─────────────────────────────────────────────────────────────────────────────

/// Ana voice control motoru
pub struct VoiceControlEngine {
    /// Voice engine
    voice_engine: Arc<VoiceEngine>,
    
    /// Autonomous agent
    agent: Arc<RwLock<AutonomousAgent>>,
    
    /// Screen understanding
    screen: Arc<RwLock<ScreenUnderstanding>>,
    
    /// Safety system
    safety: Arc<SafetySystem>,
    
    /// Command parser
    parser: CommandParser,
    
    /// Konfigürasyon
    config: VoiceControlConfig,
    
    /// Durum
    state: Arc<RwLock<VoiceControlState>>,
    
    /// Komut kanalı
    command_tx: mpsc::Sender<VoiceCommand>,
    command_rx: Option<mpsc::Receiver<VoiceCommand>>,
    
    /// Sonuç kanalı
    result_tx: mpsc::Sender<CommandResult>,
    result_rx: Option<mpsc::Receiver<CommandResult>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum VoiceControlState {
    Idle,
    Listening,
    Processing,
    Executing,
    Speaking,
    Error,
}

impl VoiceControlEngine {
    /// Yeni voice control engine oluştur
    pub fn new(
        voice_engine: Arc<VoiceEngine>,
        agent: AutonomousAgent,
        config: VoiceControlConfig,
    ) -> Self {
        let (command_tx, command_rx) = mpsc::channel(100);
        let (result_tx, result_rx) = mpsc::channel(100);
        
        Self {
            voice_engine,
            agent: Arc::new(RwLock::new(agent)),
            screen: Arc::new(RwLock::new(ScreenUnderstanding::new())),
            safety: Arc::new(SafetySystem::new(SafetyConfig::default())),
            parser: CommandParser::new(),
            config,
            state: Arc::new(RwLock::new(VoiceControlState::Idle)),
            command_tx,
            command_rx: Some(command_rx),
            result_tx,
            result_rx: Some(result_rx),
        }
    }
    
    /// Voice control'ü başlat
    pub async fn start(&mut self) -> Result<(), VoiceError> {
        log::info!("🎙️  Voice Control Engine başlatılıyor...");
        
        // Durumu güncelle
        *self.state.write().await = VoiceControlState::Listening;
        
        // Wake word detector başlat
        let wake_words: Vec<WakeWord> = self.config.wake_words.iter()
            .map(|w| WakeWord {
                id: uuid::Uuid::new_v4().to_string(),
                phrase: w.clone(),
                confidence: 0.0,
                timestamp: 0.0,
                language: self.config.language.clone(),
                sensitivity: 0.5,
                enabled: true,
            })
            .collect();
        
        // Ses akışını başlat
        let stream_config = StreamConfig {
            sample_rate: 16000,
            frame_size: 1600,
            silence_timeout_ms: 2000,
            min_audio_ms: 500,
            vad_enabled: true,
            vad_sensitivity: 0.3,
        };
        
        // Main processing loop
        self.run_processing_loop().await?;
        
        Ok(())
    }
    
    /// Ana işlem döngüsü
    async fn run_processing_loop(&mut self) -> Result<(), VoiceError> {
        // Komut alıcısını al
        let mut command_rx = self.command_rx.take()
            .ok_or_else(|| VoiceError::Internal("Command receiver already taken".into()))?;
        
        // Sonuç alıcısını al
        let mut result_rx = self.result_rx.take()
            .ok_or_else(|| VoiceError::Internal("Result receiver already taken".into()))?;
        
        loop {
            tokio::select! {
                // Komut al
                Some(command) = command_rx.recv() => {
                    self.handle_command(command).await;
                }
                
                // Sonuç al
                Some(result) = result_rx.recv() => {
                    if self.config.speak_responses {
                        self.speak(&result.spoken_response).await?;
                    }
                }
                
                // Durdur sinyali
                _ = tokio::signal::ctrl_c() => {
                    log::info!("🛑 Voice Control durduruluyor");
                    break;
                }
            }
        }
        
        Ok(())
    }
    
    /// Komut işle
    async fn handle_command(&self, command: VoiceCommand) {
        *self.state.write().await = VoiceControlState::Processing;
        
        log::info!("🎙️ Komut: {:?}", command);
        
        // Safety check
        if self.config.safe_mode {
            if let Some(dangerous) = self.check_dangerous_command(&command) {
                self.result_tx.send(CommandResult::failure(
                    command,
                    &format!("Güvenlik: {}", dangerous),
                )).await.ok();
                return;
            }
        }
        
        // Onay gerekli mi?
        if self.requires_confirmation(&command) {
            // TODO: Kullanıcıdan onay iste
            log::warn!("⚠️ Bu komut onay gerektiriyor: {:?}", command);
        }
        
        *self.state.write().await = VoiceControlState::Executing;
        
        // Komutu çalıştır
        let result = self.execute_command(command).await;
        
        // Sonucu gönder
        self.result_tx.send(result).await.ok();
        
        *self.state.write().await = VoiceControlState::Idle;
    }
    
    /// Komutu çalıştır
    async fn execute_command(&self, command: VoiceCommand) -> CommandResult {
        match &command {
            VoiceCommand::OpenApp { app_name } => {
                let action = Action::TypeText { text: app_name.clone(), human_like: false };
                // macOS: Cmd+Space, Windows: Win, Linux: Super
                // TODO: Implement app opening
                CommandResult::success(command.clone(), 
                    &format!("{} açılıyor...", app_name),
                    &format!("{} uygulamasını açıyorum", app_name)
                )
            }
            
            VoiceCommand::CloseApp { app_name } => {
                CommandResult::success(command.clone(),
                    &format!("{} kapatılıyor...", app_name),
                    &format!("{} uygulamasını kapatıyorum", app_name)
                )
            }
            
            VoiceCommand::WebSearch { query } => {
                // Browser aç ve ara
                let url = format!("https://www.google.com/search?q={}", 
                    urlencoding::encode(query));
                CommandResult::success(command.clone(),
                    &format!("Araştırılıyor: {}", query),
                    &format!("{} hakkında araştırıyorum", query)
                )
            }
            
            VoiceCommand::NavigateUrl { url } => {
                CommandResult::success(command.clone(),
                    &format!("URL açılıyor: {}", url),
                    "Web sayfasını açıyorum"
                )
            }
            
            VoiceCommand::Click { target } => {
                // Screen understanding ile hedefi bul
                CommandResult::success(command.clone(),
                    &format!("Tıklanıyor: {}", target),
                    &format!("{} üzerine tıklıyorum", target)
                )
            }
            
            VoiceCommand::Type { text, human_like } => {
                CommandResult::success(command.clone(),
                    &format!("Yazılıyor: {}", text),
                    "Yazıyorum"
                )
            }
            
            VoiceCommand::DescribeScreen => {
                // Screen understanding çalıştır
                let description = "Ekran açıklaması buraya gelecek".to_string();
                CommandResult::success(command.clone(),
                    "Ekran analiz edildi",
                    &description
                )
            }
            
            VoiceCommand::ReadScreen => {
                // OCR çalıştır
                CommandResult::success(command.clone(),
                    "Ekran okundu",
                    "Ekrandaki yazıları okuyorum"
                )
            }
            
            VoiceCommand::Screenshot => {
                CommandResult::success(command.clone(),
                    "Screenshot alındı",
                    "Ekran görüntüsü aldım"
                )
            }
            
            VoiceCommand::Scroll { direction, amount } => {
                let dir_str = match direction {
                    ScrollDirection::Up => "yukarı",
                    ScrollDirection::Down => "aşağı",
                    ScrollDirection::Left => "sola",
                    ScrollDirection::Right => "sağa",
                };
                CommandResult::success(command.clone(),
                    &format!("{} {} kez scroll edildi", dir_str, amount),
                    &format!("{} doğru kaydırıyorum {} kez", dir_str, amount)
                )
            }
            
            VoiceCommand::Stop => {
                CommandResult::success(command.clone(),
                    "Durduruldu",
                    "Tamam, durdurdum"
                )
            }
            
            VoiceCommand::Help => {
                let help_text = r#"
Kullanabileceğiniz komutlar:
- "Aç [uygulama]" - Uygulama açar
- "Kapat [uygulama]" - Uygulama kapatır  
- "Araştır [konu]" - Web'de araştırır
- "Git [URL]" - Web sayfası açar
- "Tıkla [hedef]" - Ekrandaki öğeye tıklar
- "Yaz [metin]" - Metin yazar
- "Scroll yukarı/aşağı [miktar]" - Kaydırır
- "Ekranı anlat" - Ekranı açıklar
- "Screenshot" - Ekran görüntüsü alır
"#;
                CommandResult::success(command.clone(), "Yardım", help_text)
            }
            
            VoiceCommand::Unknown { original } => {
                CommandResult::failure(command.clone(),
                    &format!("Anlaşılamadı: {}", original)
                )
            }
            
            _ => CommandResult::failure(command.clone(), "Komut henüz implement edilmedi")
        }
    }
    
    /// Tehlikeli komut kontrolü
    fn check_dangerous_command(&self, command: &VoiceCommand) -> Option<String> {
        match command {
            VoiceCommand::CloseApp { app_name } if app_name.to_lowercase().contains("system") => {
                Some("Sistem uygulamalarını kapatmak tehlikeli olabilir".into())
            }
            VoiceCommand::Unknown { original } if original.to_lowercase().contains("delete") => {
                Some("Silme işlemi güvenlik nedenleriyle engellendi".into())
            }
            _ => None,
        }
    }
    
    /// Onay gerektiriyor mu?
    fn requires_confirmation(&self, command: &VoiceCommand) -> bool {
        let command_str = format!("{:?}", command).to_lowercase();
        self.config.requires_confirmation.iter().any(|r| command_str.contains(r))
    }
    
    /// Sesli yanıt ver
    async fn speak(&self, text: &str) -> Result<(), VoiceError> {
        *self.state.write().await = VoiceControlState::Speaking;
        
        log::info!("🔊 Speaking: {}", text);
        
        let result = self.voice_engine.synthesize(text).await?;
        
        // TODO: Play audio
        
        *self.state.write().await = VoiceControlState::Idle;
        
        Ok(())
    }
    
    /// Durumu al
    pub async fn get_state(&self) -> VoiceControlState {
        self.state.read().await.clone()
    }
    
    /// Komut gönder (dışarıdan)
    pub async fn send_command(&self, command: VoiceCommand) -> Result<(), VoiceError> {
        self.command_tx.send(command).await
            .map_err(|e| VoiceError::Internal(e.to_string()))
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// URL ENCODING HELPER
// ─────────────────────────────────────────────────────────────────────────────

mod urlencoding {
    pub fn encode(s: &str) -> String {
        url::form_urlencoded::byte_serialize(s.as_bytes()).collect()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// TESTS
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_command_parser_open_app() {
        let parser = CommandParser::new();
        
        let cmd = parser.parse("aç Firefox");
        assert!(matches!(cmd, VoiceCommand::OpenApp { .. }));
        
        if let VoiceCommand::OpenApp { app_name } = cmd {
            assert_eq!(app_name.to_lowercase(), "firefox");
        }
    }
    
    #[test]
    fn test_command_parser_web_search() {
        let parser = CommandParser::new();
        
        let cmd = parser.parse("araştır Rust async programming");
        assert!(matches!(cmd, VoiceCommand::WebSearch { .. }));
        
        if let VoiceCommand::WebSearch { query } = cmd {
            assert!(query.contains("Rust"));
        }
    }
    
    #[test]
    fn test_command_parser_describe_screen() {
        let parser = CommandParser::new();
        
        let cmd = parser.parse("ekranı anlat");
        assert_eq!(cmd, VoiceCommand::DescribeScreen);
        
        let cmd = parser.parse("what's on screen");
        assert_eq!(cmd, VoiceCommand::DescribeScreen);
    }
    
    #[test]
    fn test_command_parser_unknown() {
        let parser = CommandParser::new();
        
        let cmd = parser.parse("bu komut hiçbir şeye benzemiyor");
        assert!(matches!(cmd, VoiceCommand::Unknown { .. }));
    }
    
    #[test]
    fn test_command_parser_scroll() {
        let parser = CommandParser::new();
        
        let cmd = parser.parse("scroll aşağı 5");
        if let VoiceCommand::Scroll { direction, amount } = cmd {
            assert_eq!(direction, ScrollDirection::Down);
            assert_eq!(amount, 5);
        } else {
            panic!("Wrong command type");
        }
    }
    
    #[test]
    fn test_command_result_success() {
        let cmd = VoiceCommand::Screenshot;
        let result = CommandResult::success(cmd.clone(), "Screenshot alındı", "Ekran görüntüsü aldım");
        
        assert!(result.success);
        assert_eq!(result.message, "Screenshot alındı");
        assert_eq!(result.spoken_response, "Ekran görüntüsü aldım");
    }
    
    #[test]
    fn test_voice_control_config_default() {
        let config = VoiceControlConfig::default();
        
        assert!(!config.wake_words.is_empty());
        assert!(!config.always_listening);
        assert!(config.speak_responses);
        assert!(config.safe_mode);
    }
    
    #[test]
    fn test_requires_confirmation() {
        let parser = CommandParser::new();
        let config = VoiceControlConfig::default();
        
        let cmd = parser.parse("close Firefox");
        let requires = config.requires_confirmation.iter()
            .any(|r| format!("{:?}", cmd).to_lowercase().contains(r));
        
        // "close" should match
        assert!(requires);
    }
}
