//! ─── Embedding Providers ───

use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::{
    EmbeddingProvider, EmbeddingModel, EmbeddingRequest, EmbeddingResponse, Embedding, EmbeddingInput,
    EmbedError, EmbedResult,
};

// ═══════════════════════════════════════════════════════════════════════════════
//  OPENAI EMBEDDING PROVIDER
// ═══════════════════════════════════════════════════════════════════════════════

pub struct OpenAIEmbedding {
    api_key: String,
    client: Client,
}

impl OpenAIEmbedding {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: Client::new(),
        }
    }
}

#[async_trait]
impl EmbeddingProvider for OpenAIEmbedding {
    fn name(&self) -> &str { "openai" }

    fn models(&self) -> Vec<EmbeddingModel> {
        vec![
            EmbeddingModel::text_embedding_3_large(),
            EmbeddingModel::text_embedding_3_small(),
            EmbeddingModel::ada_002(),
        ]
    }

    fn is_configured(&self) -> bool {
        !self.api_key.is_empty()
    }

    async fn embed(&self, request: EmbeddingRequest) -> EmbedResult<EmbeddingResponse> {
        #[derive(Serialize)]
        struct OpenAIRequest {
            input: EmbeddingInput,
            model: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            dimensions: Option<usize>,
        }

        #[derive(Deserialize)]
        struct OpenAIResponse {
            data: Vec<OpenAIData>,
            model: String,
            usage: OpenAIUsage,
        }

        #[derive(Deserialize)]
        struct OpenAIData {
            embedding: Vec<f32>,
            index: usize,
        }

        #[derive(Deserialize)]
        struct OpenAIUsage {
            total_tokens: usize,
        }

        let req = OpenAIRequest {
            input: request.input,
            model: request.model,
            dimensions: request.dimensions,
        };

        let response = self.client
            .post("https://api.openai.com/v1/embeddings")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&req)
            .send()
            .await
            .map_err(|e| EmbedError::RequestFailed(e.to_string()))?;

        if !response.status().is_success() {
            let text = response.text().await.unwrap_or_default();
            return Err(EmbedError::ApiError(text));
        }

        let result: OpenAIResponse = response.json().await
            .map_err(|e| EmbedError::ParseError(e.to_string()))?;

        let data_len = result.data.len();
        let embeddings: Vec<Embedding> = result.data.into_iter()
            .map(|d| Embedding {
                vector: d.embedding,
                model: result.model.clone(),
                tokens: result.usage.total_tokens / data_len.max(1),
                index: d.index,
                text: None,
            })
            .collect();

        let model = EmbeddingModel::all().into_iter()
            .find(|m| m.id == result.model)
            .unwrap_or(EmbeddingModel::text_embedding_3_small());

        Ok(EmbeddingResponse {
            embeddings,
            model: result.model,
            total_tokens: result.usage.total_tokens,
            cost: (result.usage.total_tokens as f64 / 1000.0) * model.cost_per_1k,
            processing_time_ms: 0,
        })
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  COHERE EMBEDDING PROVIDER
// ═══════════════════════════════════════════════════════════════════════════════

pub struct CohereEmbedding {
    api_key: String,
    client: Client,
}

impl CohereEmbedding {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: Client::new(),
        }
    }
}

#[async_trait]
impl EmbeddingProvider for CohereEmbedding {
    fn name(&self) -> &str { "cohere" }

    fn models(&self) -> Vec<EmbeddingModel> {
        vec![
            EmbeddingModel::cohere_english_v3(),
            EmbeddingModel::cohere_multilingual_v3(),
        ]
    }

    fn is_configured(&self) -> bool {
        !self.api_key.is_empty()
    }

    async fn embed(&self, request: EmbeddingRequest) -> EmbedResult<EmbeddingResponse> {
        #[derive(Serialize)]
        struct CohereRequest {
            texts: Vec<String>,
            model: String,
            input_type: String,
            truncate: String,
        }

        #[derive(Deserialize)]
        struct CohereResponse {
            embeddings: Vec<Vec<f32>>,
            meta: CohereMeta,
        }

        #[derive(Deserialize)]
        struct CohereMeta {
            billed_units: CohereBilledUnits,
        }

        #[derive(Deserialize)]
        struct CohereBilledUnits {
            input_tokens: usize,
        }

        let texts = request.input.texts().into_iter().map(|s| s.to_string()).collect();
        
        let req = CohereRequest {
            texts,
            model: request.model,
            input_type: "search_document".into(),
            truncate: "END".into(),
        };

        let response = self.client
            .post("https://api.cohere.ai/v1/embed")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&req)
            .send()
            .await
            .map_err(|e| EmbedError::RequestFailed(e.to_string()))?;

