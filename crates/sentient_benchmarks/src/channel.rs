//! Channel benchmarking utilities
//!
//! Benchmarks for Telegram, Discord, Slack channel operations.

use criterion::{black_box, criterion_group, Criterion, BenchmarkId, BatchSize};
use serde::{Serialize, Deserialize};
use std::time::Duration;

/// Simulated message
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ChannelMessage {
    channel_id: String,
    content: String,
    author: String,
    timestamp: i64,
}

impl ChannelMessage {
    fn new(content: &str) -> Self {
        Self {
            channel_id: "test-channel".to_string(),
            content: content.to_string(),
            author: "benchmark-user".to_string(),
            timestamp: chrono::Utc::now().timestamp(),
        }
    }
}

/// Simulated channel
struct Channel {
    name: String,
    message_count: std::sync::atomic::AtomicU64,
}

impl Channel {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            message_count: std::sync::atomic::AtomicU64::new(0),
        }
    }

    async fn send(&self, msg: ChannelMessage) -> bool {
        // Simulate network latency (1-10ms)
        let delay = Duration::from_micros(500 + msg.content.len() as u64 % 500);
        tokio::time::sleep(delay).await;

        self.message_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        true
    }

    async fn receive(&self) -> Option<ChannelMessage> {
        // Simulate receive latency
        tokio::time::sleep(Duration::from_micros(100)).await;

        Some(ChannelMessage::new("Received message"))
    }
}

/// Benchmark channel message sending
pub fn bench_channel_send(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().expect("operation failed");
    let channel = Channel::new("benchmark-channel");

    let mut group = c.benchmark_group("channel_send");

    // Benchmark simple message
    let msg = ChannelMessage::new("Hello, world!");
    group.bench_function("send_simple", |b| {
        b.to_async(&rt).iter(|| async {
            channel.send(black_box(msg.clone())).await
        })
    });

    // Benchmark messages with different sizes
    for size in [100, 500, 1000, 2000].iter() {
        let content = "x".repeat(*size);
        let msg = ChannelMessage::new(&content);
        group.bench_with_input(BenchmarkId::new("send_size", size), size, |b, _| {
            b.to_async(&rt).iter(|| async {
                channel.send(black_box(msg.clone())).await
            })
        });
    }

    group.finish();
}

/// Benchmark channel message receiving
pub fn bench_channel_receive(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().expect("operation failed");
    let channel = Channel::new("benchmark-channel");

    let mut group = c.benchmark_group("channel_receive");

    group.bench_function("receive_message", |b| {
        b.to_async(&rt).iter(|| async {
            channel.receive().await
        })
    });

    group.finish();
}

/// Benchmark channel concurrent operations
pub fn bench_channel_concurrent(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().expect("operation failed");

    let mut group = c.benchmark_group("channel_concurrent");

    // Benchmark concurrent sends
    for concurrent_count in [1, 5, 10, 20].iter() {
        group.bench_with_input(
            BenchmarkId::new("concurrent_sends", concurrent_count),
            concurrent_count,
            |b, _| {
                b.to_async(&rt).iter(|| async {
                    let mut handles = Vec::new();
                    for i in 0..*concurrent_count {
                        let channel = Channel::new(&format!("channel-{}", i));
                        let msg = ChannelMessage::new(&format!("Concurrent message {}", i));
                        handles.push(tokio::spawn(async move {
                            channel.send(msg).await
                        }));
                    }
                    for handle in handles {
                        handle.await.expect("operation failed");
                    }
                })
            },
        );
    }

    group.finish();
}

/// Benchmark message formatting
pub fn bench_message_format(c: &mut Criterion) {
    let mut group = c.benchmark_group("message_format");

    // Benchmark Markdown formatting
    group.bench_function("format_markdown", |b| {
        b.iter(|| {
            let msg = ChannelMessage::new("Test message");
            // Simulate Markdown processing
            let formatted = format!(
                "**{}**: {}\n_{}_",
                msg.author,
                msg.content,
                chrono::DateTime::from_timestamp(msg.timestamp, 0)
                    .map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string())
                    .unwrap_or_default()
            );
            black_box(formatted)
        })
    });

    // Benchmark JSON serialization
    group.bench_function("format_json", |b| {
        b.iter(|| {
            let msg = ChannelMessage::new("Test message");
            serde_json::to_string(&msg).ok()
        })
    });

    group.finish();
}

/// Benchmark channel buffer operations
pub fn bench_channel_buffer(c: &mut Criterion) {
    use std::collections::VecDeque;

    let mut group = c.benchmark_group("channel_buffer");

    // Benchmark buffer write
    group.bench_function("buffer_write", |b| {
        b.iter_batched(
            || {
                let mut buffer: VecDeque<ChannelMessage> = VecDeque::new();
                for i in 0..100 {
                    buffer.push_back(ChannelMessage::new(&format!("Buffer message {}", i)));
                }
                buffer
            },
            |mut buffer| {
                buffer.push_back(black_box(ChannelMessage::new("New buffered message")));
                buffer
            },
            BatchSize::SmallInput,
        );
    });

    // Benchmark buffer read
    group.bench_function("buffer_read", |b| {
        b.iter_batched(
            || {
                let mut buffer: VecDeque<ChannelMessage> = VecDeque::new();
                for i in 0..100 {
                    buffer.push_back(ChannelMessage::new(&format!("Buffer message {}", i)));
                }
                buffer
            },
            |mut buffer| {
                black_box(buffer.pop_front());
                buffer
            },
            BatchSize::SmallInput,
        );
    });

    group.finish();
}

/// Benchmark webhook operations (simulated)
pub fn bench_channel_webhook(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().expect("operation failed");

    let mut group = c.benchmark_group("channel_webhook");

    group.bench_function("webhook_send", |b| {
        b.to_async(&rt).iter(|| async {
            // Simulate webhook POST request
            let payload = serde_json::json!({
                "content": "Webhook test message",
                "username": "Sentient Bot"
            });
            tokio::time::sleep(Duration::from_micros(200)).await;
            black_box(payload)
        })
    });

    group.finish();
}

// Register all benchmark groups
criterion_group!(
    channel_benches,
    bench_channel_send,
    bench_channel_receive,
    bench_channel_concurrent,
    bench_message_format,
    bench_channel_buffer,
    bench_channel_webhook,
);
