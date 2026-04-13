//! ─── LLM Model Cache ───
//!
//! Intelligent caching system for LLM responses
//! - Semantic similarity caching
//! - TTL-based expiration
//! - Cost tracking
//! - Multi-tier storage (memory + disk)

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use parking_lot::RwLock;
use serde::{Deserialize, Serialize};

use crate::error::{LlmError, LlmResult};
use crate::types::{ChatRequest, ChatResponse};

// ═══════════════════════════════════════════════════════════════════════════════
//  CACHE CONFIGURATION
// ═══════════════════════════════════════════════════════════════════════════════

/// Cache configuration
#[derive(Debug, Clone)]
pub struct CacheConfig {
    /// Enable caching
    pub enabled: bool,
    /// Maximum entries in memory cache
    pub max_entries: usize,
    /// Time-to-live in seconds (default: 1 hour)
    pub ttl_secs: u64,
    /// Maximum cache size in bytes
    pub max_size_bytes: usize,
    /// Enable semantic similarity matching
    pub semantic_matching: bool,
    /// Similarity threshold (0.0 - 1.0)
    pub similarity_threshold: f32,
    /// Enable disk persistence
    pub persist_to_disk: bool,
    /// Disk cache path
    pub disk_path: Option<String>,
    /// Track cost savings
    pub track_costs: bool,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_entries: 10000,
            ttl_secs: 3600, // 1 hour
            max_size_bytes: 100 * 1024 * 1024, // 100MB
            semantic_matching: true,
            similarity_threshold: 0.95,
            persist_to_disk: false,
            disk_path: None,
            track_costs: true,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  CACHE ENTRY
// ═══════════════════════════════════════════════════════════════════════════════

/// A cached response entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry {
    /// Cache key (hash of request)
    pub key: String,
    /// Original request hash
    pub request_hash: u64,
    /// Model used
    pub model: String,
    /// Provider used
    pub provider: String,
    /// Cached response
    pub response: ChatResponse,
    /// Creation timestamp
    pub created_at: u64,
    /// Last access timestamp
    pub last_accessed: u64,
    /// Access count
    pub access_count: u64,
    /// Original cost (saved by cache)
    pub cost_saved: f64,
    /// TTL in seconds
    pub ttl_secs: u64,
    /// Entry size in bytes
    pub size_bytes: usize,
    /// Embedding for semantic matching (optional)
    pub embedding: Option<Vec<f32>>,
}

impl CacheEntry {
    /// Check if entry is expired
    pub fn is_expired(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        now > self.created_at + self.ttl_secs
    }

