//! ─── TABLE ───
//!
//! Tablo gorunumu bicimlendirme

use colored::Colorize;

/// Tablo olusturucu
pub struct Table {
    /// Kolon basliklari
    headers: Vec<String>,
    /// Satirlar
    rows: Vec<Vec<String>>,
    /// Kolon genislikleri
    widths: Vec<usize>,
    /// Kenar stili
    border_style: BorderStyle,
}

#[derive(Debug, Clone, Copy)]
pub enum BorderStyle {
    /// ASCII karakterler
    Ascii,
    /// Unicode karakterler
    Unicode,
    /// Kenar yok
    None,
}

impl Table {
    pub fn new(headers: Vec<&str>) -> Self {
        let headers: Vec<String> = headers.iter().map(|s| s.to_string()).collect();
        let widths: Vec<usize> = headers.iter().map(|h| h.len() + 2).collect();
        
        Self {
            headers,
            rows: Vec::new(),
            widths,
            border_style: BorderStyle::Unicode,
        }
    }

    /// Satir ekle
    pub fn add_row(&mut self, row: Vec<&str>) -> &mut Self {
        let row: Vec<String> = row.iter().map(|s| s.to_string()).collect();
        
        // Genislikleri guncelle
        for (i, cell) in row.iter().enumerate() {
            if i < self.widths.len() {
                self.widths[i] = self.widths[i].max(cell.len() + 2);
            }
        }
        
        self.rows.push(row);
        self
    }

    /// Kenar stili ayarla
    pub fn border(&mut self, style: BorderStyle) -> &mut Self {
        self.border_style = style;
        self
    }

    /// Render et
    pub fn render(&self) -> String {
        let mut output = String::new();
        
        // Ust kenar
        output.push_str(&self.top_border());
        output.push('\n');
        
        // Baslik satiri
        output.push_str(&self.header_row());
        output.push('\n');
        
        // Baslik ayirici
        output.push_str(&self.separator());
        output.push('\n');
        
        // Veri satırlari
        for row in &self.rows {
            output.push_str(&self.data_row(row));
            output.push('\n');
        }
        
        // Alt kenar
        output.push_str(&self.bottom_border());
        
        output
    }

    fn top_border(&self) -> String {
        match self.border_style {
            BorderStyle::Unicode => {
                let mut line = "╔".cyan().to_string();
                for (i, w) in self.widths.iter().enumerate() {
                    line.push_str(&"═".repeat(*w).cyan().to_string());
                    if i < self.widths.len() - 1 {
                        line.push_str(&"╦".cyan().to_string());
                    }
                }
                line.push_str(&"╗".cyan().to_string());
                line
            }
            BorderStyle::Ascii => {
                let mut line = "+".to_string();
                for (i, w) in self.widths.iter().enumerate() {
                    line.push_str(&"-".repeat(*w));
                    if i < self.widths.len() - 1 {
                        line.push('+');
                    }
                }
                line.push('+');
                line
            }
            BorderStyle::None => String::new(),
        }
    }

    fn bottom_border(&self) -> String {
        match self.border_style {
            BorderStyle::Unicode => {
                let mut line = "╚".cyan().to_string();
                for (i, w) in self.widths.iter().enumerate() {
                    line.push_str(&"═".repeat(*w).cyan().to_string());
                    if i < self.widths.len() - 1 {
                        line.push_str(&"╩".cyan().to_string());
                    }
                }
                line.push_str(&"╝".cyan().to_string());
                line
            }
            BorderStyle::Ascii => {
                let mut line = "+".to_string();
                for (i, w) in self.widths.iter().enumerate() {
                    line.push_str(&"-".repeat(*w));
                    if i < self.widths.len() - 1 {
                        line.push('+');
                    }
                }
                line.push('+');
                line
            }
            BorderStyle::None => String::new(),
        }
    }

    fn separator(&self) -> String {
        match self.border_style {
            BorderStyle::Unicode => {
                let mut line = "╠".cyan().to_string();
                for (i, w) in self.widths.iter().enumerate() {
                    line.push_str(&"═".repeat(*w).cyan().to_string());
                    if i < self.widths.len() - 1 {
                        line.push_str(&"╬".cyan().to_string());
                    }
                }
                line.push_str(&"╣".cyan().to_string());
                line
            }
            BorderStyle::Ascii => {
                let mut line = "+".to_string();
                for (i, w) in self.widths.iter().enumerate() {
                    line.push_str(&"-".repeat(*w));
                    if i < self.widths.len() - 1 {
                        line.push('+');
                    }
                }
                line.push('+');
                line
            }
            BorderStyle::None => String::new(),
        }
    }

    fn header_row(&self) -> String {
        match self.border_style {
            BorderStyle::Unicode | BorderStyle::Ascii => {
                let mut line = "║".cyan().to_string();
                for (i, (header, width)) in self.headers.iter().zip(self.widths.iter()).enumerate() {
                    let padding = width - header.len();
                    let left_pad = padding / 2;
                    let right_pad = padding - left_pad;
                    
                    line.push_str(&" ".repeat(left_pad));
                    line.push_str(&header.yellow().bold().to_string());
                    line.push_str(&" ".repeat(right_pad));
                    
                    if i < self.widths.len() - 1 {
                        line.push_str(&"║".cyan().to_string());
                    }
                }
                line.push_str(&"║".cyan().to_string());
                line
            }
            BorderStyle::None => {
                self.headers
                    .iter()
                    .map(|h| h.yellow().bold().to_string())
                    .collect::<Vec<_>>()
                    .join("  ")
            }
        }
    }

    fn data_row(&self, row: &[String]) -> String {
        match self.border_style {
            BorderStyle::Unicode | BorderStyle::Ascii => {
                let mut line = "║".cyan().to_string();
                for (i, (cell, width)) in row.iter().zip(self.widths.iter()).enumerate() {
                    let padding = width - cell.len();
                    let left_pad = 1;
                    let right_pad = padding - left_pad;
                    
                    line.push_str(&" ".repeat(left_pad));
                    line.push_str(&cell.green().to_string());
                    line.push_str(&" ".repeat(right_pad));
                    
                    if i < self.widths.len() - 1 {
                        line.push_str(&"║".cyan().to_string());
                    }
                }
                line.push_str(&"║".cyan().to_string());
                line
            }
            BorderStyle::None => {
                row.iter()
                    .map(|c| c.green().to_string())
                    .collect::<Vec<_>>()
                    .join("  ")
            }
        }
    }
}

/// Basit liste tablosu
pub fn list_table(items: &[(&str, &str)]) -> String {
    let mut output = String::new();
    
    let max_key_len = items.iter().map(|(k, _)| k.len()).max().unwrap_or(0);
    
    for (key, value) in items {
        output.push_str(&format!(
            "  {:width$} {} {}\n",
            key.blue().bold(),
            "→".cyan(),
            value.green(),
            width = max_key_len
        ));
    }
    
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table_creation() {
        let table = Table::new(vec!["Col1", "Col2"]);
        assert_eq!(table.headers.len(), 2);
    }

    #[test]
    fn test_table_add_row() {
        let mut table = Table::new(vec!["Col1", "Col2"]);
        table.add_row(vec!["A", "B"]);
        assert_eq!(table.rows.len(), 1);
    }
}
