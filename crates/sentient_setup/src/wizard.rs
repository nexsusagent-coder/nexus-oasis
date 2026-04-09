//! Setup Wizard - Modern Interactive TUI Sihirbazı v5.0.0
//! Arrow-key navigation, Space for multi-select, Enter to confirm
//! Universal Omni-Gateway & Full Channel Support

use dialoguer::{Input, Select, Confirm, Password, MultiSelect};
use console::{style, Emoji, Term};
use indicatif::{ProgressBar, ProgressStyle};
use std::collections::HashMap;

use crate::{SetupStatus, SetupResult, SetupConfig, IntegrationConfig};

// Emojiler
static WOLF: Emoji<'_, '_> = Emoji("🐺", "wolf");
static CHECK: Emoji<'_, '_> = Emoji("✅", "[OK]");
static CROSS: Emoji<'_, '_> = Emoji("❌", "[X]");
static ARROW: Emoji<'_, '_> = Emoji("→", "->");
static ROCKET: Emoji<'_, '_> = Emoji("🚀", ">>");
static GLOBE: Emoji<'_, '_> = Emoji("🌐", "[*]");
static SPARKLE: Emoji<'_, '_> = Emoji("✨", "*");
static GEAR: Emoji<'_, '_> = Emoji("⚙️", "[o]");

pub struct SetupWizard {
    config: SetupConfig,
    step: u8,
    total_steps: u8,
    term: Term,
}

impl SetupWizard {
    pub fn new() -> Self {
        Self {
            config: SetupConfig::default(),
            step: 0,
            total_steps: 7, // v5.0.0: Basitleştirilmiş akış
            term: Term::stdout(),
        }
    }
    
    /// Kurulum sihirbazını başlat - Modern TUI
    pub async fn run(&mut self) -> anyhow::Result<SetupResult> {
        self.print_welcome();
        
        // Ana menü döngüsü
        loop {
            self.step = 0;
            let action = self.show_main_menu()?;
            
            match action {
                MainMenuAction::QuickSetup => self.quick_setup().await?,
                MainMenuAction::FullSetup => self.full_setup().await?,
                MainMenuAction::OnlyLLM => self.configure_llm_only().await?,
                MainMenuAction::OnlyChannels => self.configure_channels_only().await?,
                MainMenuAction::ViewConfig => self.view_current_config()?,
                MainMenuAction::Exit => {
                    self.print_goodbye();
                    return Ok(SetupResult {
                        status: SetupStatus::NotStarted,
                        config_path: self.config.config_path.clone(),
                        integrations_enabled: vec![],
                        warnings: vec![],
                    });
                }
            }
        }
    }
    
    fn print_welcome(&self) {
        let _ = self.term.clear_screen();
        
        println!();
        println!("{}", style("╔════════════════════════════════════════════════════════════════════════════════╗").cyan());
        println!("{}", style("║                                                                                ║").cyan());
        println!("{} {} {}", style("║").cyan(), style("   🐺 SENTIENT NEXUS OS - v5.0.0 INTERACTIVE SETUP                          ").bold(), style("║").cyan());
        println!("{}", style("║                                                                                ║").cyan());
        println!("{}", style("║   ✨ Arrow Keys: Navigate    Space: Multi-Select    Enter: Confirm           ║").cyan());
        println!("{}", style("║                                                                                ║").cyan());
        println!("{}", style("╚════════════════════════════════════════════════════════════════════════════════╝").cyan());
        println!();
    }
    
    fn print_goodbye(&self) {
        println!();
        println!("{}", style("╔════════════════════════════════════════════════════════════════════════════════╗").green());
        println!("{}", style("║   🐺 Görüşmek üzere! SENTIENT seni bekliyor...                                ║").green());
        println!("{}", style("╚════════════════════════════════════════════════════════════════════════════════╝").green());
        println!();
    }
    
    fn print_step(&self, title: &str) {
        let progress = format!("[{}/{}]", self.step, self.total_steps);
        println!();
        println!("{}", style(format!("{} {} {}", ARROW, progress, title)).bold().yellow());
        println!("{}", style("─".repeat(70)).dim());
    }
    
