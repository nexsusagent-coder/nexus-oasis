//! ─── Video Templates ───
//!
//! Pre-defined templates for common video generation use cases

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{VideoRequest, AspectRatio, VideoResolution, GenerationParams, VideoStyle, CameraMotion};

// ═══════════════════════════════════════════════════════════════════════════════
//  VIDEO TEMPLATE
// ═══════════════════════════════════════════════════════════════════════════════

/// Pre-defined video template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoTemplate {
    /// Template ID
    pub id: String,
    /// Template name
    pub name: String,
    /// Description
    pub description: String,
    /// Prompt template with {placeholders}
    pub prompt_template: String,
    /// Negative prompt (if any)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    /// Default aspect ratio
    pub aspect_ratio: AspectRatio,
    /// Default resolution
    pub resolution: VideoResolution,
    /// Default duration
    pub duration: f32,
    /// Generation parameters
    pub params: GenerationParams,
    /// Style preset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<VideoStyle>,
    /// Camera motion
    #[serde(skip_serializing_if = "Option::is_none")]
    pub camera_motion: Option<CameraMotion>,
    /// Example placeholder values
    pub examples: HashMap<String, String>,
    /// Category
    pub category: TemplateCategory,
}

impl VideoTemplate {
    /// Create a video request from this template
    pub fn create_request(&self, values: &HashMap<&str, &str>) -> VideoRequest {
        let prompt = self.fill_prompt(values);
        
        VideoRequest {
            prompt,
            negative_prompt: self.negative_prompt.clone(),
            image_url: None,
            duration: Some(self.duration),
            aspect_ratio: self.aspect_ratio,
            resolution: self.resolution,
            seed: None,
            model: None,
            style: self.style.clone(),
            camera_motion: self.camera_motion.clone(),
            loop_video: false,
            params: self.params.clone(),
        }
    }

    /// Fill prompt template with values
    pub fn fill_prompt(&self, values: &HashMap<&str, &str>) -> String {
        let mut prompt = self.prompt_template.clone();
        for (key, value) in values {
            prompt = prompt.replace(&format!("{{{}}}", key), value);
        }
        prompt
    }

    /// Preview prompt with example values
    pub fn preview(&self) -> String {
        let values: HashMap<&str, &str> = self.examples
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect();
        self.fill_prompt(&values)
    }
}

/// Template category
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TemplateCategory {
    Marketing,
    Social,
    Education,
    Entertainment,
    Product,
    Animation,
    Nature,
    Abstract,
    Cinematic,
    Custom,
}

// ═══════════════════════════════════════════════════════════════════════════════
//  TEMPLATE LIBRARY
// ═══════════════════════════════════════════════════════════════════════════════

/// Built-in template library
pub struct TemplateLibrary {
    templates: Vec<VideoTemplate>,
}

impl TemplateLibrary {
    /// Create new library with built-in templates
    pub fn new() -> Self {
        Self {
            templates: Self::builtin_templates(),
        }
    }

    /// List all templates
    pub fn list(&self) -> &[VideoTemplate] {
        &self.templates
    }

    /// Get template by ID
    pub fn get(&self, id: &str) -> Option<&VideoTemplate> {
        self.templates.iter().find(|t| t.id == id)
    }

    /// Get templates by category
    pub fn by_category(&self, category: TemplateCategory) -> Vec<&VideoTemplate> {
        self.templates.iter().filter(|t| t.category == category).collect()
    }

    /// Search templates
    pub fn search(&self, query: &str) -> Vec<&VideoTemplate> {
        let query_lower = query.to_lowercase();
        self.templates.iter()
            .filter(|t| {
                t.name.to_lowercase().contains(&query_lower) ||
                t.description.to_lowercase().contains(&query_lower) ||
                t.prompt_template.to_lowercase().contains(&query_lower)
            })
            .collect()
    }

