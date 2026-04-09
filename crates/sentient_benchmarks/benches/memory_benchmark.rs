//! Memory operation benchmarks

use criterion::{Criterion, black_box, criterion_group};
use sentient_memory::{MemoryEntry, MemoryStore};
use sentient_lancedb::LanceDBMemory;

pub fn bench_memory_store(c: &mut Criterion) {
    // Benchmark memory storage
    c.bench_function("memory_store_insert", |b| {
        let store = MemoryStore::new();
        let entry = MemoryEntry::new("test content", vec![]);
        b.iter(|| {
            store.insert(black_box(entry.clone()))
        });
    });

    // Benchmark memory retrieval
    c.bench_function("memory_store_get", |b| {
        let store = MemoryStore::new();
        let id = store.insert(MemoryEntry::new("test content", vec![]));
        b.iter(|| {
            store.get(black_box(&id))
        });
    });

    // Benchmark memory search
    c.bench_function("memory_store_search", |b| {
        let store = MemoryStore::new();
        for i in 0..1000 {
            store.insert(MemoryEntry::new(format!("content {}", i), vec![]));
        }
        b.iter(|| {
            store.search(black_box("content"))
        });
    });
}

criterion_group!(memory_benches, bench_memory_store);
