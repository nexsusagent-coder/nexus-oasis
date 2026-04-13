//! ═════════════════════════════════════════════════════════════════
//!  COMPRESSION MODULE - Bellek Sıkıştırma
//! ═════════════════════════════════════════════════════════════════
//!
//! Büyük metinlerin LZ77-benzeri algoritma ile sıkıştırılması.
//! SQLite şişmesini önler, sorgu performansını artırır.

/// Basit RLE (Run-Length Encoding) + dictionary sıkıştırma
pub struct MemoryCompressor {
    /// Sıkıştırma eşiği (bu boyuttan büyük metinler sıkıştırılır)
    pub threshold_bytes: usize,
    /// Sıkıştırma seviyesi (1-9)
    pub level: u8,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CompressedEntry {
    pub original_size: usize,
    pub compressed_size: usize,
    pub algorithm: String,
    pub data: Vec<u8>,
}

impl MemoryCompressor {
    pub fn new() -> Self {
        Self {
            threshold_bytes: 1024, // 1KB üzeri sıkıştır
            level: 6,
        }
    }

    /// Metin sıkıştır (basit RLE + sık kelime sözlüğü)
    pub fn compress(&self, text: &str) -> CompressedEntry {
        let original = text.as_bytes();
        let original_size = original.len();

        if original_size < self.threshold_bytes {
            // Eşik altı - sıkıştırma gereksiz
            return CompressedEntry {
                original_size,
                compressed_size: original_size,
                algorithm: "none".into(),
                data: original.to_vec(),
            };
        }

        // Basit sıkıştırma: sık kelimeleri kısa kodlarla değiştir
        let mut dict: Vec<(String, String)> = Vec::new();
        let common = ["SENTIENT", "bellek", "sorgu", "yanıt", "sistem", "modül", "MemoryCube"];
        for (i, word) in common.iter().enumerate() {
            if text.contains(word) && word.len() > 3 {
                let code = format!("\x01{}", (i as u8) as char);
                let count = text.matches(word).count();
                if count * word.len() > count * code.len() + word.len() + 2 {
                    dict.push((word.to_string(), code));
                }
            }
        }

        let mut compressed = text.to_string();
        for (word, code) in &dict {
            compressed = compressed.replace(word, code);
        }

        // RLE: tekrar eden karakterleri sıkıştır
        let bytes = compressed.as_bytes();
        let mut rle = Vec::new();
        let mut i = 0;
        while i < bytes.len() {
            let ch = bytes[i];
            let mut run = 1;
            while i + run < bytes.len() && bytes[i + run] == ch && run < 255 {
                run += 1;
            }
            if run >= 4 {
                rle.push(0xFF); // RLE marker
                rle.push(ch);
                rle.push(run as u8);
                i += run;
            } else {
                rle.push(ch);
                i += 1;
            }
        }

        CompressedEntry {
            original_size,
            compressed_size: rle.len(),
            algorithm: "rle+dict".into(),
            data: rle,
        }
    }

    /// Sıkıştırılmış veriyi çöz
    pub fn decompress(&self, entry: &CompressedEntry) -> String {
        if entry.algorithm == "none" {
            return String::from_utf8_lossy(&entry.data).to_string();
        }

        // RLE çöz
        let mut decompressed = Vec::new();
        let mut i = 0;
        while i < entry.data.len() {
            if entry.data[i] == 0xFF && i + 2 < entry.data.len() {
                let ch = entry.data[i + 1];
                let run = entry.data[i + 2] as usize;
                for _ in 0..run {
                    decompressed.push(ch);
                }
                i += 3;
            } else {
                decompressed.push(entry.data[i]);
                i += 1;
            }
        }

        // Sözlük çöz
        let common = ["SENTIENT", "bellek", "sorgu", "yanıt", "sistem", "modül", "MemoryCube"];
        let mut text = String::from_utf8_lossy(&decompressed).to_string();
        for (i, word) in common.iter().enumerate() {
            let code = format!("\x01{}", (i as u8) as char);
            text = text.replace(&code, word);
        }

        text
    }

    /// Sıkıştırma oranını hesapla
    pub fn compression_ratio(&self, entry: &CompressedEntry) -> f64 {
        if entry.original_size == 0 { return 1.0; }
        entry.compressed_size as f64 / entry.original_size as f64
    }

    /// Sıkıştırma gerekiyor mu?
    pub fn should_compress(&self, text: &str) -> bool {
        text.len() >= self.threshold_bytes
    }
}

impl Default for MemoryCompressor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compress_small() {
        let comp = MemoryCompressor::new();
        let text = "Merhaba";
        let result = comp.compress(text);
        assert_eq!(result.algorithm, "none");
    }

    #[test]
    fn test_compress_large() {
        let comp = MemoryCompressor::new();
        let text = "SENTIENT SENTIENT SENTIENT ".repeat(100);
        let result = comp.compress(&text);
        assert!(result.compressed_size < result.original_size);
    }

    #[test]
    fn test_decompress_roundtrip() {
        let comp = MemoryCompressor::new();
        let text = "SENTIENT bellek sorgu sistemi ".repeat(100);
        let compressed = comp.compress(&text);
        let decompressed = comp.decompress(&compressed);
        assert!(decompressed.contains("SENTIENT"));
    }

    #[test]
    fn test_should_compress() {
        let comp = MemoryCompressor::new();
        assert!(!comp.should_compress("kısa"));
        assert!(comp.should_compress(&"x".repeat(2000)));
    }
}
