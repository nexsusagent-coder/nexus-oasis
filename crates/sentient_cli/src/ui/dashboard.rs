//! ─── DASHBOARD ───
//!
//! Ana kontrol paneli gosterimi
//! Gemma 4 KERNEL entegrasyonu

use colored::Colorize;

/// Gemma 4 Kernel Status
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct KernelStatus {
    pub model: String,
    pub version: String,
    pub is_active: bool,
    pub api_key_required: bool,
    pub context_length: usize,
    pub supports_vision: bool,
    pub supports_thinking: bool,
}

impl Default for KernelStatus {
    fn default() -> Self {
        Self {
            model: "Gemma 4 31B".to_string(),
            version: "4.0.0".to_string(),
            is_active: true,
            api_key_required: false, // NO API KEY REQUIRED!
            context_length: 262_144,  // 256K
            supports_vision: true,
            supports_thinking: true,
        }
    }
}

/// Sistem durumu paneli
pub struct SystemDashboard {
    /// Kernel status (Gemma 4)
    #[allow(dead_code)]
    kernel: KernelStatus,
    /// Modul durumları
    modules: Vec<ModuleInfo>,
    /// Aktif gorevler
    active_tasks: usize,
    /// Bellek kullanimi
    memory_usage_mb: f64,
    /// API istatistikleri
    api_stats: ApiStats,
    /// Core engines list
    core_engines: Vec<CoreEngine>,
}

