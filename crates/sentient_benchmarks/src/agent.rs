//! Agent benchmarking utilities
//!
//! Benchmarks for agent execution, task processing, and multi-agent coordination.

use criterion::{black_box, criterion_group, Criterion, BenchmarkId, BatchSize};
use std::sync::Arc;
use std::time::Duration;

/// Simulated agent task
#[derive(Debug, Clone)]
struct AgentTask {
    id: String,
    prompt: String,
    priority: u8,
    max_tokens: u32,
}

impl AgentTask {
    fn new(id: usize, prompt: &str) -> Self {
        Self {
            id: format!("task-{}", id),
            prompt: prompt.to_string(),
            priority: 1,
            max_tokens: 1000,
        }
    }
}

/// Simulated agent executor
struct AgentExecutor {
    name: String,
    tasks_completed: Arc<std::sync::atomic::AtomicU64>,
}

impl AgentExecutor {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            tasks_completed: Arc::new(std::sync::atomic::AtomicU64::new(0)),
        }
    }

    async fn execute(&self, task: AgentTask) -> String {
        // Simulate processing time (1-5ms)
        let delay = Duration::from_micros(100 + (task.id.len() as u64 % 100));
        tokio::time::sleep(delay).await;

        // Increment counter
        self.tasks_completed.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        format!("Processed: {}", task.id)
    }

    fn get_tasks_completed(&self) -> u64 {
        self.tasks_completed.load(std::sync::atomic::Ordering::Relaxed)
    }
}

/// Benchmark single agent task execution
pub fn bench_agent_task_execution(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().expect("operation failed");
    let agent = AgentExecutor::new("benchmark-agent");

    let mut group = c.benchmark_group("agent_task");

    // Benchmark simple task
    let task = AgentTask::new(1, "Hello, world!");
    group.bench_function("execute_simple", |b| {
        b.to_async(&rt).iter(|| async {
            agent.execute(black_box(task.clone())).await
        })
    });

    // Benchmark tasks with different prompt sizes
    for size in [50, 200, 500, 1000].iter() {
        let prompt = "x".repeat(*size);
        let task = AgentTask::new(1, &prompt);
        group.bench_with_input(BenchmarkId::new("execute_prompt_size", size), size, |b, _| {
            b.to_async(&rt).iter(|| async {
                agent.execute(black_box(task.clone())).await
            })
        });
    }

    group.finish();
}

/// Benchmark agent message processing
pub fn bench_agent_message_processing(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().expect("operation failed");

    let mut group = c.benchmark_group("agent_message");

    group.bench_function("process_message", |b| {
        b.iter_batched(
            || {
                AgentTask::new(1, "Test message for processing benchmark")
            },
            |task| {
                rt.block_on(async {
                    let agent = AgentExecutor::new("msg-agent");
                    agent.execute(task).await
                })
            },
            BatchSize::SmallInput,
        );
    });

    group.finish();
}

/// Benchmark multi-agent coordination
pub fn bench_multi_agent_coordination(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().expect("operation failed");

    let mut group = c.benchmark_group("multi_agent");

    // Benchmark parallel task execution
    for agent_count in [1, 2, 5, 10].iter() {
        group.bench_with_input(
            BenchmarkId::new("parallel_agents", agent_count),
            agent_count,
            |b, _| {
                b.to_async(&rt).iter(|| async {
                    let mut handles = Vec::new();
                    for i in 0..*agent_count {
                        let agent = AgentExecutor::new(&format!("agent-{}", i));
                        let task = AgentTask::new(i, "Parallel task");
                        handles.push(tokio::spawn(async move {
                            agent.execute(task).await
                        }));
                    }
                    // Wait for all agents
                    for handle in handles {
                        handle.await.expect("operation failed");
                    }
                })
            },
        );
    }

    group.finish();
}

/// Benchmark agent task queue operations
pub fn bench_agent_queue(c: &mut Criterion) {
    use std::collections::VecDeque;

    let mut group = c.benchmark_group("agent_queue");

    // Benchmark queue push
    group.bench_function("queue_push", |b| {
        b.iter_batched(
            || {
                let mut queue: VecDeque<AgentTask> = VecDeque::new();
                for i in 0..100 {
                    queue.push_back(AgentTask::new(i, "Queue task"));
                }
                queue
            },
            |mut queue| {
                queue.push_back(black_box(AgentTask::new(999, "New task")));
                queue
            },
            BatchSize::SmallInput,
        );
    });

    // Benchmark queue pop
    group.bench_function("queue_pop", |b| {
        b.iter_batched(
            || {
                let mut queue: VecDeque<AgentTask> = VecDeque::new();
                for i in 0..100 {
                    queue.push_back(AgentTask::new(i, "Queue task"));
                }
                queue
            },
            |mut queue| {
                black_box(queue.pop_front());
                queue
            },
            BatchSize::SmallInput,
        );
    });

    group.finish();
}

/// Benchmark agent state management
pub fn bench_agent_state(c: &mut Criterion) {
    use std::collections::HashMap;

    let mut group = c.benchmark_group("agent_state");

    // Benchmark state read
    let state: HashMap<String, String> = (0..100)
        .map(|i| (format!("key-{}", i), format!("value-{}", i)))
        .collect();

    group.bench_function("state_read", |b| {
        b.iter(|| {
            black_box(state.get("key-50"))
        })
    });

    // Benchmark state write
    group.bench_function("state_write", |b| {
        b.iter_batched(
            || state.clone(),
            |mut state| {
                state.insert("key-50".to_string(), "new-value".to_string());
                black_box(state)
            },
            BatchSize::SmallInput,
        );
    });

    // Benchmark state clone
    group.bench_function("state_clone", |b| {
        b.iter(|| {
            black_box(state.clone())
        })
    });

    group.finish();
}

/// Benchmark agent response generation
pub fn bench_agent_response(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().expect("operation failed");

    let mut group = c.benchmark_group("agent_response");

    // Benchmark response generation with different token counts
    for tokens in [50, 100, 250, 500].iter() {
        let response = "x".repeat(*tokens);
        group.bench_with_input(BenchmarkId::new("generate_response", tokens), tokens, |b, _| {
            b.to_async(&rt).iter(|| async {
                // Simulate response generation
                tokio::time::sleep(Duration::from_micros(50)).await;
                black_box(response.clone())
            })
        });
    }

    group.finish();
}

// Register all benchmark groups
criterion_group!(
    agent_benches,
    bench_agent_task_execution,
    bench_agent_message_processing,
    bench_multi_agent_coordination,
    bench_agent_queue,
    bench_agent_state,
    bench_agent_response,
);