    /// Ana Menü - Modern TUI
    fn show_main_menu(&self) -> anyhow::Result<MainMenuAction> {
        println!();
        println!("{}", style("🎯 Ne yapmak istersiniz?").bold().cyan());
        println!();
        
        let options = vec![
            "⚡ Quick Setup - Hızlı kurulum (önerilen)",
            "🔧 Full Setup - Tam yapılandırma",
            "🤖 Only LLM - Sadece yapay zeka modeli",
            "💬 Only Channels - Sadece mesajlaşma kanalları", 
            "📋 View Config - Mevcut yapılandırmayı görüntüle",
            "🚪 Exit - Çıkış",
        ];
        
        let selection = Select::new()
            .with_prompt("Seçiminiz (↑↓ ok tuşlarıyla gezinin, Enter ile onaylayın)")
            .items(&options)
            .default(0)
            .interact()?;
        
        Ok(match selection {
            0 => MainMenuAction::QuickSetup,
            1 => MainMenuAction::FullSetup,
            2 => MainMenuAction::OnlyLLM,
            3 => MainMenuAction::OnlyChannels,
            4 => MainMenuAction::ViewConfig,
            5 => MainMenuAction::Exit,
            _ => MainMenuAction::Exit,
        })
    }
    
    /// Hızlı Kurulum - 3 adımda tamamla
    async fn quick_setup(&mut self) -> anyhow::Result<()> {
        self.total_steps = 3;
        
        // Adım 1: LLM Seçimi
        self.step = 1;
        self.print_step("🤖 LLM Model Seçimi");
        self.configure_llm_quick()?;
        
        // Adım 2: Kanal Seçimi
        self.step = 2;
        self.print_step("💬 Mesajlaşma Kanalları");
        self.configure_channels_quick()?;
        
        // Adım 3: Kaydet
        self.step = 3;
        self.print_step("💾 Yapılandırma Kaydediliyor");
        self.save_and_show_success()?;
        
        Ok(())
    }
    
    /// Tam Kurulum - Tüm adımlar
    async fn full_setup(&mut self) -> anyhow::Result<()> {
        self.total_steps = 7;
        
        // Adım 1: Dil
        self.step = 1;
        self.print_step("🌍 Dil Seçimi");
        self.select_language()?;
        
        // Adım 2: LLM
        self.step = 2;
        self.print_step("🤖 LLM Provider");
        self.configure_llm()?;
        
        // Adım 3: API Keys
        self.step = 3;
        self.print_step("🔑 API Anahtarları");
        self.configure_api_keys()?;
        
        // Adım 4: Channels
        self.step = 4;
        self.print_step("💬 Mesajlaşma Kanalları");
        self.configure_channels()?;
        
        // Adım 5: Permissions
        self.step = 5;
        self.print_step("🔐 Yetkilendirme");
        self.configure_permissions()?;
        
        // Adım 6: Routing
        self.step = 6;
        self.print_step("🔀 Dynamic Routing");
        self.configure_routing_quick()?;
        
        // Adım 7: Save
        self.step = 7;
        self.print_step("💾 Kaydediliyor");
        self.save_and_show_success()?;
        
        Ok(())
    }
    
    /// Sadece LLM yapılandırması
    async fn configure_llm_only(&mut self) -> anyhow::Result<()> {
        self.total_steps = 2;
        
        self.step = 1;
        self.print_step("🤖 LLM Model Seçimi");
        self.configure_llm()?;
        
        self.step = 2;
        self.print_step("💾 Kaydediliyor");
        self.save_and_show_success()?;
        
        Ok(())
    }
    
    /// Sadece kanal yapılandırması
    async fn configure_channels_only(&mut self) -> anyhow::Result<()> {
        self.total_steps = 2;
        
        self.step = 1;
        self.print_step("💬 Mesajlaşma Kanalları");
        self.configure_channels()?;
        
        self.step = 2;
        self.print_step("💾 Kaydediliyor");
        self.save_and_show_success()?;
        
        Ok(())
    }
    
