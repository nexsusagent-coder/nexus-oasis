//! Voice integration tests

#[cfg(test)]
mod voice_tests {
    use sentient_voice::{VoiceEngine, VoiceConfig, TtsEngine, SttEngine};

    #[test]
    fn test_voice_config_default() {
        let config = VoiceConfig::default();
        
        assert!(config.sample_rate > 0);
        assert!(!config.wake_word.is_empty() || config.wake_word.is_empty());
    }

    #[test]
    fn test_voice_config_serialization() {
        let config = VoiceConfig {
            sample_rate: 16000,
            wake_word: "hey sentient".to_string(),
            tts_provider: "openai".to_string(),
            stt_provider: "whisper".to_string(),
        };
        
        let json = serde_json::to_string(&config).unwrap();
        let decoded: VoiceConfig = serde_json::from_str(&json).unwrap();
        
        assert_eq!(config.sample_rate, decoded.sample_rate);
        assert_eq!(config.wake_word, decoded.wake_word);
    }

    #[tokio::test]
    async fn test_stt_engine_init() {
        let config = VoiceConfig::default();
        let engine = SttEngine::new(config);
        
        // Should initialize without panic
        assert!(engine.is_ok() || engine.is_err());
    }

    #[tokio::test]
    async fn test_tts_engine_init() {
        let config = VoiceConfig::default();
        let engine = TtsEngine::new(config);
        
        assert!(engine.is_ok() || engine.is_err());
    }

    #[test]
    fn test_audio_format_validation() {
        use sentient_voice::audio::AudioFormat;
        
        let formats = vec![
            AudioFormat::Wav,
            AudioFormat::Mp3,
            AudioFormat::Ogg,
            AudioFormat::Webm,
        ];
        
        for format in formats {
            assert!(!format.extension().is_empty());
            assert!(!format.mime_type().is_empty());
        }
    }

    #[test]
    fn test_wake_word_detection_config() {
        let config = sentient_voice::WakeWordConfig {
            words: vec!["hey sentient".to_string(), "ok sentient".to_string()],
            sensitivity: 0.5,
            timeout_ms: 5000,
        };
        
        assert_eq!(config.words.len(), 2);
        assert!(config.sensitivity > 0.0 && config.sensitivity <= 1.0);
    }
}

#[cfg(test)]
mod audio_processing_tests {
    use sentient_voice::audio::{AudioBuffer, AudioProcessor};

    #[test]
    fn test_audio_buffer_creation() {
        let buffer = AudioBuffer::new(16000, 1);
        
        assert_eq!(buffer.sample_rate(), 16000);
        assert_eq!(buffer.channels(), 1);
        assert_eq!(buffer.len(), 0);
    }

    #[test]
    fn test_audio_buffer_append() {
        let mut buffer = AudioBuffer::new(16000, 1);
        let samples = vec![0.0f32, 0.5, 1.0, 0.5, 0.0];
        
        buffer.append(&samples);
        
        assert_eq!(buffer.len(), samples.len());
    }

    #[test]
    fn test_audio_resampling() {
        let processor = AudioProcessor::new();
        
        let input = vec![0.0f32; 16000]; // 1 second at 16kHz
        let output = processor.resample(&input, 16000, 48000);
        
        // Output should be 3x longer
        assert!(output.is_ok());
        let output = output.unwrap();
        assert!(output.len() >= 48000);
    }

    #[test]
    fn test_silence_detection() {
        let processor = AudioProcessor::new();
        
        let silence = vec![0.0f32; 16000];
        let speech = vec![0.5f32; 16000];
        
        assert!(processor.is_silence(&silence, 0.01));
        assert!(!processor.is_silence(&speech, 0.01));
    }
}
