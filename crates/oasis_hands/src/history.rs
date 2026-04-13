//! ═══════════════════════════════════════════════════════════════════════════════
//!  UNDO/REDO - GERİ ALMA / TEKRAR YAPMA SİSTEMİ
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Masaüstü kontrolü için geri alınabilir aksiyon yönetimi.
//! 
//! ═──────────────────────────────────────────────────────────────────────────────
//!  ÖZELLİKLER:
//!  ────────────────
//!  Action History   → Aksiyon geçmişi (VecDeque)
//!  State Snapshot   → Durum anlık görüntüsü
//!  Undo             → Son aksiyonu geri al
//!  Redo             → Geri alınanı tekrar yap
//!  Branch           → Alternatif dallar
//! ═──────────────────────────────────────────────────────────────────────────────

use crate::error::{HandsError, HandsResult};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::time::Instant;

// ───────────────────────────────────────────────────────────────────────────────
//  AKSİYON TÜRÜ
// ───────────────────────────────────────────────────────────────────────────────

/// Geri alınabilir aksiyon türü
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UndoableActionType {
    /// Mouse hareketi
    MouseMove,
    /// Mouse tıklama
    MouseClick,
    /// Mouse sürükle
    MouseDrag,
    /// Mouse scroll
    MouseScroll,
    /// Klavye tuş basma
    KeyPress,
    /// Klavye metin yazma
    TypeText,
    /// Klavye kısayol
    Shortcut,
    /// Ekran görüntüsü
    Screenshot,
    /// Pencere işlevi
    WindowAction,
    /// Özel
    Custom(String),
}

impl UndoableActionType {
    /// Açıklama
    pub fn description(&self) -> &str {
        match self {
            UndoableActionType::MouseMove => "Mouse hareketi",
            UndoableActionType::MouseClick => "Mouse tıklama",
            UndoableActionType::MouseDrag => "Mouse sürükle",
            UndoableActionType::MouseScroll => "Mouse scroll",
            UndoableActionType::KeyPress => "Tuş basma",
            UndoableActionType::TypeText => "Metin yazma",
            UndoableActionType::Shortcut => "Kısayol",
            UndoableActionType::Screenshot => "Ekran görüntüsü",
            UndoableActionType::WindowAction => "Pencere işlevi",
            UndoableActionType::Custom(s) => s,
        }
    }
    
    /// Geri alınabilir mi?
    pub fn is_undoable(&self) -> bool {
        matches!(
            self,
            UndoableActionType::MouseMove
                | UndoableActionType::MouseClick
                | UndoableActionType::MouseDrag
                | UndoableActionType::KeyPress
                | UndoableActionType::TypeText
        )
    }
    
    /// İkon
    pub fn icon(&self) -> &str {
        match self {
            UndoableActionType::MouseMove => "🖱️",
            UndoableActionType::MouseClick => "👆",
            UndoableActionType::MouseDrag => "✋",
            UndoableActionType::MouseScroll => "📜",
            UndoableActionType::KeyPress => "⌨️",
            UndoableActionType::TypeText => "📝",
            UndoableActionType::Shortcut => "⚡",
            UndoableActionType::Screenshot => "📷",
            UndoableActionType::WindowAction => "🪟",
            UndoableActionType::Custom(_) => "🔹",
        }
    }
}

// ───────────────────────────────────────────────────────────────────────────────
//  DURUM SNAPSHOT
// ───────────────────────────────────────────────────────────────────────────────

