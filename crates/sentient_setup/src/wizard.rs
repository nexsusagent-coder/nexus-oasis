//! Setup Wizard - Professional Interactive TUI v7.0.0
//! OpenClaw Standard: Professional Model Selection Interface
//! 
//! Features:
//! - Security Warning (Multi-user lockdown)
//! - QuickStart vs Manual mode selection
//! - 100+ LLM Models in provider/model_id format
//! - 20+ Communication Channels with security policies
//! - Web Search Tools (SearXNG, DuckDuckGo, Ollama)
//! - Skip option on all steps

use dialoguer::{Input, Select, Confirm, Password, MultiSelect, FuzzySelect};
use console::{style, Term};
use indicatif::{ProgressBar, ProgressStyle};
use std::collections::HashMap;

use crate::{SetupStatus, SetupResult, SetupConfig, IntegrationConfig};

/// Skip selection result
#[derive(Debug, Clone, Copy, PartialEq)]
enum StepResult {
    Completed,
    Skipped,
}

pub struct SetupWizard {
    config: SetupConfig,
    step: u8,
    total_steps: u8,
    term: Term,
    /// Track if setup was aborted
    aborted: bool,
}

impl SetupWizard {
    pub fn new() -> Self {
        Self {
            config: SetupConfig::default(),
            step: 0,
            total_steps: 6,
            term: Term::stdout(),
            aborted: false,
        }
    }
    
    /// Run setup wizard - Professional TUI
    pub async fn run(&mut self) -> anyhow::Result<SetupResult> {
        self.print_welcome();
        
        // STEP 0: Security Warning
        if !self.show_security_warning()? {
            self.print_goodbye();
            return Ok(SetupResult {
                status: SetupStatus::NotStarted,
                config_path: self.config.config_path.clone(),
                integrations_enabled: vec![],
                warnings: vec!["Setup aborted by user".to_string()],
            });
        }
        
        // STEP 1: Setup Mode Selection (QuickStart vs Manual)
        let mode = self.select_setup_mode()?;
        
        match mode {
            SetupMode::QuickStart => self.quick_start_setup().await?,
            SetupMode::Manual => self.manual_setup().await?,
            SetupMode::Exit => {
                self.print_goodbye();
                return Ok(SetupResult {
                    status: SetupStatus::NotStarted,
                    config_path: self.config.config_path.clone(),
                    integrations_enabled: vec![],
                    warnings: vec![],
                });
            }
        }
        
        if self.aborted {
            return Ok(SetupResult {
                status: SetupStatus::NotStarted,
                config_path: self.config.config_path.clone(),
                integrations_enabled: vec![],
                warnings: vec!["Setup cancelled".to_string()],
            });
        }
        
        Ok(SetupResult {
            status: SetupStatus::Completed,
            config_path: self.config.config_path.clone(),
            integrations_enabled: self.get_enabled_integrations(),
            warnings: vec![],
        })
    }
    
    // ═══════════════════════════════════════════════════════════════════════════════
    // STEP 0: Security Warning
    // ═══════════════════════════════════════════════════════════════════════════════
    
    fn show_security_warning(&self) -> anyhow::Result<bool> {
        println!();
        println!("{}", style("╔════════════════════════════════════════════════════════════════════════════════╗").red());
        println!("{}", style("║                                                                                ║").red());
        println!("{}", style("║   SECURITY WARNING                                                            ║").red());
        println!("{}", style("║                                                                                ║").red());
        println!("{}", style("╚════════════════════════════════════════════════════════════════════════════════╝").red());
        println!();
        
        println!("{}", style("This system is PERSONAL by default.").bold().yellow());
        println!();
        println!("{}", style("Multi-user access requires LOCK-DOWN mode.").red());
        println!();
        println!("   - Personal mode: Full access to all data");
        println!("   - Lock-down mode: Restricted access, audit log enabled");
        println!();
        
        let confirm = Confirm::new()
            .with_prompt("Do you want to continue?")
            .default(true)
            .interact()?;
        
        if confirm {
            println!();
            println!("[OK] Security warning accepted. Continuing...");
        }
        
        Ok(confirm)
    }
    
    // ═══════════════════════════════════════════════════════════════════════════════
    // STEP 1: Setup Mode Selection
    // ═══════════════════════════════════════════════════════════════════════════════
    
    fn select_setup_mode(&mut self) -> anyhow::Result<SetupMode> {
        self.print_step("Setup Mode Selection");
        
        println!();
        println!("{}", style("Select setup mode:").bold().cyan());
        println!();
        println!("   QuickStart    - Fast setup");
        println!("                   Port: 18789, Loopback, Token Auth");
        println!("                   Recommended for first-time setup");
        println!();
        println!("   Manual        - Full control");
        println!("                   Customize all settings");
        println!("                   Recommended for experienced users");
        println!();
        
        let options = vec![
            "QuickStart (Recommended)",
            "Manual - Full Configuration",
            "Cancel",
        ];
        
        let selection = Select::new()
            .with_prompt("Your selection")
            .items(&options)
            .default(0)
            .interact()?;
        
        Ok(match selection {
            0 => SetupMode::QuickStart,
            1 => SetupMode::Manual,
            _ => SetupMode::Exit,
        })
    }
    
    // ═══════════════════════════════════════════════════════════════════════════════
    // QuickStart Setup
    // ═══════════════════════════════════════════════════════════════════════════════
    
    async fn quick_start_setup(&mut self) -> anyhow::Result<()> {
        self.total_steps = 6;
        
        // Set QuickStart defaults
        self.config.dashboard.port = 18789;
        self.config.dashboard.host = "127.0.0.1".to_string(); // Loopback only
        self.config.permissions.require_confirmation = true; // Token auth
        
        // STEP 1: Asistan İsmi
        self.step = 1;
        self.print_step("Asistan İsmi & Kişilik");
        if self.configure_assistant_identity()? == StepResult::Skipped {
            println!("  [SKIP] Varsayılan: SENTIENT (professional)");
        }
        
        // STEP 2: Dil Seçimi
        self.step = 2;
        self.print_step("Dil Seçimi");
        if self.select_language()? == StepResult::Skipped {
            self.config.language = "tr".to_string();
        }
        
        // STEP 3: LLM Provider (100+)
        self.step = 3;
        self.print_step("LLM Provider Selection");
        if self.configure_llm_provider()? == StepResult::Skipped {
            println!("  [SKIP] LLM configuration skipped");
        }
        
        // STEP 4: Communication Channels (20+)
        self.step = 4;
        self.print_step("Communication Channels");
        if self.configure_communication_channels()? == StepResult::Skipped {
            println!("  [SKIP] Channel configuration skipped");
        }
        
        // STEP 5: Tools (Web Search)
        self.step = 5;
        self.print_step("Tools");
        if self.configure_tools()? == StepResult::Skipped {
            println!("  [SKIP] Tools configuration skipped");
        }
        
        // STEP 6: Save
        self.step = 6;
        self.print_step("Saving Configuration");
        self.save_and_show_success()?;
        
        Ok(())
    }
    
    // ═══════════════════════════════════════════════════════════════════════════════
    // Manual Setup
    // ═══════════════════════════════════════════════════════════════════════════════
    
    async fn manual_setup(&mut self) -> anyhow::Result<()> {
        self.total_steps = 8;
        
        // STEP 1: Asistan İsmi & Kişilik
        self.step = 1;
        self.print_step("Asistan İsmi & Kişilik");
        if self.configure_assistant_identity()? == StepResult::Skipped {
            println!("  [SKIP] Varsayılan: SENTIENT (professional)");
        }
        
        // STEP 2: Language Selection
        self.step = 2;
        self.print_step("Language");
        if self.select_language()? == StepResult::Skipped {
            self.config.language = "en".to_string();
        }
        
        // STEP 3: LLM Provider (100+)
        self.step = 3;
        self.print_step("LLM Provider");
        if self.configure_llm_provider()? == StepResult::Skipped {
            println!("  [SKIP] LLM configuration skipped");
        }
        
        // STEP 4: Communication Channels (20+)
        self.step = 4;
        self.print_step("Communication Channels");
        if self.configure_communication_channels()? == StepResult::Skipped {
            println!("  [SKIP] Channel configuration skipped");
        }
        
        // STEP 5: Tools
        self.step = 5;
        self.print_step("Tools");
        if self.configure_tools()? == StepResult::Skipped {
            println!("  [SKIP] Tools configuration skipped");
        }
        
        // STEP 6: Permissions
        self.step = 6;
        self.print_step("Permissions");
        if self.configure_permissions()? == StepResult::Skipped {
            println!("  [SKIP] Permissions configuration skipped");
        }
        
        // STEP 7: Voice / Wake Word
        self.step = 7;
        self.print_step("Sesli Asistan");
        if self.configure_voice()? == StepResult::Skipped {
            println!("  [SKIP] Sesli asistan devre dışı");
        }
        
        // STEP 8: Save
        self.step = 8;
        self.print_step("Saving");
        self.save_and_show_success()?;
        
        Ok(())
    }
    
    // ═══════════════════════════════════════════════════════════════════════════════
    // LLM Provider Configuration (100+ Models) - OpenClaw Standard
    // ═══════════════════════════════════════════════════════════════════════════════
    
