//! ─── Emotion Detection System ───
//!
//! Voice emotion analysis using Hume AI API

use serde::{Deserialize, Serialize};

/// Emotion detection result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionResult {
    pub primary_emotion: Emotion,
    pub confidence: f64,
    pub secondary_emotions: Vec<(Emotion, f64)>,
    pub valence: f64,        // -1 (negative) to +1 (positive)
    pub arousal: f64,        // 0 (calm) to 1 (excited)
    pub dominance: f64,      // 0 (submissive) to 1 (dominant)
}

/// Emotion types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Emotion {
    Joy,
    Sadness,
    Anger,
    Fear,
    Surprise,
    Disgust,
    Neutral,
    Excitement,
    Calm,
    Stress,
    Frustration,
    Confusion,
    Confidence,
    Enthusiasm,
    Boredom,
    Interest,
}

impl Emotion {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Joy => "😊 Joy",
            Self::Sadness => "😢 Sadness",
            Self::Anger => "😠 Anger",
            Self::Fear => "😨 Fear",
            Self::Surprise => "😲 Surprise",
            Self::Disgust => "🤢 Disgust",
            Self::Neutral => "😐 Neutral",
            Self::Excitement => "🤩 Excitement",
            Self::Calm => "😌 Calm",
            Self::Stress => "😰 Stress",
            Self::Frustration => "😤 Frustration",
            Self::Confusion => "😕 Confusion",
            Self::Confidence => "😎 Confidence",
            Self::Enthusiasm => "🤗 Enthusiasm",
            Self::Boredom => "😑 Boredom",
            Self::Interest => "🤔 Interest",
        }
    }
    
    pub fn emoji(&self) -> &'static str {
        match self {
            Self::Joy => "😊",
            Self::Sadness => "😢",
            Self::Anger => "😠",
            Self::Fear => "😨",
            Self::Surprise => "😲",
            Self::Disgust => "🤢",
            Self::Neutral => "😐",
            Self::Excitement => "🤩",
            Self::Calm => "😌",
            Self::Stress => "😰",
            Self::Frustration => "😤",
            Self::Confusion => "😕",
            Self::Confidence => "😎",
            Self::Enthusiasm => "🤗",
            Self::Boredom => "😑",
            Self::Interest => "🤔",
        }
    }
    
    /// Check if emotion is negative
    pub fn is_negative(&self) -> bool {
        matches!(self, Self::Sadness | Self::Anger | Self::Fear | Self::Disgust | Self::Stress | Self::Frustration | Self::Confusion | Self::Boredom)
    }
    
    /// Check if emotion is positive
    pub fn is_positive(&self) -> bool {
        matches!(self, Self::Joy | Self::Excitement | Self::Calm | Self::Confidence | Self::Enthusiasm | Self::Interest)
    }
}

/// Emotion detector configuration
#[derive(Debug, Clone)]
pub struct EmotionConfig {
    pub hume_api_key: Option<String>,
    pub use_local_model: bool,
    pub sample_rate: u32,
}

impl Default for EmotionConfig {
    fn default() -> Self {
        Self {
            hume_api_key: std::env::var("HUME_API_KEY").ok(),
            use_local_model: false,
            sample_rate: 16000,
        }
    }
}

/// Emotion detector
pub struct EmotionDetector {
    config: EmotionConfig,
    http: reqwest::Client,
    history: Vec<EmotionResult>,
}

impl EmotionDetector {
    /// Create new emotion detector
    pub fn new() -> Self {
        Self {
            config: EmotionConfig::default(),
            http: reqwest::Client::new(),
            history: vec![],
        }
    }
    
    /// Create with configuration
    pub fn with_config(config: EmotionConfig) -> Self {
        Self {
            config,
            http: reqwest::Client::new(),
            history: vec![],
        }
    }
    
    /// Detect emotion from audio
    pub async fn detect(&mut self, audio: &[f32]) -> crate::VoiceResult<EmotionResult> {
        let result = if let Some(_api_key) = &self.config.hume_api_key {
            self.detect_via_hume_api(audio).await?
        } else {
            self.detect_local(audio).await?
        };
        
        self.history.push(result.clone());
        Ok(result)
    }
    
    /// Detect emotion from text (alternative method)
    pub async fn detect_from_text(&mut self, text: &str) -> crate::VoiceResult<EmotionResult> {
        let result = self.analyze_text_emotion(text);
        self.history.push(result.clone());
        Ok(result)
    }
    
    /// Get emotion history
    pub fn get_history(&self) -> &[EmotionResult] {
        &self.history
    }
    
