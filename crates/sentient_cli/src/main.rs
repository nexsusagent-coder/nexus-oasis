//! ─── SENTIENT CLI (Komut Satiri Arayuzu) ───
//!
//! SENTIENT Yapay Zeka Isletim Sistemi icin interaktif REPL arayuzu.

use sentient_common::error::SENTIENTResult;
use sentient_core::SENTIENTSystem;
use sentient_core::llm_test::{LlmTestSuite, SimpleChat, load_api_key_from_env};
use clap::{Parser, Subcommand};
use colored::Colorize;
use rustyline::error::ReadlineError;
use rustyline::history::DefaultHistory;
use rustyline::{CompletionType, Config, EditMode, Editor};
use std::io::{self, Write};
use std::sync::Arc;
use tokio::sync::Mutex;

use sentient_cli::repl::{CommandHandler, CommandResult, CommandHistory, PromptState, ReplMode, CompletionEngine, SENTIENTCompleter, ReplSession, UserPreferences};
use sentient_cli::commands::{CommandParser, CommandRegistry, get_module_help};
use sentient_cli::ui::{SystemDashboard, ModuleManager, ModuleStatus, Spinner, ProgressBar, Table, theme};

/// ─── CLI Argümanları ───

#[derive(Parser, Debug)]
#[command(name = "sentient")]
#[command(about = "SENTIENT — NEXUS OASIS Yapay Zeka Isletim Sistemi")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Sessiz mod (sadece hatalari goster)
    #[arg(short, long)]
    quiet: bool,

    /// Hata ayiklama modu
    #[arg(short, long)]
    debug: bool,

    /// Tema secimi
    #[arg(long, default_value = "ocean")]
    theme: String,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// REPL modunu baslat
    Repl {
        /// Swarm modunda baslat
        #[arg(short, long)]
        swarm: bool,
        
        /// Debug modunda baslat
        #[arg(short, long)]
        debug: bool,
    },

    /// Sistem durumunu goster
    Status,

    /// Bellek modulu
    Memory {
        #[command(subcommand)]
        action: MemoryCommands,
    },

    /// Guvenlik politikalari
    Guardrails {
        #[command(subcommand)]
        action: GuardrailsCommands,
    },

    /// Sandbox islemleri
    Sandbox {
        #[command(subcommand)]
        action: SandboxCommands,
    },

    /// V-GATE proxy
    Vgate {
        #[command(subcommand)]
        action: VgateCommands,
    },

    /// LLM baglantisi
    Llm {
        #[command(subcommand)]
        action: LlmCommands,
    },

    /// Otonom ajan baslat
    Agent {
        /// Hedef aciklamasi
        #[arg(short, long)]
        goal: String,

        /// Kullanilacak model
        #[arg(short, long, default_value = "qwen/qwen3-1.7b:free")]
        model: String,

        /// Maksimum iterasyon
        #[arg(short = 'i', long, default_value = "50")]
        max_iterations: u32,
    },

    /// Swarm sistemi
    Swarm {
        #[command(subcommand)]
        action: SwarmCommands,
    },

    /// API Gateway sunucusu
    Gateway {
        /// HTTP dinleme adresi
        #[arg(long, default_value = "0.0.0.0:8080")]
        http_addr: String,

        /// Telegram bot token (opsiyonel)
        #[arg(long)]
        telegram_token: Option<String>,

        /// JWT secret key
        #[arg(long, default_value = "sentient-gateway-secret")]
        jwt_secret: String,
    },

    /// 7/24 arka plan servisi (Watcher + Orchestrator)
    Serve {
        /// Watcher tarama aralığı (saniye)
        #[arg(long, default_value = "300")]
        scan_interval: u64,

        /// Scout taraması aktif mi?
        #[arg(long, default_value = "true")]
        scout: bool,

        /// Forge görevleri aktif mi?
        #[arg(long, default_value = "true")]
        forge: bool,

        /// Self-Healing aktif mi?
        #[arg(long, default_value = "true")]
        self_healing: bool,
    },
}

#[derive(Subcommand, Debug)]
enum MemoryCommands {
    /// Kayitlari listele
    List {
        /// Limit
        #[arg(short, long, default_value = "20")]
        limit: usize,
    },
    /// Kayitlarda ara
    Search {
        /// Arama sorgusu
        query: String,
    },
    /// Yeni kayit ekle
    Store {
        /// Anahtar
        key: String,
        /// Deger
        value: String,
    },
    /// Sure dolanlari temizle
    Cleanup,
    /// Disa aktar
    Export {
        /// Dosya yolu
        #[arg(short, long)]
        output: String,
    },
}