/// Durum anlık görüntüsü
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateSnapshot {
    /// Mouse X pozisyonu
    pub mouse_x: f64,
    /// Mouse Y pozisyonu
    pub mouse_y: f64,
    /// Aktif pencere (pencere başlığı veya ID)
    pub active_window: Option<String>,
    /// Pencere boyutu (width, height)
    pub window_size: Option<(u32, u32)>,
    /// Ekran boyutu
    pub screen_size: (u32, u32),
    /// Clipboard içeriği (opsiyonel)
    pub clipboard: Option<String>,
    /// Özel veri (JSON)
    pub custom_data: Option<String>,
    /// Zaman damgası
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl StateSnapshot {
    /// Yeni snapshot oluştur
    pub fn new(mouse_x: f64, mouse_y: f64, screen_width: u32, screen_height: u32) -> Self {
        Self {
            mouse_x,
            mouse_y,
            active_window: None,
            window_size: None,
            screen_size: (screen_width, screen_height),
            clipboard: None,
            custom_data: None,
            timestamp: chrono::Utc::now(),
        }
    }
    
    /// Aktif pencere ile
    pub fn with_active_window(mut self, window: &str) -> Self {
        self.active_window = Some(window.to_string());
        self
    }
    
    /// Pencere boyutu ile
    pub fn with_window_size(mut self, width: u32, height: u32) -> Self {
        self.window_size = Some((width, height));
        self
    }
    
    /// Clipboard ile
    pub fn with_clipboard(mut self, content: &str) -> Self {
        self.clipboard = Some(content.to_string());
        self
    }
    
    /// Özel veri ile
    pub fn with_custom_data(mut self, data: &str) -> Self {
        self.custom_data = Some(data.to_string());
        self
    }
}

// ───────────────────────────────────────────────────────────────────────────────
//  TARİHSEL AKSİYON
// ───────────────────────────────────────────────────────────────────────────────

/// Tarihsel aksiyon kaydı
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoricalAction {
    /// Aksiyon ID
    pub id: u64,
    /// Aksiyon türü
    pub action_type: UndoableActionType,
    /// Aksiyon parametreleri (JSON)
    pub params: String,
    /// Aksiyon öncesi durum
    pub state_before: StateSnapshot,
    /// Aksiyon sonrası durum
    pub state_after: Option<StateSnapshot>,
    /// Zaman damgası
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Etiketler
    pub tags: Vec<String>,
    /// Geri alındı mı?
    pub undone: bool,
    /// Açıklama
    pub description: String,
}

impl HistoricalAction {
    /// Yeni tarihsel aksiyon
    pub fn new(
        id: u64,
        action_type: UndoableActionType,
        params: &str,
        state_before: StateSnapshot,
        description: &str,
    ) -> Self {
        Self {
            id,
            action_type,
            params: params.to_string(),
            state_before,
            state_after: None,
            timestamp: chrono::Utc::now(),
            tags: Vec::new(),
            undone: false,
            description: description.to_string(),
        }
    }
    
    /// Etiket ekle
    pub fn add_tag(&mut self, tag: &str) {
        if !self.tags.contains(&tag.to_string()) {
            self.tags.push(tag.to_string());
        }
    }
    
    /// Sonuç durumunu ayarla
    pub fn set_state_after(&mut self, state: StateSnapshot) {
        self.state_after = Some(state);
    }
}

// ───────────────────────────────────────────────────────────────────────────────
//  AKSİYON GEÇMİŞİ YAPILANDIRMASI
// ───────────────────────────────────────────────────────────────────────────────

/// Aksiyon geçmişi yapılandırması
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryConfig {
    /// Maksimum kayıt sayısı
    pub max_history: usize,
    /// Otomatik kaydetme
    pub auto_save: bool,
    /// Kayıt dosyası
    pub save_file: Option<String>,
    /// Geri alınabilir maksimum adım
    pub max_undo_depth: usize,
    /// Snapshot detay seviyesi
    pub snapshot_detail: SnapshotDetail,
    /// Dal oluşturma etkin mi?
    pub enable_branching: bool,
}

/// Snapshot detay seviyesi
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SnapshotDetail {
    /// Minimum (sadece mouse pozisyonu)
    Minimal,
    /// Normal (mouse + pencere)
    Normal,
    /// Tam (tüm veriler)
    Full,
}

impl Default for HistoryConfig {
    fn default() -> Self {
        Self {
            max_history: 100,
            auto_save: false,
            save_file: None,
            max_undo_depth: 50,
            snapshot_detail: SnapshotDetail::Normal,
            enable_branching: true,
        }
    }
}

impl HistoryConfig {
    /// Yeni yapılandırma
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Büyük geçmiş
    pub fn large() -> Self {
        Self {
            max_history: 500,
            max_undo_depth: 200,
            ..Default::default()
        }
    }
    
    /// Minimal
    pub fn minimal() -> Self {
        Self {
            max_history: 20,
            max_undo_depth: 10,
            snapshot_detail: SnapshotDetail::Minimal,
            ..Default::default()
        }
    }
}

// ───────────────────────────────────────────────────────────────────────────────
//  UNDO/REDO SONUCU
// ───────────────────────────────────────────────────────────────────────────────

/// Undo/Redo işlem sonucu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UndoRedoResult {
    /// İşlem başarılı mı?
    pub success: bool,
    /// İşlem türü
    pub operation: UndoRedoOperation,
    /// İlgili aksiyon ID
    pub action_id: u64,
    /// Mesaj
    pub message: String,
    /// Önceki durum
    pub previous_state: Option<StateSnapshot>,
    /// Yeni durum
    pub new_state: Option<StateSnapshot>,
}

