//! ═════════════════════════════════════════════════════════════════
//!  LIB MODULE
//! ═════════════════════════════════════════════════════════════════

pub mod circuit_breaker;
pub mod crypto;
pub mod error;
pub mod events;
pub mod metrics;
pub mod serialization;
pub mod tracing;
pub mod translate;

/// Traits re-export (sentient_core ile uyumluluk)
pub mod traits_compat {
    pub use crate::error::SENTIENTError;
    pub use crate::events::{SENTIENTEvent, EventType};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_placeholder() {
        assert!(true);
    }
}
