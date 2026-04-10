//! ─── Text-to-Speech ───

use async_trait::async_trait;
use reqwest::Client;
use crate::VoiceError;

/// Speech synthesis result
#[derive(Debug, Clone)]
pub struct SpeechResult {
    /// Audio data (PCM format)
    pub audio: Vec<f32>,
    
    /// Sample rate
    pub sample_rate: u32,
    
    /// Duration in seconds
    pub duration_secs: f32,
    
    /// Voice used
    pub voice: String,
}

/// Text-to-Speech trait
#[async_trait]
pub trait TextToSpeech: Send + Sync {
    /// Synthesize text to speech
    async fn synthesize(&self, text: &str) -> Result<SpeechResult, VoiceError>;
    
    /// Get available voices
    fn voices(&self) -> Vec<String>;
    
    /// Set voice
    fn set_voice(&mut self, voice: &str);
}

/// OpenAI TTS
pub struct OpenAiTts {
    api_key: String,
    voice: String,
    client: Client,
    model: String,
}

impl OpenAiTts {
    pub fn new(api_key: String, voice: String) -> Self {
        Self {
            api_key,
            voice,
            client: Client::new(),
            model: "tts-1".into(),
        }
    }
}

#[async_trait]
impl TextToSpeech for OpenAiTts {
    async fn synthesize(&self, text: &str) -> Result<SpeechResult, VoiceError> {
        let request = serde_json::json!({
            "model": self.model,
            "input": text,
            "voice": self.voice,
            "response_format": "pcm"
        });
        
        let response = self.client
            .post("https://api.openai.com/v1/audio/speech")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| VoiceError::NetworkError(e.to_string()))?;
        
        if !response.status().is_success() {
            let error = response.text().await.unwrap_or_default();
            return Err(VoiceError::ApiError(error));
        }
        
        let bytes = response.bytes().await
            .map_err(|e| VoiceError::NetworkError(e.to_string()))?;
        
        // Convert PCM bytes to float
        let audio: Vec<f32> = bytes
            .chunks(2)
            .map(|chunk| {
                let sample = i16::from_le_bytes([chunk[0], chunk[1]]);
                sample as f32 / 32768.0
            })
            .collect();
        
        let duration_secs = audio.len() as f32 / 24000.0; // 24kHz sample rate
        
        Ok(SpeechResult {
            audio,
            sample_rate: 24000,
            duration_secs,
            voice: self.voice.clone(),
        })
    }
    
    fn voices(&self) -> Vec<String> {
        vec!["alloy".into(), "echo".into(), "fable".into(), "onyx".into(), "nova".into(), "shimmer".into()]
    }
    
    fn set_voice(&mut self, voice: &str) {
        self.voice = voice.into();
    }
}

/// ElevenLabs TTS
pub struct ElevenLabsTts {
    api_key: String,
    voice_id: String,
    client: Client,
}

impl ElevenLabsTts {
    pub fn new(api_key: String, voice_id: String) -> Self {
        Self {
            api_key,
            voice_id,
            client: Client::new(),
        }
    }
}

#[async_trait]
impl TextToSpeech for ElevenLabsTts {
    async fn synthesize(&self, text: &str) -> Result<SpeechResult, VoiceError> {
        let request = serde_json::json!({
            "text": text,
            "model_id": "eleven_monolingual_v1",
            "voice_settings": {
                "stability": 0.5,
                "similarity_boost": 0.5
            }
        });
        
        let response = self.client
            .post(format!("https://api.elevenlabs.io/v1/text-to-speech/{}", self.voice_id))
            .header("xi-api-key", &self.api_key)
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| VoiceError::NetworkError(e.to_string()))?;
        
        if !response.status().is_success() {
            let error = response.text().await.unwrap_or_default();
            return Err(VoiceError::ApiError(error));
        }
        
        let bytes = response.bytes().await
            .map_err(|e| VoiceError::NetworkError(e.to_string()))?;
        
        // Convert MP3 to float (simplified - would need proper MP3 decoder)
        let audio = mp3_to_float(&bytes)?;
        
        Ok(SpeechResult {
            audio,
            sample_rate: 44100,
            duration_secs: 0.0, // Calculate from audio length
            voice: self.voice_id.clone(),
        })
    }
    
    fn voices(&self) -> Vec<String> {
        vec!["21m00Tc4RViOy4uUf5iH".into(), "AZnzlk1XvdvUeBnXmlld".into()]
    }
    
    fn set_voice(&mut self, voice_id: &str) {
        self.voice_id = voice_id.into();
    }
}

/// System TTS (fallback)
pub struct SystemTts {
    voice: String,
}

impl SystemTts {
    pub fn new() -> Self {
        Self {
            voice: "default".into(),
        }
    }
}

#[async_trait]
impl TextToSpeech for SystemTts {
    async fn synthesize(&self, text: &str) -> Result<SpeechResult, VoiceError> {
        // Use system TTS command
        #[cfg(target_os = "macos")]
        {
            use std::process::Command;
            Command::new("say")
                .arg(text)
                .output()
                .map_err(|e| VoiceError::Internal(e.to_string()))?;
            
            // Note: macOS 'say' doesn't return audio data
            // Would need to use -o flag and convert AIFF to PCM
        }
        
        #[cfg(target_os = "linux")]
        {
            use std::process::Command;
            Command::new("espeak")
                .arg(text)
                .output()
                .ok();
        }
        
        #[cfg(target_os = "windows")]
        {
            use std::process::Command;
            Command::new("powershell")
                .args(["-c", &format!("Add-Type -AssemblyName System.Speech; (New-Object System.Speech.Synthesis.SpeechSynthesizer).Speak('{}')", text)])
                .output()
                .ok();
        }
        
        // Return empty result (system TTS plays directly)
        Ok(SpeechResult {
            audio: Vec::new(),
            sample_rate: 16000,
            duration_secs: text.len() as f32 / 15.0, // Rough estimate
            voice: self.voice.clone(),
        })
    }
    
    fn voices(&self) -> Vec<String> {
        vec!["default".into()]
    }
    
    fn set_voice(&mut self, voice: &str) {
        self.voice = voice.into();
    }
}

impl Default for SystemTts {
    fn default() -> Self {
        Self::new()
    }
}

/// MP3 to float conversion using minimp3
fn mp3_to_float(bytes: &[u8]) -> Result<Vec<f32>, VoiceError> {
    use minimp3::Decoder;
    
    let cursor = std::io::Cursor::new(bytes);
    let mut decoder = Decoder::new(cursor);
    let mut samples: Vec<f32> = Vec::new();
    
    loop {
        match decoder.next_frame() {
            Ok(frame) => {
                // minimp3 0.5+ API: frame.data contains interleaved samples
                // frame.channels: 1=mono, 2=stereo
                let _sample_rate = frame.sample_rate;
                if frame.channels == 1 {
                    // Mono
                    samples.extend(frame.data.iter().map(|&s| s as f32 / 32768.0));
                } else {
                    // Stereo - mix to mono
                    for chunk in frame.data.chunks(2) {
                        let mono = if chunk.len() == 2 {
                            (chunk[0] as i32 + chunk[1] as i32) / 2
                        } else {
                            chunk[0] as i32
                        };
                        samples.push(mono as f32 / 32768.0);
                    }
                }
            }
            Err(minimp3::Error::Eof) => break,
            Err(e) => return Err(VoiceError::AudioError(e.to_string())),
        }
    }
    
    Ok(samples)
}