/// Undo/Redo işlem türü
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UndoRedoOperation {
    Undo,
    Redo,
    Jump,
    Branch,
}

impl std::fmt::Display for UndoRedoOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UndoRedoOperation::Undo => write!(f, "Geri Al"),
            UndoRedoOperation::Redo => write!(f, "Tekrar Yap"),
            UndoRedoOperation::Jump => write!(f, "Atla"),
            UndoRedoOperation::Branch => write!(f, "Dal"),
        }
    }
}

// ───────────────────────────────────────────────────────────────────────────────
//  GEÇMİŞ DALI
// ───────────────────────────────────────────────────────────────────────────────

/// Geçmiş dalı
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryBranch {
    /// Dal ID
    pub id: u64,
    /// Dal adı
    pub name: String,
    /// Oluşturulma zamanı
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Aksiyonlar
    pub actions: Vec<HistoricalAction>,
    /// Aktif mi?
    pub active: bool,
}

impl HistoryBranch {
    pub fn new(id: u64, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            created_at: chrono::Utc::now(),
            actions: Vec::new(),
            active: false,
        }
    }
}

// ───────────────────────────────────────────────────────────────────────────────
//  AKSİYON GEÇMİŞİ
// ───────────────────────────────────────────────────────────────────────────────

/// Aksiyon geçmişi yöneticisi
pub struct ActionHistory {
    /// Yapılandırma
    config: HistoryConfig,
    /// Ana geçmiş
    history: VecDeque<HistoricalAction>,
    /// Geri alınan aksiyonlar (redo için)
    undone_stack: VecDeque<HistoricalAction>,
    /// Dallar
    branches: Vec<HistoryBranch>,
    /// Aktif dal indeksi (None = ana dal)
    active_branch: Option<usize>,
    /// ID sayacı
    next_id: u64,
    /// İstatistikler
    stats: HistoryStats,
}

/// Geçmiş istatistikleri
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HistoryStats {
    pub total_actions: u64,
    pub undo_count: u64,
    pub redo_count: u64,
    pub branches_count: usize,
    pub mouse_actions: u64,
    pub keyboard_actions: u64,
    pub other_actions: u64,
}

impl ActionHistory {
    /// Yeni geçmiş oluştur
    pub fn new(config: HistoryConfig) -> Self {
        Self {
            config,
            history: VecDeque::with_capacity(100),
            undone_stack: VecDeque::with_capacity(50),
            branches: Vec::new(),
            active_branch: None,
            next_id: 1,
            stats: HistoryStats::default(),
        }
    }
    
    /// Varsayılan yapılandırma
    pub fn default_config() -> Self {
        Self::new(HistoryConfig::default())
    }
    
    /// Büyük geçmiş
    pub fn large() -> Self {
        Self::new(HistoryConfig::large())
    }
    
    // ─── AKSİYON KAYIT ───
    
    /// Aksiyon kaydet
    pub fn record(
        &mut self,
        action_type: UndoableActionType,
        params: &str,
        state_before: StateSnapshot,
        description: &str,
    ) -> HandsResult<u64> {
        // Yeni ID
        let id = self.next_id;
        self.next_id += 1;
        
        // Aksiyon oluştur
        let action = HistoricalAction::new(id, action_type.clone(), params, state_before, description);
        
        // Geri alınan aksiyonları temizle (yeni dal)
        self.undone_stack.clear();
        
        // Geçmişe ekle
        if self.history.len() >= self.config.max_history {
            self.history.pop_front();
        }
        self.history.push_back(action);
        
        // İstatistikleri güncelle
        self.stats.total_actions += 1;
        match action_type {
            UndoableActionType::MouseMove
            | UndoableActionType::MouseClick
            | UndoableActionType::MouseDrag
            | UndoableActionType::MouseScroll => {
                self.stats.mouse_actions += 1;
            }
            UndoableActionType::KeyPress
            | UndoableActionType::TypeText
            | UndoableActionType::Shortcut => {
                self.stats.keyboard_actions += 1;
            }
            _ => {
                self.stats.other_actions += 1;
            }
        }
        
        log::debug!(
            "📝 HISTORY: Kaydedildi #{} - {} {}",
            id,
            action_type.icon(),
            description
        );
        
        Ok(id)
    }
    
