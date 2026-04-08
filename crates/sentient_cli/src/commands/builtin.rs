//! ─── BUILTIN KOMUTLAR ───
//!
//! Temel sistem komutlari

use colored::Colorize;

/// Builtin komutlarin listesi
pub const BUILTIN_COMMANDS: &[(&str, &str, &str)] = &[
    ("help", "h, ?", "Yardim menusunu goster"),
    ("status", "s", "Sistem durumunu goster"),
    ("version", "v", "Surum bilgisini goster"),
    ("clear", "cls", "Ekrani temizle"),
    ("history", "", "Komut gecmisini goster"),
    ("exit", "q, quit", "Cikis"),
];

/// Modul komutlari
pub const MODULE_COMMANDS: &[(&str, &str, &str)] = &[
    ("memory", "mem", "Bellek islemleri"),
    ("guardrails", "guard", "Guvenlik politikalari"),
    ("sandbox", "sbx", "Docker sandbox"),
    ("vgate", "vg", "V-GATE proxy"),
    ("gateway", "gw", "API Gateway"),
    ("orchestrator", "orch", "Gorev orkestrasyonu"),
    ("graph", "", "Event graph"),
];

/// Swarm komutlari
pub const SWARM_COMMANDS: &[(&str, &str, &str)] = &[
    ("swarm", "sw", "Coklu ajan sistemleri"),
    ("agent", "ag", "Tek ajan yonetimi"),
    ("task", "ts", "Gorev yonetimi"),
    ("blackboard", "bb", "Ortak bilgi alani"),
    ("collective", "coll", "Toplu bellek"),
];

/// Debug komutlari
pub const DEBUG_COMMANDS: &[(&str, &str, &str)] = &[
    ("debug", "dbg", "Debug modunu ac/kapat"),
    ("logs", "log", "Sistem loglari"),
    ("metrics", "mt", "Performans metrikleri"),
    ("export", "exp", "Veri disa aktarimi"),
    ("import", "imp", "Veri ice aktarimi"),
    ("reset", "", "Sistemi sifirla"),
];

/// Ajent tipleri
pub const AGENT_TYPES: &[(&str, &str)] = &[
    ("coordinator", "Ana koordinatör - gorev dagilimi"),
    ("researcher", "Arastirmaci - web arastirmasi"),
    ("coder", "Kod yazici - uygulama"),
    ("critic", "Elestirmen - kalite kontrol"),
    ("planner", "Planlayici - strateji"),
    ("executor", "Yurutucu - gorev calistirma"),
    ("websurfer", "Web gezgini - tarayici"),
    ("memorykeeper", "Bellek bekçisi - bilgi yonetimi"),
];

/// Hizli baslangic yardimi
pub fn print_quick_help() -> String {
    format!(
        r#"
{}
  {} - Sistem durumu
  {} - Modul komutlari  
  {} - Swarm/ajan komutlari
  {} - Debug/log komutlari
  {} veya {} - Cikis

{}
"#,
        "🚀 HIZLI BASLANGIC".cyan().bold(),
        "status".green(),
        "memory, guardrails, sandbox...".green(),
        "swarm, agent, task...".green(),
        "debug, logs, metrics...".green(),
        "exit".yellow(),
        "q".yellow(),
        "Detayli yardim icin: help".dimmed()
    )
}

/// Surum bilgisi
pub fn print_version() -> String {
    format!(
        r#"
{}
  Surum      : {}
  Rust       : 1.70+
  Durum      : Gelistirme
  Lisans     : MIT
{}
"#,
        "═══════════════════════════════════".cyan(),
        env!("CARGO_PKG_VERSION").green(),
        "═══════════════════════════════════".cyan(),
    )
}

/// Motd (Message of the Day)
pub fn print_motd() -> String {
    let tips = [
        "Tip: 'help <modul>' ile detayli yardim alabilirsiniz",
        "Tip: Tab tusu ile komut tamamama kullanabilirsiniz",
        "Tip: Yukari/asaai ok tuslari ile gecmise erisebilirsiniz",
        "Tip: 'debug on' ile detayli log gorebilirsiniz",
        "Swarm: 'swarm spawn researcher' ile arastirmaci ajan olusturun",
        "Hizli gorev: 'task add <gorev>' ile hemen gorev ekleyin",
    ];

    let tip = &tips[chrono::Utc::now().timestamp_subsec_nanos() as usize % tips.len()];
    
    format!(
        "\n{}\n",
        tip.bright_black().italic()
    )
}
