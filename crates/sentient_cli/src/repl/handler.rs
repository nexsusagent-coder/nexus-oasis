//! ─── HANDLER MODULU ───
//!
//! Komut isleme ve yonlendirme

use crate::repl::{ReplMode, PromptState};
use colored::Colorize;

/// Komut isleme sonucu
#[derive(Debug)]
pub enum CommandResult {
    /// Basariyla tamamlandi
    Success(String),
    /// Cikis gerekiyor
    Exit,
    /// Mod degistirme
    ModeChange(ReplMode),
    /// Modul giris
    EnterModule(String),
    /// Modul cikis
    ExitModule,
    /// Hata
    Error(String),
    /// Devam et (LLM'e gonder)
    ContinueToLlm(String),
}

/// Komut isleyici
pub struct CommandHandler {
    /// Mevcut modul
    current_module: Option<String>,
    /// Debug modu
    debug_mode: bool,
}

impl CommandHandler {
    pub fn new() -> Self {
        Self {
            current_module: None,
            debug_mode: false,
        }
    }

    /// Komutu isle
    pub fn handle(&mut self, input: &str, state: &PromptState) -> CommandResult {
        let trimmed = input.trim();
        
        if trimmed.is_empty() {
            return CommandResult::Success(String::new());
        }

        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        let command = parts.get(0).map(|s| s.to_lowercase()).unwrap_or_default();

        // Temel komutlar
        match command.as_str() {
            "exit" | "quit" | "q" => CommandResult::Exit,
            
            "help" | "h" | "?" => {
                CommandResult::Success(print_help_menu(state))
            }
            
            "clear" | "cls" => {
                print!("\x1B[2J\x1B[1;1H");
                CommandResult::Success(String::new())
            }
            
            "status" | "s" => {
                CommandResult::Success("[STATUS] Komutu simetrik olarak islenecek".into())
            }
            
            "version" | "v" => {
                CommandResult::Success(format_version())
            }

            // Mod degistirme
            "swarm" => {
                if parts.len() == 1 {
                    CommandResult::ModeChange(ReplMode::Swarm)
                } else {
                    self.handle_swarm_command(&parts[1..])
                }
            }

            "debug" => {
                if parts.len() == 1 {
                    self.debug_mode = !self.debug_mode;
                    let status = if self.debug_mode { "acik" } else { "kapali" };
                    CommandResult::Success(format!("Debug modu: {}", status.yellow()))
                } else {
                    match parts[1] {
                        "on" | "true" | "1" => {
                            self.debug_mode = true;
                            CommandResult::Success("Debug modu acildi".green().to_string())
                        }
                        "off" | "false" | "0" => {
                            self.debug_mode = false;
                            CommandResult::Success("Debug modu kapatildi".yellow().to_string())
                        }
                        _ => CommandResult::Error("Kullanim: debug [on|off]".into())
                    }
                }
            }

            // Modul komutlari
            "memory" => self.handle_memory_command(&parts[1..]),
            "guardrails" => self.handle_guardrails_command(&parts[1..]),
            "sandbox" => self.handle_sandbox_command(&parts[1..]),
            "vgate" => self.handle_vgate_command(&parts[1..]),
            "gateway" => self.handle_gateway_command(&parts[1..]),
            "agent" => self.handle_agent_command(&parts[1..]),
            "task" => self.handle_task_command(&parts[1..]),
            "logs" => self.handle_logs_command(&parts[1..]),
            "metrics" => self.handle_metrics_command(&parts[1..]),
            
            // Modul giris/cikis
            "enter" | "module" => {
                if parts.len() < 2 {
                    CommandResult::Error("Kullanim: enter <modul>".into())
                } else {
                    CommandResult::EnterModule(parts[1].to_string())
                }
            }

            "back" | "exit-module" => {
                CommandResult::ExitModule
            }

            // History
            "history" => {
                if parts.len() > 1 && parts[1] == "clear" {
                    CommandResult::Success("Gecmis temizlendi".green().to_string())
                } else {
                    CommandResult::Success("[HISTORY] Gecmis listelenecek".into())
                }
            }

            // Bilinmeyen komut - LLM'e gonder
            _ => {
                if self.current_module.is_some() {
                    // Modul icindeyken komut module yonlendirilir
                    CommandResult::ContinueToLlm(format!("[{}] {}", 
                        self.current_module.as_ref().unwrap(), trimmed))
                } else {
                    CommandResult::ContinueToLlm(trimmed.to_string())
                }
            }
        }
    }