/// Core Engine Info
#[derive(Debug, Clone)]
pub struct CoreEngine {
    pub name: String,
    pub engine_type: EngineType,
    pub status: EngineStatus,
    pub description: String,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EngineType {
    Kernel,     // Gemma 4
    Memory,     // Memory Cube
    Reasoning,  // OASIS Brain
    Security,   // Guardrails
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub enum EngineStatus {
    Active,
    Idle,
    Error,
    Disabled,
}

#[derive(Debug, Clone)]
pub struct ModuleInfo {
    pub name: String,
    pub status: ModuleStatus,
    #[allow(dead_code)]
    pub uptime_secs: u64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ModuleStatus {
    Running,
    Stopped,
    Error,
    Idle,
}

#[derive(Debug, Clone, Default)]
pub struct ApiStats {
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub avg_latency_ms: u64,
    pub total_tokens: u64,
}

impl SystemDashboard {
    pub fn new() -> Self {
        let mut dashboard = Self {
            kernel: KernelStatus::default(),
            modules: Vec::new(),
            active_tasks: 0,
            memory_usage_mb: 0.0,
            api_stats: ApiStats::default(),
            core_engines: vec![
                // ═══════════════════════════════════════════════════════════════
                // GEMMA 4 - SENTIENT OS KERNEL (NO API KEY REQUIRED!)
                // ═══════════════════════════════════════════════════════════════
                CoreEngine {
                    name: "GEMMA 4 KERNEL".to_string(),
                    engine_type: EngineType::Kernel,
                    status: EngineStatus::Active,
                    description: "Native Intelligence | 31B | 256K ctx | NO API KEY".to_string(),
                },
                CoreEngine {
                    name: "MEMORY CUBE".to_string(),
                    engine_type: EngineType::Memory,
                    status: EngineStatus::Active,
                    description: "L3 Cognitive Memory | Zero-Copy | SQLite".to_string(),
                },
                CoreEngine {
                    name: "OASIS BRAIN".to_string(),
                    engine_type: EngineType::Reasoning,
                    status: EngineStatus::Active,
                    description: "Autonomous Thinking | Gemma 4 Fixed | Cognitive Loop".to_string(),
                },
                CoreEngine {
                    name: "GUARDRAILS".to_string(),
                    engine_type: EngineType::Security,
                    status: EngineStatus::Active,
                    description: "Security Layer | Prompt Injection Protection".to_string(),
                },
            ],
        };
        
        // Add default modules
        dashboard.add_module("gemma4_kernel", ModuleStatus::Running);
        dashboard.add_module("memory_cube", ModuleStatus::Running);
        dashboard.add_module("oasis_brain", ModuleStatus::Running);
        dashboard.add_module("guardrails", ModuleStatus::Running);
        dashboard.add_module("sandbox", ModuleStatus::Idle);
        dashboard.add_module("vgate", ModuleStatus::Idle);
        
        dashboard
    }

    /// Modul ekle
    pub fn add_module(&mut self, name: &str, status: ModuleStatus) {
        self.modules.push(ModuleInfo {
            name: name.into(),
            status,
            uptime_secs: 0,
        });
    }

    /// Modul durumunu guncelle
    pub fn update_module(&mut self, name: &str, status: ModuleStatus) {
        if let Some(module) = self.modules.iter_mut().find(|m| m.name == name) {
            module.status = status;
        }
    }

    /// Aktif gorevleri ayarla
    pub fn set_active_tasks(&mut self, count: usize) {
        self.active_tasks = count;
    }

    /// Bellek kullanimini ayarla
    pub fn set_memory_usage(&mut self, mb: f64) {
        self.memory_usage_mb = mb;
    }

    /// API istatistiklerini ayarla
    pub fn set_api_stats(&mut self, stats: ApiStats) {
        self.api_stats = stats;
    }

    /// Tam panelden render et
    pub fn render_full(&self) -> String {
        let mut output = String::new();

        // Header
        output.push_str(&self.render_header());
        
        // CORE ENGINES (Gemma 4 KERNEL)
        output.push_str(&self.render_core_engines());
        
        // Moduller
        output.push_str(&self.render_modules());
        
        // Istatistikler
        output.push_str(&self.render_stats());
        
        // Footer
        output.push_str(&self.render_footer());

        output
    }

    fn render_header(&self) -> String {
        format!(
            "\n{}\n  {}\n{}\n",
            "╔══════════════════════════════════════════════════════════════╗".cyan(),
            "🐺 SENTIENT SYSTEM DASHBOARD".cyan().bold(),
            "╚══════════════════════════════════════════════════════════════╝".cyan()
        )
    }

    /// Render Core Engines panel with GEMMA 4 KERNEL
    fn render_core_engines(&self) -> String {
        let mut output = String::new();
        
        output.push_str(&format!("  {}\n", "══════════════════════════════════════════════".bright_magenta()));
        output.push_str(&format!("  {}\n", "CORE ENGINES".bright_magenta().bold()));
        output.push_str(&format!("  {}\n", "══════════════════════════════════════════════".bright_magenta()));
        
        for engine in &self.core_engines {
            let (icon, status_str) = match engine.status {
                EngineStatus::Active => ("✅".green(), "ACTIVE".green()),
                EngineStatus::Idle => ("⚪".bright_black(), "IDLE".bright_black()),
                EngineStatus::Error => ("❌".red(), "ERROR".red()),
                EngineStatus::Disabled => ("🚫".red(), "DISABLED".red()),
            };
            
            // Special rendering for KERNEL (Gemma 4)
            if engine.engine_type == EngineType::Kernel {
                output.push_str(&format!(
                    "  {} {} {} {}\n",
                    icon,
                    engine.name.bright_magenta().bold(),
                    "[KERNEL]".bright_yellow().bold(),
                    status_str
                ));
                output.push_str(&format!("      {}\n", engine.description.bright_white()));
                output.push_str(&format!("      {}\n", "→ NO API KEY REQUIRED | FULLY LOCAL | 256K CONTEXT".bright_green().bold()));
            } else {
                output.push_str(&format!(
                    "  {} {} [{}]\n",
                    icon,
                    engine.name.blue(),
                    status_str
                ));
                output.push_str(&format!("      {}\n", engine.description.dimmed()));
            }
        }
        
        output.push_str("\n");
        output
    }

    fn render_modules(&self) -> String {
        let mut output = format!("  {}\n", "MODULE STATUS".yellow());
        output.push_str(&format!("  {}\n", "──────────────────────────────────────────────".cyan()));

        for module in &self.modules {
            let status_icon = match module.status {
                ModuleStatus::Running => "🟢".green(),
                ModuleStatus::Stopped => "🔴".red(),
                ModuleStatus::Error => "🔴".red(),
                ModuleStatus::Idle => "⚪".bright_black(),
            };

            let status_text = match module.status {
                ModuleStatus::Running => "RUNNING".green(),
                ModuleStatus::Stopped => "STOPPED".red(),
                ModuleStatus::Error => "ERROR".red(),
                ModuleStatus::Idle => "IDLE".bright_black(),
            };

            output.push_str(&format!(
                "  {} {:<12} [{}]\n",
                status_icon,
                module.name.blue(),
                status_text
            ));
        }

        output.push_str("\n");
        output
    }

    fn render_stats(&self) -> String {
        let success_rate = if self.api_stats.total_requests > 0 {
            (self.api_stats.successful_requests as f64 / self.api_stats.total_requests as f64 * 100.0) as u64
        } else {
            100
        };

        format!(
            "  {}\n  {}\n  {:<20} {}\n  {:<20} {}\n  {:<20} {}\n  {:<20} {} ms\n  {:<20} {}\n  {:<20} {:.1} MB\n{}\n",
            "SYSTEM METRICS".yellow(),
            "──────────────────────────────────────────────".cyan(),
            "Active Tasks".blue(),
            format!("{}", self.active_tasks).green(),
            "API Requests".blue(),
            format!("{} ({}% success)", self.api_stats.total_requests, success_rate).green(),
            "Failed Requests".blue(),
            format!("{}", self.api_stats.failed_requests).red(),
            "Avg Latency".blue(),
            self.api_stats.avg_latency_ms,
            "Total Tokens".blue(),
            format!("{}", self.api_stats.total_tokens).green(),
            "Memory Usage".blue(),
            self.memory_usage_mb,
            "──────────────────────────────────────────────".cyan()
        )
    }

    fn render_footer(&self) -> String {
        let now = chrono::Local::now();
        format!(
            "  {} {}\n",
            "Last updated:".dimmed(),
            now.format("%H:%M:%S").to_string().dimmed()
        )
    }

    /// Kompakt gorunum
    pub fn render_compact(&self) -> String {
        let running = self.modules.iter().filter(|m| m.status == ModuleStatus::Running).count();
        let total = self.modules.len();
        
        format!(
            "Dashboard: {}/{} modules active | {} tasks | {:.1}MB | {}% API success",
            running,
            total,
            self.active_tasks,
            self.memory_usage_mb,
            if self.api_stats.total_requests > 0 {
                self.api_stats.successful_requests * 100 / self.api_stats.total_requests.max(1)
            } else {
                100
            }
        )
    }
}

impl Default for SystemDashboard {
    fn default() -> Self {
        Self::new()
    }
}
