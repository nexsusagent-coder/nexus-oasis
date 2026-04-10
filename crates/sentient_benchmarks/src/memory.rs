//! Memory benchmarking utilities
//!
//! Benchmarks for MemoryCube operations including store, recall, search.

use criterion::{black_box, criterion_group, Criterion, BenchmarkId};
use sentient_memory::{MemoryCube, MemoryEntry, MemoryInput, Importance, MemoryType};

/// Create a test memory entry
fn create_test_entry(content: &str) -> MemoryEntry {
    MemoryEntry::from_input(
        MemoryInput::new(content)
            .with_importance(Importance::medium())
    )
}

/// Benchmark memory store operations
pub fn bench_memory_store(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_store");

    // Benchmark storing single entry
    group.bench_function("store_single", |b| {
        b.iter_batched(
            || {
                let mut cube = MemoryCube::new(":memory:").expect("operation failed");
                let entry = create_test_entry("Test memory content for benchmarking");
                (cube, entry)
            },
            |(mut cube, entry)| {
                black_box(cube.store(entry).expect("operation failed"))
            },
            criterion::BatchSize::SmallInput,
        )
    });

    // Benchmark storing with different content sizes
    for size in [100, 500, 1000, 5000].iter() {
        let content = "x".repeat(*size);
        group.bench_with_input(BenchmarkId::new("store_size", size), size, |b, _| {
            b.iter_batched(
                || {
                    let mut cube = MemoryCube::new(":memory:").expect("operation failed");
                    let entry = create_test_entry(&content);
                    (cube, entry)
                },
                |(mut cube, entry)| {
                    black_box(cube.store(entry).expect("operation failed"))
                },
                criterion::BatchSize::SmallInput,
            )
        });
    }

    group.finish();
}

/// Benchmark memory recall operations
pub fn bench_memory_recall(c: &mut Criterion) {
    // Pre-populate memory
    let mut cube = MemoryCube::new(":memory:").expect("operation failed");
    for i in 0..1000 {
        let entry = create_test_entry(&format!("Memory entry number {}", i));
        cube.store(entry).expect("operation failed");
    }

    let mut group = c.benchmark_group("memory_recall");

    // Benchmark recall by ID
    let all_entries = cube.list_all().expect("operation failed");
    let test_id = all_entries.first().expect("operation failed").id;

    group.bench_function("recall_by_id", |b| {
        b.iter(|| {
            black_box(cube.recall(test_id))
        })
    });

    // Benchmark list_all operation
    group.bench_function("list_all", |b| {
        b.iter(|| {
            black_box(cube.list_all())
        })
    });

    group.finish();
}

/// Benchmark memory search operations
pub fn bench_memory_search(c: &mut Criterion) {
    // Pre-populate memory with various content
    let mut cube = MemoryCube::new(":memory:").expect("operation failed");
    let topics = ["AI", "memory", "benchmark", "rust", "performance",
                 "database", "vector", "search", "embedding", "storage"];
    for i in 0..500 {
        let topic = topics[i % topics.len()];
        let entry = create_test_entry(&format!("Entry about {}: content number {}", topic, i));
        cube.store(entry).expect("operation failed");
    }

    let mut group = c.benchmark_group("memory_search");

    // Benchmark text search (all types)
    group.bench_function("search_all_types", |b| {
        b.iter(|| {
            black_box(cube.search("memory", None))
        })
    });

    // Benchmark search with memory type filter
    group.bench_function("search_semantic", |b| {
        b.iter(|| {
            black_box(cube.search("content", Some(MemoryType::Semantic)))
        })
    });

    group.finish();
}

/// Benchmark memory update operations
pub fn bench_memory_update(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_update");

    group.bench_function("update_entry", |b| {
        b.iter_batched(
            || {
                let mut cube = MemoryCube::new(":memory:").expect("operation failed");
                let input = create_test_entry("Original content");
                let id = cube.store(input).expect("operation failed");
                let mut entry = cube.recall(id).expect("operation failed").expect("operation failed");
                entry.content = "Updated content".to_string();
                (cube, entry)
            },
            |(mut cube, entry)| {
                black_box(cube.update(entry).expect("operation failed"))
            },
            criterion::BatchSize::SmallInput,
        )
    });

    group.finish();
}

/// Benchmark memory delete operations
pub fn bench_memory_delete(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_delete");

    group.bench_function("delete_entry", |b| {
        b.iter_batched(
            || {
                let mut cube = MemoryCube::new(":memory:").expect("operation failed");
                let input = create_test_entry("To be deleted");
                let id = cube.store(input).expect("operation failed");
                (cube, id)
            },
            |(mut cube, id)| {
                black_box(cube.delete(id).expect("operation failed"))
            },
            criterion::BatchSize::SmallInput,
        );
    });

    group.finish();
}

/// Benchmark bulk operations
pub fn bench_memory_bulk(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_bulk");

    // Benchmark bulk store
    for count in [10, 50, 100].iter() {
        group.bench_with_input(BenchmarkId::new("bulk_store", count), count, |b, _| {
            b.iter_batched(
                || MemoryCube::new(":memory:").expect("operation failed"),
                |mut cube| {
                    for i in 0..*count {
                        let entry = create_test_entry(&format!("Bulk entry {}", i));
                        cube.store(entry).expect("operation failed");
                    }
                },
                criterion::BatchSize::SmallInput,
            )
        });
    }

    group.finish();
}

// Register all benchmark groups
criterion_group!(
    memory_benches,
    bench_memory_store,
    bench_memory_recall,
    bench_memory_search,
    bench_memory_update,
    bench_memory_delete,
    bench_memory_bulk,
);