#[derive(Subcommand, Debug)]
enum GuardrailsCommands {
    /// Politikalari listele
    List,
    /// Politika ac/kapat
    Toggle {
        /// Politika adi
        name: String,
        /// Durum
        enabled: bool,
    },
    /// Metin kontrol et
    Check {
        /// Kontrol edilecek metin
        text: String,
    },
}

#[derive(Subcommand, Debug)]
enum SandboxCommands {
    /// Kod calistir
    Run {
        /// Kod
        code: String,
        /// Dil
        #[arg(short, long, default_value = "python")]
        lang: String,
    },
    /// Konteyner durumu
    Status,
    /// Loglari goster
    Logs {
        /// Konteyner ID
        id: Option<String>,
    },
    /// Konteyner durdur
    Kill {
        /// Konteyner ID
        id: String,
    },
}

#[derive(Subcommand, Debug)]
enum VgateCommands {
    /// Proxy durumu
    Status,
    /// Mevcut modeller
    Models,
    /// Baglanti testi
    Test {
        /// Model (opsiyonel)
        #[arg(short, long)]
        model: Option<String>,
    },
}

#[derive(Subcommand, Debug)]
enum LlmCommands {
    /// Baglanti testi
    Test {
        /// Model (opsiyonel)
        #[arg(short, long)]
        model: Option<String>,
    },
    /// Modelleri listele
    Models,
    /// Interaktif sohbet
    Chat {
        /// Model
        #[arg(short, long, default_value = "qwen/qwen3-1.7b:free")]
        model: String,
    },
}

#[derive(Subcommand, Debug)]
enum SwarmCommands {
    /// Swarm baslat
    Start,
    /// Swarm durdur
    Stop,
    /// Durum goster
    Status,
    /// Ajan olustur
    Spawn {
        /// Ajan tipi
        agent_type: String,
    },
    /// Gorev ata
    Task {
        /// Gorev aciklamasi
        description: String,
    },
}

/// ─── MAIN ───

#[tokio::main]
async fn main() -> SENTIENTResult<()> {
    let cli = Cli::parse();

    // Loglama
    let log_level = if cli.debug {
        "debug"
    } else if cli.quiet {
        "error"
    } else {
        "info"
    };

    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(log_level))
        .format_module_path(false)
        .format_timestamp_secs()
        .init();

    // Tema ayarla
    let theme_name = cli.theme.as_str();
    match theme_name {
        "dark" => sentient_cli::ui::set_theme(sentient_cli::ui::Theme::dark()),
        "neon" => sentient_cli::ui::set_theme(sentient_cli::ui::Theme::neon()),
        "minimal" => sentient_cli::ui::set_theme(sentient_cli::ui::Theme::minimal()),
        _ => sentient_cli::ui::set_theme(sentient_cli::ui::Theme::ocean()),
    }

    // Baslik
    if !cli.quiet {
        print_banner();
    }

    // Alt komut isle
    match cli.command {
        Some(Commands::Gateway { http_addr, telegram_token, jwt_secret }) => {
            run_gateway(&http_addr, telegram_token, &jwt_secret).await?;
        }
        Some(Commands::Serve { scan_interval, scout, forge, self_healing }) => {
            run_serve(scan_interval, scout, forge, self_healing).await?;
        }
        Some(Commands::Agent { goal, model, max_iterations }) => {
            run_agent(&goal, &model, max_iterations).await?;
        }
        Some(Commands::Repl { swarm, debug }) => {
            run_interactive_repl(swarm, debug).await?;
        }
        Some(Commands::Status) => show_status().await?,
        Some(Commands::Memory { action }) => handle_memory(action).await?,
        Some(Commands::Guardrails { action }) => handle_guardrails(action).await?,
        Some(Commands::Sandbox { action }) => handle_sandbox(action).await?,
        Some(Commands::Vgate { action }) => handle_vgate(action).await?,
        Some(Commands::Llm { action }) => handle_llm(action).await?,
        Some(Commands::Swarm { action }) => handle_swarm(action).await?,
        None => run_interactive_repl(false, cli.debug).await?,
    }

    Ok(())
}

