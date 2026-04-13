//! ═══════════════════════════════════════════════════════════════════════════════
//!  SETUP WIZARD - İnteraktif Kurulum Sihirbazı
//! ═══════════════════════════════════════════════════════════════════════════════

use crate::error::{HandsError, HandsResult};
use crate::setup::{
    config::{SetupConfig, Platform, SecurityMode, Rect},
    permissions::{Permission, PermissionManager},
    profiles::{ProfileManager, ProfileType, SetupProfile},
    tests::{SystemTester, TestResult},
    approval::ApprovalManager,
    config_path, profiles_dir, approvals_dir,
};
use std::io::{self, Write};

/// Setup Wizard
pub struct SetupWizard {
    /// Mevcut yapılandırma
    config: Option<SetupConfig>,
    /// İzin yöneticisi
    permissions: PermissionManager,
    /// Profil yöneticisi
    profiles: ProfileManager,
    /// Sistem testcisi
    tester: SystemTester,
    /// Onay yöneticisi
    approvals: ApprovalManager,
    /// Terminal genişliği
    term_width: usize,
}

/// Kurulum modu
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SetupMode {
    /// Tam otomatik (tüm varsayılanlar)
    Auto,
    /// İnteraktif (soru-cevap)
    Interactive,
    /// Sessiz (config dosyasından)
    Silent,
    /// Daha sonra
    Later,
    /// Sadece test
    TestOnly,
    /// Onarım modu
    Repair,
}

/// Wizard sonucu
#[derive(Debug)]
pub struct WizardResult {
    pub success: bool,
    pub config: Option<SetupConfig>,
    pub test_results: Vec<TestResult>,
    pub warnings: Vec<String>,
    pub next_steps: Vec<String>,
}

