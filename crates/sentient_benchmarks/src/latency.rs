//! Latency measurement utilities
//!
//! Comprehensive latency measurements for various system operations.

use criterion::{black_box, criterion_group, Criterion, BenchmarkId};
use std::time::{Duration, Instant};

/// Latency measurement result
#[derive(Debug, Clone)]
pub struct LatencyResult {
    pub operation: String,
    pub min_us: u64,
    pub max_us: u64,
    pub mean_us: f64,
    pub p50_us: f64,
    pub p95_us: f64,
    pub p99_us: f64,
}

impl LatencyResult {
    pub fn from_measurements(operation: &str, measurements: Vec<Duration>) -> Self {
        let mut us_values: Vec<f64> = measurements
            .iter()
            .map(|d| d.as_micros() as f64)
            .collect();
        us_values.sort_by(|a, b| a.partial_cmp(b).expect("operation failed"));

        let count = us_values.len();
        if count == 0 {
            return Self {
                operation: operation.to_string(),
                min_us: 0,
                max_us: 0,
                mean_us: 0.0,
                p50_us: 0.0,
                p95_us: 0.0,
                p99_us: 0.0,
            };
        }

        let min_us = us_values.first().copied().unwrap_or(0.0) as u64;
        let max_us = us_values.last().copied().unwrap_or(0.0) as u64;
        let mean_us = us_values.iter().sum::<f64>() / count as f64;
        let p50_us = us_values[count / 2];
        let p95_us = us_values[(count as f64 * 0.95) as usize];
        let p99_us = us_values[(count as f64 * 0.99) as usize];

        Self {
            operation: operation.to_string(),
            min_us,
            max_us,
            mean_us,
            p50_us,
            p95_us,
            p99_us,
        }
    }
}

/// Measure function execution latency
pub fn measure_latency<F, R>(operation: &str, iterations: usize, mut f: F) -> LatencyResult
where
    F: FnMut() -> R,
{
    let mut measurements = Vec::with_capacity(iterations);

    for _ in 0..iterations {
        let start = Instant::now();
        black_box(f());
        measurements.push(start.elapsed());
    }

    LatencyResult::from_measurements(operation, measurements)
}

/// Benchmark function call latency
pub fn bench_function_call_latency(c: &mut Criterion) {
    let mut group = c.benchmark_group("latency_function_call");

    // Benchmark simple function call
    group.bench_function("simple_call", |b| {
        b.iter(|| {
            let _ = black_box(|| 42)();
        })
    });

    // Benchmark closure call
    group.bench_function("closure_call", |b| {
        let closure = || 42;
        b.iter(|| {
            black_box(closure())
        })
    });

    // Benchmark trait object call
    group.bench_function("trait_object_call", |b| {
        let boxed: Box<dyn Fn() -> i32> = Box::new(|| 42);
        b.iter(|| {
            black_box(boxed())
        })
    });

    group.finish();
}

/// Benchmark async operation latency
pub fn bench_async_latency(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().expect("operation failed");

    let mut group = c.benchmark_group("latency_async");

    // Benchmark async spawn latency
    group.bench_function("task_spawn", |b| {
        b.to_async(&rt).iter(|| async {
            tokio::spawn(async { 42 }).await.expect("operation failed")
        })
    });

    // Benchmark channel send latency
    group.bench_function("channel_send", |b| {
        b.to_async(&rt).iter(|| async {
            let (tx, mut rx) = tokio::sync::mpsc::channel::<i32>(1);
            tx.send(42).await.expect("operation failed");
            rx.recv().await.expect("operation failed")
        })
    });

    // Benchmark mutex lock latency
    group.bench_function("mutex_lock", |b| {
        b.to_async(&rt).iter(|| async {
            let mutex = tokio::sync::Mutex::new(42);
            let guard = mutex.lock().await;
            *guard
        })
    });

    group.finish();
}

/// Benchmark memory operation latency
pub fn bench_memory_latency(c: &mut Criterion) {
    let mut group = c.benchmark_group("latency_memory");

    // Benchmark Vec allocation
    for size in [10, 100, 1000, 10000].iter() {
        group.bench_with_input(BenchmarkId::new("vec_alloc", size), size, |b, _| {
            b.iter(|| {
                let v: Vec<u8> = black_box(vec![0u8; *size]);
                v
            })
        });
    }

    // Benchmark HashMap insertion
    group.bench_function("hashmap_insert", |b| {
        b.iter(|| {
            let mut map = std::collections::HashMap::new();
            map.insert("key", "value");
            black_box(map)
        })
    });

    // Benchmark HashMap lookup
    let map: std::collections::HashMap<&str, &str> = (0..1000)
        .map(|i| (Box::leak(format!("key-{}", i).into_boxed_str()) as &str, "value"))
        .collect();

    group.bench_function("hashmap_lookup", |b| {
        b.iter(|| {
            black_box(map.get("key-500"))
        })
    });

    group.finish();
}

