//! ═════════════════════════════════════════════════════════════════
//!  LOAD BALANCING MODULE - Yük Dengeleme
//! ═════════════════════════════════════════════════════════════════
//!
//! Çoklu provider arasında yük dağılımı.
//! Round-robin, weighted, least-connections algoritmaları.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

/// Provider sağlığı
#[derive(Debug, Clone, serde::Serialize)]
pub struct ProviderHealth {
    pub name: String,
    pub active_requests: u64,
    pub total_requests: u64,
    pub total_errors: u64,
    pub avg_latency_ms: f64,
    pub weight: u32,
    pub available: bool,
}

/// Yük dengeleme algoritması
#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum LoadBalanceAlgorithm {
    RoundRobin,
    Weighted,
    LeastConnections,
    Random,
}

/// Yük dengeleyici
pub struct LoadBalancer {
    providers: Vec<ProviderHealth>,
    algorithm: LoadBalanceAlgorithm,
    counter: AtomicU64,
}

impl LoadBalancer {
    pub fn new(algorithm: LoadBalanceAlgorithm) -> Self {
        Self {
            providers: Vec::new(),
            algorithm,
            counter: AtomicU64::new(0),
        }
    }

    /// Provider ekle
    pub fn add_provider(&mut self, name: &str, weight: u32) {
        self.providers.push(ProviderHealth {
            name: name.into(),
            active_requests: 0,
            total_requests: 0,
            total_errors: 0,
            avg_latency_ms: 0.0,
            weight,
            available: true,
        });
    }

    /// Sonraki provider'ı seç
    pub fn next_provider(&mut self) -> Option<&ProviderHealth> {
        let available: Vec<&ProviderHealth> = self.providers.iter()
            .filter(|p| p.available)
            .collect();

        if available.is_empty() {
            return None;
        }

        match self.algorithm {
            LoadBalanceAlgorithm::RoundRobin => {
                let idx = self.counter.fetch_add(1, Ordering::Relaxed) as usize % available.len();
                Some(available[idx])
            }
            LoadBalanceAlgorithm::Weighted => {
                let total_weight: u32 = available.iter().map(|p| p.weight).sum();
                if total_weight == 0 { return Some(available[0]); }
                let mut rng = self.counter.fetch_add(1, Ordering::Relaxed) as u32 % total_weight;
                for provider in &available {
                    if rng < provider.weight {
                        return Some(provider);
                    }
                    rng -= provider.weight;
                }
                Some(available[0])
            }
            LoadBalanceAlgorithm::LeastConnections => {
                available.iter().min_by_key(|p| p.active_requests).copied()
            }
            LoadBalanceAlgorithm::Random => {
                let idx = self.counter.fetch_add(1, Ordering::Relaxed) as usize % available.len();
                Some(available[idx])
            }
        }
    }

    /// Provider'ı kullanılamaz olarak işaretle
    pub fn mark_unavailable(&mut self, name: &str) {
        if let Some(p) = self.providers.iter_mut().find(|p| p.name == name) {
            p.available = false;
        }
    }

    /// Provider'ı kullanılabilir olarak işaretle
    pub fn mark_available(&mut self, name: &str) {
        if let Some(p) = self.providers.iter_mut().find(|p| p.name == name) {
            p.available = true;
        }
    }

    /// İstek kaydet
    pub fn record_request(&mut self, name: &str, latency_ms: f64, is_error: bool) {
        if let Some(p) = self.providers.iter_mut().find(|p| p.name == name) {
            p.total_requests += 1;
            p.active_requests = p.active_requests.saturating_sub(1);
            if is_error { p.total_errors += 1; }
            // Moving average latency
            if p.avg_latency_ms == 0.0 {
                p.avg_latency_ms = latency_ms;
            } else {
                p.avg_latency_ms = p.avg_latency_ms * 0.8 + latency_ms * 0.2;
            }
        }
    }

    /// İstek başlat (active_requests artır)
    pub fn start_request(&mut self, name: &str) {
        if let Some(p) = self.providers.iter_mut().find(|p| p.name == name) {
            p.active_requests += 1;
        }
    }

    /// Tüm provider'ları listele
    pub fn list_providers(&self) -> &[ProviderHealth] {
        &self.providers
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_robin() {
        let mut lb = LoadBalancer::new(LoadBalanceAlgorithm::RoundRobin);
        lb.add_provider("openai", 1);
        lb.add_provider("anthropic", 1);
        lb.add_provider("groq", 1);

        let p1 = lb.next_provider().unwrap().name.clone();
        let p2 = lb.next_provider().unwrap().name.clone();
        let p3 = lb.next_provider().unwrap().name.clone();

        assert_ne!(p1, p2);
        assert_ne!(p2, p3);
    }

    #[test]
    fn test_least_connections() {
        let mut lb = LoadBalancer::new(LoadBalanceAlgorithm::LeastConnections);
        lb.add_provider("openai", 1);
        lb.add_provider("anthropic", 1);

        lb.start_request("openai");
        lb.start_request("openai");

        let selected = lb.next_provider().unwrap().name.clone();
        assert_eq!(selected, "anthropic"); // fewer connections
    }

    #[test]
    fn test_mark_unavailable() {
        let mut lb = LoadBalancer::new(LoadBalanceAlgorithm::RoundRobin);
        lb.add_provider("openai", 1);
        lb.add_provider("down", 1);
        lb.mark_unavailable("down");

        let selected = lb.next_provider().unwrap().name.clone();
        assert_eq!(selected, "openai");
    }
}
