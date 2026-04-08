//! ─── SENTIENT ÇEKİRDEK (MAIN) ───
//!
//! NEXUS OASIS — Yapay Zeka İşletim Sistemi
//! Tüm modülleri orkestre eden ana giriş noktası.

use sentient_common::error::SENTIENTResult;
use sentient_core::SENTIENTSystem;

/// ─── ANA GİRİŞ NOKTASI ───

#[tokio::main]
async fn main() -> SENTIENTResult<()> {
    // Loglama
    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or("info"),
    )
    .format_module_path(false)
    .format_timestamp_secs()
    .init();

    println!("");
    println!("  ╔══════════════════════════════════════════════╗");
    println!("  ║  🌟  SENTIENT (NEXUS OASIS)                     ║");
    println!("  ║  Yapay Zeka İşletim Sistemi — v{}            ║", env!("CARGO_PKG_VERSION"));
    println!("  ╚══════════════════════════════════════════════╝");
    println!("");

    // Sistemi başlat
    let system = SENTIENTSystem::init().await?;

    // Durum raporu
    println!("{}", system.status().await);

    // Sistem çalışır durumda kalır — Ctrl+C ile kapatılır
    log::info!("📡  SENTIENT: Sistem çalışıyor. Durdurmak için Ctrl+C'ye basın.");

    // Graceful shutdown
    tokio::signal::ctrl_c()
        .await
        .expect("Ctrl+C sinyali dinlenemedi");

    system.shutdown().await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_placeholder() {
        assert!(true);
    }
}