    /// Aksiyon tamamla (sonuç durumunu ekle)
    pub fn complete_action(&mut self, action_id: u64, state_after: StateSnapshot) -> HandsResult<()> {
        if let Some(action) = self.history.iter_mut().rev().find(|a| a.id == action_id) {
            action.set_state_after(state_after);
            Ok(())
        } else {
            Err(HandsError::HistoryError(format!("Aksiyon bulunamadı: {}", action_id)))
        }
    }
    
    // ─── UNDO/REDO ───
    
    /// Son aksiyonu geri al
    pub fn undo(&mut self) -> HandsResult<UndoRedoResult> {
        if let Some(mut action) = self.history.pop_back() {
            action.undone = true;
            
            // Undo stack'e ekle
            if self.undone_stack.len() >= self.config.max_undo_depth {
                self.undone_stack.pop_front();
            }
            self.undone_stack.push_back(action.clone());
            
            // İstatistik
            self.stats.undo_count += 1;
            
            log::info!("↩️ UNDO: #{} - {}", action.id, action.description);
            
            Ok(UndoRedoResult {
                success: true,
                operation: UndoRedoOperation::Undo,
                action_id: action.id,
                message: format!("Geri alındı: {}", action.description),
                previous_state: Some(action.state_after.clone().unwrap_or_else(|| {
                    action.state_before.clone()
                })),
                new_state: Some(action.state_before.clone()),
            })
        } else {
            Err(HandsError::HistoryError("Geri alınacak aksiyon yok".to_string()))
        }
    }
    
    /// Son geri alınanı tekrar yap
    pub fn redo(&mut self) -> HandsResult<UndoRedoResult> {
        if let Some(mut action) = self.undone_stack.pop_back() {
            action.undone = false;
            
            // Geçmişe geri ekle
            if self.history.len() >= self.config.max_history {
                self.history.pop_front();
            }
            self.history.push_back(action.clone());
            
            // İstatistik
            self.stats.redo_count += 1;
            
            log::info!("↪️ REDO: #{} - {}", action.id, action.description);
            
            Ok(UndoRedoResult {
                success: true,
                operation: UndoRedoOperation::Redo,
                action_id: action.id,
                message: format!("Tekrar yapıldı: {}", action.description),
                previous_state: Some(action.state_before.clone()),
                new_state: action.state_after.clone(),
            })
        } else {
            Err(HandsError::HistoryError("Tekrar yapılacak aksiyon yok".to_string()))
        }
    }
    
    /// Belirli bir aksiyona atla
    pub fn jump_to(&mut self, action_id: u64) -> HandsResult<UndoRedoResult> {
        // Aksiyonu bul
        let position = self.history.iter().position(|a| a.id == action_id);
        
        if let Some(pos) = position {
            // Bu aksiyona kadar olanları undo stack'e taşı
            let count = self.history.len() - pos - 1;
            
            for _ in 0..count {
                if let Some(mut action) = self.history.pop_back() {
                    action.undone = true;
                    self.undone_stack.push_back(action);
                }
            }
            
            let action = self.history.back().unwrap();
            
            Ok(UndoRedoResult {
                success: true,
                operation: UndoRedoOperation::Jump,
                action_id: action.id,
                message: format!("Atlandı: {}", action.description),
                previous_state: None,
                new_state: Some(action.state_after.clone().unwrap_or_else(|| action.state_before.clone())),
            })
        } else {
            // Undo stack'te ara
            let undo_pos = self.undone_stack.iter().position(|a| a.id == action_id);
            
            if let Some(pos) = undo_pos {
                // Bu aksiyona kadar olanları history'ye taşı
                let count = pos + 1;
                
                for _ in 0..count {
                    if let Some(mut action) = self.undone_stack.pop_back() {
                        action.undone = false;
                        self.history.push_back(action);
                    }
                }
                
                let action = self.history.back().unwrap();
                
                Ok(UndoRedoResult {
                    success: true,
                    operation: UndoRedoOperation::Jump,
                    action_id: action.id,
                    message: format!("Atlandı: {}", action.description),
                    previous_state: None,
                    new_state: Some(action.state_after.clone().unwrap_or_else(|| action.state_before.clone())),
                })
            } else {
                Err(HandsError::HistoryError(format!("Aksiyon bulunamadı: {}", action_id)))
            }
        }
    }
    
