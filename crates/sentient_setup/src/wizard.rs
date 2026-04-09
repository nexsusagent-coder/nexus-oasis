//! Setup Wizard - Modern Interactive TUI Sihirbazı v6.0.0
//! Strategic UX Update: Sentient Onboarding
//! 
//! Features:
//! - Security Warning (Multi-user lockdown)
//! - QuickStart vs Manual mode selection
//! - 25+ LLM Providers with hidden API key input
//! - 20+ Communication Channels with security policies
//! - Web Search Tools (SearXNG, DuckDuckGo, Ollama)
//! - Skip option on all steps

use dialoguer::{Input, Select, Confirm, Password, MultiSelect, FuzzySelect};
use console::{style, Emoji, Term};
use indicatif::{ProgressBar, ProgressStyle};
use std::collections::HashMap;

use crate::{SetupStatus, SetupResult, SetupConfig, IntegrationConfig};

// Emojis used in UI
static CHECK: Emoji<'_, '_> = Emoji("✅", "[OK]");
static ARROW: Emoji<'_, '_> = Emoji("→", "->");
static WARNING: Emoji<'_, '_> = Emoji("⚠️", "[!]");
static SKIP: Emoji<'_, '_> = Emoji("⏭️", "[skip]");

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
    
    /// Kurulum sihirbazını başlat - Modern TUI
    pub async fn run(&mut self) -> anyhow::Result<SetupResult> {
        self.print_welcome();
        
        // ADIM 0: Güvenlik Uyarısı
        if !self.show_security_warning()? {
            self.print_goodbye();
            return Ok(SetupResult {
                status: SetupStatus::NotStarted,
                config_path: self.config.config_path.clone(),
                integrations_enabled: vec![],
                warnings: vec!["Setup aborted by user".to_string()],
            });
        }
        
        // ADIM 1: Setup Mode Selection (QuickStart vs Manual)
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
    // ADIM 0: Güvenlik Uyarısı
    // ═══════════════════════════════════════════════════════════════════════════════
    
    fn show_security_warning(&self) -> anyhow::Result<bool> {
        println!();
        println!("{}", style("╔════════════════════════════════════════════════════════════════════════════════╗").red());
        println!("{}", style("║                                                                                ║").red());
        println!("{} {} {}", style("║").red(), style("   ⚠️  GÜVENLİK UYARISI / SECURITY WARNING                                   ").bold().yellow(), style("║").red());
        println!("{}", style("║                                                                                ║").red());
        println!("{}", style("╚════════════════════════════════════════════════════════════════════════════════╝").red());
        println!();
        
        println!("{}", style("🔒 Bu sistem varsayılan olarak KİŞİSELDİR.").bold().yellow());
        println!("   This system is PERSONAL by default.");
        println!();
        println!("{}", style("   Çoklu kullanım için LOCK-DOWN (kilit) gereklidir.").red());
        println!("   Multi-user access requires LOCK-DOWN mode.");
        println!();
        println!("   • Kişisel mod: Tüm verilere tam erişim");
        println!("   • Lock-down mod: Kısıtlı erişim, audit log aktif");
        println!();
        
        let confirm = Confirm::new()
            .with_prompt("Devam etmek istiyor musunuz? / Do you want to continue?")
            .default(true)
            .interact()?;
        
        if confirm {
            println!();
            println!("{} Güvenlik uyarısı kabul edildi. Devam ediliyor...", CHECK);
        }
        
        Ok(confirm)
    }
    
    // ═══════════════════════════════════════════════════════════════════════════════
    // ADIM 1: Setup Mode Selection
    // ═══════════════════════════════════════════════════════════════════════════════
    
    fn select_setup_mode(&mut self) -> anyhow::Result<SetupMode> {
        self.print_step("Kurulum Modu Seçimi / Setup Mode Selection");
        
        println!();
        println!("{}", style("🎯 Kurulum modunu seçin:").bold().cyan());
        println!();
        println!("   ⚡ QuickStart    - Hızlı başlangıç");
        println!("                     Port: 18789, Loopback, Token Auth");
        println!("                     Önerilen: İlk kez kurulum");
        println!();
        println!("   🔧 Manual       - Tam kontrol");
        println!("                     Tüm ayarları özelleştir");
        println!("                     Önerilen: Deneyimli kullanıcılar");
        println!();
        
        let options = vec![
            "⚡ QuickStart (Önerilen / Recommended)",
            "🔧 Manual - Tam Yapılandırma",
            "🚪 İptal / Cancel",
        ];
        
        let selection = Select::new()
            .with_prompt("Seçiminiz (↑↓ ok tuşları, Enter onayla)")
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
        self.total_steps = 4;
        
        // Set QuickStart defaults
        self.config.dashboard.port = 18789;
        self.config.dashboard.host = "127.0.0.1".to_string(); // Loopback only
        self.config.permissions.require_confirmation = true; // Token auth
        
        // ADIM 2: LLM Provider (25+)
        self.step = 1;
        self.print_step("🤖 LLM Provider Seçimi");
        if self.configure_llm_provider()? == StepResult::Skipped {
            println!("  {} LLM yapılandırması atlandı", SKIP);
        }
        
        // ADIM 3: İletişim Kanalları (20+)
        self.step = 2;
        self.print_step("💬 İletişim Kanalları");
        if self.configure_communication_channels()? == StepResult::Skipped {
            println!("  {} Kanal yapılandırması atlandı", SKIP);
        }
        
        // ADIM 4: Araçlar (Web Search)
        self.step = 3;
        self.print_step("🔧 Araçlar / Tools");
        if self.configure_tools()? == StepResult::Skipped {
            println!("  {} Araç yapılandırması atlandı", SKIP);
        }
        
        // ADIM 5: Kaydet
        self.step = 4;
        self.print_step("💾 Yapılandırma Kaydediliyor");
        self.save_and_show_success()?;
        
        Ok(())
    }
    
    // ═══════════════════════════════════════════════════════════════════════════════
    // Manual Setup
    // ═══════════════════════════════════════════════════════════════════════════════
    
    async fn manual_setup(&mut self) -> anyhow::Result<()> {
        self.total_steps = 6;
        
        // ADIM 2: Dil Seçimi
        self.step = 1;
        self.print_step("🌍 Dil / Language");
        if self.select_language()? == StepResult::Skipped {
            self.config.language = "tr".to_string();
        }
        
        // ADIM 3: LLM Provider (25+)
        self.step = 2;
        self.print_step("🤖 LLM Provider");
        if self.configure_llm_provider()? == StepResult::Skipped {
            println!("  {} LLM yapılandırması atlandı", SKIP);
        }
        
        // ADIM 4: İletişim Kanalları (20+)
        self.step = 3;
        self.print_step("💬 İletişim Kanalları");
        if self.configure_communication_channels()? == StepResult::Skipped {
            println!("  {} Kanal yapılandırması atlandı", SKIP);
        }
        
        // ADIM 5: Araçlar
        self.step = 4;
        self.print_step("🔧 Araçlar / Tools");
        if self.configure_tools()? == StepResult::Skipped {
            println!("  {} Araç yapılandırması atlandı", SKIP);
        }
        
        // ADIM 6: İzinler
        self.step = 5;
        self.print_step("🔐 İzinler / Permissions");
        if self.configure_permissions()? == StepResult::Skipped {
            println!("  {} İzin yapılandırması atlandı", SKIP);
        }
        
        // ADIM 7: Kaydet
        self.step = 6;
        self.print_step("💾 Kaydediliyor");
        self.save_and_show_success()?;
        
        Ok(())
    }
    
    // ═══════════════════════════════════════════════════════════════════════════════
    // LLM Provider Configuration (25+ Providers)
    // ═══════════════════════════════════════════════════════════════════════════════
    
    fn configure_llm_provider(&mut self) -> anyhow::Result<StepResult> {
        println!();
        println!("{}", style("🤖 LLM Provider Seçimi - 25+ Sağlayıcı").bold().cyan());
        println!("{}", style("   Fuzzy search aktif - yazarak arayın").dim());
        println!();
        
        // 25+ LLM Providers list
        let providers = vec![
            // Major Cloud Providers
            ("anthropic", "Claude 3.5 Sonnet", "🟠 Anthropic - Claude 3.5 Sonnet (En güçlü)"),
            ("anthropic", "claude-3-opus", "🟠 Anthropic - Claude 3 Opus"),
            ("anthropic", "claude-3-haiku", "🟠 Anthropic - Claude 3 Haiku (Hızlı)"),
            ("openai", "gpt-4o", "🟢 OpenAI - GPT-4o (Multimodal)"),
            ("openai", "gpt-4-turbo", "🟢 OpenAI - GPT-4 Turbo"),
            ("openai", "gpt-3.5-turbo", "🟢 OpenAI - GPT-3.5 Turbo (Ekonomik)"),
            ("google", "gemini-2.0-flash", "🔵 Google - Gemini 2.0 Flash"),
            ("google", "gemini-pro", "🔵 Google - Gemini Pro"),
            ("google", "gemini-1.5-pro", "🔵 Google - Gemini 1.5 Pro (1M context)"),
            
            // OpenRouter - Universal Gateway
            ("openrouter", "openrouter/auto", "🌐 OpenRouter - Auto (Akıllı routing)"),
            ("openrouter", "anthropic/claude-3.5-sonnet", "🌐 OpenRouter - Claude 3.5 Sonnet"),
            ("openrouter", "openai/gpt-4o", "🌐 OpenRouter - GPT-4o"),
            ("openrouter", "google/gemini-pro", "🌐 OpenRouter - Gemini Pro"),
            ("openrouter", "meta-llama/llama-3.1-405b", "🌐 OpenRouter - Llama 3.1 405B"),
            
            // Local/Free Providers
            ("ollama", "llama3.3:70b", "🦙 Ollama - Llama 3.3 70B (Yerel/Ücretsiz)"),
            ("ollama", "qwen2.5-coder:7b", "🦙 Ollama - Qwen 2.5 Coder 7B (Coding)"),
            ("ollama", "deepseek-r1:67b", "🦙 Ollama - DeepSeek R1 (Reasoning)"),
            ("ollama", "gemma3:27b", "🦙 Ollama - Gemma 3 27B"),
            ("ollama", "mistral:24b", "🦙 Ollama - Mistral 24B"),
            
            // Fast/Edge Providers
            ("groq", "llama-3.3-70b-versatile", "⚡ Groq - Llama 3.3 70B (Ultra hızlı)"),
            ("groq", "mixtral-8x7b-32768", "⚡ Groq - Mixtral 8x7B"),
            
            // Chinese Providers
            ("deepseek", "deepseek-chat", "🇨🇳 DeepSeek - Chat"),
            ("deepseek", "deepseek-reasoner", "🇨🇳 DeepSeek - Reasoner (R1)"),
            ("moonshot", "moonshot-v1-8k", "🇨🇳 Moonshot (Kimi) - 8K"),
            ("zhipu", "glm-4", "🇨🇳 ZhipuAI - GLM-4"),
            
            // Specialized Providers
            ("mistral", "mistral-large-latest", "🔴 Mistral AI - Large"),
            ("mistral", "codestral-latest", "🔴 Mistral AI - Codestral (Code)"),
            ("together", "meta-llama/Llama-3-70b-chat-hf", "🤝 Together AI - Llama 3 70B"),
            ("perplexity", "llama-3.1-sonar-large-128k-online", "🔮 Perplexity - Sonar (Search)"),
            
            // Enterprise Providers  
            ("azure", "gpt-4", "☁️ Azure OpenAI - GPT-4"),
            ("bedrock", "anthropic.claude-v3", "☁️ AWS Bedrock - Claude"),
            ("vertex", "gemini-pro", "☁️ Google Vertex - Gemini"),
            
            // Custom
            ("custom", "custom", "🎯 Custom Provider - Kendi API'niz"),
            
            // Skip option
            ("__skip__", "", "⏭️ Şimdilik Atla / Skip for now"),
        ];
        
        let provider_names: Vec<&str> = providers.iter().map(|(_, _, display)| *display).collect();
        
        // Fuzzy select for easy search
        let selection = FuzzySelect::new()
            .with_prompt("Provider seçin (yazarak arayın, ↑↓ gezinin)")
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
        println!("{} Seçilen: {} ({})", CHECK, default_model, provider_id);
        
        // API Key input (hidden) - except for local Ollama
        if provider_id != "ollama" {
            self.prompt_api_key(provider_id)?;
        } else {
            println!();
            println!("{}", style("💡 Ollama kurulumu gereklidir:").yellow());
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
        println!("{}", style("🔑 API Key girişi (gizli/hidden input)").bold().yellow());
        
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
            println!("{} API Key kaydedildi (gizli)", CHECK);
        } else {
            println!("{} API Key girilmedi - daha sonra ekleyebilirsiniz", WARNING);
        }
        
        Ok(())
    }
    
    fn select_model_variant(&mut self, provider: &str) -> anyhow::Result<()> {
        println!();
        
        let confirm = Confirm::new()
            .with_prompt("Farklı bir model seçmek ister misiniz?")
            .default(false)
            .interact()?;
        
        if !confirm {
            return Ok(());
        }
        
        let models: Vec<(&str, &str)> = match provider {
            "openrouter" => vec![
                ("openrouter/auto", "Auto - Akıllı routing"),
                ("anthropic/claude-3.5-sonnet", "Claude 3.5 Sonnet"),
                ("anthropic/claude-3-opus", "Claude 3 Opus"),
                ("openai/gpt-4o", "GPT-4o"),
                ("google/gemini-pro", "Gemini Pro"),
                ("meta-llama/llama-3.1-405b-instruct", "Llama 3.1 405B"),
                ("mistralai/mistral-large", "Mistral Large"),
                ("deepseek/deepseek-chat", "DeepSeek Chat"),
            ],
            "ollama" => vec![
                ("llama3.3:70b", "Llama 3.3 70B - Genel"),
                ("qwen2.5-coder:7b", "Qwen 2.5 Coder - Code"),
                ("qwen2.5:72b", "Qwen 2.5 72B - Güçlü"),
                ("deepseek-r1:67b", "DeepSeek R1 - Reasoning"),
                ("gemma3:27b", "Gemma 3 27B"),
                ("mistral:24b", "Mistral 24B"),
                ("codellama:34b", "Code Llama 34B"),
                ("phi4:14b", "Phi-4 14B - Hafif"),
            ],
            "openai" => vec![
                ("gpt-4o", "GPT-4o - Multimodal"),
                ("gpt-4-turbo", "GPT-4 Turbo"),
                ("gpt-4", "GPT-4"),
                ("gpt-3.5-turbo", "GPT-3.5 Turbo - Ekonomik"),
                ("o1-preview", "o1 Preview - Reasoning"),
                ("o1-mini", "o1 Mini - Hızlı reasoning"),
            ],
            _ => return Ok(()),
        };
        
        let model_names: Vec<&str> = models.iter().map(|(_, desc)| *desc).collect();
        
        let selection = Select::new()
            .with_prompt("Model seçin")
            .items(&model_names)
            .default(0)
            .interact()?;
        
        let (model, _) = models[selection];
        self.config.llm.model = model.to_string();
        println!("{} Model: {}", CHECK, model);
        
        Ok(())
    }
    
    // ═══════════════════════════════════════════════════════════════════════════════
    // Communication Channels Configuration (20+)
    // ═══════════════════════════════════════════════════════════════════════════════
    
    fn configure_communication_channels(&mut self) -> anyhow::Result<StepResult> {
        println!();
        println!("{}", style("💬 İletişim Kanalları - 20+ Platform").bold().cyan());
        println!("{}", style("   Space: Seç/Kaldır    Enter: Onayla    ↑↓: Gezin").dim());
        println!();
        
        // 20+ Communication channels
        let channels = vec![
            // Mobile Messengers
            ("telegram", "✈️ Telegram Bot"),
            ("whatsapp", "📱 WhatsApp Business"),
            ("signal", "🔔 Signal"),
            ("imessage", "🍎 iMessage (macOS)"),
            ("wechat", "🇨🇳 WeChat"),
            ("line", "🇯🇵 LINE"),
            ("viber", "💜 Viber"),
            ("kakaotalk", "🇰🇷 KakaoTalk"),
            
            // Enterprise Platforms
            ("discord", "🎮 Discord"),
            ("slack", "💼 Slack"),
            ("ms_teams", "👥 Microsoft Teams"),
            ("google_chat", "💬 Google Chat"),
            ("webex", "📹 Webex"),
            ("zoom", "🎥 Zoom Chat"),
            ("mattermost", "🏢 Mattermost"),
            ("rocketchat", "🚀 Rocket.Chat"),
            
            // Decentralized / Privacy
            ("matrix", "🧮 Matrix/Element"),
            ("xmpp", "🔗 XMPP/Jabber"),
            ("session", "🛡️ Session"),
            ("wire", "🔒 Wire"),
            ("threema", "🔑 Threema"),
            ("nostr", "⚡ Nostr"),
            
            // Social Platforms
            ("twitter", "🐦 Twitter/X DM"),
            ("instagram", "📸 Instagram DM"),
            ("facebook", "📘 Facebook Messenger"),
            ("linkedin", "💼 LinkedIn"),
            ("reddit", "🤖 Reddit"),
            
            // Email & SMS
            ("email", "📧 Email (SMTP/IMAP)"),
            ("sms", "📲 SMS (Twilio)"),
            ("rcs", "💬 RCS (Rich Communication)"),
            
            // Web/API
            ("web", "🌐 Web Dashboard"),
            ("api", "🔌 REST API"),
        ];
        
        let channel_names: Vec<&str> = channels.iter().map(|(_, name)| *name).collect();
        
        // Multi-select with Space
        let selections = MultiSelect::new()
            .with_prompt("Kanalları seçin (Space ile seçim, Enter onayla)")
            .items(&channel_names)
            .defaults(&[false].repeat(channels.len())) // None selected by default
            .interact()?;
        
        // Check if nothing selected = skip
        if selections.is_empty() {
            // Ask if they want to skip
            let skip = Confirm::new()
                .with_prompt("Hiçbir kanal seçilmedi. Devam etmek istiyor musunuz?")
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
                println!("  {} Web Dashboard aktif", CHECK);
            }
            "api" => {
                self.config.integrations.extra.insert("api_enabled".to_string(), "true".to_string());
                println!("  {} REST API aktif", CHECK);
            }
            _ => {
                // Generic setup for other channels
                println!("  {} {} seçildi - yapılandırma sonradan", CHECK, channel);
                self.config.integrations.extra.insert(format!("{}_enabled", channel), "true".to_string());
            }
        }
        Ok(())
    }
    
    fn setup_telegram_full(&mut self) -> anyhow::Result<()> {
        println!();
        println!("{}", style("  ✈️ Telegram Bot Kurulumu").bold().cyan());
        println!("     1. Telegram'da @BotFather'ı bulun");
        println!("     2. /newbot gönderin ve talimatları izleyin");
        println!("     3. Token'ı kopyalayın");
        println!();
        
        // Bot Token
        let token = Password::new()
            .with_prompt("  🔑 Bot Token (gizli)")
            .allow_empty_password(true)
            .interact()?;
        
        if token.is_empty() {
            println!("  {} Token girilmedi, atlanıyor", SKIP);
            return Ok(());
        }
        
        // Security Policy
        println!();
        println!("{}", style("  🔒 Güvenlik Politikası:").bold().yellow());
        
        let policies = vec![
            "🔐 DM Pairing - Sadece eşleşmiş kullanıcılar",
            "📋 Allowlist - İzin verilen kullanıcı listesi",
            "🌐 Açık - Herkes kullanabilir (önerilmez)",
            "⏭️ Atla - Varsayılan (DM Pairing)",
        ];
        
        let policy_sel = Select::new()
            .with_prompt("  Güvenlik politikası seçin")
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
        
        println!("  {} Telegram bağlandı! (Policy: {})", CHECK, policy);
        
        // If allowlist, ask for users
        if policy == "allowlist" {
            let users: String = Input::new()
                .with_prompt("  İzin verilen Telegram User ID'leri (virgülle ayırın)")
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
        println!("{}", style("  📱 WhatsApp Business API").bold().cyan());
        println!("     https://business.facebook.com");
        println!();
        
        let phone_id: String = Input::new()
            .with_prompt("  Phone Number ID")
            .allow_empty(true)
            .interact_text()?;
        
        if phone_id.is_empty() {
            println!("  {} Atlanıyor", SKIP);
            return Ok(());
        }
        
        let token = Password::new()
            .with_prompt("  🔑 Access Token (gizli)")
            .allow_empty_password(true)
            .interact()?;
        
        // Security Policy
        let policies = vec![
            "🔐 Verified Numbers - Doğrulanmış numaralar",
            "📋 Allowlist - İzin listesi",
            "⏭️ Atla",
        ];
        
        let policy_sel = Select::new()
            .with_prompt("  Güvenlik politikası")
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
        
        println!("  {} WhatsApp bağlandı!", CHECK);
        Ok(())
    }
    
    fn setup_discord_full(&mut self) -> anyhow::Result<()> {
        println!();
        println!("{}", style("  🎮 Discord Bot").bold().cyan());
        println!("     https://discord.com/developers/applications");
        println!();
        
        let token = Password::new()
            .with_prompt("  🔑 Bot Token (gizli)")
            .allow_empty_password(true)
            .interact()?;
        
        if token.is_empty() {
            println!("  {} Atlanıyor", SKIP);
            return Ok(());
        }
        
        // Server selection
        let guild_id: String = Input::new()
            .with_prompt("  Server/Guild ID (isteğe bağlı)")
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
        
        println!("  {} Discord bağlandı!", CHECK);
        Ok(())
    }
    
    fn setup_slack_full(&mut self) -> anyhow::Result<()> {
        println!();
        println!("{}", style("  💼 Slack App").bold().cyan());
        println!("     https://api.slack.com/apps");
        println!();
        
        let token = Password::new()
            .with_prompt("  🔑 Bot Token (xoxb-...) (gizli)")
            .allow_empty_password(true)
            .interact()?;
        
        if token.is_empty() {
            println!("  {} Atlanıyor", SKIP);
            return Ok(());
        }
        
        let channel: String = Input::new()
            .with_prompt("  Default Channel (ör: #general)")
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
        
        println!("  {} Slack bağlandı!", CHECK);
        Ok(())
    }
    
    fn setup_matrix_full(&mut self) -> anyhow::Result<()> {
        println!();
        println!("{}", style("  🧮 Matrix/Element").bold().cyan());
        println!("     Element → Settings → Help & About → Access Token");
        println!();
        
        let homeserver: String = Input::new()
            .default("https://matrix-client.matrix.org".to_string())
            .with_prompt("  Homeserver URL")
            .interact_text()?;
        
        let token = Password::new()
            .with_prompt("  🔑 Access Token (gizli)")
            .allow_empty_password(true)
            .interact()?;
        
        if token.is_empty() {
            println!("  {} Atlanıyor", SKIP);
            return Ok(());
        }
        
        let room_id: String = Input::new()
            .with_prompt("  Default Room ID (isteğe bağlı)")
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
        
        println!("  {} Matrix bağlandı!", CHECK);
        Ok(())
    }
    
    fn setup_signal(&mut self) -> anyhow::Result<()> {
        println!();
        println!("{}", style("  🔔 Signal").bold().cyan());
        println!("     signal-cli veya signald gereklidir");
        println!();
        
        let phone: String = Input::new()
            .with_prompt("  Telefon numarası (+90...)")
            .allow_empty(true)
            .interact_text()?;
        
        if phone.is_empty() {
            println!("  {} Atlanıyor", SKIP);
            return Ok(());
        }
        
        self.config.integrations.signal = Some(IntegrationConfig {
            enabled: true,
            token: None,
            extra: [("phone".to_string(), phone)].into_iter().collect(),
        });
        
        println!("  {} Signal yapılandırıldı!", CHECK);
        Ok(())
    }
    
    fn setup_email_full(&mut self) -> anyhow::Result<()> {
        println!();
        println!("{}", style("  📧 Email (SMTP)").bold().cyan());
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
            .with_prompt("  🔑 Password/App Password (gizli)")
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
        
        println!("  {} Email bağlandı!", CHECK);
        Ok(())
    }
    
    fn setup_twitter(&mut self) -> anyhow::Result<()> {
        println!();
        println!("{}", style("  🐦 Twitter/X API").bold().cyan());
        println!("     https://developer.twitter.com");
        println!();
        
        let bearer = Password::new()
            .with_prompt("  🔑 Bearer Token (gizli)")
            .allow_empty_password(true)
            .interact()?;
        
        if bearer.is_empty() {
            println!("  {} Atlanıyor", SKIP);
            return Ok(());
        }
        
        self.config.integrations.twitter = Some(IntegrationConfig {
            enabled: true,
            token: Some(bearer),
            extra: HashMap::new(),
        });
        
        println!("  {} Twitter bağlandı!", CHECK);
        Ok(())
    }
    
    fn setup_linkedin(&mut self) -> anyhow::Result<()> {
        println!();
        println!("{}", style("  💼 LinkedIn API").bold().cyan());
        println!();
        
        let token = Password::new()
            .with_prompt("  🔑 Access Token (gizli)")
            .allow_empty_password(true)
            .interact()?;
        
        if token.is_empty() {
            println!("  {} Atlanıyor", SKIP);
            return Ok(());
        }
        
        self.config.integrations.linkedin = Some(IntegrationConfig {
            enabled: true,
            token: Some(token),
            extra: HashMap::new(),
        });
        
        println!("  {} LinkedIn bağlandı!", CHECK);
        Ok(())
    }
    
    fn setup_reddit(&mut self) -> anyhow::Result<()> {
        println!();
        println!("{}", style("  🤖 Reddit API").bold().cyan());
        println!("     https://www.reddit.com/prefs/apps");
        println!();
        
        let client_id: String = Input::new()
            .with_prompt("  Client ID")
            .allow_empty(true)
            .interact_text()?;
        
        if client_id.is_empty() {
            println!("  {} Atlanıyor", SKIP);
            return Ok(());
        }
        
        let secret = Password::new()
            .with_prompt("  🔑 Client Secret (gizli)")
            .allow_empty_password(true)
            .interact()?;
        
        self.config.integrations.reddit = Some(IntegrationConfig {
            enabled: true,
            token: Some(secret),
            extra: [("client_id".to_string(), client_id)].into_iter().collect(),
        });
        
        println!("  {} Reddit bağlandı!", CHECK);
        Ok(())
    }
    
    // ═══════════════════════════════════════════════════════════════════════════════
    // Tools Configuration (Web Search)
    // ═══════════════════════════════════════════════════════════════════════════════
    
    fn configure_tools(&mut self) -> anyhow::Result<StepResult> {
        println!();
        println!("{}", style("🔧 Araçlar / Tools").bold().cyan());
        println!();
        
        // Web Search Provider
        println!("{}", style("🔍 Web Search Provider:").bold());
        println!();
        
        let providers = vec![
            ("searxng", "🦊 SearXNG - Açık kaynak, kendi instance"),
            ("duckduckgo", "🦆 DuckDuckGo - Gizlilik odaklı"),
            ("ollama_web", "🦙 Ollama Web Search - Yerel LLM + Web"),
            ("google_cse", "🔍 Google Custom Search - Resmi API"),
            ("bing", "🅱️ Bing Search API - Microsoft"),
            ("tavily", "⚡ Tavily - AI-optimized search"),
            ("serper", "🔑 Serper - Google API wrapper"),
            ("brave", "🦁 Brave Search - Gizlilik"),
            ("kagi", "💎 Kagi - Premium search"),
            ("__skip__", "⏭️ Şimdilik Atla / Skip for now"),
        ];
        
        let provider_names: Vec<&str> = providers.iter().map(|(_, name)| *name).collect();
        
        let selection = Select::new()
            .with_prompt("Web search sağlayıcısı seçin")
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
        println!("{}", style("🛠️ Ek Araçlar:").bold());
        
        let tools = vec![
            ("code_exec", "💻 Code Execution (Python, JS, Shell)"),
            ("file_ops", "📁 File Operations"),
            ("web_browser", "🌐 Web Browser (automation)"),
            ("calendar", "📅 Calendar Integration"),
            ("contacts", "👥 Contacts Integration"),
        ];
        
        let tool_names: Vec<&str> = tools.iter().map(|(_, name)| *name).collect();
        
        let tool_selections = MultiSelect::new()
            .with_prompt("Ek araçları seçin (Space seçim)")
            .items(&tool_names)
            .defaults(&[true, true, false, false, false])
            .interact()?;
        
        for idx in tool_selections {
            let (key, _) = tools[idx];
            self.config.integrations.extra.insert(format!("tool_{}", key), "true".to_string());
            println!("  {} {} aktif", CHECK, key);
        }
        
        Ok(StepResult::Completed)
    }
    
    fn setup_web_search(&mut self, provider: &str) -> anyhow::Result<()> {
        match provider {
            "searxng" => {
                println!();
                println!("{}", style("  🦊 SearXNG Kurulumu").bold().cyan());
                println!("     https://searx.be veya kendi instance'ınız");
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
                println!("  {} DuckDuckGo kullanılacak (API key gereksiz)", CHECK);
            }
            "ollama_web" => {
                self.config.integrations.extra.insert("web_search_provider".to_string(), "ollama_web".to_string());
                println!("  {} Ollama Web Search aktif", CHECK);
            }
            "google_cse" => {
                println!();
                println!("{}", style("  🔍 Google Custom Search").bold().cyan());
                
                let api_key = Password::new()
                    .with_prompt("  🔑 API Key (gizli)")
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
                    .with_prompt("  🔑 Bing API Key (gizli)")
                    .allow_empty_password(true)
                    .interact()?;
                
                self.config.integrations.extra.insert("web_search_provider".to_string(), "bing".to_string());
                self.config.integrations.extra.insert("bing_api_key".to_string(), key);
            }
            "tavily" => {
                println!();
                let key = Password::new()
                    .with_prompt("  🔑 Tavily API Key (gizli)")
                    .allow_empty_password(true)
                    .interact()?;
                
                self.config.integrations.extra.insert("web_search_provider".to_string(), "tavily".to_string());
                self.config.integrations.extra.insert("tavily_api_key".to_string(), key);
            }
            "serper" => {
                println!();
                let key = Password::new()
                    .with_prompt("  🔑 Serper API Key (gizli)")
                    .allow_empty_password(true)
                    .interact()?;
                
                self.config.integrations.extra.insert("web_search_provider".to_string(), "serper".to_string());
                self.config.integrations.extra.insert("serper_api_key".to_string(), key);
            }
            "brave" => {
                println!();
                let key = Password::new()
                    .with_prompt("  🔑 Brave Search API Key (gizli)")
                    .allow_empty_password(true)
                    .interact()?;
                
                self.config.integrations.extra.insert("web_search_provider".to_string(), "brave".to_string());
                self.config.integrations.extra.insert("brave_api_key".to_string(), key);
            }
            "kagi" => {
                println!();
                let key = Password::new()
                    .with_prompt("  🔑 Kagi API Key (gizli)")
                    .allow_empty_password(true)
                    .interact()?;
                
                self.config.integrations.extra.insert("web_search_provider".to_string(), "kagi".to_string());
                self.config.integrations.extra.insert("kagi_api_key".to_string(), key);
            }
            _ => {}
        }
        
        println!("  {} Web search yapılandırıldı: {}", CHECK, provider);
        Ok(())
    }
    
    // ═══════════════════════════════════════════════════════════════════════════════
    // Language Selection
    // ═══════════════════════════════════════════════════════════════════════════════
    
    fn select_language(&mut self) -> anyhow::Result<StepResult> {
        let languages = vec![
            ("tr", "🇹🇷 Türkçe"),
            ("en", "🇺🇸 English"),
            ("de", "🇩🇪 Deutsch"),
            ("fr", "🇫🇷 Français"),
            ("es", "🇪🇸 Español"),
            ("zh", "🇨🇳 中文"),
            ("ja", "🇯🇵 日本語"),
            ("ko", "🇰🇷 한국어"),
            ("__skip__", "⏭️ Şimdilik Atla"),
        ];
        
        let lang_names: Vec<&str> = languages.iter().map(|(_, name)| *name).collect();
        
        let selection = Select::new()
            .with_prompt("🌍 Dil seçin")
            .items(&lang_names)
            .default(0)
            .interact()?;
        
        let (lang_code, _) = languages[selection];
        
        if lang_code == "__skip__" {
            return Ok(StepResult::Skipped);
        }
        
        self.config.language = lang_code.to_string();
        println!("{} Dil: {}", CHECK, lang_names[selection]);
        
        Ok(StepResult::Completed)
    }
    
    // ═══════════════════════════════════════════════════════════════════════════════
    // Permissions Configuration
    // ═══════════════════════════════════════════════════════════════════════════════
    
    fn configure_permissions(&mut self) -> anyhow::Result<StepResult> {
        println!();
        println!("{}", style("🔐 İzinler / Permissions").bold().cyan());
        println!();
        
        let levels = vec![
            ("1", "📖 Level 1 - Sadece Okuma"),
            ("2", "📝 Level 2 - Dosya İşlemleri"),
            ("3", "🖱️ Level 3 - GUI Kontrol (Agent-S3)"),
            ("4", "🚀 Level 4 - Tam Otonom"),
            ("5", "⚡ Level 5 - Sistem Yönetimi"),
            ("__skip__", "⏭️ Şimdilik Atla (Level 2)"),
        ];
        
        let level_names: Vec<&str> = levels.iter().map(|(_, desc)| *desc).collect();
        
        let selection = Select::new()
            .with_prompt("Yetki seviyesi seçin")
            .items(&level_names)
            .default(5) // Skip by default
            .interact()?;
        
        let (level, _) = levels[selection];
        
        if level == "__skip__" {
            self.config.permissions.default_level = 2;
            return Ok(StepResult::Skipped);
        }
        
        self.config.permissions.default_level = level.parse().unwrap_or(2);
        
        // GUI kontrol izni
        if selection >= 2 && selection < 5 {
            let gui_confirm = Confirm::new()
                .with_prompt("🖱️ Klavye/Fare kontrolüne izin verilsin mi?")
                .default(true)
                .interact()?;
            
            self.config.permissions.allow_gui_control = gui_confirm;
        }
        
        println!();
        println!("{} Yetki seviyesi: {}", CHECK, level);
        
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
        println!("{} {} {}", style("║").cyan(), style("   🐺 SENTIENT NEXUS OS - v6.0.0 SETUP WIZARD                           ").bold(), style("║").cyan());
        println!("{}", style("║                                                                                ║").cyan());
        println!("{} {} {}", style("║").cyan(), style("   ✨ Strategic UX Update: Sentient Onboarding                           ").bold().dim(), style("║").cyan());
        println!("{}", style("║                                                                                ║").cyan());
        println!("{}", style("║   🎮 Controls:                                                                 ║").cyan());
        println!("{}", style("║      ↑↓ Arrow Keys: Navigate     Space: Multi-Select     Enter: Confirm     ║").cyan());
        println!("{}", style("║      Esc: Back                  Type: Fuzzy Search                        ║").cyan());
        println!("{}", style("║                                                                                ║").cyan());
        println!("{}", style("╚════════════════════════════════════════════════════════════════════════════════╝").cyan());
        println!();
    }
    
    fn print_goodbye(&self) {
        println!();
        println!("{}", style("╔════════════════════════════════════════════════════════════════════════════════╗").green());
        println!("{}", style("║   🐺 Görüşmek üzere! SENTIENT seni bekliyor...                                ║").green());
        println!("{}", style("║                                                                                ║").green());
        println!("{}", style("║   📚 Dokümantasyon: https://docs.sentient-os.ai                              ║").green());
        println!("{}", style("║   💬 Discord: https://discord.gg/sentient                                    ║").green());
        println!("{}", style("╚════════════════════════════════════════════════════════════════════════════════╝").green());
        println!();
    }
    
    fn print_step(&self, title: &str) {
        let progress = format!("[{}/{}]", self.step, self.total_steps);
        println!();
        println!("{}", style(format!("{} {} {}", ARROW, progress, title)).bold().yellow());
        println!("{}", style("─".repeat(70)).dim());
    }
    
    fn save_and_show_success(&mut self) -> anyhow::Result<()> {
        let pb = ProgressBar::new(100);
        pb.set_style(
            ProgressStyle::with_template("{spinner:.green} {msg} [{bar:40.cyan/blue}] {percent}%")
                .unwrap()
                .progress_chars("━╸ ")
        );
        
        pb.set_message("Yapılandırma kaydediliyor...");
        pb.inc(30);
        
        // Save config
        self.config.save()?;
        pb.inc(40);
        
        // Simulate a bit of setup work
        pb.set_message("Varsayılanlar oluşturuluyor...");
        pb.inc(30);
        
        pb.finish_with_message("Kayıt tamamlandı!");
        
        println!();
        println!("{}", style("╔════════════════════════════════════════════════════════════════════════════════╗").green());
        println!("{}", style("║                                                                                ║").green());
        println!("{}", style("║   🎉 KURULUM BAŞARIYLA TAMAMLANDI!                                            ║").green());
        println!("{}", style("║                                                                                ║").green());
        
        // Model info
        let model_display = &self.config.llm.model;
        let provider_display = &self.config.llm.provider;
        println!("{} {}{} ║", style("║   🤖 Model: ").green(), style(model_display).yellow().bold(), " ".repeat(55usize.saturating_sub(model_display.len())));
        println!("{} {}{} ║", style("║   🔌 Provider: ").green(), style(provider_display).yellow().bold(), " ".repeat(52usize.saturating_sub(provider_display.len())));
        
        // Channels
        let enabled = self.get_enabled_integrations();
        if !enabled.is_empty() {
            let channels = enabled.join(", ");
            println!("{} {}{} ║", style("║   💬 Kanallar: ").green(), style(&channels).yellow(), " ".repeat(51usize.saturating_sub(channels.len())));
        }
        
        println!("{}", style("║                                                                                ║").green());
        
        // Config path
        let config_display = &self.config.config_path;
        println!("{} {}{} ║", style("║   📁 Config: ").green(), style(config_display).yellow(), " ".repeat(52usize.saturating_sub(config_display.len())));
        
        // Dashboard info
        println!("{} {}{} ║", style("║   🌐 Dashboard: ").green(), style(format!("http://{}:{}", self.config.dashboard.host, self.config.dashboard.port)).yellow(), " ".repeat(44));
        
        println!("{}", style("║                                                                                ║").green());
        println!("{}", style("║   🚀 Başlatmak için: sentient-shell                                          ║").green());
        println!("{}", style("║   📚 Dokümantasyon: https://docs.sentient-os.ai                              ║").green());
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
