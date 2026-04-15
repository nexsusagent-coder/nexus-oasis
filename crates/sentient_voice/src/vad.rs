//! Voice Activity Detection (VAD)

use crate::{VoiceError, VoiceResult};

/// VAD configuration
#[derive(Debug, Clone)]
pub struct VadConfig {
    /// Sensitivity (0.0 - 1.0)
    pub sensitivity: f32,
    /// Frame size in samples
    pub frame_size: usize,
    /// Sample rate
    pub sample_rate: u32,
    /// Silence threshold in dB
    pub silence_threshold_db: f32,
    /// Min speech duration (frames)
    pub min_speech_frames: usize,
    /// Max silence duration (frames)
    pub max_silence_frames: usize,
}

impl Default for VadConfig {
    fn default() -> Self {
        Self {
            sensitivity: 0.5,
            frame_size: 480,  // 30ms at 16kHz
            sample_rate: 16000,
            silence_threshold_db: -35.0,
            min_speech_frames: 5,
            max_silence_frames: 15,
        }
    }
}

/// Voice Activity Detector
pub struct VoiceActivityDetector {
    config: VadConfig,
    state: VadState,
}

/// VAD state
#[derive(Debug, Clone)]
pub struct VadState {
    pub is_speech: bool,
    pub speech_frames: usize,
    pub silence_frames: usize,
    pub total_frames: usize,
    pub energy_level: f32,
}

impl Default for VadState {
    fn default() -> Self {
        Self {
            is_speech: false,
            speech_frames: 0,
            silence_frames: 0,
            total_frames: 0,
            energy_level: 0.0,
        }
    }
}

impl VoiceActivityDetector {
    pub fn new(config: VadConfig) -> Self {
        Self {
            config,
            state: VadState::default(),
        }
    }

    /// Process audio frame
    pub fn process(&mut self, frame: &[i16]) -> VoiceResult<VadResult> {
        self.state.total_frames += 1;

        // Calculate energy (RMS)
        let sum: f64 = frame.iter().map(|&s| (s as f64).powi(2)).sum();
        let rms = (sum / frame.len() as f64).sqrt();
        
        // Convert to dB
        let db = if rms > 0.0 {
            20.0 * rms.log10() - 96.0  // Normalize to dB range
        } else {
            -96.0
        };

        self.state.energy_level = db as f32;

        // Determine if speech
        let threshold = self.config.silence_threshold_db 
            + (1.0 - self.config.sensitivity) * 20.0;
        
        let is_speech_frame = db > threshold as f64;

        // Update state
        if is_speech_frame {
            self.state.speech_frames += 1;
            self.state.silence_frames = 0;
        } else {
            self.state.silence_frames += 1;
        }

        // Determine current speech state
        let was_speech = self.state.is_speech;
        
        if !self.state.is_speech && self.state.speech_frames >= self.config.min_speech_frames {
            self.state.is_speech = true;
        } else if self.state.is_speech && self.state.silence_frames >= self.config.max_silence_frames {
            self.state.is_speech = false;
        }

        Ok(VadResult {
            is_speech: self.state.is_speech,
            is_speech_start: !was_speech && self.state.is_speech,
            is_speech_end: was_speech && !self.state.is_speech,
            energy_db: db as f32,
        })
    }

    /// Reset state
    pub fn reset(&mut self) {
        self.state = VadState::default();
    }

    /// Get current state
    pub fn state(&self) -> &VadState {
        &self.state
    }

    /// Check if currently in speech
    pub fn is_speech(&self) -> bool {
        self.state.is_speech
    }

    /// Get energy level
    pub fn energy(&self) -> f32 {
        self.state.energy_level
    }
}

impl Default for VoiceActivityDetector {
    fn default() -> Self {
        Self::new(VadConfig::default())
    }
}

/// VAD result for a frame
#[derive(Debug, Clone)]
pub struct VadResult {
    pub is_speech: bool,
    pub is_speech_start: bool,
    pub is_speech_end: bool,
    pub energy_db: f32,
}

impl VadResult {
    pub fn silence() -> Self {
        Self {
            is_speech: false,
            is_speech_start: false,
            is_speech_end: false,
            energy_db: -96.0,
        }
    }
}
