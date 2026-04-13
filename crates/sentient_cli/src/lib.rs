//! ─── SENTIENT CLI LIB ───
//!
//! Komut Satiri Arayuzu Kutuphanesi

// Suppress warnings for unused code in development
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

pub mod repl;
pub mod commands;
pub mod ui;

// Re-export from repl module
pub use repl::{
    PromptState, ReplMode, success_prompt, error_prompt, warning_prompt, info_prompt, loading_prompt,
    CommandResult, CommandHandler,
    CommandHistory, HistoryEntry,
    CompletionEngine, SENTIENTCompleter,
    ReplSession, SessionMode, SessionStats, SessionReport, UserPreferences,
};

// Re-export from commands module (excluding ModuleStatus and ModuleManager to avoid conflict)
pub use commands::{
    CommandRegistry, CommandDef, CommandCategory, CommandParam,
    ParsedCommand, CommandParser,
    print_quick_help, print_version, print_motd,
    get_module_subcommands, get_module_help,
};

// Re-export from ui module (use this as the source of truth for ModuleStatus/ModuleManager)
pub use ui::{
    SystemDashboard, ModuleStatus, ModuleManager,
    Spinner, ProgressBar, Table,
};
pub use ui::theme;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_placeholder() {
        assert!(true);
    }
}
