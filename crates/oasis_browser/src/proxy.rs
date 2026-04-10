//! ═══════════════════════════════════════════════════════════════════════════════
//!  RESIDENTIAL PROXY ROTASYON SİSTEMİ
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Residential proxy havuzu ve akıllı rotasyon sistemi:
//! - Coğrafi konum bazlı rotasyon
//! - Sağlık kontrolü
//! - Başarı oranı takibi
//! - Otomatik blacklist yönetimi

use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::time::Duration;

/// Proxy türü
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProxyType {
    /// Residential proxy (gerçek ISP IP)
    Residential,
    /// Datacenter proxy
    Datacenter,
    /// Mobile proxy
    Mobile,
    /// Rotating proxy
    Rotating,
}

/// Proxy konumu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyLocation {
    /// Ülke kodu (ISO 3166-1 alpha-2)
    pub country: String,
    /// Şehir
    pub city: Option<String>,
    /// Bölge
    pub region: Option<String>,
    /// ISP adı
    pub isp: Option<String>,
}

/// Proxy sağlığı
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyHealth {
    /// Son kontrol zamanı
    pub last_check: Option<u64>,  // timestamp ms
    /// Yanıt süresi (ms)
    pub response_time_ms: Option<u64>,
    /// Başarı oranı (0.0 - 1.0)
    pub success_rate: f64,
    /// Toplam istek sayısı
    pub total_requests: u64,
    /// Başarılı istek sayısı
    pub successful_requests: u64,
    /// Son hata mesajı
    pub last_error: Option<String>,
    /// Aktif mi?
    pub is_active: bool,
}

impl Default for ProxyHealth {
    fn default() -> Self {
        Self {
            last_check: None,
            response_time_ms: None,
            success_rate: 1.0,
            total_requests: 0,
            successful_requests: 0,
            last_error: None,
            is_active: true,
        }
    }
}

/// Tek bir proxy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proxy {
    /// Proxy ID
    pub id: String,
    /// Proxy URL (socks5://user:pass@host:port)
    pub url: String,
    /// Host
    pub host: String,
    /// Port
    pub port: u16,
    /// Kullanıcı adı (opsiyonel)
    pub username: Option<String>,
    /// Şifre (opsiyonel)
    pub password: Option<String>,
    /// Proxy türü
    pub proxy_type: ProxyType,
    /// Konum
    pub location: ProxyLocation,
    /// Sağlık durumu
    #[serde(skip)]
    pub health: ProxyHealth,
    /// Ek meta veriler
    pub metadata: HashMap<String, String>,
}

impl Proxy {
    /// Proxy URL'sini oluştur
    pub fn to_url(&self) -> String {
        match (&self.username, &self.password) {
            (Some(user), Some(pass)) => {
                format!("{}://{}:{}@{}:{}", 
                    self.protocol(), user, pass, self.host, self.port)
            }
            _ => {
                format!("{}://{}:{}", self.protocol(), self.host, self.port)
            }
        }
    }
    
    /// Protokol tipi
    fn protocol(&self) -> &'static str {
        match self.proxy_type {
            ProxyType::Residential | ProxyType::Mobile => "http",
            ProxyType::Datacenter => "http",
            ProxyType::Rotating => "http",
        }
    }
}

/// Proxy havuzu
pub struct ProxyPool {
    /// Tüm proxy'ler
    proxies: Vec<Proxy>,
    /// Aktif proxy'ler (ID bazlı)
    active_proxies: Vec<String>,
    /// Blacklist (ID bazlı)
    blacklist: HashMap<String, String>, // ID -> Reason
    /// Son kullanılan proxy
    last_used: Option<String>,
    /// Rotasyon geçmişi
    rotation_history: VecDeque<String>,
    /// Yapılandırma
    config: ProxyConfig,
    /// Konum bazlı gruplar
    location_groups: HashMap<String, Vec<String>>,
}

