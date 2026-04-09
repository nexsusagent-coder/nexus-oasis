//! Memory adapter - SENTIENT Memory entegrasyonu
//! 
//! Bu modül, Cevahir AI'ın bellek sisteminiSENTIENT OS'in
//! bellek katmanı ile entegre eder.

use crate::error::{CevahirError, Result};
use crate::types::MemoryEntry;
use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;
use chrono::Utc;

/// Memory adapter -SENTIENT Memory ile köprü
pub struct MemoryAdapter {
    /// In-memory cache (geçici)
    cache: Arc<RwLock<HashMap<String, MemoryEntry>>>,
    
    /// Vector store referansı (opsiyonel)
    vector_store: Option<Arc<dyn VectorStore>>,
    
    /// Maksimum boyut
    max_size: usize,
}

/// Vector store trait (SENTIENT vector store için)
pub trait VectorStore: Send + Sync {
    /// Vektör ekle
    fn insert(&self, key: &str, value: &str, embedding: Option<Vec<f32>>) -> Result<()>;
    
    /// Benzerlik ara
    fn search(&self, query: &str, limit: usize) -> Result<Vec<String>>;
    
    /// Sil
    fn delete(&self, key: &str) -> Result<()>;
}

impl MemoryAdapter {
    /// Yeni memory adapter oluştur
    pub fn new() -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            vector_store: None,
            max_size: 10000,
        }
    }
    
    /// Vector store ile oluştur
    pub fn with_vector_store(vector_store: Arc<dyn VectorStore>) -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            vector_store: Some(vector_store),
            max_size: 10000,
        }
    }
    
    /// Boyut limiti ile oluştur
    pub fn with_max_size(max_size: usize) -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            vector_store: None,
            max_size,
        }
    }
    
    /// Belleğe kaydet
    pub async fn store(&self, key: &str, value: &str) -> Result<()> {
        let entry = MemoryEntry {
            key: key.to_string(),
            value: value.to_string(),
            embedding: None,  // TODO: Embedding üret
            timestamp: Utc::now().timestamp(),
        };
        
        // In-memory cache'e kaydet
        {
            let mut cache = self.cache.write();
            
            // Boyut kontrolü
            if cache.len() >= self.max_size {
                // En eski girdiyi sil (LRU)
                if let Some(oldest_key) = cache.iter()
                    .min_by_key(|(_, e)| e.timestamp)
                    .map(|(k, _)| k.clone())
                {
                    cache.remove(&oldest_key);
                }
            }
            
            cache.insert(key.to_string(), entry);
        }
        
        // Vector store'a kaydet (varsa)
        if let Some(vs) = &self.vector_store {
            vs.insert(key, value, None)?;
        }
        
        Ok(())
    }
    
    /// Bellekten ara
    pub async fn search(&self, query: &str, limit: usize) -> Result<Vec<String>> {
        // Önce vector store'da ara (varsa)
        if let Some(vs) = &self.vector_store {
            return vs.search(query, limit);
        }
        
        // Değilse in-memory cache'de basit arama
        let cache = self.cache.read();
        let results: Vec<String> = cache.values()
            .filter(|entry| {
                entry.key.contains(query) || entry.value.contains(query)
            })
            .take(limit)
            .map(|entry| entry.value.clone())
            .collect();
        
        Ok(results)
    }
    
    /// Anahtar ile al
    pub async fn get(&self, key: &str) -> Option<MemoryEntry> {
        let cache = self.cache.read();
        cache.get(key).cloned()
    }
    
    /// Sil
    pub async fn delete(&self, key: &str) -> Result<()> {
        {
            let mut cache = self.cache.write();
            cache.remove(key);
        }
        
        if let Some(vs) = &self.vector_store {
            vs.delete(key)?;
        }
        
        Ok(())
    }
    
    /// Tümünü temizle
    pub async fn clear(&self) -> Result<()> {
        {
            let mut cache = self.cache.write();
            cache.clear();
        }
        
        Ok(())
    }
    
    /// Boyut al
    pub fn size(&self) -> usize {
        let cache = self.cache.read();
        cache.len()
    }
    
    ///SENTIENT SQLite memory ile entegre et
    pub fn connect_to_sentient_memory(&mut self, _db_path: &str) -> Result<()> {
        // TODO: sentient_memory crate'ine bağlan
        // Bu, sentient_memory::MemoryStore ile entegrasyon sağlayacak
        log::info!("[MemoryAdapter] SENTIENT memory integration prepared");
        Ok(())
    }
}

impl Default for MemoryAdapter {
    fn default() -> Self {
        Self::new()
    }
}

///SENTIENT vector store adapter
pub struct SentientVectorStore {
    // TODO: sentient_vector crate referansı
}

impl SentientVectorStore {
    pub fn new() -> Self {
        Self {}
    }
}

impl VectorStore for SentientVectorStore {
    fn insert(&self, key: &str, value: &str, _embedding: Option<Vec<f32>>) -> Result<()> {
        // TODO: sentient_vector entegrasyonu
        log::debug!("[SentientVectorStore] Insert: {} = {}", key, value);
        Ok(())
    }
    
    fn search(&self, query: &str, limit: usize) -> Result<Vec<String>> {
        // TODO: sentient_vector ile semantic search
        log::debug!("[SentientVectorStore] Search: {} (limit: {})", query, limit);
        Ok(vec![])
    }
    
    fn delete(&self, key: &str) -> Result<()> {
        log::debug!("[SentientVectorStore] Delete: {}", key);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_memory_store_and_retrieve() {
        let memory = MemoryAdapter::new();
        
        memory.store("key1", "value1").await.unwrap();
        memory.store("key2", "value2").await.unwrap();
        
        let entry = memory.get("key1").await;
        assert!(entry.is_some());
        assert_eq!(entry.unwrap().value, "value1");
    }
    
    #[tokio::test]
    async fn test_memory_search() {
        let memory = MemoryAdapter::new();
        
        memory.store("hello", "Hello World").await.unwrap();
        memory.store("test", "Test value").await.unwrap();
        
        let results = memory.search("Hello", 10).await.unwrap();
        assert!(!results.is_empty());
    }
    
    #[test]
    fn test_sentient_vector_store() {
        let store = SentientVectorStore::new();
        
        store.insert("key", "value", None).unwrap();
        let results = store.search("query", 10).unwrap();
        assert!(results.is_empty());  // Henüz implement edilmedi
    }
}