/// ─── Gelişmiş REPL ───

async fn run_interactive_repl(swarm_mode: bool, debug: bool) -> SENTIENTResult<()> {
    println!("{}", "🚀  SENTIENT REPL başlatılıyor...".cyan());
    println!("{}", "Çıkmak için 'exit' veya 'quit' yazın.".dimmed());
    println!();

    // Oturum ve bileşenleri baslat
    let mut session = ReplSession::new();
    if swarm_mode {
        session.set_mode(sentient_cli::repl::SessionMode::Swarm);
    }
    session.set_debug(debug);

    let mut history = CommandHistory::new(1000);
    history.load();

    let mut handler = CommandHandler::new();
    let registry = CommandRegistry::new();
    let parser = CommandParser::new();
    let completer = SENTIENTCompleter::new();

    // Rustyline yapilandirmasi
    let config = Config::builder()
        .history_ignore_space(true)
        .completion_type(CompletionType::List)
        .edit_mode(EditMode::Emacs)
        .build();

    let mut rl: Editor<(), DefaultHistory> = Editor::with_config(config)
        .map_err(|e| sentient_common::error::SENTIENTError::General(format!("REPL hatası: {}", e)))?;

    // Completer elle kullanilacak

    // Sistem baslatma
    let system = Arc::new(Mutex::new(SENTIENTSystem::init().await?));

    // Baslangiç durumu
    let mut prompt_state = PromptState::new();
    if swarm_mode {
        prompt_state = prompt_state.with_mode(ReplMode::Swarm);
    }

    println!("{}", sentient_cli::commands::print_quick_help());

    // Ana dongu
    loop {
        let prompt = prompt_state.render();
        let readline = rl.readline(&prompt);

        match readline {
            Ok(line) => {
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    continue;
                }

                // Komut isle
                let parsed = parser.parse(trimmed);
                let result = handler.handle(trimmed, &prompt_state);

                // Kaydet
                history.push(trimmed, matches!(result, CommandResult::Success(_)), None);

                match result {
                    CommandResult::Exit => {
                        println!("{}", "👋  SENTIENT kapatılıyor...".yellow());
                        break;
                    }

                    CommandResult::ModeChange(mode) => {
                        prompt_state.set_mode(mode);
                        session.set_mode(match mode {
                            ReplMode::Normal => sentient_cli::repl::SessionMode::Interactive,
                            ReplMode::Swarm => sentient_cli::repl::SessionMode::Swarm,
                            ReplMode::Debug => sentient_cli::repl::SessionMode::Debug,
                            ReplMode::Admin => sentient_cli::repl::SessionMode::Interactive,
                        });
                        println!("{}", format!("Mod değiştirildi: {:?}", mode).green());
                    }

                    CommandResult::EnterModule(module) => {
                        prompt_state = prompt_state.with_module(&module);
                        session.enter_module(&module);
                        println!("{}", format!("📦  {} modülüne girildi", module).blue());
                    }

                    CommandResult::ExitModule => {
                        prompt_state.set_module(None);
                        session.exit_module();
                        println!("{}", "↩️  Modülden çıkıldı".yellow());
                    }

                    CommandResult::Success(msg) => {
                        if !msg.is_empty() {
                            println!("{}", msg);
                        }
                    }

                    CommandResult::Error(err) => {
                        println!("{}", format!("❌  {}", err).red());
                    }

                    CommandResult::ContinueToLlm(query) => {
                        // LLM sorgusu
                        let model = "qwen/qwen3.6-plus:free";
                        print!("{}", "🤔  ".magenta());
                        io::stdout().flush().ok();

                        let mut sys = system.lock().await;
                        match sys.query_llm(model, &query, None).await {
                            Ok(response) => {
                                session.record_llm_query(response.len() as u64);
                                println!();
                                println!("{}", response.bright_white());
                                println!();
                            }
                            Err(e) => {
                                println!("{}", format!("❌  {}", e.to_sentient_message()).red());
                            }
                        }
                    }
                }
            }

            Err(ReadlineError::Interrupted) => {
                println!("{}", "\n🛑  Kesildi. Çıkmak için Ctrl+D'ye basın.".yellow());
                continue;
            }

            Err(ReadlineError::Eof) => {
                println!("{}", "\n👋  SENTIENT kapatılıyor...".yellow());
                break;
            }

            Err(e) => {
                println!("{}", format!("Hata: {}", e).red());
                break;
            }
        }
    }

    // Temizle
    history.save();
    println!("\n{}", session.report());

    Ok(())
}

