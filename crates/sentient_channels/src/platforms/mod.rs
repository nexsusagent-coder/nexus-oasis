//! ─── Platform Implementations ───

pub mod whatsapp;
pub mod signal;
pub mod slack;
pub mod matrix_;
pub mod irc_;

pub use whatsapp::WhatsAppChannel;
pub use signal::SignalChannel;
pub use slack::SlackChannel;
