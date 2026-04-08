//! ─── OPERATION MODES ───
//!
//! Çalışma modları ve davranışları

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::{ModeError, ModeResult, ModeType};

/// İşletim modu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationMode {
    /// Mod tipi
    pub mode_type: ModeType,
    /// Ad
    pub name: String,
    /// Açıklama
    pub description: String,
    /// Davranış
    pub behavior: ModeBehavior,
    /// Ayarlar
    pub settings: ModeSettings,
    /// İzin verilen geçişler
    pub allowed_transitions: Vec<ModeType>,
}

/// Mod davranışı
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModeBehavior {
    /// Maksimum döngü sayısı
    pub max_iterations: u32,
    /// Zaman aşımı (saniye)
    pub timeout_secs: u64,
    /// Otomatik devam
    pub auto_continue: bool,
    /// İnsan onayı gerekli mi?
    pub require_approval: bool,
    /// Tool kullanımı
    pub tool_usage: ToolUsagePolicy,
    /// Hata davranışı
    pub error_behavior: ErrorBehavior,
}

impl Default for ModeBehavior {
    fn default() -> Self {
        Self {
            max_iterations: 50,
            timeout_secs: 300,
            auto_continue: true,
            require_approval: false,
            tool_usage: ToolUsagePolicy::default(),
            error_behavior: ErrorBehavior::default(),
        }
    }
}

/// Tool kullanım politikası
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolUsagePolicy {
    /// İzin verilen tool'lar
    pub allowed_tools: Vec<String>,
    /// Yasaklı tool'lar
    pub denied_tools: Vec<String>,
    /// Maksimum paralel tool
    pub max_parallel: usize,
    /// Onay gerekli tool'lar
    pub approval_required: Vec<String>,
}

impl Default for ToolUsagePolicy {
    fn default() -> Self {
        Self {
            allowed_tools: vec!["*".into()],
            denied_tools: vec![],
            max_parallel: 5,
            approval_required: vec!["bash".into(), "write".into()],
        }
    }
}

/// Hata davranışı
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorBehavior {
    /// Yeniden deneme sayısı
    pub max_retries: u32,
    /// Geri çekilme stratejisi
    pub backoff_strategy: BackoffStrategy,
    /// Hata loglama
    pub log_errors: bool,
    /// Kritik hatalarda dur
    pub stop_on_critical: bool,
}

