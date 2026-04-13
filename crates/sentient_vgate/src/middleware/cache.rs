//! ═════════════════════════════════════════════════════════════════
//!  RESPONSE CACHE MODULE - LLM Yanıt Önbelleği
//! ═════════════════════════════════════════════════════════════════
//!
//! Aynı sorguların tekrar provider'a gitmesini önler.
//! TTL bazlı önbellek, maliyet tasarrufu sağlar.

use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};
use std::hash::{Hash, Hasher};

/// Önbellek girdisi
#[derive(Debug, Clone)]
struct CacheEntry {
    response: String,
    model: String,
    created_at: Instant,
    ttl: Duration,
    hit_count: u64,
}

/// Önbellek anahtarı
#[derive(Debug, Clone)]
pub struct CacheKey {
    pub model: String,
    pub messages_hash: u64,
}

impl CacheKey {
    pub fn new(model: &str, messages: &str) -> Self {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        messages.hash(&mut hasher);
        Self {
            model: model.into(),
            messages_hash: hasher.finish(),
        }
    }
}

/// Önbellek yapılandırması
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CacheConfig {
    /// Önbellek etkin mi?
    pub enabled: bool,
    /// Maksimum girdi sayısı
    pub max_entries: usize,
    /// Varsayılan TTL (saniye)
    pub default_ttl_secs: u64,
    /// Maksimum önbellek boyutu (bayt)
    pub max_size_bytes: usize,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_entries: 1000,
            default_ttl_secs: 300, // 5 dakika
            max_size_bytes: 50 * 1024 * 1024, // 50MB
        }
    }
}

/// LLM Yanıt Önbelleği
pub struct ResponseCache {
    entries: Mutex<HashMap<u64, CacheEntry>>,
    config: CacheConfig,
    stats: Mutex<CacheStats>,
}

/// Önbellek istatistikleri
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct CacheStats {
    pub hits: u64,
    pub misses: u64,
    pub evictions: u64,
    pub total_entries: usize,
    pub estimated_size_bytes: usize,
}

impl ResponseCache {
    pub fn new(config: CacheConfig) -> Self {
        Self {
            entries: Mutex::new(HashMap::new()),
            config,
            stats: Mutex::new(CacheStats::default()),
        }
    }

    pub fn default_cache() -> Self {
        Self::new(CacheConfig::default())
    }

    /// Önbellekte ara
    pub fn get(&self, key: &CacheKey) -> Option<String> {
        if !self.config.enabled {
            return None;
        }

        let mut entries = self.entries.lock().unwrap();
        let cache_key = Self::make_key(key);

        if let Some(entry) = entries.get_mut(&cache_key) {
            if entry.created_at.elapsed() > entry.ttl {
                entries.remove(&cache_key);
                let mut stats = self.stats.lock().unwrap();
                stats.misses += 1;
                return None;
            }
            entry.hit_count += 1;
            let mut stats = self.stats.lock().unwrap();
            stats.hits += 1;
            Some(entry.response.clone())
        } else {
            let mut stats = self.stats.lock().unwrap();
            stats.misses += 1;
            None
        }
    }

    /// Önbelleğe yaz
    pub fn put(&self, key: &CacheKey, response: &str, model: &str) {
        if !self.config.enabled {
            return;
        }

        let mut entries = self.entries.lock().unwrap();
        let cache_key = Self::make_key(key);
        let mut stats = self.stats.lock().unwrap();

        // Kapasite kontrolü
        while entries.len() >= self.config.max_entries {
            if let Some(oldest_key) = entries.iter()
                .min_by_key(|(_, v)| v.created_at)
                .map(|(k, _)| *k)
            {
                entries.remove(&oldest_key);
                stats.evictions += 1;
            }
        }

        entries.insert(cache_key, CacheEntry {
            response: response.into(),
            model: model.into(),
            created_at: Instant::now(),
            ttl: Duration::from_secs(self.config.default_ttl_secs),
            hit_count: 0,
        });

        stats.total_entries = entries.len();
        stats.estimated_size_bytes += response.len();
    }

    /// Önbelleği temizle
    pub fn clear(&self) {
        self.entries.lock().unwrap().clear();
        let mut stats = self.stats.lock().unwrap();
        *stats = CacheStats::default();
    }

    /// Süresi dolmuş girdileri temizle
    pub fn cleanup_expired(&self) -> usize {
        let mut entries = self.entries.lock().unwrap();
        let now = Instant::now();
        let before = entries.len();
        entries.retain(|_, entry| now.duration_since(entry.created_at) < entry.ttl);
        let removed = before - entries.len();
        let mut stats = self.stats.lock().unwrap();
        stats.total_entries = entries.len();
        stats.evictions += removed as u64;
        removed
    }

    /// İstatistikler
    pub fn stats(&self) -> CacheStats {
        let entries = self.entries.lock().unwrap();
        let mut stats = self.stats.lock().unwrap().clone();
        stats.total_entries = entries.len();
        stats.estimated_size_bytes = entries.values().map(|e| e.response.len()).sum();
        stats
    }

    fn make_key(key: &CacheKey) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        key.model.hash(&mut hasher);
        key.messages_hash.hash(&mut hasher);
        hasher.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_put_get() {
        let cache = ResponseCache::default_cache();
        let key = CacheKey::new("gpt-4", "Merhaba");
        cache.put(&key, "Merhaba! Size nasıl yardımcı olabilirim?", "gpt-4");
        let result = cache.get(&key);
        assert!(result.is_some());
    }

    #[test]
    fn test_cache_miss() {
        let cache = ResponseCache::default_cache();
        let key = CacheKey::new("gpt-4", "test");
        assert!(cache.get(&key).is_none());
    }

    #[test]
    fn test_cache_disabled() {
        let config = CacheConfig { enabled: false, ..Default::default() };
        let cache = ResponseCache::new(config);
        let key = CacheKey::new("gpt-4", "test");
        cache.put(&key, "response", "gpt-4");
        assert!(cache.get(&key).is_none());
    }

    #[test]
    fn test_cache_stats() {
        let cache = ResponseCache::default_cache();
        let key = CacheKey::new("gpt-4", "hello");
        cache.put(&key, "hi", "gpt-4");
        cache.get(&key); // hit
        cache.get(&CacheKey::new("gpt-4", "missing")); // miss
        let stats = cache.stats();
        assert_eq!(stats.hits, 1);
        assert_eq!(stats.misses, 1);
        assert_eq!(stats.total_entries, 1);
    }
}