    /// Record an access
    pub fn record_access(&mut self) {
        self.last_accessed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        self.access_count += 1;
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  CACHE STATISTICS
// ═══════════════════════════════════════════════════════════════════════════════

/// Cache statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CacheStats {
    /// Total cache hits
    pub hits: u64,
    /// Total cache misses
    pub misses: u64,
    /// Total entries
    pub entries: usize,
    /// Total size in bytes
    pub total_size_bytes: usize,
    /// Total cost saved
    pub cost_saved: f64,
    /// Evictions due to size
    pub evictions: u64,
    /// Evictions due to TTL
    pub ttl_evictions: u64,
    /// Average access count
    pub avg_access_count: f64,
}

impl CacheStats {
    /// Calculate hit rate
    pub fn hit_rate(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            0.0
        } else {
            self.hits as f64 / total as f64
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  MODEL CACHE
// ═══════════════════════════════════════════════════════════════════════════════

/// LLM Response Cache
pub struct ModelCache {
    config: CacheConfig,
    entries: Arc<RwLock<HashMap<String, CacheEntry>>>,
    stats: Arc<RwLock<CacheStats>>,
    embeddings: Arc<RwLock<HashMap<String, Vec<f32>>>>,
}

impl ModelCache {
    /// Create new model cache
    pub fn new(config: CacheConfig) -> Self {
        Self {
            config,
            entries: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(RwLock::new(CacheStats::default())),
            embeddings: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Create with default config
    pub fn default_cache() -> Self {
        Self::new(CacheConfig::default())
    }

    /// Generate cache key from request
    pub fn generate_key(&self, request: &ChatRequest) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();

        // Hash model
        request.model.hash(&mut hasher);

        // Hash messages
        for msg in &request.messages {
            msg.role.hash(&mut hasher);
            if let Some(content) = msg.content.as_text() {
                content.hash(&mut hasher);
            }
        }

        // Hash temperature and other params
        if let Some(temp) = request.temperature {
            temp.to_bits().hash(&mut hasher);
        }
        if let Some(max_tokens) = request.max_tokens {
            max_tokens.hash(&mut hasher);
        }

        format!("{:016x}", hasher.finish())
    }

    /// Get cached response
    pub fn get(&self, request: &ChatRequest) -> Option<CacheEntry> {
        if !self.config.enabled {
            return None;
        }

        let key = self.generate_key(request);

        let mut entries = self.entries.write();

        if let Some(entry) = entries.get_mut(&key) {
            if entry.is_expired() {
                // Remove expired entry
                let size = entry.size_bytes;
                entries.remove(&key);

                let mut stats = self.stats.write();
                stats.ttl_evictions += 1;
                stats.total_size_bytes -= size;
                stats.misses += 1;

                return None;
            }

            // Record access
            entry.record_access();

            // Update stats
            let cost_saved = entry.cost_saved;
            let mut stats = self.stats.write();
            stats.hits += 1;
            stats.cost_saved += cost_saved;

            return Some(entry.clone());
        }

        // Try semantic matching if enabled
        if self.config.semantic_matching {
            drop(entries);
            return self.semantic_search(request);
        }

        self.stats.write().misses += 1;
        None
    }

    /// Store response in cache
    pub fn put(
        &self,
        request: &ChatRequest,
        response: ChatResponse,
        provider: &str,
        cost: f64,
    ) -> LlmResult<()> {
        if !self.config.enabled {
            return Ok(());
        }

        let key = self.generate_key(request);

        // Estimate size
        let size_bytes = self.estimate_size(&response);

        // Check capacity
        self.evict_if_needed(size_bytes);

        let entry = CacheEntry {
            key: key.clone(),
            request_hash: self.hash_request(request),
            model: request.model.clone(),
            provider: provider.to_string(),
            response,
            created_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            last_accessed: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            access_count: 0,
            cost_saved: cost,
            ttl_secs: self.config.ttl_secs,
            size_bytes,
            embedding: None, // Would be computed by embedding model
        };

        let mut entries = self.entries.write();
        let mut stats = self.stats.write();

        entries.insert(key, entry);
        stats.entries = entries.len();
        stats.total_size_bytes += size_bytes;

        Ok(())
    }

    /// Clear cache
    pub fn clear(&self) {
        let mut entries = self.entries.write();
        entries.clear();

        let mut stats = self.stats.write();
        stats.entries = 0;
        stats.total_size_bytes = 0;
    }

    /// Get cache statistics
    pub fn stats(&self) -> CacheStats {
        self.stats.read().clone()
    }

    /// Invalidate entries older than duration
    pub fn invalidate_older_than(&self, duration: Duration) -> u64 {
        let cutoff = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            - duration.as_secs();

        let mut entries = self.entries.write();
        let mut stats = self.stats.write();

        let before = entries.len();
        entries.retain(|_, e| e.created_at > cutoff);
        let removed = before - entries.len();

        stats.ttl_evictions += removed as u64;
        stats.entries = entries.len();

        removed as u64
    }

    /// Invalidate by model
    pub fn invalidate_by_model(&self, model: &str) -> u64 {
        let mut entries = self.entries.write();
        let mut stats = self.stats.write();

        let before = entries.len();
        entries.retain(|_, e| e.model != model);
        let removed = before - entries.len();

        stats.entries = entries.len();

        removed as u64
    }

    /// Invalidate by provider
    pub fn invalidate_by_provider(&self, provider: &str) -> u64 {
        let mut entries = self.entries.write();
        let mut stats = self.stats.write();

        let before = entries.len();
        entries.retain(|_, e| e.provider != provider);
        let removed = before - entries.len();

        stats.entries = entries.len();

        removed as u64
    }

    // ─────────────────────────────────────────────────────────────────────────
    //  PRIVATE METHODS
    // ─────────────────────────────────────────────────────────────────────────

    fn hash_request(&self, request: &ChatRequest) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        request.model.hash(&mut hasher);
        for msg in &request.messages {
            msg.role.hash(&mut hasher);
            if let Some(content) = msg.content.as_text() {
                content.hash(&mut hasher);
            }
        }
        hasher.finish()
    }

    fn estimate_size(&self, response: &ChatResponse) -> usize {
        // Rough estimate: content length + metadata
        let content_size: usize = response
            .choices
            .iter()
            .map(|c| {
                c.message
                    .content
                    .as_text()
                    .map(|t| t.len())
                    .unwrap_or(0)
            })
            .sum();

        content_size + 1024 // Base overhead
    }

    fn evict_if_needed(&self, needed_size: usize) {
        let mut entries = self.entries.write();
        let mut stats = self.stats.write();

        // Check max entries
        while entries.len() >= self.config.max_entries {
            self.evict_lru(&mut entries, &mut stats);
        }

        // Check max size
        while stats.total_size_bytes + needed_size > self.config.max_size_bytes {
            self.evict_lru(&mut entries, &mut stats);
        }
    }

    fn evict_lru(
        &self,
        entries: &mut HashMap<String, CacheEntry>,
        stats: &mut CacheStats,
    ) {
        // Find LRU entry
        let lru_key = entries
            .iter()
            .min_by_key(|(_, e)| e.last_accessed)
            .map(|(k, _)| k.clone());

        if let Some(key) = lru_key {
            if let Some(entry) = entries.remove(&key) {
                stats.total_size_bytes -= entry.size_bytes;
                stats.evictions += 1;
                stats.entries = entries.len();
            }
        }
    }

    fn semantic_search(&self, _request: &ChatRequest) -> Option<CacheEntry> {
        // Semantic similarity search would require embedding model
        // This is a placeholder for the feature
        None
    }

    /// Get top entries by access count
    pub fn top_entries(&self, limit: usize) -> Vec<CacheEntry> {
        let entries = self.entries.read();
        let mut entries: Vec<_> = entries.values().cloned().collect();
        entries.sort_by(|a, b| b.access_count.cmp(&a.access_count));
        entries.into_iter().take(limit).collect()
    }

    /// Get entries by cost saved
    pub fn top_cost_savers(&self, limit: usize) -> Vec<CacheEntry> {
        let entries = self.entries.read();
        let mut entries: Vec<_> = entries.values().cloned().collect();
        entries.sort_by(|a, b| {
            b.cost_saved
                .partial_cmp(&a.cost_saved)
                .unwrap()
        });
        entries.into_iter().take(limit).collect()
    }
}

impl Default for ModelCache {
    fn default() -> Self {
        Self::default_cache()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  CACHE KEY BUILDER
// ═══════════════════════════════════════════════════════════════════════════════

/// Builder for cache keys with custom parameters
pub struct CacheKeyBuilder {
    parts: Vec<String>,
}

impl CacheKeyBuilder {
    pub fn new() -> Self {
        Self { parts: Vec::new() }
    }

    pub fn model(mut self, model: &str) -> Self {
        self.parts.push(format!("model:{}", model));
        self
    }

    pub fn message(mut self, role: &str, content: &str) -> Self {
        // Truncate long content for key
        let truncated = if content.len() > 200 {
            &content[..200]
        } else {
            content
        };
        self.parts.push(format!("msg:{}:{}", role, truncated));
        self
    }

    pub fn temperature(mut self, temp: f32) -> Self {
        self.parts.push(format!("temp:{:.2}", temp));
        self
    }

    pub fn max_tokens(mut self, tokens: u32) -> Self {
        self.parts.push(format!("max:{}", tokens));
        self
    }

    pub fn custom(mut self, key: &str, value: &str) -> Self {
        self.parts.push(format!("{}:{}", key, value));
        self
    }

    pub fn build(self) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        for part in &self.parts {
            part.hash(&mut hasher);
        }
        format!("{:016x}", hasher.finish())
    }
}

impl Default for CacheKeyBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  MULTI-TIER CACHE
// ═══════════════════════════════════════════════════════════════════════════════

/// Multi-tier cache with memory and optional disk persistence
pub struct MultiTierCache {
    memory: ModelCache,
    disk_path: Option<String>,
}

impl MultiTierCache {
    pub fn new(config: CacheConfig) -> Self {
        let disk_path = config.disk_path.clone();
        let memory = ModelCache::new(config);

        Self { memory, disk_path }
    }

    pub fn get(&self, request: &ChatRequest) -> Option<CacheEntry> {
        // First check memory
        if let Some(entry) = self.memory.get(request) {
            return Some(entry);
        }

        // Then check disk if configured
        if let Some(_path) = &self.disk_path {
            // Disk lookup would be implemented here
            // For now, return None
        }

        None
    }

    pub fn put(
        &self,
        request: &ChatRequest,
        response: ChatResponse,
        provider: &str,
        cost: f64,
    ) -> LlmResult<()> {
        // Store in memory
        self.memory.put(request, response.clone(), provider, cost)?;

        // Store on disk if configured
        if let Some(_path) = &self.disk_path {
            // Disk persistence would be implemented here
        }

        Ok(())
    }

    pub fn stats(&self) -> CacheStats {
        self.memory.stats()
    }

    pub fn clear(&self) {
        self.memory.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_creation() {
        let cache = ModelCache::default_cache();
        assert!(cache.config.enabled);
    }

    #[test]
    fn test_cache_key_generation() {
        let cache = ModelCache::default_cache();
        let request = ChatRequest {
            model: "gpt-4o".into(),
            messages: vec![],
            ..Default::default()
        };

        let key = cache.generate_key(&request);
        assert!(!key.is_empty());
        assert_eq!(key.len(), 16);
    }

    #[test]
    fn test_cache_key_builder() {
        let key = CacheKeyBuilder::new()
            .model("gpt-4o")
            .message("user", "Hello")
            .temperature(0.7)
            .build();

        assert!(!key.is_empty());
    }

    #[test]
    fn test_cache_stats() {
        let stats = CacheStats {
            hits: 100,
            misses: 50,
            ..Default::default()
        };

        assert!((stats.hit_rate() - 0.666).abs() < 0.01);
    }

    #[test]
    fn test_cache_entry_expiration() {
        let entry = CacheEntry {
            key: "test".into(),
            request_hash: 0,
            model: "test".into(),
            provider: "test".into(),
            response: ChatResponse::default(),
            created_at: 0, // Very old
            last_accessed: 0,
            access_count: 0,
            cost_saved: 0.0,
            ttl_secs: 60,
            size_bytes: 100,
            embedding: None,
        };

        assert!(entry.is_expired());
    }
}