    fn configure_llm_provider(&mut self) -> anyhow::Result<StepResult> {
        println!();
        println!("{}", style("LLM Model Selection - OpenClaw Standard Format").bold().cyan());
        println!("{}", style("   Format: provider/model_id | Fuzzy search enabled").dim());
        println!();
        
        // ═══════════════════════════════════════════════════════════════════════════════
        // STEP 1: Use Case Selection - Model Recommendation System
        // ═══════════════════════════════════════════════════════════════════════════════
        
        let use_case = self.select_use_case()?;
        
        // Show recommendations based on use case
        if use_case != "all" {
            self.show_model_recommendations(&use_case)?;
        }
        
        // 100+ Models - Professional provider/model format (NO emojis, NO subjective descriptions)
        let providers: Vec<(&str, &str, &str)> = vec![
            // === ANTHROPIC ===
            ("anthropic", "claude-3-5-sonnet-20241022", "anthropic/claude-3-5-sonnet-20241022"),
            ("anthropic", "claude-3-5-haiku-20241022", "anthropic/claude-3-5-haiku-20241022"),
            ("anthropic", "claude-3-opus-20240229", "anthropic/claude-3-opus-20240229"),
            ("anthropic", "claude-3-sonnet-20240229", "anthropic/claude-3-sonnet-20240229"),
            ("anthropic", "claude-3-haiku-20240307", "anthropic/claude-3-haiku-20240307"),
            ("anthropic", "claude-2.1", "anthropic/claude-2.1"),
            ("anthropic", "claude-2.0", "anthropic/claude-2.0"),
            ("anthropic", "claude-instant-1.2", "anthropic/claude-instant-1.2"),
            
            // === OPENAI ===
            ("openai", "gpt-4o", "openai/gpt-4o"),
            ("openai", "gpt-4o-mini", "openai/gpt-4o-mini"),
            ("openai", "gpt-4-turbo", "openai/gpt-4-turbo"),
            ("openai", "gpt-4-turbo-preview", "openai/gpt-4-turbo-preview"),
            ("openai", "gpt-4", "openai/gpt-4"),
            ("openai", "gpt-4-32k", "openai/gpt-4-32k"),
            ("openai", "gpt-4-vision-preview", "openai/gpt-4-vision-preview"),
            ("openai", "gpt-3.5-turbo", "openai/gpt-3.5-turbo"),
            ("openai", "gpt-3.5-turbo-16k", "openai/gpt-3.5-turbo-16k"),
            ("openai", "gpt-3.5-turbo-instruct", "openai/gpt-3.5-turbo-instruct"),
            ("openai", "o1", "openai/o1"),
            ("openai", "o1-preview", "openai/o1-preview"),
            ("openai", "o1-mini", "openai/o1-mini"),
            ("openai", "chatgpt-4o-latest", "openai/chatgpt-4o-latest"),
            ("openai", "gpt-4o-realtime-preview", "openai/gpt-4o-realtime-preview"),
            ("openai", "gpt-4o-audio-preview", "openai/gpt-4o-audio-preview"),
            
            // === GOOGLE ===
            ("google", "gemini-2.0-flash", "google/gemini-2.0-flash"),
            ("google", "gemini-2.0-flash-lite", "google/gemini-2.0-flash-lite"),
            ("google", "gemini-2.0-pro-exp", "google/gemini-2.0-pro-exp"),
            ("google", "gemini-1.5-pro", "google/gemini-1.5-pro"),
            ("google", "gemini-1.5-flash", "google/gemini-1.5-flash"),
            ("google", "gemini-1.5-flash-8b", "google/gemini-1.5-flash-8b"),
            ("google", "gemini-1.5-pro-latest", "google/gemini-1.5-pro-latest"),
            ("google", "gemini-1.5-flash-latest", "google/gemini-1.5-flash-latest"),
            ("google", "gemini-pro", "google/gemini-pro"),
            ("google", "gemini-pro-vision", "google/gemini-pro-vision"),
            ("google", "gemma-2-27b-it", "google/gemma-2-27b-it"),
            ("google", "gemma-2-9b-it", "google/gemma-2-9b-it"),
            
            // === OPENROUTER - Anthropic ===
            ("openrouter", "anthropic/claude-3.5-sonnet", "openrouter/anthropic/claude-3.5-sonnet"),
            ("openrouter", "anthropic/claude-3.5-haiku", "openrouter/anthropic/claude-3.5-haiku"),
            ("openrouter", "anthropic/claude-3-opus", "openrouter/anthropic/claude-3-opus"),
            ("openrouter", "anthropic/claude-3-haiku", "openrouter/anthropic/claude-3-haiku"),
            ("openrouter", "anthropic/claude-3-sonnet", "openrouter/anthropic/claude-3-sonnet"),
            ("openrouter", "anthropic/claude-2.1", "openrouter/anthropic/claude-2.1"),
            
            // === OPENROUTER - OpenAI ===
            ("openrouter", "openai/gpt-4o", "openrouter/openai/gpt-4o"),
            ("openrouter", "openai/gpt-4o-mini", "openrouter/openai/gpt-4o-mini"),
            ("openrouter", "openai/gpt-4-turbo", "openrouter/openai/gpt-4-turbo"),
            ("openrouter", "openai/gpt-4", "openrouter/openai/gpt-4"),
            ("openrouter", "openai/o1-preview", "openrouter/openai/o1-preview"),
            ("openrouter", "openai/o1-mini", "openrouter/openai/o1-mini"),
            ("openrouter", "openai/gpt-3.5-turbo", "openrouter/openai/gpt-3.5-turbo"),
            ("openrouter", "openai/chatgpt-4o-latest", "openrouter/openai/chatgpt-4o-latest"),
            
            // === OPENROUTER - Google ===
            ("openrouter", "google/gemini-2.0-flash-exp", "openrouter/google/gemini-2.0-flash-exp"),
            ("openrouter", "google/gemini-pro", "openrouter/google/gemini-pro"),
            ("openrouter", "google/gemini-1.5-pro", "openrouter/google/gemini-1.5-pro"),
            ("openrouter", "google/gemini-1.5-flash", "openrouter/google/gemini-1.5-flash"),
            ("openrouter", "google/gemma-2-27b-it", "openrouter/google/gemma-2-27b-it"),
            ("openrouter", "google/palm-2-chat-bison", "openrouter/google/palm-2-chat-bison"),
            
            // === OPENROUTER - Meta Llama ===
            ("openrouter", "meta-llama/llama-3.3-70b-instruct", "openrouter/meta-llama/llama-3.3-70b-instruct"),
            ("openrouter", "meta-llama/llama-3.2-90b-vision-instruct", "openrouter/meta-llama/llama-3.2-90b-vision-instruct"),
            ("openrouter", "meta-llama/llama-3.2-11b-vision-instruct", "openrouter/meta-llama/llama-3.2-11b-vision-instruct"),
            ("openrouter", "meta-llama/llama-3.2-3b-instruct", "openrouter/meta-llama/llama-3.2-3b-instruct"),
            ("openrouter", "meta-llama/llama-3.2-1b-instruct", "openrouter/meta-llama/llama-3.2-1b-instruct"),
            ("openrouter", "meta-llama/llama-3.1-405b-instruct", "openrouter/meta-llama/llama-3.1-405b-instruct"),
            ("openrouter", "meta-llama/llama-3.1-70b-instruct", "openrouter/meta-llama/llama-3.1-70b-instruct"),
            ("openrouter", "meta-llama/llama-3.1-8b-instruct", "openrouter/meta-llama/llama-3.1-8b-instruct"),
            ("openrouter", "meta-llama/llama-3-70b-instruct", "openrouter/meta-llama/llama-3-70b-instruct"),
            ("openrouter", "meta-llama/llama-3-8b-instruct", "openrouter/meta-llama/llama-3-8b-instruct"),
            ("openrouter", "meta-llama/llama-2-70b-chat", "openrouter/meta-llama/llama-2-70b-chat"),
            ("openrouter", "meta-llama/llama-2-13b-chat", "openrouter/meta-llama/llama-2-13b-chat"),
            
            // === OPENROUTER - Mistral ===
            ("openrouter", "mistralai/mistral-large", "openrouter/mistralai/mistral-large"),
            ("openrouter", "mistralai/mistral-large-2407", "openrouter/mistralai/mistral-large-2407"),
            ("openrouter", "mistralai/mistral-medium", "openrouter/mistralai/mistral-medium"),
            ("openrouter", "mistralai/mistral-small", "openrouter/mistralai/mistral-small"),
            ("openrouter", "mistralai/codestral-mamba", "openrouter/mistralai/codestral-mamba"),
            ("openrouter", "mistralai/ministral-8b", "openrouter/mistralai/ministral-8b"),
            ("openrouter", "mistralai/ministral-3b", "openrouter/mistralai/ministral-3b"),
            ("openrouter", "mistralai/mixtral-8x22b-instruct", "openrouter/mistralai/mixtral-8x22b-instruct"),
            ("openrouter", "mistralai/mixtral-8x7b-instruct", "openrouter/mistralai/mixtral-8x7b-instruct"),
            ("openrouter", "mistralai/mistral-7b-instruct", "openrouter/mistralai/mistral-7b-instruct"),
            
            // === OPENROUTER - DeepSeek ===
            ("openrouter", "deepseek/deepseek-chat", "openrouter/deepseek/deepseek-chat"),
            ("openrouter", "deepseek/deepseek-coder", "openrouter/deepseek/deepseek-coder"),
            ("openrouter", "deepseek/deepseek-r1", "openrouter/deepseek/deepseek-r1"),
            ("openrouter", "deepseek/deepseek-r1-distill-llama-70b", "openrouter/deepseek/deepseek-r1-distill-llama-70b"),
            ("openrouter", "deepseek/deepseek-r1-distill-qwen-32b", "openrouter/deepseek/deepseek-r1-distill-qwen-32b"),
            
            // === OPENROUTER - Qwen ===
            ("openrouter", "qwen/qwen-2.5-72b-instruct", "openrouter/qwen/qwen-2.5-72b-instruct"),
            ("openrouter", "qwen/qwen-2.5-32b-instruct", "openrouter/qwen/qwen-2.5-32b-instruct"),
            ("openrouter", "qwen/qwen-2.5-14b-instruct", "openrouter/qwen/qwen-2.5-14b-instruct"),
            ("openrouter", "qwen/qwen-2.5-7b-instruct", "openrouter/qwen/qwen-2.5-7b-instruct"),
            ("openrouter", "qwen/qwen-2.5-coder-32b-instruct", "openrouter/qwen/qwen-2.5-coder-32b-instruct"),
            ("openrouter", "qwen/qwen-2.5-coder-7b-instruct", "openrouter/qwen/qwen-2.5-coder-7b-instruct"),
            ("openrouter", "qwen/qwen-2-72b-instruct", "openrouter/qwen/qwen-2-72b-instruct"),
            ("openrouter", "qwen/qwq-32b-preview", "openrouter/qwen/qwq-32b-preview"),
            ("openrouter", "qwen/qwen-2.5-vl-72b-instruct", "openrouter/qwen/qwen-2.5-vl-72b-instruct"),
            
            // === OPENROUTER - Other ===
            ("openrouter", "openrouter/auto", "openrouter/auto"),
            ("openrouter", "cohere/command-r-plus", "openrouter/cohere/command-r-plus"),
            ("openrouter", "cohere/command-r", "openrouter/cohere/command-r"),
            ("openrouter", "cohere/command", "openrouter/cohere/command"),
            ("openrouter", "perplexity/llama-3.1-sonar-small-128k-online", "openrouter/perplexity/llama-3.1-sonar-small-128k-online"),
            ("openrouter", "perplexity/llama-3.1-sonar-large-128k-online", "openrouter/perplexity/llama-3.1-sonar-large-128k-online"),
            ("openrouter", "perplexity/llama-3.1-sonar-huge-128k-online", "openrouter/perplexity/llama-3.1-sonar-huge-128k-online"),
            ("openrouter", "x-ai/grok-beta", "openrouter/x-ai/grok-beta"),
            ("openrouter", "x-ai/grok-2-1212", "openrouter/x-ai/grok-2-1212"),
            ("openrouter", "x-ai/grok-2-vision-1212", "openrouter/x-ai/grok-2-vision-1212"),
            ("openrouter", "microsoft/phi-4", "openrouter/microsoft/phi-4"),
            ("openrouter", "microsoft/phi-3-medium-128k-instruct", "openrouter/microsoft/phi-3-medium-128k-instruct"),
            ("openrouter", "microsoft/phi-3-mini-128k-instruct", "openrouter/microsoft/phi-3-mini-128k-instruct"),
            ("openrouter", "nousresearch/hermes-3-llama-3.1-405b", "openrouter/nousresearch/hermes-3-llama-3.1-405b"),
            ("openrouter", "nousresearch/hermes-3-llama-3.1-70b", "openrouter/nousresearch/hermes-3-llama-3.1-70b"),
            ("openrouter", "cognitivecomputations/dolphin-mixtral-8x22b", "openrouter/cognitivecomputations/dolphin-mixtral-8x22b"),
            ("openrouter", "allenai/olmo-7b-instruct", "openrouter/allenai/olmo-7b-instruct"),
            ("openrouter", "databricks/dbrx-instruct", "openrouter/databricks/dbrx-instruct"),
            ("openrouter", "inflection/inflection-3-pi", "openrouter/inflection/inflection-3-pi"),
            ("openrouter", "inflection/inflection-3-productivity", "openrouter/inflection/inflection-3-productivity"),
            ("openrouter", "sao10k/l3-euryale-70b", "openrouter/sao10k/l3-euryale-70b"),
            ("openrouter", "teknium/openhermes-2.5-mistral-7b", "openrouter/teknium/openhermes-2.5-mistral-7b"),
            
            // === OLLAMA ===
            ("ollama", "llama3.3:70b", "ollama/llama3.3:70b"),
            ("ollama", "llama3.2:3b", "ollama/llama3.2:3b"),
            ("ollama", "llama3.2:1b", "ollama/llama3.2:1b"),
            ("ollama", "llama3.1:8b", "ollama/llama3.1:8b"),
            ("ollama", "llama3.1:70b", "ollama/llama3.1:70b"),
            ("ollama", "llama3.1:405b", "ollama/llama3.1:405b"),
            ("ollama", "llama3:70b", "ollama/llama3:70b"),
            ("ollama", "llama3:8b", "ollama/llama3:8b"),
            ("ollama", "llama2:70b", "ollama/llama2:70b"),
            ("ollama", "llama2:13b", "ollama/llama2:13b"),
            ("ollama", "llama2:7b", "ollama/llama2:7b"),
            ("ollama", "qwen2.5:72b", "ollama/qwen2.5:72b"),
            ("ollama", "qwen2.5:32b", "ollama/qwen2.5:32b"),
            ("ollama", "qwen2.5:14b", "ollama/qwen2.5:14b"),
            ("ollama", "qwen2.5:7b", "ollama/qwen2.5:7b"),
            ("ollama", "qwen2.5-coder:32b", "ollama/qwen2.5-coder:32b"),
            ("ollama", "qwen2.5-coder:7b", "ollama/qwen2.5-coder:7b"),
            ("ollama", "qwen2:72b", "ollama/qwen2:72b"),
            ("ollama", "qwen2:7b", "ollama/qwen2:7b"),
            ("ollama", "deepseek-r1:671b", "ollama/deepseek-r1:671b"),
            ("ollama", "deepseek-r1:70b", "ollama/deepseek-r1:70b"),
            ("ollama", "deepseek-r1:32b", "ollama/deepseek-r1:32b"),
            ("ollama", "deepseek-r1:7b", "ollama/deepseek-r1:7b"),
            ("ollama", "deepseek-v2:236b", "ollama/deepseek-v2:236b"),
            ("ollama", "deepseek-coder-v2:236b", "ollama/deepseek-coder-v2:236b"),
            ("ollama", "mistral:7b", "ollama/mistral:7b"),
            ("ollama", "mistral-nemo:12b", "ollama/mistral-nemo:12b"),
            ("ollama", "mixtral:8x7b", "ollama/mixtral:8x7b"),
            ("ollama", "mixtral:8x22b", "ollama/mixtral:8x22b"),
            ("ollama", "codellama:34b", "ollama/codellama:34b"),
            ("ollama", "codellama:13b", "ollama/codellama:13b"),
            ("ollama", "codellama:7b", "ollama/codellama:7b"),
            // ═══════════════════════════════════════════════════════════════
            // GEMMA 4 - SENTIENT OS KERNEL (DEFAULT LOCAL)
            // ═══════════════════════════════════════════════════════════════
            ("ollama", "gemma4:31b", "ollama/gemma4:31b (KERNEL)"),
            ("ollama", "gemma4:26b-moe", "ollama/gemma4:26b-moe"),
            ("ollama", "gemma4:e4b", "ollama/gemma4:e4b (Edge)"),
            ("ollama", "gemma4:e2b", "ollama/gemma4:e2b (Mobile)"),
            // ═══════════════════════════════════════════════════════════════
            // GEMMA 2 - Previous Generation
            // ═══════════════════════════════════════════════════════════════
            ("ollama", "gemma2:27b", "ollama/gemma2:27b"),
            ("ollama", "gemma2:9b", "ollama/gemma2:9b"),
            ("ollama", "gemma:7b", "ollama/gemma:7b"),
            ("ollama", "phi4:14b", "ollama/phi4:14b"),
            ("ollama", "phi3.5:3.8b", "ollama/phi3.5:3.8b"),
            ("ollama", "phi3:14b", "ollama/phi3:14b"),
            ("ollama", "phi3:medium", "ollama/phi3:medium"),
            ("ollama", "command-r:35b", "ollama/command-r:35b"),
            ("ollama", "llava:13b", "ollama/llava:13b"),
            ("ollama", "llava:7b", "ollama/llava:7b"),
            ("ollama", "moondream:latest", "ollama/moondream:latest"),
            ("ollama", "nomic-embed-text:latest", "ollama/nomic-embed-text:latest"),
            ("ollama", "mxbai-embed-large:latest", "ollama/mxbai-embed-large:latest"),
            ("ollama", "starcoder2:7b", "ollama/starcoder2:7b"),
            ("ollama", "codeqwen:7b", "ollama/codeqwen:7b"),
            ("ollama", "dolphin-mixtral:8x7b", "ollama/dolphin-mixtral:8x7b"),
            ("ollama", "openchat:7b", "ollama/openchat:7b"),
            ("ollama", "wizardlm2:7b", "ollama/wizardlm2:7b"),
            
            // === GROQ ===
            ("groq", "llama-3.3-70b-versatile", "groq/llama-3.3-70b-versatile"),
            ("groq", "llama-3.3-70b-specdec", "groq/llama-3.3-70b-specdec"),
            ("groq", "llama-3.1-70b-versatile", "groq/llama-3.1-70b-versatile"),
            ("groq", "llama-3.1-8b-instant", "groq/llama-3.1-8b-instant"),
            ("groq", "llama-3.2-90b-vision-preview", "groq/llama-3.2-90b-vision-preview"),
            ("groq", "llama-3.2-11b-vision-preview", "groq/llama-3.2-11b-vision-preview"),
            ("groq", "llama-3.2-3b-preview", "groq/llama-3.2-3b-preview"),
            ("groq", "llama-3.2-1b-preview", "groq/llama-3.2-1b-preview"),
            ("groq", "mixtral-8x7b-32768", "groq/mixtral-8x7b-32768"),
            ("groq", "gemma2-9b-it", "groq/gemma2-9b-it"),
            
            // === DEEPSEEK ===
            ("deepseek", "deepseek-chat", "deepseek/deepseek-chat"),
            ("deepseek", "deepseek-coder", "deepseek/deepseek-coder"),
            ("deepseek", "deepseek-reasoner", "deepseek/deepseek-reasoner"),
            
            // === MISTRAL AI ===
            ("mistral", "mistral-large-latest", "mistral/mistral-large-latest"),
            ("mistral", "mistral-medium-latest", "mistral/mistral-medium-latest"),
            ("mistral", "mistral-small-latest", "mistral/mistral-small-latest"),
            ("mistral", "codestral-latest", "mistral/codestral-latest"),
            ("mistral", "ministral-8b-latest", "mistral/ministral-8b-latest"),
            ("mistral", "ministral-3b-latest", "mistral/ministral-3b-latest"),
            ("mistral", "open-mistral-nemo", "mistral/open-mistral-nemo"),
            ("mistral", "open-mixtral-8x22b", "mistral/open-mixtral-8x22b"),
            ("mistral", "open-mixtral-8x7b", "mistral/open-mixtral-8x7b"),
            ("mistral", "mistral-embed", "mistral/mistral-embed"),
            
            // === PERPLEXITY ===
            ("perplexity", "llama-3.1-sonar-small-128k-online", "perplexity/llama-3.1-sonar-small-128k-online"),
            ("perplexity", "llama-3.1-sonar-large-128k-online", "perplexity/llama-3.1-sonar-large-128k-online"),
            ("perplexity", "llama-3.1-sonar-huge-128k-online", "perplexity/llama-3.1-sonar-huge-128k-online"),
            ("perplexity", "llama-3.1-sonar-small-128k-chat", "perplexity/llama-3.1-sonar-small-128k-chat"),
            ("perplexity", "llama-3.1-sonar-large-128k-chat", "perplexity/llama-3.1-sonar-large-128k-chat"),
            
            // === COHERE ===
            ("cohere", "command-r-plus", "cohere/command-r-plus"),
            ("cohere", "command-r", "cohere/command-r"),
            ("cohere", "command", "cohere/command"),
            ("cohere", "command-light", "cohere/command-light"),
            ("cohere", "command-nightly", "cohere/command-nightly"),
            ("cohere", "rerank-english-v3.0", "cohere/rerank-english-v3.0"),
            ("cohere", "rerank-multilingual-v3.0", "cohere/rerank-multilingual-v3.0"),
            
            // === TOGETHER AI ===
            ("together", "meta-llama/Llama-3.3-70B-Instruct-Turbo", "together/meta-llama/Llama-3.3-70B-Instruct-Turbo"),
            ("together", "meta-llama/Llama-3.2-90B-Vision-Instruct-Turbo", "together/meta-llama/Llama-3.2-90B-Vision-Instruct-Turbo"),
            ("together", "meta-llama/Llama-3.2-11B-Vision-Instruct-Turbo", "together/meta-llama/Llama-3.2-11B-Vision-Instruct-Turbo"),
            ("together", "meta-llama/Llama-3.1-405B-Instruct-Turbo", "together/meta-llama/Llama-3.1-405B-Instruct-Turbo"),
            ("together", "mistralai/Mixtral-8x7B-Instruct-v0.1", "together/mistralai/Mixtral-8x7B-Instruct-v0.1"),
            ("together", "mistralai/Mixtral-8x22B-Instruct-v0.1", "together/mistralai/Mixtral-8x22B-Instruct-v0.1"),
            ("together", "Qwen/Qwen2.5-72B-Instruct-Turbo", "together/Qwen/Qwen2.5-72B-Instruct-Turbo"),
            ("together", "Qwen/Qwen2.5-Coder-32B-Instruct", "together/Qwen/Qwen2.5-Coder-32B-Instruct"),
            ("together", "deepseek-ai/DeepSeek-V3", "together/deepseek-ai/DeepSeek-V3"),
            ("together", "databricks/dbrx-instruct", "together/databricks/dbrx-instruct"),
            
            // === X.AI ===
            ("xai", "grok-beta", "xai/grok-beta"),
            ("xai", "grok-2-1212", "xai/grok-2-1212"),
            ("xai", "grok-2-vision-1212", "xai/grok-2-vision-1212"),
            ("xai", "grok-vision-beta", "xai/grok-vision-beta"),
            
            // === MOONSHOT ===
            ("moonshot", "moonshot-v1-8k", "moonshot/moonshot-v1-8k"),
            ("moonshot", "moonshot-v1-32k", "moonshot/moonshot-v1-32k"),
            ("moonshot", "moonshot-v1-128k", "moonshot/moonshot-v1-128k"),
            
            // === ZHIPU AI ===
            ("zhipu", "glm-4", "zhipu/glm-4"),
            ("zhipu", "glm-4-flash", "zhipu/glm-4-flash"),
            ("zhipu", "glm-4-plus", "zhipu/glm-4-plus"),
            ("zhipu", "glm-4-long", "zhipu/glm-4-long"),
            ("zhipu", "glm-4v-plus", "zhipu/glm-4v-plus"),
            ("zhipu", "glm-4v", "zhipu/glm-4v"),
            ("zhipu", "embedding-3", "zhipu/embedding-3"),
            
            // === BAIDU (ERNIE) ===
            ("baidu", "ernie-4.0-8k", "baidu/ernie-4.0-8k"),
            ("baidu", "ernie-4.0-turbo-8k", "baidu/ernie-4.0-turbo-8k"),
            ("baidu", "ernie-3.5-8k", "baidu/ernie-3.5-8k"),
            ("baidu", "ernie-speed-8k", "baidu/ernie-speed-8k"),
            
            // === ALIBABA (QWEN via DASHSCOPE) ===
            ("alibaba", "qwen-max", "alibaba/qwen-max"),
            ("alibaba", "qwen-plus", "alibaba/qwen-plus"),
            ("alibaba", "qwen-turbo", "alibaba/qwen-turbo"),
            ("alibaba", "qwen-vl-max", "alibaba/qwen-vl-max"),
            ("alibaba", "qwen-vl-plus", "alibaba/qwen-vl-plus"),
            
            // === REPLICATE ===
            ("replicate", "meta/llama-3.3-70b-instruct", "replicate/meta/llama-3.3-70b-instruct"),
            ("replicate", "meta/llama-3.1-405b-instruct", "replicate/meta/llama-3.1-405b-instruct"),
            ("replicate", "mistralai/mixtral-8x7b-instruct-v0.1", "replicate/mistralai/mixtral-8x7b-instruct-v0.1"),
            
            // === AZURE OPENAI ===
            ("azure", "gpt-4o", "azure/gpt-4o"),
            ("azure", "gpt-4o-mini", "azure/gpt-4o-mini"),
            ("azure", "gpt-4-turbo", "azure/gpt-4-turbo"),
            ("azure", "gpt-4", "azure/gpt-4"),
            ("azure", "gpt-4-32k", "azure/gpt-4-32k"),
            ("azure", "gpt-3.5-turbo", "azure/gpt-3.5-turbo"),
            ("azure", "gpt-3.5-turbo-16k", "azure/gpt-3.5-turbo-16k"),
            
            // === AWS BEDROCK ===
            ("bedrock", "anthropic.claude-3-5-sonnet-20241022-v2:0", "bedrock/anthropic.claude-3-5-sonnet"),
            ("bedrock", "anthropic.claude-3-opus-20240229-v1:0", "bedrock/anthropic.claude-3-opus"),
            ("bedrock", "anthropic.claude-3-haiku-20240307-v1:0", "bedrock/anthropic.claude-3-haiku"),
            ("bedrock", "anthropic.claude-3-sonnet-20240229-v1:0", "bedrock/anthropic.claude-3-sonnet"),
            ("bedrock", "meta.llama3-3-70b-instruct-v1:0", "bedrock/meta.llama3-3-70b"),
            ("bedrock", "meta.llama3-1-405b-instruct-v1:0", "bedrock/meta.llama3-1-405b"),
            ("bedrock", "meta.llama3-1-70b-instruct-v1:0", "bedrock/meta.llama3-1-70b"),
            ("bedrock", "meta.llama3-1-8b-instruct-v1:0", "bedrock/meta.llama3-1-8b"),
            ("bedrock", "mistral.mistral-large-2407-v1:0", "bedrock/mistral-large"),
            ("bedrock", "amazon.titan-text-premier-v1:0", "bedrock/amazon.titan-premier"),
            ("bedrock", "cohere.command-r-plus-v1:0", "bedrock/cohere.command-r-plus"),
            
            // === GOOGLE VERTEX ===
            ("vertex", "gemini-2.0-flash", "vertex/gemini-2.0-flash"),
            ("vertex", "gemini-2.0-flash-lite", "vertex/gemini-2.0-flash-lite"),
            ("vertex", "gemini-1.5-pro", "vertex/gemini-1.5-pro"),
            ("vertex", "gemini-1.5-flash", "vertex/gemini-1.5-flash"),
            ("vertex", "gemini-1.5-flash-8b", "vertex/gemini-1.5-flash-8b"),
            
            // === IBM WATSONX ===
            ("watsonx", "ibm/granite-13b-chat-v2", "watsonx/ibm/granite-13b-chat-v2"),
            ("watsonx", "ibm/granite-20b-code-instruct-v1", "watsonx/ibm/granite-20b-code-instruct-v1"),
            ("watsonx", "meta-llama/llama-3-70b-instruct", "watsonx/meta-llama/llama-3-70b-instruct"),
            
            // === CUSTOM ===

            // ═══════════════════════════════════════════════════════════════
            // HUGGINGFACE - Açık Kaynak Model Hub (ÜCRETSİZ TIER)
            // ═══════════════════════════════════════════════════════════════
            ("huggingface", "meta-llama/Llama-3.3-70B-Instruct", "huggingface/meta-llama/Llama-3.3-70B-Instruct"),
            ("huggingface", "meta-llama/Llama-3.1-405B-Instruct", "huggingface/meta-llama/Llama-3.1-405B-Instruct"),
            ("huggingface", "mistralai/Mistral-Large-Instruct-2407", "huggingface/mistralai/Mistral-Large-Instruct"),
            ("huggingface", "mistralai/Mixtral-8x22B-Instruct-v0.1", "huggingface/mistralai/Mixtral-8x22B-Instruct"),
            ("huggingface", "mistralai/Mixtral-8x7B-Instruct-v0.1", "huggingface/mistralai/Mixtral-8x7B-Instruct"),
            ("huggingface", "mistralai/Mistral-7B-Instruct-v0.3", "huggingface/mistralai/Mistral-7B-Instruct"),
            ("huggingface", "google/gemma-2-27b-it", "huggingface/google/gemma-2-27b-it"),
            ("huggingface", "google/gemma-2-9b-it", "huggingface/google/gemma-2-9b-it"),
            ("huggingface", "google/gemma-2-2b-it", "huggingface/google/gemma-2-2b-it"),
            ("huggingface", "Qwen/Qwen2.5-72B-Instruct", "huggingface/Qwen/Qwen2.5-72B-Instruct"),
            ("huggingface", "Qwen/Qwen2.5-32B-Instruct", "huggingface/Qwen/Qwen2.5-32B-Instruct"),
            ("huggingface", "Qwen/Qwen2.5-14B-Instruct", "huggingface/Qwen/Qwen2.5-14B-Instruct"),
            ("huggingface", "Qwen/Qwen2.5-7B-Instruct", "huggingface/Qwen/Qwen2.5-7B-Instruct"),
            ("huggingface", "Qwen/Qwen2.5-Coder-32B-Instruct", "huggingface/Qwen/Qwen2.5-Coder-32B-Instruct"),
            ("huggingface", "deepseek-ai/DeepSeek-V3", "huggingface/deepseek-ai/DeepSeek-V3"),
            ("huggingface", "deepseek-ai/DeepSeek-Coder-V2-Instruct", "huggingface/deepseek-ai/DeepSeek-Coder-V2"),
            ("huggingface", "microsoft/Phi-4", "huggingface/microsoft/Phi-4"),
            ("huggingface", "microsoft/Phi-3-medium-4k-instruct", "huggingface/microsoft/Phi-3-medium"),
            ("huggingface", "microsoft/Phi-3-mini-4k-instruct", "huggingface/microsoft/Phi-3-mini"),
            ("huggingface", "tiiuae/falcon-180B-chat", "huggingface/tiiuae/falcon-180B-chat"),
            ("huggingface", "tiiuae/falcon-40B-instruct", "huggingface/tiiuae/falcon-40B-instruct"),
            ("huggingface", "databricks/dbrx-instruct", "huggingface/databricks/dbrx-instruct"),
            ("huggingface", "allenai/OLMo-7B-Instruct", "huggingface/allenai/OLMo-7B-Instruct"),
            ("huggingface", "bigscience/bloom", "huggingface/bigscience/bloom"),
            ("huggingface", "openchat/openchat-3.5-1210", "huggingface/openchat/openchat-3.5"),
            ("huggingface", "NousResearch/Nous-Hermes-2-Mixtral-8x7B-DPO", "huggingface/Nous-Hermes-2-Mixtral"),
            
            // ═══════════════════════════════════════════════════════════════
            // FIREWORKS AI - Hızlı Inference (UCUZ)
            // ═══════════════════════════════════════════════════════════════
            ("fireworks", "accounts/fireworks/models/llama-v3p3-70b-instruct", "fireworks/llama-v3.3-70b"),
            ("fireworks", "accounts/fireworks/models/llama-v3p1-405b-instruct", "fireworks/llama-v3.1-405b"),
            ("fireworks", "accounts/fireworks/models/llama-v3p1-70b-instruct", "fireworks/llama-v3.1-70b"),
            ("fireworks", "accounts/fireworks/models/llama-v3p1-8b-instruct", "fireworks/llama-v3.1-8b"),
            ("fireworks", "accounts/fireworks/models/qwen2p5-72b-instruct", "fireworks/qwen2.5-72b"),
            ("fireworks", "accounts/fireworks/models/qwen2p5-coder-32b-instruct", "fireworks/qwen2.5-coder-32b"),
            ("fireworks", "accounts/fireworks/models/mixtral-8x7b-instruct", "fireworks/mixtral-8x7b"),
            ("fireworks", "accounts/fireworks/models/mixtral-8x22b-instruct", "fireworks/mixtral-8x22b"),
            ("fireworks", "accounts/fireworks/models/deepseek-v3", "fireworks/deepseek-v3"),
            ("fireworks", "accounts/fireworks/models/gemma2-27b-it", "fireworks/gemma2-27b"),
            
            // ═══════════════════════════════════════════════════════════════
            // NVIDIA NIM - Enterprise GPU Inference
            // ═══════════════════════════════════════════════════════════════
            ("nvidia", "meta/llama-3.3-70b-instruct", "nvidia/llama-3.3-70b"),
            ("nvidia", "meta/llama-3.1-405b-instruct", "nvidia/llama-3.1-405b"),
            ("nvidia", "meta/llama-3.1-70b-instruct", "nvidia/llama-3.1-70b"),
            ("nvidia", "meta/llama-3.1-8b-instruct", "nvidia/llama-3.1-8b"),
            ("nvidia", "mistralai/mistral-large", "nvidia/mistral-large"),
            ("nvidia", "mistralai/mixtral-8x7b-instruct-v0.1", "nvidia/mixtral-8x7b"),
            ("nvidia", "google/gemma-2-27b-it", "nvidia/gemma-2-27b"),
            ("nvidia", "google/gemma-2-9b-it", "nvidia/gemma-2-9b"),
            ("nvidia", "nvidia/nemotron-4-340b-instruct", "nvidia/nemotron-4-340b"),
            ("nvidia", "microsoft/phi-3-mini-4k-instruct", "nvidia/phi-3-mini"),
            
            // ═══════════════════════════════════════════════════════════════
            // NOVITA AI - Ucuz LLM API
            // ═══════════════════════════════════════════════════════════════
            ("novita", "meta-llama/llama-3.3-70b-instruct", "novita/llama-3.3-70b"),
            ("novita", "meta-llama/llama-3.1-70b-instruct", "novita/llama-3.1-70b"),
            ("novita", "meta-llama/llama-3.1-8b-instruct", "novita/llama-3.1-8b"),
            ("novita", "mistralai/mistral-7b-instruct", "novita/mistral-7b"),
            ("novita", "mistralai/mixtral-8x7b-instruct", "novita/mixtral-8x7b"),
            ("novita", "google/gemma-2-9b-it", "novita/gemma-2-9b"),
            ("novita", "qwen/qwen-2-7b-instruct", "novita/qwen-2-7b"),
            ("novita", "deepseek/deepseek-r1", "novita/deepseek-r1"),
            
            // ═══════════════════════════════════════════════════════════════
            // AI21 LABS - Jamba & Jurassic
            // ═══════════════════════════════════════════════════════════════
            ("ai21", "jamba-1-5-large", "ai21/jamba-1-5-large"),
            ("ai21", "jamba-1-5-mini", "ai21/jamba-1-5-mini"),
            ("ai21", "jamba-instruct", "ai21/jamba-instruct"),
            ("ai21", "jurassic-2-mid", "ai21/jurassic-2-mid"),
            ("ai21", "jurassic-2-large", "ai21/jurassic-2-large"),
            
            // ═══════════════════════════════════════════════════════════════
            // STABILITY AI - StableLM
            // ═══════════════════════════════════════════════════════════════
            ("stability", "stablelm-2-12b-chat", "stability/stablelm-2-12b-chat"),
            ("stability", "stablelm-2-7b-chat", "stability/stablelm-2-7b-chat"),
            ("stability", "stablelm-2-1-6b-chat", "stability/stablelm-2-1-6b-chat"),
            ("stability", "stablelm-zephyr-3b", "stability/stablelm-zephyr-3b"),
            ("stability", "stable-code-3b", "stability/stable-code-3b"),
            
            // ═══════════════════════════════════════════════════════════════
            // INFLECTION AI - Pi
            // ═══════════════════════════════════════════════════════════════
            ("inflection", "inflection-3-pi", "inflection/inflection-3-pi"),
            ("inflection", "inflection-3-productivity", "inflection/inflection-3-productivity"),
            
            // ═══════════════════════════════════════════════════════════════
            // SILICONFLOW - Çin Provider
            // ═══════════════════════════════════════════════════════════════
            ("siliconflow", "Qwen/Qwen2.5-72B-Instruct", "siliconflow/Qwen2.5-72B"),
            ("siliconflow", "Qwen/Qwen2.5-32B-Instruct", "siliconflow/Qwen2.5-32B"),
            ("siliconflow", "Qwen/Qwen2.5-7B-Instruct", "siliconflow/Qwen2.5-7B"),
            ("siliconflow", "deepseek-ai/DeepSeek-V3", "siliconflow/DeepSeek-V3"),
            ("siliconflow", "deepseek-ai/DeepSeek-R1", "siliconflow/DeepSeek-R1"),
            ("siliconflow", "meta-llama/Llama-3.3-70B-Instruct", "siliconflow/Llama-3.3-70B"),
            
            // ═══════════════════════════════════════════════════════════════
            // HYPERBOLIC - Decentralized Inference
            // ═══════════════════════════════════════════════════════════════
            ("hyperbolic", "meta-llama/llama-3.3-70b-instruct", "hyperbolic/llama-3.3-70b"),
            ("hyperbolic", "meta-llama/llama-3.1-70b-instruct", "hyperbolic/llama-3.1-70b"),
            ("hyperbolic", "mistralai/mistral-7b-instruct", "hyperbolic/mistral-7b"),
            ("hyperbolic", "deepseek-ai/deepseek-v3", "hyperbolic/deepseek-v3"),
            ("hyperbolic", "qwen/qwen-2.5-72b-instruct", "hyperbolic/qwen-2.5-72b"),
            
            // ═══════════════════════════════════════════════════════════════
            // LEPTON AI - Serverless Inference
            // ═══════════════════════════════════════════════════════════════
            ("lepton", "llama3-70b", "lepton/llama3-70b"),
            ("lepton", "llama3-8b", "lepton/llama3-8b"),
            ("lepton", "mixtral-8x7b", "lepton/mixtral-8x7b"),
            ("lepton", "qwen2.5-72b", "lepton/qwen2.5-72b"),
            ("lepton", "gemma-2-27b", "lepton/gemma-2-27b"),
            
            // ═══════════════════════════════════════════════════════════════
            // RUNPOD - Serverless GPU
            // ═══════════════════════════════════════════════════════════════
            ("runpod", "llama-3-70b", "runpod/llama-3-70b"),
            ("runpod", "llama-3-8b", "runpod/llama-3-8b"),
            ("runpod", "mixtral-8x7b", "runpod/mixtral-8x7b"),
            ("runpod", "qwen-2.5-72b", "runpod/qwen-2.5-72b"),
            
            // ═══════════════════════════════════════════════════════════════
            // MODAL - Serverless Inference
            // ═══════════════════════════════════════════════════════════════
            ("modal", "llama-3.3-70b", "modal/llama-3.3-70b"),
            ("modal", "llama-3.1-405b", "modal/llama-3.1-405b"),
            ("modal", "mixtral-8x22b", "modal/mixtral-8x22b"),
            
            // ═══════════════════════════════════════════════════════════════
            // G4F - GPT4Free (ÜCRETSİZ)
            // ═══════════════════════════════════════════════════════════════
            ("g4f", "gpt-4", "g4f/gpt-4"),
            ("g4f", "gpt-4-turbo", "g4f/gpt-4-turbo"),
            ("g4f", "gpt-3.5-turbo", "g4f/gpt-3.5-turbo"),
            ("g4f", "claude-3-opus", "g4f/claude-3-opus"),
            ("g4f", "gemini-pro", "g4f/gemini-pro"),
            
            ("custom", "custom", "custom/custom"),
            
            // === SKIP ===
            ("__skip__", "", "Skip for now"),
        ];
        
        let provider_names: Vec<&str> = providers.iter().map(|(_, _, display)| *display).collect();
        
        // Fuzzy select for easy search
        let selection = FuzzySelect::new()
            .with_prompt("Select provider (type to search)")
            .items(&provider_names)
            .default(0)
            .interact()?;
        
        let (provider_id, default_model, _) = providers[selection];
        
        // Check if skipped
        if provider_id == "__skip__" {
            return Ok(StepResult::Skipped);
        }
        
        // Set provider and model
        self.config.llm.provider = provider_id.to_string();
        self.config.llm.model = default_model.to_string();
        
        println!();
        println!("[OK] Selected: {} ({})", default_model, provider_id);
        
        // API Key input (hidden) - except for local Ollama
        if provider_id != "ollama" && provider_id != "g4f" {
            self.prompt_api_key(provider_id)?;
        } else {
            println!();
            println!("{}", style("Note: Ollama installation required:").yellow());
            println!("   curl -fsSL https://ollama.com/install.sh | sh");
            println!("   ollama pull {}", default_model);
        }
        
        // Model selection for providers with multiple models
        if provider_id == "openrouter" || provider_id == "ollama" || provider_id == "openai" {
            self.select_model_variant(provider_id)?;
        }
        
        Ok(StepResult::Completed)
    }
    
