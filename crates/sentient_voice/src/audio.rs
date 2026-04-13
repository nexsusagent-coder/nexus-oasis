//! ─── Audio Processing ───

use serde::{Deserialize, Serialize};

/// Audio format specification
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct AudioFormat {
    pub sample_rate: u32,
    pub channels: u16,
    pub bits_per_sample: u16,
}

impl Default for AudioFormat {
    fn default() -> Self {
        Self {
            sample_rate: 16000,
            channels: 1,
            bits_per_sample: 16,
        }
    }
}

/// Audio buffer
#[derive(Debug, Clone)]
pub struct AudioBuffer {
    pub samples: Vec<f32>,
    pub format: AudioFormat,
    pub duration_secs: f32,
}

impl AudioBuffer {
    /// Create new audio buffer
    pub fn new(samples: Vec<f32>, format: AudioFormat) -> Self {
        let duration_secs = samples.len() as f32 / format.sample_rate as f32;
        Self {
            samples,
            format,
            duration_secs,
        }
    }
    
    /// Create silence
    pub fn silence(format: AudioFormat, duration_secs: f32) -> Self {
        let num_samples = (format.sample_rate as f32 * duration_secs) as usize;
        Self::new(vec![0.0; num_samples], format)
    }
    
    /// Resample to target sample rate
    pub fn resample(&self, target_rate: u32) -> Self {
        if self.format.sample_rate == target_rate {
            return self.clone();
        }
        
        let ratio = target_rate as f64 / self.format.sample_rate as f64;
        let target_len = (self.samples.len() as f64 * ratio) as usize;
        
        let mut resampled = Vec::with_capacity(target_len);
        for i in 0..target_len {
            let src_idx = i as f64 / ratio;
            let idx = src_idx as usize;
            let frac = src_idx - idx as f64;
            
            let sample = if idx + 1 < self.samples.len() {
                self.samples[idx] * (1.0 - frac as f32) + self.samples[idx + 1] * frac as f32
            } else if idx < self.samples.len() {
                self.samples[idx]
            } else {
                0.0
            };
            resampled.push(sample);
        }
        
        Self::new(resampled, AudioFormat {
            sample_rate: target_rate,
            ..self.format
        })
    }
    
    /// Normalize audio
    pub fn normalize(&mut self) {
        let max = self.samples.iter().fold(0.0f32, |a, &b| a.max(b.abs()));
        if max > 0.0 {
            for sample in &mut self.samples {
                *sample /= max;
            }
        }
    }
    
    /// Apply gain
    pub fn apply_gain(&mut self, gain: f32) {
        for sample in &mut self.samples {
            *sample = (*sample * gain).clamp(-1.0, 1.0);
        }
    }
    
    /// Fade in/out
    pub fn fade(&mut self, fade_samples: usize) {
        // Fade in
        for i in 0..fade_samples.min(self.samples.len()) {
            let factor = i as f32 / fade_samples as f32;
            self.samples[i] *= factor;
        }
        
        // Fade out
        let len = self.samples.len();
        for i in 0..fade_samples.min(len) {
            let factor = i as f32 / fade_samples as f32;
            self.samples[len - 1 - i] *= factor;
        }
    }
    
    /// Concatenate buffers
    pub fn concat(&mut self, other: &AudioBuffer) {
        self.samples.extend(&other.samples);
        self.duration_secs = self.samples.len() as f32 / self.format.sample_rate as f32;
    }
    
    /// Split at position
    pub fn split_at(&self, position_secs: f32) -> (AudioBuffer, AudioBuffer) {
        let split_idx = (position_secs * self.format.sample_rate as f32) as usize;
        let split_idx = split_idx.min(self.samples.len());
        
        let first = AudioBuffer::new(
            self.samples[..split_idx].to_vec(),
            self.format,
        );
        let second = AudioBuffer::new(
            self.samples[split_idx..].to_vec(),
            self.format,
        );
        
        (first, second)
    }
    
    /// Get RMS energy
    pub fn rms_energy(&self) -> f32 {
        let sum: f32 = self.samples.iter().map(|s| s * s).sum();
        (sum / self.samples.len() as f32).sqrt()
    }
    
    /// Detect silence
    pub fn is_silence(&self, threshold: f32) -> bool {
        self.rms_energy() < threshold
    }
    
    /// Trim silence from start/end
    pub fn trim_silence(&mut self, threshold: f32) {
        // Find first non-silent sample
        let start = self.samples.iter()
            .position(|&s| s.abs() > threshold)
            .unwrap_or(0);
        
        // Find last non-silent sample
        let end = self.samples.iter()
            .rposition(|&s| s.abs() > threshold)
            .map(|i| i + 1)
            .unwrap_or(self.samples.len());
        
        self.samples = self.samples[start..end].to_vec();
        self.duration_secs = self.samples.len() as f32 / self.format.sample_rate as f32;
    }
}

/// Voice Activity Detection
pub struct VoiceActivityDetector {
    threshold: f32,
    #[allow(dead_code)]
    frame_size: usize,
    energy_history: Vec<f32>,
}

impl VoiceActivityDetector {
    pub fn new(threshold: f32, frame_size: usize) -> Self {
        Self {
            threshold,
            frame_size,
            energy_history: Vec::new(),
        }
    }
    
    /// Process frame and return true if voice detected
    pub fn process(&mut self, frame: &[f32]) -> bool {
        let energy: f32 = frame.iter().map(|s| s * s).sum::<f32>().sqrt() / frame.len() as f32;
        self.energy_history.push(energy);
        
        // Keep last 10 frames
        if self.energy_history.len() > 10 {
            self.energy_history.remove(0);
        }
        
        // Smoothed energy
        let avg_energy: f32 = self.energy_history.iter().sum::<f32>() / self.energy_history.len() as f32;
        
        avg_energy > self.threshold
    }
    
    /// Reset detector
    pub fn reset(&mut self) {
        self.energy_history.clear();
    }
}

impl Default for VoiceActivityDetector {
    fn default() -> Self {
        Self::new(0.01, 512)
    }
}
