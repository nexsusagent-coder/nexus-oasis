//! Voice benchmarking utilities
//!
//! Benchmarks for STT (Speech-to-Text) and TTS (Text-to-Speech) operations.

use criterion::{black_box, criterion_group, Criterion, BenchmarkId, BatchSize};
use std::time::Duration;

/// Simulated audio buffer
#[derive(Debug, Clone)]
struct AudioBuffer {
    sample_rate: u32,
    channels: u8,
    samples: Vec<i16>,
}

impl AudioBuffer {
    fn new(duration_ms: u32, sample_rate: u32, channels: u8) -> Self {
        let sample_count = (duration_ms as u64 * sample_rate as u64 * channels as u64 / 1000) as usize;
        Self {
            sample_rate,
            channels,
            samples: vec![0i16; sample_count],
        }
    }

    fn byte_size(&self) -> usize {
        self.samples.len() * 2
    }
}

/// Simulated STT engine
struct SttEngine {
    name: String,
}

impl SttEngine {
    fn new(name: &str) -> Self {
        Self { name: name.to_string() }
    }

    async fn transcribe(&self, audio: &AudioBuffer) -> String {
        // Simulate STT processing (10-50ms per second of audio)
        let delay = Duration::from_micros(
            (audio.samples.len() as f64 / audio.sample_rate as f64 * 10000.0) as u64
        );
        tokio::time::sleep(delay).await;

        format!("Transcribed {} samples from {}", audio.samples.len(), self.name)
    }
}

/// Simulated TTS engine
struct TtsEngine {
    name: String,
}

impl TtsEngine {
    fn new(name: &str) -> Self {
        Self { name: name.to_string() }
    }

    async fn synthesize(&self, text: &str, sample_rate: u32) -> AudioBuffer {
        // Simulate TTS processing (1-5ms per character)
        let delay = Duration::from_micros((text.len() as u64 * 100).min(50000));
        tokio::time::sleep(delay).await;

        // Create audio buffer proportional to text length
        let sample_count = (text.len() as f64 * sample_rate as f64 * 0.05) as usize;
        AudioBuffer {
            sample_rate,
            channels: 1,
            samples: vec![0i16; sample_count.max(1000)],
        }
    }
}

/// Benchmark STT operations
pub fn bench_stt_transcription(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().expect("operation failed");
    let stt = SttEngine::new("benchmark-stt");

    let mut group = c.benchmark_group("voice_stt");

    // Benchmark transcription of different audio durations
    for duration_ms in [100, 500, 1000, 5000].iter() {
        let audio = AudioBuffer::new(*duration_ms, 16000, 1);

        group.throughput(criterion::Throughput::Bytes(audio.byte_size() as u64));
        group.bench_with_input(
            BenchmarkId::new("transcribe", duration_ms),
            &audio,
            |b, audio| {
                b.to_async(&rt).iter(|| async {
                    stt.transcribe(black_box(audio)).await
                })
            },
        );
    }

    group.finish();
}

/// Benchmark TTS operations
pub fn bench_tts_synthesis(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().expect("operation failed");
    let tts = TtsEngine::new("benchmark-tts");

    let mut group = c.benchmark_group("voice_tts");

    // Benchmark synthesis of different text lengths
    for text_len in [10, 50, 100, 500, 1000].iter() {
        let text = "x".repeat(*text_len);

        group.throughput(criterion::Throughput::Bytes(*text_len as u64));
        group.bench_with_input(
            BenchmarkId::new("synthesize", text_len),
            &text,
            |b, text| {
                b.to_async(&rt).iter(|| async {
                    tts.synthesize(black_box(text), 16000).await
                })
            },
        );
    }

    group.finish();
}

