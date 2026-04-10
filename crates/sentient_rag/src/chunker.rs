//! Document chunking strategies

use crate::types::*;
use crate::{RagError, Result};
use regex::Regex;
use std::sync::LazyLock;

/// Static regex patterns
static SENTENCE_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"[.!?]+\s+").expect("Invalid sentence regex")
});

static PARAGRAPH_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\n\s*\n").expect("Invalid paragraph regex")
});

static CODE_BLOCK_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?m)^(fn |struct |impl |class |def |function |pub |async |const |let |var )")
        .expect("Invalid code block regex")
});

/// Document chunker
pub struct Chunker {
    config: ChunkingConfig,
}

impl Chunker {
    /// Create new chunker with config
    pub fn new(config: ChunkingConfig) -> Self {
        Self { config }
    }

    /// Create chunker with default config
    pub fn default_config() -> Self {
        Self::new(ChunkingConfig::default())
    }

    /// Chunk a document
    pub fn chunk(&self, document: &Document) -> Result<Vec<Chunk>> {
        match self.config.strategy {
            ChunkingStrategy::FixedSize => self.chunk_fixed(document),
            ChunkingStrategy::Sentence => self.chunk_sentences(document),
            ChunkingStrategy::Paragraph => self.chunk_paragraphs(document),
            ChunkingStrategy::Recursive => self.chunk_recursive(document),
            ChunkingStrategy::Semantic => self.chunk_semantic(document),
            ChunkingStrategy::Code => self.chunk_code(document),
        }
    }

    /// Chunk text directly
    pub fn chunk_text(&self, text: &str) -> Result<Vec<Chunk>> {
        let doc = Document::new(text);
        self.chunk(&doc)
    }

    /// Fixed size chunking
    fn chunk_fixed(&self, document: &Document) -> Result<Vec<Chunk>> {
        let content = &document.content;
        let len = content.len();

        if len == 0 {
            return Ok(Vec::new());
        }

        let mut chunks = Vec::new();
        let mut start = 0;
        let mut index = 0;

        while start < len {
            let end = (start + self.config.chunk_size).min(len);
            let chunk_content = &content[start..end];

            chunks.push(
                Chunk::new(&document.id, chunk_content.to_string(), index)
                    .with_position(start, end)
            );

            start = if self.config.overlap > 0 && end < len {
                end.saturating_sub(self.config.overlap)
            } else {
                end
            };
            index += 1;
        }

        Ok(chunks)
    }

    /// Sentence-based chunking
    fn chunk_sentences(&self, document: &Document) -> Result<Vec<Chunk>> {
        let content = &document.content;

        // Find all sentence boundaries
        let mut boundaries = vec![0];
        for cap in SENTENCE_REGEX.find_iter(content) {
            boundaries.push(cap.end());
        }
        boundaries.push(content.len());

        // Create chunks from sentences
        let mut chunks = Vec::new();
        let mut current_start = 0;
        let mut current_end = 0;
        let mut index = 0;

        for i in 0..boundaries.len() - 1 {
            let sent_start = boundaries[i];
            let sent_end = boundaries[i + 1];

            if sent_end - current_start >= self.config.chunk_size {
                // Save current chunk
                if current_start < current_end {
                    let chunk_content = &content[current_start..current_end];
                    chunks.push(
                        Chunk::new(&document.id, chunk_content.to_string(), index)
                            .with_position(current_start, current_end)
                    );
                    index += 1;
                }

                // Start new chunk with overlap
                current_start = if self.config.overlap > 0 && current_end > self.config.overlap {
                    current_end.saturating_sub(self.config.overlap)
                } else {
                    sent_start
                };
            }

            current_end = sent_end;
        }

        // Add remaining content
        if current_start < content.len() {
            let chunk_content = &content[current_start..];
            chunks.push(
                Chunk::new(&document.id, chunk_content.to_string(), index)
                    .with_position(current_start, content.len())
            );
        }

        Ok(chunks)
    }

    /// Paragraph-based chunking
    fn chunk_paragraphs(&self, document: &Document) -> Result<Vec<Chunk>> {
        let content = &document.content;

        // Split by paragraph boundaries
        let paragraphs: Vec<&str> = PARAGRAPH_REGEX.split(content).collect();

        let mut chunks = Vec::new();
        let mut current_content = String::new();
        let mut current_start = 0;
        let mut index = 0;
        let mut pos = 0;

        for para in paragraphs {
            let para_len = para.len();

            if current_content.len() + para_len > self.config.chunk_size && !current_content.is_empty()
            {
                // Save chunk
                chunks.push(
                    Chunk::new(&document.id, current_content.clone(), index)
                        .with_position(current_start, pos)
                );
                index += 1;

                current_content = String::new();
                current_start = pos;
            }

            current_content.push_str(para);
            current_content.push_str("\n\n");
            pos += para_len + 2;
        }

        // Add remaining
        if !current_content.is_empty() {
            chunks.push(
                Chunk::new(&document.id, current_content.trim().to_string(), index)
                    .with_position(current_start, content.len())
            );
        }

        Ok(chunks)
    }

