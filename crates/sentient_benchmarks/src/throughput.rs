//! Throughput measurement utilities
//!
//! Comprehensive throughput measurements for system operations.

use criterion::{black_box, criterion_group, Criterion, BenchmarkId, Throughput};
use std::time::Duration;

/// Throughput result
#[derive(Debug, Clone)]
pub struct ThroughputResult {
    pub operation: String,
    pub total_items: u64,
    pub total_time_ms: u64,
    pub items_per_second: f64,
    pub bytes_per_second: Option<u64>,
}

impl ThroughputResult {
    pub fn new(operation: &str, total_items: u64, total_time_ms: u64) -> Self {
        let items_per_second = if total_time_ms > 0 {
            (total_items as f64) / (total_time_ms as f64 / 1000.0)
        } else {
            0.0
        };

        Self {
            operation: operation.to_string(),
            total_items,
            total_time_ms,
            items_per_second,
            bytes_per_second: None,
        }
    }

    pub fn with_bytes(mut self, bytes: u64) -> Self {
        if self.total_time_ms > 0 {
            self.bytes_per_second = Some(
                (bytes as f64 / (self.total_time_ms as f64 / 1000.0)) as u64
            );
        }
        self
    }
}

/// Benchmark message throughput
pub fn bench_message_throughput(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().expect("operation failed");

    let mut group = c.benchmark_group("throughput_messages");
    group.sample_size(50);

    // Benchmark message generation throughput
    for count in [100, 500, 1000, 5000].iter() {
        group.throughput(Throughput::Elements(*count as u64));
        group.bench_with_input(
            BenchmarkId::new("generate_messages", count),
            count,
            |b, _| {
                b.to_async(&rt).iter(|| async {
                    let mut messages = Vec::with_capacity(*count);
                    for i in 0..*count {
                        messages.push(format!("Message {}", i));
                    }
                    black_box(messages)
                })
            },
        );
    }

    group.finish();
}

/// Benchmark channel throughput
pub fn bench_channel_throughput(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().expect("operation failed");

    let mut group = c.benchmark_group("throughput_channel");
    group.sample_size(50);

    // Benchmark mpsc channel throughput
    for count in [100, 500, 1000].iter() {
        group.throughput(Throughput::Elements(*count as u64));
        group.bench_with_input(
            BenchmarkId::new("mpsc_throughput", count),
            count,
            |b, _| {
                b.to_async(&rt).iter(|| async {
                    let (tx, mut rx) = tokio::sync::mpsc::channel::<u32>(100);
                    let sender = tokio::spawn(async move {
                        for i in 0..*count as u32 {
                            if tx.send(i).await.is_err() {
                                break;
                            }
                        }
                    });

                    let receiver = tokio::spawn(async move {
                        let mut received = 0;
                        while let Some(_) = rx.recv().await {
                            received += 1;
                        }
                        received
                    });

                    sender.await.expect("operation failed");
                    black_box(receiver.await.expect("operation failed"))
                })
            },
        );
    }

    group.finish();
}

/// Benchmark memory write throughput
pub fn bench_memory_write_throughput(c: &mut Criterion) {
    let mut group = c.benchmark_group("throughput_memory_write");
    group.sample_size(50);

    // Benchmark Vec push throughput
    for size in [1000, 10000, 100000, 1000000].iter() {
        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_with_input(
            BenchmarkId::new("vec_push_bytes", size),
            size,
            |b, _| {
                b.iter(|| {
                    let mut v = Vec::with_capacity(*size);
                    for _ in 0..*size {
                        v.push(0u8);
                    }
                    black_box(v)
                })
            },
        );
    }

    // Benchmark Vec extend throughput
    for size in [1000, 10000, 100000, 1000000].iter() {
        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_with_input(
            BenchmarkId::new("vec_extend", size),
            size,
            |b, _| {
                b.iter(|| {
                    let mut v = Vec::with_capacity(*size);
                    v.extend(std::iter::repeat(0u8).take(*size));
                    black_box(v)
                })
            },
        );
    }

    group.finish();
}

