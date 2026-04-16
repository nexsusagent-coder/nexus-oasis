//! ─── Vector Index Types ───
//!
//! Different index algorithms for vector search:
//! - **HNSW**: Hierarchical Navigable Small World (fast, accurate, high memory)
//! - **IVF**: Inverted File Index (fast, lower accuracy, lower memory)
//! - **Flat**: Brute force (exact, slow, low memory)
//! - **PQ**: Product Quantization (compressed, fast, lower accuracy)
//! - **LSH**: Locality Sensitive Hashing (fast, approximate)

use serde::{Deserialize, Serialize};

// ═══════════════════════════════════════════════════════════════════════════════
//  INDEX TYPES
// ═══════════════════════════════════════════════════════════════════════════════

/// Vector index type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IndexType {
    /// Flat index (brute force)
    Flat,
    /// HNSW index
    Hnsw(HnswConfig),
    /// IVF index
    Ivf(IvfConfig),
    /// Product quantization
    ProductQuantization(PqConfig),
    /// Locality sensitive hashing
    Lsh(LshConfig),
}

impl Default for IndexType {
    fn default() -> Self {
        Self::Hnsw(HnswConfig::default())
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  HNSW CONFIGURATION
// ═══════════════════════════════════════════════════════════════════════════════

/// HNSW index configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HnswConfig {
    /// Number of connections per node (M)
    pub m: usize,
    /// Size of dynamic candidate list for construction
    pub ef_construction: usize,
    /// Size of dynamic candidate list for search
    pub ef_search: usize,
    /// Enable quantization
    pub quantize: bool,
    /// Quantization type
    pub quantization_bits: u8,
}

impl Default for HnswConfig {
    fn default() -> Self {
        Self {
            m: 16,
            ef_construction: 200,
            ef_search: 50,
            quantize: false,
            quantization_bits: 8,
        }
    }
}

impl HnswConfig {
    /// Create HNSW config optimized for speed
    pub fn fast() -> Self {
        Self {
            m: 8,
            ef_construction: 100,
            ef_search: 20,
            quantize: true,
            quantization_bits: 8,
        }
    }

    /// Create HNSW config optimized for accuracy
    pub fn accurate() -> Self {
        Self {
            m: 32,
            ef_construction: 400,
            ef_search: 100,
            quantize: false,
            quantization_bits: 8,
        }
    }