    /// Recursive chunking (tries larger separators first)
    fn chunk_recursive(&self, document: &Document) -> Result<Vec<Chunk>> {
        let content = &document.content;
        let mut chunks = Vec::new();
        let mut index = 0;

        self.chunk_recursive_inner(
            content,
            &self.config.separators,
            0,
            &mut chunks,
            &mut index,
            &document.id,
        )?;

        Ok(chunks)
    }

    fn chunk_recursive_inner(
        &self,
        text: &str,
        separators: &[String],
        separator_index: usize,
        chunks: &mut Vec<Chunk>,
        index: &mut usize,
        doc_id: &str,
    ) -> Result<()> {
        if text.is_empty() {
            return Ok(());
        }

        // If text is small enough, add as chunk
        if text.len() <= self.config.chunk_size {
            chunks.push(
                Chunk::new(doc_id, text.to_string(), *index)
            );
            *index += 1;
            return Ok(());
        }

        // Try current separator
        if separator_index >= separators.len() {
            // No more separators, do fixed size
            let mut start = 0;
            while start < text.len() {
                let end = (start + self.config.chunk_size).min(text.len());
                chunks.push(
                    Chunk::new(doc_id, text[start..end].to_string(), *index)
                );
                *index += 1;
                start = end.saturating_sub(self.config.overlap);
            }
            return Ok(());
        }

        let separator = &separators[separator_index];
        let parts: Vec<&str> = text.split(separator).collect();

        let mut current_chunk = String::new();

        for part in parts {
            if current_chunk.len() + part.len() + separator.len() > self.config.chunk_size {
                if !current_chunk.is_empty() {
                    // Check if chunk is still too big
                    if current_chunk.len() > self.config.max_chunk_size {
                        // Recurse with next separator
                        self.chunk_recursive_inner(
                            &current_chunk,
                            separators,
                            separator_index + 1,
                            chunks,
                            index,
                            doc_id,
                        )?;
                    } else {
                        chunks.push(
                            Chunk::new(doc_id, current_chunk.clone(), *index)
                        );
                        *index += 1;
                    }
                    current_chunk = String::new();
                }

                // If part itself is too large, recurse
                if part.len() > self.config.chunk_size {
                    self.chunk_recursive_inner(
                        part,
                        separators,
                        separator_index + 1,
                        chunks,
                        index,
                        doc_id,
                    )?;
                } else {
                    current_chunk.push_str(part);
                }
            } else {
                if !current_chunk.is_empty() {
                    current_chunk.push_str(separator);
                }
                current_chunk.push_str(part);
            }
        }

        if !current_chunk.is_empty() {
            if current_chunk.len() > self.config.max_chunk_size {
                self.chunk_recursive_inner(
                    &current_chunk,
                    separators,
                    separator_index + 1,
                    chunks,
                    index,
                    doc_id,
                )?;
            } else {
                chunks.push(
                    Chunk::new(doc_id, current_chunk, *index)
                );
                *index += 1;
            }
        }

        Ok(())
    }

    /// Semantic chunking (requires embeddings - stub)
    fn chunk_semantic(&self, document: &Document) -> Result<Vec<Chunk>> {
        // Semantic chunking requires embeddings
        // Fall back to recursive for now
        #[cfg(not(feature = "embeddings"))]
        {
            tracing::warn!("Semantic chunking requires 'embeddings' feature, falling back to recursive");
            self.chunk_recursive(document)
        }

        #[cfg(feature = "embeddings")]
        {
            // TODO: Implement semantic chunking with embeddings
            self.chunk_recursive(document)
        }
    }

    /// Code-aware chunking
    fn chunk_code(&self, document: &Document) -> Result<Vec<Chunk>> {
        let content = &document.content;

        // Find code block boundaries
        let mut boundaries = vec![0];
        for cap in CODE_BLOCK_REGEX.find_iter(content) {
            boundaries.push(cap.start());
        }
        boundaries.push(content.len());

        // Remove duplicates and sort
        boundaries.sort();
        boundaries.dedup();

        let mut chunks = Vec::new();
        let mut index = 0;

        for i in 0..boundaries.len() - 1 {
            let start = boundaries[i];
            let end = boundaries[i + 1];
            let chunk_content = &content[start..end];

            // If chunk is too large, split further
            if chunk_content.len() > self.config.max_chunk_size {
                let sub_chunks = self.split_code_block(chunk_content, start);
                for (sub_start, sub_end) in sub_chunks {
                    chunks.push(
                        Chunk::new(&document.id, content[sub_start..sub_end].to_string(), index)
                            .with_position(sub_start, sub_end)
                    );
                    index += 1;
                }
            } else {
                chunks.push(
                    Chunk::new(&document.id, chunk_content.to_string(), index)
                        .with_position(start, end)
                );
                index += 1;
            }
        }

        Ok(chunks)
    }