    /// Mevcut yapılandırmayı görüntüle
    fn view_current_config(&self) -> anyhow::Result<()> {
        println!();
        println!("{}", style("📋 MEVCUT YAPIANDIRMA:").bold().cyan());
        println!();
        println!("  🤖 Provider: {}", style(&self.config.llm.provider).yellow());
        println!("  🎯 Model: {}", style(&self.config.llm.model).yellow());
        println!("  🌡️ Temperature: {}", self.config.llm.temperature);
        println!("  📊 Max Tokens: {}", self.config.llm.max_tokens);
        println!();
        
        // Aktif kanalları listele
        let enabled = self.get_enabled_integrations();
        if enabled.is_empty() {
            println!("  💬 Kanallar: {}", style("Hiçbiri aktif değil").dim());
        } else {
            println!("  💬 Aktif Kanallar: {}", style(enabled.join(", ")).green());
        }
        
        println!();
        println!("{}", style("Devam etmek için Enter'a basın...").dim());
        let _ = std::io::stdin().read_line(&mut String::new());
        
        Ok(())
    }
    
    // ═══════════════════════════════════════════════════════════════════════════════
    // LLM Configuration
    // ═══════════════════════════════════════════════════════════════════════════════
    
    fn select_language(&mut self) -> anyhow::Result<()> {
        let languages = vec![
            "🇹🇷 Türkçe",
            "🇺🇸 English",
            "🇩🇪 Deutsch",
            "🇫🇷 Français",
            "🇪🇸 Español",
        ];
        
        let selection = Select::new()
            .with_prompt("🌍 Dil seçin (↑↓ gezin, Enter onayla)")
            .items(&languages)
            .default(0)
            .interact()?;
        
        self.config.language = match selection {
            0 => "tr",
            1 => "en",
            2 => "de",
            3 => "fr",
            4 => "es",
            _ => "en",
        }.to_string();
        
        println!("{} Dil: {}", CHECK, languages[selection]);
        Ok(())
    }
    
    fn configure_llm_quick(&mut self) -> anyhow::Result<()> {
        println!("{}", style("🤖 LLM Model Seçimi - Hızlı Kurulum").bold());
        println!();
        
        // Kategoriler
        let categories = vec![
            "🏠 Yerel (Ollama) - Ücretsiz, Gizli",
            "☁️  Bulut (OpenAI, Claude, Gemini)",
            "⚡ Hızlı (Groq)",
            "🎯 Custom Provider",
        ];
        
        let cat_sel = Select::new()
            .with_prompt("Kategori seçin (↑↓ gezin, Enter onayla)")
            .items(&categories)
            .default(0)
            .interact()?;
        
        match cat_sel {
            0 => self.select_ollama_model()?,
            1 => self.select_cloud_model()?,
            2 => self.select_groq_model()?,
            3 => self.configure_custom_provider()?,
            _ => {}
        }
        
        println!();
        println!("{} Seçilen: {} ({})", CHECK, self.config.llm.model, self.config.llm.provider);
        Ok(())
    }
    
    fn select_ollama_model(&mut self) -> anyhow::Result<()> {
        let models = vec![
            ("gemma3:27b", "Gemma 3 27B - Dengeli, Güçlü"),
            ("llama3.3:70b", "Llama 3.3 70B - Genel Kullanım"),
            ("qwen2.5-coder:7b", "Qwen 2.5 Coder 7B - Coding ⚡"),
            ("qwen2.5:72b", "Qwen 2.5 72B - En Güçlü"),
            ("deepseek-r1:67b", "DeepSeek R1 - Reasoning"),
            ("mistral:24b", "Mistral 24B - Hızlı"),
            ("codellama:34b", "Code Llama 34B - Code Specialist"),
        ];
        
        let model_names: Vec<&str> = models.iter().map(|(name, desc)| *desc).collect();
        
        let selection = Select::new()
            .with_prompt("🦙 Ollama Modeli Seçin (↑↓ gezin, Enter onayla)")
            .items(&model_names)
            .default(0)
            .interact()?;
        
        let (model, _) = models[selection];
        self.config.llm.provider = "ollama".to_string();
        self.config.llm.model = model.to_string();
        
        println!();
        println!("{}", style("💡 Ollama kurulumu gereklidir:").yellow());
        println!("   curl -fsSL https://ollama.com/install.sh | sh");
        println!("   ollama pull {}", model);
        
        Ok(())
    }
    