    /// Get average emotion over recent samples
    pub fn get_average_emotion(&self, samples: usize) -> Option<EmotionResult> {
        if self.history.is_empty() {
            return None;
        }
        
        let recent: Vec<_> = self.history.iter().rev().take(samples).collect();
        let count = recent.len();
        
        let avg_valence = recent.iter().map(|r| r.valence).sum::<f64>() / count as f64;
        let avg_arousal = recent.iter().map(|r| r.arousal).sum::<f64>() / count as f64;
        let avg_dominance = recent.iter().map(|r| r.dominance).sum::<f64>() / count as f64;
        
        // Find most common primary emotion
        let mut emotion_counts: std::collections::HashMap<Emotion, u32> = std::collections::HashMap::new();
        for r in &recent {
            *emotion_counts.entry(r.primary_emotion).or_insert(0) += 1;
        }
        
        let primary = emotion_counts.into_iter()
            .max_by_key(|(_, count)| *count)
            .map(|(e, _)| e)
            .unwrap_or(Emotion::Neutral);
        
        Some(EmotionResult {
            primary_emotion: primary,
            confidence: recent.iter().map(|r| r.confidence).sum::<f64>() / count as f64,
            secondary_emotions: vec![],
            valence: avg_valence,
            arousal: avg_arousal,
            dominance: avg_dominance,
        })
    }
    
    /// Clear history
    pub fn clear_history(&mut self) {
        self.history.clear();
    }
    
    // Internal methods
    
    async fn detect_via_hume_api(&self, _audio: &[f32]) -> crate::VoiceResult<EmotionResult> {
        // TODO: Implement Hume AI API call
        // https://api.hume.ai/v0/models/batch
        tracing::info!("Detecting emotion via Hume AI API");
        
        // Placeholder - would make actual API call
        Ok(EmotionResult {
            primary_emotion: Emotion::Neutral,
            confidence: 0.5,
            secondary_emotions: vec![],
            valence: 0.0,
            arousal: 0.3,
            dominance: 0.5,
        })
    }
    
    async fn detect_local(&self, _audio: &[f32]) -> crate::VoiceResult<EmotionResult> {
        // TODO: Implement local emotion detection model
        // Could use a lightweight model or audio features
        
        Ok(EmotionResult {
            primary_emotion: Emotion::Neutral,
            confidence: 0.5,
            secondary_emotions: vec![],
            valence: 0.0,
            arousal: 0.3,
            dominance: 0.5,
        })
    }
    
    fn analyze_text_emotion(&self, text: &str) -> EmotionResult {
        let text_lower = text.to_lowercase();
        
        // Keyword-based emotion detection
        let mut scores: std::collections::HashMap<Emotion, f64> = std::collections::HashMap::new();
        
        // Joy keywords
        let joy_words = ["happy", "great", "awesome", "wonderful", "love", "excited", "amazing", "fantastic", "mutlu", "harika", "süper"];
        if joy_words.iter().any(|w| text_lower.contains(w)) {
            scores.insert(Emotion::Joy, 0.8);
        }
        
        // Sadness keywords
        let sad_words = ["sad", "unhappy", "depressed", "crying", "tears", "upset", "üzgün", "mutsuz", "ağlıyorum"];
        if sad_words.iter().any(|w| text_lower.contains(w)) {
            scores.insert(Emotion::Sadness, 0.8);
        }
        
        // Anger keywords
        let anger_words = ["angry", "furious", "mad", "hate", "annoyed", "frustrated", "kızgın", "öfkeli", "sinirli"];
        if anger_words.iter().any(|w| text_lower.contains(w)) {
            scores.insert(Emotion::Anger, 0.8);
        }
        
        // Stress keywords
        let stress_words = ["stressed", "anxious", "worried", "overwhelmed", "pressure", "stresli", "kaygılı", "endişeli"];
        if stress_words.iter().any(|w| text_lower.contains(w)) {
            scores.insert(Emotion::Stress, 0.8);
        }
        
        // Excitement keywords
        let excite_words = ["excited", "thrilled", "pumped", "cant wait", "heyecanlı", "sabırsızlanıyorum"];
        if excite_words.iter().any(|w| text_lower.contains(w)) {
            scores.insert(Emotion::Excitement, 0.8);
        }
        
        // Calm keywords
        let calm_words = ["calm", "relaxed", "peaceful", "serene", "tranquil", "sakin", "huzurlu"];
        if calm_words.iter().any(|w| text_lower.contains(w)) {
            scores.insert(Emotion::Calm, 0.8);
        }
        
        // Frustration keywords
        let frust_words = ["frustrated", "stuck", "not working", "annoying", "bothering", "bunaldım", "çıkmaza girdim"];
        if frust_words.iter().any(|w| text_lower.contains(w)) {
            scores.insert(Emotion::Frustration, 0.8);
        }
        
        // Confidence keywords
        let conf_words = ["confident", "sure", "certain", "definitely", "absolutely", "eminim", "kesinlikle"];
        if conf_words.iter().any(|w| text_lower.contains(w)) {
            scores.insert(Emotion::Confidence, 0.8);
        }
        
        // Find primary emotion
        let (primary, confidence) = scores.iter()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(e, s)| (*e, *s))
            .unwrap_or((Emotion::Neutral, 0.5));
        