    fn split_code_block(&self, content: &str, offset: usize) -> Vec<(usize, usize)> {
        let mut ranges = Vec::new();
        let mut start = 0;

        // Split by lines, trying to keep functions together
        let lines: Vec<&str> = content.lines().collect();
        let mut current_start = 0;
        let mut current_len = 0;

        for (i, line) in lines.iter().enumerate() {
            if current_len + line.len() > self.config.chunk_size && current_start < i {
                let abs_start = offset + lines[current_start..current_start].iter().map(|l| l.len() + 1).sum::<usize>();
                let abs_end = offset + lines[current_start..i].iter().map(|l| l.len() + 1).sum::<usize>();
                ranges.push((abs_start, abs_end));
                current_start = i;
                current_len = 0;
            }
            current_len += line.len() + 1;
        }

        if current_start < lines.len() {
            let abs_start = offset;
            let abs_end = offset + content.len();
            ranges.push((abs_start, abs_end));
        }

        ranges
    }

    /// Estimate token count for text
    pub fn estimate_tokens(text: &str) -> usize {
        // Rough approximation: ~4 characters per token
        text.len() / 4
    }

    /// Get config
    pub fn config(&self) -> &ChunkingConfig {
        &self.config
    }
}

impl Default for Chunker {
    fn default() -> Self {
        Self::default_config()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed_size_chunking() {
        let config = ChunkingConfig {
            strategy: ChunkingStrategy::FixedSize,
            chunk_size: 10,
            overlap: 0,
            ..Default::default()
        };
        let chunker = Chunker::new(config);
        let doc = Document::new("This is a test document with enough content.");

        let chunks = chunker.chunk(&doc).unwrap();
        assert!(!chunks.is_empty());

        // Verify all chunks together cover the document
        let total_len: usize = chunks.iter().map(|c| c.content.len()).sum();
        assert_eq!(total_len, doc.content.len());
    }

    #[test]
    fn test_fixed_size_with_overlap() {
        let config = ChunkingConfig {
            strategy: ChunkingStrategy::FixedSize,
            chunk_size: 20,
            overlap: 5,
            ..Default::default()
        };
        let chunker = Chunker::new(config);
        let doc = Document::new("This is a test document with enough content for multiple chunks.");

        let chunks = chunker.chunk(&doc).unwrap();
        assert!(chunks.len() > 1);
    }

    #[test]
    fn test_sentence_chunking() {
        let config = ChunkingConfig {
            strategy: ChunkingStrategy::Sentence,
            chunk_size: 30,
            ..Default::default()
        };
        let chunker = Chunker::new(config);
        let doc = Document::new("First sentence. Second sentence. Third sentence. Fourth sentence.");

        let chunks = chunker.chunk(&doc).unwrap();
        assert!(!chunks.is_empty());
    }

    #[test]
    fn test_paragraph_chunking() {
        let config = ChunkingConfig {
            strategy: ChunkingStrategy::Paragraph,
            chunk_size: 100,
            ..Default::default()
        };
        let chunker = Chunker::new(config);
        let doc = Document::new("First paragraph.\n\nSecond paragraph.\n\nThird paragraph.");

        let chunks = chunker.chunk(&doc).unwrap();
        assert!(!chunks.is_empty());
    }

    #[test]
    fn test_recursive_chunking() {
        let config = ChunkingConfig::default();
        let chunker = Chunker::new(config);
        let doc = Document::new("This is a test.\n\nThis is another paragraph.\n\nAnd one more.");

        let chunks = chunker.chunk(&doc).unwrap();
        assert!(!chunks.is_empty());
    }

    #[test]
    fn test_empty_document() {
        let chunker = Chunker::default();
        let doc = Document::new("");

        let chunks = chunker.chunk(&doc).unwrap();
        assert!(chunks.is_empty());
    }

    #[test]
    fn test_small_document() {
        let config = ChunkingConfig {
            chunk_size: 1000,
            ..Default::default()
        };
        let chunker = Chunker::new(config);
        let doc = Document::new("Small content");

        let chunks = chunker.chunk(&doc).unwrap();
        assert_eq!(chunks.len(), 1);
    }

    #[test]
    fn test_chunk_text_directly() {
        let chunker = Chunker::default();
        let chunks = chunker.chunk_text("Some text to chunk").unwrap();
        assert!(!chunks.is_empty());
    }

    #[test]
    fn test_code_chunking() {
        let config = ChunkingConfig::code();
        let chunker = Chunker::new(config);
        let doc = Document::new(r#"
fn main() {
    println!("Hello");
}

struct User {
    name: String,
}

impl User {
    fn new(name: String) -> Self {
        Self { name }
    }
}
"#);

        let chunks = chunker.chunk(&doc).unwrap();
        assert!(!chunks.is_empty());
    }

    #[test]
    fn test_estimate_tokens() {
        let text = "This is a test sentence with some tokens";
        let tokens = Chunker::estimate_tokens(text);
        assert!(tokens > 0);
    }
}