/// Proxy yapılandırması
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyConfig {
    /// Rotasyon aktif mi?
    pub rotation_enabled: bool,
    /// Rotasyon modu
    pub rotation_mode: RotationMode,
    /// Minimum başarı oranı eşiği
    pub min_success_rate: f64,
    /// Maksimum yanıt süresi (ms)
    pub max_response_time_ms: u64,
    /// Sağlık kontrolü aralığı (saniye)
    pub health_check_interval_secs: u64,
    /// Başarısız deneme sonrası blacklist süresi (saniye)
    pub blacklist_duration_secs: u64,
    /// Aynı proxy tekrar kullanım bekleme süresi (saniye)
    pub reuse_delay_secs: u64,
    /// Otomatik blacklist aktif mi?
    pub auto_blacklist: bool,
    /// Tercih edilen ülkeler
    pub preferred_countries: Vec<String>,
}

impl Default for ProxyConfig {
    fn default() -> Self {
        Self {
            rotation_enabled: true,
            rotation_mode: RotationMode::RoundRobin,
            min_success_rate: 0.8,
            max_response_time_ms: 10000,
            health_check_interval_secs: 300,
            blacklist_duration_secs: 3600,
            reuse_delay_secs: 60,
            auto_blacklist: true,
            preferred_countries: vec!["US".into(), "DE".into(), "GB".into()],
        }
    }
}

/// Rotasyon modu
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum RotationMode {
    /// Sırayla
    RoundRobin,
    /// Rastgele
    Random,
    /// En az kullanılan
    LeastUsed,
    /// En hızlı
    Fastest,
    /// Konum bazlı
    LocationBased,
    /// Ağırlıklı rastgele
    WeightedRandom,
}

impl ProxyPool {
    /// Yeni proxy havuzu oluştur
    pub fn new(config: ProxyConfig) -> Self {
        log::info!("🌐 PROXY-POOL: Residential proxy havuzu başlatılıyor...");
        
        Self {
            proxies: Vec::new(),
            active_proxies: Vec::new(),
            blacklist: HashMap::new(),
            last_used: None,
            rotation_history: VecDeque::with_capacity(100),
            config,
            location_groups: HashMap::new(),
        }
    }
    
    /// Proxy ekle
    pub fn add_proxy(&mut self, mut proxy: Proxy) {
        proxy.health = ProxyHealth::default();
        
        let id = proxy.id.clone();
        let country = proxy.location.country.clone();
        
        self.proxies.push(proxy);
        self.active_proxies.push(id.clone());
        
        // Konum grubuna ekle
        self.location_groups
            .entry(country)
            .or_insert_with(Vec::new)
            .push(id.clone());
        
        log::debug!("🌐 PROXY-POOL: Proxy eklendi -> {} ({})", 
            id, self.proxies.last().expect("operation failed").location.country);
    }
    
    /// Toplu proxy ekle
    pub fn add_proxies(&mut self, proxies: Vec<Proxy>) {
        for proxy in proxies {
            self.add_proxy(proxy);
        }
    }
    
    /// Sonraki proxy'yi seç
    pub fn next(&mut self) -> Option<Proxy> {
        if !self.config.rotation_enabled {
            return self.get_first_active();
        }
        
        let selected_id = match self.config.rotation_mode {
            RotationMode::RoundRobin => self.select_round_robin(),
            RotationMode::Random => self.select_random(),
            RotationMode::LeastUsed => self.select_least_used(),
            RotationMode::Fastest => self.select_fastest(),
            RotationMode::LocationBased => self.select_location_based(),
            RotationMode::WeightedRandom => self.select_weighted_random(),
        };
        
        if let Some(id) = selected_id {
            self.last_used = Some(id.clone());
            self.rotation_history.push_back(id.clone());
            
            if self.rotation_history.len() > 100 {
                self.rotation_history.pop_front();
            }
            
            self.proxies.iter()
                .find(|p| p.id == id)
                .cloned()
        } else {
            None
        }
    }
    
