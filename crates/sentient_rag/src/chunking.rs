// ═══════════════════════════════════════════════════════════════════════════════
//  SENTIENT OS - Chunking Strategies
// ═══════════════════════════════════════════════════════════════════════════════

use serde::{Deserialize, Serialize};
use crate::{Document, Result, RAGError};

/// Text chunk
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chunk {
    /// Source document ID
    pub doc_id: String,
    /// Chunk content
    pub content: String,
    /// Start position in document
    pub start: usize,
    /// End position in document
    pub end: usize,
}

impl Chunk {
    pub fn new(doc_id: impl Into<String>, content: impl Into<String>, start: usize, end: usize) -> Self {
        Self {
            doc_id: doc_id.into(),
            content: content.into(),
            start,
            end,
        }
    }

    pub fn len(&self) -> usize {
        self.content.len()
    }

    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }
}

/// Chunking strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChunkingStrategy {
    /// Fixed character size
    FixedSize,
    /// Sentence-based
    Sentence,
    /// Paragraph-based
    Paragraph,
    /// Semantic (requires embeddings)
    Semantic,
    /// Recursive (hierarchical)
    Recursive,
}

/// Chunker configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkerConfig {
    /// Chunk size (characters)
    pub chunk_size: usize,
    /// Overlap between chunks
    pub overlap: usize,
    /// Chunking strategy
    pub strategy: ChunkingStrategy,
    /// Minimum chunk size
    pub min_chunk_size: usize,
}

impl Default for ChunkerConfig {
    fn default() -> Self {
        Self {
            chunk_size: 512,
            overlap: 50,
            strategy: ChunkingStrategy::Recursive,
            min_chunk_size: 100,
        }
    }
}

/// Text chunker
pub struct Chunker {
    config: ChunkerConfig,
}

impl Chunker {
    pub fn new(config: ChunkerConfig) -> Self {
        Self { config }
    }

    pub fn with_defaults() -> Self {
        Self::new(ChunkerConfig::default())
    }

    /// Chunk a document
    pub fn chunk(&self, document: &Document) -> Result<Vec<Chunk>> {
        match self.config.strategy {
            ChunkingStrategy::FixedSize => self.chunk_fixed(document),
            ChunkingStrategy::Sentence => self.chunk_sentences(document),
            ChunkingStrategy::Paragraph => self.chunk_paragraphs(document),
            ChunkingStrategy::Recursive => self.chunk_recursive(document),
            ChunkingStrategy::Semantic => {
                // Requires embeddings - fallback to recursive
                self.chunk_recursive(document)
            }
        }
    }

    /// Fixed-size chunking
    fn chunk_fixed(&self, document: &Document) -> Result<Vec<Chunk>> {
        let mut chunks = Vec::new();
        let content = &document.content;
        let mut start = 0;

        while start < content.len() {
            let end = (start + self.config.chunk_size).min(content.len());
            let chunk_content = content[start..end].to_string();

            if chunk_content.len() >= self.config.min_chunk_size {
                chunks.push(Chunk::new(&document.id, chunk_content, start, end));
            }

            start = end.saturating_sub(self.config.overlap);
            if end == content.len() {
                break;
            }
        }

        Ok(chunks)
    }

    /// Sentence-based chunking
    fn chunk_sentences(&self, document: &Document) -> Result<Vec<Chunk>> {
        let sentences: Vec<(usize, usize)> = document.content
            .match_indices(&['.', '!', '?', '\n'][..])
            .filter_map(|(i, _)| {
                // Find sentence boundaries
                let start = document.content[..i]
                    .rfind(&['.', '!', '?', '\n'][..])
                    .map(|p| p + 1)
                    .unwrap_or(0);
                Some((start, i + 1))
            })
            .collect();

        let mut chunks = Vec::new();
        let mut current_start = 0;
        let mut current_end = 0;

        for (start, end) in sentences {
            if end - current_start >= self.config.chunk_size {
                if current_end > current_start {
                    let content = document.content[current_start..current_end].to_string();
                    if content.len() >= self.config.min_chunk_size {
                        chunks.push(Chunk::new(&document.id, content, current_start, current_end));
                    }
                }
                current_start = current_end.saturating_sub(self.config.overlap);
            }
            current_end = end;
        }

        // Last chunk
        if current_end > current_start {
            let content = document.content[current_start..current_end].to_string();
            if content.len() >= self.config.min_chunk_size {
                chunks.push(Chunk::new(&document.id, content, current_start, current_end));
            }
        }

        Ok(chunks)
    }