impl Default for ErrorBehavior {
    fn default() -> Self {
        Self {
            max_retries: 3,
            backoff_strategy: BackoffStrategy::Exponential,
            log_errors: true,
            stop_on_critical: true,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum BackoffStrategy {
    None,
    Linear,
    Exponential,
}

/// Mod ayarları
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModeSettings {
    /// Sistem promptu eklemesi
    pub system_prompt_addon: Option<String>,
    /// Temperature
    pub temperature: f32,
    /// Özel parametreler
    pub custom_params: HashMap<String, serde_json::Value>,
}

impl Default for ModeSettings {
    fn default() -> Self {
        Self {
            system_prompt_addon: None,
            temperature: 0.7,
            custom_params: HashMap::new(),
        }
    }
}

/// Mod kayıt defteri
#[derive(Debug, Clone, Default)]
pub struct ModeRegistry {
    modes: HashMap<ModeType, OperationMode>,
}

impl ModeRegistry {
    pub fn new() -> Self {
        let mut modes = HashMap::new();
        
        // ReAct Modu
        modes.insert(ModeType::ReAct, OperationMode {
            mode_type: ModeType::ReAct,
            name: "ReAct Döngüsü".into(),
            description: "Standart düşün-eylem-gözlem döngüsü".into(),
            behavior: ModeBehavior {
                max_iterations: 50,
                timeout_secs: 300,
                auto_continue: true,
                require_approval: false,
                tool_usage: ToolUsagePolicy {
                    allowed_tools: vec!["*".into()],
                    denied_tools: vec![],
                    max_parallel: 5,
                    approval_required: vec!["bash".into()],
                },
                error_behavior: ErrorBehavior::default(),
            },
            settings: ModeSettings::default(),
            allowed_transitions: vec![
                ModeType::Plan,
                ModeType::Research,
                ModeType::Development,
                ModeType::Interactive,
            ],
        });
        
        // Plan Modu
        modes.insert(ModeType::Plan, OperationMode {
            mode_type: ModeType::Plan,
            name: "Planlama Modu".into(),
            description: "Yazma işlemleri engelli, sadece planlama".into(),
            behavior: ModeBehavior {
                max_iterations: 20,
                timeout_secs: 600,
                auto_continue: false,
                require_approval: false,
                tool_usage: ToolUsagePolicy {
                    allowed_tools: vec!["read".into(), "search".into(), "ls".into()],
                    denied_tools: vec!["write".into(), "bash".into(), "edit".into()],
                    max_parallel: 3,
                    approval_required: vec![],
                },
                error_behavior: ErrorBehavior::default(),
            },
            settings: ModeSettings {
                system_prompt_addon: Some(
                    "PLANLAMA MODU: Sadece plan yap, kod yazma. Kullanıcıya adım adım plan sun.".into()
                ),
                temperature: 0.5,
                custom_params: HashMap::new(),
            },
            allowed_transitions: vec![
                ModeType::ReAct,
                ModeType::Development,
            ],
        });
        
        // Research Modu
        modes.insert(ModeType::Research, OperationMode {
            mode_type: ModeType::Research,
            name: "Araştırma Modu".into(),
            description: "Derin araştırma ve bilgi toplama".into(),
            behavior: ModeBehavior {
                max_iterations: 100,
                timeout_secs: 1800,
                auto_continue: true,
                require_approval: false,
                tool_usage: ToolUsagePolicy {
                    allowed_tools: vec![
                        "read".into(),
                        "search".into(),
                        "web_search".into(),
                        "browser".into(),
                        "fetch".into(),
                    ],
                    denied_tools: vec!["write".into(), "bash".into()],
                    max_parallel: 10,
                    approval_required: vec![],
                },
                error_behavior: ErrorBehavior::default(),
            },
            settings: ModeSettings {
                system_prompt_addon: Some(
                    "ARAŞTIRMA MODU: Derinlemesine araştırma yap, kaynakları doğrula, özetle.".into()
                ),
                temperature: 0.3,
                custom_params: HashMap::new(),
            },
            allowed_transitions: vec![
                ModeType::ReAct,
                ModeType::Development,
                ModeType::Interactive,
            ],
        });
        
        // Development Modu
        modes.insert(ModeType::Development, OperationMode {
            mode_type: ModeType::Development,
            name: "Geliştirme Modu".into(),
            description: "Kod yazma ve dosya düzenleme".into(),
            behavior: ModeBehavior {
                max_iterations: 100,
                timeout_secs: 3600,
                auto_continue: true,
                require_approval: true,
                tool_usage: ToolUsagePolicy {
                    allowed_tools: vec!["*".into()],
                    denied_tools: vec![],
                    max_parallel: 3,
                    approval_required: vec!["write".into(), "bash".into()],
                },
                error_behavior: ErrorBehavior {
                    max_retries: 3,
                    backoff_strategy: BackoffStrategy::Exponential,
                    log_errors: true,
                    stop_on_critical: false,
                },
            },
            settings: ModeSettings {
                system_prompt_addon: Some(
                    "GELİŞTİRME MODU: Kod yaz, test et, refactor yap. Kaliteli kod üret.".into()
                ),
                temperature: 0.5,
                custom_params: HashMap::new(),
            },
            allowed_transitions: vec![
                ModeType::ReAct,
                ModeType::Plan,
                ModeType::Interactive,
            ],
        });
        
        // Interactive Modu
        modes.insert(ModeType::Interactive, OperationMode {
            mode_type: ModeType::Interactive,
            name: "İnteraktif Mod".into(),
            description: "Kullanıcı ile sohbet".into(),
            behavior: ModeBehavior {
                max_iterations: 1,
                timeout_secs: 60,
                auto_continue: false,
                require_approval: false,
                tool_usage: ToolUsagePolicy {
                    allowed_tools: vec!["read".into()],
                    denied_tools: vec!["write".into(), "bash".into()],
                    max_parallel: 1,
                    approval_required: vec![],
                },
                error_behavior: ErrorBehavior::default(),
            },
            settings: ModeSettings {
                system_prompt_addon: Some(
                    "İNTERAKTİF MOD: Kullanıcıyla sohbet et, soruları yanıtla.".into()
                ),
                temperature: 0.8,
                custom_params: HashMap::new(),
            },
            allowed_transitions: vec![
                ModeType::ReAct,
                ModeType::Plan,
                ModeType::Development,
                ModeType::Research,
            ],
        });
        
        // Autonomous Modu
        modes.insert(ModeType::Autonomous, OperationMode {
            mode_type: ModeType::Autonomous,
            name: "Otonom Mod".into(),
            description: "Tam otonom çalışma, kullanıcı müdahalesi yok".into(),
            behavior: ModeBehavior {
                max_iterations: 1000,
                timeout_secs: 86400, // 24 saat
                auto_continue: true,
                require_approval: false,
                tool_usage: ToolUsagePolicy {
                    allowed_tools: vec!["*".into()],
                    denied_tools: vec![],
                    max_parallel: 10,
                    approval_required: vec![],
                },
                error_behavior: ErrorBehavior {
                    max_retries: 5,
                    backoff_strategy: BackoffStrategy::Exponential,
                    log_errors: true,
                    stop_on_critical: false,
                },
            },
            settings: ModeSettings {
                system_prompt_addon: Some(
                    "OTONOM MOD: Tamamen özerk çalış. Hedefe ulaşmak için gerekli tüm adımları at.".into()
                ),
                temperature: 0.5,
                custom_params: HashMap::new(),
            },
            allowed_transitions: vec![
                ModeType::Interactive,
            ],
        });
        
        Self { modes }
    }
    
    pub fn get(&self, mode_type: ModeType) -> ModeResult<OperationMode> {
        self.modes.get(&mode_type)
            .cloned()
            .ok_or_else(|| ModeError::NotFound(format!("{:?}", mode_type)))
    }
    
    pub fn list(&self) -> Vec<OperationMode> {
        self.modes.values().cloned().collect()
    }
}
