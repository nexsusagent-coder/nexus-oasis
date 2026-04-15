//! ─── Mobile Voice Control ───

use serde::{Deserialize, Serialize};
use crate::{RemoteResult, RemoteError};

/// Mobile voice control configuration
#[derive(Debug, Clone)]
pub struct VoiceConfig {
    pub sample_rate: u32,
    pub channels: u8,
    pub language: String,
    pub wake_word: String,
}

impl Default for VoiceConfig {
    fn default() -> Self {
        Self {
            sample_rate: 16000,
            channels: 1,
            language: "tr-TR".into(),
            wake_word: "sentient".into(),
        }
    }
}

/// Mobile voice control handler
pub struct MobileVoiceControl {
    config: VoiceConfig,
    listening: bool,
}

impl MobileVoiceControl {
    pub fn new(config: VoiceConfig) -> Self {
        Self { config, listening: false }
    }
    
    pub async fn start_listening(&mut self) -> RemoteResult<()> {
        tracing::info!("Mobile voice: start listening for wake word '{}'", self.config.wake_word);
        self.listening = true;
        Ok(())
    }
    
    pub fn stop_listening(&mut self) {
        self.listening = false;
    }
    
    pub async fn process_audio(&self, _audio_data: &[f32]) -> RemoteResult<VoiceResult> {
        Ok(VoiceResult {
            transcript: "mock transcription".into(),
            is_wake_word: false,
            confidence: 0.85,
        })
    }
    
    pub async fn parse_command(&self, transcript: &str) -> RemoteResult<ParsedCommand> {
        let lower = transcript.to_lowercase();
        let wake = &self.config.wake_word;
        
        if !lower.contains(wake) {
            return Ok(ParsedCommand { command: None, raw: transcript.to_string() });
        }
        
        let cmd_text = lower.replace(wake, "");
        let cmd_text = cmd_text.trim();
        
        let command = if cmd_text.contains("ekran") || cmd_text.contains("screenshot") {
            Some("screenshot".into())
        } else if cmd_text.contains("kilitle") || cmd_text.contains("lock") {
            Some("lock".into())
        } else if cmd_text.contains("durum") || cmd_text.contains("status") {
            Some("status".into())
        } else {
            None
        };
        
        Ok(ParsedCommand { command, raw: transcript.to_string() })
    }
    
    pub async fn send_response(&self, text: &str) -> RemoteResult<()> {
        tracing::info!("Mobile voice response: {}", text);
        Ok(())
    }
    
    pub fn is_listening(&self) -> bool {
        self.listening
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceResult {
    pub transcript: String,
    pub is_wake_word: bool,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedCommand {
    pub command: Option<String>,
    pub raw: String,
}
