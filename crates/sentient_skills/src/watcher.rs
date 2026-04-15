//! ─── Screen Watcher ───

use serde::{Deserialize, Serialize};

/// User action types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserActionType {
    KeyboardShortcut(String),
    MouseClick { x: i32, y: i32, button: String },
    MouseMove { x: i32, y: i32 },
    TextInput(String),
    OpenApp(String),
    OpenUrl(String),
    RunCommand(String),
    FileOperation(String),
    VoiceCommand(String),
    Wait(u64),
    Scroll { direction: String, amount: u32 },
    Custom(String),
}

/// A captured user action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAction {
    pub id: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub action_type: UserActionType,
    pub window_title: Option<String>,
    pub app_name: Option<String>,
    pub description: String,
    pub duration_ms: u64,
}

impl UserAction {
    pub fn new(action_type: UserActionType) -> Self {
        let description = match &action_type {
            UserActionType::KeyboardShortcut(s) => format!("Press {}", s),
            UserActionType::MouseClick { x, y, button } => format!("Click {} at ({}, {})", button, x, y),
            UserActionType::MouseMove { x, y } => format!("Move to ({}, {})", x, y),
            UserActionType::TextInput(text) => format!("Type: {}", text),
            UserActionType::OpenApp(app) => format!("Open {}", app),
            UserActionType::OpenUrl(url) => format!("Navigate to {}", url),
            UserActionType::RunCommand(cmd) => format!("Run: {}", cmd),
            UserActionType::FileOperation(op) => format!("File: {}", op),
            UserActionType::VoiceCommand(cmd) => format!("Voice: {}", cmd),
            UserActionType::Wait(ms) => format!("Wait {}ms", ms),
            UserActionType::Scroll { direction, amount } => format!("Scroll {} by {}", direction, amount),
            UserActionType::Custom(desc) => desc.clone(),
        };
        
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: chrono::Utc::now(),
            action_type,
            window_title: None,
            app_name: None,
            description,
            duration_ms: 0,
        }
    }
}

/// Watcher event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WatcherEvent {
    Start,
    Stop,
    Action(UserAction),
    Screenshot(String),
    PatternDetected(crate::patterns::ActionPattern),
    Error(String),
}

/// Watcher config
#[derive(Debug, Clone)]
pub struct WatcherConfig {
    pub capture_screenshots: bool,
    pub max_actions: usize,
}

impl Default for WatcherConfig {
    fn default() -> Self {
        Self {
            capture_screenshots: false,
            max_actions: 10000,
        }
    }
}

/// Screen watcher
pub struct ScreenWatcher {
    config: WatcherConfig,
    running: bool,
    actions: Vec<UserAction>,
}

impl ScreenWatcher {
    pub fn new() -> Self {
        Self {
            config: WatcherConfig::default(),
            running: false,
            actions: vec![],
        }
    }
    
    pub async fn start(&mut self) -> crate::SkillResult<()> {
        self.running = true;
        Ok(())
    }
    
    pub fn stop(&mut self) {
        self.running = false;
    }
    
    pub fn is_running(&self) -> bool {
        self.running
    }
    
    pub fn record(&mut self, action: UserAction) {
        self.actions.push(action);
        if self.actions.len() > self.config.max_actions {
            self.actions.remove(0);
        }
    }
    
    pub fn get_actions(&self) -> &[UserAction] {
        &self.actions
    }
    
    pub fn clear(&mut self) {
        self.actions.clear();
    }
}

impl Default for ScreenWatcher {
    fn default() -> Self { Self::new() }
}

/// Action recorder
pub struct ActionRecorder {
    recording: bool,
    actions: Vec<UserAction>,
}

impl ActionRecorder {
    pub fn new() -> Self {
        Self { recording: false, actions: vec![] }
    }
    
    pub fn start(&mut self) {
        self.recording = true;
        self.actions.clear();
    }
    
    pub fn stop(&mut self) -> Vec<UserAction> {
        self.recording = false;
        self.actions.clone()
    }
    
    pub fn record(&mut self, action: UserAction) {
        if self.recording {
            self.actions.push(action);
        }
    }
    
    pub fn is_recording(&self) -> bool {
        self.recording
    }
}

impl Default for ActionRecorder {
    fn default() -> Self { Self::new() }
}
