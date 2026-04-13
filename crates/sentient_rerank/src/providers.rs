//! ─── Reranker Providers ───

use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::{
    Reranker, RerankModel, RerankRequest, RerankResponse, RerankedDocument, RerankDocument,
    RerankError, RerankResult as Result,
};

// ═══════════════════════════════════════════════════════════════════════════════
//  COHERE RERANKER
// ═══════════════════════════════════════════════════════════════════════════════

pub struct CohereReranker {
    api_key: String,
    client: Client,
}

impl CohereReranker {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: Client::new(),
        }
    }
}

#[async_trait]
impl Reranker for CohereReranker {
    fn name(&self) -> &str { "cohere" }

    fn models(&self) -> Vec<RerankModel> {
        vec![
            RerankModel::cohere_rerank_english_v3(),
            RerankModel::cohere_rerank_multilingual_v3(),
        ]
    }

    fn is_configured(&self) -> bool {
        !self.api_key.is_empty()
    }

    async fn rerank(&self, request: RerankRequest) -> Result<RerankResponse> {
        #[derive(Serialize)]
        struct CohereRequest {
            query: String,
            documents: Vec<String>,
            model: String,
            top_n: usize,
            #[serde(skip_serializing_if = "Option::is_none")]
            max_chunks_per_doc: Option<usize>,
        }

        #[derive(Deserialize)]
        struct CohereResponse {
            results: Vec<CohereResult>,
            meta: CohereMeta,
        }

        #[derive(Deserialize)]
        struct CohereResult {
            index: usize,
            relevance_score: f32,
            document: Option<CohereDoc>,
        }

        #[derive(Deserialize)]
        struct CohereDoc {
            text: String,
        }

        #[derive(Deserialize)]
        struct CohereMeta {
            billed_units: BilledUnits,
        }

        #[derive(Deserialize)]
        struct BilledUnits {
            search_units: usize,
        }

        let model = request.model.unwrap_or_else(|| "rerank-english-v3.0".into());
        let documents: Vec<String> = request.documents.iter().map(|d| d.text.clone()).collect();
        
        let req = CohereRequest {
            query: request.query,
            documents,
            model: model.clone(),
            top_n: request.top_n,
            max_chunks_per_doc: request.max_chunks_per_doc,
        };

        let response = self.client
            .post("https://api.cohere.ai/v1/rerank")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&req)
            .send()
            .await
            .map_err(|e| RerankError::RequestFailed(e.to_string()))?;

        if !response.status().is_success() {
            let text = response.text().await.unwrap_or_default();
            return Err(RerankError::ApiError(text));
        }

        let result: CohereResponse = response.json().await
            .map_err(|e| RerankError::ParseError(e.to_string()))?;

        let results: Vec<RerankedDocument> = result.results.into_iter()
            .map(|r| {
                let original_doc = request.documents.get(r.index);
                RerankedDocument {
                    index: r.index,
                    id: original_doc.and_then(|d| d.id.clone()),
                    text: r.document.map(|d| d.text).unwrap_or_default(),
                    relevance_score: r.relevance_score,
                    original_score: original_doc.and_then(|d| d.original_score),
                    score_delta: original_doc.and_then(|d| d.original_score)
                        .map(|s| r.relevance_score - s),
                    metadata: original_doc.map(|d| d.metadata.clone()).unwrap_or_default(),
                }
            })
            .collect();

        Ok(RerankResponse {
            results,
            model,
            total_tokens: result.meta.billed_units.search_units * 500, // Estimate
            processing_time_ms: 0,
            cost: result.meta.billed_units.search_units as f64 * 0.002,
        })
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  JINA RERANKER
// ═══════════════════════════════════════════════════════════════════════════════

pub struct JinaReranker {
    api_key: String,
    client: Client,
}

impl JinaReranker {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: Client::new(),
        }
    }
}

#[async_trait]
impl Reranker for JinaReranker {
    fn name(&self) -> &str { "jina" }

    fn models(&self) -> Vec<RerankModel> {
        vec![
            RerankModel::jina_reranker_v1(),
            RerankModel::jina_colbert_v2(),
        ]
    }

    fn is_configured(&self) -> bool {
        !self.api_key.is_empty()
    }