    fn prompt_api_key(&mut self, provider: &str) -> anyhow::Result<()> {
        println!();
        println!("{}", style("API Key Input (hidden)").bold().yellow());
        
        let prompt = match provider {
            "anthropic" => "Anthropic API Key (sk-ant-...)",
            "openai" => "OpenAI API Key (sk-...)",
            "google" => "Google AI API Key (AIza...)",
            "openrouter" => "OpenRouter API Key (sk-or-...)",
            "groq" => "Groq API Key (gsk_...)",
            "deepseek" => "DeepSeek API Key",
            "mistral" => "Mistral API Key",
            "together" => "Together AI API Key",
            "perplexity" => "Perplexity API Key (pplx-...)",
            "azure" => "Azure OpenAI Key",
            "bedrock" => "AWS Access Key ID",
            "vertex" => "Google Cloud API Key",
            "moonshot" => "Moonshot API Key",
            "zhipu" => "ZhipuAI API Key",
            "baidu" => "Baidu API Key",
            "alibaba" => "Alibaba DashScope API Key",
            "xai" => "X.AI API Key",
            "cohere" => "Cohere API Key",
            "replicate" => "Replicate API Key",
            "watsonx" => "IBM WatsonX API Key",
            // === YENİ PROVIDER'LAR ===
            "huggingface" => "HuggingFace API Token (hf_...)",
            "fireworks" => "Fireworks AI API Key",
            "nvidia" => "NVIDIA API Key (nvapi-...)",
            "novita" => "Novita AI API Key",
            "ai21" => "AI21 Labs API Key",
            "stability" => "Stability AI API Key (sk-...)",
            "inflection" => "Inflection AI API Key",
            "character" => "Character.AI Token",
            "siliconflow" => "SiliconFlow API Key",
            "hyperbolic" => "Hyperbolic API Key",
            "lepton" => "Lepton AI API Key",
            "runpod" => "RunPod API Key",
            "modal" => "Modal API Key",
            "g4f" => "G4F - Ücretsiz (API Key gerekmez)",
            _ => "API Key",
        };
        
        let api_key = Password::new()
            .with_prompt(prompt)
            .allow_empty_password(true)
            .interact()?;
        
        if !api_key.is_empty() {
            match provider {
                "anthropic" => self.config.api_keys.anthropic = Some(api_key),
                "openai" => self.config.api_keys.openai = Some(api_key),
                "google" => self.config.api_keys.google = Some(api_key),
                "deepseek" => self.config.api_keys.deepseek = Some(api_key),
                "mistral" => self.config.api_keys.mistral = Some(api_key),
                "groq" => self.config.api_keys.groq = Some(api_key),
                "together" => self.config.api_keys.together = Some(api_key),
                _ => { self.config.api_keys.extra.insert(provider.to_string(), api_key); }
            };
            println!("[OK] API Key saved (hidden)");
        } else {
            println!("[!] API Key not provided - you can add it later");
        }
        
        Ok(())
    }
    