    // ─── DAL YÖNETİMİ ───
    
    /// Yeni dal oluştur
    pub fn create_branch(&mut self, name: &str) -> HandsResult<u64> {
        if !self.config.enable_branching {
            return Err(HandsError::HistoryError("Dal oluşturma devre dışı".to_string()));
        }
        
        let id = self.next_id;
        self.next_id += 1;
        
        let mut branch = HistoryBranch::new(id, name);
        branch.actions = self.history.iter().cloned().collect();
        
        self.branches.push(branch);
        self.stats.branches_count = self.branches.len();
        
        log::info!("🌿 BRANCH: '{}' oluşturuldu (ID: {})", name, id);
        
        Ok(id)
    }
    
    /// Dala geç
    pub fn switch_branch(&mut self, branch_id: u64) -> HandsResult<()> {
        let pos = self.branches.iter().position(|b| b.id == branch_id);
        
        if let Some(pos) = pos {
            // Mevcut durumu kaydet
            if let Some(ref mut current) = self.active_branch {
                self.branches[*current].actions = self.history.iter().cloned().collect();
                self.branches[*current].active = false;
            }
            
            // Yeni dala geç
            self.history.clear();
            for action in &self.branches[pos].actions {
                self.history.push_back(action.clone());
            }
            self.branches[pos].active = true;
            self.active_branch = Some(pos);
            
            log::info!("🌿 BRANCH: '{}' dalına geçildi", self.branches[pos].name);
            
            Ok(())
        } else {
            Err(HandsError::HistoryError(format!("Dal bulunamadı: {}", branch_id)))
        }
    }
    
    /// Ana dala dön
    pub fn switch_to_main(&mut self) {
        if let Some(ref mut current) = self.active_branch {
            self.branches[*current].actions = self.history.iter().cloned().collect();
            self.branches[*current].active = false;
        }
        
        self.active_branch = None;
        log::info!("🌿 BRANCH: Ana dala dönüldü");
    }
    
    /// Dalları listele
    pub fn list_branches(&self) -> &[HistoryBranch] {
        &self.branches
    }
    
    /// Dal sil
    pub fn delete_branch(&mut self, branch_id: u64) -> HandsResult<()> {
        let pos = self.branches.iter().position(|b| b.id == branch_id);
        
        if let Some(pos) = pos {
            if self.active_branch == Some(pos) {
                return Err(HandsError::HistoryError("Aktif dal silinemez".to_string()));
            }
            
            self.branches.remove(pos);
            self.stats.branches_count = self.branches.len();
            
            // Aktif dal indeksini güncelle
            if let Some(ref mut current) = self.active_branch {
                if *current > pos {
                    *current -= 1;
                }
            }
            
            log::info!("🌿 BRANCH: Dal {} silindi", branch_id);
            
            Ok(())
        } else {
            Err(HandsError::HistoryError(format!("Dal bulunamadı: {}", branch_id)))
        }
    }
    
    // ─── RAPORLAMA ───
    
    /// Geçmişi getir
    pub fn get_history(&self) -> &VecDeque<HistoricalAction> {
        &self.history
    }
    
    /// Geri alınanları getir
    pub fn get_undone(&self) -> &VecDeque<HistoricalAction> {
        &self.undone_stack
    }
    
    /// Son N aksiyonu getir
    pub fn get_recent(&self, n: usize) -> Vec<&HistoricalAction> {
        self.history.iter().rev().take(n).collect()
    }
    
    /// Aksiyonu ID ile getir
    pub fn get_action(&self, id: u64) -> Option<&HistoricalAction> {
        self.history.iter().find(|a| a.id == id)
            .or_else(|| self.undone_stack.iter().find(|a| a.id == id))
    }
    
    /// İstatistikleri getir
    pub fn stats(&self) -> &HistoryStats {
        &self.stats
    }
    
    /// Geri alınabilir mi?
    pub fn can_undo(&self) -> bool {
        !self.history.is_empty()
    }
    
    /// Tekrar yapılabilir mi?
    pub fn can_redo(&self) -> bool {
        !self.undone_stack.is_empty()
    }
    
    /// Geçmiş boyutu
    pub fn size(&self) -> usize {
        self.history.len()
    }
    
    /// Tümünü temizle
    pub fn clear(&mut self) {
        self.history.clear();
        self.undone_stack.clear();
        log::info!("📝 HISTORY: Tüm geçmiş temizlendi");
    }
    