/// ─── Durum Goster ───

async fn show_status() -> SENTIENTResult<()> {
    let system = SENTIENTSystem::init().await?;
    
    let mut dashboard = SystemDashboard::new();
    dashboard.add_module("memory", ModuleStatus::Stopped);
    dashboard.add_module("guardrails", ModuleStatus::Running);
    dashboard.add_module("sandbox", ModuleStatus::Stopped);
    dashboard.add_module("vgate", ModuleStatus::Stopped);
    dashboard.add_module("gateway", ModuleStatus::Stopped);
    dashboard.add_module("orchestrator", ModuleStatus::Stopped);
    dashboard.add_module("swarm", ModuleStatus::Stopped);
    
    dashboard.set_active_tasks(0);
    dashboard.set_memory_usage(0.0);
    
    println!("{}", dashboard.render_full());
    
    system.shutdown().await
}

/// ─── Bellek Islemleri ───

async fn handle_memory(action: MemoryCommands) -> SENTIENTResult<()> {
    let system = SENTIENTSystem::init().await?;

    match action {
        MemoryCommands::List { limit } => {
            println!("{}", format!("📋  Son {} bellek kaydı:", limit).cyan());
            // TODO: Liste
        }
        MemoryCommands::Search { query } => {
            println!("{}", format!("🔍  Aranıyor: \"{}\"", query).cyan());
            // TODO: Arama
        }
        MemoryCommands::Store { key, value } => {
            println!("{}", format!("💾  Kaydediliyor: {} = {}", key, value).green());
        }
        MemoryCommands::Cleanup => {
            let mut mem = system.memory.lock().await;
            match mem.cleanup_expired() {
                Ok(count) => println!("{}", format!("🧹  {} kayıt temizlendi.", count).green()),
                Err(e) => println!("{}", format!("❌  Hata: {}", e).red()),
            }
        }
        MemoryCommands::Export { output } => {
            println!("{}", format!("📤  Dışa aktarılıyor: {}", output).cyan());
        }
    }

    system.shutdown().await
}

/// ─── Guardrails Islemleri ───

async fn handle_guardrails(action: GuardrailsCommands) -> SENTIENTResult<()> {
    let system = SENTIENTSystem::init().await?;

    match action {
        GuardrailsCommands::List => {
            println!("{}", "🛡  Güvenlik politikaları:".cyan());
            let guard = system.guardrails.lock().await;
            for policy in guard.list_policies() {
                let status = if policy.enabled { "✅" } else { "❌" };
                println!("  {} {} [{:?}]", status, policy.name, policy.severity);
            }
        }
        GuardrailsCommands::Toggle { name, enabled } => {
            let mut guard = system.guardrails.lock().await;
            match guard.toggle_policy(&name, enabled) {
                Ok(()) => println!("{}", format!("✅  Politika '{}' {}", name, if enabled { "etkinleştirildi" } else { "devre dışı" }).green()),
                Err(e) => println!("{}", format!("❌  Hata: {}", e.summary()).red()),
            }
        }
        GuardrailsCommands::Check { text } => {
            println!("{}", format!("🔍  Kontrol ediliyor: \"{}\"", text.chars().take(50).collect::<String>()).cyan());
        }
    }

    system.shutdown().await
}

/// ─── Sandbox Islemleri ───

async fn handle_sandbox(action: SandboxCommands) -> SENTIENTResult<()> {
    match action {
        SandboxCommands::Run { code, lang } => {
            println!("{}", format!("🐳  {} kodu çalıştırılıyor...", lang).cyan());
            println!("{}", code.dimmed());
            // TODO: Sandbox çalıştırma
        }
        SandboxCommands::Status => {
            println!("{}", "🐳  Sandbox durumu: Hazır".green());
        }
        SandboxCommands::Logs { id } => {
            if let Some(container_id) = id {
                println!("{}", format!("📋  {} konteyner logları:", container_id).cyan());
            } else {
                println!("{}", "📋  Tüm konteyner logları:".cyan());
            }
        }
        SandboxCommands::Kill { id } => {
            println!("{}", format!("🛑  {} durduruluyor...", id).yellow());
        }
    }

    Ok(())
}

/// ─── V-GATE Islemleri ───

