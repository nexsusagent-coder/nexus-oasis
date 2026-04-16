// ═══════════════════════════════════════════════════════════════════════════════
//  SENTIENT OS - Image Generation Types
// ═══════════════════════════════════════════════════════════════════════════════

use serde::{Deserialize, Serialize};

/// Image generation request
#[derive(Debug, Clone, Serialize)]
pub struct ImageRequest {
    /// Text prompt
    pub prompt: String,
    /// Model to use
    pub model: String,
    /// Image size
    pub size: ImageSize,
    /// Number of images (1-4)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u8>,
    /// Quality (for DALL-E)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<ImageQuality>,
    /// Style (for DALL-E)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<ImageStyle>,
    /// Response format (url or b64_json)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<String>,
    /// Seed for reproducibility
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<u64>,
    /// Negative prompt (for SD)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    /// Steps (for SD)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steps: Option<u32>,
    /// CFG scale (for SD)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cfg_scale: Option<f32>,
}

impl ImageRequest {
    /// Create new request
    pub fn new(prompt: impl Into<String>, model: impl Into<String>) -> Self {
        Self {
            prompt: prompt.into(),
            model: model.into(),
            size: ImageSize::Square1024,
            n: Some(1),
            quality: None,
            style: None,
            response_format: None,
            seed: None,
            negative_prompt: None,
            steps: None,
            cfg_scale: None,
        }
    }

    /// DALL-E 3 request
    pub fn dalle3(prompt: impl Into<String>) -> Self {
        Self::new(prompt, "dall-e-3")
            .with_quality(ImageQuality::Standard)
            .with_style(ImageStyle::Vivid)
    }

    /// DALL-E 2 request
    pub fn dalle2(prompt: impl Into<String>) -> Self {
        Self::new(prompt, "dall-e-2")
    }

    /// Stable Diffusion XL request
    pub fn sdxl(prompt: impl Into<String>) -> Self {
        Self::new(prompt, "stable-diffusion-xl-1024-v1-0")
            .with_steps(30)
            .with_cfg_scale(7.0)
    }

    /// Flux Pro request
    pub fn flux_pro(prompt: impl Into<String>) -> Self {
        Self::new(prompt, "flux-pro")
    }

    /// DALL-E 3 HD request
    pub fn dalle3_hd(prompt: impl Into<String>) -> Self {
        Self::new(prompt, "dall-e-3")
            .with_quality(ImageQuality::HD)
            .with_style(ImageStyle::Vivid)
    }

    /// GPT Image 1 (2026) — OpenAI's native image model
    pub fn gpt_image_1(prompt: impl Into<String>) -> Self {
        Self::new(prompt, "gpt-image-1")
    }

    /// Ideogram v2 request
    pub fn ideogram_v2(prompt: impl Into<String>) -> Self {
        Self::new(prompt, "ideogram-v2")
    }

    /// Ideogram v2 Turbo request (faster)
    pub fn ideogram_v2_turbo(prompt: impl Into<String>) -> Self {
        Self::new(prompt, "ideogram-v2-turbo")
    }

    /// Ideogram v3 (2026) — latest Ideogram
    pub fn ideogram_v3(prompt: impl Into<String>) -> Self {
        Self::new(prompt, "ideogram-v3")
    }

    /// Flux 1.1 Pro (2026) — improved Flux
    pub fn flux_11_pro(prompt: impl Into<String>) -> Self {
        Self::new(prompt, "flux-1.1-pro")
            .with_steps(28)
            .with_cfg_scale(3.5)
    }

    /// Flux Dev request (open weights)
    pub fn flux_dev(prompt: impl Into<String>) -> Self {
        Self::new(prompt, "flux-dev")
            .with_steps(28)
            .with_cfg_scale(3.5)
    }

    /// Flux Schnell request (fastest Flux)
    pub fn flux_schnell(prompt: impl Into<String>) -> Self {
        Self::new(prompt, "flux-schnell")
            .with_steps(4)
    }

    /// Stable Diffusion 3.5 (2026)
    pub fn sd35(prompt: impl Into<String>) -> Self {
        Self::new(prompt, "stable-diffusion-3.5-large")
            .with_steps(30)
            .with_cfg_scale(7.0)
    }

    /// Stable Diffusion 3.5 Turbo (faster)
    pub fn sd35_turbo(prompt: impl Into<String>) -> Self {
        Self::new(prompt, "stable-diffusion-3.5-large-turbo")
            .with_steps(12)
            .with_cfg_scale(7.0)
    }

    /// Playground v3 (2026)
    pub fn playground_v3(prompt: impl Into<String>) -> Self {
        Self::new(prompt, "playground-v3")
    }

    /// Recraft V3 (2026) — vector & brand design
    pub fn recraft_v3(prompt: impl Into<String>) -> Self {
        Self::new(prompt, "recraft-v3")
    }

    /// Imagen 3 (Google, 2026)
    pub fn imagen3(prompt: impl Into<String>) -> Self {
        Self::new(prompt, "imagen-3")
    }

    /// Midjourney API (via Replicate)
    pub fn midjourney(prompt: impl Into<String>) -> Self {
        Self::new(prompt, "midjourney-v6.1")
    }