    fn select_cloud_model(&mut self) -> anyhow::Result<()> {
        let providers = vec![
            ("openai", "gpt-4o", "OpenAI GPT-4o - Multimodal"),
            ("openai", "gpt-4-turbo", "OpenAI GPT-4 Turbo - Hızlı"),
            ("anthropic", "claude-3-opus-20240229", "Claude 3 Opus - En Güçlü"),
            ("anthropic", "claude-3-sonnet-20240229", "Claude 3 Sonnet - Dengeli"),
            ("google", "gemini-pro", "Google Gemini Pro"),
            ("google", "gemini-2.0-flash-exp", "Google Gemini 2.0 Flash"),
        ];
        
        let provider_names: Vec<&str> = providers.iter().map(|(_, _, desc)| *desc).collect();
        
        let selection = Select::new()
            .with_prompt("☁️ Bulut Modeli Seçin (↑↓ gezin, Enter onayla)")
            .items(&provider_names)
            .default(0)
            .interact()?;
        
        let (provider, model, _) = providers[selection];
        self.config.llm.provider = provider.to_string();
        self.config.llm.model = model.to_string();
        
        // API Key sor
        println!();
        let key_prompt = format!("🔑 {} API Key", provider.to_uppercase());
        let api_key = Password::new()
            .with_prompt(key_prompt)
            .allow_empty_password(true)
            .interact()?;
        
        if !api_key.is_empty() {
            match provider {
                "openai" => self.config.api_keys.openai = Some(api_key),
                "anthropic" => self.config.api_keys.anthropic = Some(api_key),
                "google" => self.config.api_keys.google = Some(api_key),
                _ => {}
            }
        }
        
        Ok(())
    }
    
    fn select_groq_model(&mut self) -> anyhow::Result<()> {
        let models = vec![
            ("llama-3.3-70b-versatile", "Llama 3.3 70B - Güçlü"),
            ("mixtral-8x7b-32768", "Mixtral 8x7B - Hızlı"),
            ("gemma2-9b-it", "Gemma 2 9B - Hafif"),
        ];
        
        let model_names: Vec<&str> = models.iter().map(|(_, desc)| *desc).collect();
        
        let selection = Select::new()
            .with_prompt("⚡ Groq Modeli Seçin (↑↓ gezin, Enter onayla)")
            .items(&model_names)
            .default(0)
            .interact()?;
        
        let (model, _) = models[selection];
        self.config.llm.provider = "groq".to_string();
        self.config.llm.model = model.to_string();
        
        println!();
        println!("{}", style("⚡ Groq API Key:").yellow());
        let api_key = Password::new()
            .with_prompt("🔑 Groq API Key")
            .allow_empty_password(true)
            .interact()?;
        
        if !api_key.is_empty() {
            self.config.api_keys.extra.insert("groq".to_string(), api_key);
        }
        
        Ok(())
    }
    
    fn configure_custom_provider(&mut self) -> anyhow::Result<()> {
        println!();
        println!("{}", style("🎯 Custom Provider - HERHANGİ bir API'ye bağlanın!").bold().cyan());
        println!();
        
        // Base URL
        let base_url: String = Input::new()
            .with_prompt("🌐 Base URL (ör: https://api.together.xyz/v1)")
            .interact_text()?;
        
        // API Key
        let api_key = Password::new()
            .with_prompt("🔑 API Key")
            .allow_empty_password(true)
            .interact()?;
        
        // Model
        let model: String = Input::new()
            .with_prompt("🤖 Model adı (ör: mistralai/Mixtral-8x7B-Instruct-v0.1)")
            .interact_text()?;
        
        self.config.llm.provider = "custom".to_string();
        self.config.llm.model = model.clone();
        self.config.llm.base_url = Some(base_url.clone());
        
        if !api_key.is_empty() {
            self.config.api_keys.extra.insert("custom".to_string(), api_key);
        }
        
        println!();
        println!("{} Custom provider eklendi!", CHECK);
        
        Ok(())
    }
    
    fn configure_llm(&mut self) -> anyhow::Result<()> {
        self.configure_llm_quick()
    }
    