    fn select_model_variant(&mut self, provider: &str) -> anyhow::Result<()> {
        println!();
        
        let confirm = Confirm::new()
            .with_prompt("Select a different model?")
            .default(false)
            .interact()?;
        
        if !confirm {
            return Ok(());
        }
        
        let models: Vec<(&str, &str)> = match provider {
            "openrouter" => vec![
                ("openrouter/auto", "openrouter/auto"),
                ("anthropic/claude-3.5-sonnet", "openrouter/anthropic/claude-3.5-sonnet"),
                ("anthropic/claude-3-opus", "openrouter/anthropic/claude-3-opus"),
                ("openai/gpt-4o", "openrouter/openai/gpt-4o"),
                ("google/gemini-pro", "openrouter/google/gemini-pro"),
                ("meta-llama/llama-3.1-405b-instruct", "openrouter/meta-llama/llama-3.1-405b-instruct"),
                ("mistralai/mistral-large", "openrouter/mistralai/mistral-large"),
                ("deepseek/deepseek-chat", "openrouter/deepseek/deepseek-chat"),
                ("qwen/qwen-2.5-72b-instruct", "openrouter/qwen/qwen-2.5-72b-instruct"),
                ("x-ai/grok-beta", "openrouter/x-ai/grok-beta"),
            ],
            "ollama" => vec![
                ("llama3.3:70b", "ollama/llama3.3:70b"),
                ("llama3.2:3b", "ollama/llama3.2:3b"),
                ("qwen2.5:72b", "ollama/qwen2.5:72b"),
                ("qwen2.5-coder:32b", "ollama/qwen2.5-coder:32b"),
                ("deepseek-r1:70b", "ollama/deepseek-r1:70b"),
                ("gemma2:27b", "ollama/gemma2:27b"),
                ("mistral:7b", "ollama/mistral:7b"),
                ("codellama:34b", "ollama/codellama:34b"),
                ("phi4:14b", "ollama/phi4:14b"),
                ("mixtral:8x7b", "ollama/mixtral:8x7b"),
            ],
            "openai" => vec![
                ("gpt-4o", "openai/gpt-4o"),
                ("gpt-4o-mini", "openai/gpt-4o-mini"),
                ("gpt-4-turbo", "openai/gpt-4-turbo"),
                ("gpt-4", "openai/gpt-4"),
                ("gpt-3.5-turbo", "openai/gpt-3.5-turbo"),
                ("o1-preview", "openai/o1-preview"),
                ("o1-mini", "openai/o1-mini"),
            ],
            _ => return Ok(()),
        };
        
        let model_names: Vec<&str> = models.iter().map(|(_, desc)| *desc).collect();
        
        let selection = Select::new()
            .with_prompt("Select model")
            .items(&model_names)
            .default(0)
            .interact()?;
        
        let (model, _) = models[selection];
        self.config.llm.model = model.to_string();
        println!("[OK] Model: {}", model);
        
        Ok(())
    }
    
