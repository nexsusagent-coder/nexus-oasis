//! ─── COMPLETION MODULU ───
//!
//! Tab-tamamlama sistemi

use std::collections::HashSet;

/// Tum mevcut komutlar ve oneriler
pub struct CompletionEngine {
    /// Temel komutlar
    base_commands: HashSet<String>,
    /// Modul komutlari
    module_commands: HashSet<String>,
    /// Swarm komutlari
    swarm_commands: HashSet<String>,
    /// Admin komutlari
    admin_commands: HashSet<String>,
    /// Modul isimleri
    modules: HashSet<String>,
    /// Ajent isimleri
    agents: HashSet<String>,
}

impl CompletionEngine {
    pub fn new() -> Self {
        let mut base_commands = HashSet::new();
        base_commands.extend(vec![
            "help".into(), "h".into(), "?".into(),
            "exit".into(), "quit".into(), "q".into(),
            "status".into(), "s".into(),
            "clear".into(), "cls".into(),
            "history".into(),
            "version".into(), "v".into(),
            "config".into(),
            "reload".into(),
        ]);

        let mut module_commands = HashSet::new();
        module_commands.extend(vec![
            "memory".into(),
            "guardrails".into(),
            "sandbox".into(),
            "vgate".into(),
            "gateway".into(),
            "orchestrator".into(),
            "graph".into(),
        ]);

        let mut swarm_commands = HashSet::new();
        swarm_commands.extend(vec![
            "swarm".into(),
            "agent".into(),
            "task".into(),
            "blackboard".into(),
            "collective".into(),
            "spawn".into(),
            "kill".into(),
        ]);

        let mut admin_commands = HashSet::new();
        admin_commands.extend(vec![
            "debug".into(),
            "logs".into(),
            "metrics".into(),
            "export".into(),
            "import".into(),
            "reset".into(),
            "shutdown".into(),
        ]);

        let mut modules = HashSet::new();
        modules.extend(vec![
            "memory".into(),
            "guardrails".into(),
            "sandbox".into(),
            "vgate".into(),
            "gateway".into(),
            "orchestrator".into(),
            "graph".into(),
            "swarm".into(),
        ]);

        let mut agents = HashSet::new();
        agents.extend(vec![
            "coordinator".into(),
            "researcher".into(),
            "coder".into(),
            "critic".into(),
            "planner".into(),
            "executor".into(),
            "websurfer".into(),
            "memorykeeper".into(),
        ]);

        Self {
            base_commands,
            module_commands,
            swarm_commands,
            admin_commands,
            modules,
            agents,
        }
    }

    /// Tamamlama onerileri uret
    pub fn complete(&self, input: &str) -> Vec<String> {
        if input.is_empty() {
            return self.all_commands();
        }

        let input_lower = input.to_lowercase();
        let parts: Vec<&str> = input.split_whitespace().collect();

        // Ilk kelime tamamlama
        if parts.len() == 1 {
            return self.complete_command(&input_lower);
        }

        // Ikinci seviye tamamlama (modul alt komutlari)
        if parts.len() == 2 {
            return self.complete_subcommand(parts[0], parts[1]);
        }

        // Ucuncu seviye tamamlama
        if parts.len() >= 3 {
            return self.complete_args(parts[0], parts[1], &input_lower);
        }

        Vec::new()
    }

    fn all_commands(&self) -> Vec<String> {
        let mut commands: Vec<String> = Vec::new();
        commands.extend(self.base_commands.iter().cloned());
        commands.extend(self.module_commands.iter().cloned());
        commands.extend(self.swarm_commands.iter().cloned());
        commands.extend(self.admin_commands.iter().cloned());
        commands.sort();
        commands
    }

    fn complete_command(&self, input: &str) -> Vec<String> {
        self.all_commands()
            .into_iter()
            .filter(|cmd| cmd.starts_with(input))
            .collect()
    }

