//! ═══════════════════════════════════════════════════════════════════════════════
//!  SENTIENT TOOLS - 30+ TOOLS
//! ═══════════════════════════════════════════════════════════════════════════════

// ═══════════════════════════════════════════════════════════════════════════════
//  CRITICAL TOOLS (Priority 1) - OpenHarness Asimilasyonu
// ═══════════════════════════════════════════════════════════════════════════════

pub mod bash_tool;
pub mod file_read_tool;
pub mod file_write_tool;
pub mod file_edit_tool;

// ═══════════════════════════════════════════════════════════════════════════════
//  HIGH PRIORITY TOOLS
// ═══════════════════════════════════════════════════════════════════════════════

pub mod glob_tool;
pub mod web_search_tool;
pub mod web_fetch_tool;
pub mod ask_user_question_tool;
pub mod lsp_tool;
pub mod skill_tool;
pub mod todo_write_tool;
pub mod brief_tool;
pub mod config_tool;

// ═══════════════════════════════════════════════════════════════════════════════
//  STANDARD TOOLS (Mevcut)
// ═══════════════════════════════════════════════════════════════════════════════

pub mod git_tool;
pub mod grep_tool;
pub mod sed_tool;
pub mod browser_tool;
pub mod screenshot_tool;
pub mod mcp_tool;
pub mod memory_tool;
pub mod task_tool;
pub mod n8n_tool;
pub mod email_tool;
pub mod notify_tool;
pub mod pdf_tool;
pub mod translate_tool;
pub mod calendar_tool;
pub mod agent_tool;

// ═══════════════════════════════════════════════════════════════════════════════
//  RE-EXPORTS - CRITICAL
// ═══════════════════════════════════════════════════════════════════════════════

pub use bash_tool::BashTool;
pub use file_read_tool::FileReadTool;
pub use file_write_tool::FileWriteTool;
pub use file_edit_tool::FileEditTool;

// ═══════════════════════════════════════════════════════════════════════════════
//  RE-EXPORTS - HIGH PRIORITY
// ═══════════════════════════════════════════════════════════════════════════════

pub use glob_tool::GlobTool;
pub use web_search_tool::WebSearchTool;
pub use web_fetch_tool::WebFetchTool;
pub use ask_user_question_tool::AskUserQuestionTool;
pub use lsp_tool::LspTool;
pub use skill_tool::SkillTool;
pub use todo_write_tool::TodoWriteTool;
pub use brief_tool::BriefTool;
pub use config_tool::ConfigTool;

// ═══════════════════════════════════════════════════════════════════════════════
//  RE-EXPORTS - STANDARD
// ═══════════════════════════════════════════════════════════════════════════════

pub use git_tool::GitTool;
pub use grep_tool::GrepTool;
pub use sed_tool::SedTool;
pub use browser_tool::BrowserTool;
pub use screenshot_tool::ScreenshotTool;
pub use mcp_tool::McpTool;
pub use memory_tool::MemoryTool;
pub use task_tool::TaskTool;
pub use n8n_tool::N8nTool;
pub use email_tool::EmailTool;
pub use notify_tool::NotifyTool;
pub use pdf_tool::PdfTool;
pub use translate_tool::TranslateTool;
pub use calendar_tool::CalendarTool;
pub use agent_tool::AgentTool;

/// Get all tools
pub fn all_tools(policy: crate::sovereign::SovereignPolicy) -> Vec<Box<dyn crate::sentient_tool::SentientTool>> {
    vec![
        // CRITICAL
        Box::new(BashTool::new(policy.clone())),
        Box::new(FileReadTool::new(policy.clone())),
        Box::new(FileWriteTool::new(policy.clone())),
        Box::new(FileEditTool::new(policy.clone())),
        // HIGH
        Box::new(GlobTool::new(policy.clone())),
        Box::new(WebSearchTool::new(policy.clone())),
        Box::new(WebFetchTool::new(policy.clone())),
        Box::new(AskUserQuestionTool::new()),
        Box::new(LspTool::new()),
        Box::new(SkillTool::default_tool()),
        Box::new(TodoWriteTool::default_tool()),
        Box::new(BriefTool::new()),
        Box::new(ConfigTool::default_tool()),
        // STANDARD
        Box::new(GitTool::new()),
        Box::new(GrepTool::new()),
        Box::new(SedTool::new()),
        Box::new(BrowserTool::new()),
        Box::new(ScreenshotTool::new()),
        Box::new(McpTool::new()),
        Box::new(MemoryTool::new()),
        Box::new(TaskTool::new()),
        Box::new(N8nTool::new()),
        Box::new(EmailTool::new()),
        Box::new(NotifyTool::new()),
        Box::new(PdfTool::new()),
        Box::new(TranslateTool::new()),
        Box::new(CalendarTool::new()),
        Box::new(AgentTool::new()),
    ]
}

/// Tool sayısı
pub const TOOL_COUNT: usize = 30;