impl SetupWizard {
    /// Yeni wizard oluştur
    pub fn new() -> HandsResult<Self> {
        let term_width = std::env::var("COLUMNS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(80);
        
        // Dizinleri oluştur
        std::fs::create_dir_all(profiles_dir()).ok();
        std::fs::create_dir_all(approvals_dir()).ok();
        
        Ok(Self {
            config: SetupConfig::load().ok(),
            permissions: PermissionManager::new(),
            profiles: ProfileManager::new(),
            tester: SystemTester::new(),
            approvals: ApprovalManager::new(),
            term_width,
        })
    }
    
    /// Wizard'ı çalıştır
    pub async fn run(&mut self, mode: SetupMode) -> HandsResult<WizardResult> {
        self.print_banner();
        
        let result = match mode {
            SetupMode::Auto => self.run_auto().await,
            SetupMode::Interactive => self.run_interactive().await,
            SetupMode::Silent => self.run_silent().await,
            SetupMode::Later => self.run_later().await,
            SetupMode::TestOnly => self.run_tests_only().await,
            SetupMode::Repair => self.run_repair().await,
        };
        
        result
    }
    
    // ═══════════════════════════════════════════════════════════════════════════
    //  MOD'LAR
    // ═══════════════════════════════════════════════════════════════════════════
    
    /// Otomatik kurulum
    async fn run_auto(&mut self) -> HandsResult<WizardResult> {
        self.print_section("🚀 OTOMATİK KURULUM");
        println!();
        println!("  Tüm varsayılan ayarlarla kurulum yapılacak.");
        println!("  Bu işlem şunları içerecek:");
        println!("    • Ekran yakalama izni");
        println!("    • Fare kontrolü izni");
        println!("    • Klavye kontrolü izni");
        println!("    • Güvenlik ayarları (Normal mod)");
        println!("    • İnsan benzerlik ayarları (aktif)");
        println!();
        
        let confirm = self.ask_yes_no("Devam edilsin mi?", true)?;
        if !confirm {
            return self.run_later().await;
        }
        
        println!();
        self.print_progress("Platform tespit ediliyor...");
        let platform = self.detect_platform();
        self.print_success(&format!("Platform: {:?}", platform));
        
        self.print_progress("Sistem testleri yapılıyor...");
        let test_results = self.tester.run_all().await;
        let failed = test_results.iter().filter(|r| !r.passed).count();
        
        if failed > 0 {
            self.print_warning(&format!("{} test başarısız!", failed));
            for result in &test_results {
                if !result.passed {
                    println!("    ❌ {}", result.name);
                }
            }
        } else {
            self.print_success("Tüm testler geçti!");
        }
        
        // Varsayılan config oluştur
        let config = SetupConfig::default()
            .with_platform(platform)
            .with_permissions(vec![
                Permission::ScreenCapture,
                Permission::MouseControl,
                Permission::KeyboardControl,
                Permission::WindowManagement,
            ]);
        
        config.save()?;
        self.config = Some(config.clone());
        
        println!();
        self.print_success("✅ Kurulum tamamlandı!");
        println!();
        println!("  Config: {}", config_path().display());
        println!();
        
        Ok(WizardResult {
            success: true,
            config: Some(config),
            test_results,
            warnings: vec![],
            next_steps: vec![
                "oasis-hands run".to_string(),
                "oasis-hands test".to_string(),
            ],
        })
    }
    
    /// İnteraktif kurulum
    async fn run_interactive(&mut self) -> HandsResult<WizardResult> {
        self.print_section("📋 İNTERAKTİF KURULUM");
        println!();
        
        let mut warnings = Vec::new();
        let mut test_results = Vec::new();
        
        // ═══════════════════════════════════════════════════════════════════════
        //  ADIM 1: PLATFORM
        // ═══════════════════════════════════════════════════════════════════════
        self.print_step(1, 8, "Platform Tespiti");
        let platform = self.detect_platform();
        println!("  Tespit edilen platform: {:?}", platform);
        println!();
        
        // ═══════════════════════════════════════════════════════════════════════
        //  ADIM 2: İZİNLER
        // ═══════════════════════════════════════════════════════════════════════
        self.print_step(2, 8, "İzin Ayarları");
        println!();
        println!("  Masaüstü kontrolü için şu izinler gerekiyor:");
        println!();
        
        let screen = self.ask_permission(
            "📸 Ekran Yakalama",
            "Ekran görüntüsü almak için gerekli",
            true,
        )?;
        
        let mouse = self.ask_permission(
            "🖱️  Fare Kontrolü",
            "Fare hareketleri ve tıklamalar için gerekli",
            true,
        )?;
        
        let keyboard = self.ask_permission(
            "⌨️  Klavye Kontrolü",
            "Yazma ve kısayollar için gerekli",
            true,
        )?;
        
        let window = self.ask_permission(
            "🪟 Pencere Yönetimi",
            "Pencereleri açma/kapama için gerekli",
            true,
        )?;
        
        let filesystem = self.ask_permission(
            "📁 Dosya Sistemi (Sınırlı)",
            "İzin verilen dizinlerde okuma/yazma",
            false,
        )?;
        
        let process = self.ask_permission(
            "⚡ Process Başlatma",
            "İzin verilen uygulamaları başlatma",
            false,
        )?;
        
        // ═══════════════════════════════════════════════════════════════════════
        //  ADIM 3: GÜVENLİK
        // ═══════════════════════════════════════════════════════════════════════
        self.print_step(3, 8, "Güvenlik Ayarları");
        println!();
        
        println!("  Güvenlik modu seçin:");
        println!("    [1] Strict  - En yüksek güvenlik (her aksiyon onay gerekir)");
        println!("    [2] Normal  - Dengeli (tehlikeli aksiyonlar onay gerekir)");
        println!("    [3] Relaxed - Rahat (sadece yasaklı komutlar engellenir)");
        println!();
        
        let security_choice = self.ask_number("Seçiminiz", 2, 1, 3)?;
        let security_mode = match security_choice {
            1 => SecurityMode::Strict,
            2 => SecurityMode::Normal,
            3 => SecurityMode::Relaxed,
            _ => SecurityMode::Normal,
        };
        
        let require_approval = match security_mode {
            SecurityMode::Strict => true,
            SecurityMode::Normal => self.ask_yes_no(
                "Kritik işlemler için onay istensin mi?",
                true,
            )?,
            SecurityMode::Relaxed => false,
        };
        
        let max_actions = self.ask_number(
            "Maksimum aksiyon/dakika (30-300)",
            60,
            30,
            300,
        )?;
        
        let emergency_stop = self.ask_yes_no(
            "Acil durum durdurması (ESC x3) aktif olsun mu?",
            true,
        )?;
        
        // ═══════════════════════════════════════════════════════════════════════
        //  ADIM 4: İNSAN BENZERLİK (GELİŞMİŞ)
        // ═══════════════════════════════════════════════════════════════════════
        self.print_step(4, 8, "İnsan Benzerlik Ayarları (Gelişmiş)");
        println!();
        
        // --- MOUSE HAREKETİ ---
        println!("  🖱️  FARE HAREKET AYARLARI");
        println!("  ─────────────────────────────");
        
        let human_mouse = self.ask_yes_no(
            "Fare hareketleri insan gibi olsun mu?",
            true,
        )?;
        
        let (mouse_pattern, bezier_quality, tremor, tremor_intensity) = if human_mouse {
            println!();
            println!("  Hareket paterni seçin:");
            println!("    [1] Linear    - Düz çizgi (robotik)");
            println!("    [2] Curved    - Eğrisel (Bezier)");
            println!("    [3] Wavy      - Dalgalı");
            println!("    [4] Natural   - Doğal (RNN-LSTM)");
            println!("    [5] Adaptive  - Duruma göre adaptif");
            println!();
            
            let pattern_choice = self.ask_number("Seçiminiz", 4, 1, 5)?;
            let mouse_pattern = match pattern_choice {
                1 => "linear",
                2 => "curved",
                3 => "wavy",
                4 => "natural",
                5 => "adaptive",
                _ => "natural",
            };
            
            let bezier_quality = self.ask_number(
                "Bezier eğri kalitesi (20-100 nokta)",
                50,
                20,
                100,
            )? as u32;
            
            let tremor = self.ask_yes_no(
                "Doğal fare titreşimi eklensin mi? (anti-detect)",
                true,
            )?;
            
            let tremor_intensity = if tremor {
                self.ask_number(
                    "Titreşim yoğunluğu % (1-20)",
                    5,
                    1,
                    20,
                )? as f64 / 100.0
            } else {
                0.0
            };
            
            (mouse_pattern, bezier_quality, tremor, tremor_intensity)
        } else {
            ("linear", 20, false, 0.0)
        };
        
        println!();
        
        // --- KLAVYE YAZMA ---
        println!("  ⌨️  KLAVYE YAZMA AYARLARI");
        println!("  ─────────────────────────────");
        
        let human_typing = self.ask_yes_no(
            "Yazma hareketleri insan gibi olsun mu?",
            true,
        )?;
        
        let (typing_profile, typing_wpm, simulate_errors, error_rate) = if human_typing {
            println!();
            println!("  Yazma profili seçin:");
            println!("    [1] Beginner    - Yavaş, çok hata (20 WPM)");
            println!("    [2] Intermediate - Orta, az hata (40 WPM)");
            println!("    [3] Expert      - Hızlı, nadir hata (70 WPM)");
            println!("    [4] Custom      - Özel ayarlar");
            println!();
            
            let profile_choice = self.ask_number("Seçiminiz", 2, 1, 4)?;
            
            let (profile, wpm, err_rate) = match profile_choice {
                1 => ("beginner", 20u32, 0.08),
                2 => ("intermediate", 40u32, 0.03),
                3 => ("expert", 70u32, 0.01),
                4 => {
                    let wpm = self.ask_number(
                        "Özel WPM (10-120)",
                        45,
                        10,
                        120,
                    )? as u32;
                    let err = self.ask_number(
                        "Hata oranı % (0-20)",
                        2,
                        0,
                        20,
                    )? as f64 / 100.0;
                    ("custom", wpm, err)
                },
                _ => ("intermediate", 40u32, 0.03),
            };
            
            let simulate_errors = err_rate > 0.0;
            
            (profile, wpm, simulate_errors, err_rate)
        } else {
            ("robot", 120, false, 0.0)
        };
        
        println!();
        
        // --- DAVRANIŞ MODELİ ---
        println!("  🧠 DAVRANIŞ MODELİ AYARLARI");
        println!("  ─────────────────────────────");
        
        let use_rnn_model = self.ask_yes_no(
            "RNN-LSTM davranış modeli kullanılsın mı? (daha akıllı seçimler)",
            true,
        )?;
        
        let best_of_n = if use_rnn_model {
            self.ask_number(
                "Best-of-N değeri (3-10, yüksek = daha iyi seçimler)",
                5,
                3,
                10,
            )? as u32
        } else {
            1
        };
        
        let exploration_rate = self.ask_number(
            "Keşif oranı % (0-30, bazen rastgele deneme)",
            10,
            0,
            30,
        )? as f64 / 100.0;
        
        println!();
        
        // --- DİKKAT VE YORGUNLUK ---
        println!("  👁️  DİKKAT VE YORGUNLUK SİMÜLASYONU");
        println!("  ─────────────────────────────");
        
        let simulate_attention = self.ask_yes_no(
            "Dikkat dağılma simülasyonu aktif olsun mu?",
            true,
        )?;
        
        let (attention_span, distraction_rate) = if simulate_attention {
            let span = self.ask_number(
                "Ortalama odaklanma süresi saniye (30-300)",
                120,
                30,
                300,
            )? as u32;
            let dist = self.ask_number(
                "Dikkat dağılma oranı % (0-30)",
                5,
                0,
                30,
            )? as f64 / 100.0;
            (span, dist)
        } else {
            (u32::MAX, 0.0)
        };
        
        let simulate_fatigue = self.ask_yes_no(
            "Yorgunluk simülasyonu aktif olsun mu? (zamanla yavaşlama)",
            true,
        )?;
        
        let fatigue_rate = if simulate_fatigue {
            self.ask_number(
                "Yorgunluk hızı %/saat (0-50)",
                10,
                0,
                50,
            )? as f64 / 100.0
        } else {
            0.0
        };
        
        println!();
        
        // --- KARAR VERME ---
        println!("  🤔 KARAR VERME AYARLARI");
        println!("  ─────────────────────────────");
        
        let decision_delay_min = self.ask_number(
            "Minimum karar verme süresi ms (50-500)",
            100,
            50,
            500,
        )? as u32;
        
        let decision_delay_max = self.ask_number(
            "Maksimum karar verme süresi ms (100-2000)",
            500,
            decision_delay_min as u64,
            2000,
        )? as u32;
        
        let hesitation_rate = self.ask_number(
            "Tereddüt oranı % (0-20, bazen duraklama)",
            5,
            0,
            20,
        )? as f64 / 100.0;
        
        println!();
        
        // --- EL TERCİHİ ---
        let hand_preference = if self.ask_yes_no("El tercihi belirtmek ister misiniz?", false)? {
            println!("  [1] Sağ el (varsayılan)");
            println!("  [2] Sol el");
            println!("  [3] Her iki el");
            let choice = self.ask_number("Seçiminiz", 1, 1, 3)?;
            match choice {
                1 => "right",
                2 => "left",
                3 => "both",
                _ => "right",
            }
        } else {
            "right"
        };
        
        println!();
        
        // --- GENEL İNSAN BENZERLİĞİ ---
        let humanlikeness = self.ask_number(
            "Genel insan benzerliği seviyesi % (50-100)",
            85,
            50,
            100,
        )? as f64 / 100.0;
        
        // ═══════════════════════════════════════════════════════════════════════
        //  ADIM 5: YASAKLI ALANLAR
        // ═══════════════════════════════════════════════════════════════════════
        self.print_step(5, 8, "Yasaklı Alanlar");
        println!();
        
        let mut forbidden_regions: Vec<Rect> = Vec::new();
        
        if self.ask_yes_no("Ekranın bazı bölgelerini yasaklamak ister misiniz?", false)? {
            println!();
            println!("  Yasaklı bölge formatı: x,y,width,height");
            println!("  Örnek: 0,0,200,100 (sol üst köşe)");
            println!();
            
            loop {
                let region = self.ask_string("Yasaklı bölge (boş = bitir)")?;
                if region.is_empty() {
                    break;
                }
                
                if let Some(rect) = self.parse_region(&region) {
                    forbidden_regions.push(rect.clone());
                    println!("    ✓ Eklendi: x={}, y={}, w={}, h={}", 
                        rect.x, rect.y, rect.width, rect.height);
                } else {
                    println!("    ✗ Geçersiz format!");
                }
            }
        }
        
        let mut forbidden_apps: Vec<String> = Vec::new();
        
        if self.ask_yes_no("Bazı uygulamaları yasaklamak ister misiniz?", false)? {
            println!();
            println!("  Uygulama adı girin (boş = bitir)");
            println!();
            
            loop {
                let app = self.ask_string("Yasaklı uygulama")?;
                if app.is_empty() {
                    break;
                }
                forbidden_apps.push(app);
            }
        }
        
        // ═══════════════════════════════════════════════════════════════════════
        //  ADIM 6: ONAY SİSTEMİ
        // ═══════════════════════════════════════════════════════════════════════
        self.print_step(6, 8, "Onay Sistemi");
        println!();
        
        let remember_approvals = self.ask_yes_no(
            "Verilen onayları hatırla mı? (aynı işlem tekrar sorulmaz)",
            true,
        )?;
        
        let approval_timeout = if require_approval {
            self.ask_number(
                "Onay bekleme süresi saniye (10-300, 0=sınırsız)",
                30,
                0,
                300,
            )?
        } else {
            0
        };
        
        let auto_approve_safe = self.ask_yes_no(
            "Güvenli işlemler otomatik onaylansın mı?",
            true,
        )?;
        
        // ═══════════════════════════════════════════════════════════════════════
        //  ADIM 7: SİSTEM TESTLERİ
        // ═══════════════════════════════════════════════════════════════════════
        self.print_step(7, 8, "Sistem Testleri");
        println!();
        
        let run_tests = self.ask_yes_no("Sistem testleri çalıştırılsın mı?", true)?;
        
        if run_tests {
            self.print_progress("Testler çalışıyor...");
            test_results = self.tester.run_all().await;
            
            println!();
            for result in &test_results {
                if result.passed {
                    println!("    ✅ {} - {}", result.name, result.message);
                } else {
                    println!("    ❌ {} - {}", result.name, result.message);
                    warnings.push(format!("{} testi başarısız", result.name));
                }
            }
        }
        
        // ═══════════════════════════════════════════════════════════════════════
        //  ADIM 8: PROFIL KAYDETME
        // ═══════════════════════════════════════════════════════════════════════
        self.print_step(8, 8, "Profil Kaydetme");
        println!();
        
        let profile_name = if self.ask_yes_no("Bu ayarları profil olarak kaydetmek ister misiniz?", true)? {
            let name = self.ask_string("Profil adı")?;
            if name.is_empty() {
                "default".to_string()
            } else {
                name
            }
        } else {
            "default".to_string()
        };
        
        // ═══════════════════════════════════════════════════════════════════════
        //  ÖZET VE ONAY
        // ═══════════════════════════════════════════════════════════════════════
        println!();
        self.print_section("📊 KURULUM ÖZETİ");
        println!();
        
        self.print_summary_box(&SummaryData {
            platform: &format!("{:?}", platform),
            permissions: &PermissionSummary {
                screen, mouse, keyboard, window, filesystem, process,
            },
            security: SecuritySummary {
                mode: &format!("{:?}", security_mode),
                require_approval,
                max_actions,
                emergency_stop,
            },
            human: &HumanSummary {
                mouse: human_mouse,
                mouse_pattern: mouse_pattern.to_string(),
                bezier_quality,
                tremor,
                tremor_intensity: (tremor_intensity * 100.0) as u32,
                typing: human_typing,
                typing_profile: typing_profile.to_string(),
                wpm: typing_wpm,
                errors: simulate_errors,
                error_rate: (error_rate * 100.0) as u32,
                use_rnn_model,
                best_of_n,
                exploration_rate: (exploration_rate * 100.0) as u32,
                simulate_attention,
                attention_span_sec: attention_span,
                distraction_rate: (distraction_rate * 100.0) as u32,
                simulate_fatigue,
                fatigue_rate: (fatigue_rate * 100.0) as u32,
                decision_delay_min_ms: decision_delay_min,
                decision_delay_max_ms: decision_delay_max,
                hesitation_rate: (hesitation_rate * 100.0) as u32,
                hand_preference: hand_preference.to_string(),
                level: (humanlikeness * 100.0) as u32,
            },
            forbidden: &ForbiddenSummary {
                regions: forbidden_regions.len(),
                apps: forbidden_apps.len(),
            },
            approval: &ApprovalSummary {
                remember: remember_approvals,
                timeout: approval_timeout,
                auto_safe: auto_approve_safe,
            },
            profile: &profile_name,
        });
        
        println!();
        let final_confirm = self.ask_yes_no("Bu ayarlarla kurulum tamamlansın mı?", true)?;
        
        if !final_confirm {
            return self.run_later().await;
        }
        
        // Config oluştur ve kaydet
        let mut config = SetupConfig::default()
            .with_platform(platform)
            .with_permissions({
                let mut perms = Vec::new();
                if screen { perms.push(Permission::ScreenCapture); }
                if mouse { perms.push(Permission::MouseControl); }
                if keyboard { perms.push(Permission::KeyboardControl); }
                if window { perms.push(Permission::WindowManagement); }
                if filesystem { perms.push(Permission::FileAccess); }
                if process { perms.push(Permission::ProcessSpawn); }
                perms
            })
            .with_security_mode(security_mode)
            .with_approval_settings(require_approval, remember_approvals, approval_timeout, auto_approve_safe)
            .with_human_settings(
                human_mouse, mouse_pattern, bezier_quality, tremor, tremor_intensity,
                human_typing, typing_profile, typing_wpm, simulate_errors, error_rate,
                use_rnn_model, best_of_n, exploration_rate,
                simulate_attention, attention_span, distraction_rate,
                simulate_fatigue, fatigue_rate,
                decision_delay_min, decision_delay_max, hesitation_rate,
                hand_preference,
                humanlikeness
            )
            .with_forbidden_areas(forbidden_regions, forbidden_apps)
            .with_emergency_stop(emergency_stop)
            .with_max_actions(max_actions);
        
        config.profile_name = profile_name.clone();
        config.save()?;
        
        // Profili kaydet
        let profile = SetupProfile::from_config(&config, profile_name, ProfileType::Custom);
        self.profiles.save(&profile)?;
        
        self.config = Some(config.clone());
        
        println!();
        self.print_success("🎉 Kurulum başarıyla tamamlandı!");
        println!();
        println!("  📁 Config: {}", config_path().display());
        println!("  📁 Profil: ~/.config/sentient/profiles/{}.toml", config.profile_name);
        println!();
        println!("  Sonraki adımlar:");
        println!("    • oasis-hands run          # Masaüstü kontrolünü başlat");
        println!("    • oasis-hands test         # Testleri tekrar çalıştır");
        println!("    • oasis-hands config show  # Ayarları görüntüle");
        println!("    • oasis-hands profile list # Profilleri listele");
        println!();
        
        Ok(WizardResult {
            success: true,
            config: Some(config),
            test_results,
            warnings,
            next_steps: vec![
                "oasis-hands run".to_string(),
                "oasis-hands test".to_string(),
            ],
        })
    }
    
    /// Sessiz kurulum
    async fn run_silent(&mut self) -> HandsResult<WizardResult> {
        self.print_section("🔇 SESSEZ KURULUM");
        println!();
        
        match SetupConfig::load() {
            Ok(config) => {
                self.config = Some(config.clone());
                self.print_success("Config dosyası yüklendi!");
                Ok(WizardResult {
                    success: true,
                    config: Some(config),
                    test_results: vec![],
                    warnings: vec![],
                    next_steps: vec![],
                })
            }
            Err(_) => {
                self.print_error("Config dosyası bulunamadı!");
                println!("  Önce interaktif kurulum çalıştırın: oasis-hands setup");
                Err(HandsError::ConfigNotFound)
            }
        }
    }
    
    /// Daha sonra
    async fn run_later(&self) -> HandsResult<WizardResult> {
        println!();
        println!("  ⏸️  Kurulum ertelendi.");
        println!();
        println!("  Daha sonra şu komutlarla kurulum yapabilirsiniz:");
        println!("    • oasis-hands setup          # İnteraktif kurulum");
        println!("    • oasis-hands setup --auto   # Otomatik kurulum");
        println!();
        
        Ok(WizardResult {
            success: false,
            config: None,
            test_results: vec![],
            warnings: vec![],
            next_steps: vec!["oasis-hands setup".to_string()],
        })
    }
    
    /// Sadece test
    async fn run_tests_only(&mut self) -> HandsResult<WizardResult> {
        self.print_section("🧪 SİSTEM TESTLERİ");
        println!();
        
        let results = self.tester.run_all().await;
        
        println!();
        println!("  ═════════════════════════════════════════════════════════");
        
        let passed = results.iter().filter(|r| r.passed).count();
        let total = results.len();
        
        for result in &results {
            if result.passed {
                println!("  ✅ {} - {}", result.name, result.message);
            } else {
                println!("  ❌ {} - {}", result.name, result.message);
            }
        }
        
        println!("  ═════════════════════════════════════════════════════════");
        println!();
        println!("  Sonuç: {}/{} test geçti", passed, total);
        println!();
        
        Ok(WizardResult {
            success: passed == total,
            config: self.config.clone(),
            test_results: results,
            warnings: vec![],
            next_steps: vec![],
        })
    }
    
    /// Onarım modu
    async fn run_repair(&mut self) -> HandsResult<WizardResult> {
        self.print_section("🔧 ONARIM MODU");
        println!();
        
        let mut fixed = Vec::new();
        let mut errors = Vec::new();
        
        // Config dosyasını kontrol et
        self.print_progress("Config dosyası kontrol ediliyor...");
        if config_path().exists() {
            match SetupConfig::load() {
                Ok(config) => {
                    self.print_success("Config dosyası geçerli");
                    self.config = Some(config);
                }
                Err(_) => {
                    self.print_warning("Config dosyası bozuk, yeniden oluşturuluyor...");
                    let config = SetupConfig::default();
                    config.save()?;
                    self.config = Some(config);
                    fixed.push("Config dosyası yeniden oluşturuldu".to_string());
                }
            }
        } else {
            self.print_warning("Config dosyası yok, oluşturuluyor...");
            let config = SetupConfig::default();
            config.save()?;
            self.config = Some(config);
            fixed.push("Config dosyası oluşturuldu".to_string());
        }
        
        // Dizinleri kontrol et
        self.print_progress("Gerekli dizinler kontrol ediliyor...");
        let dirs = vec![profiles_dir(), approvals_dir()];
        
        for dir in dirs {
            if !dir.exists() {
                std::fs::create_dir_all(&dir)?;
                fixed.push(format!("Dizin oluşturuldu: {:?}", dir));
            }
        }
        
        // Testleri çalıştır
        self.print_progress("Sistem testleri yapılıyor...");
        let test_results = self.tester.run_all().await;
        
        for result in &test_results {
            if !result.passed {
                errors.push(format!("{}: {}", result.name, result.message));
            }
        }
        
        println!();
        
        if !fixed.is_empty() {
            println!("  🔧 Düzeltilen sorunlar:");
            for f in &fixed {
                println!("    ✓ {}", f);
            }
            println!();
        }
        
        if !errors.is_empty() {
            println!("  ⚠️  Dikkat edilmesi gerekenler:");
            for e in &errors {
                println!("    ! {}", e);
            }
            println!();
        }
        
        if fixed.is_empty() && errors.is_empty() {
            self.print_success("Sistem sağlıklı, sorun bulunamadı!");
        } else {
            self.print_success(&format!("{} sorun düzeltildi!", fixed.len()));
        }
        
        Ok(WizardResult {
            success: true,
            config: self.config.clone(),
            test_results,
            warnings: errors,
            next_steps: vec![],
        })
    }
    
    // ═══════════════════════════════════════════════════════════════════════════
    //  YARDIMCI FONKSİYONLAR
    // ═══════════════════════════════════════════════════════════════════════════
    
    fn detect_platform(&self) -> Platform {
        #[cfg(target_os = "linux")]
        { Platform::Linux }
        
        #[cfg(target_os = "windows")]
        { Platform::Windows }
        
        #[cfg(target_os = "macos")]
        { Platform::MacOS }
        
        #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
        { Platform::Unknown }
    }
    
    fn parse_region(&self, input: &str) -> Option<Rect> {
        let parts: Vec<&str> = input.split(',').collect();
        if parts.len() != 4 {
            return None;
        }
        
        Some(Rect {
            x: parts[0].trim().parse().ok()?,
            y: parts[1].trim().parse().ok()?,
            width: parts[2].trim().parse().ok()?,
            height: parts[3].trim().parse().ok()?,
        })
    }
    
    fn ask_yes_no(&self, prompt: &str, default: bool) -> HandsResult<bool> {
        let default_str = if default { "E/n" } else { "e/N" };
        print!("  {} [{}]: ", prompt, default_str);
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        let input = input.trim().to_lowercase();
        
        if input.is_empty() {
            return Ok(default);
        }
        
        Ok(input == "e" || input == "evet" || input == "y" || input == "yes")
    }
    
    fn ask_number(&self, prompt: &str, default: u64, min: u64, max: u64) -> HandsResult<u64> {
        print!("  {} [{}]: ", prompt, default);
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        let input = input.trim();
        
        if input.is_empty() {
            return Ok(default);
        }
        
        let num: u64 = input.parse().unwrap_or(default);
        Ok(num.clamp(min, max))
    }
    
    fn ask_string(&self, prompt: &str) -> HandsResult<String> {
        print!("  {}: ", prompt);
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        Ok(input.trim().to_string())
    }
    
    fn ask_permission(&self, name: &str, desc: &str, default: bool) -> HandsResult<bool> {
        println!("  {}", name);
        println!("    {}", desc);
        self.ask_yes_no("    İzin verilsin mi?", default)
    }
    
    // ═══════════════════════════════════════════════════════════════════════════
    //  GÖRÜNÜM
    // ═══════════════════════════════════════════════════════════════════════════
    
    fn print_banner(&self) {
        println!();
        println!("  ╔════════════════════════════════════════════════════════════════╗");
        println!("  ║                                                                ║");
        println!("  ║        🖐 OASIS HANDS SETUP WIZARD                             ║");
        println!("  ║                                                                ║");
        println!("  ║     Masaüstü Kontrolü için Kurulum Sihirbazı                  ║");
        println!("  ║                                                                ║");
        println!("  ╚════════════════════════════════════════════════════════════════╝");
        println!();
    }
    
    fn print_section(&self, title: &str) {
        println!();
        println!("  ═════════════════════════════════════════════════════════════════");
        println!("    {}", title);
        println!("  ═════════════════════════════════════════════════════════════════");
        println!();
    }
    
    fn print_step(&self, step: usize, total: usize, title: &str) {
        println!();
        println!("  ┌─────────────────────────────────────────────────────────────┐");
        println!("  │  ADIM {}/{}: {:<49}│", step, total, title);
        println!("  └─────────────────────────────────────────────────────────────┘");
        println!();
    }
    
    fn print_progress(&self, msg: &str) {
        print!("  ⏳ {}...", msg);
        io::stdout().flush().ok();
    }
    
    fn print_success(&self, msg: &str) {
        println!();
        println!("  ✅ {}", msg);
    }
    
    fn print_warning(&self, msg: &str) {
        println!();
        println!("  ⚠️  {}", msg);
    }
    
    fn print_error(&self, msg: &str) {
        println!();
        println!("  ❌ {}", msg);
    }
    
    fn print_summary_box(&self, data: &SummaryData) {
        println!("  ╔════════════════════════════════════════════════════════╗");
        println!("  ║                    PLATFORM                            ║");
        println!("  ╠════════════════════════════════════════════════════════╣");
        println!("  ║  İşletim Sistemi: {:<36} ║", data.platform);
        println!("  ╚════════════════════════════════════════════════════════╝");
        println!();
        
        println!("  ╔════════════════════════════════════════════════════════╗");
        println!("  ║                    İZİNLER                             ║");
        println!("  ╠════════════════════════════════════════════════════════╣");
        println!("  ║  📸 Ekran Yakalama:    {:>33} ║", if data.permissions.screen { "✅" } else { "❌" });
        println!("  ║  🖱️  Fare Kontrolü:    {:>33} ║", if data.permissions.mouse { "✅" } else { "❌" });
        println!("  ║  ⌨️  Klavye Kontrolü:  {:>33} ║", if data.permissions.keyboard { "✅" } else { "❌" });
        println!("  ║  🪟 Pencere Yönetimi: {:>33} ║", if data.permissions.window { "✅" } else { "❌" });
        println!("  ║  📁 Dosya Sistemi:    {:>33} ║", if data.permissions.filesystem { "✅" } else { "❌" });
        println!("  ║  ⚡ Process Başlatma: {:>33} ║", if data.permissions.process { "✅" } else { "❌" });
        println!("  ╚════════════════════════════════════════════════════════╝");
        println!();
        
        println!("  ╔════════════════════════════════════════════════════════╗");
        println!("  ║                    GÜVENLİK                            ║");
        println!("  ╠════════════════════════════════════════════════════════╣");
        println!("  ║  Mod:              {:>36} ║", data.security.mode);
        println!("  ║  Onay Gerekli:     {:>36} ║", if data.security.require_approval { "Evet" } else { "Hayır" });
        println!("  ║  Max Aksiyon/dk:   {:>36} ║", data.security.max_actions);
        println!("  ║  Acil Durdur:      {:>36} ║", if data.security.emergency_stop { "Aktif" } else { "Pasif" });
        println!("  ╚════════════════════════════════════════════════════════╝");
        println!();
        
        println!("  ╔════════════════════════════════════════════════════════╗");
        println!("  ║                 İNSAN BENZERLİK                        ║");
        println!("  ╠════════════════════════════════════════════════════════╣");
        println!("  ║  🖱️  FARE HAREKETİ                                     ║");
        println!("  ║    Aktif:          {:>35} ║", if data.human.mouse { "Evet" } else { "Hayır" });
        println!("  ║    Pattern:        {:>35} ║", data.human.mouse_pattern);
        println!("  ║    Bezier Kalite:  {:>35} ║", data.human.bezier_quality);
        println!("  ║    Titreşim:       {:>35} ║", if data.human.tremor { format!("Evet ({}%)", data.human.tremor_intensity) } else { "Hayır".into() });
        println!("  ╠────────────────────────────────────────────────────────╣");
        println!("  ║  ⌨️  KLAVYE YAZMA                                      ║");
        println!("  ║    Aktif:          {:>35} ║", if data.human.typing { "Evet" } else { "Hayır" });
        println!("  ║    Profil:         {:>35} ║", data.human.typing_profile);
        println!("  ║    WPM:            {:>35} ║", data.human.wpm);
        println!("  ║    Hata Oranı:     {:>34}% ║", data.human.error_rate);
        println!("  ╠────────────────────────────────────────────────────────╣");
        println!("  ║  🧠 DAVRANIŞ MODELİ                                   ║");
        println!("  ║    RNN-LSTM:       {:>35} ║", if data.human.use_rnn_model { "Aktif" } else { "Pasif" });
        println!("  ║    Best-of-N:      {:>35} ║", data.human.best_of_n);
        println!("  ║    Keşif Oranı:    {:>34}% ║", data.human.exploration_rate);
        println!("  ╠────────────────────────────────────────────────────────╣");
        println!("  ║  👁️  DİKKAT VE YORGUNLUK                              ║");
        println!("  ║    Odak Süresi:    {:>33} sn ║", data.human.attention_span_sec);
        println!("  ║    Dikkat Dağılma: {:>34}% ║", data.human.distraction_rate);
        println!("  ║    Yorgunluk:      {:>34}%/sa ║", data.human.fatigue_rate);
        println!("  ╠────────────────────────────────────────────────────────╣");
        println!("  ║  🤔 KARAR VERME                                      ║");
        println!("  ║    Gecikme:        {:>28}-{} ms ║", data.human.decision_delay_min_ms, data.human.decision_delay_max_ms);
        println!("  ║    Tereddüt:       {:>34}% ║", data.human.hesitation_rate);
        println!("  ╠────────────────────────────────────────────────────────╣");
        println!("  ║  ✋ FİZİKSEL                                          ║");
        println!("  ║    El Tercihi:     {:>35} ║", match data.human.hand_preference.as_str() {
            "right" => "Sağ El",
            "left" => "Sol El",
            "both" => "Her İki El",
            _ => "Sağ El",
        });
        println!("  ╠────────────────────────────────────────────────────────╣");
        println!("  ║  📊 GENEL BENZERLİK: {:>33}% ║", data.human.level);
        println!("  ╚════════════════════════════════════════════════════════╝");
        println!();
        
        if data.forbidden.regions > 0 || data.forbidden.apps > 0 {
            println!("  ╔════════════════════════════════════════════════════════╗");
            println!("  ║                  YASAKLI ALANLAR                       ║");
            println!("  ╠════════════════════════════════════════════════════════╣");
            println!("  ║  Yasaklı Bölge:   {:>36} ║", data.forbidden.regions);
            println!("  ║  Yasaklı Uygulama:{:>36} ║", data.forbidden.apps);
            println!("  ╚════════════════════════════════════════════════════════╝");
            println!();
        }
        
        println!("  ╔════════════════════════════════════════════════════════╗");
        println!("  ║                  ONAY SİSTEMİ                          ║");
        println!("  ╠════════════════════════════════════════════════════════╣");
        println!("  ║  Onayları Hatırla:    {:>32} ║", if data.approval.remember { "Evet" } else { "Hayır" });
        println!("  ║  Onay Timeout:         {:>32} ║", if data.approval.timeout > 0 { format!("{} sn", data.approval.timeout) } else { "Sınırsız".into() });
        println!("  ║  Güvenli Oto-Onay:     {:>32} ║", if data.approval.auto_safe { "Evet" } else { "Hayır" });
        println!("  ╚════════════════════════════════════════════════════════╝");
        println!();
        
        println!("  📁 Profil: {}", data.profile);
    }
}

impl Default for SetupWizard {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| Self {
            config: None,
            permissions: PermissionManager::new(),
            profiles: ProfileManager::new(),
            tester: SystemTester::new(),
            approvals: ApprovalManager::new(),
            term_width: 80,
        })
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  ÖZET VERİ YAPILARI
// ═══════════════════════════════════════════════════════════════════════════════

struct SummaryData<'a> {
    platform: &'a str,
    permissions: &'a PermissionSummary,
    security: SecuritySummary<'a>,
    human: &'a HumanSummary,
    forbidden: &'a ForbiddenSummary,
    approval: &'a ApprovalSummary,
    profile: &'a str,
}