        // Calculate valence, arousal, dominance
        let valence = if primary.is_positive() { 0.5 } else if primary.is_negative() { -0.5 } else { 0.0 };
        let arousal = match primary {
            Emotion::Excitement | Emotion::Anger | Emotion::Fear => 0.8,
            Emotion::Calm | Emotion::Boredom => 0.2,
            _ => 0.5,
        };
        let dominance = match primary {
            Emotion::Confidence | Emotion::Anger => 0.7,
            Emotion::Fear | Emotion::Sadness => 0.3,
            _ => 0.5,
        };
        
        // Get secondary emotions
        let secondary: Vec<_> = scores.iter()
            .filter(|(e, _)| **e != primary)
            .map(|(e, s)| (*e, *s))
            .collect();
        
        EmotionResult {
            primary_emotion: primary,
            confidence,
            secondary_emotions: secondary,
            valence,
            arousal,
            dominance,
        }
    }
}

impl Default for EmotionDetector {
    fn default() -> Self {
        Self::new()
    }
}

/// Mood-based response adapter
pub struct MoodAdapter {
    emotion_history: Vec<Emotion>,
}

impl MoodAdapter {
    pub fn new() -> Self {
        Self { emotion_history: vec![] }
    }
    
    /// Add emotion to history
    pub fn add_emotion(&mut self, emotion: Emotion) {
        self.emotion_history.push(emotion);
        if self.emotion_history.len() > 10 {
            self.emotion_history.remove(0);
        }
    }
    
    /// Get current mood
    pub fn get_current_mood(&self) -> Mood {
        if self.emotion_history.is_empty() {
            return Mood::Neutral;
        }
        
        let recent = &self.emotion_history[self.emotion_history.len().saturating_sub(3)..];
        
        // Check for patterns
        if recent.iter().all(|e| e.is_negative()) {
            Mood::Negative
        } else if recent.iter().all(|e| e.is_positive()) {
            Mood::Positive
        } else if recent.iter().any(|e| matches!(e, Emotion::Stress | Emotion::Frustration)) {
            Mood::Stressed
        } else {
            Mood::Neutral
        }
    }
    
    /// Get response style based on mood
    pub fn get_response_style(&self) -> ResponseStyle {
        match self.get_current_mood() {
            Mood::Positive => ResponseStyle::Enthusiastic,
            Mood::Negative => ResponseStyle::Empathetic,
            Mood::Stressed => ResponseStyle::Calm,
            Mood::Neutral => ResponseStyle::Normal,
        }
    }
    
    /// Get suggested break message
    pub fn get_break_suggestion(&self) -> Option<String> {
        let mood = self.get_current_mood();
        
        match mood {
            Mood::Stressed => Some("You seem stressed. Would you like to take a short break? 🧘".into()),
            Mood::Negative => Some("I notice you might be having a tough time. Remember, it's okay to take breaks. 💪".into()),
            _ => None,
        }
    }
}

impl Default for MoodAdapter {
    fn default() -> Self {
        Self::new()
    }
}

/// Mood state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mood {
    Positive,
    Negative,
    Stressed,
    Neutral,
}

/// Response style
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResponseStyle {
    Enthusiastic,
    Empathetic,
    Calm,
    Normal,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_emotion_is_positive() {
        assert!(Emotion::Joy.is_positive());
        assert!(!Emotion::Anger.is_positive());
    }
    
    #[test]
    fn test_text_emotion_analysis() {
        let detector = EmotionDetector::new();
        let result = detector.analyze_text_emotion("I am so happy and excited!");
        
        assert!(result.primary_emotion == Emotion::Joy || result.primary_emotion == Emotion::Excitement);
    }
    
    #[test]
    fn test_mood_adapter() {
        let mut adapter = MoodAdapter::new();
        adapter.add_emotion(Emotion::Stress);
        adapter.add_emotion(Emotion::Frustration);
        
        assert_eq!(adapter.get_current_mood(), Mood::Stressed);
        assert!(adapter.get_break_suggestion().is_some());
    }
}
