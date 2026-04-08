//! ═════════════════════════════════════════════════════════════════
//!  LIB MODULE
//! ═════════════════════════════════════════════════════════════════

pub mod error;
pub mod events;
pub mod translate;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_placeholder() {
        assert!(true);
    }
}