    async fn rerank(&self, request: RerankRequest) -> Result<RerankResponse> {
        #[derive(Serialize)]
        struct JinaRequest {
            query: String,
            documents: Vec<String>,
            model: String,
            top_n: usize,
        }

        #[derive(Deserialize)]
        struct JinaResponse {
            results: Vec<JinaResult>,
            usage: JinaUsage,
        }

        #[derive(Deserialize)]
        struct JinaResult {
            index: usize,
            relevance_score: f32,
            document: JinaDoc,
        }

        #[derive(Deserialize)]
        struct JinaDoc {
            text: String,
        }

        #[derive(Deserialize)]
        struct JinaUsage {
            total_tokens: usize,
        }

        let model = request.model.unwrap_or_else(|| "jina-reranker-v1-base-en".into());
        let documents: Vec<String> = request.documents.iter().map(|d| d.text.clone()).collect();
        
        let req = JinaRequest {
            query: request.query,
            documents,
            model: model.clone(),
            top_n: request.top_n,
        };

        let response = self.client
            .post("https://api.jina.ai/v1/rerank")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&req)
            .send()
            .await
            .map_err(|e| RerankError::RequestFailed(e.to_string()))?;

        if !response.status().is_success() {
            let text = response.text().await.unwrap_or_default();
            return Err(RerankError::ApiError(text));
        }

        let result: JinaResponse = response.json().await
            .map_err(|e| RerankError::ParseError(e.to_string()))?;

        let results: Vec<RerankedDocument> = result.results.into_iter()
            .map(|r| {
                let original_doc = request.documents.get(r.index);
                RerankedDocument {
                    index: r.index,
                    id: original_doc.and_then(|d| d.id.clone()),
                    text: r.document.text,
                    relevance_score: r.relevance_score,
                    original_score: original_doc.and_then(|d| d.original_score),
                    score_delta: original_doc.and_then(|d| d.original_score)
                        .map(|s| r.relevance_score - s),
                    metadata: original_doc.map(|d| d.metadata.clone()).unwrap_or_default(),
                }
            })
            .collect();

        Ok(RerankResponse {
            results,
            model,
            total_tokens: result.usage.total_tokens,
            processing_time_ms: 0,
            cost: (result.usage.total_tokens as f64 / 1000.0) * 0.00002,
        })
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  LOCAL RERANKER (Cross-Encoder)
// ═══════════════════════════════════════════════════════════════════════════════

pub struct LocalReranker {
    model_path: Option<String>,
}

impl LocalReranker {
    pub fn new() -> Self {
        Self { model_path: None }
    }

    pub fn with_model(path: impl Into<String>) -> Self {
        Self { model_path: Some(path.into()) }
    }
}

impl Default for LocalReranker {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Reranker for LocalReranker {
    fn name(&self) -> &str { "local" }

    fn models(&self) -> Vec<RerankModel> {
        vec![
            RerankModel::cross_encoder_msmarco(),
            RerankModel::bge_reranker_large(),
        ]
    }

    fn is_configured(&self) -> bool {
        true
    }

    async fn rerank(&self, request: RerankRequest) -> Result<RerankResponse> {
        // Local reranking using cross-encoder
        // This would integrate with ONNX/TensorFlow for actual inference
        
        log::warn!("LOCAL RERANKER: Using placeholder - integrate cross-encoder model");
        
        // Simple BM25-style scoring as placeholder
        let query_lower = request.query.to_lowercase();
        let query_terms: Vec<&str> = query_lower.split_whitespace().collect();
        
        let mut results: Vec<RerankedDocument> = request.documents.iter()
            .enumerate()
            .map(|(i, doc)| {
                let text_lower = doc.text.to_lowercase();
                let matches = query_terms.iter()
                    .filter(|t| text_lower.contains(*t))
                    .count();
                let score = matches as f32 / query_terms.len().max(1) as f32;
                
                RerankedDocument {
                    index: i,
                    id: doc.id.clone(),
                    text: doc.text.clone(),
                    relevance_score: score,
                    original_score: doc.original_score,
                    score_delta: doc.original_score.map(|s| score - s),
                    metadata: doc.metadata.clone(),
                }
            })
            .collect();
        
        results.sort_by(|a, b| b.relevance_score.partial_cmp(&a.relevance_score).unwrap());
        results.truncate(request.top_n);
        
        Ok(RerankResponse {
            results,
            model: "local-cross-encoder".into(),
            total_tokens: 0,
            processing_time_ms: 0,
            cost: 0.0,
        })
    }
}