    /// Built-in templates
    fn builtin_templates() -> Vec<VideoTemplate> {
        let mut templates = Vec::new();
        
        // Marketing templates
        templates.push(VideoTemplate {
            id: "product-showcase".into(),
            name: "Product Showcase".into(),
            description: "Showcase a product with dramatic lighting".into(),
            prompt_template: "{product} with dramatic studio lighting, professional product photography, 4k quality".into(),
            negative_prompt: Some("blurry, low quality, amateur".into()),
            aspect_ratio: AspectRatio::Landscape16x9,
            resolution: VideoResolution::FullHD,
            duration: 4.0,
            params: GenerationParams { guidance_scale: 8.0, ..Default::default() },
            style: Some(VideoStyle::Commercial),
            camera_motion: Some(CameraMotion::OrbitLeft),
            examples: HashMap::from([("product".into(), "luxury watch".into())]),
            category: TemplateCategory::Marketing,
        });

        templates.push(VideoTemplate {
            id: "brand-intro".into(),
            name: "Brand Intro".into(),
            description: "Dynamic brand introduction video".into(),
            prompt_template: "Elegant {brand} logo reveal, cinematic animation, particle effects, premium feel".into(),
            negative_prompt: None,
            aspect_ratio: AspectRatio::Landscape16x9,
            resolution: VideoResolution::FullHD,
            duration: 5.0,
            params: GenerationParams::default(),
            style: Some(VideoStyle::Cinematic),
            camera_motion: Some(CameraMotion::ZoomIn),
            examples: HashMap::from([("brand".into(), "TECH CORP".into())]),
            category: TemplateCategory::Marketing,
        });

        // Social media templates
        templates.push(VideoTemplate {
            id: "social-reel".into(),
            name: "Social Media Reel".into(),
            description: "Short vertical video for TikTok/Reels".into(),
            prompt_template: "{action} in slow motion, aesthetic, trending style, viral potential".into(),
            negative_prompt: Some("boring, static, amateur".into()),
            aspect_ratio: AspectRatio::Portrait9x16,
            resolution: VideoResolution::HD,
            duration: 5.0,
            params: GenerationParams { guidance_scale: 7.0, ..Default::default() },
            style: Some(VideoStyle::MusicVideo),
            camera_motion: Some(CameraMotion::DollyIn),
            examples: HashMap::from([("action".into(), "coffee pouring into cup".into())]),
            category: TemplateCategory::Social,
        });

        // Nature templates
        templates.push(VideoTemplate {
            id: "nature-sunset".into(),
            name: "Nature Sunset".into(),
            description: "Beautiful sunset scene".into(),
            prompt_template: "Stunning sunset over {landscape}, golden hour, cinematic, peaceful atmosphere".into(),
            negative_prompt: None,
            aspect_ratio: AspectRatio::Landscape16x9,
            resolution: VideoResolution::FullHD,
            duration: 5.0,
            params: GenerationParams::default(),
            style: Some(VideoStyle::Cinematic),
            camera_motion: Some(CameraMotion::PanRight),
            examples: HashMap::from([("landscape".into(), "mountains".into())]),
            category: TemplateCategory::Nature,
        });

        templates.push(VideoTemplate {
            id: "nature-forest".into(),
            name: "Forest Walk".into(),
            description: "Peaceful forest atmosphere".into(),
            prompt_template: "Walking through a {forest_type} forest, sunlight filtering through trees, peaceful, nature documentary style".into(),
            negative_prompt: Some("scary, dark, horror".into()),
            aspect_ratio: AspectRatio::Landscape16x9,
            resolution: VideoResolution::FullHD,
            duration: 6.0,
            params: GenerationParams::default(),
            style: Some(VideoStyle::Documentary),
            camera_motion: Some(CameraMotion::DollyIn),
            examples: HashMap::from([("forest_type".into(), "bamboo".into())]),
            category: TemplateCategory::Nature,
        });

        // Cinematic templates
        templates.push(VideoTemplate {
            id: "cinematic-scene".into(),
            name: "Cinematic Scene".into(),
            description: "Film-quality cinematic shot".into(),
            prompt_template: "{scene}, 35mm film, cinematic lighting, professional cinematography, movie quality".into(),
            negative_prompt: Some("amateur, shaky, low quality".into()),
            aspect_ratio: AspectRatio::Cinematic2_39_1,
            resolution: VideoResolution::FullHD,
            duration: 5.0,
            params: GenerationParams { guidance_scale: 8.0, ..Default::default() },
            style: Some(VideoStyle::Cinematic),
            camera_motion: Some(CameraMotion::CraneUp),
            examples: HashMap::from([("scene".into(), "detective walking in rain-soaked neon city streets".into())]),
            category: TemplateCategory::Cinematic,
        });

        // Animation templates
        templates.push(VideoTemplate {
            id: "anime-scene".into(),
            name: "Anime Scene".into(),
            description: "Japanese anime style animation".into(),
            prompt_template: "{character} in anime style, {action}, vibrant colors, Japanese animation, high quality".into(),
            negative_prompt: Some("realistic, photorealistic".into()),
            aspect_ratio: AspectRatio::Landscape16x9,
            resolution: VideoResolution::HD,
            duration: 4.0,
            params: GenerationParams::default(),
            style: Some(VideoStyle::Anime),
            camera_motion: None,
            examples: HashMap::from([
                ("character".into(), "young warrior".into()),
                ("action".into(), "drawing sword with determination".into()),
            ]),
            category: TemplateCategory::Animation,
        });

        templates.push(VideoTemplate {
            id: "3d-animation".into(),
            name: "3D Animation".into(),
            description: "Pixar-style 3D animation".into(),
            prompt_template: "{character}, 3D animated, Pixar style, smooth rendering, expressive, family friendly".into(),
            negative_prompt: Some("scary, horror, realistic".into()),
            aspect_ratio: AspectRatio::Landscape16x9,
            resolution: VideoResolution::FullHD,
            duration: 5.0,
            params: GenerationParams::default(),
            style: Some(VideoStyle::ThreeDAnimation),
            camera_motion: Some(CameraMotion::OrbitRight),
            examples: HashMap::from([("character".into(), "cute robot dancing happily".into())]),
            category: TemplateCategory::Animation,
        });

        // Product templates
        templates.push(VideoTemplate {
            id: "product-360".into(),
            name: "Product 360°".into(),
            description: "Rotating product showcase".into(),
            prompt_template: "{product} rotating 360 degrees, studio lighting, white background, professional product video".into(),
            negative_prompt: Some("blurry, shaky, amateur".into()),
            aspect_ratio: AspectRatio::Square,
            resolution: VideoResolution::FullHD,
            duration: 6.0,
            params: GenerationParams::default(),
            style: Some(VideoStyle::Commercial),
            camera_motion: Some(CameraMotion::OrbitLeft),
            examples: HashMap::from([("product".into(), "luxury perfume bottle".into())]),
            category: TemplateCategory::Product,
        });

        // Sci-fi templates
        templates.push(VideoTemplate {
            id: "scifi-city".into(),
            name: "Sci-Fi City".into(),
            description: "Futuristic cityscape".into(),
            prompt_template: "Futuristic {city_type} city, flying vehicles, neon lights, cyberpunk atmosphere, high tech".into(),
            negative_prompt: Some("old, vintage, primitive".into()),
            aspect_ratio: AspectRatio::Landscape16x9,
            resolution: VideoResolution::FullHD,
            duration: 5.0,
            params: GenerationParams::default(),
            style: Some(VideoStyle::SciFi),
            camera_motion: Some(CameraMotion::Drone),
            examples: HashMap::from([("city_type".into(), "nighttime Asian megacity".into())]),
            category: TemplateCategory::Cinematic,
        });

        templates
    }
}