    /// Rapor oluştur
    pub fn report(&self) -> HistoryReport {
        HistoryReport {
            total_actions: self.stats.total_actions,
            history_size: self.history.len(),
            undo_stack_size: self.undone_stack.len(),
            branches_count: self.branches.len(),
            active_branch: self.active_branch.map(|i| self.branches[i].name.clone()),
            undo_count: self.stats.undo_count,
            redo_count: self.stats.redo_count,
            mouse_actions: self.stats.mouse_actions,
            keyboard_actions: self.stats.keyboard_actions,
            other_actions: self.stats.other_actions,
            can_undo: self.can_undo(),
            can_redo: self.can_redo(),
        }
    }
    
    /// Config
    pub fn config(&self) -> &HistoryConfig {
        &self.config
    }
}

/// Geçmiş raporu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryReport {
    pub total_actions: u64,
    pub history_size: usize,
    pub undo_stack_size: usize,
    pub branches_count: usize,
    pub active_branch: Option<String>,
    pub undo_count: u64,
    pub redo_count: u64,
    pub mouse_actions: u64,
    pub keyboard_actions: u64,
    pub other_actions: u64,
    pub can_undo: bool,
    pub can_redo: bool,
}

impl std::fmt::Display for HistoryReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "╔════════════════════════════════════════════╗")?;
        writeln!(f, "║         ACTION HISTORY RAPORU              ║")?;
        writeln!(f, "╠════════════════════════════════════════════╣")?;
        writeln!(f, "║ Toplam Aksiyon: {:<26} ║", self.total_actions)?;
        writeln!(f, "║ Geçmiş Boyutu: {:<26} ║", self.history_size)?;
        writeln!(f, "║ Undo Stack: {:<31} ║", self.undo_stack_size)?;
        writeln!(f, "╠════════════════════════════════════════════╣")?;
        writeln!(f, "║ Undo/Redo:                                   ║")?;
        writeln!(f, "║ ├─ Undo Sayısı: {:<27} ║", self.undo_count)?;
        writeln!(f, "║ ├─ Redo Sayısı: {:<27} ║", self.redo_count)?;
        writeln!(f, "║ ├─ Can Undo: {:<31} ║", if self.can_undo { "Evet" } else { "Hayır" })?;
        writeln!(f, "║ └─ Can Redo: {:<31} ║", if self.can_redo { "Evet" } else { "Hayır" })?;
        writeln!(f, "╠════════════════════════════════════════════╣")?;
        writeln!(f, "║ Aksiyon Dağılımı:                            ║")?;
        writeln!(f, "║ ├─ Mouse: {:<34} ║", self.mouse_actions)?;
        writeln!(f, "║ ├─ Klavye: {:<33} ║", self.keyboard_actions)?;
        writeln!(f, "║ └─ Diğer: {:<34} ║", self.other_actions)?;
        writeln!(f, "╠════════════════════════════════════════════╣")?;
        writeln!(f, "║ Dallar: {:<36} ║", self.branches_count)?;
        if let Some(ref branch) = self.active_branch {
            writeln!(f, "║ Aktif Dal: {:<33} ║", branch)?;
        }
        writeln!(f, "╚════════════════════════════════════════════╝")
    }
}

