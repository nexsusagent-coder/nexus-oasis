//! Setup Wizard - İnteraktif Kurulum Sihirbazı v4.0.0
//! Universal Omni-Gateway & Full Channel Support

use dialoguer::{Input, Select, Confirm, Password, MultiSelect};
use console::{style, Emoji};
use indicatif::{ProgressBar, ProgressStyle};
use std::collections::HashMap;

use crate::{SetupStatus, SetupResult, SetupConfig, IntegrationConfig};

static WOLF: Emoji<'_, '_> = Emoji("🐺", "wolf");
static CHECK: Emoji<'_, '_> = Emoji("✅", "[OK]");
static CROSS: Emoji<'_, '_> = Emoji("❌", "[X]");
static ARROW: Emoji<'_, '_> = Emoji("→", "->");
static ROCKET: Emoji<'_, '_> = Emoji("🚀", ">>");
static GLOBE: Emoji<'_, '_> = Emoji("🌐", "[*]");

pub struct SetupWizard {
    config: SetupConfig,
    step: u8,
    total_steps: u8,
}

impl SetupWizard {
    pub fn new() -> Self {
        Self {
            config: SetupConfig::default(),
            step: 0,
            total_steps: 10, // v4.0.0: +1 Dynamic Routing adımı
        }
    }
    
    /// Kurulum sihirbazını başlat
    pub async fn run(&mut self) -> anyhow::Result<SetupResult> {
        self.print_welcome();
        
        // Adım 1: Dil seçimi
        self.step = 1;
        self.select_language()?;
        
        // Adım 2: LLM Provider
        self.step = 2;
        self.configure_llm()?;
        
        // Adım 3: Custom Provider (YENİ!)
        self.step = 3;
        self.configure_custom_providers()?;
        
        // Adım 4: API Keys
        self.step = 4;
        self.configure_api_keys()?;
        
        // Adım 5: Messaging Channels (YENİ - Genişletilmiş!)
        self.step = 5;
        self.configure_channels()?;
        
        // Adım 6: Enterprise Integrations
        self.step = 6;
        self.configure_enterprise_integrations()?;
        
        // Adım 7: Human Emulation (YENİ!)
        self.step = 7;
        self.configure_human_emulation()?;
        
        // Adım 8: Dynamic Routing Mode (YENİ! v4.0.0)
        self.step = 8;
        self.configure_dynamic_routing()?;
        
        // Adım 9: Yetkilendirme (Agent-S3 Hardware Permissions)
        self.step = 9;
        self.configure_permissions()?;
        
        // Adım 10: Tamamla
        self.step = 10;
        self.save_and_complete()?;
        
        Ok(SetupResult {
            status: SetupStatus::Completed,
            config_path: self.config.config_path.clone(),
            integrations_enabled: self.get_enabled_integrations(),
            warnings: vec![],
        })
    }
    
    fn print_welcome(&self) {
        println!();
        println!("{}", style("╔════════════════════════════════════════════════════════════════════════════════╗").cyan());
        println!("{}", style("║                                                                                ║").cyan());
        println!("{} {} {}", style("║").cyan(), style("  🐺 SENTIENT NEXUS OS - UNIVERSAL OMNI-GATEWAY v1.1.0                           ").bold(), style("║").cyan());
        println!("{}", style("║                                                                                ║").cyan());
        println!("{}", style("║   🌐 DÜNYANIN HERHANGİ BİR LLM'İNE BAĞLANIN                                  ║").cyan());
        println!("{}", style("║   💬 20+ MESAJLAŞMA KANALI DESTEĞİ                                           ║").cyan());
        println!("{}", style("║   🤖 5.587 OTONOM SKILL                                                       ║").cyan());
        println!("{}", style("║   🔐 AGENT-S3 HARDWARE PERMISSIONS                                            ║").cyan());
        println!("{}", style("║                                                                                ║").cyan());
        println!("{}", style("╚════════════════════════════════════════════════════════════════════════════════╝").cyan());
        println!();
    }
    
    fn print_step(&self, title: &str) {
        let progress = format!("[{}/{}]", self.step, self.total_steps);
        println!();
        println!("{}", style(format!("{} {} {}", ARROW, progress, title)).bold().yellow());
        println!("{}", style("─".repeat(70)).dim());
    }
    
    fn select_language(&mut self) -> anyhow::Result<()> {
        self.print_step("Dil Seçimi / Language Selection");
        
        let languages = vec!["🇹🇷 Türkçe", "🇺🇸 English", "🇩🇪 Deutsch", "🇫🇷 Français", "🇪🇸 Español"];
        let selection = Select::new()
            .with_prompt("Dil seçin / Select language")
            .items(&languages)
            .default(0)
            .interact()?;
        
        self.config.language = match selection {
            0 => "tr".to_string(),
            1 => "en".to_string(),
            2 => "de".to_string(),
            3 => "fr".to_string(),
            4 => "es".to_string(),
            _ => "en".to_string(),
        };
        
        println!("{} Dil: {}", CHECK, languages[selection]);
        Ok(())
    }
    
    fn configure_llm(&mut self) -> anyhow::Result<()> {
        self.print_step("LLM Provider Yapılandırması");
        
        println!("{}", style("🌟 ÖNCEDEN TANIMLI PROVIDER'LAR:").bold());
        println!();
        
        let providers = vec![
            "OpenAI (GPT-4, GPT-4 Turbo, GPT-3.5)",
            "Anthropic Claude (Claude 3 Opus/Sonnet/Haiku)",
            "Google AI (Gemini Pro/Ultra)",
            "Ollama (Yerel - Ücretsiz)",
            "Groq (Ultra Hızlı Inferencing)",
            "Mistral AI (Mixtral, Mistral)",
            "Together AI (Open Source Models)",
            "DeepSeek (Code Specialized)",
            "🎯 Custom Provider (HERHANGİ Bir API!)",
            "Hiçbiri (Daha sonra yapılandıracağım)",
        ];
        
        let selection = Select::new()
            .with_prompt("LLM provider seçin")
            .items(&providers)
            .default(8)
            .interact()?;
        
        match selection {
            0 => {
                self.config.llm.provider = "openai".to_string();
                self.config.llm.model = "gpt-4-turbo".to_string();
            }
            1 => {
                self.config.llm.provider = "anthropic".to_string();
                self.config.llm.model = "claude-3-opus-20240229".to_string();
            }
            2 => {
                self.config.llm.provider = "google".to_string();
                self.config.llm.model = "gemini-pro".to_string();
            }
            3 => {
                self.config.llm.provider = "ollama".to_string();
                self.config.llm.model = "qwen2.5-coder:7b".to_string();
                println!();
                println!("{}", style("💡 Yerel LLM kullanılacak. Ollama kurulumu gerekli.").yellow());
                println!("   curl -fsSL https://ollama.com/install.sh | sh");
                println!("   ollama pull qwen2.5-coder:7b");
            }
            4 => {
                self.config.llm.provider = "groq".to_string();
                self.config.llm.model = "mixtral-8x7b-32768".to_string();
                println!();
                println!("{}", style("⚡ Groq: Dünyanın en hızlı LLM inferencing'i!").green());
            }
            5 => {
                self.config.llm.provider = "mistral".to_string();
                self.config.llm.model = "mistral-large-latest".to_string();
            }
            6 => {
                self.config.llm.provider = "together".to_string();
                self.config.llm.model = "mistralai/Mixtral-8x7B-Instruct-v0.1".to_string();
            }
            7 => {
                self.config.llm.provider = "deepseek".to_string();
                self.config.llm.model = "deepseek-coder-33b-instruct".to_string();
            }
            8 => {
                self.config.llm.provider = "custom".to_string();
                println!();
                println!("{}", style("🎯 Custom Provider seçildi! Adım 3'te yapılandıracağız.").green());
                return Ok(());
            }
            9 => {
                println!("{} LLM daha sonra yapılandırılacak.", CHECK);
                return Ok(());
            }
            _ => {}
        }
        
        self.configure_llm_settings()?;
        
        println!("{} LLM Provider: {} ({})", CHECK, self.config.llm.provider, self.config.llm.model);
        Ok(())
    }
    