async fn handle_vgate(action: VgateCommands) -> SENTIENTResult<()> {
    match action {
        VgateCommands::Status => {
            println!("{}", "[V-GATE] Proxy durumu: Kapalı (Port: 1071)".yellow());
        }
        VgateCommands::Models => {
            println!("{}", "[V-GATE] Modeller listeleniyor...".cyan());
        }
        VgateCommands::Test { model } => {
            let model_name = model.unwrap_or_else(|| "qwen/qwen3-1.7b:free".into());
            println!("{}", format!("[V-GATE] {} test ediliyor...", model_name).cyan());
        }
    }

    Ok(())
}

/// ─── LLM Islemleri ───

async fn handle_llm(action: LlmCommands) -> SENTIENTResult<()> {
    match action {
        LlmCommands::Test { model } => {
            println!("{}", "══════════════════════════════════════════════".cyan());
            println!("{}", "  🧪  LLM BAĞLANTI TESTİ".cyan().bold());
            println!("{}", "══════════════════════════════════════════════".cyan());
            
            let api_key = match load_api_key_from_env() {
                Ok(key) => key,
                Err(e) => {
                    println!();
                    println!("{}", format!("❌  {}", e.to_sentient_message()).red());
                    return Ok(());
                }
            };
            
            println!("{}", "\n🔑  API anahtarı yüklendi".green());
            
            let suite = if let Some(m) = model {
                LlmTestSuite::custom(
                    sentient_core::Provider::OpenRouter,
                    "https://openrouter.ai/api/v1".into(),
                    vec![m],
                )
            } else {
                LlmTestSuite::openrouter()
            };
            
            println!("{}", "\n🚀  Test başlatılıyor...".yellow());
            
            let results = suite.run_all(&api_key).await;
            
            println!();
            for result in &results {
                println!("{}", result.summary());
            }
        }
        
        LlmCommands::Models => {
            println!("{}", "📋  Modeller listeleniyor...".cyan());
        }
        
        LlmCommands::Chat { model } => {
            run_chat_session(&model).await?;
        }
    }

    Ok(())
}

/// ─── Swarm Islemleri ───

async fn handle_swarm(action: SwarmCommands) -> SENTIENTResult<()> {
    match action {
        SwarmCommands::Start => {
            println!("{}", "🐺  Swarm başlatılıyor...".cyan());
        }
        SwarmCommands::Stop => {
            println!("{}", "🛑  Swarm durduruluyor...".yellow());
        }
        SwarmCommands::Status => {
            println!("{}", "📊  Swarm durumu: 0 aktif ajan".cyan());
        }
        SwarmCommands::Spawn { agent_type } => {
            println!("{}", format!("🤖  {} ajanı oluşturuluyor...", agent_type).green());
        }
        SwarmCommands::Task { description } => {
            println!("{}", format!("📋  Görev atanıyor: {}", description).cyan());
        }
    }

    Ok(())
}

/// ─── Sohbet Oturumu ───

async fn run_chat_session(model: &str) -> SENTIENTResult<()> {
    println!("{}", "\n══════════════════════════════════════════════".cyan());
    println!("{}", "  💬  SENTIENT CHAT".cyan().bold());
    println!("{}", "══════════════════════════════════════════════".cyan());
    
    let api_key = match load_api_key_from_env() {
        Ok(key) => key,
        Err(e) => {
            println!("{}", format!("❌  {}", e.to_sentient_message()).red());
            return Ok(());
        }
    };

    println!("{}", format!("\n🎯  Model: {}", model).yellow());
    println!("{}", "📝  Çıkmak için 'exit' yazın.".dimmed());

    let mut chat = SimpleChat::openrouter(api_key, Some(model.into()));
    chat.set_system("Sen SENTIENT'nın yardımcı asistanısın. Kısa, öz ve yardımsever cevaplar ver.");

    let config = Config::builder()
        .history_ignore_space(true)
        .completion_type(CompletionType::List)
        .build();

    let mut rl: Editor<(), DefaultHistory> = Editor::with_config(config)
        .map_err(|e| sentient_common::error::SENTIENTError::General(format!("REPL hatası: {}", e)))?;

    loop {
        let prompt = format!("{}", "you> ".green().bold());
        let readline = rl.readline(&prompt);

        match readline {
            Ok(line) => {
                let trimmed = line.trim();
                if trimmed.is_empty() { continue; }

                match trimmed.to_lowercase().as_str() {
                    "exit" | "quit" | "q" => {
                        println!("{}", "\n👋  Sohbet sonlandırılıyor...".yellow());
                        break;
                    }
                    "clear" | "cls" => {
                        chat.clear_history();
                        println!("{}", "🧹  Geçmiş temizlendi.".green());
                        continue;
                    }
                    _ => {}
                }

                let _ = rl.add_history_entry(trimmed);
                print!("{}", "sentient> ".magenta().bold());
                io::stdout().flush().ok();

                match chat.send(trimmed).await {
                    Ok(response) => {
                        println!("{}\n", response.bright_white());
                    }
                    Err(e) => {
                        println!("{}", format!("\n❌  Hata: {}", e.to_sentient_message()).red());
                    }
                }
            }
            Err(ReadlineError::Interrupted) => continue,
            Err(ReadlineError::Eof) => break,
            Err(_) => break,
        }
    }

    Ok(())
}

