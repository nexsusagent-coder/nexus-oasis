//! ─── KOMUT REGISTRY ───
//!
//! Komut kayit ve yonetim sistemi

use std::collections::HashMap;

/// Komut kayit sistemi
pub struct CommandRegistry {
    /// Kayitli komutlar
    commands: HashMap<String, CommandDef>,
    /// Kategoriler
    categories: HashMap<String, Vec<String>>,
}

/// Komut tanimi
#[derive(Debug, Clone)]
pub struct CommandDef {
    /// Komut adi
    pub name: String,
    /// Kisaltmalar
    pub aliases: Vec<String>,
    /// Aciklama
    pub description: String,
    /// Kategori
    pub category: CommandCategory,
    /// Parametreler
    pub params: Vec<CommandParam>,
    /// Ornek kullanimlar
    pub examples: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CommandCategory {
    /// Temel sistem komutlari
    System,
    /// Bellek islemleri
    Memory,
    /// Guvenlik/Guardrails
    Security,
    /// Swarm/Coklu ajan
    Swarm,
    /// Sandbox/Kod calistirma
    Sandbox,
    /// Ag/Proxy
    Network,
    /// Debug/Log
    Debug,
    /// Admin/Yonetim
    Admin,
}

/// Komut parametresi
#[derive(Debug, Clone)]
pub struct CommandParam {
    /// Parametre adi
    pub name: String,
    /// Opsiyonel mi
    pub optional: bool,
    /// Aciklama
    pub description: String,
    /// Varsayilan deger
    pub default: Option<String>,
}

impl CommandRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            commands: HashMap::new(),
            categories: HashMap::new(),
        };
        
        // Temel komutlari kaydet
        registry.register_builtins();
        registry
    }

    /// Komut kaydet
    pub fn register(&mut self, def: CommandDef) {
        let name = def.name.clone();
        let category = def.category;
        
        // Asil adi kaydet
        self.commands.insert(name.clone(), def.clone());
        
        // Kisaltmalari kaydet
        for alias in &def.aliases {
            self.commands.insert(alias.clone(), def.clone());
        }
        
        // Kategoriye ekle
        self.categories
            .entry(format!("{:?}", category))
            .or_default()
            .push(name);
    }

    /// Komut ara
    pub fn get(&self, name: &str) -> Option<&CommandDef> {
        self.commands.get(name)
    }

    /// Kategorideki komutlari getir
    pub fn get_by_category(&self, category: CommandCategory) -> Vec<&CommandDef> {
        let key = format!("{:?}", category);
        self.categories
            .get(&key)
            .map(|names| {
                names
                    .iter()
                    .filter_map(|n| self.commands.get(n))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Tum komutlari getir
    pub fn all(&self) -> Vec<&CommandDef> {
        self.commands.values().collect()
    }

    /// Komut sayisi
    pub fn len(&self) -> usize {
        self.commands.len()
    }

    pub fn is_empty(&self) -> bool {
        self.commands.is_empty()
    }

    /// Temel komutlari kaydet
    fn register_builtins(&mut self) {
        // Sistem komutlari
        self.register(CommandDef {
            name: "help".into(),
            aliases: vec!["h".into(), "?".into()],
            description: "Yardim menusunu goster".into(),
            category: CommandCategory::System,
            params: vec![],
            examples: vec!["help".into(), "help memory".into()],
        });

        self.register(CommandDef {
            name: "status".into(),
            aliases: vec!["s".into()],
            description: "Sistem durumunu goster".into(),
            category: CommandCategory::System,
            params: vec![],
            examples: vec!["status".into()],
        });

        self.register(CommandDef {
            name: "version".into(),
            aliases: vec!["v".into()],
            description: "Surum bilgisi".into(),
            category: CommandCategory::System,
            params: vec![],
            examples: vec!["version".into()],
        });

        self.register(CommandDef {
            name: "exit".into(),
            aliases: vec!["quit".into(), "q".into()],
            description: "CLI'dan cik".into(),
            category: CommandCategory::System,
            params: vec![],
            examples: vec!["exit".into()],
        });

        // Memory komutlari
        self.register(CommandDef {
            name: "memory".into(),
            aliases: vec!["mem".into()],
            description: "Bellek islemleri".into(),
            category: CommandCategory::Memory,
            params: vec![
                CommandParam {
                    name: "alt_komut".into(),
                    optional: false,
                    description: "list|search|store|cleanup".into(),
                    default: None,
                },
            ],
            examples: vec!["memory list".into(), "memory search AI".into()],
        });

        // Swarm komutlari
        self.register(CommandDef {
            name: "swarm".into(),
            aliases: vec!["sw".into()],
            description: "Coklu ajan orkestrasyonu".into(),
            category: CommandCategory::Swarm,
            params: vec![
                CommandParam {
                    name: "alt_komut".into(),
                    optional: false,
                    description: "start|stop|status|spawn|task".into(),
                    default: None,
                },
            ],
            examples: vec!["swarm start".into(), "swarm spawn researcher".into()],
        });

        self.register(CommandDef {
            name: "agent".into(),
            aliases: vec!["ag".into()],
            description: "Tek ajan yonetimi".into(),
            category: CommandCategory::Swarm,
            params: vec![
                CommandParam {
                    name: "alt_komut".into(),
                    optional: false,
                    description: "spawn|list|status|kill|task".into(),
                    default: None,
                },
            ],
            examples: vec!["agent spawn coder".into(), "agent list".into()],
        });

        self.register(CommandDef {
            name: "task".into(),
            aliases: vec!["ts".into()],
            description: "Gorev yonetimi".into(),
            category: CommandCategory::Swarm,
            params: vec![
                CommandParam {
                    name: "alt_komut".into(),
                    optional: false,
                    description: "add|list|status|cancel|result".into(),
                    default: None,
                },
            ],
            examples: vec!["task add Python script yaz".into()],
        });

        // Sandbox komutlari
        self.register(CommandDef {
            name: "sandbox".into(),
            aliases: vec!["sbx".into()],
            description: "Docker sandbox".into(),
            category: CommandCategory::Sandbox,
            params: vec![
                CommandParam {
                    name: "alt_komut".into(),
                    optional: false,
                    description: "run|status|logs|kill".into(),
                    default: None,
                },
            ],
            examples: vec!["sandbox run python -c 'print(1+1)'".into()],
        });

        // Network komutlari
        self.register(CommandDef {
            name: "vgate".into(),
            aliases: vec!["vg".into()],
            description: "V-GATE proxy".into(),
            category: CommandCategory::Network,
            params: vec![
                CommandParam {
                    name: "alt_komut".into(),
                    optional: false,
                    description: "status|models|test|config".into(),
                    default: None,
                },
            ],
            examples: vec!["vgate status".into(), "vgate models".into()],
        });

        self.register(CommandDef {
            name: "gateway".into(),
            aliases: vec!["gw".into()],
            description: "API Gateway".into(),
            category: CommandCategory::Network,
            params: vec![
                CommandParam {
                    name: "alt_komut".into(),
                    optional: false,
                    description: "start|stop|status|config".into(),
                    default: None,
                },
            ],
            examples: vec!["gateway start".into()],
        });

        // Debug komutlari
        self.register(CommandDef {
            name: "logs".into(),
            aliases: vec!["log".into()],
            description: "Sistem loglari".into(),
            category: CommandCategory::Debug,
            params: vec![
                CommandParam {
                    name: "alt_komut".into(),
                    optional: true,
                    description: "tail|clear|export".into(),
                    default: Some("show".into()),
                },
            ],
            examples: vec!["logs tail".into()],
        });

        self.register(CommandDef {
            name: "metrics".into(),
            aliases: vec!["mt".into()],
            description: "Performans metrikleri".into(),
            category: CommandCategory::Debug,
            params: vec![],
            examples: vec!["metrics".into()],
        });

        self.register(CommandDef {
            name: "debug".into(),
            aliases: vec!["dbg".into()],
            description: "Debug modu".into(),
            category: CommandCategory::Debug,
            params: vec![
                CommandParam {
                    name: "durum".into(),
                    optional: true,
                    description: "on|off".into(),
                    default: None,
                },
            ],
            examples: vec!["debug on".into()],
        });
    }
}

impl Default for CommandRegistry {
    fn default() -> Self {
        Self::new()
    }
}