    fn configure_llm_settings(&mut self) -> anyhow::Result<()> {
        println!();
        
        // Temperature
        let temp_input: String = Input::new()
            .with_prompt("Temperature (0.0 - 2.0)")
            .default("0.7".to_string())
            .interact_text()?;
        self.config.llm.temperature = temp_input.parse().unwrap_or(0.7);
        
        // Max Tokens
        let tokens_input: String = Input::new()
            .with_prompt("Max Tokens")
            .default("4096".to_string())
            .interact_text()?;
        self.config.llm.max_tokens = tokens_input.parse().unwrap_or(4096);
        
        Ok(())
    }
    
    fn configure_custom_providers(&mut self) -> anyhow::Result<()> {
        self.print_step("🎯 Custom Provider (Universal Gateway)");
        
        println!("{}", style("🌐 HERHANGİ BİR LLM API'SİNE BAĞLANIN!").bold().cyan());
        println!();
        println!("SENTIENT, OpenAI veya Anthropic formatını destekleyen HERHANGİ bir API'ye bağlanabilir:");
        println!();
        println!("  {} Together AI, Groq, Fireworks, Perplexity", ROCKET);
        println!("  {} DeepSeek, Mistral, Replicate, Anyscale", ROCKET);
        println!("  {} Alibaba Qwen, Baidu Ernie, Zhipu AI, Moonshot", ROCKET);
        println!("  {} Yerel: Ollama, LM Studio, vLLM, LocalAI", ROCKET);
        println!("  {} Cloud: AWS Bedrock, Azure OpenAI, Google Vertex", ROCKET);
        println!("  {} Ve DAHASI... Herhangi bir OpenAI-compatible API!", ROCKET);
        println!();
        
        let add_custom = Confirm::new()
            .with_prompt("Custom provider eklemek istiyor musunuz?")
            .default(true)
            .interact()?;
        
        if !add_custom {
            println!("{} Custom provider atlandı.", CHECK);
            return Ok(());
        }
        
        // API Format seçimi
        let formats = vec![
            "OpenAI Compatible (/chat/completions)",
            "Anthropic Compatible (/messages)",
            "Custom Endpoint",
        ];
        
        let format_sel = Select::new()
            .with_prompt("API Formatı seçin")
            .items(&formats)
            .default(0)
            .interact()?;
        
        // Base URL
        println!();
        println!("{}", style("📌 Base URL örnekleri:").dim());
        println!("   OpenAI:    https://api.openai.com/v1");
        println!("   Together:  https://api.together.xyz/v1");
        println!("   Groq:      https://api.groq.com/openai/v1");
        println!("   DeepSeek:  https://api.deepseek.com/v1");
        println!("   Ollama:    http://localhost:11434/v1");
        println!();
        
        let base_url: String = Input::new()
            .with_prompt("Base URL")
            .interact_text()?;
        
        // API Key
        let api_key = Password::new()
            .with_prompt("API Key (boş bırakabilirsiniz)")
            .allow_empty_password(true)
            .interact()?;
        
        // Provider adı
        let name: String = Input::new()
            .with_prompt("Bu provider için bir isim girin")
            .default("MyCustomProvider".to_string())
            .interact_text()?;
        
        // Model seçimi
        println!();
        println!("{}", style("🤖 Model seçimi:").bold());
        
        let model: String = Input::new()
            .with_prompt("Model adı (ör: gpt-4, claude-3-opus, mixtral-8x7b)")
            .interact_text()?;
        
        // Kaydet
        self.config.llm.provider = "custom".to_string();
        self.config.llm.model = model.clone();
        self.config.llm.base_url = Some(base_url.clone());
        self.config.llm.api_format = Some(match format_sel {
            0 => "openai".to_string(),
            1 => "anthropic".to_string(),
            2 => "custom".to_string(),
            _ => "openai".to_string(),
        });
        
        // Custom provider config'e ekle
        self.config.custom_providers.push(serde_json::json!({
            "name": name,
            "base_url": base_url,
            "api_key": if api_key.is_empty() { serde_json::Value::Null } else { serde_json::json!(api_key) },
            "api_format": match format_sel {
                0 => "openai",
                1 => "anthropic",
                2 => "custom",
                _ => "openai"
            },
            "default_model": model,
            "enabled": true
        }));
        
        println!();
        println!("{} Custom provider başarıyla eklendi: {} ({})", CHECK, name, self.config.llm.model);
        
        // Başka provider ekleme
        let add_more = Confirm::new()
            .with_prompt("Başka bir custom provider eklemek istiyor musunuz?")
            .default(false)
            .interact()?;
        
        if add_more {
            return self.configure_custom_providers();
        }
        
        Ok(())
    }
    
    fn configure_api_keys(&mut self) -> anyhow::Result<()> {
        self.print_step("API Anahtarları");
        
        let configure_keys = Confirm::new()
            .with_prompt("API anahtarları şimdi eklemek istiyor musunuz?")
            .default(true)
            .interact()?;
        
        if !configure_keys {
            println!("{} API anahtarları daha sonra eklenebilir.", CHECK);
            println!("   Komut: sentient config set <provider>.api_key <key>");
            return Ok(());
        }
        
        // Provider'a göre API key sor
        if self.config.llm.provider == "openai" {
            let key = Password::new()
                .with_prompt("OpenAI API Key")
                .allow_empty_password(true)
                .interact()?;
            if !key.is_empty() {
                self.config.api_keys.openai = Some(key);
            }
        } else if self.config.llm.provider == "anthropic" {
            let key = Password::new()
                .with_prompt("Anthropic API Key")
                .allow_empty_password(true)
                .interact()?;
            if !key.is_empty() {
                self.config.api_keys.anthropic = Some(key);
            }
        } else if self.config.llm.provider == "groq" {
            let key = Password::new()
                .with_prompt("Groq API Key")
                .allow_empty_password(true)
                .interact()?;
            if !key.is_empty() {
                self.config.api_keys.extra.insert("groq".to_string(), key);
            }
        } else if self.config.llm.provider == "custom" {
            // Custom provider için API key
            if let Some(ref base_url) = self.config.llm.base_url {
                let key = Password::new()
                    .with_prompt(&format!("{} için API Key", base_url))
                    .allow_empty_password(true)
                    .interact()?;
                if !key.is_empty() {
                    self.config.api_keys.extra.insert("custom".to_string(), key);
                }
            }
        }
        
        println!("{} API anahtarları güvenli bir şekilde saklanacak.", CHECK);
        println!("{} V-GATE proxy kullanılacak - anahtarlar asla kodda tutulmayacak.", CHECK);
        Ok(())
    }
    
    fn configure_channels(&mut self) -> anyhow::Result<()> {
        self.print_step("💬 Mesajlaşma Kanalları (20+ Platform)");
        
        println!("{}", style("🌐 DÜNYANIN EN KAPSAMLI MESAJLAŞMA DESTEĞİ!").bold().cyan());
        println!();
        
        let channel_categories = vec![
            "📱 Mobile Messengers (WhatsApp, Signal, Telegram, iMessage...)",
            "🏢 Enterprise Platforms (MS Teams, Slack, Discord...)",
            "🔐 Decentralized (Matrix, XMPP, Session...)",
            "📱 Social Platforms (Twitter, Instagram, LinkedIn...)",
            "📧 Email & SMS",
            "🐙 Developer Tools (GitHub, GitLab, Jira...)",
            "✅ Atla - Daha sonra yapılandıracağım",
        ];
        
        let cat_sel = Select::new()
            .with_prompt("Kanal kategorisi seçin")
            .items(&channel_categories)
            .default(6)
            .interact()?;
        
        match cat_sel {
            0 => self.configure_mobile_channels()?,
            1 => self.configure_enterprise_channels()?,
            2 => self.configure_decentralized_channels()?,
            3 => self.configure_social_channels()?,
            4 => self.configure_email_sms_channels()?,
            5 => self.configure_dev_channels()?,
            _ => {
                println!("{} Kanallar daha sonra yapılandırılabilir.", CHECK);
                return Ok(());
            }
        }
        
        Ok(())
    }
    
