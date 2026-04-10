//! Vision providers for AI-based image analysis

use crate::types::*;
use crate::{Result, VisionError};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Vision provider trait
#[async_trait]
pub trait VisionProvider: Send + Sync {
    /// Get provider name
    fn name(&self) -> &str;

    /// Analyze an image
    async fn analyze(&self, image: &[u8], options: &VisionOptions) -> Result<ImageAnalysis>;

    /// Describe an image
    async fn describe(&self, image: &[u8], prompt: Option<&str>) -> Result<ImageDescription>;

    /// Answer questions about an image
    async fn answer_question(&self, image: &[u8], question: &str) -> Result<String>;

    /// Check if provider is available
    fn is_available(&self) -> bool {
        true
    }

    /// Get supported features
    fn supported_features(&self) -> Vec<Feature> {
        vec![Feature::Description, Feature::QuestionAnswering]
    }
}

/// Vision features
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Feature {
    Description,
    QuestionAnswering,
    ObjectDetection,
    FaceDetection,
    Ocr,
    Embedding,
    Segmentation,
}

/// OpenAI Vision provider
#[cfg(feature = "api")]
pub struct OpenAIVision {
    api_key: Option<String>,
    model: String,
    client: reqwest::Client,
}

#[cfg(feature = "api")]
impl OpenAIVision {
    pub fn new(api_key: Option<String>) -> Self {
        Self {
            api_key,
            model: "gpt-4o".to_string(),
            client: reqwest::Client::new(),
        }
    }

    pub fn with_model(mut self, model: impl Into<String>) -> Self {
        self.model = model.into();
        self
    }

    fn build_message(&self, prompt: &str, image_base64: &str) -> serde_json::Value {
        serde_json::json!({
            "model": self.model,
            "messages": [{
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": prompt
                    },
                    {
                        "type": "image_url",
                        "image_url": {
                            "url": format!("data:image/jpeg;base64,{}", image_base64)
                        }
                    }
                ]
            }],
            "max_tokens": 1000
        })
    }
}

#[cfg(feature = "api")]
#[async_trait]
impl VisionProvider for OpenAIVision {
    fn name(&self) -> &str {
        "openai"
    }

    async fn analyze(&self, image: &[u8], options: &VisionOptions) -> Result<ImageAnalysis> {
        let description = self.describe(image, options.prompt.as_deref()).await?;

        Ok(ImageAnalysis {
            description: Some(description),
            objects: Vec::new(),
            faces: Vec::new(),
            text: None,
            colors: Vec::new(),
            category: None,
        })
    }

    async fn describe(&self, image: &[u8], prompt: Option<&str>) -> Result<ImageDescription> {
        let api_key = self.api_key.as_ref()
            .ok_or_else(|| VisionError::Auth("OpenAI API key not set".to_string()))?;

        let image_base64 = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, image);
        let prompt_text = prompt.unwrap_or("Describe this image in detail.");

        let body = self.build_message(prompt_text, &image_base64);

        let response = self.client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        if !response.status().is_success() {
            let error = response.text().await.unwrap_or_default();
            return Err(VisionError::api(format!("OpenAI API error: {}", error)));
        }

        let json: serde_json::Value = response.json().await?;
        let description = json["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("No description available")
            .to_string();

        Ok(ImageDescription::new(description))
    }

    async fn answer_question(&self, image: &[u8], question: &str) -> Result<String> {
        let description = self.describe(image, Some(question)).await?;
        Ok(description.description)
    }

    fn is_available(&self) -> bool {
        self.api_key.is_some()
    }
}

/// Anthropic Claude Vision provider
#[cfg(feature = "api")]
pub struct ClaudeVision {
    api_key: Option<String>,
    model: String,
    client: reqwest::Client,
}

#[cfg(feature = "api")]
impl ClaudeVision {
    pub fn new(api_key: Option<String>) -> Self {
        Self {
            api_key,
            model: "claude-3-sonnet-20240229".to_string(),
            client: reqwest::Client::new(),
        }
    }

    pub fn with_model(mut self, model: impl Into<String>) -> Self {
        self.model = model.into();
        self
    }
}

#[cfg(feature = "api")]
#[async_trait]
impl VisionProvider for ClaudeVision {
    fn name(&self) -> &str {
        "claude"
    }

    async fn analyze(&self, image: &[u8], options: &VisionOptions) -> Result<ImageAnalysis> {
        let description = self.describe(image, options.prompt.as_deref()).await?;

        Ok(ImageAnalysis {
            description: Some(description),
            objects: Vec::new(),
            faces: Vec::new(),
            text: None,
            colors: Vec::new(),
            category: None,
        })
    }

    async fn describe(&self, image: &[u8], prompt: Option<&str>) -> Result<ImageDescription> {
        let api_key = self.api_key.as_ref()
            .ok_or_else(|| VisionError::Auth("Claude API key not set".to_string()))?;

        let image_base64 = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, image);
        let prompt_text = prompt.unwrap_or("Describe this image in detail.");

        let body = serde_json::json!({
            "model": self.model,
            "max_tokens": 1024,
            "messages": [{
                "role": "user",
                "content": [
                    {
                        "type": "image",
                        "source": {
                            "type": "base64",
                            "media_type": "image/jpeg",
                            "data": image_base64
                        }
                    },
                    {
                        "type": "text",
                        "text": prompt_text
                    }
                ]
            }]
        });

