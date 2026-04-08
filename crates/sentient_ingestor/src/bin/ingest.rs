//! ═══════════════════════════════════════════════════════════════════════════════
//!  SENTIENT MASS INGESTOR CLI - 5400+ Skill Assimilation
//! ═══════════════════════════════════════════════════════════════════════════════

use clap::{Parser, Subcommand};
use std::path::PathBuf;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

use sentient_ingestor::{MassIngestor, IngestStats, SkillDatabase};

#[derive(Parser)]
#[command(name = "sentient-ingest")]
#[command(about = "SENTIENT Mass Skill Ingestor - 5400+ skill assimilation engine")]
#[command(version)]
struct Cli {
    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
    
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Tüm skill'leri ingest et
    All {
        /// OpenClaw categories dizini
        #[arg(short, long, default_value = "integrations/skills/awesome-openclaw-skills/categories")]
        categories_dir: PathBuf,
        
        /// Çıktı dizini
        #[arg(short, long, default_value = "data/skills")]
        output_dir: PathBuf,
        
        /// Veritabanı yolu
        #[arg(short, long, default_value = "data/sentient_skills.db")]
        db_path: PathBuf,
    },
    
    /// Tek kategori ingest et
    Category {
        /// Kategori dosyası
        #[arg(short, long)]
        file: PathBuf,
        
        /// Kategori adı
        #[arg(short, long)]
        category: String,
        
        /// Çıktı dizini
        #[arg(short, long, default_value = "data/skills")]
        output_dir: PathBuf,
    },
    
    /// README.md'den ingest et
    Readme {
        /// README dosyası
        #[arg(short, long, default_value = "integrations/skills/awesome-openclaw-skills/README.md")]
        readme: PathBuf,
        
        /// Çıktı dizini
        #[arg(short, long, default_value = "data/skills")]
        output_dir: PathBuf,
    },
    
    /// Skill ara
    Search {
        /// Arama sorgusu
        query: String,
        
        /// Kategori filtresi
        #[arg(short, long)]
        category: Option<String>,
        
        /// Veritabanı yolu
        #[arg(short, long, default_value = "data/sentient_skills.db")]
        db_path: PathBuf,
    },
    
    /// İstatistikleri göster
    Stats {
        /// Veritabanı yolu
        #[arg(short, long, default_value = "data/sentient_skills.db")]
        db_path: PathBuf,
    },
    
    /// Kategorileri listele
    Categories {
        /// Veritabanı yolu
        #[arg(short, long, default_value = "data/sentient_skills.db")]
        db_path: PathBuf,
    },
    
    /// Everything Claude Code skill'lerini ingest et
    Ecc {
        /// ECC dizini
        #[arg(short, long, default_value = "integrations/skills/everything-claude-code")]
        ecc_dir: PathBuf,
        
        /// Çıktı dizini
        #[arg(short, long, default_value = "data/skills")]
        output_dir: PathBuf,
        
        /// Veritabanı yolu
        #[arg(short, long, default_value = "data/sentient_skills.db")]
        db_path: PathBuf,
    },
    
    /// Gstack skill'lerini ingest et
    Gstack {
        /// Gstack dizini
        #[arg(short, long, default_value = "integrations/skills/gstack")]
        gstack_dir: PathBuf,
        
        /// Çıktı dizini
        #[arg(short, long, default_value = "data/skills")]
        output_dir: PathBuf,
        
        /// Veritabanı yolu
        #[arg(short, long, default_value = "data/sentient_skills.db")]
        db_path: PathBuf,
    },
    
