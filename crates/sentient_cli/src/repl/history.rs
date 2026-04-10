//! ─── HISTORY MODULU ───
//!
//! Komut gecmisi yonetimi

use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::path::PathBuf;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use chrono::{DateTime, Utc};

/// Komut gecmisi
#[derive(Debug)]
pub struct CommandHistory {
    /// Gecmis kayitlari
    entries: VecDeque<HistoryEntry>,
    /// Maksimum kayit sayisi
    max_size: usize,
    /// Gecmis dosyasi yolu
    file_path: PathBuf,
    /// Mevcut pozisyon (navigate icin)
    position: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    /// Komut
    pub command: String,
    /// Zaman damgasi
    pub timestamp: DateTime<Utc>,
    /// Sonuc basarili mi
    pub success: bool,
    /// Modul baglami
    pub context: Option<String>,
}

impl CommandHistory {
    pub fn new(max_size: usize) -> Self {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
        let file_path = PathBuf::from(home).join(".sentient_history");

        Self {
            entries: VecDeque::with_capacity(max_size),
            max_size,
            file_path,
            position: None,
        }
    }

    /// Dosyadan yukle
    pub fn load(&mut self) {
        if !self.file_path.exists() {
            return;
        }

        let file = match File::open(&self.file_path) {
            Ok(f) => f,
            Err(_) => return,
        };

        let reader = BufReader::new(file);
        for line in reader.lines().filter_map(|l| l.ok()) {
            if let Ok(entry) = serde_json::from_str::<HistoryEntry>(&line) {
                if self.entries.len() >= self.max_size {
                    self.entries.pop_front();
                }
                self.entries.push_back(entry);
            }
        }
    }

    /// Dosyaya kaydet
    pub fn save(&self) {
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&self.file_path);

        if let Ok(mut file) = file {
            for entry in &self.entries {
                if let Ok(json) = serde_json::to_string(entry) {
                    let _ = writeln!(file, "{}", json);
                }
            }
        }
    }

    /// Komut ekle
    pub fn push(&mut self, command: &str, success: bool, context: Option<&str>) {
        let entry = HistoryEntry {
            command: command.to_string(),
            timestamp: Utc::now(),
            success,
            context: context.map(|s| s.to_string()),
        };

        // Ayni komut tekrarini onle (en son girilen hariç)
        if let Some(last) = self.entries.back() {
            if last.command == command {
                self.entries.pop_back();
            }
        }

        if self.entries.len() >= self.max_size {
            self.entries.pop_front();
        }

        self.entries.push_back(entry);
        self.position = None;
    }

    /// Onceki komutu getir (navigate up)
    pub fn previous(&mut self) -> Option<&str> {
        if self.entries.is_empty() {
            return None;
        }

        let pos = self.position.map_or(self.entries.len() - 1, |p| {
            if p > 0 { p - 1 } else { 0 }
        });

        self.position = Some(pos);
        self.entries.iter().nth(pos).map(|e| e.command.as_str())
    }

    /// Sonraki komutu getir (navigate down)
    pub fn next(&mut self) -> Option<&str> {
        let pos = self.position?;

        if pos >= self.entries.len() - 1 {
            self.position = None;
            return None;
        }

        let new_pos = pos + 1;
        self.position = Some(new_pos);
        self.entries.iter().nth(new_pos).map(|e| e.command.as_str())
    }

    /// Arama yap
    pub fn search(&self, query: &str) -> Vec<&HistoryEntry> {
        let query_lower = query.to_lowercase();
        self.entries
            .iter()
            .filter(|e| e.command.to_lowercase().contains(&query_lower))
            .collect()
    }

    /// Son N komutu listele
    pub fn list_recent(&self, n: usize) -> Vec<&HistoryEntry> {
        self.entries.iter().rev().take(n).collect()
    }

    /// Gecmisi temizle
    pub fn clear(&mut self) {
        self.entries.clear();
        self.position = None;
        let _ = std::fs::remove_file(&self.file_path);
    }

    /// Gecmis boyutu
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_history_push() {
        let mut history = CommandHistory::new(10);
        history.push("test command", true, None);
        assert_eq!(history.len(), 1);
    }

    #[test]
    fn test_history_navigation() {
        let mut history = CommandHistory::new(10);
        history.push("cmd1", true, None);
        history.push("cmd2", true, None);
        history.push("cmd3", true, None);

        assert_eq!(history.previous(), Some("cmd3"));
        assert_eq!(history.previous(), Some("cmd2"));
        assert_eq!(history.next(), Some("cmd3"));
        assert_eq!(history.next(), None);
    }

    #[test]
    fn test_history_search() {
        let mut history = CommandHistory::new(10);
        history.push("status", true, None);
        history.push("help", true, None);
        history.push("status verbose", true, None);

        let results = history.search("status");
        assert_eq!(results.len(), 2);
    }
}