    // ═══════════════════════════════════════════════════════════════════════════════
    // Communication Channels Configuration (20+)
    // ═══════════════════════════════════════════════════════════════════════════════
    
    fn configure_communication_channels(&mut self) -> anyhow::Result<StepResult> {
        println!();
        println!("{}", style("Communication Channels - 20+ Platforms").bold().cyan());
        println!("{}", style("   Space: Select/Remove    Enter: Confirm").dim());
        println!();
        
        // 20+ Communication channels (NO emojis)
        let channels = vec![
            // Mobile Messengers
            ("telegram", "Telegram Bot"),
            ("whatsapp", "WhatsApp Business"),
            ("signal", "Signal"),
            ("imessage", "iMessage (macOS)"),
            ("wechat", "WeChat"),
            ("line", "LINE"),
            ("viber", "Viber"),
            ("kakaotalk", "KakaoTalk"),
            
            // Enterprise Platforms
            ("discord", "Discord"),
            ("slack", "Slack"),
            ("ms_teams", "Microsoft Teams"),
            ("google_chat", "Google Chat"),
            ("webex", "Webex"),
            ("zoom", "Zoom Chat"),
            ("mattermost", "Mattermost"),
            ("rocketchat", "Rocket.Chat"),
            
            // Decentralized / Privacy
            ("matrix", "Matrix/Element"),
            ("xmpp", "XMPP/Jabber"),
            ("session", "Session"),
            ("wire", "Wire"),
            ("threema", "Threema"),
            ("nostr", "Nostr"),
            
            // Social Platforms
            ("twitter", "Twitter/X DM"),
            ("instagram", "Instagram DM"),
            ("facebook", "Facebook Messenger"),
            ("linkedin", "LinkedIn"),
            ("reddit", "Reddit"),
            
            // Email & SMS
            ("email", "Email (SMTP/IMAP)"),
            ("sms", "SMS (Twilio)"),
            ("rcs", "RCS (Rich Communication)"),
            
            // Web/API
            ("web", "Web Dashboard"),
            ("api", "REST API"),
        ];
        
        let channel_names: Vec<&str> = channels.iter().map(|(_, name)| *name).collect();
        
        // Multi-select with Space
        let selections = MultiSelect::new()
            .with_prompt("Select channels (Space to select, Enter to confirm)")
            .items(&channel_names)
            .defaults(&[false].repeat(channels.len())) // None selected by default
            .interact()?;
        
        // Check if nothing selected = skip
        if selections.is_empty() {
            // Ask if they want to skip
            let skip = Confirm::new()
                .with_prompt("No channels selected. Continue anyway?")
                .default(true)
                .interact()?;
            if skip {
                return Ok(StepResult::Skipped);
            }
            // If no, return to selection
            return self.configure_communication_channels();
        }
        
        // Configure each selected channel
        for idx in selections {
            let (key, _) = channels[idx];
            self.setup_channel(key)?;
        }
        
        Ok(StepResult::Completed)
    }
    
    fn setup_channel(&mut self, channel: &str) -> anyhow::Result<()> {
        match channel {
            "telegram" => self.setup_telegram_full()?,
            "whatsapp" => self.setup_whatsapp_full()?,
            "discord" => self.setup_discord_full()?,
            "slack" => self.setup_slack_full()?,
            "matrix" => self.setup_matrix_full()?,
            "signal" => self.setup_signal()?,
            "email" => self.setup_email_full()?,
            "twitter" => self.setup_twitter()?,
            "linkedin" => self.setup_linkedin()?,
            "reddit" => self.setup_reddit()?,
            "web" => {
                self.config.integrations.extra.insert("web_enabled".to_string(), "true".to_string());
                println!("  [OK] Web Dashboard enabled");
            }
            "api" => {
                self.config.integrations.extra.insert("api_enabled".to_string(), "true".to_string());
                println!("  [OK] REST API enabled");
            }
            _ => {
                // Generic setup for other channels
                println!("  [OK] {} selected - configure later", channel);
                self.config.integrations.extra.insert(format!("{}_enabled", channel), "true".to_string());
            }
        }
        Ok(())
    }
    
    fn setup_telegram_full(&mut self) -> anyhow::Result<()> {
        println!();
        println!("{}", style("  Telegram Bot Setup").bold().cyan());
        println!("     1. Find @BotFather on Telegram");
        println!("     2. Send /newbot and follow instructions");
        println!("     3. Copy the token");
        println!();
        
        // Bot Token
        let token = Password::new()
            .with_prompt("  Bot Token (hidden)")
            .allow_empty_password(true)
            .interact()?;
        
        if token.is_empty() {
            println!("  [SKIP] No token provided, skipping");
            return Ok(());
        }
        
        // Security Policy
        println!();
        println!("{}", style("  Security Policy:").bold().yellow());
        
        let policies = vec![
            "DM Pairing - Only paired users",
            "Allowlist - Allowed users list",
            "Open - Anyone can use (not recommended)",
            "Skip - Default (DM Pairing)",
        ];
        
        let policy_sel = Select::new()
            .with_prompt("  Select security policy")
            .items(&policies)
            .default(3)
            .interact()?;
        
        let policy = match policy_sel {
            0 => "dm_pairing",
            1 => "allowlist",
            2 => "open",
            _ => "dm_pairing",
        };
        
        self.config.integrations.telegram = Some(IntegrationConfig {
            enabled: true,
            token: Some(token),
            extra: [
                ("security_policy".to_string(), policy.to_string()),
                ("allowed_users".to_string(), "".to_string()),
            ].into_iter().collect(),
        });
        
        println!("  [OK] Telegram connected! (Policy: {})", policy);
        
        // If allowlist, ask for users
        if policy == "allowlist" {
            let users: String = Input::new()
                .with_prompt("  Allowed Telegram User IDs (comma-separated)")
                .allow_empty(true)
                .interact_text()?;
            
            if let Some(ref mut telegram) = self.config.integrations.telegram {
                telegram.extra.insert("allowed_users".to_string(), users);
            }
        }
        
        Ok(())
    }
    
    fn setup_whatsapp_full(&mut self) -> anyhow::Result<()> {
        println!();
        println!("{}", style("  WhatsApp Business API").bold().cyan());
        println!("     https://business.facebook.com");
        println!();
        
        let phone_id: String = Input::new()
            .with_prompt("  Phone Number ID")
            .allow_empty(true)
            .interact_text()?;
        
        if phone_id.is_empty() {
            println!("  [SKIP] Skipping");
            return Ok(());
        }
        
        let token = Password::new()
            .with_prompt("  Access Token (hidden)")
            .allow_empty_password(true)
            .interact()?;
        
        // Security Policy
        let policies = vec![
            "Verified Numbers - Verified numbers only",
            "Allowlist - Allowed list",
            "Skip",
        ];
        
        let policy_sel = Select::new()
            .with_prompt("  Security policy")
            .items(&policies)
            .default(2)
            .interact()?;
        
        let policy = match policy_sel {
            0 => "verified",
            1 => "allowlist",
            _ => "default",
        };
        
        self.config.integrations.whatsapp = Some(IntegrationConfig {
            enabled: true,
            token: Some(token),
            extra: [
                ("phone_id".to_string(), phone_id),
                ("security_policy".to_string(), policy.to_string()),
            ].into_iter().collect(),
        });
        
        println!("  [OK] WhatsApp connected!");
        Ok(())
    }
    
    fn setup_discord_full(&mut self) -> anyhow::Result<()> {
        println!();
        println!("{}", style("  Discord Bot").bold().cyan());
        println!("     https://discord.com/developers/applications");
        println!();
        
        let token = Password::new()
            .with_prompt("  Bot Token (hidden)")
            .allow_empty_password(true)
            .interact()?;
        
        if token.is_empty() {
            println!("  [SKIP] Skipping");
            return Ok(());
        }
        
        // Server selection
        let guild_id: String = Input::new()
            .with_prompt("  Server/Guild ID (optional)")
            .allow_empty(true)
            .interact_text()?;
        
        self.config.integrations.discord = Some(IntegrationConfig {
            enabled: true,
            token: Some(token),
            extra: if !guild_id.is_empty() {
                [("guild_id".to_string(), guild_id)].into_iter().collect()
            } else {
                HashMap::new()
            },
        });
        
        println!("  [OK] Discord connected!");
        Ok(())
    }
    