struct PermissionSummary {
    screen: bool,
    mouse: bool,
    keyboard: bool,
    window: bool,
    filesystem: bool,
    process: bool,
}

struct SecuritySummary<'a> {
    mode: &'a str,
    require_approval: bool,
    max_actions: u64,
    emergency_stop: bool,
}

struct HumanSummary {
    // Mouse
    mouse: bool,
    mouse_pattern: String,
    bezier_quality: u32,
    tremor: bool,
    tremor_intensity: u32,
    // Keyboard
    typing: bool,
    typing_profile: String,
    wpm: u32,
    errors: bool,
    error_rate: u32,
    // Behavior
    use_rnn_model: bool,
    best_of_n: u32,
    exploration_rate: u32,
    // Attention
    simulate_attention: bool,
    attention_span_sec: u32,
    distraction_rate: u32,
    // Fatigue
    simulate_fatigue: bool,
    fatigue_rate: u32,
    // Decision
    decision_delay_min_ms: u32,
    decision_delay_max_ms: u32,
    hesitation_rate: u32,
    // Physical
    hand_preference: String,
    // General
    level: u32,
}

struct ForbiddenSummary {
    regions: usize,
    apps: usize,
}

struct ApprovalSummary {
    remember: bool,
    timeout: u64,
    auto_safe: bool,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  CLI FONKSİYONLARI
// ═══════════════════════════════════════════════════════════════════════════════

/// CLI'den çağrılacak ana fonksiyon
pub async fn run_setup(mode: SetupMode) -> HandsResult<WizardResult> {
    let mut wizard = SetupWizard::new()?;
    wizard.run(mode).await
}

/// Config'i göster
pub fn show_config() -> HandsResult<()> {
    match SetupConfig::load() {
        Ok(config) => {
            println!();
            println!("  📁 Config: {}", config_path().display());
            println!();
            println!("  {:?}", config);
            println!();
            Ok(())
        }
        Err(_) => {
            println!();
            println!("  ❌ Config dosyası bulunamadı.");
            println!("     Önce kurulum yapın: oasis-hands setup");
            println!();
            Err(HandsError::ConfigNotFound)
        }
    }
}

/// Config'i sıfırla
pub fn reset_config() -> HandsResult<()> {
    let path = config_path();
    if path.exists() {
        std::fs::remove_file(&path)?;
        println!();
        println!("  ✅ Config silindi: {}", path.display());
        println!("     Yeniden kurulum için: oasis-hands setup");
        println!();
    }
    Ok(())
}