        if !response.status().is_success() {
            let text = response.text().await.unwrap_or_default();
            return Err(EmbedError::ApiError(text));
        }

        let result: CohereResponse = response.json().await
            .map_err(|e| EmbedError::ParseError(e.to_string()))?;

        let embeddings_len = result.embeddings.len();
        let embeddings: Vec<Embedding> = result.embeddings.into_iter()
            .enumerate()
            .map(|(i, vec)| Embedding {
                vector: vec,
                model: "cohere".into(),
                tokens: result.meta.billed_units.input_tokens / embeddings_len.max(1),
                index: i,
                text: None,
            })
            .collect();

        Ok(EmbeddingResponse {
            embeddings,
            model: "embed-v3".into(),
            total_tokens: result.meta.billed_units.input_tokens,
            cost: (result.meta.billed_units.input_tokens as f64 / 1000.0) * 0.0001,
            processing_time_ms: 0,
        })
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  VOYAGE EMBEDDING PROVIDER
// ═══════════════════════════════════════════════════════════════════════════════

pub struct VoyageEmbedding {
    api_key: String,
    client: Client,
}

impl VoyageEmbedding {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: Client::new(),
        }
    }
}

#[async_trait]
impl EmbeddingProvider for VoyageEmbedding {
    fn name(&self) -> &str { "voyage" }

    fn models(&self) -> Vec<EmbeddingModel> {
        vec![
            EmbeddingModel::voyage_large_2(),
            EmbeddingModel::voyage_code_2(),
        ]
    }

    fn is_configured(&self) -> bool {
        !self.api_key.is_empty()
    }

    async fn embed(&self, request: EmbeddingRequest) -> EmbedResult<EmbeddingResponse> {
        #[derive(Serialize)]
        struct VoyageRequest {
            input: EmbeddingInput,
            model: String,
        }

        #[derive(Deserialize)]
        struct VoyageResponse {
            data: Vec<VoyageData>,
            usage: VoyageUsage,
        }

        #[derive(Deserialize)]
        struct VoyageData {
            embedding: Vec<f32>,
            index: usize,
        }

        #[derive(Deserialize)]
        struct VoyageUsage {
            total_tokens: usize,
        }

        let req = VoyageRequest {
            input: request.input,
            model: request.model,
        };

        let response = self.client
            .post("https://api.voyageai.com/v1/embeddings")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&req)
            .send()
            .await
            .map_err(|e| EmbedError::RequestFailed(e.to_string()))?;

        if !response.status().is_success() {
            let text = response.text().await.unwrap_or_default();
            return Err(EmbedError::ApiError(text));
        }

        let result: VoyageResponse = response.json().await
            .map_err(|e| EmbedError::ParseError(e.to_string()))?;

        let data_len = result.data.len();
        let embeddings: Vec<Embedding> = result.data.into_iter()
            .map(|d| Embedding {
                vector: d.embedding,
                model: "voyage".into(),
                tokens: result.usage.total_tokens / data_len.max(1),
                index: d.index,
                text: None,
            })
            .collect();

        Ok(EmbeddingResponse {
            embeddings,
            model: "voyage".into(),
            total_tokens: result.usage.total_tokens,
            cost: (result.usage.total_tokens as f64 / 1000.0) * 0.00012,
            processing_time_ms: 0,
        })
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  JINA EMBEDDING PROVIDER
// ═══════════════════════════════════════════════════════════════════════════════

pub struct JinaEmbedding {
    api_key: String,
    client: Client,
}

impl JinaEmbedding {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: Client::new(),
        }
    }
}

#[async_trait]
impl EmbeddingProvider for JinaEmbedding {
    fn name(&self) -> &str { "jina" }

    fn models(&self) -> Vec<EmbeddingModel> {
        vec![EmbeddingModel::jina_v2_base()]
    }

    fn is_configured(&self) -> bool {
        !self.api_key.is_empty()
    }

    async fn embed(&self, request: EmbeddingRequest) -> EmbedResult<EmbeddingResponse> {
        #[derive(Serialize)]
        struct JinaRequest {
            input: EmbeddingInput,
            model: String,
        }

        #[derive(Deserialize)]
        struct JinaResponse {
            data: Vec<JinaData>,
            usage: JinaUsage,
        }

        #[derive(Deserialize)]
        struct JinaData {
            embedding: Vec<f32>,
            index: usize,
        }

        #[derive(Deserialize)]
        struct JinaUsage {
            total_tokens: usize,
        }

        let req = JinaRequest {
            input: request.input,
            model: request.model,
        };

        let response = self.client
            .post("https://api.jina.ai/v1/embeddings")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&req)
            .send()
            .await
            .map_err(|e| EmbedError::RequestFailed(e.to_string()))?;