// ───────────────────────────────────────────────────────────────────────────────
//  TESTS
// ───────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_undoable_action_type_description() {
        assert_eq!(UndoableActionType::MouseMove.description(), "Mouse hareketi");
        assert_eq!(UndoableActionType::KeyPress.description(), "Tuş basma");
    }
    
    #[test]
    fn test_undoable_action_type_is_undoable() {
        assert!(UndoableActionType::MouseMove.is_undoable());
        assert!(UndoableActionType::TypeText.is_undoable());
        // Screenshot geri alınamaz
        assert!(!UndoableActionType::Screenshot.is_undoable());
    }
    
    #[test]
    fn test_undoable_action_type_icon() {
        assert_eq!(UndoableActionType::MouseMove.icon(), "🖱️");
        assert_eq!(UndoableActionType::KeyPress.icon(), "⌨️");
    }
    
    #[test]
    fn test_state_snapshot_creation() {
        let snapshot = StateSnapshot::new(100.0, 200.0, 1920, 1080)
            .with_active_window("Test Window")
            .with_window_size(800, 600);
        
        assert_eq!(snapshot.mouse_x, 100.0);
        assert_eq!(snapshot.mouse_y, 200.0);
        assert_eq!(snapshot.screen_size, (1920, 1080));
        assert_eq!(snapshot.active_window, Some("Test Window".to_string()));
        assert_eq!(snapshot.window_size, Some((800, 600)));
    }
    
    #[test]
    fn test_historical_action_creation() {
        let state = StateSnapshot::new(0.0, 0.0, 1920, 1080);
        let action = HistoricalAction::new(
            1,
            UndoableActionType::MouseClick,
            r#"{"x":100,"y":200}"#,
            state,
            "Tıklama"
        );
        
        assert_eq!(action.id, 1);
        assert_eq!(action.action_type, UndoableActionType::MouseClick);
        assert!(!action.undone);
    }
    
    #[test]
    fn test_historical_action_tags() {
        let state = StateSnapshot::new(0.0, 0.0, 1920, 1080);
        let mut action = HistoricalAction::new(
            1,
            UndoableActionType::MouseClick,
            "{}",
            state,
            "Test"
        );
        
        action.add_tag("important");
        action.add_tag("manual");
        action.add_tag("important"); // Duplicate
        
        assert_eq!(action.tags.len(), 2);
    }
    
    #[test]
    fn test_history_config_default() {
        let config = HistoryConfig::default();
        assert_eq!(config.max_history, 100);
        assert!(!config.auto_save);
    }
    
    #[test]
    fn test_history_config_presets() {
        let large = HistoryConfig::large();
        assert_eq!(large.max_history, 500);
        
        let minimal = HistoryConfig::minimal();
        assert_eq!(minimal.max_history, 20);
    }
    
    #[test]
    fn test_action_history_creation() {
        let history = ActionHistory::default_config();
        assert_eq!(history.size(), 0);
        assert!(!history.can_undo());
        assert!(!history.can_redo());
    }
    
    #[test]
    fn test_action_history_record() {
        let mut history = ActionHistory::default_config();
        let state = StateSnapshot::new(0.0, 0.0, 1920, 1080);
        
        let id = history.record(
            UndoableActionType::MouseClick,
            r#"{"x":100}"#,
            state,
            "Test click"
        ).unwrap();
        
        assert_eq!(id, 1);
        assert_eq!(history.size(), 1);
        assert!(history.can_undo());
        assert_eq!(history.stats().total_actions, 1);
    }
    
    #[test]
    fn test_action_history_undo() {
        let mut history = ActionHistory::default_config();
        let state = StateSnapshot::new(0.0, 0.0, 1920, 1080);
        
        history.record(UndoableActionType::MouseClick, "{}", state.clone(), "Test").unwrap();
        
        let result = history.undo().unwrap();
        assert!(result.success);
        assert_eq!(result.operation, UndoRedoOperation::Undo);
        assert!(!history.can_undo()); // Boşaldı
        assert!(history.can_redo()); // Redo var
    }
    
    #[test]
    fn test_action_history_redo() {
        let mut history = ActionHistory::default_config();
        let state = StateSnapshot::new(0.0, 0.0, 1920, 1080);
        
        history.record(UndoableActionType::MouseClick, "{}", state.clone(), "Test").unwrap();
        history.undo().unwrap();
        
        let result = history.redo().unwrap();
        assert!(result.success);
        assert_eq!(result.operation, UndoRedoOperation::Redo);
        assert!(history.can_undo());
        assert!(!history.can_redo());
    }
    
    #[test]
    fn test_action_history_undo_empty() {
        let mut history = ActionHistory::default_config();
        let result = history.undo();
        assert!(result.is_err());
    }
    
    #[test]
    fn test_action_history_redo_empty() {
        let mut history = ActionHistory::default_config();
        let result = history.redo();
        assert!(result.is_err());
    }
    
    #[test]
    fn test_action_history_multiple_actions() {
        let mut history = ActionHistory::default_config();
        let state = StateSnapshot::new(0.0, 0.0, 1920, 1080);
        
        history.record(UndoableActionType::MouseClick, "{}", state.clone(), "Click 1").unwrap();
        history.record(UndoableActionType::KeyPress, "{}", state.clone(), "Key 1").unwrap();
        history.record(UndoableActionType::TypeText, r#"{"text":"hello"}"#, state, "Type").unwrap();
        
        assert_eq!(history.size(), 3);
        assert_eq!(history.stats().mouse_actions, 1);
        assert_eq!(history.stats().keyboard_actions, 2);
    }
    
    #[test]
    fn test_action_history_max_limit() {
        let config = HistoryConfig {
            max_history: 3,
            ..Default::default()
        };
        let mut history = ActionHistory::new(config);
        let state = StateSnapshot::new(0.0, 0.0, 1920, 1080);
        
        for i in 0..5 {
            history.record(
                UndoableActionType::MouseClick,
                "{}",
                state.clone(),
                &format!("Click {}", i)
            ).unwrap();
        }
        
        // Sadece son 3 kalmalı
        assert_eq!(history.size(), 3);
    }
    
    #[test]
    fn test_action_history_clear() {
        let mut history = ActionHistory::default_config();
        let state = StateSnapshot::new(0.0, 0.0, 1920, 1080);
        
        history.record(UndoableActionType::MouseClick, "{}", state, "Test").unwrap();
        assert_eq!(history.size(), 1);
        
        history.clear();
        assert_eq!(history.size(), 0);
    }
    
    #[test]
    fn test_action_history_get_recent() {
        let mut history = ActionHistory::default_config();
        let state = StateSnapshot::new(0.0, 0.0, 1920, 1080);
        
        for i in 0..5 {
            history.record(
                UndoableActionType::MouseClick,
                "{}",
                state.clone(),
                &format!("Click {}", i)
            ).unwrap();
        }
        
        let recent = history.get_recent(3);
        assert_eq!(recent.len(), 3);
        // En son eklenen ilk
        assert_eq!(recent[0].description, "Click 4");
    }
    
    #[test]
    fn test_action_history_get_action() {
        let mut history = ActionHistory::default_config();
        let state = StateSnapshot::new(0.0, 0.0, 1920, 1080);
        
        let id = history.record(UndoableActionType::MouseClick, "{}", state, "Test").unwrap();
        
        let action = history.get_action(id).unwrap();
        assert_eq!(action.id, id);
    }
    
    #[test]
    fn test_action_history_branch() {
        let mut history = ActionHistory::default_config();
        let state = StateSnapshot::new(0.0, 0.0, 1920, 1080);
        
        history.record(UndoableActionType::MouseClick, "{}", state.clone(), "Click 1").unwrap();
        
        let branch_id = history.create_branch("test-branch").unwrap();
        assert_eq!(history.list_branches().len(), 1);
        
        history.switch_branch(branch_id).unwrap();
        assert!(history.active_branch.is_some());
        
        history.switch_to_main();
        assert!(history.active_branch.is_none());
    }
    
    #[test]
    fn test_action_history_delete_branch() {
        let mut history = ActionHistory::default_config();
        let state = StateSnapshot::new(0.0, 0.0, 1920, 1080);
        
        history.record(UndoableActionType::MouseClick, "{}", state, "Click").unwrap();
        
        let branch_id = history.create_branch("to-delete").unwrap();
        assert_eq!(history.list_branches().len(), 1);
        
        history.delete_branch(branch_id).unwrap();
        assert_eq!(history.list_branches().len(), 0);
    }
    
    #[test]
    fn test_action_history_report() {
        let mut history = ActionHistory::default_config();
        let state = StateSnapshot::new(0.0, 0.0, 1920, 1080);
        
        history.record(UndoableActionType::MouseClick, "{}", state, "Test").unwrap();
        
        let report = history.report();
        assert_eq!(report.total_actions, 1);
        assert!(report.can_undo);
        assert!(!report.can_redo);
    }
    
    #[test]
    fn test_history_report_display() {
        let history = ActionHistory::default_config();
        let report = history.report();
        let output = format!("{}", report);
        
        assert!(output.contains("ACTION HISTORY RAPORU"));
    }
    
    #[test]
    fn test_undo_redo_result() {
        let result = UndoRedoResult {
            success: true,
            operation: UndoRedoOperation::Undo,
            action_id: 1,
            message: "Test".to_string(),
            previous_state: None,
            new_state: None,
        };
        
        assert!(result.success);
        assert_eq!(result.operation, UndoRedoOperation::Undo);
    }
    
    #[test]
    fn test_undo_redo_operation_display() {
        assert_eq!(format!("{}", UndoRedoOperation::Undo), "Geri Al");
        assert_eq!(format!("{}", UndoRedoOperation::Redo), "Tekrar Yap");
        assert_eq!(format!("{}", UndoRedoOperation::Jump), "Atla");
        assert_eq!(format!("{}", UndoRedoOperation::Branch), "Dal");
    }
}
