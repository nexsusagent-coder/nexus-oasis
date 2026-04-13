//! Multimodal embedding support

use crate::types::MultimodalEmbedding;
use crate::{Result, VisionError};
use async_trait::async_trait;
#[allow(unused_imports)]
use serde::Serialize;

/// Embedding provider trait
#[async_trait]
pub trait EmbeddingProvider: Send + Sync {
    /// Get provider name
    fn name(&self) -> &str;

    /// Generate embedding from image
    async fn embed_image(&self, image: &[u8]) -> Result<MultimodalEmbedding>;

    /// Generate embedding from text
    async fn embed_text(&self, text: &str) -> Result<MultimodalEmbedding>;

    /// Get embedding dimension
    fn dimension(&self) -> usize;

    /// Check if provider is available
    fn is_available(&self) -> bool {
        true
    }
}

/// CLIP-style embedding provider
pub struct ClipEmbedding {
    model_name: String,
    dimension: usize,
    #[cfg(feature = "api")]
    client: Option<reqwest::Client>,
    #[cfg(feature = "api")]
    api_key: Option<String>,
}

impl ClipEmbedding {
    pub fn new(model_name: impl Into<String>, dimension: usize) -> Self {
        Self {
            model_name: model_name.into(),
            dimension,
            #[cfg(feature = "api")]
            client: Some(reqwest::Client::new()),
            #[cfg(feature = "api")]
            api_key: None,
        }
    }

    /// OpenAI CLIP-style embeddings
    pub fn openai() -> Self {
        Self::new("text-embedding-3-small", 1536)
    }

    /// Cohere embeddings
    pub fn cohere() -> Self {
        Self::new("embed-multilingual-v3.0", 1024)
    }

    #[cfg(feature = "api")]
    pub fn with_api_key(mut self, key: impl Into<String>) -> Self {
        self.api_key = Some(key.into());
        self
    }
}

#[async_trait]
impl EmbeddingProvider for ClipEmbedding {
    fn name(&self) -> &str {
        &self.model_name
    }

    fn dimension(&self) -> usize {
        self.dimension
    }

    async fn embed_image(&self, _image: &[u8]) -> Result<MultimodalEmbedding> {
        // Stub - would need actual CLIP model or API
        tracing::warn!("ClipEmbedding image embedding is a stub");

        Ok(MultimodalEmbedding::new(
            vec![0.0; self.dimension],
            &self.model_name,
        ))
    }

    async fn embed_text(&self, text: &str) -> Result<MultimodalEmbedding> {
        #[cfg(feature = "api")]
        {
            if let (Some(client), Some(api_key)) = (&self.client, &self.api_key) {
                return self.embed_text_via_api(client, api_key, text).await;
            }
        }

        // Fallback stub
        tracing::warn!("ClipEmbedding text embedding returning stub");

        Ok(MultimodalEmbedding::new(
            vec![0.0; self.dimension],
            &self.model_name,
        ))
    }

    fn is_available(&self) -> bool {
        #[cfg(feature = "api")]
        {
            self.api_key.is_some()
        }
        #[cfg(not(feature = "api"))]
        {
            false
        }
    }
}

#[cfg(feature = "api")]
impl ClipEmbedding {
    async fn embed_text_via_api(
        &self,
        client: &reqwest::Client,
        api_key: &str,
        text: &str,
    ) -> Result<MultimodalEmbedding> {
        let body = serde_json::json!({
            "model": self.model_name,
            "input": text
        });

        let response = client
            .post("https://api.openai.com/v1/embeddings")
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        if !response.status().is_success() {
            let error = response.text().await.unwrap_or_default();
            return Err(VisionError::api(format!("Embedding API error: {}", error)));
        }

        let json: serde_json::Value = response.json().await?;
        let vector: Vec<f32> = json["data"][0]["embedding"]
            .as_array()
            .ok_or_else(|| VisionError::api("Invalid embedding response"))?
            .iter()
            .filter_map(|v| v.as_f64().map(|f| f as f32))
            .collect();

        Ok(MultimodalEmbedding::new(vector, &self.model_name))
    }
}