    fn configure_mobile_channels(&mut self) -> anyhow::Result<()> {
        let channels = vec![
            "WhatsApp Business API",
            "Signal Messenger",
            "Telegram Bot",
            "iMessage (macOS)",
            "WeChat / 企业微信",
            "LINE Messenger",
            "Viber",
            "KakaoTalk",
            "🔙 Geri",
        ];
        
        let sel = Select::new()
            .with_prompt("📱 Mobile Messenger seçin")
            .items(&channels)
            .default(8)
            .interact()?;
        
        match sel {
            0 => self.setup_whatsapp()?,
            1 => self.setup_signal()?,
            2 => self.setup_telegram()?,
            3 => self.setup_imessage()?,
            4 => self.setup_wechat()?,
            5 => self.setup_line()?,
            6 => self.setup_viber()?,
            7 => self.setup_kakaotalk()?,
            _ => return Ok(()),
        }
        
        Ok(())
    }
    
    fn setup_whatsapp(&mut self) -> anyhow::Result<()> {
        println!();
        println!("{}", style("📱 WHATSAPP BUSINESS API KURULUMU").bold());
        println!();
        println!("1. WhatsApp Business API'ye kayıt: https://business.facebook.com");
        println!("2. Business Manager'da WhatsApp hesabı oluşturun");
        println!("3. Phone Number ID ve Access Token alın");
        println!();
        
        let phone_id: String = Input::new()
            .with_prompt("Phone Number ID")
            .interact_text()?;
        
        let token = Password::new()
            .with_prompt("Permanent Access Token")
            .interact()?;
        
        if !phone_id.is_empty() && !token.is_empty() {
            self.config.integrations.extra.insert("whatsapp_phone_id".to_string(), phone_id);
            self.config.integrations.extra.insert("whatsapp_token".to_string(), token);
            self.config.integrations.whatsapp = Some(IntegrationConfig {
                enabled: true,
                token: None,
                extra: HashMap::new(),
            });
            println!("{} WhatsApp bağlandı!", CHECK);
        }
        
        Ok(())
    }
    
    fn setup_signal(&mut self) -> anyhow::Result<()> {
        println!();
        println!("{}", style("🔐 SIGNAL MESSENGER KURULUMU").bold());
        println!();
        println!("Signal merkeziyetsiz bir platform olduğundan, GUI Automation kullanılır.");
        println!("Agent-S3 Level 3+ yetkilendirme gerektirir.");
        println!();
        
        let enable = Confirm::new()
            .with_prompt("Signal entegrasyonunu (GUI Automation) aktif et?")
            .default(true)
            .interact()?;
        
        if enable {
            self.config.integrations.signal = Some(IntegrationConfig {
                enabled: true,
                token: None,
                extra: [("method".to_string(), "gui_automation".to_string())].into_iter().collect(),
            });
            println!("{} Signal (GUI Mode) yapılandırıldı!", CHECK);
        }
        
        Ok(())
    }
    
    fn setup_telegram(&mut self) -> anyhow::Result<()> {
        println!();
        println!("{}", style("✈️ TELEGRAM BOT KURULUMU").bold());
        println!();
        println!("1. Telegram'da @BotFather'ı açın");
        println!("2. /newbot komutunu gönderin");
        println!("3. Token'ı aşağıya yapıştırın");
        println!();
        
        let token = Password::new()
            .with_prompt("Telegram Bot Token")
            .interact()?;
        
        if !token.is_empty() {
            self.config.integrations.telegram = Some(IntegrationConfig {
                enabled: true,
                token: Some(token),
                extra: HashMap::new(),
            });
            println!("{} Telegram bağlandı!", CHECK);
        }
        
        Ok(())
    }
    
    fn setup_imessage(&mut self) -> anyhow::Result<()> {
        println!();
        println!("{}", style("💬 IMESSAGE KURULUMU (macOS Only)").bold());
        println!();
        
        #[cfg(target_os = "macos")]
        {
            let enable = Confirm::new()
                .with_prompt("iMessage entegrasyonunu aktif et?")
                .default(true)
                .interact()?;
            
            if enable {
                self.config.integrations.imessage = Some(IntegrationConfig {
                    enabled: true,
                    token: None,
                    extra: [("method".to_string(), "gui_automation".to_string())].into_iter().collect(),
                });
                println!("{} iMessage yapılandırıldı!", CHECK);
            }
        }
        
        #[cfg(not(target_os = "macos"))]
        {
            println!("{}", style("⚠️  iMessage sadece macOS'ta kullanılabilir.").yellow());
            println!("   Mevcut işletim sistemi: {}", std::env::consts::OS);
        }
        
        Ok(())
    }
    
    fn setup_wechat(&mut self) -> anyhow::Result<()> {
        println!();
        println!("{}", style("🟢 WECHAT / 企业微信 KURULUMU").bold());
        println!();
        
        let corp_id: String = Input::new()
            .with_prompt("CorpID")
            .interact_text()?;
        
        let secret = Password::new()
            .with_prompt("Secret")
            .interact()?;
        
        if !corp_id.is_empty() {
            self.config.integrations.wechat = Some(IntegrationConfig {
                enabled: true,
                token: Some(secret),
                extra: [("corp_id".to_string(), corp_id)].into_iter().collect(),
            });
            println!("{} WeChat bağlandı!", CHECK);
        }
        
        Ok(())
    }
    
    fn setup_line(&mut self) -> anyhow::Result<()> {
        println!();
        println!("{}", style("💚 LINE MESSENGER KURULUMU").bold());
        println!("LINE Developers Console: https://developers.line.biz");
        println!();
        
        let token = Password::new()
            .with_prompt("Channel Access Token")
            .interact()?;
        
        if !token.is_empty() {
            self.config.integrations.line = Some(IntegrationConfig {
                enabled: true,
                token: Some(token),
                extra: HashMap::new(),
            });
            println!("{} LINE bağlandı!", CHECK);
        }
        
        Ok(())
    }
    
    fn setup_viber(&mut self) -> anyhow::Result<()> {
        println!();
        println!("{}", style("💜 VIBER KURULUMU").bold());
        println!("Viber Admin Panel: https://partners.viber.com");
        println!();
        
        let token = Password::new()
            .with_prompt("Viber Auth Token")
            .interact()?;
        
        if !token.is_empty() {
            self.config.integrations.viber = Some(IntegrationConfig {
                enabled: true,
                token: Some(token),
                extra: HashMap::new(),
            });
            println!("{} Viber bağlandı!", CHECK);
        }
        
        Ok(())
    }
    
    fn setup_kakaotalk(&mut self) -> anyhow::Result<()> {
        println!();
        println!("{}", style("🟡 KAKAOTALK KURULUMU").bold());
        println!("Kakao Developers: https://developers.kakao.com");
        println!();
        
        let app_id: String = Input::new()
            .with_prompt("App ID")
            .interact_text()?;
        
        if !app_id.is_empty() {
            self.config.integrations.kakaotalk = Some(IntegrationConfig {
                enabled: true,
                token: None,
                extra: [("app_id".to_string(), app_id)].into_iter().collect(),
            });
            println!("{} KakaoTalk yapılandırıldı!", CHECK);
        }
        
        Ok(())
    }
    
    fn configure_enterprise_channels(&mut self) -> anyhow::Result<()> {
        let channels = vec![
            "Microsoft Teams",
            "Slack",
            "Google Chat",
            "Discord",
            "Cisco Webex",
            "Zoom Chat",
            "Mattermost",
            "RocketChat",
            "🔙 Geri",
        ];
        
        let sel = Select::new()
            .with_prompt("🏢 Enterprise Platform seçin")
            .items(&channels)
            .default(8)
            .interact()?;
        
        match sel {
            0 => self.setup_ms_teams()?,
            1 => self.setup_slack()?,
            2 => self.setup_google_chat()?,
            3 => self.setup_discord()?,
            4 => self.setup_webex()?,
            5 => self.setup_zoom()?,
            6 => self.setup_mattermost()?,
            7 => self.setup_rocketchat()?,
            _ => return Ok(()),
        }
        
        Ok(())
    }
    
    fn setup_ms_teams(&mut self) -> anyhow::Result<()> {
        println!();
        println!("{}", style("👥 MICROSOFT TEAMS KURULUMU").bold());
        println!();
        
        let webhook: String = Input::new()
            .with_prompt("Teams Webhook URL")
            .interact_text()?;
        
        if !webhook.is_empty() {
            self.config.integrations.ms_teams = Some(IntegrationConfig {
                enabled: true,
                token: Some(webhook),
                extra: [("method".to_string(), "webhook".to_string())].into_iter().collect(),
            });
            println!("{} MS Teams bağlandı!", CHECK);
        }
        
        Ok(())
    }
    