    fn setup_slack_full(&mut self) -> anyhow::Result<()> {
        println!();
        println!("{}", style("  Slack App").bold().cyan());
        println!("     https://api.slack.com/apps");
        println!();
        
        let token = Password::new()
            .with_prompt("  Bot Token (xoxb-...) (hidden)")
            .allow_empty_password(true)
            .interact()?;
        
        if token.is_empty() {
            println!("  [SKIP] Skipping");
            return Ok(());
        }
        
        let channel: String = Input::new()
            .with_prompt("  Default Channel (e.g. #general)")
            .allow_empty(true)
            .interact_text()?;
        
        self.config.integrations.slack = Some(IntegrationConfig {
            enabled: true,
            token: Some(token),
            extra: if !channel.is_empty() {
                [("default_channel".to_string(), channel)].into_iter().collect()
            } else {
                HashMap::new()
            },
        });
        
        println!("  [OK] Slack connected!");
        Ok(())
    }
    
    fn setup_matrix_full(&mut self) -> anyhow::Result<()> {
        println!();
        println!("{}", style("  Matrix/Element").bold().cyan());
        println!("     Element -> Settings -> Help & About -> Access Token");
        println!();
        
        let homeserver: String = Input::new()
            .default("https://matrix-client.matrix.org".to_string())
            .with_prompt("  Homeserver URL")
            .interact_text()?;
        
        let token = Password::new()
            .with_prompt("  Access Token (hidden)")
            .allow_empty_password(true)
            .interact()?;
        
        if token.is_empty() {
            println!("  [SKIP] Skipping");
            return Ok(());
        }
        
        let room_id: String = Input::new()
            .with_prompt("  Default Room ID (optional)")
            .allow_empty(true)
            .interact_text()?;
        
        self.config.integrations.matrix = Some(IntegrationConfig {
            enabled: true,
            token: Some(token),
            extra: [
                ("homeserver".to_string(), homeserver),
                ("room_id".to_string(), room_id),
            ].into_iter().collect(),
        });
        
        println!("  [OK] Matrix connected!");
        Ok(())
    }
    
    fn setup_signal(&mut self) -> anyhow::Result<()> {
        println!();
        println!("{}", style("  Signal").bold().cyan());
        println!("     signal-cli or signald required");
        println!();
        
        let phone: String = Input::new()
            .with_prompt("  Phone number (+90...)")
            .allow_empty(true)
            .interact_text()?;
        
        if phone.is_empty() {
            println!("  [SKIP] Skipping");
            return Ok(());
        }
        
        self.config.integrations.signal = Some(IntegrationConfig {
            enabled: true,
            token: None,
            extra: [("phone".to_string(), phone)].into_iter().collect(),
        });
        
        println!("  [OK] Signal configured!");
        Ok(())
    }
    
    fn setup_email_full(&mut self) -> anyhow::Result<()> {
        println!();
        println!("{}", style("  Email (SMTP)").bold().cyan());
        println!();
        
        let host: String = Input::new()
            .default("smtp.gmail.com".to_string())
            .with_prompt("  SMTP Host")
            .interact_text()?;
        
        let port: String = Input::new()
            .default("587".to_string())
            .with_prompt("  SMTP Port")
            .interact_text()?;
        
        let user: String = Input::new()
            .with_prompt("  Email/Username")
            .interact_text()?;
        
        let pass = Password::new()
            .with_prompt("  Password/App Password (hidden)")
            .interact()?;
        
        self.config.integrations.email = Some(IntegrationConfig {
            enabled: true,
            token: None,
            extra: [
                ("host".to_string(), host),
                ("port".to_string(), port),
                ("user".to_string(), user),
                ("pass".to_string(), pass),
            ].into_iter().collect(),
        });
        
        println!("  [OK] Email connected!");
        Ok(())
    }
    
    fn setup_twitter(&mut self) -> anyhow::Result<()> {
        println!();
        println!("{}", style("  Twitter/X API").bold().cyan());
        println!("     https://developer.twitter.com");
        println!();
        
        let bearer = Password::new()
            .with_prompt("  Bearer Token (hidden)")
            .allow_empty_password(true)
            .interact()?;
        
        if bearer.is_empty() {
            println!("  [SKIP] Skipping");
            return Ok(());
        }
        
        self.config.integrations.twitter = Some(IntegrationConfig {
            enabled: true,
            token: Some(bearer),
            extra: HashMap::new(),
        });
        
        println!("  [OK] Twitter connected!");
        Ok(())
    }
    
    fn setup_linkedin(&mut self) -> anyhow::Result<()> {
        println!();
        println!("{}", style("  LinkedIn API").bold().cyan());
        println!();
        
        let token = Password::new()
            .with_prompt("  Access Token (hidden)")
            .allow_empty_password(true)
            .interact()?;
        
        if token.is_empty() {
            println!("  [SKIP] Skipping");
            return Ok(());
        }
        
        self.config.integrations.linkedin = Some(IntegrationConfig {
            enabled: true,
            token: Some(token),
            extra: HashMap::new(),
        });
        
        println!("  [OK] LinkedIn connected!");
        Ok(())
    }
    
    fn setup_reddit(&mut self) -> anyhow::Result<()> {
        println!();
        println!("{}", style("  Reddit API").bold().cyan());
        println!("     https://www.reddit.com/prefs/apps");
        println!();
        
        let client_id: String = Input::new()
            .with_prompt("  Client ID")
            .allow_empty(true)
            .interact_text()?;
        
        if client_id.is_empty() {
            println!("  [SKIP] Skipping");
            return Ok(());
        }
        
        let secret = Password::new()
            .with_prompt("  Client Secret (hidden)")
            .allow_empty_password(true)
            .interact()?;
        
        self.config.integrations.reddit = Some(IntegrationConfig {
            enabled: true,
            token: Some(secret),
            extra: [("client_id".to_string(), client_id)].into_iter().collect(),
        });
        
        println!("  [OK] Reddit connected!");
        Ok(())
    }
    
    // ═══════════════════════════════════════════════════════════════════════════════
    // Tools Configuration (Web Search)
    // ═══════════════════════════════════════════════════════════════════════════════
    
    fn configure_tools(&mut self) -> anyhow::Result<StepResult> {
        println!();
        println!("{}", style("Tools").bold().cyan());
        println!();
        
        // Web Search Provider
        println!("{}", style("Web Search Provider:").bold());
        println!();
        
        let providers = vec![
            ("searxng", "SearXNG - Open source, self-hosted"),
            ("duckduckgo", "DuckDuckGo - Privacy focused"),
            ("ollama_web", "Ollama Web Search - Local LLM + Web"),
            ("google_cse", "Google Custom Search - Official API"),
            ("bing", "Bing Search API - Microsoft"),
            ("tavily", "Tavily - AI-optimized search"),
            ("serper", "Serper - Google API wrapper"),
            ("brave", "Brave Search - Privacy"),
            ("kagi", "Kagi - Premium search"),
            ("__skip__", "Skip for now"),
        ];
        
        let provider_names: Vec<&str> = providers.iter().map(|(_, name)| *name).collect();
        
        let selection = Select::new()
            .with_prompt("Select web search provider")
            .items(&provider_names)
            .default(9) // Skip by default
            .interact()?;
        
        let (provider_id, _) = providers[selection];
        
        if provider_id == "__skip__" {
            return Ok(StepResult::Skipped);
        }
        
        // Configure the selected provider
        self.setup_web_search(provider_id)?;
        
        // Additional tools
        println!();
        println!("{}", style("Additional Tools:").bold());
        
        let tools = vec![
            ("code_exec", "Code Execution (Python, JS, Shell)"),
            ("file_ops", "File Operations"),
            ("web_browser", "Web Browser (automation)"),
            ("calendar", "Calendar Integration"),
            ("contacts", "Contacts Integration"),
        ];
        
        let tool_names: Vec<&str> = tools.iter().map(|(_, name)| *name).collect();
        
        let tool_selections = MultiSelect::new()
            .with_prompt("Select additional tools (Space to select)")
            .items(&tool_names)
            .defaults(&[true, true, false, false, false])
            .interact()?;
        
        for idx in tool_selections {
            let (key, _) = tools[idx];
            self.config.integrations.extra.insert(format!("tool_{}", key), "true".to_string());
            println!("  [OK] {} enabled", key);
        }
        
        Ok(StepResult::Completed)
    }
    
    fn setup_web_search(&mut self, provider: &str) -> anyhow::Result<()> {
        match provider {
            "searxng" => {
                println!();
                println!("{}", style("  SearXNG Setup").bold().cyan());
                println!("     https://searx.be or your own instance");
                println!();
                
                let instance: String = Input::new()
                    .default("https://searx.be".to_string())
                    .with_prompt("  SearXNG Instance URL")
                    .interact_text()?;
                
                self.config.integrations.extra.insert("web_search_provider".to_string(), "searxng".to_string());
                self.config.integrations.extra.insert("searxng_instance".to_string(), instance);
            }
            "duckduckgo" => {
                self.config.integrations.extra.insert("web_search_provider".to_string(), "duckduckgo".to_string());
                println!("  [OK] DuckDuckGo will be used (no API key required)");
            }
            "ollama_web" => {
                self.config.integrations.extra.insert("web_search_provider".to_string(), "ollama_web".to_string());
                println!("  [OK] Ollama Web Search enabled");
            }
            "google_cse" => {
                println!();
                println!("{}", style("  Google Custom Search").bold().cyan());
                
                let api_key = Password::new()
                    .with_prompt("  API Key (hidden)")
                    .allow_empty_password(true)
                    .interact()?;
                
                let cx: String = Input::new()
                    .with_prompt("  Custom Search Engine ID (cx)")
                    .allow_empty(true)
                    .interact_text()?;
                
                self.config.integrations.extra.insert("web_search_provider".to_string(), "google_cse".to_string());
                self.config.integrations.extra.insert("google_cse_key".to_string(), api_key);
                self.config.integrations.extra.insert("google_cse_cx".to_string(), cx);
            }
            "bing" => {
                println!();
                let key = Password::new()
                    .with_prompt("  Bing API Key (hidden)")
                    .allow_empty_password(true)
                    .interact()?;
                
                self.config.integrations.extra.insert("web_search_provider".to_string(), "bing".to_string());
                self.config.integrations.extra.insert("bing_api_key".to_string(), key);
            }
            "tavily" => {
                println!();
                let key = Password::new()
                    .with_prompt("  Tavily API Key (hidden)")
                    .allow_empty_password(true)
                    .interact()?;
                
                self.config.integrations.extra.insert("web_search_provider".to_string(), "tavily".to_string());
                self.config.integrations.extra.insert("tavily_api_key".to_string(), key);
            }
            "serper" => {
                println!();
                let key = Password::new()
                    .with_prompt("  Serper API Key (hidden)")
                    .allow_empty_password(true)
                    .interact()?;
                
                self.config.integrations.extra.insert("web_search_provider".to_string(), "serper".to_string());
                self.config.integrations.extra.insert("serper_api_key".to_string(), key);
            }
            "brave" => {
                println!();
                let key = Password::new()
                    .with_prompt("  Brave Search API Key (hidden)")
                    .allow_empty_password(true)
                    .interact()?;
                
                self.config.integrations.extra.insert("web_search_provider".to_string(), "brave".to_string());
                self.config.integrations.extra.insert("brave_api_key".to_string(), key);
            }
            "kagi" => {
                println!();
                let key = Password::new()
                    .with_prompt("  Kagi API Key (hidden)")
                    .allow_empty_password(true)
                    .interact()?;
                
                self.config.integrations.extra.insert("web_search_provider".to_string(), "kagi".to_string());
                self.config.integrations.extra.insert("kagi_api_key".to_string(), key);
            }
            _ => {}
        }
        
        println!("  [OK] Web search configured: {}", provider);
        Ok(())
    }
    
    // ═══════════════════════════════════════════════════════════════════════════════
    // Language Selection
    // ═══════════════════════════════════════════════════════════════════════════════
    
    // ═══════════════════════════════════════════════════════════════════════════════
    // Asistan İsmi & Kişilik Seçimi (Sprint 1 - Personal AI)
    // ═══════════════════════════════════════════════════════════════════════════════
    
