//! ═══════════════════════════════════════════════════════════════════════════════
//!  🧠 SENTIENT SHELL - Native Hybrid Terminal
//!  The Operating System That Thinks
//! ═══════════════════════════════════════════════════════════════════════════════

use std::collections::HashMap;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;
use std::sync::Arc;
use tokio::sync::RwLock;

/// SENTIENT Shell Ana Yapısı
pub struct SENTIENTShell {
    /// Shell geçmişi
    history: Vec<String>,
    /// Mevcut çalışma dizini
    current_dir: PathBuf,
    /// Alias'lar
    aliases: HashMap<String, String>,
    /// Çalışma modu
    mode: ShellMode,
    /// V-GATE durumu
    vgate_connected: bool,
}

/// Shell çalışma modu
#[derive(Debug, Clone, PartialEq)]
pub enum ShellMode {
    /// Normal bash modu
    Bash,
    /// SENTIENT komut modu
    SENTIENT,
    /// Google CLI modu
    Google,
    /// Skill modu
    Skill,
}

/// Komut sonucu
#[derive(Debug, Clone)]
pub enum CommandResult {
    Exit,
    Continue,
    Output(String),
    Error(String),
}

impl SENTIENTShell {
    pub fn new() -> Self {
        let mut aliases = HashMap::new();
        aliases.insert("ll".to_string(), "ls -la".to_string());
        aliases.insert("la".to_string(), "ls -a".to_string());
        aliases.insert("..".to_string(), "cd ..".to_string());
        aliases.insert("gs".to_string(), "git status".to_string());
        
        Self {
            history: Vec::new(),
            current_dir: std::env::current_dir().unwrap_or_else(|_| PathBuf::from("/")),
            aliases,
            mode: ShellMode::Bash,
            vgate_connected: false,
        }
    }
    
    /// Shell'i başlat
    pub async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.print_welcome();
        
        loop {
            let prompt = self.get_prompt();
            let input = self.read_line(&prompt)?;
            
            if input.is_empty() {
                continue;
            }
            
            self.history.push(input.clone());
            
            match self.process_command(&input).await {
                CommandResult::Exit => {
                    println!("👋 SENTIENT Shell kapatılıyor...");
                    break;
                }
                CommandResult::Continue => continue,
                CommandResult::Output(msg) => println!("{}", msg),
                CommandResult::Error(msg) => eprintln!("❌ {}", msg),
            }
        }
        
