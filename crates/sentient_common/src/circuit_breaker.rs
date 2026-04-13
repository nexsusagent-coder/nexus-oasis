//! ═════════════════════════════════════════════════════════════════
//!  CIRCUIT BREAKER MODULE - Devre Kesici
//! ═════════════════════════════════════════════════════════════════
//!
//! LLM provider hatalarında sistem koruması.
//! Ardışık hatalarda devreyi girer, iyileşme olasılığını test eder.
//!
//! Durumlar:
//! - Closed: Normal çalışma (istekler geçer)
//! - Open: Devre açık (istekler reddedilir)
//! - HalfOpen: Test modu (sınırlı istek denenir)

use std::sync::atomic::{AtomicU64, AtomicU32, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

// ═════════════════════════════════════════════════════════════════
//  DEVRE KESİCİ DURUMLARI
// ═════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CircuitState {
    /// Normal çalışma - istekler geçer
    Closed,
    /// Devre açık - istekler reddedilir
    Open,
    /// Test modu - sınırlı istek denenir
    HalfOpen,
}

impl std::fmt::Display for CircuitState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CircuitState::Closed => write!(f, "✅ KAPALI (Normal)"),
            CircuitState::Open => write!(f, "🔴 AÇIK (Koruma)"),
            CircuitState::HalfOpen => write!(f, "🟡 YARI AÇIK (Test)"),
        }
    }
}

// ═════════════════════════════════════════════════════════════════
//  DEVRE KESİCİ YAPILANDIRMASI
// ═════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CircuitBreakerConfig {
    /// Ardışık hata eşiği (bu sayıya ulaşınca devre açılır)
    pub failure_threshold: u32,
    /// Kurtarma bekleme süresi (ms) - Open -> HalfOpen geçişi
    pub recovery_timeout_ms: u64,
    /// HalfOpen'da izin verilen test isteği sayısı
    pub half_open_max_requests: u32,
    /// HalfOpen'da başarı eşiği (bu sayıya ulaşınca devre kapanır)
    pub half_open_success_threshold: u32,
    /// Zaman penceresi (saniye) - bu süre içindeki hatalar sayılır
    pub window_seconds: u64,
}

impl Default for CircuitBreakerConfig {
    fn default() -> Self {
        Self {
            failure_threshold: 5,
            recovery_timeout_ms: 30_000, // 30 saniye
            half_open_max_requests: 3,
            half_open_success_threshold: 2,
            window_seconds: 60,
        }
    }
}

// ═════════════════════════════════════════════════════════════════
//  DEVRE KESİCİ
// ═════════════════════════════════════════════════════════════════

pub struct CircuitBreaker {
    /// Yapılandırma
    config: CircuitBreakerConfig,
    /// Mevcut durum
    state: std::sync::Mutex<CircuitState>,
    /// Ardışık hata sayısı
    consecutive_failures: AtomicU32,
    /// Toplam başarı sayısı (HalfOpen'da kullanılır)
    half_open_successes: AtomicU32,
    /// HalfOpen'daki test isteği sayısı
    half_open_requests: AtomicU32,
    /// Son hata zamanı
    last_failure_time: std::sync::Mutex<Option<Instant>>,
    /// Son durum değişikliği zamanı
    last_state_change: std::sync::Mutex<Option<Instant>>,
    /// Toplam istek sayısı
    total_requests: AtomicU64,
    /// Toplam reddedilen istek
    total_rejected: AtomicU64,
    /// Toplam başarılı istek
    total_successes: AtomicU64,
    /// Toplam başarısız istek
    total_failures: AtomicU64,
    /// Provider adı
    provider_name: String,
}

impl CircuitBreaker {
    /// Yeni devre kesici oluştur
    pub fn new(provider_name: impl Into<String>, config: CircuitBreakerConfig) -> Self {
        Self {
            config,
            state: std::sync::Mutex::new(CircuitState::Closed),
            consecutive_failures: AtomicU32::new(0),
            half_open_successes: AtomicU32::new(0),
            half_open_requests: AtomicU32::new(0),
            last_failure_time: std::sync::Mutex::new(None),
            last_state_change: std::sync::Mutex::new(None),
            total_requests: AtomicU64::new(0),
            total_rejected: AtomicU64::new(0),
            total_successes: AtomicU64::new(0),
            total_failures: AtomicU64::new(0),
            provider_name: provider_name.into(),
        }
    }

    /// Varsayılan yapılandırmayla oluştur
    pub fn default_for(provider_name: impl Into<String>) -> Self {
        Self::new(provider_name, CircuitBreakerConfig::default())
    }

    /// İstek geçebilir mi?
    pub fn allow_request(&self) -> bool {
        self.total_requests.fetch_add(1, Ordering::Relaxed);

        let state = self.state.lock().unwrap();
        match *state {
            CircuitState::Closed => true,
            CircuitState::Open => {
                // Kurtarma süresi dolmuş mu?
                if let Some(last_change) = *self.last_state_change.lock().unwrap() {
                    if last_change.elapsed() >= Duration::from_millis(self.config.recovery_timeout_ms) {
                        // HalfOpen'a geç
                        drop(state);
                        self.transition_to_half_open();
                        return true;
                    }
                }
                self.total_rejected.fetch_add(1, Ordering::Relaxed);
                false
            }
            CircuitState::HalfOpen => {
                let requests = self.half_open_requests.load(Ordering::Relaxed);
                if requests < self.config.half_open_max_requests {
                    self.half_open_requests.fetch_add(1, Ordering::Relaxed);
                    true
                } else {
                    self.total_rejected.fetch_add(1, Ordering::Relaxed);
                    false
                }
            }
        }
    }

