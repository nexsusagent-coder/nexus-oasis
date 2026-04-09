# SENTIENT AI - Voice Guide

Complete guide to voice features in SENTIENT AI.

---

## Overview

SENTIENT provides comprehensive voice capabilities:

- **Speech-to-Text (STT)**: OpenAI Whisper API or local Whisper
- **Text-to-Speech (TTS)**: OpenAI, ElevenLabs, or System TTS
- **Wake Word Detection**: Porcupine, Vosk, or Whisper
- **Real-time Streaming**: Continuous voice interaction

---

## Quick Start

### Enable Voice

```bash
sentient voice enable
sentient voice start
```

Say "Hey SENTIENT" to activate voice interaction!

---

## Speech-to-Text (STT)

### OpenAI Whisper API

Default option with highest accuracy.

```bash
# Configure
sentient config set voice.stt_provider whisper-api

# Transcribe file
sentient voice transcribe recording.mp3

# Real-time transcription
sentient voice transcribe --realtime
```

```rust
use sentient_voice::{WhisperSTT, SttConfig};

let stt = WhisperSTT::new(SttConfig {
    model: "whisper-1",
    language: Some("tr"),
    ..Default::default()
}).await?;

let text = stt.transcribe("audio.mp3").await?;
```

### Local Whisper

For offline use and privacy.

```bash
# Install local model
sentient voice model download whisper-base

# Use local model
sentient config set voice.stt_provider whisper-local
sentient config set voice.whisper_model whisper-base
```

```rust
let stt = WhisperSTT::new(SttConfig {
    model: "local:whisper-base",  // or whisper-small, whisper-medium
    language: Some("tr"),
    ..Default::default()
}).await?;
```

### Supported Models

| Model | Size | Accuracy | Speed | Use Case |
|-------|------|----------|-------|----------|
| `whisper-tiny` | 39MB | ⭐⭐ | ⭐⭐⭐⭐⭐ | Quick tests |
| `whisper-base` | 74MB | ⭐⭐⭐ | ⭐⭐⭐⭐ | General use |
| `whisper-small` | 244MB | ⭐⭐⭐⭐ | ⭐⭐⭐ | Production |
| `whisper-medium` | 769MB | ⭐⭐⭐⭐⭐ | ⭐⭐ | High accuracy |
| `whisper-large` | 1.5GB | ⭐⭐⭐⭐⭐ | ⭐ | Best quality |

---

## Text-to-Speech (TTS)

### OpenAI TTS

High-quality, natural-sounding voices.

```bash
# Configure
sentient config set voice.tts_provider openai

# Synthesize
sentient voice speak "Hello world!" --output hello.mp3

# Stream to speakers
sentient voice speak "Hello world!" --play
```

```rust
use sentient_voice::{OpenAITTS, TtsConfig};

let tts = OpenAITTS::new(TtsConfig {
    voice: "alloy",
    model: "tts-1",
    speed: 1.0,
}).await?;

let audio = tts.synthesize("Hello world!").await?;
```

### Available Voices

| Voice | Description |
|-------|-------------|
| `alloy` | Neutral, balanced |
| `echo` | Male, warm |
| `fable` | British accent |
| `onyx` | Deep, male |
| `nova` | Female, clear |
| `shimmer` | Female, soft |

### ElevenLabs TTS

Premium quality with custom voices.

```bash
# Configure
sentient config set voice.tts_provider elevenlabs
sentient config set voice.elevenlabs_key "xi-..."

# List voices
sentient voice elevenlabs voices

# Use specific voice
sentient voice speak "Hello!" --voice "Rachel"
```

```rust
use sentient_voice::{ElevenLabsTTS, TtsConfig};

let tts = ElevenLabsTTS::new(TtsConfig {
    api_key: "xi-...",
    voice_id: "21m00Tcm4TlvDq8ikWAM",  // Rachel
    ..Default::default()
}).await?;
```

### System TTS

Use OS built-in TTS (free, no API key needed).

```bash
sentient config set voice.tts_provider system
sentient voice speak "Hello!" --play
```

---

## Wake Word Detection

### Porcupine (Recommended)

High accuracy, low CPU usage.

```bash
# Configure
sentient config set voice.wake_engine porcupine
sentient config set voice.porcupine_key "YOUR_KEY"

# Custom wake word
sentient config set voice.wake_word "hey sentient"
```