    fn configure_assistant_identity(&mut self) -> anyhow::Result<StepResult> {
        println!();
        println!("{}", style("Asistanınızın Kimliğini Belirleyin").bold().cyan());
        println!("{}", style("Your AI, Your Name, Your Style").dim());
        println!();
        
        // ── İsim Seçimi ──
        println!("{}", style("1. Asistan İsmi").bold());
        println!("   Asistanınıza bir isim verin. Bu isim tüm sistemde kullanılacak.");
        println!("   Wake word, konuşmalar, bildirimler — her yerde bu isim görünecek.");
        println!();
        
        let preset_names = vec![
            "Jarvis",
            "Atlas",
            "Nova",
            "Echo",
            "Luna",
            "Orion",
            "Iris",
            "Axel",
            "Kai",
            "Zara",
            "__custom__",
            "__skip__",
        ];
        
        let preset_labels: Vec<String> = preset_names.iter().map(|n| {
            match *n {
                "__custom__" => "✏️  Özel isim yaz...".to_string(),
                "__skip__" => "Atla (SENTIENT)".to_string(),
                _ => n.to_string(),
            }
        }).collect();
        
        let label_refs: Vec<&str> = preset_labels.iter().map(|s| s.as_str()).collect();
        
        let selection = Select::new()
            .with_prompt("Asistan ismi seçin")
            .items(&label_refs)
            .default(0)
            .interact()?;
        
        let name = match preset_names[selection] {
            "__skip__" => {
                println!("  [SKIP] Varsayılan: SENTIENT");
                self.config.assistant_name = "SENTIENT".to_string();
                // Kişilik seçimine devam et
                self.configure_personality()?;
                return Ok(StepResult::Skipped);
            }
            "__custom__" => {
                let custom: String = Input::new()
                    .with_prompt("Asistan ismini yazın")
                    .default("SENTIENT".to_string())
                    .interact_text()?;
                custom
            }
            other => other.to_string(),
        };
        
        self.config.assistant_name = name.clone();
        println!("[OK] Asistan ismi: {}", style(&name).green().bold());
        
        // ── Kişilik Seçimi ──
        self.configure_personality()?;
        
        // ── Özet ──
        println!();
        println!("{}", style("── Asistan Kimliği ──").dim());
        println!("  İsim:      {}", style(&self.config.assistant_name).yellow().bold());
        println!("  Kişilik:   {}", style(&self.config.personality).yellow());
        println!();
        
        Ok(StepResult::Completed)
    }
    
    fn configure_personality(&mut self) -> anyhow::Result<()> {
        println!();
        println!("{}", style("2. Kişilik Tarzı").bold());
        println!("   Asistanınız nasıl konuşsun?");
        println!();
        
        let personalities = vec![
            ("friendly",   "Samimi",      "İçten, sıcak, sohbet havasında"),
            ("professional", "Profesyonel", "Ciddi, net, iş odaklı"),
            ("technical",  "Teknik",      "Detaylı, kod odaklı, açıklayıcı"),
            ("casual",     "Günlük",      "Rahat, eğlenceli, kısa cevaplar"),
            ("creative",   "Yaratıcı",    "İlham verici, metaforik, cesur"),
            ("mentor",     "Mentor",      "Öğretici, sabırlı, adım adım"),
        ];
        
        let personality_labels: Vec<String> = personalities.iter()
            .map(|(_, name, desc)| format!("{} — {}", name, desc))
            .collect();
        
        let label_refs: Vec<&str> = personality_labels.iter().map(|s| s.as_str()).collect();
        
        let selection = Select::new()
            .with_prompt("Kişilik tarzı seçin")
            .items(&label_refs)
            .default(0)
            .interact()?;
        
        let (style_key, style_name, _) = personalities[selection];
        self.config.personality = style_key.to_string();
        println!("[OK] Kişilik: {} ({})", style_name, style_key);
        
        Ok(())
    }
    
    // ═══════════════════════════════════════════════════════════════════════════════
    // Sesli Asistan Yapılandırması (Sprint 1 - Personal AI)
    // ═══════════════════════════════════════════════════════════════════════════════
    
    fn configure_voice(&mut self) -> anyhow::Result<StepResult> {
        println!();
        println!("{}", style("Sesli Asistan").bold().cyan());
        println!("   Asistanınızı sesle uyandırın ve sesli yanıt alın");
        println!();
        
        let enable = Confirm::new()
            .with_prompt(format!(
                "Sesli asistanı etkinleştir? (\"Hey {}\" ile uyandır)",
                self.config.assistant_name
            ))
            .default(false)
            .interact()?;
        
        if !enable {
            self.config.voice_enabled = false;
            println!("  [OK] Sesli asistan devre dışı");
            return Ok(StepResult::Skipped);
        }
        
        self.config.voice_enabled = true;
        
        println!();
        println!("  Wake word: \"Hey {}\"", style(&self.config.assistant_name).green().bold());
        println!("  STT: Whisper (yüklü değilse kurulacak)");
        println!("  TTS: Sistem ses motoru");
        println!();
        println!("[OK] Sesli asistan etkinleştirildi!");
        
        Ok(StepResult::Completed)
    }

    fn select_language(&mut self) -> anyhow::Result<StepResult> {
        let languages = vec![
            ("en", "English"),
            ("tr", "Turkce"),
            ("de", "Deutsch"),
            ("fr", "Francais"),
            ("es", "Espanol"),
            ("zh", "Chinese"),
            ("ja", "Japanese"),
            ("ko", "Korean"),
            ("__skip__", "Skip"),
        ];
        
        let lang_names: Vec<&str> = languages.iter().map(|(_, name)| *name).collect();
        
        let selection = Select::new()
            .with_prompt("Select language")
            .items(&lang_names)
            .default(0)
            .interact()?;
        
        let (lang_code, _) = languages[selection];
        
        if lang_code == "__skip__" {
            return Ok(StepResult::Skipped);
        }
        
        self.config.language = lang_code.to_string();
        println!("[OK] Language: {}", lang_names[selection]);
        
        Ok(StepResult::Completed)
    }
    
    // ═══════════════════════════════════════════════════════════════════════════════
    // Permissions Configuration
    // ═══════════════════════════════════════════════════════════════════════════════
    
    fn configure_permissions(&mut self) -> anyhow::Result<StepResult> {
        println!();
        println!("{}", style("Permissions").bold().cyan());
        println!();
        
        let levels = vec![
            ("1", "Level 1 - Read Only"),
            ("2", "Level 2 - File Operations"),
            ("3", "Level 3 - GUI Control (Agent-S3)"),
            ("4", "Level 4 - Full Autonomous"),
            ("5", "Level 5 - System Administration"),
            ("__skip__", "Skip (Level 2)"),
        ];
        
        let level_names: Vec<&str> = levels.iter().map(|(_, desc)| *desc).collect();
        
        let selection = Select::new()
            .with_prompt("Select permission level")
            .items(&level_names)
            .default(5) // Skip by default
            .interact()?;
        
        let (level, _) = levels[selection];
        
        if level == "__skip__" {
            self.config.permissions.default_level = 2;
            return Ok(StepResult::Skipped);
        }
        
        self.config.permissions.default_level = level.parse().unwrap_or(2);
        
        // GUI control permission
        if selection >= 2 && selection < 5 {
            let gui_confirm = Confirm::new()
                .with_prompt("Allow keyboard/mouse control?")
                .default(true)
                .interact()?;
            
            self.config.permissions.allow_gui_control = gui_confirm;
        }
        
        println!();
        println!("[OK] Permission level: {}", level);
        
        Ok(StepResult::Completed)
    }
    
    // ═══════════════════════════════════════════════════════════════════════════════
    // Helper Methods
    // ═══════════════════════════════════════════════════════════════════════════════
    
    fn print_welcome(&self) {
        let _ = self.term.clear_screen();
        
        println!();
        println!("{}", style("╔════════════════════════════════════════════════════════════════════════════════╗").cyan());
        println!("{}", style("║                                                                                ║").cyan());
        println!("{} {} {}", style("║").cyan(), style("   SENTIENT OS - Personal AI Setup Wizard                               ").bold(), style("║").cyan());
        println!("{}", style("║                                                                                ║").cyan());
        println!("{} {} {}", style("║").cyan(), style("   Your AI, Your Name, Your Style                                         ").dim(), style("║").cyan());
        println!("{}", style("║                                                                                ║").cyan());
        println!("{}", style("║   Controls:                                                                    ║").cyan());
        println!("{}", style("║      Arrow Keys: Navigate     Space: Multi-Select     Enter: Confirm          ║").cyan());
        println!("{}", style("║      Esc: Back                Type: Fuzzy Search                           ║").cyan());
        println!("{}", style("║                                                                                ║").cyan());
        println!("{}", style("╚════════════════════════════════════════════════════════════════════════════════╝").cyan());
        println!();
    }
    
    fn print_goodbye(&self) {
        println!();
        let name = &self.config.assistant_name;
        println!("{}", style("╔════════════════════════════════════════════════════════════════════════════════╗").green());
        println!("{} {}{} ║", style("║   ").green(), style(format!("{} sizi bekliyor...", name)).green().bold(), " ".repeat(56usize.saturating_sub(name.len())));
        println!("{}", style("║                                                                                ║").green());
        println!("{}", style("║   Documentation: https://docs.sentient-os.ai                                   ║").green());
        println!("{}", style("║   Discord: https://discord.gg/sentient                                         ║").green());
        println!("{}", style("╚════════════════════════════════════════════════════════════════════════════════╝").green());
        println!();
    }
    
    fn print_step(&self, title: &str) {
        let progress = format!("[{}/{}]", self.step, self.total_steps);
        println!();
        println!("{}", style(format!("-> {} {}", progress, title)).bold().yellow());
        println!("{}", style("─".repeat(70)).dim());
    }
    
    fn save_and_show_success(&mut self) -> anyhow::Result<()> {
        let pb = ProgressBar::new(100);
        pb.set_style(
            ProgressStyle::with_template("{spinner:.green} {msg} [{bar:40.cyan/blue}] {percent}%")
                .expect("operation failed")
                .progress_chars("=> ")
        );
        
        pb.set_message("Saving configuration...");
        pb.inc(30);
        
        // Save config
        self.config.save()?;
        pb.inc(40);
        
        // Simulate a bit of setup work
        pb.set_message("Creating defaults...");
        pb.inc(30);
        
        pb.finish_with_message("Save completed!");
        
        println!();
        println!("{}", style("╔════════════════════════════════════════════════════════════════════════════════╗").green());
        println!("{}", style("║                                                                                ║").green());
        println!("{}", style("║   SETUP COMPLETED SUCCESSFULLY                                                 ║").green());
        println!("{}", style("║                                                                                ║").green());
        
        // Asistan ismi & kişilik
        let name_display = &self.config.assistant_name;
        let personality_display = &self.config.personality;
        println!("{} {}{} ║", style("║   Assistant: ").green(), style(name_display).yellow().bold(), " ".repeat(56usize.saturating_sub(name_display.len())));
        println!("{} {}{} ║", style("║   Personality: ").green(), style(personality_display).yellow(), " ".repeat(53usize.saturating_sub(personality_display.len())));
        println!("{} {}{} ║", style("║   Language: ").green(), style(&self.config.language).yellow(), " ".repeat(56usize.saturating_sub(self.config.language.len())));
        if self.config.voice_enabled {
            println!("{} Hey {}{} ║", style("║   Wake Word: ").green(), style(name_display).yellow().bold(), " ".repeat(53usize.saturating_sub(name_display.len())));
        }
        println!("{}", style("║                                                                                ║").green());
        
        // Model info
        let model_display = &self.config.llm.model;
        let provider_display = &self.config.llm.provider;
        println!("{} {}{} ║", style("║   Model: ").green(), style(model_display).yellow().bold(), " ".repeat(58usize.saturating_sub(model_display.len())));
        println!("{} {}{} ║", style("║   Provider: ").green(), style(provider_display).yellow().bold(), " ".repeat(55usize.saturating_sub(provider_display.len())));
        
        // Channels
        let enabled = self.get_enabled_integrations();
        if !enabled.is_empty() {
            let channels = enabled.join(", ");
            println!("{} {}{} ║", style("║   Channels: ").green(), style(&channels).yellow(), " ".repeat(54usize.saturating_sub(channels.len())));
        }
        
        println!("{}", style("║                                                                                ║").green());
        
        // Config path
        let config_display = &self.config.config_path;
        println!("{} {}{} ║", style("║   Config: ").green(), style(config_display).yellow(), " ".repeat(55usize.saturating_sub(config_display.len())));
        
        // Dashboard info
        println!("{} {}{} ║", style("║   Dashboard: ").green(), style(format!("http://{}:{}", self.config.dashboard.host, self.config.dashboard.port)).yellow(), " ".repeat(44));
        
        println!("{}", style("║                                                                                ║").green());
        println!("{}", style("║   To start: sentient                                                           ║").green());
        println!("{}", style("║   Documentation: https://docs.sentient-os.ai                                   ║").green());
        println!("{}", style("║                                                                                ║").green());
        println!("{}", style("╚════════════════════════════════════════════════════════════════════════════════╝").green());
        println!();
        
        Ok(())
    }
    
