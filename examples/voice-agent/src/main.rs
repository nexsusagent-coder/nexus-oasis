//! Voice Agent Example
//!
//! A voice-enabled agent that can listen and speak.
//!
//! # Features
//! - Wake word detection ("Hey SENTIENT")
//! - Speech-to-Text (Whisper)
//! - Text-to-Speech (OpenAI TTS)
//!
//! # Usage
//! ```bash
//! cargo run --example voice-agent
//! ```
//!
//! # Requirements
//! - Microphone
//! - Speakers
//! - OPENAI_API_KEY environment variable

use sentient_core::{Agent, AgentConfig, Message, LlmProvider};
use sentient_voice::{VoiceConfig, SttProvider, TtsProvider, VoiceEngine};
use sentient_wake::{WakeWordDetector, WakeWordConfig, WakeWordProvider};
use std::env;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    println!("🎤 SENTIENT Voice Agent Example\n");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let api_key = env::var("OPENAI_API_KEY")
        .expect("Please set OPENAI_API_KEY environment variable");

    // Create voice engine
    println!("📦 Initializing voice engine...");
    let voice_config = VoiceConfig {
        stt_provider: SttProvider::WhisperApi,
        tts_provider: TtsProvider::OpenAI,
        whisper_api_key: Some(api_key.clone()),
        openai_tts_api_key: Some(api_key.clone()),
        tts_voice: "alloy".to_string(),
        ..Default::default()
    };
    let mut voice_engine = VoiceEngine::new(voice_config).await?;
    println!("✅ Voice engine ready (Whisper STT + OpenAI TTS)\n");

    // Create wake word detector
    println!("📦 Initializing wake word detector...");
    let wake_config = WakeWordConfig {
        provider: WakeWordProvider::Whisper,
        keyword: "hey sentient".to_string(),
        sensitivity: 0.5,
        ..Default::default()
    };
    let mut wake_detector = WakeWordDetector::new(wake_config)?;
    println!("✅ Wake word detector ready (\"Hey SENTIENT\")\n");

    // Create agent
    println!("📦 Creating AI agent...");
    let agent_config = AgentConfig {
        name: "VoiceAgent".to_string(),
        llm_provider: LlmProvider::OpenAI,
        llm_model: "gpt-4o".to_string(),
        api_key: api_key.clone(),
        system_prompt: Some(
            "You are a helpful voice assistant. Keep responses concise and natural \
             for spoken conversation. Maximum 2-3 sentences per response.".to_string()
        ),
        ..Default::default()
    };
    let mut agent = Agent::new(agent_config).await?;
    println!("✅ Agent ready (GPT-4o)\n");

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("🎤 Voice Agent Ready!");
    println!("   Say \"Hey SENTIENT\" to activate");
    println!("   Press Ctrl+C to exit");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // Main voice loop
    loop {
        // Wait for wake word
        print!("🔇 Listening for wake word... ");
        io::stdout().flush()?;

        wake_detector.wait_for_wake_word().await?;
        println!("✓");

        // Play activation sound
        voice_engine.play_chime().await?;

        println!("🎤 Listening... (speak now)");

        // Record and transcribe
        let transcript = voice_engine.listen_and_transcribe().await?;

        if transcript.trim().is_empty() {
            println!("🔇 No speech detected, continuing...\n");
            continue;
        }

        println!("👤 You: {}", transcript);

        // Check for exit commands
        let lower = transcript.to_lowercase();
        if lower.contains("goodbye") || lower.contains("exit") || lower.contains("stop") {
            println!("\n👋 Goodbye!");
            voice_engine.speak("Goodbye! Have a great day!").await?;
            break;
        }

        // Process with agent
        print!("🤖 Thinking... ");
        io::stdout().flush()?;

        let response = agent
            .send(Message::user(&transcript))
            .await?;

        println!("{}", response.content);

        // Speak the response
        voice_engine.speak(&response.content).await?;
        println!();
    }

    Ok(())
}