        Ok(())
    }
    
    /// Komut işle
    async fn process_command(&mut self, input: &str) -> CommandResult {
        let input = input.trim();
        
        // SENTIENT özel komutları
        if input.starts_with('/') {
            return self.handle_sentient_command(input).await;
        }
        
        // Mode değiştirme
        if input.starts_with("mode:") {
            return self.handle_mode_change(input);
        }
        
        // Alias çözümle
        let resolved = self.resolve_alias(input);
        
        // Yerleşik komutlar
        match resolved.split_whitespace().next() {
            Some("exit") | Some("quit") => return CommandResult::Exit,
            Some("help") => return self.show_help(),
            Some("status") => return self.show_status(),
            Some("skills") => return self.show_skills().await,
            Some("history") => return self.show_history(),
            Some("cd") => return self.change_directory(&resolved),
            Some("clear") => {
                print!("\x1B[2J\x1B[1;1H");
                return CommandResult::Continue;
            }
            _ => {}
        }
        
        // Harici komut
        self.execute_external(&resolved)
    }
    
    /// SENTIENT komutları
    async fn handle_sentient_command(&mut self, input: &str) -> CommandResult {
        let parts: Vec<&str> = input.split_whitespace().collect();
        
        match parts.get(0).map(|s| *s) {
            Some("/help") => self.show_help(),
            Some("/status") => self.show_status(),
            Some("/skills") => self.show_skills().await,
            Some("/search") => {
                if parts.len() > 1 {
                    CommandResult::Output(format!("🔍 Aranıyor: {}", parts[1..].join(" ")))
                } else {
                    CommandResult::Output("Kullanım: /search <sorgu>".to_string())
                }
            }
            Some("/skill") => {
                if parts.len() > 1 {
                    CommandResult::Output(format!("📦 Skill: {}", parts[1..].join(" ")))
                } else {
                    CommandResult::Output("Kullanım: /skill <ad>".to_string())
                }
            }
            Some("/team") => {
                if parts.len() > 1 {
                    CommandResult::Output(format!("👥 Takım: {}", parts[1..].join(" ")))
                } else {
                    CommandResult::Output("Kullanım: /team <görev>".to_string())
                }
            }
            Some("/vgate") => self.toggle_vgate(),
            Some("/exit") => CommandResult::Exit,
            _ => CommandResult::Error(format!("Bilinmeyen komut: {}", input))
        }
    }
    
    /// Mode değiştir
    fn handle_mode_change(&mut self, input: &str) -> CommandResult {
        let mode_str = input.strip_prefix("mode:").unwrap_or(input);
        
        self.mode = match mode_str.to_lowercase().as_str() {
            "bash" | "b" => ShellMode::Bash,
            "sentient" | "a" => ShellMode::SENTIENT,
            "google" | "g" => ShellMode::Google,
            "skill" | "s" => ShellMode::Skill,
            _ => return CommandResult::Error(format!("Bilinmeyen mod: {}", mode_str)),
        };
        
        CommandResult::Output(format!("Mod: {:?}", self.mode))
    }
    
    /// Alias çözümle
    fn resolve_alias(&self, input: &str) -> String {
        let first_word = input.split_whitespace().next().unwrap_or(input);
        
        if let Some(expanded) = self.aliases.get(first_word) {
            input.replacen(first_word, expanded, 1)
        } else {
            input.to_string()
        }
    }
    
    /// Dizin değiştir
    fn change_directory(&mut self, input: &str) -> CommandResult {
        let parts: Vec<&str> = input.split_whitespace().collect();
        let target = parts.get(1).unwrap_or(&"~");
        
        let new_dir = if *target == "~" {
            dirs::home_dir().unwrap_or_else(|| PathBuf::from("/"))
        } else {
            self.current_dir.join(target)
        };
        
        if new_dir.exists() && new_dir.is_dir() {
            self.current_dir = new_dir.canonicalize().unwrap_or(new_dir);
            std::env::set_current_dir(&self.current_dir).ok();
            CommandResult::Continue
        } else {
            CommandResult::Error(format!("Dizin bulunamadı: {}", target))
        }
    }
    
    /// Harici komut
    fn execute_external(&self, input: &str) -> CommandResult {
        let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/bash".to_string());
        
        let result = Command::new(&shell)
            .arg("-c")
            .arg(input)
            .current_dir(&self.current_dir)
            .status();
        
        match result {
            Ok(_) => CommandResult::Continue,
            Err(e) => CommandResult::Error(format!("{}: {}", input, e)),
        }
    }
    
    /// Skill'leri göster
    async fn show_skills(&self) -> CommandResult {
        CommandResult::Output(format!(
            "📦 SENTIENT Skill Library\n\n\
             Toplam: 5587+ skill\n\
             Kategori: 29 adet\n\n\
             Öne çıkan kategoriler:\n\
             • Coding Agents & IDEs: 1160+\n\
             • Web & Frontend Dev: 900+\n\
             • DevOps & Cloud: 375+\n\
             • Browser Automation: 336+"
        ))
    }
    
    /// V-GATE toggle
    fn toggle_vgate(&mut self) -> CommandResult {
        self.vgate_connected = !self.vgate_connected;
        let status = if self.vgate_connected { "✅ BAĞLI" } else { "❌ KESİK" };
        CommandResult::Output(format!("🔐 V-GATE: {}", status))
    }
    
    /// Geçmiş
    fn show_history(&self) -> CommandResult {
        let mut output = "📜 Komut Geçmişi:\n".to_string();
        
        for (i, cmd) in self.history.iter().rev().take(20).enumerate() {
            output.push_str(&format!("  {:>3} {}\n", i + 1, cmd));
        }
        
        CommandResult::Output(output)
    }
    
    /// Durum
    fn show_status(&self) -> CommandResult {
        let vgate_status = if self.vgate_connected { "✅ BAĞLI" } else { "❌ KESİK" };
        
        CommandResult::Output(format!(
            "🐺 SENTIENT STATUS\n\n\
             📂 Dizin: {}\n\
             🔧 Mod: {:?}\n\
             🔐 V-GATE: {}\n\
             📦 Skills: 5587+\n\
             📜 Geçmiş: {} komut",
            self.current_dir.display(),
            self.mode,
            vgate_status,
            self.history.len()
        ))
    }
    
    /// Yardım
    fn show_help(&self) -> CommandResult {
        CommandResult::Output(r#"
🐺 SENTIENT SHELL - YARDIM

📌 SENTIENT KOMUTLARI:
  /help           - Yardım
  /status         - Durum
  /skills         - Skill listesi
  /skill <ad>     - Skill çalıştır
  /search <sorgu> - Web ara
  /team <görev>   - Takım spawn
  /vgate          - V-GATE toggle
  /history        - Geçmiş
  /clear          - Temizle
  /exit           - Çıkış

📌 MOD'LAR:
  mode:bash       - Bash modu
  mode:sentient      - SENTIENT modu
  mode:google     - Google CLI
  mode:skill      - Skill modu

📌 YERLEŞİK KOMUTLAR:
  cd, ls, pwd, cat, grep...
  exit, help, clear, history

"#.to_string())
    }
    
    /// Prompt
    fn get_prompt(&self) -> String {
        let mode_indicator = match self.mode {
            ShellMode::Bash => "",
            ShellMode::SENTIENT => "🧠",
            ShellMode::Google => "🔷",
            ShellMode::Skill => "📦",
        };
        
        let vgate_indicator = if self.vgate_connected { "🔐" } else { "" };
        
        let dir = self.current_dir.display().to_string();
        let short_dir = if dir.starts_with("/root") {
            dir.replace("/root", "~")
        } else {
            dir
        };
        
        format!("{}{} {} ❯ ", mode_indicator, vgate_indicator, short_dir)
    }
    
    /// Satır oku
    fn read_line(&mut self, prompt: &str) -> Result<String, io::Error> {
        print!("{}", prompt);
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        Ok(input.trim().to_string())
    }
    
    /// Hoşgeldin
    fn print_welcome(&self) {
        println!(r#"
╔═══════════════════════════════════════════════════════════════════╗
║                                                                   ║
║   🧠 SENTIENT SHELL v1.0                                         ║
║   The Operating System That Thinks                               ║
║                                                                   ║
║   📦 5587+ Skills | 🔧 Hybrid Terminal | 🔐 V-GATE               ║
║                                                                   ║
║   /help - Yardım | /skills - Skill'ler | /exit - Çıkış           ║
║                                                                   ║
╚═══════════════════════════════════════════════════════════════════╝
"#);
    }
}

impl Default for SENTIENTShell {
    fn default() -> Self {
        Self::new()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut shell = SENTIENTShell::new();
    shell.run().await
}
