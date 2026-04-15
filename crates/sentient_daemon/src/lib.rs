//! ═══════════════════════════════════════════════════════════════════════════════
//!  SENTIENT DAEMON - Arka Planda Çalışan Asistan Servisi
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Bu modül, kullanıcının "Hey Luna, müzik aç" gibi sesli komutlarını
//! dinleyen ve otomatik olarak aksiyon alan daemon servisidir.
//!
//! Örnek senaryo:
//! 1. Kullanıcı yatakta yatıyor
//! 2. "Hey Luna, bana rahatlatıcı müzik aç" diyor
//! 3. Wake word algılanıyor
//! 4. STT ile ses metne çevriliyor
//! 5. Command parser ile intent belirleniyor (PlayMusic)
//! 6. Browser açılıyor, YouTube'da arama yapılıyor
//! 7. İlk şarkı tıklanıyor ve çalıyor
//! 8. TTS ile "Şarkıyı açıyorum" yanıtı veriliyor

pub mod error;
pub mod commands;
pub mod actions;
pub mod daemon;

pub use error::{DaemonError, DaemonResult};
pub use commands::{CommandIntent, CommandParser, ParsedCommand};
pub use actions::{ActionResult, VoiceActionExecutor};
pub use daemon::{SentientDaemon, DaemonConfig, DaemonState};