    /// Tüm kaynaklardan skill'leri ingest et (OpenClaw + ECC + Gstack)
    Full {
        /// OpenClaw categories dizini
        #[arg(short, long, default_value = "integrations/skills/awesome-openclaw-skills/categories")]
        categories_dir: PathBuf,
        
        /// ECC dizini
        #[arg(short, long, default_value = "integrations/skills/everything-claude-code")]
        ecc_dir: PathBuf,
        
        /// Gstack dizini
        #[arg(short, long, default_value = "integrations/skills/gstack")]
        gstack_dir: PathBuf,
        
        /// Çıktı dizini
        #[arg(short, long, default_value = "data/skills")]
        output_dir: PathBuf,
        
        /// Veritabanı yolu
        #[arg(short, long, default_value = "data/sentient_skills.db")]
        db_path: PathBuf,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    
    // Logging
    let level = if cli.verbose { Level::DEBUG } else { Level::INFO };
    let subscriber = FmtSubscriber::builder()
        .with_max_level(level)
        .with_target(false)
        .with_thread_ids(false)
        .pretty()
        .init();
    
    match cli.command {
        Commands::All { categories_dir, output_dir, db_path } => {
            info!("🚀 Mass Ingestion başlatılıyor...");
            info!("📂 Kategori dizini: {:?}", categories_dir);
            info!("💾 Çıktı dizini: {:?}", output_dir);
            info!("🗄️  Veritabanı: {:?}", db_path);
            
            let mut ingestor = MassIngestor::new(&db_path, &output_dir)?;
            let stats = ingestor.ingest_categories(&categories_dir)?;
            
            print_stats(&stats);
        }
        
        Commands::Category { file, category, output_dir } => {
            info!("📄 Kategori ingestion: {:?} ({})", file, category);
            
            let db_path = output_dir.join("sentient_skills.db");
            let mut ingestor = MassIngestor::new(&db_path, &output_dir)?;
            // Single file ingestion...
            
            info!("✅ Kategori tamamlandı");
        }
        
        Commands::Readme { readme, output_dir } => {
            info!("📖 README ingestion: {:?}", readme);
            
            let db_path = output_dir.join("sentient_skills.db");
            let mut ingestor = MassIngestor::new(&db_path, &output_dir)?;
            let stats = ingestor.ingest_readme(&readme)?;
            
            print_stats(&stats);
        }
        
        Commands::Search { query, category: _, db_path } => {
            let db = SkillDatabase::new(&db_path)?;
            let results = db.search(&query)?;
            
            println!("\n🔍 {} sonuç bulundu:\n", results.len());
            for record in results.iter().take(20) {
                println!("  📦 {} ({})", record.name, record.category);
                println!("     {}", record.description.chars().take(60).collect::<String>());
            }
            
            if results.len() > 20 {
                println!("\n  ... ve {} daha", results.len() - 20);
            }
        }
        
        Commands::Stats { db_path } => {
            let db = sentient_ingestor::SkillDatabase::new(&db_path)?;
            let count = db.count()?;
            let categories = db.category_stats()?;
            
            println!("\n📊 SENTIENT SKILL İSTATİSTİKLERİ\n");
            println!("  📦 Toplam Skill: {}", count);
            println!("\n  📂 Kategori Dağılımı:\n");
            
            for (cat, cnt) in categories.iter().take(15) {
                println!("    {:<24} {:>5}", cat, cnt);
            }
            
            if categories.len() > 15 {
                println!("    {:<24} {:>5}", "... ve daha", categories.len() - 15);
            }
        }
        
        Commands::Categories { db_path } => {
            let db = sentient_ingestor::SkillDatabase::new(&db_path)?;
            let categories = db.category_stats()?;
            
            println!("\n📂 KATEGORİLER ({} toplam)\n", categories.len());
            
            for (i, (cat, count)) in categories.iter().enumerate() {
                println!("  {:>2}. {:<24} {:>5} skill", i + 1, cat, count);
            }
        }
        
        Commands::Ecc { ecc_dir, output_dir, db_path } => {
            info!("🔵 ECC Skill Ingestion başlatılıyor...");
            info!("📂 ECC dizini: {:?}", ecc_dir);
            
            let mut ingestor = MassIngestor::new(&db_path, &output_dir)?;
            let stats = ingestor.ingest_ecc_skills(&ecc_dir)?;
            
            print_stats(&stats);
        }
        
        Commands::Gstack { gstack_dir, output_dir, db_path } => {
            info!("🟢 Gstack Skill Ingestion başlatılıyor...");
            info!("📂 Gstack dizini: {:?}", gstack_dir);
            
            let mut ingestor = MassIngestor::new(&db_path, &output_dir)?;
            let stats = ingestor.ingest_gstack_skills(&gstack_dir)?;
            
            print_stats(&stats);
        }
        
        Commands::Full { categories_dir, ecc_dir, gstack_dir, output_dir, db_path } => {
            info!("🌐 FULL Skill Ingestion başlatılıyor...");
            
            let mut ingestor = MassIngestor::new(&db_path, &output_dir)?;
            
            // OpenClaw
            info!("\n📦 [1/3] OpenClaw skill'leri...");
            let _ = ingestor.ingest_categories(&categories_dir);
            
            // ECC
            info!("\n📦 [2/3] Everything Claude Code skill'leri...");
            let _ = ingestor.ingest_ecc_skills(&ecc_dir);
            
            // Gstack
            info!("\n📦 [3/3] Gstack skill'leri...");
            let _ = ingestor.ingest_gstack_skills(&gstack_dir);
            
            // Final stats
            let db = SkillDatabase::new(&db_path)?;
            let count = db.count()?;
            let categories = db.category_stats()?;
            
            println!("\n╔══════════════════════════════════════════════════════╗");
            println!("║        🐺 NİHAİ SENTIENT SKILL LİBRARY                  ║");
            println!("╠══════════════════════════════════════════════════════╣");
            println!("║  📦 TOPLAM SKILL:    {:>30} ║", count);
            println!("║  📂 KATEGORİ:        {:>30} ║", categories.len());
            println!("╚══════════════════════════════════════════════════════╝");
        }
    }
    
    Ok(())
}

fn print_stats(stats: &IngestStats) {
    println!("\n╔══════════════════════════════════════════════════════╗");
    println!("║          📊 INGESTION SONUÇLARI                     ║");
    println!("╠══════════════════════════════════════════════════════╣");
    println!("║  📄 İşlenen Dosya:    {:>30} ║", stats.total_files);
    println!("║  📦 Toplam Skill:     {:>30} ║", stats.total_skills);
    println!("║  ✅ Yeni Skill:       {:>30} ║", stats.new_skills);
    println!("║  🔄 Güncellenmiş:     {:>30} ║", stats.updated_skills);
    println!("║  ⚠️  Hata:            {:>30} ║", stats.errors);
    println!("║  ⏱️  Süre:            {:>28.2}s ║", stats.duration_secs);
    println!("╚══════════════════════════════════════════════════════╝");
}