    fn configure_api_keys(&mut self) -> anyhow::Result<()> {
        println!("{}", style("🔑 API Anahtarları").bold());
        println!();
        
        // Provider'a göre sor
        if self.config.llm.provider == "openai" && self.config.api_keys.openai.is_none() {
            let key = Password::new()
                .with_prompt("🔑 OpenAI API Key")
                .allow_empty_password(true)
                .interact()?;
            if !key.is_empty() {
                self.config.api_keys.openai = Some(key);
            }
        }
        
        if self.config.llm.provider == "anthropic" && self.config.api_keys.anthropic.is_none() {
            let key = Password::new()
                .with_prompt("🔑 Anthropic API Key")
                .allow_empty_password(true)
                .interact()?;
            if !key.is_empty() {
                self.config.api_keys.anthropic = Some(key);
            }
        }
        
        println!("{} API anahtarları güvenle saklanacak.", CHECK);
        Ok(())
    }
    
    // ═══════════════════════════════════════════════════════════════════════════════
    // Channel Configuration - Multi-Select TUI
    // ═══════════════════════════════════════════════════════════════════════════════
    
    fn configure_channels_quick(&mut self) -> anyhow::Result<()> {
        println!("{}", style("💬 Mesajlaşma Kanalları - Hızlı Seçim").bold());
        println!();
        println!("{}", style("Space: Seç/Kaldır    Enter: Onayla    ↑↓: Gezin").dim());
        println!();
        
        let channels = vec![
            ("telegram", "✈️ Telegram Bot"),
            ("whatsapp", "📱 WhatsApp Business"),
            ("discord", "🎮 Discord Bot"),
            ("slack", "💼 Slack"),
            ("matrix", "🧮 Matrix/Element"),
            ("email", "📧 Email (SMTP)"),
            ("web", "🌐 Web Arayüzü"),
            ("api", "🔌 REST API"),
        ];
        
        let channel_names: Vec<&str> = channels.iter().map(|(_, name)| *name).collect();
        
        // Multi-select with Space
        let selections = MultiSelect::new()
            .with_prompt("Kanalları seçin (Space ile çoklu seçim)")
            .items(&channel_names)
            .defaults(&[true, false, false, false, false, false, true, true]) // Web ve API varsayılan
            .interact()?;
        
        // Seçilen kanalları yapılandır
        for idx in selections {
            let (key, _) = channels[idx];
            
            match key {
                "telegram" => self.setup_telegram()?,
                "whatsapp" => self.setup_whatsapp()?,
                "discord" => self.setup_discord()?,
                "slack" => self.setup_slack()?,
                "matrix" => self.setup_matrix()?,
                "email" => self.setup_email()?,
                "web" => {
                    self.config.integrations.extra.insert("web_enabled".to_string(), "true".to_string());
                    println!("  {} Web arayüzü aktif", CHECK);
                }
                "api" => {
                    self.config.integrations.extra.insert("api_enabled".to_string(), "true".to_string());
                    println!("  {} REST API aktif", CHECK);
                }
                _ => {}
            }
        }
        
        Ok(())
    }
    
    fn configure_channels(&mut self) -> anyhow::Result<()> {
        self.configure_channels_quick()
    }
    
    // Kanal kurulum metodları
    fn setup_telegram(&mut self) -> anyhow::Result<()> {
        println!();
        println!("  ✈️ Telegram Bot Kurulumu");
        println!("     1. @BotFather'ı açın → /newbot");
        println!("     2. Token'ı alın");
        println!();
        
        let token = Password::new()
            .with_prompt("  🔑 Telegram Bot Token")
            .allow_empty_password(true)
            .interact()?;
        
        if !token.is_empty() {
            self.config.integrations.telegram = Some(IntegrationConfig {
                enabled: true,
                token: Some(token),
                extra: HashMap::new(),
            });
            println!("  {} Telegram bağlandı!", CHECK);
        } else {
            println!("  ⚠️ Token girilmedi, atlanıyor");
        }
        
        Ok(())
    }
    
    fn setup_whatsapp(&mut self) -> anyhow::Result<()> {
        println!();
        println!("  📱 WhatsApp Business API");
        println!("     https://business.facebook.com");
        println!();
        
        let phone_id: String = Input::new()
            .with_prompt("  Phone Number ID")
            .allow_empty(true)
            .interact_text()?;
        
        if !phone_id.is_empty() {
            let token = Password::new()
                .with_prompt("  Access Token")
                .allow_empty_password(true)
                .interact()?;
            
            self.config.integrations.whatsapp = Some(IntegrationConfig {
                enabled: true,
                token: Some(token),
                extra: [("phone_id".to_string(), phone_id)].into_iter().collect(),
            });
            println!("  {} WhatsApp bağlandı!", CHECK);
        }
        
        Ok(())
    }
    