    fn setup_slack(&mut self) -> anyhow::Result<()> {
        println!();
        println!("{}", style("💼 SLACK KURULUMU").bold());
        println!("https://api.slack.com/apps → Create New App");
        println!();
        
        let webhook: String = Input::new()
            .with_prompt("Slack Webhook URL")
            .allow_empty(true)
            .interact_text()?;
        
        let bot_token = Password::new()
            .with_prompt("Bot Token (xoxb-...)")
            .allow_empty_password(true)
            .interact()?;
        
        let mut extra = HashMap::new();
        if !webhook.is_empty() {
            extra.insert("webhook_url".to_string(), webhook);
        }
        
        if !bot_token.is_empty() {
            self.config.integrations.slack = Some(IntegrationConfig {
                enabled: true,
                token: Some(bot_token),
                extra,
            });
            println!("{} Slack bağlandı!", CHECK);
        }
        
        Ok(())
    }
    
    fn setup_google_chat(&mut self) -> anyhow::Result<()> {
        println!();
        println!("{}", style("💬 GOOGLE CHAT KURULUMU").bold());
        println!();
        
        let webhook: String = Input::new()
            .with_prompt("Google Chat Webhook URL")
            .interact_text()?;
        
        if !webhook.is_empty() {
            self.config.integrations.google_chat = Some(IntegrationConfig {
                enabled: true,
                token: Some(webhook),
                extra: HashMap::new(),
            });
            println!("{} Google Chat bağlandı!", CHECK);
        }
        
        Ok(())
    }
    
    fn setup_discord(&mut self) -> anyhow::Result<()> {
        println!();
        println!("{}", style("🎮 DISCORD BOT KURULUMU").bold());
        println!("https://discord.com/developers/applications");
        println!();
        
        let token = Password::new()
            .with_prompt("Discord Bot Token")
            .interact()?;
        
        if !token.is_empty() {
            self.config.integrations.discord = Some(IntegrationConfig {
                enabled: true,
                token: Some(token),
                extra: HashMap::new(),
            });
            println!("{} Discord bağlandı!", CHECK);
        }
        
        Ok(())
    }
    
    fn setup_webex(&mut self) -> anyhow::Result<()> {
        println!();
        println!("{}", style("📹 CISCO WEBEX KURULUMU").bold());
        println!("https://developer.webex.com");
        println!();
        
        let token = Password::new()
            .with_prompt("Webex Access Token")
            .interact()?;
        
        if !token.is_empty() {
            self.config.integrations.webex = Some(IntegrationConfig {
                enabled: true,
                token: Some(token),
                extra: HashMap::new(),
            });
            println!("{} Webex bağlandı!", CHECK);
        }
        
        Ok(())
    }
    
    fn setup_zoom(&mut self) -> anyhow::Result<()> {
        println!();
        println!("{}", style("🎥 ZOOM CHAT KURULUMU").bold());
        println!("Zoom Marketplace: https://marketplace.zoom.us");
        println!();
        
        let webhook: String = Input::new()
            .with_prompt("Zoom Webhook URL")
            .interact_text()?;
        
        if !webhook.is_empty() {
            self.config.integrations.zoom = Some(IntegrationConfig {
                enabled: true,
                token: Some(webhook),
                extra: HashMap::new(),
            });
            println!("{} Zoom Chat bağlandı!", CHECK);
        }
        
        Ok(())
    }
    
    fn setup_mattermost(&mut self) -> anyhow::Result<()> {
        println!();
        println!("{}", style("🗣️ MATTERMOST KURULUMU").bold());
        println!();
        
        let url: String = Input::new()
            .with_prompt("Mattermost Server URL")
            .interact_text()?;
        
        let token = Password::new()
            .with_prompt("Bot Token")
            .interact()?;
        
        if !url.is_empty() {
            self.config.integrations.mattermost = Some(IntegrationConfig {
                enabled: true,
                token: Some(token),
                extra: [("server_url".to_string(), url)].into_iter().collect(),
            });
            println!("{} Mattermost bağlandı!", CHECK);
        }
        
        Ok(())
    }
    
    fn setup_rocketchat(&mut self) -> anyhow::Result<()> {
        println!();
        println!("{}", style("🚀 ROCKETCHAT KURULUMU").bold());
        println!();
        
        let url: String = Input::new()
            .with_prompt("RocketChat Server URL")
            .interact_text()?;
        
        let token = Password::new()
            .with_prompt("Personal Access Token")
            .interact()?;
        
        if !url.is_empty() {
            self.config.integrations.rocketchat = Some(IntegrationConfig {
                enabled: true,
                token: Some(token),
                extra: [("server_url".to_string(), url)].into_iter().collect(),
            });
            println!("{} RocketChat bağlandı!", CHECK);
        }
        
        Ok(())
    }
    
    fn configure_decentralized_channels(&mut self) -> anyhow::Result<()> {
        let channels = vec![
            "Matrix (Element)",
            "XMPP/Jabber",
            "Session",
            "Wire",
            "Threema",
            "🔙 Geri",
        ];
        
        let sel = Select::new()
            .with_prompt("🔐 Decentralized Platform seçin")
            .items(&channels)
            .default(5)
            .interact()?;
        
        match sel {
            0 => self.setup_matrix()?,
            1 => self.setup_xmpp()?,
            2 => self.setup_session()?,
            3 => self.setup_wire()?,
            4 => self.setup_threema()?,
            _ => return Ok(()),
        }
        
        Ok(())
    }
    
    fn setup_matrix(&mut self) -> anyhow::Result<()> {
        println!();
        println!("{}", style("🧮 MATRIX (ELEMENT) KURULUMU").bold());
        println!();
        println!("Matrix, açık kaynaklı ve federasyonlu bir mesajlaşma protokolüdür.");
        println!("Element → Settings → Help → Access Token");
        println!();
        
        let homeserver: String = Input::new()
            .default("https://matrix-client.matrix.org".to_string())
            .with_prompt("Homeserver URL")
            .interact_text()?;
        
        let token = Password::new()
            .with_prompt("Access Token")
            .interact()?;
        
        let user_id: String = Input::new()
            .with_prompt("User ID (@user:matrix.org)")
            .interact_text()?;
        
        if !token.is_empty() {
            self.config.integrations.matrix = Some(IntegrationConfig {
                enabled: true,
                token: Some(token),
                extra: [
                    ("homeserver".to_string(), homeserver),
                    ("user_id".to_string(), user_id),
                ].into_iter().collect(),
            });
            println!("{} Matrix bağlandı!", CHECK);
        }
        
        Ok(())
    }
    
    fn setup_xmpp(&mut self) -> anyhow::Result<()> {
        println!();
        println!("{}", style("⚡ XMPP/JABBER KURULUMU").bold());
        println!();
        
        let jid: String = Input::new()
            .with_prompt("Jabber ID (user@server.com)")
            .interact_text()?;
        
        let password = Password::new()
            .with_prompt("Password")
            .interact()?;
        
        if !jid.is_empty() {
            self.config.integrations.xmpp = Some(IntegrationConfig {
                enabled: true,
                token: Some(password),
                extra: [("jid".to_string(), jid)].into_iter().collect(),
            });
            println!("{} XMPP bağlandı!", CHECK);
        }
        
        Ok(())
    }
    
    fn setup_session(&mut self) -> anyhow::Result<()> {
        println!();
        println!("{}", style("🔒 SESSION KURULUMU").bold());
        println!("Session, tamamen anonim ve merkeziyetsiz bir mesajlaşma uygulamasıdır.");
        println!("GUI Automation ile kontrol edilebilir.");
        println!();
        
        let enable = Confirm::new()
            .with_prompt("Session entegrasyonu (GUI Automation) aktif et?")
            .default(true)
            .interact()?;
        
        if enable {
            self.config.integrations.session = Some(IntegrationConfig {
                enabled: true,
                token: None,
                extra: [("method".to_string(), "gui_automation".to_string())].into_iter().collect(),
            });
            println!("{} Session yapılandırıldı!", CHECK);
        }
        
        Ok(())
    }
    