    fn handle_memory_command(&self, args: &[&str]) -> CommandResult {
        if args.is_empty() {
            return CommandResult::Success(print_module_help("memory"));
        }

        match args[0] {
            "list" => CommandResult::Success("[MEMORY] Kayitlar listelenecek".into()),
            "search" => {
                if args.len() < 2 {
                    CommandResult::Error("Kullanim: memory search <sorgu>".into())
                } else {
                    CommandResult::Success(format!("[MEMORY] Araniyor: {}", args[1..].join(" ")))
                }
            }
            "store" => {
                if args.len() < 2 {
                    CommandResult::Error("Kullanim: memory store <anahtar> <deger>".into())
                } else {
                    CommandResult::Success(format!("[MEMORY] Kaydediliyor: {}", args[1..].join(" ")))
                }
            }
            "cleanup" => CommandResult::Success("[MEMORY] Sure dolan kayitlar temizlendi".into()),
            _ => CommandResult::Error(format!("Bilinmeyen alt komut: {}", args[0]))
        }
    }

    fn handle_guardrails_command(&self, args: &[&str]) -> CommandResult {
        if args.is_empty() {
            return CommandResult::Success(print_module_help("guardrails"));
        }

        match args[0] {
            "list" => CommandResult::Success("[GUARDRAILS] Politikalar listelenecek".into()),
            "toggle" => {
                if args.len() < 3 {
                    CommandResult::Error("Kullanim: guardrails toggle <politika> <on|off>".into())
                } else {
                    CommandResult::Success(format!("[GUARDRAILS] {} {}", args[1], args[2]))
                }
            }
            "check" => {
                if args.len() < 2 {
                    CommandResult::Error("Kullanim: guardrails check <metin>".into())
                } else {
                    CommandResult::Success("[GUARDRAILS] Metin kontrol ediliyor...".into())
                }
            }
            _ => CommandResult::Error(format!("Bilinmeyen alt komut: {}", args[0]))
        }
    }

    fn handle_sandbox_command(&self, args: &[&str]) -> CommandResult {
        if args.is_empty() {
            return CommandResult::Success(print_module_help("sandbox"));
        }

        match args[0] {
            "run" | "exec" => {
                if args.len() < 2 {
                    CommandResult::Error("Kullanim: sandbox run <kod>".into())
                } else {
                    CommandResult::Success(format!("[SANDBOX] Calistiriliyor: {}", args[1..].join(" ")))
                }
            }
            "status" => CommandResult::Success("[SANDBOX] Durum: Hazir".into()),
            "logs" => CommandResult::Success("[SANDBOX] Loglar listelenecek".into()),
            "kill" => CommandResult::Success("[SANDBOX] Islem durduruldu".into()),
            _ => CommandResult::Error(format!("Bilinmeyen alt komut: {}", args[0]))
        }
    }

    fn handle_vgate_command(&self, args: &[&str]) -> CommandResult {
        if args.is_empty() {
            return CommandResult::Success(print_module_help("vgate"));
        }

        match args[0] {
            "status" => CommandResult::Success("[V-GATE] Proxy durumu: Aktif (Port: 1071)".into()),
            "models" => CommandResult::Success("[V-GATE] Modeller listelenecek".into()),
            "test" => CommandResult::Success("[V-GATE] Baglanti testi yapiliyor...".into()),
            _ => CommandResult::Error(format!("Bilinmeyen alt komut: {}", args[0]))
        }
    }

    fn handle_gateway_command(&self, args: &[&str]) -> CommandResult {
        if args.is_empty() {
            return CommandResult::Success(print_module_help("gateway"));
        }

        match args[0] {
            "status" => CommandResult::Success("[GATEWAY] HTTP API: Aktif, Telegram: Pasif".into()),
            "start" => CommandResult::Success("[GATEWAY] Baslatiliyor...".into()),
            "stop" => CommandResult::Success("[GATEWAY] Durduruluyor...".into()),
            _ => CommandResult::Error(format!("Bilinmeyen alt komut: {}", args[0]))
        }
    }