    fn setup_discord(&mut self) -> anyhow::Result<()> {
        println!();
        println!("  🎮 Discord Bot");
        println!("     https://discord.com/developers/applications");
        println!();
        
        let token = Password::new()
            .with_prompt("  🔑 Discord Bot Token")
            .allow_empty_password(true)
            .interact()?;
        
        if !token.is_empty() {
            self.config.integrations.discord = Some(IntegrationConfig {
                enabled: true,
                token: Some(token),
                extra: HashMap::new(),
            });
            println!("  {} Discord bağlandı!", CHECK);
        }
        
        Ok(())
    }
    
    fn setup_slack(&mut self) -> anyhow::Result<()> {
        println!();
        println!("  💼 Slack App");
        println!("     https://api.slack.com/apps");
        println!();
        
        let token = Password::new()
            .with_prompt("  🔑 Slack Bot Token (xoxb-...)")
            .allow_empty_password(true)
            .interact()?;
        
        if !token.is_empty() {
            self.config.integrations.slack = Some(IntegrationConfig {
                enabled: true,
                token: Some(token),
                extra: HashMap::new(),
            });
            println!("  {} Slack bağlandı!", CHECK);
        }
        
        Ok(())
    }
    
    fn setup_matrix(&mut self) -> anyhow::Result<()> {
        println!();
        println!("  🧮 Matrix/Element");
        println!("     Element → Settings → Help → Access Token");
        println!();
        
        let homeserver: String = Input::new()
            .default("https://matrix-client.matrix.org".to_string())
            .with_prompt("  Homeserver URL")
            .interact_text()?;
        
        let token = Password::new()
            .with_prompt("  🔑 Access Token")
            .allow_empty_password(true)
            .interact()?;
        
        if !token.is_empty() {
            self.config.integrations.matrix = Some(IntegrationConfig {
                enabled: true,
                token: Some(token),
                extra: [("homeserver".to_string(), homeserver)].into_iter().collect(),
            });
            println!("  {} Matrix bağlandı!", CHECK);
        }
        
        Ok(())
    }
    
    fn setup_email(&mut self) -> anyhow::Result<()> {
        println!();
        println!("  📧 Email (SMTP)");
        println!();
        
        let host: String = Input::new()
            .default("smtp.gmail.com".to_string())
            .with_prompt("  SMTP Host")
            .interact_text()?;
        
        let user: String = Input::new()
            .with_prompt("  Email")
            .interact_text()?;
        
        let pass = Password::new()
            .with_prompt("  🔑 App Password")
            .interact()?;
        
        self.config.integrations.email = Some(IntegrationConfig {
            enabled: true,
            token: None,
            extra: [
                ("host".to_string(), host),
                ("user".to_string(), user),
                ("pass".to_string(), pass),
            ].into_iter().collect(),
        });
        println!("  {} Email bağlandı!", CHECK);
        
        Ok(())
    }
    
    // ═══════════════════════════════════════════════════════════════════════════════
    // Permissions Configuration
    // ═══════════════════════════════════════════════════════════════════════════════
    
    fn configure_permissions(&mut self) -> anyhow::Result<()> {
        println!("{}", style("🔐 Yetkilendirme Seviyesi").bold());
        println!();
        
        let levels = vec![
            ("1", "📖 Level 1 - Sadece Okuma"),
            ("2", "📝 Level 2 - Dosya İşlemleri"),
            ("3", "🖱️ Level 3 - GUI Kontrol (Agent-S3)"),
            ("4", "🚀 Level 4 - Tam Otonom"),
            ("5", "⚡ Level 5 - Sistem Yönetimi"),
        ];
        
        let level_names: Vec<&str> = levels.iter().map(|(_, desc)| *desc).collect();
        
        let selection = Select::new()
            .with_prompt("Yetki seviyesi seçin (↑↓ gezin, Enter onayla)")
            .items(&level_names)
            .default(1)
            .interact()?;
        
        let (level, _) = levels[selection];
        self.config.permissions.default_level = level.parse().unwrap_or(2);
        
        // GUI kontrol izni
        if selection >= 2 {
            let gui_confirm = Confirm::new()
                .with_prompt("🖱️ Klavye/Fare kontrolüne izin verilsin mi?")
                .default(true)
                .interact()?;
            
            self.config.permissions.allow_gui_control = gui_confirm;
        }
        
        println!();
        println!("{} Yetki seviyesi: {}", CHECK, level);
        
        Ok(())
    }
    