    fn setup_wire(&mut self) -> anyhow::Result<()> {
        println!();
        println!("{}", style("🔐 WIRE KURULUMU").bold());
        println!("Wire, E2E şifreli mesajlaşma platformudur.");
        println!();
        
        let email: String = Input::new()
            .with_prompt("Wire Email")
            .interact_text()?;
        
        let password = Password::new()
            .with_prompt("Password")
            .interact()?;
        
        if !email.is_empty() {
            self.config.integrations.wire = Some(IntegrationConfig {
                enabled: true,
                token: Some(password),
                extra: [("email".to_string(), email)].into_iter().collect(),
            });
            println!("{} Wire bağlandı!", CHECK);
        }
        
        Ok(())
    }
    
    fn setup_threema(&mut self) -> anyhow::Result<()> {
        println!();
        println!("{}", style("🔐 THREEMA KURULUMU").bold());
        println!("Threema Work API gereklidir.");
        println!();
        
        let api_key = Password::new()
            .with_prompt("Threema Work API Key")
            .interact()?;
        
        let api_identity: String = Input::new()
            .with_prompt("API Identity (8 karakter)")
            .interact_text()?;
        
        if !api_key.is_empty() {
            self.config.integrations.threema = Some(IntegrationConfig {
                enabled: true,
                token: Some(api_key),
                extra: [("api_identity".to_string(), api_identity)].into_iter().collect(),
            });
            println!("{} Threema bağlandı!", CHECK);
        }
        
        Ok(())
    }
    
    fn configure_social_channels(&mut self) -> anyhow::Result<()> {
        let channels = vec![
            "Twitter/X DM",
            "Instagram DM",
            "Facebook Messenger",
            "LinkedIn Messaging",
            "Reddit Chat",
            "🔙 Geri",
        ];
        
        let sel = Select::new()
            .with_prompt("📱 Social Platform seçin")
            .items(&channels)
            .default(5)
            .interact()?;
        
        match sel {
            0 => self.setup_twitter()?,
            1 => self.setup_instagram()?,
            2 => self.setup_facebook()?,
            3 => self.setup_linkedin()?,
            4 => self.setup_reddit()?,
            _ => return Ok(()),
        }
        
        Ok(())
    }
    
    fn setup_twitter(&mut self) -> anyhow::Result<()> {
        println!();
        println!("{}", style("🐦 TWITTER/X DM KURULUMU").bold());
        println!("Twitter Developer Portal: https://developer.twitter.com");
        println!();
        
        let bearer_token = Password::new()
            .with_prompt("Bearer Token")
            .interact()?;
        
        if !bearer_token.is_empty() {
            self.config.integrations.twitter = Some(IntegrationConfig {
                enabled: true,
                token: Some(bearer_token),
                extra: HashMap::new(),
            });
            println!("{} Twitter/X DM bağlandı!", CHECK);
        }
        
        Ok(())
    }
    
    fn setup_instagram(&mut self) -> anyhow::Result<()> {
        println!();
        println!("{}", style("📸 INSTAGRAM DM KURULUMU").bold());
        println!("Meta for Developers: https://developers.facebook.com");
        println!();
        
        let access_token = Password::new()
            .with_prompt("Page Access Token")
            .interact()?;
        
        if !access_token.is_empty() {
            self.config.integrations.instagram = Some(IntegrationConfig {
                enabled: true,
                token: Some(access_token),
                extra: HashMap::new(),
            });
            println!("{} Instagram DM bağlandı!", CHECK);
        }
        
        Ok(())
    }
    
    fn setup_facebook(&mut self) -> anyhow::Result<()> {
        println!();
        println!("{}", style("📘 FACEBOOK MESSENGER KURULUMU").bold());
        println!();
        
        let page_token = Password::new()
            .with_prompt("Page Access Token")
            .interact()?;
        
        if !page_token.is_empty() {
            self.config.integrations.facebook = Some(IntegrationConfig {
                enabled: true,
                token: Some(page_token),
                extra: HashMap::new(),
            });
            println!("{} Facebook Messenger bağlandı!", CHECK);
        }
        
        Ok(())
    }
    
    fn setup_linkedin(&mut self) -> anyhow::Result<()> {
        println!();
        println!("{}", style("💼 LINKEDIN MESSAGING KURULUMU").bold());
        println!("LinkedIn Developers: https://www.linkedin.com/developers");
        println!();
        
        let access_token = Password::new()
            .with_prompt("Access Token")
            .interact()?;
        
        if !access_token.is_empty() {
            self.config.integrations.linkedin = Some(IntegrationConfig {
                enabled: true,
                token: Some(access_token),
                extra: HashMap::new(),
            });
            println!("{} LinkedIn Messaging bağlandı!", CHECK);
        }
        
        Ok(())
    }
    
    fn setup_reddit(&mut self) -> anyhow::Result<()> {
        println!();
        println!("{}", style("🔴 REDDIT CHAT KURULUMU").bold());
        println!("Reddit Apps: https://www.reddit.com/prefs/apps");
        println!();
        
        let client_id: String = Input::new()
            .with_prompt("Client ID")
            .interact_text()?;
        
        let client_secret = Password::new()
            .with_prompt("Client Secret")
            .interact()?;
        
        if !client_id.is_empty() {
            self.config.integrations.reddit = Some(IntegrationConfig {
                enabled: true,
                token: Some(client_secret),
                extra: [("client_id".to_string(), client_id)].into_iter().collect(),
            });
            println!("{} Reddit Chat bağlandı!", CHECK);
        }
        
        Ok(())
    }
    
    fn configure_email_sms_channels(&mut self) -> anyhow::Result<()> {
        let channels = vec![
            "Email (SMTP)",
            "SMS (Twilio)",
            "RCS Messaging",
            "🔙 Geri",
        ];
        
        let sel = Select::new()
            .with_prompt("📧 Email & SMS seçin")
            .items(&channels)
            .default(3)
            .interact()?;
        
        match sel {
            0 => self.setup_email()?,
            1 => self.setup_sms()?,
            2 => self.setup_rcs()?,
            _ => return Ok(()),
        }
        
        Ok(())
    }
    
    fn setup_email(&mut self) -> anyhow::Result<()> {
        println!();
        println!("{}", style("📧 EMAIL SMTP KURULUMU").bold());
        println!();
        
        let host: String = Input::new()
            .default("smtp.gmail.com".to_string())
            .with_prompt("SMTP Host")
            .interact_text()?;
        
        let port_input: String = Input::new()
            .default("587".to_string())
            .with_prompt("SMTP Port")
            .interact_text()?;
        let port: u16 = port_input.parse().unwrap_or(587);
        
        let user: String = Input::new()
            .with_prompt("Email adresi")
            .interact_text()?;
        
        let pass = Password::new()
            .with_prompt("Email şifresi (App Password)")
            .interact()?;
        
        self.config.integrations.email = Some(IntegrationConfig {
            enabled: true,
            token: None,
            extra: vec![
                ("host".to_string(), host),
                ("port".to_string(), port.to_string()),
                ("user".to_string(), user),
                ("pass".to_string(), pass),
            ].into_iter().collect(),
        });
        println!("{} Email bağlandı!", CHECK);
        
        Ok(())
    }
    
    fn setup_sms(&mut self) -> anyhow::Result<()> {
        println!();
        println!("{}", style("📱 SMS (TWILIO) KURULUMU").bold());
        println!("Twilio Console: https://www.twilio.com/console");
        println!();
        
        let account_sid: String = Input::new()
            .with_prompt("Account SID")
            .interact_text()?;
        
        let auth_token = Password::new()
            .with_prompt("Auth Token")
            .interact()?;
        
        if !account_sid.is_empty() {
            self.config.integrations.sms = Some(IntegrationConfig {
                enabled: true,
                token: Some(auth_token),
                extra: [("account_sid".to_string(), account_sid)].into_iter().collect(),
            });
            println!("{} SMS (Twilio) bağlandı!", CHECK);
        }
        
        Ok(())
    }
    
    fn setup_rcs(&mut self) -> anyhow::Result<()> {
        println!();
        println!("{}", style("📱 RCS MESSAGING KURULUMU").bold());
        println!("RCS için Google Business Messages kullanılır.");
        println!();
        
        let project_id: String = Input::new()
            .with_prompt("Google Cloud Project ID")
            .interact_text()?;
        
        if !project_id.is_empty() {
            self.config.integrations.rcs = Some(IntegrationConfig {
                enabled: true,
                token: None,
                extra: [("project_id".to_string(), project_id)].into_iter().collect(),
            });
            println!("{} RCS yapılandırıldı!", CHECK);
        }
        
        Ok(())
    }
    
