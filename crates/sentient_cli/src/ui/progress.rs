//! ─── PROGRESS ───
//!
//! Ilerleme cizgisi gosterimi

use colored::Colorize;

/// Ilerleme cizgisi
pub struct ProgressBar {
    /// Toplam deger
    total: u64,
    /// Mevcut deger
    current: u64,
    /// Genislik (karakter)
    width: usize,
    /// Baslik
    label: String,
    /// Stil
    style: ProgressStyle,
}

#[derive(Debug, Clone, Copy)]
pub enum ProgressStyle {
    /// Klasik bar
    Bar,
    /// Blok bar
    Blocks,
    /// Noktali
    Dots,
}

impl ProgressBar {
    pub fn new(total: u64, label: &str) -> Self {
        Self {
            total,
            current: 0,
            width: 40,
            label: label.into(),
            style: ProgressStyle::Bar,
        }
    }

    /// Ilerleme guncelle
    pub fn set(&mut self, current: u64) {
        self.current = current.min(self.total);
    }

    /// Ilerleme artir
    pub fn inc(&mut self, amount: u64) {
        self.current = (self.current + amount).min(self.total);
    }

    /// Tamamlandi isaretle
    pub fn finish(&mut self) {
        self.current = self.total;
    }

    /// Render et
    pub fn render(&self) -> String {
        let percent = if self.total > 0 {
            self.current as f64 / self.total as f64
        } else {
            1.0
        };

        let filled = (self.width as f64 * percent) as usize;
        let empty = self.width - filled;

        let bar = match self.style {
            ProgressStyle::Bar => {
                format!(
                    "{}{}",
                    "=".repeat(filled).green(),
                    " ".repeat(empty)
                )
            }
            ProgressStyle::Blocks => {
                let blocks = ["▏", "▎", "▍", "▌", "▋", "▊", "▉", "█"];
                let filled_blocks = filled / 8;
                let partial = filled % 8;
                
                let mut bar = String::new();
                bar.push_str(&"█".repeat(filled_blocks).green().to_string());
                if partial > 0 && filled_blocks < self.width / 8 {
                    bar.push_str(&blocks[partial].green().to_string());
                }
                bar
            }
            ProgressStyle::Dots => {
                format!(
                    "{}{}",
                    "●".repeat(filled).green(),
                    "○".repeat(empty).bright_black()
                )
            }
        };

        let percent_text = format!("{:.0}%", percent * 100.0);
        
        format!(
            "{} [{}] {} {}/{}",
            self.label.blue(),
            bar,
            percent_text.yellow(),
            self.current.to_string().green(),
            self.total.to_string().dimmed()
        )
    }

    /// Tek satir olarak yazdir
    pub fn print(&self) {
        print!("\r{}", self.render());
        use std::io::Write;
        std::io::stdout().flush().ok();
    }

    /// Yeni satira gec
    pub fn println(&self) {
        println!("\r{}", self.render());
    }
}

/// Coklu ilerleme cizgisi
pub struct MultiProgress {
    /// Ilerleme cizgileri
    bars: Vec<ProgressBar>,
}

impl MultiProgress {
    pub fn new() -> Self {
        Self { bars: Vec::new() }
    }

    /// Ilerleme cizgisi ekle
    pub fn add(&mut self, bar: ProgressBar) -> usize {
        let idx = self.bars.len();
        self.bars.push(bar);
        idx
    }

    /// Ilerleme cizgisi guncelle
    pub fn set(&mut self, idx: usize, current: u64) {
        if let Some(bar) = self.bars.get_mut(idx) {
            bar.set(current);
        }
    }

    /// Tumunu render et
    pub fn render(&self) -> String {
        self.bars.iter().map(|b| b.render()).collect::<Vec<_>>().join("\n")
    }

    /// Tumunu yazdir
    pub fn print(&self) {
        // Imleci yukari tasi
        print!("\x1B[{}A", self.bars.len());
        println!("{}", self.render());
    }
}

impl Default for MultiProgress {
    fn default() -> Self {
        Self::new()
    }
}

/// Basit ilerleme gostergesi
pub fn progress_summary(current: usize, total: usize, label: &str) -> String {
    let percent = if total > 0 { current * 100 / total } else { 100 };
    let bar_width = 20;
    let filled = percent * bar_width / 100;
    
    format!(
        "{} [{}{}] {}%",
        label.blue(),
        "█".repeat(filled).green(),
        "░".repeat(bar_width - filled).dimmed(),
        percent.to_string().yellow()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_progress_bar() {
        let bar = ProgressBar::new(100, "Test");
        assert_eq!(bar.current, 0);
    }

    #[test]
    fn test_progress_render() {
        let bar = ProgressBar::new(100, "Test");
        let rendered = bar.render();
        assert!(rendered.contains("0%"));
    }
}