    fn get_enabled_integrations(&self) -> Vec<String> {
        let mut enabled = vec![];
        
        if self.config.integrations.telegram.as_ref().map_or(false, |c| c.enabled) {
            enabled.push("Telegram".to_string());
        }
        if self.config.integrations.discord.as_ref().map_or(false, |c| c.enabled) {
            enabled.push("Discord".to_string());
        }
        if self.config.integrations.slack.as_ref().map_or(false, |c| c.enabled) {
            enabled.push("Slack".to_string());
        }
        if self.config.integrations.whatsapp.as_ref().map_or(false, |c| c.enabled) {
            enabled.push("WhatsApp".to_string());
        }
        if self.config.integrations.matrix.as_ref().map_or(false, |c| c.enabled) {
            enabled.push("Matrix".to_string());
        }
        if self.config.integrations.email.as_ref().map_or(false, |c| c.enabled) {
            enabled.push("Email".to_string());
        }
        if self.config.integrations.signal.as_ref().map_or(false, |c| c.enabled) {
            enabled.push("Signal".to_string());
        }
        if self.config.integrations.twitter.as_ref().map_or(false, |c| c.enabled) {
            enabled.push("Twitter".to_string());
        }
        if self.config.integrations.linkedin.as_ref().map_or(false, |c| c.enabled) {
            enabled.push("LinkedIn".to_string());
        }
        if self.config.integrations.reddit.as_ref().map_or(false, |c| c.enabled) {
            enabled.push("Reddit".to_string());
        }
        
        if self.config.integrations.extra.get("web_enabled").map_or(false, |v| v == "true") {
            enabled.push("Web".to_string());
        }
        if self.config.integrations.extra.get("api_enabled").map_or(false, |v| v == "true") {
            enabled.push("API".to_string());
        }
        
        enabled
    }
    
    // ═══════════════════════════════════════════════════════════════════════════════
    // Model Recommendation System - Use Case Based Selection
    // ═══════════════════════════════════════════════════════════════════════════════
    
    /// Select use case for model recommendations
    fn select_use_case(&self) -> anyhow::Result<String> {
        println!();
        println!("{}", style("What will you use SENTIENT for?").bold().yellow());
        println!("{}", style("   This helps us recommend the best models for your needs.").dim());
        println!();
        
        let use_cases = vec![
            ("Code Development", "Code generation, debugging, refactoring, code review"),
            ("Research & Analysis", "Deep analysis, summarization, fact-checking"),
            ("Reasoning & Math", "Complex reasoning, logic, mathematical proofs"),
            ("Chat & Assistant", "General conversation, Q&A, daily tasks"),
            ("Content Creation", "Writing, editing, creative work"),
            ("Data Processing", "ETL, data transformation, analysis"),
            ("Automation", "Workflow automation, task scheduling"),
            ("Security & Audit", "Penetration testing, security analysis, audit"),
            ("Show All Models", "Browse all 600+ models without filtering"),
        ];
        
        let use_case_names: Vec<&str> = use_cases.iter().map(|(name, _)| *name).collect();
        
        let selection = Select::new()
            .with_prompt("Select your primary use case")
            .items(&use_case_names)
            .default(0)
            .interact()?;
        
        // Return identifier for each use case
        Ok(match selection {
            0 => "code".to_string(),
            1 => "research".to_string(),
            2 => "reasoning".to_string(),
            3 => "chat".to_string(),
            4 => "content".to_string(),
            5 => "data".to_string(),
            6 => "automation".to_string(),
            7 => "security".to_string(),
            _ => "all".to_string(),
        })
    }
    
    /// Show model recommendations based on use case
    fn show_model_recommendations(&mut self, use_case: &str) -> anyhow::Result<()> {
        println!();
        println!("{}", style("══════════════════════════════════════════════════════════════════════════").cyan());
        println!("{}", style("  RECOMMENDED MODELS").bold().cyan());
        println!("{}", style("══════════════════════════════════════════════════════════════════════════").cyan());
        println!();
        
        let recommendations = self.get_recommended_models(use_case);
        
        // Group by category
        println!("{}", style("  🆓 FREE (Local/Ollama)").bold().green());
        println!("{}", style("  ─────────────────────────────────────────────────────────────────────").dim());
        for (model, desc) in recommendations.iter().filter(|(m, _)| m.starts_with("ollama/")) {
            println!("  {}  {}", style(format!("{:40}", model)).cyan(), style(desc).dim());
        }
        println!();
        
        println!("{}", style("  💰 PAID (Cloud APIs)").bold().yellow());
        println!("{}", style("  ─────────────────────────────────────────────────────────────────────").dim());
        for (model, desc) in recommendations.iter().filter(|(m, _)| !m.starts_with("ollama/") && !m.starts_with("g4f/")) {
            println!("  {}  {}", style(format!("{:40}", model)).cyan(), style(desc).dim());
        }
        println!();
        
        // Show free options
        let free_models: Vec<_> = recommendations.iter().filter(|(m, _)| m.starts_with("ollama/")).collect();
        if !free_models.is_empty() {
            println!("{}", style("  💡 TIP: Free models run locally via Ollama. Install: curl -fsSL https://ollama.com/install.sh | sh").yellow());
            println!();
        }
        
        // Ask if user wants to use a recommendation
        let use_recommendation = Confirm::new()
            .with_prompt("Select from recommended models?")
            .default(true)
            .interact()?;
        
        if use_recommendation {
            let rec_models: Vec<&str> = recommendations.iter().map(|(m, _)| m.as_str()).collect();
            let selection = Select::new()
                .with_prompt("Select a recommended model")
                .items(&rec_models)
                .default(0)
                .interact()?;
            
            let selected_model = recommendations[selection].0.clone();
            
            // Parse provider and model
            let parts: Vec<&str> = selected_model.split('/').collect();
            if parts.len() >= 2 {
                let provider = parts[0].to_string();
                let model = parts[1..].join("/");
                self.config.llm.provider = provider.clone();
                self.config.llm.model = model;
                
                println!();
                println!("[OK] Selected: {}", selected_model);
                
                // Prompt API key if needed
                if !provider.starts_with("ollama") && !provider.starts_with("g4f") {
                    self.prompt_api_key(&provider)?;
                } else {
                    println!();
                    println!("{}", style("Note: Ollama installation required:").yellow());
                    println!("   curl -fsSL https://ollama.com/install.sh | sh");
                    println!("   ollama pull {}", self.config.llm.model);
                }
            }
        }
        
        Ok(())
    }
    
    /// Get recommended models for a use case
    fn get_recommended_models(&self, use_case: &str) -> Vec<(String, String)> {
        match use_case {
            "code" => vec![
                // FREE - Local Models
                ("ollama/deepseek-coder-v2:236b".to_string(), "Best local code model (236B params)".to_string()),
                ("ollama/qwen2.5-coder:32b".to_string(), "Excellent code generation (32B)".to_string()),
                ("ollama/qwen2.5-coder:7b".to_string(), "Fast code assistant (7B)".to_string()),
                ("ollama/codellama:70b".to_string(), "Meta's code Llama (70B)".to_string()),
                ("ollama/codellama:34b".to_string(), "Balanced code model (34B)".to_string()),
                ("ollama/starcoder2:7b".to_string(), "Lightweight code model".to_string()),
                // PAID - Cloud APIs
                ("openai/gpt-4.1".to_string(), "Best overall for code".to_string()),
                ("openai/o3-mini".to_string(), "Efficient reasoning + code".to_string()),
                ("anthropic/claude-3-7-sonnet".to_string(), "Excellent code understanding".to_string()),
                ("anthropic/claude-sonnet-4".to_string(), "Latest Claude for coding".to_string()),
                ("deepseek/deepseek-coder-v3".to_string(), "Specialized code model".to_string()),
                ("mistral/codestral-2501".to_string(), "Mistral's code specialist".to_string()),
            ],
            "research" => vec![
                // FREE
                ("ollama/gemma4:31b".to_string(), "KERNEL DEFAULT - Great research".to_string()),
                ("ollama/llama3.3:70b".to_string(), "Excellent for analysis".to_string()),
                ("ollama/qwen2.5:72b".to_string(), "Strong research capabilities".to_string()),
                ("ollama/mixtral:8x22b".to_string(), "MoE for complex analysis".to_string()),
                // PAID
                ("openai/gpt-4.1".to_string(), "Best for research".to_string()),
                ("anthropic/claude-opus-4".to_string(), "Deepest analysis".to_string()),
                ("anthropic/claude-3-7-sonnet".to_string(), "Extended thinking mode".to_string()),
                ("google/gemini-2.5-pro-preview".to_string(), "1M context for documents".to_string()),
                ("perplexity/sonar-pro".to_string(), "Research + web search".to_string()),
            ],
            "reasoning" => vec![
                // FREE
                ("ollama/deepseek-r1:671b".to_string(), "Best local reasoning (671B)".to_string()),
                ("ollama/deepseek-r1:70b".to_string(), "Reasoning distill (70B)".to_string()),
                ("ollama/qwq:32b".to_string(), "Qwen reasoning model".to_string()),
                ("ollama/deepseek-r1:32b".to_string(), "Reasoning distill (32B)".to_string()),
                ("ollama/deepseek-r1:7b".to_string(), "Lightweight reasoning".to_string()),
                // PAID
                ("openai/o1".to_string(), "Best reasoning model".to_string()),
                ("openai/o1-pro".to_string(), "Professional reasoning".to_string()),
                ("openai/o3-mini".to_string(), "Efficient reasoning".to_string()),
                ("anthropic/claude-3-7-sonnet".to_string(), "Extended thinking".to_string()),
                ("deepseek/deepseek-r1".to_string(), "DeepSeek R1 reasoning".to_string()),
            ],
            "chat" => vec![
                // FREE
                ("ollama/gemma4:31b".to_string(), "KERNEL DEFAULT - Great chat".to_string()),
                ("ollama/llama3.3:70b".to_string(), "Excellent conversation".to_string()),
                ("ollama/mistral:7b".to_string(), "Fast and capable".to_string()),
                ("ollama/gemma2:27b".to_string(), "Good conversation".to_string()),
                ("ollama/phi4:14b".to_string(), "Efficient chat model".to_string()),
                // PAID
                ("openai/gpt-4o".to_string(), "Best general purpose".to_string()),
                ("openai/gpt-4o-mini".to_string(), "Fast and cheap".to_string()),
                ("anthropic/claude-3-5-sonnet".to_string(), "Natural conversation".to_string()),
                ("google/gemini-2.0-flash".to_string(), "Fast multimodal".to_string()),
                ("xai/grok-3".to_string(), "X.AI's latest".to_string()),
            ],
            "content" => vec![
                // FREE
                ("ollama/gemma4:31b".to_string(), "Creative writing".to_string()),
                ("ollama/llama3.3:70b".to_string(), "Content generation".to_string()),
                ("ollama/qwen2.5:72b".to_string(), "Writing assistant".to_string()),
                // PAID
                ("openai/gpt-4.5-preview".to_string(), "Best for content".to_string()),
                ("anthropic/claude-3-7-sonnet".to_string(), "Excellent writer".to_string()),
                ("google/gemini-2.5-pro-preview".to_string(), "Creative + long context".to_string()),
            ],
            "data" => vec![
                // FREE
                ("ollama/qwen2.5:72b".to_string(), "Data analysis".to_string()),
                ("ollama/deepseek-v3:671b".to_string(), "Complex data tasks".to_string()),
                ("ollama/gemma4:31b".to_string(), "Data processing".to_string()),
                // PAID
                ("openai/gpt-4.1".to_string(), "Best for data".to_string()),
                ("anthropic/claude-3-7-sonnet".to_string(), "Data analysis".to_string()),
                ("google/gemini-2.5-pro-preview".to_string(), "Large datasets".to_string()),
            ],
            "automation" => vec![
                // FREE
                ("ollama/gemma4:31b".to_string(), "KERNEL DEFAULT".to_string()),
                ("ollama/llama3.3:70b".to_string(), "Task automation".to_string()),
                ("ollama/qwen2.5:72b".to_string(), "Workflow automation".to_string()),
                // PAID
                ("openai/gpt-4.1".to_string(), "Best automation".to_string()),
                ("anthropic/claude-3-7-sonnet".to_string(), "Reliable automation".to_string()),
                ("groq/llama-3.3-70b-versatile".to_string(), "Fast automation".to_string()),
            ],
            "security" => vec![
                // FREE
                ("ollama/deepseek-r1:70b".to_string(), "Security analysis".to_string()),
                ("ollama/qwen2.5-coder:32b".to_string(), "Code audit".to_string()),
                ("ollama/gemma4:31b".to_string(), "General security".to_string()),
                // PAID
                ("openai/o1".to_string(), "Best security analysis".to_string()),
                ("openai/gpt-4.1".to_string(), "Penetration testing".to_string()),
                ("anthropic/claude-3-7-sonnet".to_string(), "Security audit".to_string()),
            ],
            _ => vec![
                // All - show top picks from each category
                ("ollama/gemma4:31b".to_string(), "KERNEL DEFAULT - Best all-around".to_string()),
                ("ollama/llama3.3:70b".to_string(), "Meta Llama 3.3 (70B)".to_string()),
                ("ollama/qwen2.5:72b".to_string(), "Alibaba Qwen 2.5 (72B)".to_string()),
                ("ollama/deepseek-r1:671b".to_string(), "DeepSeek R1 Reasoning".to_string()),
                ("openai/gpt-4.1".to_string(), "OpenAI GPT-4.1".to_string()),
                ("anthropic/claude-3-7-sonnet".to_string(), "Anthropic Claude 3.7".to_string()),
                ("google/gemini-2.5-pro-preview".to_string(), "Google Gemini 2.5".to_string()),
                ("xai/grok-3".to_string(), "X.AI Grok 3".to_string()),
            ],
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum SetupMode {
    QuickStart,
    Manual,
    Exit,
}

impl Default for SetupWizard {
    fn default() -> Self {
        Self::new()
    }
}