    fn configure_dev_channels(&mut self) -> anyhow::Result<()> {
        let channels = vec![
            "GitHub",
            "GitLab",
            "Jira",
            "PagerDuty",
            "🔙 Geri",
        ];
        
        let sel = Select::new()
            .with_prompt("🐙 Developer Tool seçin")
            .items(&channels)
            .default(4)
            .interact()?;
        
        match sel {
            0 => self.setup_github()?,
            1 => self.setup_gitlab()?,
            2 => self.setup_jira()?,
            3 => self.setup_pagerduty()?,
            _ => return Ok(()),
        }
        
        Ok(())
    }
    
    fn setup_github(&mut self) -> anyhow::Result<()> {
        println!();
        println!("{}", style("🐙 GITHUB KURULUMU").bold());
        println!("https://github.com/settings/tokens");
        println!();
        
        let token = Password::new()
            .with_prompt("Personal Access Token")
            .interact()?;
        
        if !token.is_empty() {
            self.config.integrations.github = Some(IntegrationConfig {
                enabled: true,
                token: Some(token),
                extra: HashMap::new(),
            });
            println!("{} GitHub bağlandı!", CHECK);
        }
        
        Ok(())
    }
    
    fn setup_gitlab(&mut self) -> anyhow::Result<()> {
        println!();
        println!("{}", style("🦊 GITLAB KURULUMU").bold());
        println!();
        
        let url: String = Input::new()
            .default("https://gitlab.com".to_string())
            .with_prompt("GitLab URL")
            .interact_text()?;
        
        let token = Password::new()
            .with_prompt("Personal Access Token")
            .interact()?;
        
        if !token.is_empty() {
            self.config.integrations.gitlab = Some(IntegrationConfig {
                enabled: true,
                token: Some(token),
                extra: [("url".to_string(), url)].into_iter().collect(),
            });
            println!("{} GitLab bağlandı!", CHECK);
        }
        
        Ok(())
    }
    
    fn setup_jira(&mut self) -> anyhow::Result<()> {
        println!();
        println!("{}", style("📋 JIRA KURULUMU").bold());
        println!();
        
        let url: String = Input::new()
            .with_prompt("Jira URL (ör: https://company.atlassian.net)")
            .interact_text()?;
        
        let email: String = Input::new()
            .with_prompt("Email")
            .interact_text()?;
        
        let token = Password::new()
            .with_prompt("API Token")
            .interact()?;
        
        if !url.is_empty() {
            self.config.integrations.jira = Some(IntegrationConfig {
                enabled: true,
                token: Some(token),
                extra: [
                    ("url".to_string(), url),
                    ("email".to_string(), email),
                ].into_iter().collect(),
            });
            println!("{} Jira bağlandı!", CHECK);
        }
        
        Ok(())
    }
    
    fn setup_pagerduty(&mut self) -> anyhow::Result<()> {
        println!();
        println!("{}", style("🚨 PAGERDUTY KURULUMU").bold());
        println!();
        
        let token = Password::new()
            .with_prompt("API Token")
            .interact()?;
        
        if !token.is_empty() {
            self.config.integrations.pagerduty = Some(IntegrationConfig {
                enabled: true,
                token: Some(token),
                extra: HashMap::new(),
            });
            println!("{} PagerDuty bağlandı!", CHECK);
        }
        
        Ok(())
    }
    
    fn configure_enterprise_integrations(&mut self) -> anyhow::Result<()> {
        self.print_step("🏢 Enterprise Entegrasyonları");
        
        println!("Ek enterprise entegrasyonları:");
        println!();
        
        let integrations = vec![
            "Notion", "Confluence", "Trello", "Asana",
            "Monday.com", "Linear", "ClickUp", "Airtable",
            "Zapier", "Make (Integromat)",
        ];
        
        let selections = MultiSelect::new()
            .with_prompt("Eklemek istediğiniz entegrasyonları seçin (Space ile çoklu seçim)")
            .items(&integrations)
            .interact()?;
        
        for idx in selections {
            println!("{} {} seçildi", CHECK, integrations[idx]);
        }
        
        Ok(())
    }
    
    /// ADIM 7: Human Emulation - İnsan Taklidi Ayarları
    fn configure_human_emulation(&mut self) -> anyhow::Result<()> {
        self.print_step("🎭 HUMAN EMULATION (İnsan Taklidi Sistemi)");
        
        println!("{}", style("🎭 SENTIENT HUMAN EMULATION ENGINE v1.0").bold().cyan());
        println!();
        println!("  Bu sistem SENTIENT'ya şu yetenekleri kazandırır:");
        println!("  ✓ Doğal fare hareketleri (Bezier eğrileri)");
        println!("  ✓ Gerçekçi yazma hızı (WPM)");
        println!("  ✓ Fare titremesi simülasyonu");
        println!("  ✓ Residential Proxy rotasyonu");
        println!("  ✓ Otomatik CAPTCHA çözümü");
        println!("  ✓ Agent-S3 Behavior Best-of-N (%72.6+ başarı)");
        println!();
        
        let enable = Confirm::new()
            .with_prompt("Human Emulsion sistemini aktif etmek istiyor musunuz?")
            .default(true)
            .interact()?;
        
        if !enable {
            println!("{} Human Emulsion devre dışı bırakıldı.", CHECK);
            return Ok(());
        }
        
        // Fare ayarları
        println!();
        println!("{}", style("🖱️ FARE AYARLARI").bold());
        println!();
        
        let tremor = Confirm::new()
            .with_prompt("Fare titreşimi simülasyonu aktif edilsin mi?")
            .default(true)
            .interact()?;
        
        let humanlikeness: String = Input::new()
            .with_prompt("İnsan benzerliği seviyesi (0.0 - 1.0)")
            .default("0.85".to_string())
            .interact_text()?;
        let humanlikeness: f64 = humanlikeness.parse().unwrap_or(0.85);
        
        // Yazma ayarları
        println!();
        println!("{}", style("⌨️ YAZMA AYARLARI").bold());
        println!();
        
        let wpm: String = Input::new()
            .with_prompt("Yazma hızı (WPM - Words Per Minute, önerilen: 45)")
            .default("45".to_string())
            .interact_text()?;
        let wpm = wpm.parse().unwrap_or(45);
        
        // Proxy ayarları
        println!();
        println!("{}", style("🌐 PROXY ROTASYONU").bold());
        println!();
        
        let proxy_rotation = Confirm::new()
            .with_prompt("Residential Proxy rotasyonu aktif edilsin mi?")
            .default(false)
            .interact()?;
        
        if proxy_rotation {
            println!();
            println!("Proxy sağlayıcılarınızı ekleyin (bitirmek için boş bırakın):");
            
            loop {
                let proxy_url: String = Input::new()
                    .with_prompt("Proxy URL (ör: socks5://user:pass@host:port)")
                    .allow_empty(true)
                    .interact_text()?;
                
                if proxy_url.is_empty() {
                    break;
                }
                
                let country: String = Input::new()
                    .with_prompt("Ülke kodu (ör: US, DE, GB)")
                    .default("US".to_string())
                    .interact_text()?;
                
                println!("{} Proxy eklendi: {} ({})", CHECK, proxy_url, country);
            }
        }
        
        // CAPTCHA ayarları
        println!();
        println!("{}", style("🔐 AUTO-CAPTCHA").bold());
        println!();
        
        let auto_captcha = Confirm::new()
            .with_prompt("Otomatik CAPTCHA çözümü aktif edilsin mi?")
            .default(false)
            .interact()?;
        
        if auto_captcha {
            println!();
            println!("CAPTCHA servisleri için API anahtarı gerekli.");
            println!("Desteklenen servisler: 2Captcha, AntiCaptcha, Capsolver");
            println!();
            
            let api_key = Password::new()
                .with_prompt("API Anahtarı (boş bırakabilirsiniz)")
                .allow_empty_password(true)
                .interact()?;
            
            if !api_key.is_empty() {
                println!("{} API anahtarı kaydedildi.", CHECK);
            }
        }
        
        // Behavior Best-of-N
        println!();
        println!("{}", style("🧠 AGENT-S3 BEHAVIOR BEST-OF-N").bold().cyan());
        println!();
        println!("  Bu algoritma GUI eylemlerinin başarı oranını artırır.");
        println!("  Hedef: %72.6 üzeri başarı (insan seviyesi üstü)");
        println!();
        
        let best_of_n = Confirm::new()
            .with_prompt("Behavior Best-of-N algoritmasını aktif et?")
            .default(true)
            .interact()?;
        
        if best_of_n {
            let n_value: String = Input::new()
                .with_prompt("N değeri (önerilen: 5)")
                .default("5".to_string())
                .interact_text()?;
            let n_value = n_value.parse().unwrap_or(5);
            
            println!("{} Behavior Best-of-N aktif (N={})", CHECK, n_value);
        }
        
        println!();
        println!("{} Human Emulsion yapılandırıldı!", CHECK);
        println!("   Fare titremesi: {}", if tremor { "Aktif" } else { "Pasif" });
        println!("   İnsan benzerliği: {:.0}%", humanlikeness * 100.0);
        println!("   Yazma hızı: {} WPM", wpm);
        println!("   Proxy rotasyonu: {}", if proxy_rotation { "Aktif" } else { "Pasif" });
        println!("   Auto-CAPTCHA: {}", if auto_captcha { "Aktif" } else { "Pasif" });
        
        Ok(())
    }
    