    /// Round-robin seçim
    fn select_round_robin(&self) -> Option<String> {
        if self.active_proxies.is_empty() {
            return None;
        }
        
        let start_idx = if let Some(ref last) = self.last_used {
            self.active_proxies.iter()
                .position(|id| id == last)
                .map(|i| (i + 1) % self.active_proxies.len())
                .unwrap_or(0)
        } else {
            0
        };
        
        // Blacklist'te olmayan ilk proxy'yi bul
        for i in 0..self.active_proxies.len() {
            let idx = (start_idx + i) % self.active_proxies.len();
            let id = &self.active_proxies[idx];
            
            if !self.blacklist.contains_key(id) {
                return Some(id.clone());
            }
        }
        
        None
    }
    
    /// Rastgele seçim
    fn select_random(&self) -> Option<String> {
        let available: Vec<_> = self.active_proxies.iter()
            .filter(|id| !self.blacklist.contains_key(*id))
            .collect();
        
        if available.is_empty() {
            return None;
        }
        
        let mut rng = rand::thread_rng();
        Some(available[rng.gen_range(0..available.len())].clone())
    }
    
    /// En az kullanılan seçim
    fn select_least_used(&self) -> Option<String> {
        self.proxies.iter()
            .filter(|p| self.active_proxies.contains(&p.id) && !self.blacklist.contains_key(&p.id))
            .min_by_key(|p| p.health.total_requests)
            .map(|p| p.id.clone())
    }
    
    /// En hızlı seçim
    fn select_fastest(&self) -> Option<String> {
        self.proxies.iter()
            .filter(|p| {
                self.active_proxies.contains(&p.id) 
                && !self.blacklist.contains_key(&p.id)
                && p.health.is_active
            })
            .filter(|p| p.health.response_time_ms.is_some())
            .min_by_key(|p| p.health.response_time_ms.unwrap_or(u64::MAX))
            .map(|p| p.id.clone())
    }
    
    /// Konum bazlı seçim
    fn select_location_based(&self) -> Option<String> {
        if self.config.preferred_countries.is_empty() {
            return self.select_round_robin();
        }
        
        for country in &self.config.preferred_countries {
            if let Some(ids) = self.location_groups.get(country) {
                for id in ids {
                    if self.active_proxies.contains(id) && !self.blacklist.contains_key(id) {
                        return Some(id.clone());
                    }
                }
            }
        }
        
        self.select_round_robin()
    }
    
    /// Ağırlıklı rastgele seçim
    fn select_weighted_random(&self) -> Option<String> {
        let available: Vec<_> = self.proxies.iter()
            .filter(|p| self.active_proxies.contains(&p.id) && !self.blacklist.contains_key(&p.id))
            .collect();
        
        if available.is_empty() {
            return None;
        }
        
        // Ağırlık = başarı oranı * hız faktörü
        let weights: Vec<f64> = available.iter()
            .map(|p| {
                let success_weight = p.health.success_rate;
                let speed_weight = p.health.response_time_ms
                    .map(|t| 1.0 / (t as f64 / 1000.0 + 1.0))
                    .unwrap_or(0.5);
                
                success_weight * 0.7 + speed_weight * 0.3
            })
            .collect();
        
        // Ağırlıklı rastgele seçim
        let total_weight: f64 = weights.iter().sum();
        let mut rng = rand::thread_rng();
        let mut random = rng.gen::<f64>() * total_weight;
        
        for (proxy, weight) in available.iter().zip(weights.iter()) {
            random -= weight;
            if random <= 0.0 {
                return Some(proxy.id.clone());
            }
        }
        
        available.last().map(|p| p.id.clone())
    }
    
    /// İlk aktif proxy'yi getir
    fn get_first_active(&self) -> Option<Proxy> {
        self.proxies.iter()
            .find(|p| self.active_proxies.contains(&p.id) && !self.blacklist.contains_key(&p.id))
            .cloned()
    }
    
