//! ─── SENTIENT Voice Tests ───

#[cfg(test)]
mod tests {
    use sentient_voice::{SttConfig, TtsConfig, VoiceProvider};

    #[test]
    fn test_stt_config_default() {
        let config = SttConfig::default();
        
        assert_eq!(config.provider, VoiceProvider::OpenAI);
        assert_eq!(config.model, "whisper-1");
        assert_eq!(config.language, None);
    }

    #[test]
    fn test_tts_config_default() {
        let config = TtsConfig::default();
        
        assert_eq!(config.provider, VoiceProvider::OpenAI);
        assert_eq!(config.voice, "alloy");
        assert_eq!(config.speed, 1.0);
    }

    #[test]
    fn test_voice_provider_from_str() {
        assert!(matches!(VoiceProvider::from_str("openai"), Ok(VoiceProvider::OpenAI)));
        assert!(matches!(VoiceProvider::from_str("elevenlabs"), Ok(VoiceProvider::ElevenLabs)));
        assert!(matches!(VoiceProvider::from_str("system"), Ok(VoiceProvider::System)));
    }
}