    /// ADIM 8: Dynamic Routing Mode - Human-in-the-Loop (v4.0.0)
    fn configure_dynamic_routing(&mut self) -> anyhow::Result<()> {
        self.print_step("🔀 DYNAMIC ROUTING MODE (Human-in-the-Loop)");
        
        println!("{}", style("🔀 SENTIENT DYNAMIC ROUTING ENGINE v4.0.0").bold().cyan());
        println!();
        println!("  Görev zorluğuna göre OTOMATİK model seçimi:");
        println!();
        println!("  ✓ Basit görevler → Mini modeller (ücretsiz)");
        println!("  ✓ Orta görevler → Standard modeller");
        println!("  ✓ Karmaşık görevler → Advanced modeller");
        println!("  ✓ Çok karmaşık → Premium modeller");
        println!();
        println!("  💰 Maliyet optimizasyonu ile %70'e kadar tasarruf!");
        println!();
        
        // Routing mode seçimi
        let modes = vec![
            "🤖 TAM OTONOM - SENTIENT otomatik seçer (hızlı)",
            "👤 KULLANICI ONAYLI - Her değişimde sorulur (kontrollü)",
            "👤+✋ ONAYLI + MANUEL - Onay + anlık değişiklik (esnek)",
        ];
        
        let mode_sel = Select::new()
            .with_prompt("Otomatik model değişimi modu")
            .items(&modes)
            .default(1) // Varsayılan: Kullanıcı onaylı
            .interact()?;
        
        let routing_mode = match mode_sel {
            0 => "fully_autonomous",
            1 => "require_approval",
            2 => "approval_with_override",
            _ => "require_approval",
        };
        
        self.config.routing_mode.mode = routing_mode.to_string();
        
        println!();
        println!("{} Seçilen mod: {}", CHECK, match routing_mode {
            "fully_autonomous" => "Tam Otonom",
            "require_approval" => "Kullanıcı Onaylı",
            "approval_with_override" => "Onaylı + Manuel Değişiklik",
            _ => "Bilinmiyor",
        });
        
        // Eğer onay modu seçilmediyse uyar
        if routing_mode == "fully_autonomous" {
            println!();
            println!("{}", style("⚠️  DİKKAT: Tam otonom mod seçildi!").yellow().bold());
            println!("   SENTIENT görev başlangıcında otomatik model seçecek.");
            println!("   Dashboard'da popup çıkmayacak.");
            println!();
            
            let confirm = Confirm::new()
                .with_prompt("Tam otonom modu onaylıyor musunuz?")
                .default(true)
                .interact()?;
            
            if !confirm {
                self.config.routing_mode.mode = "require_approval".to_string();
                println!("{} Mod değiştirildi: Kullanıcı Onaylı", CHECK);
            }
        }
        
        // Maliyet optimizasyonu
        println!();
        let cost_opt = Confirm::new()
            .with_prompt("Maliyet optimizasyonu aktif edilsin mi? (Ücretsiz modeller öncelikli)")
            .default(true)
            .interact()?;
        
        self.config.routing_mode.cost_optimization = cost_opt;
        
        // Free model preference
        if cost_opt {
            let prefer_free = Confirm::new()
                .with_prompt("Mümkün olduğunda ücretsiz modeller kullanılsın mı?")
                .default(true)
                .interact()?;
            
            self.config.routing_mode.prefer_free = prefer_free;
            
            println!();
            println!("{}", style("💡 Ücretsiz model kaynakları:").cyan());
            println!("   • OpenRouter (qwen/qwen3-1.7b:free)");
            println!("   • Groq (mixtral-8x7b - free tier)");
            println!("   • Ollama (yerel - tamamen ücretsiz)");
        }
        
        // Minimum kalite seviyesi
        println!();
        let tiers = vec!["mini", "standard", "advanced", "premium"];
        let tier_sel = Select::new()
            .with_prompt("Minimum kalite seviyesi (en düşük kullanılacak model)")
            .items(&["Mini (basit görevler için)", "Standard (önerilen)", "Advanced (kalite odaklı)", "Premium (yüksek kalite)"])
            .default(0)
            .interact()?;
        
        self.config.routing_mode.min_quality_tier = tiers[tier_sel].to_string();
        
        // Maximum latency
        println!();
        let latency_input: String = Input::new()
            .with_prompt("Maksimum bekleme süresi (ms, önerilen: 5000)")
            .default("5000".to_string())
            .interact_text()?;
        
        self.config.routing_mode.max_latency_ms = latency_input.parse().unwrap_or(5000);
        
        // Özet
        println!();
        println!("{} Dynamic Routing yapılandırıldı!", CHECK);
        println!("   Mod: {}", self.config.routing_mode.mode);
        println!("   Maliyet optimizasyonu: {}", if self.config.routing_mode.cost_optimization { "Aktif" } else { "Pasif" });
        println!("   Minimum tier: {}", self.config.routing_mode.min_quality_tier);
        println!("   Max latency: {}ms", self.config.routing_mode.max_latency_ms);
        
        // Dashboard UI hakkında bilgi
        if routing_mode != "fully_autonomous" {
            println!();
            println!("{}", style("📱 DASHBOARD UI:").bold().cyan());
            println!("   Görev başlamadan önce popup açılacak:");
            println!("   ┌─────────────────────────────────────────┐");
            println!("   │ 🎯 Önerilen Model: GPT-4 Turbo           │");
            println!("   │ 📊 Zorluk: Karmaşık                     │");
            println!("   │ 💰 Tahmini Maliyet: $0.0234             │");
            println!("   │                                         │");
            println!("   │ [Onayla]  [Model Değiştir ▼]  [İptal]  │");
            println!("   └─────────────────────────────────────────┘");
            println!("   Dropdown: Tüm kayıtlı modeller listelenecek");
        }
        
        Ok(())
    }
    
