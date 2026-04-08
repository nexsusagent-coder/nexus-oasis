//! ═══════════════════════════════════════════════════════════════════════════════
//!  STEALTH ENGINE - Anti-Detection & Fingerprint Masking
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Bot tespit sistemlerini atlamak için parmak izi maskeleme.

use rand::seq::SliceRandom;
use rand::Rng;
use serde::{Deserialize, Serialize};

/// Stealth yapılandırması
#[derive(Debug, Clone)]
pub struct StealthConfig {
    /// User-Agent rotation
    pub rotate_user_agent: bool,
    /// WebGL fingerprint maskeleme
    pub mask_webgl: bool,
    /// Canvas fingerprint maskeleme
    pub mask_canvas: bool,
    /// Audio fingerprint maskeleme
    pub mask_audio: bool,
    /// Navigator özellikleri maskeleme
    pub mask_navigator: bool,
    /// Screen resolution maskeleme
    pub mask_screen: bool,
    /// Zamanlama rastgeleliği
    pub randomize_timing: bool,
    /// Mouse hareket simülasyonu
    pub simulate_mouse: bool,
    /// Scroll davranışı simülasyonu
    pub simulate_scroll: bool,
}

impl Default for StealthConfig {
    fn default() -> Self {
        Self {
            rotate_user_agent: true,
            mask_webgl: true,
            mask_canvas: true,
            mask_audio: true,
            mask_navigator: true,
            mask_screen: true,
            randomize_timing: true,
            simulate_mouse: true,
            simulate_scroll: true,
        }
    }
}

/// Browser fingerprint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fingerprint {
    /// User-Agent
    pub user_agent: String,
    /// Platform
    pub platform: String,
    /// Language
    pub language: String,
    /// Screen width
    pub screen_width: u32,
    /// Screen height
    pub screen_height: u32,
    /// Device pixel ratio
    pub pixel_ratio: f64,
    /// Color depth
    pub color_depth: u32,
    /// Timezone
    pub timezone: String,
    /// WebGL vendor
    pub webgl_vendor: String,
    /// WebGL renderer
    pub webgl_renderer: String,
    /// Canvas hash (rastgele)
    pub canvas_hash: String,
    /// Audio hash (rastgele)
    pub audio_hash: String,
}

/// Stealth Engine
pub struct StealthEngine {
    config: StealthConfig,
    user_agents: Vec<String>,
    platforms: Vec<String>,
    screen_resolutions: Vec<(u32, u32)>,
    timezones: Vec<String>,
}

impl StealthEngine {
    pub fn new(config: StealthConfig) -> Self {
        Self {
            config,
            user_agents: COMMON_USER_AGENTS.iter().map(|s| s.to_string()).collect(),
            platforms: vec!["Linux x86_64".into(), "Windows NT 10.0".into(), "Macintosh".into()],
            screen_resolutions: vec![
                (1920, 1080),
                (2560, 1440),
                (1366, 768),
                (1536, 864),
            ],
            timezones: vec![
                "Europe/Istanbul".into(),
                "Europe/London".into(),
                "America/New_York".into(),
            ],
        }
    }
    
    /// Rastgele fingerprint oluştur
    pub fn generate_fingerprint(&self) -> Fingerprint {
        let mut rng = rand::thread_rng();
        
        Fingerprint {
            user_agent: self.user_agents.choose(&mut rng).unwrap().clone(),
            platform: self.platforms.choose(&mut rng).unwrap().clone(),
            language: "en-US".into(),
            screen_width: 1920,
            screen_height: 1080,
            pixel_ratio: 1.0,
            color_depth: 24,
            timezone: self.timezones.choose(&mut rng).unwrap().clone(),
            webgl_vendor: "Google Inc. (NVIDIA)".into(),
            webgl_renderer: "ANGLE (NVIDIA, NVIDIA GeForce GTX 1080 Direct3D11 vs_5_0 ps_5_0)".into(),
            canvas_hash: Self::random_hash(&mut rng),
            audio_hash: Self::random_hash(&mut rng),
        }
    }
    
    /// Rastgele hash oluştur
    fn random_hash(rng: &mut impl Rng) -> String {
        (0..32)
            .map(|_| format!("{:02x}", rng.gen_range(0..255)))
            .collect()
    }
    
    /// Rastgele gecikme (insan taklidi)
    pub fn human_delay(&self) -> std::time::Duration {
        if !self.config.randomize_timing {
            return std::time::Duration::from_millis(100);
        }
        
        let mut rng = rand::thread_rng();
        let base_ms = rng.gen_range(50..500);
        std::time::Duration::from_millis(base_ms)
    }
    
    /// Mouse hareketi simüle et
    pub fn generate_mouse_path(&self, from: (f64, f64), to: (f64, f64)) -> Vec<(f64, f64)> {
        if !self.config.simulate_mouse {
            return vec![to];
        }
        
        let mut path = vec![from];
        let steps = rand::thread_rng().gen_range(5..15);
        
        for i in 1..=steps {
            let progress = i as f64 / steps as f64;
            let x = from.0 + (to.0 - from.0) * progress;
            let y = from.1 + (to.1 - from.1) * progress;
            
            // Hafif rastgelelik ekle
            let jitter_x = (rand::thread_rng().gen::<f64>() - 0.5) * 5.0;
            let jitter_y = (rand::thread_rng().gen::<f64>() - 0.5) * 5.0;
            
            path.push((x + jitter_x, y + jitter_y));
        }
        
        path
    }
    
    /// JavaScript injection script'i oluştur
    pub fn generate_injection_script(&self, fingerprint: &Fingerprint) -> String {
        format!(r#"
// SENTIENT Stealth Injection
Object.defineProperty(navigator, 'userAgent', {{
    get: () => '{}'
}});
Object.defineProperty(navigator, 'platform', {{
    get: () => '{}'
}});
Object.defineProperty(screen, 'width', {{
    get: () => {}
}});
Object.defineProperty(screen, 'height', {{
    get: () => {}
}});
Object.defineProperty(screen, 'colorDepth', {{
    get: () => {}
}});
"#,
            fingerprint.user_agent,
            fingerprint.platform,
            fingerprint.screen_width,
            fingerprint.screen_height,
            fingerprint.color_depth
        )
    }
}

/// Yaygın User-Agent'lar
const COMMON_USER_AGENTS: &[&str] = &[
    "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36",
    "Mozilla/5.0 (X11; Linux x86_64; rv:123.0) Gecko/20100101 Firefox/123.0",
];

impl Default for StealthEngine {
    fn default() -> Self {
        Self::new(StealthConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_fingerprint_generation() {
        let engine = StealthEngine::default();
        let fp = engine.generate_fingerprint();
        
        assert!(!fp.user_agent.is_empty());
        assert!(!fp.platform.is_empty());
        assert!(fp.screen_width > 0);
    }
    
    #[test]
    fn test_human_delay() {
        let engine = StealthEngine::default();
        let delay = engine.human_delay();
        
        assert!(delay.as_millis() >= 50);
        assert!(delay.as_millis() <= 500);
    }
    
    #[test]
    fn test_mouse_path() {
        let engine = StealthEngine::default();
        let path = engine.generate_mouse_path((0.0, 0.0), (100.0, 100.0));
        
        assert!(path.len() > 1);
        assert_eq!(path.first().unwrap(), &(0.0, 0.0));
    }
}