    // ═══════════════════════════════════════════════════════════════════════════════
    // Routing Configuration
    // ═══════════════════════════════════════════════════════════════════════════════
    
    fn configure_routing_quick(&mut self) -> anyhow::Result<()> {
        println!("{}", style("🔀 Dynamic Routing").bold());
        println!();
        
        let modes = vec![
            "🤖 Tam Otonom - Otomatik model seçimi",
            "👤 Onaylı - Her değişimde sor",
            "💰 Maliyet Odaklı - Ucuz modeller öncelikli",
        ];
        
        let selection = Select::new()
            .with_prompt("Routing modu seçin (↑↓ gezin, Enter onayla)")
            .items(&modes)
            .default(1)
            .interact()?;
        
        self.config.routing_mode.mode = match selection {
            0 => "fully_autonomous".to_string(),
            1 => "require_approval".to_string(),
            2 => "cost_optimized".to_string(),
            _ => "require_approval".to_string(),
        };
        
        println!();
        println!("{} Routing: {}", CHECK, self.config.routing_mode.mode);
        
        Ok(())
    }
    
    // ═══════════════════════════════════════════════════════════════════════════════
    // Save & Complete
    // ═══════════════════════════════════════════════════════════════════════════════
    
    fn save_and_show_success(&mut self) -> anyhow::Result<()> {
        let pb = ProgressBar::new(100);
        pb.set_style(
            ProgressStyle::with_template("{spinner:.green} {msg} [{bar:40.cyan/blue}] {percent}%")
                .unwrap()
                .progress_chars("━╸ ")
        );
        
        pb.set_message("Kaydediliyor...");
        pb.inc(50);
        
        self.config.save()?;
        pb.inc(50);
        
        pb.finish_with_message("Kayıt tamamlandı!");
        
        println!();
        println!("{}", style("╔════════════════════════════════════════════════════════════════════════════════╗").green());
        println!("{}", style("║                                                                                ║").green());
        println!("{}", style("║   🎉 KURULUM BAŞARIYLA TAMAMLANDI!                                            ║").green());
        println!("{}", style("║                                                                                ║").green());
        let model_display = format!("{}", self.config.llm.model);
        let provider_display = format!("{}", self.config.llm.provider);
        println!("{} {}{} ║", style("║   🤖 Model: ").green(), style(&model_display).yellow().bold(), " ".repeat(55 - model_display.len()));
        println!("{} {}{} ║", style("║   🔌 Provider: ").green(), style(&provider_display).yellow().bold(), " ".repeat(52 - provider_display.len()));
        
        let enabled = self.get_enabled_integrations();
        if !enabled.is_empty() {
            let channels = enabled.join(", ");
            println!("{} {}{} ║", style("║   💬 Kanallar: ").green(), style(&channels).yellow(), " ".repeat(51 - channels.len()));
        }
        
        println!("{}", style("║                                                                                ║").green());
        let config_display = format!("{}", self.config.config_path);
        println!("{} {}{} ║", style("║   📁 Config: ").green(), style(&config_display).yellow(), " ".repeat(52 - config_display.len()));
        println!("{}", style("║                                                                                ║").green());
        println!("{}", style("║   🚀 Başlatmak için: sentient-shell                                          ║").green());
        println!("{}", style("║   🌐 Dashboard: http://localhost:8080                                        ║").green());
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
enum MainMenuAction {
    QuickSetup,
    FullSetup,
    OnlyLLM,
    OnlyChannels,
    ViewConfig,
    Exit,
}

impl Default for SetupWizard {
    fn default() -> Self {
        Self::new()
    }
}