        if !response.status().is_success() {
            let text = response.text().await.unwrap_or_default();
            return Err(EmbedError::ApiError(text));
        }

        let result: JinaResponse = response.json().await
            .map_err(|e| EmbedError::ParseError(e.to_string()))?;

        let data_len = result.data.len();
        let embeddings: Vec<Embedding> = result.data.into_iter()
            .map(|d| Embedding {
                vector: d.embedding,
                model: "jina".into(),
                tokens: result.usage.total_tokens / data_len.max(1),
                index: d.index,
                text: None,
            })
            .collect();

        Ok(EmbeddingResponse {
            embeddings,
            model: "jina".into(),
            total_tokens: result.usage.total_tokens,
            cost: 0.0, // Jina has free tier
            processing_time_ms: 0,
        })
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  HUGGING FACE EMBEDDING PROVIDER
// ═══════════════════════════════════════════════════════════════════════════════

pub struct HuggingFaceEmbedding {
    api_key: String,
    client: Client,
}

impl HuggingFaceEmbedding {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: Client::new(),
        }
    }
}

#[async_trait]
impl EmbeddingProvider for HuggingFaceEmbedding {
    fn name(&self) -> &str { "huggingface" }

    fn models(&self) -> Vec<EmbeddingModel> {
        vec![
            EmbeddingModel::e5_large_v2(),
            EmbeddingModel::bge_large_en(),
        ]
    }

    fn is_configured(&self) -> bool {
        !self.api_key.is_empty()
    }

    async fn embed(&self, request: EmbeddingRequest) -> EmbedResult<EmbeddingResponse> {
        // HuggingFace inference API
        let url = format!(
            "https://api-inference.huggingface.co/pipeline/feature-extraction/{}",
            request.model
        );

        let texts: Vec<String> = request.input.texts().into_iter().map(|s| s.to_string()).collect();

        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&texts)
            .send()
            .await
            .map_err(|e| EmbedError::RequestFailed(e.to_string()))?;

        if !response.status().is_success() {
            let text = response.text().await.unwrap_or_default();
            return Err(EmbedError::ApiError(text));
        }

        let embeddings_vec: Vec<Vec<f32>> = response.json().await
            .map_err(|e| EmbedError::ParseError(e.to_string()))?;

        let embeddings: Vec<Embedding> = embeddings_vec.into_iter()
            .enumerate()
            .map(|(i, vec)| Embedding {
                vector: vec,
                model: "huggingface".into(),
                tokens: 0,
                index: i,
                text: None,
            })
            .collect();

        Ok(EmbeddingResponse {
            embeddings,
            model: "huggingface".into(),
            total_tokens: 0,
            cost: 0.0,
            processing_time_ms: 0,
        })
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  LOCAL EMBEDDING PROVIDER (Sentence Transformers)
// ═══════════════════════════════════════════════════════════════════════════════

pub struct LocalEmbedding {
    model_path: Option<String>,
}

impl LocalEmbedding {
    pub fn new() -> Self {
        Self { model_path: None }
    }

    pub fn with_model(path: impl Into<String>) -> Self {
        Self { model_path: Some(path.into()) }
    }
}

impl Default for LocalEmbedding {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl EmbeddingProvider for LocalEmbedding {
    fn name(&self) -> &str { "local" }

    fn models(&self) -> Vec<EmbeddingModel> {
        vec![
            EmbeddingModel::sentence_transformers_mpnet(),
            EmbeddingModel::nomic_embed_v1(),
        ]
    }

    fn is_configured(&self) -> bool {
        true // Always available
    }

    async fn embed(&self, request: EmbeddingRequest) -> EmbedResult<EmbeddingResponse> {
        // Local inference - would use ort/candle for actual inference
        // This is a placeholder implementation
        let texts = request.input.texts();
        let dims = 768; // Default dimension
        
        // Generate random embeddings for placeholder
        // In real implementation, would load model and run inference
        let embeddings: Vec<Embedding> = texts.iter()
            .enumerate()
            .map(|(i, _)| Embedding {
                vector: vec![0.0; dims], // Placeholder
                model: "local".into(),
                tokens: 0,
                index: i,
                text: None,
            })
            .collect();

        log::warn!("LOCAL EMBEDDING: Using placeholder embeddings - integrate sentence-transformers for real embeddings");

        Ok(EmbeddingResponse {
            embeddings,
            model: "local".into(),
            total_tokens: 0,
            cost: 0.0,
            processing_time_ms: 0,
        })
    }
}