    /// Başarılı istek kaydet
    pub fn record_success(&self) {
        self.total_successes.fetch_add(1, Ordering::Relaxed);
        self.consecutive_failures.store(0, Ordering::Relaxed);

        let state = self.state.lock().unwrap();
        if *state == CircuitState::HalfOpen {
            drop(state);
            let successes = self.half_open_successes.fetch_add(1, Ordering::Relaxed) + 1;
            if successes >= self.config.half_open_success_threshold {
                self.transition_to_closed();
            }
        }
    }

    /// Başarısız istek kaydet
    pub fn record_failure(&self) {
        self.total_failures.fetch_add(1, Ordering::Relaxed);
        let failures = self.consecutive_failures.fetch_add(1, Ordering::Relaxed) + 1;

        *self.last_failure_time.lock().unwrap() = Some(Instant::now());

        let state = self.state.lock().unwrap();
        match *state {
            CircuitState::Closed => {
                if failures >= self.config.failure_threshold {
                    drop(state);
                    self.transition_to_open();
                }
            }
            CircuitState::HalfOpen => {
                drop(state);
                self.transition_to_open();
            }
            CircuitState::Open => {
                // Zaten açık, değişiklik yok
            }
        }
    }

    /// Mevcut durumu al
    pub fn state(&self) -> CircuitState {
        *self.state.lock().unwrap()
    }

    /// Durum geçişi: Closed
    fn transition_to_closed(&self) {
        let mut state = self.state.lock().unwrap();
        if *state != CircuitState::Closed {
            log::info!("🔄  CIRCUIT [{}]: {} → ✅ Closed", self.provider_name, state);
            *state = CircuitState::Closed;
            *self.last_state_change.lock().unwrap() = Some(Instant::now());
            self.consecutive_failures.store(0, Ordering::Relaxed);
            self.half_open_successes.store(0, Ordering::Relaxed);
            self.half_open_requests.store(0, Ordering::Relaxed);
        }
    }

    /// Durum geçişi: Open
    fn transition_to_open(&self) {
        let mut state = self.state.lock().unwrap();
        if *state != CircuitState::Open {
            log::warn!("🔄  CIRCUIT [{}]: {} → 🔴 Open (ardışık hata: {})", 
                self.provider_name, state,
                self.consecutive_failures.load(Ordering::Relaxed));
            *state = CircuitState::Open;
            *self.last_state_change.lock().unwrap() = Some(Instant::now());
        }
    }

    /// Durum geçişi: HalfOpen
    fn transition_to_half_open(&self) {
        let mut state = self.state.lock().unwrap();
        if *state != CircuitState::HalfOpen {
            log::info!("🔄  CIRCUIT [{}]: {} → 🟡 HalfOpen (test modu)", self.provider_name, state);
            *state = CircuitState::HalfOpen;
            *self.last_state_change.lock().unwrap() = Some(Instant::now());
            self.half_open_successes.store(0, Ordering::Relaxed);
            self.half_open_requests.store(0, Ordering::Relaxed);
        }
    }

    /// Devreyi sıfırla
    pub fn reset(&self) {
        self.transition_to_closed();
        log::info!("🔄  CIRCUIT [{}]: Manuel sıfırlama", self.provider_name);
    }

    /// İstatistik raporu
    pub fn stats(&self) -> CircuitBreakerStats {
        CircuitBreakerStats {
            provider_name: self.provider_name.clone(),
            state: self.state.lock().unwrap().clone(),
            consecutive_failures: self.consecutive_failures.load(Ordering::Relaxed),
            total_requests: self.total_requests.load(Ordering::Relaxed),
            total_successes: self.total_successes.load(Ordering::Relaxed),
            total_failures: self.total_failures.load(Ordering::Relaxed),
            total_rejected: self.total_rejected.load(Ordering::Relaxed),
        }
    }
}

// ═════════════════════════════════════════════════════════════════
//  İSTATİSTİK YAPILARI
// ═════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, serde::Serialize)]
pub struct CircuitBreakerStats {
    pub provider_name: String,
    pub state: CircuitState,
    pub consecutive_failures: u32,
    pub total_requests: u64,
    pub total_successes: u64,
    pub total_failures: u64,
    pub total_rejected: u64,
}

impl std::fmt::Display for CircuitBreakerStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let success_rate = if self.total_requests > 0 {
            (self.total_successes as f64 / self.total_requests as f64) * 100.0
        } else {
            0.0
        };
        write!(
            f,
            "CircuitBreaker [{}] Durum: {} | İstek: {} | Başarı: {} ({:.1}%) | Hata: {} | Reddedilen: {} | Ardışık Hata: {}",
            self.provider_name,
            self.state,
            self.total_requests,
            self.total_successes,
            success_rate,
            self.total_failures,
            self.total_rejected,
            self.consecutive_failures
        )
    }
}

