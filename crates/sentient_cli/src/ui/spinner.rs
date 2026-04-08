//! ─── SPINNER ───
//!
//! Asenkron islem gostergeleri

use std::io::{self, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use colored::Colorize;

/// Asenkron islem gostergesi
pub struct Spinner {
    /// Spinner durumu
    running: Arc<AtomicBool>,
    /// Islem aciklamasi
    message: String,
    /// Spinner handle
    handle: Option<thread::JoinHandle<()>>,
}

impl Spinner {
    pub fn new(message: &str) -> Self {
        Self {
            running: Arc::new(AtomicBool::new(false)),
            message: message.into(),
            handle: None,
        }
    }

    /// Spinner'i baslat
    pub fn start(&mut self) {
        self.running.store(true, Ordering::SeqCst);
        let running = self.running.clone();
        let message = self.message.clone();

        self.handle = Some(thread::spawn(move || {
            let frames = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
            let mut i = 0;

            while running.load(Ordering::SeqCst) {
                let frame = frames[i % frames.len()];
                print!("\r{} {}", frame.cyan(), message.yellow());
                io::stdout().flush().ok();
                thread::sleep(Duration::from_millis(80));
                i += 1;
            }
        }));
    }

    /// Spinner'i durdur
    pub fn stop(&mut self) {
        self.running.store(false, Ordering::SeqCst);
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
        print!("\r{}\r", " ".repeat(self.message.len() + 5));
        io::stdout().flush().ok();
    }

    /// Basariyla tamamlandi
    pub fn success(&mut self, msg: &str) {
        self.stop();
        println!("{} {}", "✓".green().bold(), msg.green());
    }

    /// Hata ile tamamlandi
    pub fn error(&mut self, msg: &str) {
        self.stop();
        println!("{} {}", "✗".red().bold(), msg.red());
    }
}

/// Farkli spinner stilleri
pub enum SpinnerStyle {
    /// Klasik donen cizgi
    Dots,
    /// Yuvarlak animasyon
    Circle,
    /// Atlar tarzı
    Moon,
    /// Progress bar
    Bar,
}

impl SpinnerStyle {
    pub fn frames(&self) -> &[&str] {
        match self {
            Self::Dots => &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"],
            Self::Circle => &["◜", "◠", "◝", "◞", "◡", "◟"],
            Self::Moon => &["🌑", "🌒", "🌓", "🌔", "🌕", "🌖", "🌗", "🌘"],
            Self::Bar => &["[=  ]", "[== ]", "[===]", "[ ==]", "[  =]"],
        }
    }
}

/// Basit spinner yardimci fonksiyonu
pub fn spin_while<F, T>(message: &str, f: F) -> T
where
    F: FnOnce() -> T,
{
    print!("{} ", message.yellow());
    io::stdout().flush().ok();
    let result = f();
    println!("{}", "✓".green());
    result
}

/// Asenkron spinner yardimci fonksiyonu
pub async fn spin_while_async<F, T, Fut>(message: &str, f: F) -> T
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = T>,
{
    let mut spinner = Spinner::new(message);
    spinner.start();
    let result = f().await;
    spinner.success("Tamamlandı");
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spinner_creation() {
        let spinner = Spinner::new("Test");
        assert!(!spinner.running.load(Ordering::SeqCst));
    }
}
