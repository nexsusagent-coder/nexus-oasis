//! ─── Desktop Voice Widget ───

use serde::{Deserialize, Serialize};

/// Voice widget state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VoiceState {
    Idle,
    Listening,
    Processing,
    Speaking,
    Error,
}

impl VoiceState {
    pub fn icon(&self) -> &'static str {
        match self {
            VoiceState::Idle => "🎤",
            VoiceState::Listening => "👂",
            VoiceState::Processing => "⚙️",
            VoiceState::Speaking => "🔊",
            VoiceState::Error => "❌",
        }
    }
    
    pub fn color(&self) -> &'static str {
        match self {
            VoiceState::Idle => "#808080",
            VoiceState::Listening => "#4CAF50",
            VoiceState::Processing => "#2196F3",
            VoiceState::Speaking => "#FF9800",
            VoiceState::Error => "#F44336",
        }
    }
}

/// Voice widget configuration
#[derive(Debug, Clone)]
pub struct VoiceWidgetConfig {
    pub width: u32,
    pub height: u32,
    pub position_x: i32,
    pub position_y: i32,
    pub always_on_top: bool,
    pub opacity: f32,
    pub show_waveform: bool,
    pub show_transcript: bool,
}

impl Default for VoiceWidgetConfig {
    fn default() -> Self {
        Self {
            width: 300,
            height: 150,
            position_x: -20,
            position_y: -20,
            always_on_top: true,
            opacity: 0.9,
            show_waveform: true,
            show_transcript: true,
        }
    }
}

/// Desktop voice widget
pub struct VoiceWidget {
    config: VoiceWidgetConfig,
    state: VoiceState,
    visible: bool,
    transcript: String,
    response: String,
}

impl VoiceWidget {
    pub fn new(config: VoiceWidgetConfig) -> Self {
        Self {
            config,
            state: VoiceState::Idle,
            visible: false,
            transcript: String::new(),
            response: String::new(),
        }
    }
    
    /// Show the voice widget
    pub async fn show(&mut self) -> crate::Result<()> {
        tracing::info!("Showing voice widget");
        self.visible = true;
        // TODO: Create window with overlay
        Ok(())
    }
    
    /// Hide the voice widget
    pub fn hide(&mut self) {
        tracing::info!("Hiding voice widget");
        self.visible = false;
    }
    
    /// Toggle visibility
    pub async fn toggle(&mut self) -> crate::Result<()> {
        if self.visible {
            self.hide();
        } else {
            self.show().await?;
        }
        Ok(())
    }
    
    /// Set state
    pub fn set_state(&mut self, state: VoiceState) {
        self.state = state;
    }
    
    /// Get state
    pub fn state(&self) -> VoiceState {
        self.state
    }
    
    /// Update transcript
    pub fn set_transcript(&mut self, text: &str) {
        self.transcript = text.to_string();
    }
    
    /// Update response
    pub fn set_response(&mut self, text: &str) {
        self.response = text.to_string();
    }
    
    /// Clear current session
    pub fn clear(&mut self) {
        self.transcript.clear();
        self.response.clear();
        self.state = VoiceState::Idle;
    }
    
    /// Start listening animation
    pub async fn start_listening(&mut self) -> crate::Result<()> {
        self.state = VoiceState::Listening;
        self.transcript.clear();
        self.response.clear();
        self.show().await
    }
    
    /// Processing animation
    pub fn processing(&mut self) {
        self.state = VoiceState::Processing;
    }
    
    /// Speaking animation
    pub fn speaking(&mut self) {
        self.state = VoiceState::Speaking;
    }
    
    /// Error state
    pub fn error(&mut self, msg: &str) {
        self.state = VoiceState::Error;
        self.response = format!("Error: {}", msg);
    }
    
    /// Idle state
    pub fn idle(&mut self) {
        self.state = VoiceState::Idle;
    }
    
    pub fn is_visible(&self) -> bool {
        self.visible
    }
    
    pub fn get_transcript(&self) -> &str {
        &self.transcript
    }
    
    pub fn get_response(&self) -> &str {
        &self.response
    }
    
    pub fn get_config(&self) -> &VoiceWidgetConfig {
        &self.config
    }
}

impl Default for VoiceWidget {
    fn default() -> Self {
        Self::new(VoiceWidgetConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_voice_state_icon() {
        assert_eq!(VoiceState::Idle.icon(), "🎤");
        assert_eq!(VoiceState::Listening.icon(), "👂");
    }
    
    #[test]
    fn test_widget_creation() {
        let widget = VoiceWidget::default();
        assert_eq!(widget.state(), VoiceState::Idle);
        assert!(!widget.is_visible());
    }
}