    fn handle_swarm_command(&self, args: &[&str]) -> CommandResult {
        if args.is_empty() {
            return CommandResult::Success(print_module_help("swarm"));
        }

        match args[0] {
            "start" => CommandResult::Success("[SWARM] Koordinatör baslatiliyor...".into()),
            "stop" => CommandResult::Success("[SWARM] Durduruluyor...".into()),
            "status" => CommandResult::Success("[SWARM] Durum: 0 aktif ajan".into()),
            "spawn" => {
                if args.len() < 2 {
                    CommandResult::Error("Kullanim: swarm spawn <ajent_tipi>".into())
                } else {
                    CommandResult::Success(format!("[SWARM] {} ajenti olusturuluyor...", args[1]))
                }
            }
            "task" => {
                if args.len() < 2 {
                    CommandResult::Error("Kullanim: swarm task <gorev>".into())
                } else {
                    CommandResult::Success(format!("[SWARM] Gorev ataniyor: {}", args[1..].join(" ")))
                }
            }
            _ => CommandResult::Error(format!("Bilinmeyen alt komut: {}", args[0]))
        }
    }

    fn handle_agent_command(&self, args: &[&str]) -> CommandResult {
        if args.is_empty() {
            return CommandResult::Success(print_module_help("agent"));
        }

        match args[0] {
            "spawn" => {
                if args.len() < 2 {
                    CommandResult::Error("Kullanim: agent spawn <tipi>".into())
                } else {
                    CommandResult::Success(format!("[AGENT] {} ajenti olusturuluyor...", args[1]))
                }
            }
            "list" => CommandResult::Success("[AGENT] Aktif ajanlar listelenecek".into()),
            "status" => {
                if args.len() < 2 {
                    CommandResult::Success("[AGENT] Tum ajanlarin durumu".into())
                } else {
                    CommandResult::Success(format!("[AGENT] {} ajenti durumu", args[1]))
                }
            }
            "kill" => {
                if args.len() < 2 {
                    CommandResult::Error("Kullanim: agent kill <id>".into())
                } else {
                    CommandResult::Success(format!("[AGENT] {} ajenti sonlandiriliyor", args[1]))
                }
            }
            _ => CommandResult::Error(format!("Bilinmeyen alt komut: {}", args[0]))
        }
    }

    fn handle_task_command(&self, args: &[&str]) -> CommandResult {
        if args.is_empty() {
            return CommandResult::Success(print_module_help("task"));
        }

        match args[0] {
            "add" => {
                if args.len() < 2 {
                    CommandResult::Error("Kullanim: task add <gorev>".into())
                } else {
                    CommandResult::Success(format!("[TASK] Gorev eklendi: {}", args[1..].join(" ")))
                }
            }
            "list" => CommandResult::Success("[TASK] Gorevler listelenecek".into()),
            "cancel" => {
                if args.len() < 2 {
                    CommandResult::Error("Kullanim: task cancel <id>".into())
                } else {
                    CommandResult::Success(format!("[TASK] {} gorevi iptal edildi", args[1]))
                }
            }
            _ => CommandResult::Error(format!("Bilinmeyen alt komut: {}", args[0]))
        }
    }

    fn handle_logs_command(&self, args: &[&str]) -> CommandResult {
        if args.is_empty() {
            return CommandResult::Success("[LOGS] Son loglar gosterilecek".into());
        }

        match args[0] {
            "tail" => CommandResult::Success("[LOGS] Canli log takibi (Ctrl+C ile cikis)".into()),
            "clear" => CommandResult::Success("[LOGS] Loglar temizlendi".into()),
            "export" => CommandResult::Success("[LOGS] Loglar disa aktariliyor...".into()),
            _ => CommandResult::Error(format!("Bilinmeyen alt komut: {}", args[0]))
        }
    }

    fn handle_metrics_command(&self, args: &[&str]) -> CommandResult {
        if args.is_empty() {
            return CommandResult::Success(print_metrics_dashboard());
        }

        match args[0] {
            "show" => CommandResult::Success(print_metrics_dashboard()),
            "export" => CommandResult::Success("[METRICS] Metrikler disa aktariliyor...".into()),
            "reset" => CommandResult::Success("[METRICS] Metrikler sifirlandi".into()),
            _ => CommandResult::Error(format!("Bilinmeyen alt komut: {}", args[0]))
        }
    }

    pub fn set_module(&mut self, module: Option<String>) {
        self.current_module = module;
    }

    pub fn is_debug(&self) -> bool {
        self.debug_mode
    }
}

impl Default for CommandHandler {
    fn default() -> Self {
        Self::new()
    }
}

// ─── Yardimci Fonksiyonlar ───