impl Default for TemplateLibrary {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
//  TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_library() {
        let library = TemplateLibrary::new();
        assert!(!library.list().is_empty());
    }

    #[test]
    fn test_get_template() {
        let library = TemplateLibrary::new();
        let template = library.get("product-showcase");
        assert!(template.is_some());
    }

    #[test]
    fn test_create_request() {
        let library = TemplateLibrary::new();
        let template = library.get("nature-sunset").unwrap();
        
        let mut values = HashMap::new();
        values.insert("landscape", "ocean");
        
        let request = template.create_request(&values);
        assert!(request.prompt.contains("ocean"));
    }

    #[test]
    fn test_fill_prompt() {
        let template = VideoTemplate {
            id: "test".into(),
            name: "Test".into(),
            description: "Test template".into(),
            prompt_template: "Hello {name}, welcome to {place}!".into(),
            negative_prompt: None,
            aspect_ratio: AspectRatio::Landscape16x9,
            resolution: VideoResolution::HD,
            duration: 4.0,
            params: GenerationParams::default(),
            style: None,
            camera_motion: None,
            examples: HashMap::new(),
            category: TemplateCategory::Custom,
        };

        let mut values = HashMap::new();
        values.insert("name", "Alice");
        values.insert("place", "SENTIENT OS");

        let result = template.fill_prompt(&values);
        assert_eq!(result, "Hello Alice, welcome to SENTIENT OS!");
    }

    #[test]
    fn test_by_category() {
        let library = TemplateLibrary::new();
        let marketing = library.by_category(TemplateCategory::Marketing);
        assert!(!marketing.is_empty());
    }

    #[test]
    fn test_search() {
        let library = TemplateLibrary::new();
        let results = library.search("sunset");
        assert!(!results.is_empty());
    }

    #[test]
    fn test_preview() {
        let library = TemplateLibrary::new();
        let template = library.get("product-showcase").unwrap();
        let preview = template.preview();
        assert!(preview.contains("luxury watch"));
    }
}