/// Benchmark JSON parsing throughput
pub fn bench_json_throughput(c: &mut Criterion) {
    let mut group = c.benchmark_group("throughput_json");
    group.sample_size(50);

    // Create test JSON data
    fn create_json_array(count: usize) -> String {
        let items: Vec<serde_json::Value> = (0..count)
            .map(|i| {
                serde_json::json!({
                    "id": i,
                    "name": format!("item-{}", i),
                    "value": i * 100,
                    "active": i % 2 == 0,
                })
            })
            .collect();
        serde_json::to_string(&items).expect("operation failed")
    }

    // Benchmark JSON parse throughput
    for count in [10, 50, 100, 500].iter() {
        let json = create_json_array(*count);
        let json_size = json.len() as u64;
        group.throughput(Throughput::Bytes(json_size));

        group.bench_with_input(
            BenchmarkId::new("parse_json_array", count),
            &json,
            |b, json| {
                b.iter(|| {
                    let parsed: Vec<serde_json::Value> =
                        serde_json::from_str(black_box(json)).expect("operation failed");
                    parsed
                })
            },
        );
    }

    // Benchmark JSON stringify throughput
    for count in [10, 50, 100, 500].iter() {
        let items: Vec<serde_json::Value> = (0..*count)
            .map(|i| serde_json::json!({"id": i, "name": format!("item-{}", i)}))
            .collect();

        group.bench_with_input(
            BenchmarkId::new("stringify_json_array", count),
            &items,
            |b, items| {
                b.iter(|| {
                    serde_json::to_string(black_box(items)).expect("operation failed")
                })
            },
        );
    }

    group.finish();
}

/// Benchmark concurrent task throughput
pub fn bench_concurrent_throughput(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().expect("operation failed");

    let mut group = c.benchmark_group("throughput_concurrent");
    group.sample_size(50);

    // Benchmark concurrent task spawn throughput
    for count in [10, 50, 100, 500].iter() {
        group.throughput(Throughput::Elements(*count as u64));
        group.bench_with_input(
            BenchmarkId::new("spawn_tasks", count),
            count,
            |b, _| {
                b.to_async(&rt).iter(|| async {
                    let mut handles = Vec::with_capacity(*count);
                    for i in 0..*count {
                        handles.push(tokio::spawn(async move {
                            // Simple computation
                            let mut sum = 0u64;
                            for j in 0..100 {
                                sum += (i + j) as u64;
                            }
                            sum
                        }));
                    }
                    // Wait for all tasks
                    let mut total = 0u64;
                    for handle in handles {
                        total += handle.await.expect("operation failed");
                    }
                    black_box(total)
                })
            },
        );
    }

    group.finish();
}

/// Benchmark string processing throughput
pub fn bench_string_throughput(c: &mut Criterion) {
    let mut group = c.benchmark_group("throughput_string");
    group.sample_size(50);

    // Benchmark string concatenation
    for count in [100, 500, 1000, 5000].iter() {
        let strings: Vec<String> = (0..*count).map(|i| format!("string-{}", i)).collect();
        let total_size: usize = strings.iter().map(|s| s.len()).sum();
        group.throughput(Throughput::Bytes(total_size as u64));

        group.bench_with_input(
            BenchmarkId::new("concat_strings", count),
            &strings,
            |b, strings| {
                b.iter(|| {
                    let mut result = String::new();
                    for s in strings {
                        result.push_str(black_box(s));
                    }
                    result
                })
            },
        );
    }

    // Benchmark string split
    let long_string = (0..1000).map(|i| format!("word{} ", i)).collect::<String>();

    group.bench_function("split_string", |b| {
        b.iter(|| {
            let parts: Vec<&str> = long_string.split(' ').collect();
            black_box(parts)
        })
    });

    group.finish();
}

/// Benchmark hash computation throughput
pub fn bench_hash_throughput(c: &mut Criterion) {
    use std::collections::HashMap;

    let mut group = c.benchmark_group("throughput_hash");
    group.sample_size(50);

    // Benchmark HashMap insert throughput
    for count in [100, 500, 1000, 5000].iter() {
        group.throughput(Throughput::Elements(*count as u64));
        group.bench_with_input(
            BenchmarkId::new("hashmap_insert", count),
            count,
            |b, _| {
                b.iter(|| {
                    let mut map = HashMap::with_capacity(*count);
                    for i in 0..*count {
                        map.insert(format!("key-{}", i), format!("value-{}", i));
                    }
                    black_box(map)
                })
            },
        );
    }

    // Benchmark HashMap lookup throughput
    let prepopulated: HashMap<String, String> = (0..5000)
        .map(|i| (format!("key-{}", i), format!("value-{}", i)))
        .collect();

    for count in [100, 500, 1000, 5000].iter() {
        group.throughput(Throughput::Elements(*count as u64));
        group.bench_with_input(
            BenchmarkId::new("hashmap_lookup", count),
            count,
            |b, _| {
                b.iter(|| {
                    let mut found = 0;
                    for i in 0..*count {
                        if prepopulated.contains_key(&format!("key-{}", i)) {
                            found += 1;
                        }
                    }
                    black_box(found)
                })
            },
        );
    }

    group.finish();
}

// Register all benchmark groups
criterion_group!(
    throughput_benches,
    bench_message_throughput,
    bench_channel_throughput,
    bench_memory_write_throughput,
    bench_json_throughput,
    bench_concurrent_throughput,
    bench_string_throughput,
    bench_hash_throughput,
);