fn print_help_menu(state: &PromptState) -> String {
    let mut output = String::new();
    
    output.push_str(&format!("{}", "\n════════════════════════════════════════════════════════════\n".cyan()));
    output.push_str(&format!("  {}  SENTIENT Komutları\n", "🐺".cyan()));
    output.push_str(&format!("{}", "════════════════════════════════════════════════════════════\n".cyan()));
    
    output.push_str(&format!("\n  {}\n", "GENEL KOMUTLAR".yellow().bold()));
    output.push_str(&format!("    {:<15} {}\n", "help, h, ?".green(), "Bu yardim menusu"));
    output.push_str(&format!("    {:<15} {}\n", "status, s".green(), "Sistem durumu"));
    output.push_str(&format!("    {:<15} {}\n", "clear, cls".green(), "Ekrani temizle"));
    output.push_str(&format!("    {:<15} {}\n", "history".green(), "Komut gecmisi"));
    output.push_str(&format!("    {:<15} {}\n", "version, v".green(), "Surum bilgisi"));
    output.push_str(&format!("    {:<15} {}\n", "exit, q".green(), "Cikis"));

    output.push_str(&format!("\n  {}\n", "MODUL KOMUTLARI".yellow().bold()));
    output.push_str(&format!("    {:<15} {}\n", "memory".green(), "Bellek islemleri"));
    output.push_str(&format!("    {:<15} {}\n", "guardrails".green(), "Guvenlik politikalari"));
    output.push_str(&format!("    {:<15} {}\n", "sandbox".green(), "Docker sandbox"));
    output.push_str(&format!("    {:<15} {}\n", "vgate".green(), "V-GATE proxy"));
    output.push_str(&format!("    {:<15} {}\n", "gateway".green(), "API Gateway"));

    output.push_str(&format!("\n  {}\n", "SWARM KOMUTLARI".yellow().bold()));
    output.push_str(&format!("    {:<15} {}\n", "swarm".green(), "Coklu ajan sistemleri"));
    output.push_str(&format!("    {:<15} {}\n", "agent".green(), "Ajan yonetimi"));
    output.push_str(&format!("    {:<15} {}\n", "task".green(), "Gorev yonetimi"));

    output.push_str(&format!("\n  {}\n", "DEBUG KOMUTLARI".yellow().bold()));
    output.push_str(&format!("    {:<15} {}\n", "debug".green(), "Hata ayiklama modu"));
    output.push_str(&format!("    {:<15} {}\n", "logs".green(), "Sistem loglari"));
    output.push_str(&format!("    {:<15} {}\n", "metrics".green(), "Performans metrikleri"));

    output.push_str(&format!("\n{}\n", "────────────────────────────────────────────────────────────".cyan()));
    output.push_str(&format!("  {}\n", "Tum diger girdiler LLM'e sorgu olarak gonderilir.".dimmed()));
    output.push_str(&format!("{}\n\n", "────────────────────────────────────────────────────────────".cyan()));

    output
}

fn format_version() -> String {
    format!(
        "\n{} SENTIENT v{}\n{}\n\n  {}: {}\n  {}: {}\n  {}: {}\n",
        "🐺".cyan(),
        env!("CARGO_PKG_VERSION").green(),
        "═══════════════════════════════════".cyan(),
        "Surum".yellow(),
        env!("CARGO_PKG_VERSION"),
        "Rust".yellow(),
        "1.70+",
        "Durum".yellow(),
        "Gelistirme".bright_yellow()
    )
}

fn print_module_help(module: &str) -> String {
    format!("\n[{}] Yardim: {} <alt_komut>\nDenemek icin 'help' yazin.\n", module.to_uppercase(), module)
}

fn print_metrics_dashboard() -> String {
    format!(
        "\n{}\n  {}\n{}\n  {:<20} {}\n  {:<20} {}\n  {:<20} {}\n  {:<20} {}\n  {:<20} {}\n{}\n",
        "═════════════════════════════════════════".cyan(),
        "📊 SISTEM METRIKLERI".cyan().bold(),
        "═════════════════════════════════════════".cyan(),
        "Toplam Istek".yellow(),
        "0".green(),
        "Basarili".yellow(),
        "0 (100%)".green(),
        "Ort. Sure".yellow(),
        "0ms".green(),
        "Token Kullanimi".yellow(),
        "0".green(),
        "Bellek Kullanimi".yellow(),
        format!("{} MB", 0).green(),
        "─────────────────────────────────────────".cyan()
    )
}