        let response = self.client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", api_key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        if !response.status().is_success() {
            let error = response.text().await.unwrap_or_default();
            return Err(VisionError::api(format!("Claude API error: {}", error)));
        }

        let json: serde_json::Value = response.json().await?;
        let description = json["content"][0]["text"]
            .as_str()
            .unwrap_or("No description available")
            .to_string();

        Ok(ImageDescription::new(description))
    }

    async fn answer_question(&self, image: &[u8], question: &str) -> Result<String> {
        let description = self.describe(image, Some(question)).await?;
        Ok(description.description)
    }

    fn is_available(&self) -> bool {
        self.api_key.is_some()
    }
}

/// Local vision model (stub for ONNX-based models)
pub struct LocalVision {
    model_path: Option<String>,
}

impl LocalVision {
    pub fn new() -> Self {
        Self { model_path: None }
    }

    pub fn with_model(mut self, path: impl Into<String>) -> Self {
        self.model_path = Some(path.into());
        self
    }
}

impl Default for LocalVision {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl VisionProvider for LocalVision {
    fn name(&self) -> &str {
        "local"
    }

    async fn analyze(&self, _image: &[u8], _options: &VisionOptions) -> Result<ImageAnalysis> {
        // Stub implementation
        tracing::warn!("LocalVision is a stub - returning empty analysis");

        Ok(ImageAnalysis::default())
    }

    async fn describe(&self, _image: &[u8], _prompt: Option<&str>) -> Result<ImageDescription> {
        // Stub implementation
        Ok(ImageDescription::new("Local vision not implemented"))
    }

    async fn answer_question(&self, _image: &[u8], _question: &str) -> Result<String> {
        Ok("Local vision not implemented".to_string())
    }

    fn is_available(&self) -> bool {
        false
    }

    fn supported_features(&self) -> Vec<Feature> {
        vec![]
    }
}

/// Vision manager
pub struct VisionManager {
    providers: HashMap<String, Box<dyn VisionProvider>>,
    default_provider: Option<String>,
}

impl VisionManager {
    /// Create new vision manager
    pub fn new() -> Self {
        let mut manager = Self {
            providers: HashMap::new(),
            default_provider: None,
        };

        // Register local vision as fallback
        manager.register("local", Box::new(LocalVision::new()));
        manager.default_provider = Some("local".to_string());

        manager
    }

    /// Register a provider
    pub fn register(&mut self, name: &str, provider: Box<dyn VisionProvider>) {
        self.providers.insert(name.to_string(), provider);
    }

    /// Set default provider
    pub fn set_default(&mut self, name: &str) -> Result<()> {
        if self.providers.contains_key(name) {
            self.default_provider = Some(name.to_string());
            Ok(())
        } else {
            Err(VisionError::provider_not_available(name))
        }
    }

    /// Analyze image with default provider
    pub async fn analyze(&self, image: &[u8], options: &VisionOptions) -> Result<ImageAnalysis> {
        let provider_name = self.default_provider.as_ref()
            .ok_or_else(|| VisionError::config("No default vision provider set"))?;

        self.analyze_with(provider_name, image, options).await
    }

    /// Analyze image with specific provider
    pub async fn analyze_with(&self, provider_name: &str, image: &[u8], options: &VisionOptions) -> Result<ImageAnalysis> {
        let provider = self.providers.get(provider_name)
            .ok_or_else(|| VisionError::provider_not_available(provider_name))?;

        provider.analyze(image, options).await
    }

    /// Describe image
    pub async fn describe(&self, image: &[u8], prompt: Option<&str>) -> Result<ImageDescription> {
        let provider_name = self.default_provider.as_ref()
            .ok_or_else(|| VisionError::config("No default vision provider set"))?;

        let provider = self.providers.get(provider_name)
            .ok_or_else(|| VisionError::provider_not_available(provider_name))?;

        provider.describe(image, prompt).await
    }

    /// Answer question about image
    pub async fn answer_question(&self, image: &[u8], question: &str) -> Result<String> {
        let provider_name = self.default_provider.as_ref()
            .ok_or_else(|| VisionError::config("No default vision provider set"))?;

        let provider = self.providers.get(provider_name)
            .ok_or_else(|| VisionError::provider_not_available(provider_name))?;

        provider.answer_question(image, question).await
    }

    /// List available providers
    pub fn list_providers(&self) -> Vec<&str> {
        self.providers.keys().map(String::as_str).collect()
    }

    /// Get provider
    pub fn get(&self, name: &str) -> Option<&dyn VisionProvider> {
        self.providers.get(name).map(|p| p.as_ref())
    }
}

impl Default for VisionManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vision_manager_creation() {
        let manager = VisionManager::new();
        assert!(manager.list_providers().contains(&"local"));
    }

    #[test]
    fn test_local_vision() {
        let vision = LocalVision::new();
        assert_eq!(vision.name(), "local");
        assert!(!vision.is_available());
    }

    #[test]
    fn test_feature() {
        let features = vec![Feature::Description, Feature::ObjectDetection];
        assert_eq!(features.len(), 2);
    }
}
