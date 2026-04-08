//! ─── MODUL KOMUTLARI ───
//!
//! Modul bazli komut isleyicileri

use colored::Colorize;

/// Modul yonetimi
pub struct ModuleManager {
    /// Aktif moduller
    active_modules: Vec<String>,
    /// Modul durumları
    module_status: std::collections::HashMap<String, ModuleStatus>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ModuleStatus {
    Running,
    Stopped,
    Error,
    Initializing,
}

impl ModuleManager {
    pub fn new() -> Self {
        let mut manager = Self {
            active_modules: Vec::new(),
            module_status: std::collections::HashMap::new(),
        };
        
        // Tum modulleri kaydet
        manager.register_all();
        manager
    }

    fn register_all(&mut self) {
        let modules = vec![
            ("memory", ModuleStatus::Stopped),
            ("guardrails", ModuleStatus::Stopped),
            ("sandbox", ModuleStatus::Stopped),
            ("vgate", ModuleStatus::Stopped),
            ("gateway", ModuleStatus::Stopped),
            ("orchestrator", ModuleStatus::Stopped),
            ("graph", ModuleStatus::Stopped),
            ("swarm", ModuleStatus::Stopped),
        ];

        for (name, status) in modules {
            self.module_status.insert(name.into(), status);
        }
    }

    /// Modul durumu guncelle
    pub fn set_status(&mut self, module: &str, status: ModuleStatus) {
        self.module_status.insert(module.into(), status);
        
        if status == ModuleStatus::Running {
            if !self.active_modules.contains(&module.to_string()) {
                self.active_modules.push(module.into());
            }
        } else {
            self.active_modules.retain(|m| m != module);
        }
    }

    /// Modul durumu al
    pub fn get_status(&self, module: &str) -> Option<ModuleStatus> {
        self.module_status.get(module).copied()
    }

    /// Aktif moduller
    pub fn active(&self) -> &[String] {
        &self.active_modules
    }

    /// Tum moduller
    pub fn all(&self) -> &std::collections::HashMap<String, ModuleStatus> {
        &self.module_status
    }

    /// Modul durumu gosterimi
    pub fn status_indicator(&self, module: &str) -> String {
        let status = self.module_status.get(module);
        match status {
            Some(ModuleStatus::Running) => "🟢".green().to_string(),
            Some(ModuleStatus::Stopped) => "🔴".red().to_string(),
            Some(ModuleStatus::Error) => "🔴".red().to_string(),
            Some(ModuleStatus::Initializing) => "🟡".yellow().to_string(),
            None => "⚪".dimmed().to_string(),
        }
    }

    /// Modul raporu
    pub fn report(&self) -> String {
        let mut output = String::new();
        
        output.push_str(&format!("\n{}\n", "════════════════════════════════════════════════════════════".cyan()));
        output.push_str(&format!("  {}\n", "📦 MODUL DURUMU".cyan().bold()));
        output.push_str(&format!("{}\n", "════════════════════════════════════════════════════════════".cyan()));

        for (module, status) in self.all() {
            let indicator = self.status_indicator(module);
            let status_text = match status {
                ModuleStatus::Running => "Calisiyor".green(),
                ModuleStatus::Stopped => "Durdu".red(),
                ModuleStatus::Error => "Hata".red(),
                ModuleStatus::Initializing => "Baslatiliyor...".yellow(),
            };
            
            output.push_str(&format!(
                "  {} {:<15} {}\n",
                indicator,
                module.blue(),
                status_text
            ));
        }

        output.push_str(&format!("{}\n\n", "────────────────────────────────────────────────────────────".cyan()));
        output
    }
}

impl Default for ModuleManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Modul alt komutlari
pub fn get_module_subcommands(module: &str) -> Vec<&'static str> {
    match module {
        "memory" => vec!["list", "search", "store", "recall", "cleanup", "export"],
        "guardrails" => vec!["list", "toggle", "check", "test", "report"],
        "sandbox" => vec!["run", "exec", "status", "logs", "kill", "clean"],
        "vgate" => vec!["status", "models", "test", "config", "logs"],
        "gateway" => vec!["start", "stop", "status", "config", "metrics"],
        "orchestrator" => vec!["run", "status", "plan", "cancel", "history"],
        "graph" => vec!["show", "add", "remove", "query", "export"],
        "swarm" => vec!["start", "stop", "status", "spawn", "task", "report"],
        _ => vec![],
    }
}

/// Modul yardim dokumani
pub fn get_module_help(module: &str) -> String {
    let subcommands = get_module_subcommands(module);
    
    let description = match module {
        "memory" => "SQLite tabanli uzun sureli bellek yonetimi",
        "guardrails" => "Prompt injection ve veri sizintisi korumasi",
        "sandbox" => "Docker icerisinde izole kod calistirma",
        "vgate" => "API proxy katmani (Port: 1071)",
        "gateway" => "HTTP REST API ve Telegram bot",
        "orchestrator" => "Otonom gorev döngusu (Agent Loop)",
        "graph" => "Event graph ve lock-free eszamanlilik",
        "swarm" => "Coklu ajan orkestrasyon sistemi",
        _ => "Bilinmeyen modul",
    };

    let mut output = format!(
        "\n{}\n",
        format!("════════════════════════════════════════════════════════════").cyan()
    );
    output.push_str(&format!("  {} {}\n\n", module.to_uppercase().blue().bold(), description));
    output.push_str(&format!("{}\n", "────────────────────────────────────────────────────────────".cyan()));
    output.push_str(&format!("  {}\n", "Alt Komutlar:".yellow()));

    for cmd in subcommands {
        output.push_str(&format!("    {} {:<12} {}\n", 
            "•".green(), 
            cmd.green(),
            get_subcommand_description(module, cmd)
        ));
    }

    output.push_str(&format!("{}\n\n", "────────────────────────────────────────────────────────────".cyan()));
    output
}

fn get_subcommand_description(module: &str, cmd: &str) -> &'static str {
    match (module, cmd) {
        ("memory", "list") => "Kayitlari listele",
        ("memory", "search") => "Kayitlarda ara",
        ("memory", "store") => "Yeni kayit ekle",
        ("memory", "recall") => "Kayit getir",
        ("memory", "cleanup") => "Sure dolan kayitlari temizle",
        
        ("guardrails", "list") => "Politikalari listele",
        ("guardrails", "toggle") => "Politika ac/kapat",
        ("guardrails", "check") => "Metin kontrol et",
        
        ("sandbox", "run") => "Kod calistir",
        ("sandbox", "exec") => "Komut calistir",
        ("sandbox", "status") => "Konteyner durumu",
        
        ("vgate", "status") => "Proxy durumu",
        ("vgate", "models") => "Mevcut modeller",
        ("vgate", "test") => "Baglanti testi",
        
        ("swarm", "spawn") => "Yeni ajan olustur",
        ("swarm", "task") => "Gorev ata",
        ("swarm", "status") => "Swarm durumu",
        
        _ => ""
    }
}
