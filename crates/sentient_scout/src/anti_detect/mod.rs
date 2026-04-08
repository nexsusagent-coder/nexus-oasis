//! ─── ANTI-DETECTION ───
//!
//! Bot algilama sistemlerini atlatma mekanizmalari

use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};

/// Browser parmak izi
#[derive(Debug, Clone)]
pub struct BrowserFingerprint {
    /// User Agent
    pub user_agent: String,
    /// Canvas hash
    pub canvas_hash: String,
    /// WebGL renderer
    pub webgl_renderer: String,
    /// Ekran cozunurlugu
    pub screen_resolution: (u32, u32),
    /// Timezone offset (dakika)
    pub timezone_offset: i32,
    /// Dil
    pub language: String,
    /// Platform
    pub platform: String,
    /// Plugin listesi
    pub plugins: Vec<String>,
    /// Font listesi
    pub fonts: Vec<String>,
}

impl BrowserFingerprint {
    /// Rastgele parmak izi olustur
    pub fn random() -> Self {
        let user_agents = vec![
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:121.0) Gecko/20100101 Firefox/121.0",
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.2 Safari/605.1.15",
        ];
        
        let screens = vec![(1920, 1080), (2560, 1440), (1366, 768), (1536, 864)];
        let screen = screens[fastrand::usize(0..screens.len())];
        
        let platforms = vec!["Win32", "MacIntelT64", "Linux x86_64"];
        
        Self {
            user_agent: user_agents[fastrand::usize(0..user_agents.len())].into(),
            canvas_hash: generate_canvas_hash(),
            webgl_renderer: "ANGLE (NVIDIA)".into(),
            screen_resolution: screen,
            timezone_offset: -180, // UTC+3
            language: "tr-TR".into(),
            platform: platforms[fastrand::usize(0..platforms.len())].into(),
            plugins: vec!["Chrome PDF Plugin".into()],
            fonts: default_fonts(),
        }
    }
    
    /// Jasmine fingerprint'i donustur
    pub fn to_headers(&self) -> HashMap<String, String> {
        let mut headers = HashMap::new();
        headers.insert("User-Agent".into(), self.user_agent.clone());
        headers.insert("Accept-Language".into(), format!("{},en;q=0.9", self.language));
        headers.insert("Accept".into(), "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8".into());
        headers.insert("Connection".into(), "keep-alive".into());
        headers
    }
}

fn generate_canvas_hash() -> String {
    use sha2::{Sha256, Digest};
    let noise: u64 = fastrand::u64(0..u64::MAX);
    let mut hasher = Sha256::new();
    hasher.update(noise.to_le_bytes());
    format!("{:x}", hasher.finalize())
}

fn default_fonts() -> Vec<String> {
    vec![
        "Arial".into(),
        "Helvetica".into(),
        "Times New Roman".into(),
        "Georgia".into(),
    ]
}

/// Mouse hareket simulasyonu
pub struct MouseSimulator {
    path: Vec<(f32, f32)>,
    current_index: usize,
}

impl MouseSimulator {
    /// Baslangic ve bitis noktaları arasinda dogal bir yol olustur
    pub fn new(start: (f32, f32), end: (f32, f32)) -> Self {
        let path = generate_mouse_path(start, end);
        Self { path, current_index: 0 }
    }
    
    /// Sonraki noktayi al
    pub fn next(&mut self) -> Option<(f32, f32)> {
        if self.current_index < self.path.len() {
            let point = self.path[self.current_index];
            self.current_index += 1;
            Some(point)
        } else {
            None
        }
    }
}

fn generate_mouse_path(start: (f32, f32), end: (f32, f32)) -> Vec<(f32, f32)> {
    let mut path = vec![start];
    let distance = ((end.0 - start.0).powi(2) + (end.1 - start.1).powi(2)).sqrt();
    let steps = (distance / 10.0).max(5.0) as usize;
    
    for i in 1..steps {
        let t = i as f32 / steps as f32;
        // Bezier curve approximation
        let noise = (fastrand::f32() - 0.5) * 5.0;
        let x = start.0 + (end.0 - start.0) * t + noise;
        let y = start.1 + (end.1 - start.1) * t + noise;
        path.push((x, y));
    }
    
    path.push(end);
    path
}

/// Behavior pattern - Insan davranisi simulasyonu
pub struct BehaviorPattern {
    /// Sayfada kalma suresi araligi (ms)
    pub page_duration_range: (u32, u32),
    /// Scroll hizi araligi
    pub scroll_speed_range: (u32, u32),
    /// Click araligi (ms)
    pub click_interval_range: (u32, u32),
    /// Mouse hiz araligi
    pub mouse_speed_range: (f32, f32),
}

impl Default for BehaviorPattern {
    fn default() -> Self {
        Self {
            page_duration_range: (5000, 30000),
            scroll_speed_range: (100, 500),
            click_interval_range: (500, 2000),
            mouse_speed_range: (200.0, 800.0),
        }
    }
}

impl BehaviorPattern {
    /// Rastgele bekleme suresi olustur
    pub fn random_delay(&self) -> std::time::Duration {
        let min = self.page_duration_range.0;
        let max = self.page_duration_range.1;
        let delay = min + fastrand::u32(0..(max - min));
        std::time::Duration::from_millis(delay as u64)
    }
}

/// Request rotasyonu
pub struct RequestRotator {
    fingerprint_counter: AtomicU32,
    fingerprints: Vec<BrowserFingerprint>,
}

impl RequestRotator {
    pub fn new(fingerprint_count: usize) -> Self {
        let fingerprints = (0..fingerprint_count)
            .map(|_| BrowserFingerprint::random())
            .collect();
        
        Self {
            fingerprint_counter: AtomicU32::new(0),
            fingerprints,
        }
    }
    
    /// Sonraki fingerprint'i al
    pub fn next(&self) -> &BrowserFingerprint {
        let index = self.fingerprint_counter.fetch_add(1, Ordering::Relaxed);
        &self.fingerprints[index as usize % self.fingerprints.len()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_fingerprint_random() {
        let fp1 = BrowserFingerprint::random();
        let fp2 = BrowserFingerprint::random();
        
        // Fingerprint'ler farkli olmali
        assert!(fp1.user_agent != fp2.user_agent || fp1.canvas_hash != fp2.canvas_hash);
    }
    
    #[test]
    fn test_mouse_simulator() {
        let mut sim = MouseSimulator::new((0.0, 0.0), (100.0, 100.0));
        let mut count = 0;
        
        while sim.next().is_some() {
            count += 1;
        }
        
        assert!(count > 2);
    }
}