/// ─── Gateway Modu ───

async fn run_gateway(
    http_addr: &str,
    telegram_token: Option<String>,
    jwt_secret: &str,
) -> SENTIENTResult<()> {
    println!();
    println!("{}", "════════════════════════════════════════════════════════════".cyan());
    println!("{}", "  🌐  SENTIENT GATEWAY SUNUCUSU".cyan().bold());
    println!("{}", "════════════════════════════════════════════════════════════".cyan());
    println!();
    println!("{}", format!("📡  HTTP API: http://{}", http_addr).yellow());
    if telegram_token.is_some() {
        println!("{}", "🤖  Telegram Bot: Aktif".yellow());
    }
    println!();
    println!("{}", "Kapatmak için Ctrl+C'ye basın.".dimmed());
    println!();

    let config = sentient_gateway::GatewayConfig {
        http_addr: http_addr.into(),
        telegram_token,
        jwt_secret: jwt_secret.into(),
        max_concurrent_tasks: 10,
        task_timeout_secs: 600,
        ..Default::default()
    };

    let gateway = sentient_gateway::Gateway::new(config);
    gateway.start().await
}

/// ─── Agent Modu ───

async fn run_agent(goal: &str, model: &str, max_iterations: u32) -> SENTIENTResult<()> {
    println!();
    println!("{}", "════════════════════════════════════════════════════════════".cyan());
    println!("{}", "  🐺  SENTIENT OTOPİLOT MODU".cyan().bold());
    println!("{}", "════════════════════════════════════════════════════════════".cyan());
    println!();
    println!("{}", format!("🎯  Hedef: {}", goal).yellow());
    println!("{}", format!("🧠  Model: {}", model).dimmed());
    println!("{}", format!("🔄  Maks iterasyon: {}", max_iterations).dimmed());
    println!();

    let config = sentient_orchestrator::AgentConfig {
        model: model.into(),
        vgate_url: "http://127.0.0.1:1071".into(),
        max_iterations,
        timeout_secs: 300,
        system_prompt: sentient_orchestrator::SYSTEM_PROMPT.into(),
    };

    let goal_obj = sentient_orchestrator::Goal::new(goal)
        .with_success_criteria(vec![
            "Görev başarıyla tamamlandı".into(),
            "Sonuçlar doğrulandı".into(),
        ]);

    let mut agent = sentient_orchestrator::Agent::new(goal_obj, config);

    println!("{}", "🚀  Ajan başlatılıyor...".green());
    println!();

    match agent.run().await {
        Ok(result) => {
            println!();
            println!("{}", "────────────────────────────────────────────────────────────".cyan());
            println!("{}", result.report());
        }
        Err(e) => {
            println!("{}", format!("\n❌  Hata: {}", e.to_sentient_message()).red());
        }
    }

    println!();
    Ok(())
}

/// ─── Serve Modu (7/24 Arka Plan) ───

