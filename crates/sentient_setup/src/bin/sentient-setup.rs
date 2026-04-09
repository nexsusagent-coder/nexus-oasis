//! ═══════════════════════════════════════════════════════════════════════════════
//!  SENTIENT SETUP WIZARD - Interactive TUI Binary
//!  Arrow-key navigation, Space for multi-select, Enter to confirm
//! ═══════════════════════════════════════════════════════════════════════════════

use sentient_setup::SetupWizard;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // TUI Sihirbazını başlat
    let mut wizard = SetupWizard::new();
    let result = wizard.run().await?;
    
    // Sonuç göster
    match result.status {
        sentient_setup::SetupStatus::Completed => {
            println!("\n✅ Kurulum başarıyla tamamlandı!");
            println!("📁 Config: {}", result.config_path);
            if !result.integrations_enabled.is_empty() {
                println!("🔌 Aktif integrasyonlar: {}", result.integrations_enabled.join(", "));
            }
            std::process::exit(0);
        }
        sentient_setup::SetupStatus::Failed(err) => {
            eprintln!("\n❌ Kurulum başarısız: {}", err);
            std::process::exit(1);
        }
        _ => {
            std::process::exit(0);
        }
    }
}