/// Benchmark serialization latency
pub fn bench_serialization_latency(c: &mut Criterion) {
    #[derive(serde::Serialize, serde::Deserialize)]
    struct TestData {
        id: u32,
        name: String,
        values: Vec<i32>,
        nested: NestedData,
    }

    #[derive(serde::Serialize, serde::Deserialize)]
    struct NestedData {
        x: f64,
        y: f64,
    }

    let test_data = TestData {
        id: 42,
        name: "benchmark".to_string(),
        values: vec![1, 2, 3, 4, 5],
        nested: NestedData { x: 1.5, y: 2.5 },
    };

    let mut group = c.benchmark_group("latency_serialization");

    // Benchmark JSON serialization
    group.bench_function("json_serialize", |b| {
        b.iter(|| {
            serde_json::to_string(black_box(&test_data))
        })
    });

    // Benchmark JSON deserialization
    let json = serde_json::to_string(&test_data).expect("operation failed");
    group.bench_function("json_deserialize", |b| {
        b.iter(|| {
            serde_json::from_str::<TestData>(black_box(&json))
        })
    });

    // Benchmark binary serialization (bincode style)
    group.bench_function("binary_serialize", |b| {
        b.iter(|| {
            // Manual binary packing simulation
            let mut buf = Vec::new();
            buf.extend_from_slice(&test_data.id.to_le_bytes());
            buf.extend_from_slice(&(test_data.name.len() as u32).to_le_bytes());
            buf.extend_from_slice(test_data.name.as_bytes());
            black_box(buf)
        })
    });

    group.finish();
}

/// Benchmark network simulation latency
pub fn bench_network_latency(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().expect("operation failed");

    let mut group = c.benchmark_group("latency_network");

    // Simulate local network latency
    group.bench_function("localhost_sim", |b| {
        b.to_async(&rt).iter(|| async {
            tokio::time::sleep(Duration::from_micros(100)).await;
            black_box(42)
        })
    });

    // Simulate remote network latency
    group.bench_function("remote_sim", |b| {
        b.to_async(&rt).iter(|| async {
            tokio::time::sleep(Duration::from_millis(10)).await;
            black_box(42)
        })
    });

    // Simulate API call latency
    group.bench_function("api_call_sim", |b| {
        b.to_async(&rt).iter(|| async {
            // DNS resolution: 1-5ms
            tokio::time::sleep(Duration::from_micros(500)).await;
            // TLS handshake: 10-50ms
            tokio::time::sleep(Duration::from_micros(1000)).await;
            // HTTP request: 5-20ms
            tokio::time::sleep(Duration::from_micros(200)).await;
            black_box(42)
        })
    });

    group.finish();
}

/// Benchmark database operation latency
pub fn bench_database_latency(c: &mut Criterion) {
    let mut group = c.benchmark_group("latency_database");

    // Simulate SQLite operations
    group.bench_function("sqlite_insert_sim", |b| {
        b.iter(|| {
            // Simulate SQLite INSERT latency (~1-5ms)
            std::thread::sleep(Duration::from_micros(500));
            black_box(1)
        })
    });

    group.bench_function("sqlite_select_sim", |b| {
        b.iter(|| {
            // Simulate SQLite SELECT latency (~0.1-1ms)
            std::thread::sleep(Duration::from_micros(100));
            black_box(1)
        })
    });

    // Simulate Redis operations
    group.bench_function("redis_get_sim", |b| {
        b.iter(|| {
            // Simulate Redis GET latency (~0.1-0.5ms)
            std::thread::sleep(Duration::from_micros(50));
            black_box("value")
        })
    });

    group.bench_function("redis_set_sim", |b| {
        b.iter(|| {
            // Simulate Redis SET latency (~0.1-0.5ms)
            std::thread::sleep(Duration::from_micros(50));
            black_box("OK")
        })
    });

    group.finish();
}

// Register all benchmark groups
criterion_group!(
    latency_benches,
    bench_function_call_latency,
    bench_async_latency,
    bench_memory_latency,
    bench_serialization_latency,
    bench_network_latency,
    bench_database_latency,
);
