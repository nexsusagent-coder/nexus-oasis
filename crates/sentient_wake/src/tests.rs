//! ─── SENTIENT Wake Word Tests ───

#[cfg(test)]
mod tests {
    use crate::config::{WakeWordConfig, WakeEngine};

    #[test]
    fn test_wake_config_default() {
        let config = WakeWordConfig::default();
        
        assert_eq!(config.wake_word, "sentient");
        assert_eq!(config.engine, WakeEngine::Vosk);
        assert_eq!(config.sensitivity, 0.5);
        assert_eq!(config.sample_rate, 16000);
    }

    #[test]
    fn test_porcupine_config() {
        let config = WakeWordConfig::porcupine("test-key");
        assert_eq!(config.engine, WakeEngine::Porcupine);
        assert_eq!(config.porcupine_access_key, Some("test-key".to_string()));
    }

    #[test]
    fn test_vosk_config() {
        let config = WakeWordConfig::vosk("/path/to/model");
        assert_eq!(config.engine, WakeEngine::Vosk);
        assert_eq!(config.vosk_model_path, Some("/path/to/model".to_string()));
    }

    #[test]
    fn test_simple_config() {
        let config = WakeWordConfig::simple();
        assert_eq!(config.engine, WakeEngine::Simple);
    }
}