```rust
use sentient_wake::{WakeWordDetector, WakeConfig, Engine};

let detector = WakeWordDetector::new(WakeConfig {
    engine: Engine::Porcupine,
    access_key: "YOUR_PORCUPINE_KEY",
    keyword: "hey-sentient",  // or custom
    sensitivity: 0.5,
}).await?;

detector.start(|_| async move {
    println!("Wake word detected!");
    start_listening().await;
}).await?;
```

### Vosk (Offline)

Completely offline, no API key needed.

```bash
# Download model
sentient voice model download vosk-small

# Configure
sentient config set voice.wake_engine vosk
```

```rust
let detector = WakeWordDetector::new(WakeConfig {
    engine: Engine::Vosk,
    model_path: "./vosk-model-small",
    keyword: "hey sentient",
}).await?;
```

### Whisper Wake Word

Uses Whisper for detection (slower but flexible).

```rust
let detector = WakeWordDetector::new(WakeConfig {
    engine: Engine::Whisper,
    model: "whisper-base",
    keyword: "hey sentient",
}).await?;
```

---

## Real-Time Voice Mode

### Start Voice Session

```bash
sentient voice chat
```

### How It Works

```
┌─────────────────────────────────────────┐
│  1. Wait for wake word "Hey SENTIENT"   │
│  2. Record user speech                  │
│  3. Transcribe to text                  │
│  4. Send to LLM                         │
│  5. Synthesize response                 │
│  6. Play audio response                 │
│  7. Return to step 1                    │
└─────────────────────────────────────────┘
```

### Configuration

```toml
[voice]
enabled = true
wake_word = "hey sentient"
wake_engine = "porcupine"
stt_provider = "whisper-api"
tts_provider = "openai"
voice = "alloy"
silence_threshold = 0.5  # seconds of silence to stop recording
max_recording_time = 30  # seconds
```

---

## Streaming

### Streaming STT

```rust
let mut stream = stt.transcribe_stream(microphone_stream).await?;

while let Some(segment) = stream.next().await {
    print!("{}", segment.text);
    if segment.is_final {
        println!();
    }
}
```

### Streaming TTS

```rust
let mut stream = tts.synthesize_stream(long_text).await?;

while let Some(chunk) = stream.next().await {
    speaker.play(chunk).await?;
}
```

---

## Voice Channels

### Telegram Voice

```bash
# Enable voice messages
sentient channel telegram voice enable

# Voice message handling
sentient channel telegram voice auto-transcribe
```

### Discord Voice

```bash
# Join voice channel
sentient channel discord voice join #general-voice

# Enable wake word in Discord
sentient channel discord voice wake-word on
```

---

## Audio Processing

### Supported Formats

- MP3
- WAV
- FLAC
- M4A
- OGG
- WebM

### Audio Settings

```toml
[voice.audio]
sample_rate = 16000
channels = 1
bit_depth = 16
format = "wav"
```

---

## Troubleshooting

### No Audio Input

```bash
# Check microphone
sentient voice test mic

# List devices
sentient voice devices
```

### Poor Transcription

```bash
# Use better model
sentient config set voice.whisper_model whisper-medium

# Specify language
sentient config set voice.language tr
```

### Wake Word Not Working

```bash
# Adjust sensitivity
sentient config set voice.wake_sensitivity 0.7

# Test wake word
sentient voice test wake
```

### High Latency

```bash
# Use local Whisper for faster STT
sentient config set voice.stt_provider whisper-local
sentient config set voice.whisper_model whisper-base

# Use smaller TTS model
sentient config set voice.tts_model tts-1-hd
```

---

## Performance

| Configuration | Latency | CPU | RAM |
|---------------|---------|-----|-----|
| API (default) | ~1s | 5% | 100MB |
| Local (base) | ~500ms | 20% | 500MB |
| Local (medium) | ~1.5s | 50% | 1.5GB |
| Local (large) | ~3s | 80% | 3GB |

---

## API Reference

### HTTP API

```bash
# Transcribe
POST /v1/voice/transcribe
Content-Type: multipart/form-data
file: audio.mp3

# Synthesize
POST /v1/voice/synthesize
{
  "text": "Hello world!",
  "voice": "alloy",
  "format": "mp3"
}

# Stream synthesize
GET /v1/voice/synthesize/stream?text=Hello&voice=alloy
```

---

**Voice your AI with SENTIENT! 🎤**