    /// Proxy kullanım sonucunu kaydet
    pub fn record_result(&mut self, proxy_id: &str, success: bool, response_time_ms: Option<u64>, error: Option<String>) {
        if let Some(proxy) = self.proxies.iter_mut().find(|p| p.id == proxy_id) {
            proxy.health.total_requests += 1;
            if success {
                proxy.health.successful_requests += 1;
            }
            
            // Başarı oranını güncelle
            proxy.health.success_rate = proxy.health.successful_requests as f64 
                / proxy.health.total_requests as f64;
            
            // Yanıt süresini güncelle (exponential moving average)
            if let Some(rt) = response_time_ms {
                proxy.health.response_time_ms = Some(
                    proxy.health.response_time_ms.map(|prev: u64| (prev as f64 * 0.7 + rt as f64 * 0.3) as u64).unwrap_or(rt)
                );
            }
            
            // Hata kaydı
            if let Some(err) = error {
                proxy.health.last_error = Some(err);
            }
            
            let now_ts = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_millis() as u64)
                .unwrap_or(0);
            proxy.health.last_check = Some(now_ts);
            
            // Otomatik blacklist kontrolü
            if self.config.auto_blacklist && proxy.health.success_rate < self.config.min_success_rate {
                self.blacklist_proxy(proxy_id, "Düşük başarı oranı");
            }
        }
    }
    
    /// Proxy'yi blacklist'e ekle
    pub fn blacklist_proxy(&mut self, proxy_id: &str, reason: &str) {
        self.blacklist.insert(proxy_id.to_string(), reason.to_string());
        self.active_proxies.retain(|id| id != proxy_id);
        
        log::warn!("🌐 PROXY-POOL: Proxy blacklist'e eklendi -> {} ({})", proxy_id, reason);
    }
    
    /// Proxy'yi blacklist'ten çıkar
    pub fn unblacklist_proxy(&mut self, proxy_id: &str) {
        if self.blacklist.remove(proxy_id).is_some() {
            self.active_proxies.push(proxy_id.to_string());
            log::info!("🌐 PROXY-POOL: Proxy blacklist'ten çıkarıldı -> {}", proxy_id);
        }
    }
    
    /// Sağlık kontrolü yap
    pub async fn health_check(&mut self) {
        log::info!("🌐 PROXY-POOL: Sağlık kontrolü başlatılıyor...");
        
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis() as u64)
            .unwrap_or(0);
        let mut to_remove = Vec::new();
        
        // URL'leri ve ID'leri önce topla
        let check_list: Vec<(String, String)> = self.proxies.iter()
            .filter(|p| {
                p.health.last_check.map(|lc| {
                    now.saturating_sub(lc) > self.config.health_check_interval_secs * 1000
                }).unwrap_or(true)
            })
            .map(|p| (p.id.clone(), p.url.clone()))
            .collect();
        
        // Her birini kontrol et
        for (id, url) in check_list {
            let healthy = self.check_proxy_health(&url).await;
            
            if let Some(proxy) = self.proxies.iter_mut().find(|p| p.id == id) {
                if !healthy {
                    proxy.health.is_active = false;
                    if self.config.auto_blacklist {
                        to_remove.push(proxy.id.clone());
                    }
                } else {
                    proxy.health.is_active = true;
                }
                proxy.health.last_check = Some(now);
            }
        }
        
        // Başarısız proxy'leri blacklist'e ekle
        for id in to_remove {
            self.blacklist_proxy(&id, "Sağlık kontrolü başarısız");
        }
        
        log::info!("🌐 PROXY-POOL: Sağlık kontrolü tamamlandı (aktif: {})", self.active_proxies.len());
    }
    
    /// Proxy sağlık kontrolü
    async fn check_proxy_health(&self, _proxy_url: &str) -> bool {
        // Basitleştirilmiş sağlık kontrolü
        // Gerçek implementasyonda HTTP isteği yapılır
        tokio::time::sleep(Duration::from_millis(100)).await;
        true
    }
    
    /// İstatistikleri getir
    pub fn stats(&self) -> ProxyPoolStats {
        let total = self.proxies.len();
        let active = self.active_proxies.len();
        let blacklisted = self.blacklist.len();
        
        let avg_success_rate = if total > 0 {
            self.proxies.iter()
                .map(|p| p.health.success_rate)
                .sum::<f64>() / total as f64
        } else {
            0.0
        };
        
        let avg_response_time = self.proxies.iter()
            .filter_map(|p| p.health.response_time_ms)
            .sum::<u64>() as f64 / total.max(1) as f64;
        
        ProxyPoolStats {
            total_proxies: total,
            active_proxies: active,
            blacklisted_proxies: blacklisted,
            avg_success_rate,
            avg_response_time_ms: avg_response_time,
        }
    }
    
    /// Proxy sayısı
    pub fn len(&self) -> usize {
        self.proxies.len()
    }
    
    /// Boş mu?
    pub fn is_empty(&self) -> bool {
        self.proxies.is_empty()
    }
}