    /// Create HNSW config for large scale
    pub fn large_scale() -> Self {
        Self {
            m: 24,
            ef_construction: 200,
            ef_search: 50,
            quantize: true,
            quantization_bits: 8,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  IVF CONFIGURATION
// ═══════════════════════════════════════════════════════════════════════════════

/// IVF index configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IvfConfig {
    /// Number of clusters (centroids)
    pub nlist: usize,
    /// Number of clusters to probe during search
    pub nprobe: usize,
    /// Number of iterations for k-means training
    pub niter: usize,
    /// Use accelerated k-means
    pub accelerated: bool,
}

impl Default for IvfConfig {
    fn default() -> Self {
        Self {
            nlist: 100,
            nprobe: 10,
            niter: 20,
            accelerated: true,
        }
    }
}

impl IvfConfig {
    /// Create IVF config for small datasets (< 100K)
    pub fn small() -> Self {
        Self {
            nlist: 50,
            nprobe: 10,
            niter: 20,
            accelerated: true,
        }
    }

    /// Create IVF config for medium datasets (100K - 1M)
    pub fn medium() -> Self {
        Self {
            nlist: 256,
            nprobe: 16,
            niter: 20,
            accelerated: true,
        }
    }

    /// Create IVF config for large datasets (> 1M)
    pub fn large() -> Self {
        Self {
            nlist: 1024,
            nprobe: 32,
            niter: 25,
            accelerated: true,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  PRODUCT QUANTIZATION CONFIGURATION
// ═══════════════════════════════════════════════════════════════════════════════

/// Product Quantization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PqConfig {
    /// Number of sub-vectors
    pub m: usize,
    /// Number of bits per sub-quantizer
    pub nbits: usize,
    /// Use IVF+PQ
    pub use_ivf: bool,
    /// IVF config if using IVF+PQ
    pub ivf: Option<IvfConfig>,
}

impl Default for PqConfig {
    fn default() -> Self {
        Self {
            m: 8,
            nbits: 8,
            use_ivf: false,
            ivf: None,
        }
    }
}

impl PqConfig {
    /// Create PQ config for 1536-dimensional vectors (OpenAI)
    pub fn for_openai() -> Self {
        Self {
            m: 48, // 1536 / 32 = 48 sub-vectors of 32 dims
            nbits: 8,
            use_ivf: false,
            ivf: None,
        }
    }

    /// Create PQ config for 768-dimensional vectors (BERT)
    pub fn for_bert() -> Self {
        Self {
            m: 24, // 768 / 32 = 24 sub-vectors
            nbits: 8,
            use_ivf: false,
            ivf: None,
        }
    }

    /// Create IVF+PQ config
    pub fn with_ivf(mut self, ivf_config: IvfConfig) -> Self {
        self.use_ivf = true;
        self.ivf = Some(ivf_config);
        self
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  LSH CONFIGURATION
// ═══════════════════════════════════════════════════════════════════════════════

/// LSH index configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LshConfig {
    /// Number of hash tables
    pub num_tables: usize,
    /// Number of hash functions per table
    pub num_hashes: usize,
    /// Bucket width (w parameter)
    pub bucket_width: f32,
    /// Random seed for hash functions
    pub seed: u64,
}

impl Default for LshConfig {
    fn default() -> Self {
        Self {
            num_tables: 10,
            num_hashes: 12,
            bucket_width: 3.0,
            seed: 42,
        }
    }
}

impl LshConfig {
    /// Create LSH config for high recall
    pub fn high_recall() -> Self {
        Self {
            num_tables: 20,
            num_hashes: 10,
            bucket_width: 3.0,
            seed: 42,
        }
    }

    /// Create LSH config for speed
    pub fn fast() -> Self {
        Self {
            num_tables: 5,
            num_hashes: 16,
            bucket_width: 4.0,
            seed: 42,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  INDEX STATISTICS
// ═══════════════════════════════════════════════════════════════════════════════

/// Index statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexStats {
    /// Index type
    pub index_type: String,
    /// Number of vectors
    pub vector_count: usize,
    /// Vector dimension
    pub dimension: usize,
    /// Memory usage in bytes
    pub memory_bytes: usize,
    /// Build time in milliseconds
    pub build_time_ms: u64,
    /// Average search latency in microseconds
    pub avg_search_latency_us: f64,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  INDEX RECOMMENDATIONS
// ═══════════════════════════════════════════════════════════════════════════════

/// Recommend index type based on data characteristics
pub fn recommend_index(vector_count: usize, dimension: usize, memory_budget_mb: usize) -> IndexType {
    let vector_size = vector_count * dimension * 4; // 4 bytes per f32

    if vector_count < 10_000 {
        // Small dataset: use flat index
        IndexType::Flat
    } else if vector_count < 100_000 {
        // Medium dataset: use HNSW
        IndexType::Hnsw(HnswConfig::default())
    } else if memory_budget_mb * 1024 * 1024 < vector_size / 2 {
        // Memory constrained: use PQ
        let m = if dimension >= 768 { dimension / 32 } else { dimension / 8 };
        IndexType::ProductQuantization(PqConfig {
            m,
            nbits: 8,
            use_ivf: vector_count > 1_000_000,
            ivf: if vector_count > 1_000_000 { Some(IvfConfig::large()) } else { None },
        })
    } else if vector_count > 1_000_000 {
        // Large dataset: use IVF
        IndexType::Ivf(IvfConfig::large())
    } else {
        // Default: HNSW
        IndexType::Hnsw(HnswConfig::large_scale())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recommend_index() {
        // Small dataset
        let index = recommend_index(5_000, 1536, 1024);
        assert!(matches!(index, IndexType::Flat));

        // Medium dataset
        let index = recommend_index(50_000, 1536, 1024);
        assert!(matches!(index, IndexType::Hnsw(_)));

        // Large dataset (memory constrained will return PQ, otherwise IVF)
        let index = recommend_index(2_000_000, 1536, 1024);
        assert!(matches!(index, IndexType::Ivf(_) | IndexType::ProductQuantization(_)));

        // Memory constrained
        let index = recommend_index(500_000, 1536, 100);
        assert!(matches!(index, IndexType::ProductQuantization(_)));
    }

    #[test]
    fn test_hnsw_configs() {
        let fast = HnswConfig::fast();
        assert!(fast.m < 16);

        let accurate = HnswConfig::accurate();
        assert!(accurate.m > 16);
    }
}