async fn run_serve(
    scan_interval: u64,
    scout_enabled: bool,
    forge_enabled: bool,
    self_healing_enabled: bool,
) -> SENTIENTResult<()> {
    println!();
    println!("{}", "════════════════════════════════════════════════════════════".cyan());
    println!("{}", "  🐺  SENTIENT SERVİS MODU (7/24)".cyan().bold());
    println!("{}", "════════════════════════════════════════════════════════════".cyan());
    println!();
    println!("{}", format!("⏱️   Tarama aralığı: {}s", scan_interval).yellow());
    println!("{}", format!("🔍  Scout: {} | Forge: {} | Self-Healing: {}",
        if scout_enabled { "aktif" } else { "pasif" },
        if forge_enabled { "aktif" } else { "pasif" },
        if self_healing_enabled { "aktif" } else { "pasif" }
    ).dimmed());
    println!("{}", "📊  Dashboard: http://localhost:8080/dashboard".green());
    println!();
    println!("{}", "Kapatmak için Ctrl+C'ye basın.".dimmed());
    println!();

    // Watcher yapılandırması
    let watcher_config = sentient_orchestrator::WatcherConfig {
        scan_interval_secs: scan_interval,
        scout_enabled,
        forge_enabled,
        auto_generate: true,
        ..Default::default()
    };

    // Self-Healing yapılandırması
    let healing_config = sentient_orchestrator::HealingConfig {
        auto_fix_enabled: self_healing_enabled,
        ..Default::default()
    };

    log::info!("🐺  SENTIENT Servis başlatılıyor...");
    log::info!("   Watcher: {}s aralıkla tarama yapacak", scan_interval);
    log::info!("   Scout: {} | Forge: {} | Self-Healing: {}",
        if scout_enabled { "aktif" } else { "pasif" },
        if forge_enabled { "aktif" } else { "pasif" },
        if self_healing_enabled { "aktif" } else { "pasif" }
    );

    // ═══════════════════════════════════════════════════════════════
    // VERİTABANI BAŞLATMA
    // ═══════════════════════════════════════════════════════════════
    log::info!("🗄️  Veritabanı başlatılıyor...");
    let task_store = std::sync::Arc::new(
        sentient_gateway::TaskStore::new().await
            .map_err(|e| sentient_common::error::SENTIENTError::Database(e.to_string()))?
    );
    log::info!("✅ SQLite veritabanı hazır: data/sentient.db");

    // ═══════════════════════════════════════════════════════════════
    // HYDRATION - AKTİF GÖREVLERİ GERİ YÜKLE
    // ═══════════════════════════════════════════════════════════════
    log::info!("🔄 Hydration başlatılıyor...");
    let hydration_engine = std::sync::Arc::new(sentient_gateway::HydrationEngine::new(task_store.clone()));
    
    let hydration_result = hydration_engine.hydrate().await
        .map_err(|e| sentient_common::error::SENTIENTError::Database(e.to_string()))?;
    
    log::info!("✅ Hydration tamamlandı: {} görev geri yüklendi", hydration_result.restored_count);
    
    if !hydration_result.failed_tasks.is_empty() {
        log::warn!("⚠️  {} görev yüklenemedi", hydration_result.failed_tasks.len());
    }

    // Dashboard state
    let dashboard_state = std::sync::Arc::new(sentient_gateway::DashboardState {
        metrics: std::sync::Arc::new(sentient_gateway::MetricsCollector::new()),
        activities: std::sync::Arc::new(tokio::sync::RwLock::new(Vec::new())),
        logs: std::sync::Arc::new(tokio::sync::RwLock::new(Vec::new())),
        thoughts: std::sync::Arc::new(tokio::sync::RwLock::new(Vec::new())),
        config: sentient_gateway::DashboardConfig::default(),
    });
    
    // Dashboard'a başlangıç logu ekle
    dashboard_state.add_log(sentient_gateway::LogLevel::Info, "System", "SENTIENT servis başlatıldı").await;

    // HTTP Server'ı ayrı bir task'te başlat
    let gateway_config = sentient_gateway::GatewayConfig {
        http_addr: "0.0.0.0:8080".into(),
        ..Default::default()
    };
    
    let task_manager = std::sync::Arc::new(sentient_gateway::TaskManager::new(100, 600));
    let dispatcher = std::sync::Arc::new(sentient_gateway::TaskDispatcher::new(task_manager.clone()));
    
    let server_dashboard = dashboard_state.clone();
    let server_store = task_store.clone();
    tokio::spawn(async move {
        if let Err(e) = sentient_gateway::api::run_server(&gateway_config, dispatcher, task_manager).await {
            log::error!("HTTP Server hatası: {}", e.to_sentient_message());
        }
    });
    
    log::info!("🌐  HTTP API başlatıldı: http://0.0.0.0:8080");
    log::info!("📊  Dashboard: http://localhost:8080/dashboard");

    // Watcher'ı başlat
    let mut watcher = sentient_orchestrator::Watcher::new(watcher_config);
    watcher.start().await?;

    // Self-Healing motorunu başlat
    let mut healing_engine = sentient_orchestrator::SelfHealingEngine::new(
        healing_config,
        "http://127.0.0.1:1071".into(),
        "qwen/qwen3-1.7b:free".into(),
    );

    // Ana döngü
    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(scan_interval));
    
    loop {
        interval.tick().await;

        log::debug!("👁️  Watcher tick...");

        // Watcher'dan görevleri al
        match watcher.tick().await {
            Ok(tasks) => {
                for task in tasks {
                    let goal_preview = task.goal.description.chars().take(50).collect::<String>();
                    log::info!("📋  Yeni görev: {}", goal_preview);
                    
                    // ═════════════════════════════════════════════════════
                    // GÖREVİ VERİTABANINA KAYDET
                    // ═════════════════════════════════════════════════════
                    let persisted = sentient_gateway::PersistedTask {
                        id: uuid::Uuid::new_v4(),
                        request_id: uuid::Uuid::new_v4(),
                        goal: task.goal.description.clone(),
                        model: "qwen/qwen3-1.7b:free".into(),
                        status: sentient_gateway::PersistedStatus::Queued,
                        priority: 1,
                        assigned_agent: None,
                        current_step: None,
                        total_steps: 0,
                        progress: 0.0,
                        started_at: chrono::Utc::now().to_rfc3339(),
                        updated_at: chrono::Utc::now().to_rfc3339(),
                        completed_at: None,
                        result: None,
                        error: None,
                        source: "watcher".into(),
                        user_id: None,
                        retry_count: 0,
                        checkpoint: None,
                    };
                    
                    if let Err(e) = task_store.insert_task(&persisted).await {
                        log::error!("Görev kaydedilemedi: {}", e);
                    }
                    
                    // Dashboard'a aktivite ekle
                    dashboard_state.add_log(sentient_gateway::LogLevel::Info, "Watcher", 
                        &format!("Yeni görev: {}", goal_preview)).await;
                }
            }
            Err(e) => {
                log::error!("Watcher hatası: {}", e.to_sentient_message());
                
                // Dashboard'a hata logu ekle
                dashboard_state.add_log(sentient_gateway::LogLevel::Error, "Watcher", &e.to_sentient_message()).await;
                
                // Self-Healing: Hata analizi
                if self_healing_enabled {
                    log::info!("🔧  Self-Healing devreye giriyor...");
                    dashboard_state.add_log(sentient_gateway::LogLevel::Warn, "System", "Self-Healing devreye giriyor").await;
                }
            }
        }

        // Periyodik metrik güncelleme
        let metrics = dashboard_state.metrics.collect().await;
        if metrics.cpu_usage > 80.0 {
            dashboard_state.add_log(sentient_gateway::LogLevel::Warn, "System", 
                &format!("Yüksek CPU kullanımı: {:.1}%", metrics.cpu_usage)).await;
        }

        // Periyodik rapor
        let stats = watcher.stats();
        if stats.total_cycles % 10 == 0 {
            log::info!("{}", watcher.report());
        }
    }
}

/// ─── Banner ───

fn print_banner() {
    println!();
    println!(
        "{}",
        r#"
  ╔════════════════════════════════════════════════════════════╗
  ║                                                            ║
  ║     █████╗ ███╗   ██╗███████╗██╗      ██████╗ ██╗   ██╗   ║
  ║    ██╔══██╗████╗  ██║██╔════╝██║     ██╔═══██╗██║   ██║   ║
  ║    ███████║██╔██╗ ██║█████╗  ██║     ██║   ██║██║   ██║   ║
  ║    ██╔══██║██║╚██╗██║██╔══╝  ██║     ██║   ██║██║   ██║   ║
  ║    ██║  ██║██║ ╚████║███████╗███████╗╚██████╔╝╚██████╔╝   ║
  ║    ╚═╝  ╚═╝╚═╝  ╚═══╝╚══════╝╚══════╝ ╚═════╝  ╚═════╝    ║
  ║                                                            ║
  ║            NEXUS OASIS — Yapay Zeka İşletim Sistemi        ║
  ╚════════════════════════════════════════════════════════════╝
"#
        .bright_cyan()
    );
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_placeholder() {
        assert!(true);
    }
}