    /// Paragraph-based chunking
    fn chunk_paragraphs(&self, document: &Document) -> Result<Vec<Chunk>> {
        let paragraphs: Vec<&str> = document.content
            .split("\n\n")
            .filter(|p| !p.trim().is_empty())
            .collect();

        let mut chunks = Vec::new();
        let mut current_pos = 0;

        for para in paragraphs {
            let para_start = document.content[current_pos..]
                .find(para)
                .map(|i| current_pos + i)
                .unwrap_or(current_pos);
            let para_end = para_start + para.len();

            chunks.push(Chunk::new(&document.id, para.to_string(), para_start, para_end));
            current_pos = para_end;
        }

        Ok(chunks)
    }

    /// Recursive chunking (hierarchical)
    fn chunk_recursive(&self, document: &Document) -> Result<Vec<Chunk>> {
        let mut chunks = Vec::new();
        let content = &document.content;

        // Try paragraphs first
        let paragraphs: Vec<&str> = content.split("\n\n").collect();

        for para in paragraphs {
            if para.len() <= self.config.chunk_size {
                if para.len() >= self.config.min_chunk_size {
                    let start = content.find(para).unwrap_or(0);
                    chunks.push(Chunk::new(&document.id, para.to_string(), start, start + para.len()));
                }
            } else {
                // Split large paragraphs into sentences
                let sentences: Vec<&str> = para.split(&['.', '!', '?'][..]).collect();
                let mut current_chunk = String::new();
                let mut chunk_start = 0;

                for sentence in sentences {
                    let trimmed = sentence.trim();
                    if trimmed.is_empty() {
                        continue;
                    }

                    if current_chunk.len() + trimmed.len() > self.config.chunk_size {
                        if current_chunk.len() >= self.config.min_chunk_size {
                            let start = content.find(&current_chunk).unwrap_or(0);
                            chunks.push(Chunk::new(&document.id, current_chunk.clone(), start, start + current_chunk.len()));
                        }
                        current_chunk = trimmed.to_string();
                    } else {
                        if current_chunk.is_empty() {
                            chunk_start = content.find(trimmed).unwrap_or(0);
                        }
                        current_chunk.push_str(trimmed);
                        current_chunk.push('.');
                    }
                }

                if current_chunk.len() >= self.config.min_chunk_size {
                    let start = content.find(&current_chunk).unwrap_or(0);
                    let len = current_chunk.len();
                    chunks.push(Chunk::new(&document.id, current_chunk, start, start + len));
                }
            }
        }

        Ok(chunks)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_creation() {
        let chunk = Chunk::new("doc1", "Hello world", 0, 11);
        assert_eq!(chunk.doc_id, "doc1");
        assert_eq!(chunk.len(), 11);
    }

    #[test]
    fn test_chunker_fixed_size() {
        let config = ChunkerConfig {
            chunk_size: 50,
            overlap: 10,
            strategy: ChunkingStrategy::FixedSize,
            min_chunk_size: 10,
        };
        let chunker = Chunker::new(config);

        let doc = Document::new("doc1", "This is a test document with enough content to be chunked into multiple pieces for testing purposes.");
        let chunks = chunker.chunk(&doc).unwrap();

        assert!(!chunks.is_empty());
    }

    #[test]
    fn test_chunker_paragraphs() {
        let config = ChunkerConfig {
            strategy: ChunkingStrategy::Paragraph,
            ..Default::default()
        };
        let chunker = Chunker::new(config);

        let doc = Document::new("doc1", "First paragraph.\n\nSecond paragraph.\n\nThird paragraph.");
        let chunks = chunker.chunk(&doc).unwrap();

        assert_eq!(chunks.len(), 3);
    }
}