    fn complete_subcommand(&self, command: &str, partial: &str) -> Vec<String> {
        let subcommands = match command {
            "memory" => vec!["list", "search", "store", "recall", "cleanup", "export"],
            "guardrails" => vec!["list", "toggle", "check", "test", "report"],
            "sandbox" => vec!["run", "exec", "status", "logs", "kill", "clean"],
            "vgate" => vec!["status", "models", "test", "config", "logs"],
            "gateway" => vec!["start", "stop", "status", "config", "metrics"],
            "orchestrator" => vec!["run", "status", "plan", "cancel", "history"],
            "graph" => vec!["show", "add", "remove", "query", "export"],
            "swarm" => vec!["start", "stop", "status", "spawn", "task", "report"],
            "agent" => vec!["spawn", "list", "status", "kill", "task", "logs"],
            "task" => vec!["add", "list", "status", "cancel", "result"],
            "logs" => vec!["show", "tail", "clear", "export", "level"],
            "metrics" => vec!["show", "export", "reset"],
            "debug" => vec!["on", "off", "level", "profile"],
            _ => return Vec::new(),
        };

        subcommands
            .into_iter()
            .filter(|s| s.starts_with(partial))
            .map(|s| format!("{} {}", command, s))
            .collect()
    }

    fn complete_args(&self, cmd: &str, subcmd: &str, _input: &str) -> Vec<String> {
        // Modul tamamlama
        if subcmd == "module" || subcmd == "enter" {
            return self.modules
                .iter()
                .map(|m| format!("{} {} {}", cmd, subcmd, m))
                .collect();
        }

        // Ajent tamamlama
        if cmd == "agent" && subcmd == "spawn" {
            return self.agents
                .iter()
                .map(|a| format!("agent spawn {}", a))
                .collect();
        }

        if cmd == "swarm" && subcmd == "spawn" {
            return self.agents
                .iter()
                .map(|a| format!("swarm spawn {}", a))
                .collect();
        }

        Vec::new()
    }

    /// Komut aciklamasi
    pub fn describe(&self, command: &str) -> Option<&'static str> {
        let descriptions = [
            ("help", "Yardim menusunu goster"),
            ("status", "Sistem durumunu goster"),
            ("memory", "Bellek modulu komutlari"),
            ("guardrails", "Guvenlik politika komutlari"),
            ("sandbox", "Sandbox kod calistirma"),
            ("vgate", "V-GATE proxy komutlari"),
            ("swarm", "Coklu ajan sistem komutlari"),
            ("agent", "Tek ajan yonetim komutlari"),
            ("task", "Gorev yonetim komutlari"),
            ("logs", "Sistem loglarini izle"),
            ("debug", "Hata ayiklama modu"),
        ];

        descriptions
            .iter()
            .find(|(cmd, _)| cmd == &command)
            .map(|(_, desc)| *desc)
    }
}

impl Default for CompletionEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Rustyline icin tamamlama
pub struct SENTIENTCompleter {
    engine: CompletionEngine,
}

impl SENTIENTCompleter {
    pub fn new() -> Self {
        Self {
            engine: CompletionEngine::new(),
        }
    }
}

impl rustyline::completion::Completer for SENTIENTCompleter {
    type Candidate = String;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &rustyline::Context<'_>,
    ) -> rustyline::Result<(usize, Vec<String>)> {
        let input = &line[..pos];
        let completions = self.engine.complete(input);
        Ok((pos - input.len(), completions))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_completion() {
        let engine = CompletionEngine::new();
        let completions = engine.complete("sta");
        assert!(completions.contains(&"status".to_string()));
    }

    #[test]
    fn test_subcommand_completion() {
        let engine = CompletionEngine::new();
        let completions = engine.complete("memory l");
        assert!(completions.iter().any(|c| c.contains("list")));
    }

    #[test]
    fn test_empty_completion() {
        let engine = CompletionEngine::new();
        let completions = engine.complete("");
        assert!(!completions.is_empty());
    }
}