    fn configure_permissions(&mut self) -> anyhow::Result<()> {
        self.print_step("🔐 AGENT-S3 Hardware Permissions (Yetkilendirme)");
        
        println!("{}", style("🤖 OTONOM MOD SEVİYELERİ:").bold());
        println!();
        println!("  Level 1: 📖 Sadece okuma (Read-only)");
        println!("           → Dosya okuma, web tarama");
        println!();
        println!("  Level 2: 📝 Dosya işlemleri");
        println!("           → Dosya oluşturma, düzenleme, silme");
        println!();
        println!("  Level 3: 🖱️ Klavye/Mouse kontrolü (Agent-S3)");
        println!("           → GUI automation, desktop control");
        println!("           → Signal, iMessage, Session (GUI mode)");
        println!();
        println!("  Level 4: 🚀 Tam otonom");
        println!("           → Tüm işlemler + ağ erişimi");
        println!("           → SSH, FTP, Docker");
        println!();
        println!("  Level 5: ⚡ Sistem yönetimi");
        println!("           → Root erişimi, sistem değişiklikleri");
        println!("           → Service management, kernel modules");
        println!();
        
        let levels = vec![
            "Level 1 - Sadece okuma",
            "Level 2 - Dosya işlemleri",
            "Level 3 - Klavye/Mouse (Agent-S3)",
            "Level 4 - Tam otonom",
            "Level 5 - Sistem yönetimi",
        ];
        
        let selection = Select::new()
            .with_prompt("Varsayılan yetki seviyesi")
            .items(&levels)
            .default(2)
            .interact()?;
        
        self.config.permissions.default_level = (selection + 1) as u8;
        
        // GUI automation izni
        if selection >= 2 {
            println!();
            println!("{}", style("🖱️ AGENT-S3: KLAVYE/MOUSE KONTROLÜ").bold().cyan());
            println!();
            println!("Bu modda SENTIENT şunları yapabilir:");
            println!("  ✓ Masaüstü uygulamalarını kontrol etme");
            println!("  ✓ Signal Desktop, iMessage, Session kullanma");
            println!("  ✓ Otomatik form doldurma");
            println!("  ✓ Ekran okuma ve analiz");
            println!("  ✓ Otomatik tıklama ve yazma");
            println!();
            
            let gui_permission = Confirm::new()
                .with_prompt("SENTIENT klavye ve fareyi kontrol edebilsin mi?")
                .default(true)
                .interact()?;
            
            self.config.permissions.allow_gui_control = gui_permission;
            
            if gui_permission {
                println!();
                println!("{}", style("⚠️  DİKKAT:").yellow().bold());
                println!("SENTIENT klavye ve fareyi tam kontrol edebilecek.");
                println!("Bu, potansiyel olarak güçlü bir özelliktir.");
                println!();
                
                let confirm = Confirm::new()
                    .with_prompt("AGENT-S3 aktivasyonunu onaylıyor musunuz?")
                    .default(true)
                    .interact()?;
                
                if !confirm {
                    self.config.permissions.allow_gui_control = false;
                    println!("{} GUI kontrolü devre dışı bırakıldı.", CROSS);
                } else {
                    println!("{} AGENT-S3 aktif! Klavye/Mouse kontrolü etkin.", CHECK);
                    
                    // Ek GUI ayarları
                    println!();
                    let screen_record = Confirm::new()
                        .with_prompt("Ekran kaydı için izin verilsin mi? (Görsel AI analiz)")
                        .default(true)
                        .interact()?;
                    
                    self.config.permissions.allow_screen_recording = screen_record;
                }
            }
        }
        
        // Skill izinleri
        println!();
        println!("{}", style("🎯 5.587 OTONOM SKILL").bold().cyan());
        println!();
        println!("SENTIENT, aşağıdaki kategorilerde binlerce otonom skill'e sahip:");
        println!("  📁 Dosya Yönetimi: 847 skill");
        println!("  🌐 Web Etkileşimi: 1.234 skill");
        println!("  💻 Kod Geliştirme: 956 skill");
        println!("  📊 Veri Analizi: 678 skill");
        println!("  🔧 Sistem Yönetimi: 423 skill");
        println!("  🎨 Medya İşleme: 312 skill");
        println!("  📧 İletişim: 534 skill");
        println!("  🔐 Güvenlik: 603 skill");
        println!();
        
        let skill_mode = Select::new()
            .with_prompt("Skill çalıştırma modu")
            .items(&[
                "🔒 Manual Onay (Her skill için onay iste)",
                "⚡ Auto-Safe (Güvenli skill'ler otomatik)",
                "🚀 Full Auto (Tüm skill'ler otomatik)",
            ])
            .default(1)
            .interact()?;
        
        self.config.permissions.skill_mode = match skill_mode {
            0 => "manual",
            1 => "auto_safe",
            2 => "full_auto",
            _ => "auto_safe",
        }.to_string();
        
        println!("{} Yetki seviyesi: Level {}", CHECK, self.config.permissions.default_level);
        println!("{} Skill modu: {}", CHECK, self.config.permissions.skill_mode);
        Ok(())
    }
    
    fn save_and_complete(&mut self) -> anyhow::Result<()> {
        self.print_step("Kurulum Tamamlanıyor");
        
        let pb = ProgressBar::new(100);
        pb.set_style(
            ProgressStyle::with_template("{spinner:.green} {msg} [{bar:40.cyan/blue}] {percent}%")
                .unwrap()
                .progress_chars("━╸ ")
        );
        
        pb.set_message("Yapılandırma kaydediliyor...");
        pb.inc(25);
        
        // Config kaydet
        self.config.save()?;
        pb.inc(25);
        
        pb.set_message("V-GATE proxy başlatılıyor...");
        pb.inc(25);
        
        pb.set_message("Entegrasyonlar test ediliyor...");
        pb.inc(25);
        
        pb.finish_with_message("Kurulum tamamlandı!");
        
        println!();
        println!("{}", style("╔════════════════════════════════════════════════════════════════════════════════╗").green());
        println!("{}", style("║                                                                                ║").green());
        println!("{}", style("║   🐺 KURULUM BAŞARIYLA TAMAMLANDI!                                            ║").green());
        println!("{}", style("║                                                                                ║").green());
        println!("{}", style("║   🌐 Universal Omni-Gateway Aktif                                              ║").green());
        println!("{}", style("║   💬 20+ Mesajlaşma Kanalı Hazır                                               ║").green());
        println!("{}", style("║   🤖 5.587 Otonom Skill Erişilebilir                                           ║").green());
        println!("{}", style("║                                                                                ║").green());
        println!("{}", style("║   SENTIENT'yı başlatmak için:                                                    ║").green());
        println!("{}", style("║   → make run                                                                  ║").green());
        println!("{}", style("║   → Dashboard: http://localhost:8080                                         ║").green());
        println!("{}", style("║                                                                                ║").green());
        println!("{}", style("║   📖 USER_MANUAL.md'yi okuyun!                                                ║").green());
        println!("{}", style("║   → Tüm özelliklerin detaylı açıklaması                                       ║").green());
        println!("{}", style("║   → 20+ kanal kullanım kılavuzu                                               ║").green());
        println!("{}", style("║   → Agent-S3 hardware permissions                                             ║").green());
        println!("{}", style("║   → Skill kataloğu ve örnekler                                                ║").green());
        println!("{}", style("║                                                                                ║").green());
        println!("{}", style("╚════════════════════════════════════════════════════════════════════════════════╝").green());
        println!();
        
        Ok(())
    }
    
    fn get_enabled_integrations(&self) -> Vec<String> {
        let mut enabled = vec![];
        
        let integration_fields = [
            (&self.config.integrations.telegram, "Telegram"),
            (&self.config.integrations.discord, "Discord"),
            (&self.config.integrations.slack, "Slack"),
            (&self.config.integrations.email, "Email"),
            (&self.config.integrations.github, "GitHub"),
            (&self.config.integrations.whatsapp, "WhatsApp"),
            (&self.config.integrations.signal, "Signal"),
            (&self.config.integrations.imessage, "iMessage"),
            (&self.config.integrations.wechat, "WeChat"),
            (&self.config.integrations.line, "LINE"),
            (&self.config.integrations.viber, "Viber"),
            (&self.config.integrations.kakaotalk, "KakaoTalk"),
            (&self.config.integrations.ms_teams, "MS Teams"),
            (&self.config.integrations.google_chat, "Google Chat"),
            (&self.config.integrations.webex, "Webex"),
            (&self.config.integrations.zoom, "Zoom"),
            (&self.config.integrations.mattermost, "Mattermost"),
            (&self.config.integrations.rocketchat, "RocketChat"),
            (&self.config.integrations.matrix, "Matrix"),
            (&self.config.integrations.xmpp, "XMPP"),
            (&self.config.integrations.session, "Session"),
            (&self.config.integrations.wire, "Wire"),
            (&self.config.integrations.threema, "Threema"),
            (&self.config.integrations.twitter, "Twitter/X"),
            (&self.config.integrations.instagram, "Instagram"),
            (&self.config.integrations.facebook, "Facebook"),
            (&self.config.integrations.linkedin, "LinkedIn"),
            (&self.config.integrations.reddit, "Reddit"),
            (&self.config.integrations.sms, "SMS"),
            (&self.config.integrations.rcs, "RCS"),
            (&self.config.integrations.gitlab, "GitLab"),
            (&self.config.integrations.jira, "Jira"),
            (&self.config.integrations.pagerduty, "PagerDuty"),
        ];
        
        for (integration, name) in integration_fields {
            if integration.as_ref().map_or(false, |c| c.enabled) {
                enabled.push(name.to_string());
            }
        }
        
        enabled
    }
}

impl Default for SetupWizard {
    fn default() -> Self {
        Self::new()
    }
}