    /// Set size
    pub fn with_size(mut self, size: ImageSize) -> Self {
        self.size = size;
        self
    }

    /// Set quality
    pub fn with_quality(mut self, quality: ImageQuality) -> Self {
        self.quality = Some(quality);
        self
    }

    /// Set style
    pub fn with_style(mut self, style: ImageStyle) -> Self {
        self.style = Some(style);
        self
    }

    /// Set number of images
    pub fn with_n(mut self, n: u8) -> Self {
        self.n = Some(n.min(4).max(1));
        self
    }

    /// Set seed
    pub fn with_seed(mut self, seed: u64) -> Self {
        self.seed = Some(seed);
        self
    }

    /// Set negative prompt
    pub fn with_negative_prompt(mut self, prompt: impl Into<String>) -> Self {
        self.negative_prompt = Some(prompt.into());
        self
    }

    /// Set steps
    pub fn with_steps(mut self, steps: u32) -> Self {
        self.steps = Some(steps);
        self
    }

    /// Set CFG scale
    pub fn with_cfg_scale(mut self, scale: f32) -> Self {
        self.cfg_scale = Some(scale);
        self
    }

    /// Request base64 response
    pub fn as_base64(mut self) -> Self {
        self.response_format = Some("b64_json".to_string());
        self
    }
}

/// Image size
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImageSize {
    #[serde(rename = "256x256")]
    Small256,
    #[serde(rename = "512x512")]
    Medium512,
    #[serde(rename = "1024x1024")]
    Square1024,
    #[serde(rename = "1792x1024")]
    Landscape1792,
    #[serde(rename = "1024x1792")]
    Portrait1024,
    #[serde(rename = "1280x720")]
    HD1280,
    #[serde(rename = "1920x1080")]
    FullHD1920,
    #[serde(rename = "2048x2048")]
    Square2048,
    #[serde(rename = "1536x1024")]
    Landscape1536,
    #[serde(rename = "1024x1536")]
    Portrait1536,
}

impl ImageSize {
    pub fn dimensions(&self) -> (u32, u32) {
        match self {
            Self::Small256 => (256, 256),
            Self::Medium512 => (512, 512),
            Self::Square1024 => (1024, 1024),
            Self::Landscape1792 => (1792, 1024),
            Self::Portrait1024 => (1024, 1792),
            Self::HD1280 => (1280, 720),
            Self::FullHD1920 => (1920, 1080),
            Self::Square2048 => (2048, 2048),
            Self::Landscape1536 => (1536, 1024),
            Self::Portrait1536 => (1024, 1536),
        }
    }

    pub fn width(&self) -> u32 {
        self.dimensions().0
    }

    pub fn height(&self) -> u32 {
        self.dimensions().1
    }

    pub fn is_square(&self) -> bool {
        self.width() == self.height()
    }

    pub fn is_landscape(&self) -> bool {
        self.width() > self.height()
    }

    pub fn is_portrait(&self) -> bool {
        self.height() > self.width()
    }

    /// Convert to string for API
    pub fn to_string_api(&self) -> String {
        let (w, h) = self.dimensions();
        format!("{}x{}", w, h)
    }
}

/// Image quality (DALL-E 3)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImageQuality {
    #[serde(rename = "standard")]
    Standard,
    #[serde(rename = "hd")]
    HD,
}

/// Image style (DALL-E 3)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImageStyle {
    #[serde(rename = "vivid")]
    Vivid,
    #[serde(rename = "natural")]
    Natural,
}

/// Image generation response
#[derive(Debug, Clone, Deserialize)]
pub struct ImageResponse {
    /// Created timestamp
    pub created: i64,
    /// Generated images
    pub data: Vec<ImageDataResponse>,
}

/// Single image in response
#[derive(Debug, Clone, Deserialize)]
pub struct ImageDataResponse {
    /// URL or base64
    pub url: Option<String>,
    /// Base64 JSON
    pub b64_json: Option<String>,
    /// Revised prompt
    pub revised_prompt: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_request() {
        let req = ImageRequest::dalle3("A cat")
            .with_size(ImageSize::Square1024)
            .with_quality(ImageQuality::HD);

        assert_eq!(req.prompt, "A cat");
        assert_eq!(req.model, "dall-e-3");
        assert_eq!(req.quality, Some(ImageQuality::HD));
    }

    #[test]
    fn test_image_size() {
        let size = ImageSize::Square1024;
        assert_eq!(size.dimensions(), (1024, 1024));
        assert!(size.is_square());
        assert!(!size.is_landscape());

        let landscape = ImageSize::Landscape1792;
        assert!(landscape.is_landscape());
    }

    #[test]
    fn test_sdxl_request() {
        let req = ImageRequest::sdxl("A sunset")
            .with_negative_prompt("blurry")
            .with_steps(50);

        assert_eq!(req.model, "stable-diffusion-xl-1024-v1-0");
        assert_eq!(req.negative_prompt, Some("blurry".to_string()));
        assert_eq!(req.steps, Some(50));
    }
}