// ═════════════════════════════════════════════════════════════════
//  ÇOKLU PROVIDER DEVRİ KESİCİ YÖNETİCİSİ
// ═════════════════════════════════════════════════════════════════

/// Birden fazla provider için devre kesici yöneticisi
pub struct CircuitBreakerManager {
    breakers: Vec<Arc<CircuitBreaker>>,
    config: CircuitBreakerConfig,
}

impl CircuitBreakerManager {
    pub fn new(config: CircuitBreakerConfig) -> Self {
        Self {
            breakers: Vec::new(),
            config,
        }
    }

    /// Yeni provider ekle
    pub fn add_provider(&mut self, name: impl Into<String>) -> Arc<CircuitBreaker> {
        let breaker = Arc::new(CircuitBreaker::new(name, self.config.clone()));
        self.breakers.push(breaker.clone());
        breaker
    }

    /// İstek kabul eden ilk provider'ı bul
    pub fn find_available(&self) -> Option<&Arc<CircuitBreaker>> {
        self.breakers.iter().find(|b| b.allow_request())
    }

    /// Failover: Birincil kapalıysa ikincili dene
    pub fn try_with_failover<F, R>(&self, primary_idx: usize, operation: F) -> Result<R, CircuitError>
    where
        F: Fn(&CircuitBreaker) -> Result<R, CircuitError>,
    {
        // Birincil dene
        if primary_idx < self.breakers.len() {
            let primary = &self.breakers[primary_idx];
            if primary.allow_request() {
                match operation(primary) {
                    Ok(r) => {
                        primary.record_success();
                        return Ok(r);
                    }
                    Err(e) => {
                        primary.record_failure();
                        log::warn!("CIRCUIT: Birincil başarısız, failover deneniyor: {}", e);
                    }
                }
            }
        }

        // İkincil provider'ları dene
        for (i, breaker) in self.breakers.iter().enumerate() {
            if i == primary_idx {
                continue;
            }
            if breaker.allow_request() {
                match operation(breaker) {
                    Ok(r) => {
                        breaker.record_success();
                        return Ok(r);
                    }
                    Err(e) => {
                        breaker.record_failure();
                        log::warn!("CIRCUIT: Failover provider {} başarısız: {}", i, e);
                    }
                }
            }
        }

        Err(CircuitError::AllCircuitsOpen)
    }

    /// Tüm provider istatistikleri
    pub fn all_stats(&self) -> Vec<CircuitBreakerStats> {
        self.breakers.iter().map(|b| b.stats()).collect()
    }
}

// ═════════════════════════════════════════════════════════════════
//  HATA TİPLERİ
// ═════════════════════════════════════════════════════════════════

#[derive(Debug)]
pub enum CircuitError {
    /// Tüm devreler açık
    AllCircuitsOpen,
    /// Provider hatası
    ProviderError(String),
}

impl std::fmt::Display for CircuitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CircuitError::AllCircuitsOpen => write!(f, "Tüm provider devreleri açık - istek reddedildi"),
            CircuitError::ProviderError(e) => write!(f, "Provider hatası: {}", e),
        }
    }
}

impl std::error::Error for CircuitError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circuit_closed_allows_requests() {
        let cb = CircuitBreaker::default_for("test_provider");
        assert_eq!(cb.state(), CircuitState::Closed);
        assert!(cb.allow_request());
    }

    #[test]
    fn test_circuit_opens_after_threshold() {
        let cb = CircuitBreaker::new("test", CircuitBreakerConfig {
            failure_threshold: 3,
            ..Default::default()
        });
        
        assert_eq!(cb.state(), CircuitState::Closed);
        
        cb.record_failure(); // 1
        cb.record_failure(); // 2
        assert_eq!(cb.state(), CircuitState::Closed);
        
        cb.record_failure(); // 3 - threshold reached
        assert_eq!(cb.state(), CircuitState::Open);
    }

    #[test]
    fn test_circuit_rejects_when_open() {
        let cb = CircuitBreaker::new("test", CircuitBreakerConfig {
            failure_threshold: 1,
            recovery_timeout_ms: 60000, // Long timeout
            ..Default::default()
        });
        
        cb.record_failure(); // Opens circuit
        assert_eq!(cb.state(), CircuitState::Open);
        assert!(!cb.allow_request());
    }

    #[test]
    fn test_stats() {
        let cb = CircuitBreaker::default_for("test");
        cb.allow_request();
        cb.record_success();
        let stats = cb.stats();
        assert_eq!(stats.total_requests, 1);
        assert_eq!(stats.total_successes, 1);
    }

    #[test]
    fn test_manager() {
        let mut manager = CircuitBreakerManager::new(CircuitBreakerConfig::default());
        manager.add_provider("openai");
        manager.add_provider("anthropic");
        manager.add_provider("groq");
        
        let stats = manager.all_stats();
        assert_eq!(stats.len(), 3);
    }
}
