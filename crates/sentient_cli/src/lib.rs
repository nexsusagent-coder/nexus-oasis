//! ─── SENTIENT CLI LIB ───
//!
//! Komut Satiri Arayuzu Kutuphanesi

pub mod repl;
pub mod commands;
pub mod ui;

// Yeniden ihracat
pub use repl::*;
pub use commands::*;
pub use ui::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_placeholder() {
        assert!(true);
    }
}