/// Proxy havuzu istatistikleri
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyPoolStats {
    pub total_proxies: usize,
    pub active_proxies: usize,
    pub blacklisted_proxies: usize,
    pub avg_success_rate: f64,
    pub avg_response_time_ms: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn create_test_proxy(id: &str, country: &str) -> Proxy {
        Proxy {
            id: id.to_string(),
            url: format!("http://proxy{}.example.com:8080", id),
            host: format!("proxy{}.example.com", id),
            port: 8080,
            username: None,
            password: None,
            proxy_type: ProxyType::Residential,
            location: ProxyLocation {
                country: country.to_string(),
                city: None,
                region: None,
                isp: None,
            },
            health: ProxyHealth::default(),
            metadata: HashMap::new(),
        }
    }
    
    #[test]
    fn test_proxy_config_default() {
        let config = ProxyConfig::default();
        assert!(config.rotation_enabled);
        assert!(config.auto_blacklist);
    }
    
    #[test]
    fn test_proxy_pool_creation() {
        let pool = ProxyPool::new(ProxyConfig::default());
        assert!(pool.is_empty());
    }
    
    #[test]
    fn test_add_proxy() {
        let mut pool = ProxyPool::new(ProxyConfig::default());
        pool.add_proxy(create_test_proxy("1", "US"));
        
        assert_eq!(pool.len(), 1);
    }
    
    #[test]
    fn test_next_proxy() {
        let mut pool = ProxyPool::new(ProxyConfig::default());
        pool.add_proxy(create_test_proxy("1", "US"));
        pool.add_proxy(create_test_proxy("2", "DE"));
        
        let proxy = pool.next();
        assert!(proxy.is_some());
    }
    
    #[test]
    fn test_round_robin_rotation() {
        let mut config = ProxyConfig::default();
        config.rotation_mode = RotationMode::RoundRobin;
        
        let mut pool = ProxyPool::new(config);
        pool.add_proxy(create_test_proxy("1", "US"));
        pool.add_proxy(create_test_proxy("2", "DE"));
        
        let first = pool.next().expect("operation failed");
        let second = pool.next().expect("operation failed");
        
        assert_ne!(first.id, second.id);
    }
    
    #[test]
    fn test_record_result() {
        let mut pool = ProxyPool::new(ProxyConfig::default());
        pool.add_proxy(create_test_proxy("1", "US"));
        
        pool.record_result("1", true, Some(500), None);
        pool.record_result("1", false, Some(1000), Some("Timeout".into()));
        
        let proxy = pool.proxies.iter().find(|p| p.id == "1").expect("operation failed");
        assert_eq!(proxy.health.total_requests, 2);
        assert!((proxy.health.success_rate - 0.5).abs() < 0.01);
    }
    
    #[test]
    fn test_blacklist() {
        let mut pool = ProxyPool::new(ProxyConfig::default());
        pool.add_proxy(create_test_proxy("1", "US"));
        
        pool.blacklist_proxy("1", "Test blacklist");
        
        assert!(pool.blacklist.contains_key("1"));
        assert!(pool.next().is_none());
    }
    
    #[test]
    fn test_stats() {
        let mut pool = ProxyPool::new(ProxyConfig::default());
        pool.add_proxy(create_test_proxy("1", "US"));
        pool.add_proxy(create_test_proxy("2", "DE"));
        
        let stats = pool.stats();
        assert_eq!(stats.total_proxies, 2);
        assert_eq!(stats.active_proxies, 2);
    }
}
