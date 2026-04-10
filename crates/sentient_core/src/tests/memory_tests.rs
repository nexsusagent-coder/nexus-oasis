//! Memory unit tests

use super::*;

#[cfg(test)]
mod memory_unit_tests {
    use sentient_memory::{MemoryCube, MemoryType};
    use tempfile::tempdir;

    fn create_test_memory() -> MemoryCube {
        let dir = tempdir().expect("Failed to create temp dir");
        let path = dir.path().join("test_memory.db");
        MemoryCube::new(path.to_str().expect("operation failed")).expect("Failed to create memory")
    }

    #[test]
    fn test_memory_creation() {
        let memory = create_test_memory();
        let count = memory.count().expect("Failed to count");
        assert_eq!(count, 0, "New memory should be empty");
    }

    #[test]
    fn test_memory_store_and_retrieve() {
        let memory = create_test_memory();
        
        let id = memory.create(
            "Test content".to_string(),
            MemoryType::Working,
            Some(serde_json::json!({ "key": "value" })),
            None,
        ).expect("Failed to create memory entry");
        
        let entries = memory.search("Test", 10).expect("Failed to search");
        assert!(!entries.is_empty(), "Should find stored entry");
        assert_eq!(entries[0].content, "Test content");
    }

    #[test]
    fn test_memory_types() {
        let memory = create_test_memory();
        
        // Test different memory types
        let types = vec![
            MemoryType::Working,
            MemoryType::LongTerm,
            MemoryType::Episodic,
            MemoryType::Semantic,
        ];
        
        for mem_type in types {
            let id = memory.create(
                format!("Content for {:?}", mem_type),
                mem_type.clone(),
                None,
                None,
            ).expect("Failed to create memory");
            
            let entry = memory.get(id).expect("Failed to get memory");
            assert_eq!(entry.memory_type, mem_type);
        }
    }

    #[test]
    fn test_memory_ttl() {
        let memory = create_test_memory();
        
        // Create entry with 1 second TTL
        let id = memory.create(
            "Short-lived content".to_string(),
            MemoryType::Working,
            None,
            Some(1), // 1 second TTL
        ).expect("Failed to create memory");
        
        // Should be retrievable immediately
        let entry = memory.get(id).expect("Failed to get memory");
        assert!(entry.is_some());
        
        // Wait for TTL to expire
        std::thread::sleep(std::time::Duration::from_secs(2));
        
        // Cleanup expired
        let cleaned = memory.cleanup_expired().expect("Failed to cleanup");
        assert!(cleaned > 0, "Should clean up expired entries");
    }

    #[test]
    fn test_memory_search_semantic() {
        let memory = create_test_memory();
        
        // Store multiple related entries
        memory.create("The quick brown fox".to_string(), MemoryType::Semantic, None, None).expect("operation failed");
        memory.create("A fast fox jumped".to_string(), MemoryType::Semantic, None, None).expect("operation failed");
        memory.create("Dogs are loyal pets".to_string(), MemoryType::Semantic, None, None).expect("operation failed");
        
        // Search for fox-related content
        let results = memory.search("fox", 10).expect("Failed to search");
        assert!(results.len() >= 2, "Should find at least 2 fox-related entries");
    }

    #[test]
    fn test_memory_metadata() {
        let memory = create_test_memory();
        
        let metadata = serde_json::json!({
            "source": "telegram",
            "user_id": "12345",
            "confidence": 0.95,
        });
        
        let id = memory.create(
            "Content with metadata".to_string(),
            MemoryType::Working,
            Some(metadata.clone()),
            None,
        ).expect("Failed to create memory");
        
        let entry = memory.get(id).expect("Failed to get memory").expect("operation failed");
        assert_eq!(entry.metadata["source"], "telegram");
        assert_eq!(entry.metadata["user_id"], "12345");
    }

    #[test]
    fn test_memory_update() {
        let memory = create_test_memory();
        
        let id = memory.create(
            "Original content".to_string(),
            MemoryType::Working,
            None,
            None,
        ).expect("Failed to create memory");
        
        memory.update(id, "Updated content".to_string(), None).expect("Failed to update");
        
        let entry = memory.get(id).expect("Failed to get memory").expect("operation failed");
        assert_eq!(entry.content, "Updated content");
    }

    #[test]
    fn test_memory_delete() {
        let memory = create_test_memory();
        
        let id = memory.create(
            "Content to delete".to_string(),
            MemoryType::Working,
            None,
            None,
        ).expect("Failed to create memory");
        
        memory.delete(id).expect("Failed to delete memory");
        
        let entry = memory.get(id).expect("Failed to get memory");
        assert!(entry.is_none(), "Deleted entry should not be found");
    }

    #[test]
    fn test_memory_count() {
        let memory = create_test_memory();
        
        assert_eq!(memory.count().expect("operation failed"), 0);
        
        for i in 0..10 {
            memory.create(
                format!("Content {}", i),
                MemoryType::Working,
                None,
                None,
            ).expect("operation failed");
        }
        
        assert_eq!(memory.count().expect("operation failed"), 10);
    }

    #[test]
    fn test_memory_clear() {
        let memory = create_test_memory();
        
        for i in 0..5 {
            memory.create(format!("Content {}", i), MemoryType::Working, None, None).expect("operation failed");
        }
        
        assert_eq!(memory.count().expect("operation failed"), 5);
        
        memory.clear().expect("Failed to clear memory");
        
        assert_eq!(memory.count().expect("operation failed"), 0);
    }
}
