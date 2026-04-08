//! ─── MODULE UI ───
//!
//! Modul durumu UI bileşenleri

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
                ModuleStatus::Running => "Çalışıyor".green(),
                ModuleStatus::Stopped => "Durdu".red(),
                ModuleStatus::Error => "Hata".red(),
                ModuleStatus::Initializing => "Başlatılıyor...".yellow(),
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