/// Benchmark audio processing operations
pub fn bench_audio_processing(c: &mut Criterion) {
    let mut group = c.benchmark_group("voice_audio_processing");

    // Benchmark resampling
    for sample_count in [16000, 32000, 48000, 96000].iter() {
        let mut samples = vec![0i16; *sample_count as usize];

        group.throughput(criterion::Throughput::Bytes(
            (*sample_count * 2) as u64
        ));
        group.bench_with_input(
            BenchmarkId::new("resample", sample_count),
            &mut samples,
            |b, samples| {
                b.iter(|| {
                    // Simulate simple resampling (linear interpolation)
                    let ratio = 0.5;
                    let new_len = (samples.len() as f64 * ratio) as usize;
                    let mut output = Vec::with_capacity(new_len);
                    for i in 0..new_len {
                        let src_idx = (i as f64 / ratio) as usize;
                        let sample = samples.get(src_idx).copied().unwrap_or(0);
                        output.push(black_box(sample));
                    }
                    output
                })
            },
        );
    }

    // Benchmark normalization
    group.bench_function("normalize_audio", |b| {
        b.iter(|| {
            let mut samples = vec![100i16; 16000];
            let max_sample = samples.iter().map(|&s| s.abs()).max().unwrap_or(1);
            let scale = 32767.0 / max_sample as f64;
            for sample in samples.iter_mut() {
                *sample = (*sample as f64 * scale) as i16;
            }
            black_box(samples)
        })
    });

    // Benchmark silence detection
    group.bench_function("detect_silence", |b| {
        b.iter_batched(
            || vec![100i16; 16000],
            |samples| {
                let threshold = 500i16;
                let silent_count = samples
                    .iter()
                    .filter(|&&s| s.abs() < threshold)
                    .count();
                black_box(silent_count)
            },
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

/// Benchmark audio buffer operations
pub fn bench_audio_buffer_ops(c: &mut Criterion) {
    let mut group = c.benchmark_group("voice_buffer_ops");

    // Benchmark buffer creation
    for size in [16000, 44100, 48000, 96000].iter() {
        group.bench_with_input(
            BenchmarkId::new("create_buffer", size),
            size,
            |b, _| {
                b.iter(|| {
                    let buffer = vec![0i16; *size];
                    black_box(buffer)
                })
            },
        );
    }

    // Benchmark buffer concatenation
    let buf1 = vec![1i16; 16000];
    let buf2 = vec![2i16; 16000];

    group.bench_function("concat_buffers", |b| {
        b.iter(|| {
            let mut combined = Vec::with_capacity(buf1.len() + buf2.len());
            combined.extend_from_slice(black_box(&buf1));
            combined.extend_from_slice(black_box(&buf2));
            combined
        })
    });

    // Benchmark buffer mixing
    let left = vec![100i16; 16000];
    let right = vec![200i16; 16000];

    group.bench_function("mix_buffers", |b| {
        b.iter(|| {
            let mut mixed = Vec::with_capacity(left.len());
            for (l, r) in left.iter().zip(right.iter()) {
                // Simple mix with clipping prevention
                let mixed_sample = (*l as i32 + *r as i32) / 2;
                mixed.push(black_box(mixed_sample as i16));
            }
            mixed
        })
    });

    group.finish();
}

/// Benchmark voice feature extraction (simulated)
pub fn bench_voice_features(c: &mut Criterion) {
    let mut group = c.benchmark_group("voice_features");

    // Benchmark MFCC simulation
    for frame_size in [256, 512, 1024, 2048].iter() {
        let frame = vec![0.5f32; *frame_size];

        group.bench_with_input(
            BenchmarkId::new("extract_features", frame_size),
            &frame,
            |b, frame| {
                b.iter(|| {
                    // Simulate MFCC feature extraction
                    let mut features = Vec::with_capacity(13);

                    // Simulate FFT magnitudes (simplified)
                    let mut magnitudes = Vec::with_capacity(frame.len() / 2);
                    for i in 0..frame.len() / 2 {
                        let real = frame[i];
                        let imag = frame.get(i + frame.len() / 2).copied().unwrap_or(0.0);
                        let mag = (real * real + imag * imag).sqrt();
                        magnitudes.push(black_box(mag));
                    }

                    // Simulate Mel filterbank
                    for _ in 0..13 {
                        features.push(0.1);
                    }

                    features
                })
            },
        );
    }

    // Benchmark energy calculation
    group.bench_function("calculate_energy", |b| {
        let samples = vec![100i16; 16000];
        b.iter(|| {
            let energy: f64 = samples
                .iter()
                .map(|&s| (s as f64).powi(2))
                .sum();
            black_box(energy)
        })
    });

    // Benchmark zero-crossing rate
    group.bench_function("zero_crossing_rate", |b| {
        let samples = vec![100i16; 16000];
        b.iter(|| {
            let mut crossings = 0;
            for i in 1..samples.len() {
                if (samples[i] >= 0) != (samples[i - 1] >= 0) {
                    crossings += 1;
                }
            }
            let rate = crossings as f64 / samples.len() as f64;
            black_box(rate)
        })
    });

    group.finish();
}

/// Benchmark concurrent voice operations
pub fn bench_voice_concurrent(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().expect("operation failed");

    let mut group = c.benchmark_group("voice_concurrent");

    // Benchmark concurrent STT
    for concurrent_count in [1, 2, 5, 10].iter() {
        group.bench_with_input(
            BenchmarkId::new("concurrent_stt", concurrent_count),
            concurrent_count,
            |b, _| {
                b.to_async(&rt).iter(|| async {
                    let mut handles = Vec::new();
                    for i in 0..*concurrent_count {
                        let stt = SttEngine::new(&format!("stt-{}", i));
                        let audio = AudioBuffer::new(1000, 16000, 1);
                        handles.push(tokio::spawn(async move {
                            stt.transcribe(&audio).await
                        }));
                    }
                    for handle in handles {
                        handle.await.expect("operation failed");
                    }
                })
            },
        );
    }

    // Benchmark concurrent TTS
    for concurrent_count in [1, 2, 5, 10].iter() {
        group.bench_with_input(
            BenchmarkId::new("concurrent_tts", concurrent_count),
            concurrent_count,
            |b, _| {
                b.to_async(&rt).iter(|| async {
                    let mut handles = Vec::new();
                    for i in 0..*concurrent_count {
                        let tts = TtsEngine::new(&format!("tts-{}", i));
                        let text = format!("Hello world {}", i);
                        handles.push(tokio::spawn(async move {
                            tts.synthesize(&text, 16000).await
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

// Register all benchmark groups
criterion_group!(
    voice_benches,
    bench_stt_transcription,
    bench_tts_synthesis,
    bench_audio_processing,
    bench_audio_buffer_ops,
    bench_voice_features,
    bench_voice_concurrent,
);