/// Local embedding model (stub)
pub struct LocalEmbedding {
    dimension: usize,
}

impl LocalEmbedding {
    pub fn new(dimension: usize) -> Self {
        Self { dimension }
    }
}

impl Default for LocalEmbedding {
    fn default() -> Self {
        Self::new(512)
    }
}

#[async_trait]
impl EmbeddingProvider for LocalEmbedding {
    fn name(&self) -> &str {
        "local"
    }

    fn dimension(&self) -> usize {
        self.dimension
    }

    async fn embed_image(&self, _image: &[u8]) -> Result<MultimodalEmbedding> {
        Ok(MultimodalEmbedding::new(
            vec![0.0; self.dimension],
            "local",
        ))
    }

    async fn embed_text(&self, _text: &str) -> Result<MultimodalEmbedding> {
        Ok(MultimodalEmbedding::new(
            vec![0.0; self.dimension],
            "local",
        ))
    }

    fn is_available(&self) -> bool {
        false
    }
}

/// Embedding manager
pub struct EmbeddingManager {
    providers: std::collections::HashMap<String, Box<dyn EmbeddingProvider>>,
    default_provider: Option<String>,
}

impl EmbeddingManager {
    pub fn new() -> Self {
        let mut manager = Self {
            providers: std::collections::HashMap::new(),
            default_provider: None,
        };

        manager.register("local", Box::new(LocalEmbedding::default()));
        manager.default_provider = Some("local".to_string());

        manager
    }

    pub fn register(&mut self, name: &str, provider: Box<dyn EmbeddingProvider>) {
        self.providers.insert(name.to_string(), provider);
    }

    pub fn set_default(&mut self, name: &str) -> Result<()> {
        if self.providers.contains_key(name) {
            self.default_provider = Some(name.to_string());
            Ok(())
        } else {
            Err(VisionError::provider_not_available(name))
        }
    }

    pub async fn embed_image(&self, image: &[u8]) -> Result<MultimodalEmbedding> {
        let provider_name = self.default_provider.as_ref()
            .ok_or_else(|| VisionError::config("No default embedding provider set"))?;

        self.embed_image_with(provider_name, image).await
    }

    pub async fn embed_image_with(&self, provider_name: &str, image: &[u8]) -> Result<MultimodalEmbedding> {
        let provider = self.providers.get(provider_name)
            .ok_or_else(|| VisionError::provider_not_available(provider_name))?;

        provider.embed_image(image).await
    }

    pub async fn embed_text(&self, text: &str) -> Result<MultimodalEmbedding> {
        let provider_name = self.default_provider.as_ref()
            .ok_or_else(|| VisionError::config("No default embedding provider set"))?;

        self.embed_text_with(provider_name, text).await
    }

    pub async fn embed_text_with(&self, provider_name: &str, text: &str) -> Result<MultimodalEmbedding> {
        let provider = self.providers.get(provider_name)
            .ok_or_else(|| VisionError::provider_not_available(provider_name))?;

        provider.embed_text(text).await
    }

    /// Compute similarity between image and text
    pub async fn similarity(&self, image: &[u8], text: &str) -> Result<f32> {
        let image_emb = self.embed_image(image).await?;
        let text_emb = self.embed_text(text).await?;
        Ok(image_emb.cosine_similarity(&text_emb))
    }

    pub fn list_providers(&self) -> Vec<&str> {
        self.providers.keys().map(String::as_str).collect()
    }
}

impl Default for EmbeddingManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_embedding_manager() {
        let manager = EmbeddingManager::new();
        assert!(manager.list_providers().contains(&"local"));
    }

    #[test]
    fn test_local_embedding() {
        let embedding = LocalEmbedding::new(512);
        assert_eq!(embedding.dimension(), 512);
        assert!(!embedding.is_available());
    }

    #[test]
    fn test_multimodal_embedding() {
        let emb1 = MultimodalEmbedding::new(vec![1.0, 0.0, 0.0], "test");
        let emb2 = MultimodalEmbedding::new(vec![0.0, 1.0, 0.0], "test");

        assert!((emb1.cosine_similarity(&emb2) - 0.0).abs() < 0.001);
    }
}
