//! ─── Embedding Cache ───

use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

use parking_lot::RwLock;
use lru::LruCache;
use serde::{Deserialize, Serialize};

use crate::Embedding;

// ═══════════════════════════════════════════════════════════════════════════════
//  CACHE CONFIGURATION
// ═══════════════════════════════════════════════════════════════════════════════

/// Cache configuration
#[derive(Debug, Clone)]
pub struct CacheConfig {
    /// Enable caching
    pub enabled: bool,
    /// Maximum entries
    pub max_entries: usize,
    /// Time-to-live in seconds
    pub ttl_secs: u64,
    /// Enable disk persistence
    pub persist: bool,
    /// Persistence path
    pub persist_path: Option<String>,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_entries: 10000,
            ttl_secs: 86400, // 24 hours
            persist: false,
            persist_path: None,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  CACHE ENTRY
// ═══════════════════════════════════════════════════════════════════════════════

/// Cache entry with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry {
    /// The cached embedding
    pub embedding: Embedding,
    /// Creation timestamp
    pub created_at: u64,
    /// Access count
    pub access_count: u64,
}

impl CacheEntry {
    /// Check if expired
    pub fn is_expired(&self, ttl_secs: u64) -> bool {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        now > self.created_at + ttl_secs
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  EMBEDDING CACHE
// ═══════════════════════════════════════════════════════════════════════════════

/// LRU cache for embeddings
pub struct EmbeddingCache {
    config: CacheConfig,
    cache: RwLock<LruCache<String, CacheEntry>>,
    stats: RwLock<CacheStats>,
}

/// Cache statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CacheStats {
    pub hits: u64,
    pub misses: u64,
    pub evictions: u64,
    pub entries: usize,
}

impl CacheStats {
    pub fn hit_rate(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 { 0.0 } else { self.hits as f64 / total as f64 }
    }
}

impl EmbeddingCache {
    /// Create new cache
    pub fn new(config: CacheConfig) -> Self {
        let cache = LruCache::new(std::num::NonZeroUsize::new(config.max_entries).unwrap());
        
        Self {
            config,
            cache: RwLock::new(cache),
            stats: RwLock::new(CacheStats::default()),
        }
    }

    /// Get cached embedding
    pub fn get(&self, text: &str, model: &str) -> Option<Embedding> {
        if !self.config.enabled {
            return None;
        }

        let key = Self::make_key(text, model);
        let mut cache = self.cache.write();

        if let Some(entry) = cache.get_mut(&key) {
            if entry.is_expired(self.config.ttl_secs) {
                cache.pop(&key);
                self.stats.write().misses += 1;
                return None;
            }

            entry.access_count += 1;
            self.stats.write().hits += 1;
            return Some(entry.embedding.clone());
        }

        self.stats.write().misses += 1;
        None
    }

    /// Put embedding in cache
    pub fn put(&self, text: &str, model: &str, embedding: Embedding) {
        if !self.config.enabled {
            return;
        }

        let key = Self::make_key(text, model);
        let entry = CacheEntry {
            embedding,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            access_count: 0,
        };

        let mut cache = self.cache.write();
        if cache.put(key, entry).is_some() {
            self.stats.write().evictions += 1;
        }
        
        self.stats.write().entries = cache.len();
    }

    /// Clear cache
    pub fn clear(&self) {
        let mut cache = self.cache.write();
        cache.clear();
        self.stats.write().entries = 0;
    }

    /// Get stats
    pub fn stats(&self) -> CacheStats {
        self.stats.read().clone()
    }

    /// Make cache key
    fn make_key(text: &str, model: &str) -> String {
        let mut hasher = DefaultHasher::new();
        text.hash(&mut hasher);
        model.hash(&mut hasher);
        format!("{:016x}-{}", hasher.finish(), model)
    }

    /// Invalidate by model
    pub fn invalidate_model(&self, model: &str) -> u64 {
        let mut cache = self.cache.write();
        let before = cache.len();
        
        // LruCache doesn't have retain, so we rebuild
        let entries: Vec<_> = cache.iter()
            .filter(|(k, _)| !k.ends_with(model))
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        
        cache.clear();
        for (k, v) in entries {
            cache.put(k, v);
        }
        
        (before - cache.len()) as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_put_get() {
        let cache = EmbeddingCache::new(CacheConfig::default());
        let emb = Embedding {
            vector: vec![1.0, 2.0, 3.0],
            model: "test".into(),
            tokens: 1,
            index: 0,
            text: None,
        };
        
        cache.put("hello", "test", emb.clone());
        let result = cache.get("hello", "test");
        
        assert!(result.is_some());
        assert_eq!(result.unwrap().vector, vec![1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_cache_miss() {
        let cache = EmbeddingCache::new(CacheConfig::default());
        let result = cache.get("nonexistent", "test");
        assert!(result.is_none());
    }

    #[test]
    fn test_cache_stats() {
        let cache = EmbeddingCache::new(CacheConfig::default());
        let emb = Embedding {
            vector: vec![1.0],
            model: "test".into(),
            tokens: 1,
            index: 0,
            text: None,
        };
        
        cache.put("hello", "test", emb.clone());
        cache.get("hello", "test");
        cache.get("miss", "test");
        
        let stats = cache.stats();
        assert_eq!(stats.hits, 1);
        assert_eq!(stats.misses, 1);
    }
}
