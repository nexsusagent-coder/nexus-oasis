//! ─── UI MODULU ───
//!
//! Terminal kullanici arayuzu bileşenleri

mod dashboard;
mod spinner;
mod progress;
mod table;
pub mod theme;
mod module;

// Explicitly import from dashboard to avoid ambiguous glob imports
// ModuleStatus is defined in both dashboard and module - use dashboard's version
pub use dashboard::{SystemDashboard, ModuleStatus};
pub use module::ModuleManager;
pub use spinner::*;
pub use progress::*;
pub use table::*;
pub use theme::*;
