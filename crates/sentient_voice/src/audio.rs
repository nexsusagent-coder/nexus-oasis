//! Audio capture and playback

use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{AudioSettings, VoiceError, VoiceResult};

/// Audio capture
pub struct AudioCapture {
    config: AudioSettings,
    is_capturing: Arc<Mutex<bool>>,
}

impl AudioCapture {
    pub fn new(config: AudioSettings) -> Self {
        Self {
            config,
            is_capturing: Arc::new(Mutex::new(false)),
        }
    }

    /// Start capturing audio
    pub async fn start(&self) -> VoiceResult<()> {
        let mut capturing = self.is_capturing.lock().await;
        *capturing = true;
        log::info!("Audio capture started ({}Hz, {} channels)", 
            self.config.input_sample_rate, self.config.channels);
        Ok(())
    }

    /// Stop capturing
    pub async fn stop(&self) -> VoiceResult<()> {
        let mut capturing = self.is_capturing.lock().await;
        *capturing = false;
        log::info!("Audio capture stopped");
        Ok(())
    }

    /// Check if capturing
    pub async fn is_capturing(&self) -> bool {
        *self.is_capturing.lock().await
    }

    /// Capture audio for duration (seconds)
    pub async fn capture(&self, duration_secs: f32) -> VoiceResult<Vec<i16>> {
        log::info!("Capturing audio for {:.1} seconds", duration_secs);
        
        // Simulate audio capture
        // In real implementation, this would use cpal to capture from microphone
        let sample_count = (self.config.input_sample_rate as f32 * duration_secs) as usize;
        let samples = vec![0i16; sample_count];
        
        Ok(samples)
    }

    /// Get available input devices
    pub fn list_devices() -> VoiceResult<Vec<AudioDevice>> {
        // In real implementation, this would enumerate cpal devices
        Ok(vec![
            AudioDevice {
                name: "Default Microphone".to_string(),
                is_default: true,
                is_input: true,
                is_output: false,
                sample_rates: vec![16000, 44100, 48000],
                channels: 1,
            },
        ])
    }

    /// Get configuration
    pub fn config(&self) -> &AudioSettings {
        &self.config
    }
}

impl Default for AudioCapture {
    fn default() -> Self {
        Self::new(AudioSettings::default())
    }
}

/// Audio playback
pub struct AudioPlayback {
    config: AudioSettings,
    is_playing: Arc<Mutex<bool>>,
}

impl AudioPlayback {
    pub fn new(config: AudioSettings) -> Self {
        Self {
            config,
            is_playing: Arc::new(Mutex::new(false)),
        }
    }

    /// Play audio data
    pub async fn play(&self, audio: &[u8]) -> VoiceResult<()> {
        let mut playing = self.is_playing.lock().await;
        *playing = true;
        drop(playing);

        log::info!("Playing {} bytes of audio", audio.len());

        // In real implementation, this would use cpal to play audio
        // Simulate playback duration
        let duration_ms = (audio.len() as f64 / self.config.output_sample_rate as f64 * 1000.0) as u64;
        tokio::time::sleep(std::time::Duration::from_millis(duration_ms.min(1000))).await;

        let mut playing = self.is_playing.lock().await;
        *playing = false;
        Ok(())
    }

    /// Play audio from file
    pub async fn play_file(&self, path: &str) -> VoiceResult<()> {
        let audio = tokio::fs::read(path).await?;
        self.play(&audio).await
    }

    /// Stop playback
    pub async fn stop(&self) -> VoiceResult<()> {
        let mut playing = self.is_playing.lock().await;
        *playing = false;
        log::info!("Audio playback stopped");
        Ok(())
    }

    /// Check if playing
    pub async fn is_playing(&self) -> bool {
        *self.is_playing.lock().await
    }

    /// Get available output devices
    pub fn list_devices() -> VoiceResult<Vec<AudioDevice>> {
        Ok(vec![
            AudioDevice {
                name: "Default Speaker".to_string(),
                is_default: true,
                is_input: false,
                is_output: true,
                sample_rates: vec![44100, 48000],
                channels: 2,
            },
        ])
    }
}

impl Default for AudioPlayback {
    fn default() -> Self {
        Self::new(AudioSettings::default())
    }
}

/// Audio device info
#[derive(Debug, Clone)]
pub struct AudioDevice {
    pub name: String,
    pub is_default: bool,
    pub is_input: bool,
    pub is_output: bool,
    pub sample_rates: Vec<u32>,
    pub channels: u16,
}

impl Default for AudioDevice {
    fn default() -> Self {
        Self {
            name: "Default".to_string(),
            is_default: true,
            is_input: false,
            is_output: true,
            sample_rates: vec![48000],
            channels: 2,
        }
    }
}

/// Audio configuration
#[derive(Debug, Clone)]
pub struct AudioConfig {
    pub settings: AudioSettings,
}

impl AudioConfig {
    pub fn new(settings: AudioSettings) -> Self {
        Self { settings }
    }
}

impl Default for AudioConfig {
    fn default() -> Self {
        Self::new(AudioSettings::default())
    }
}
