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
    Transition,
    Background,
    Occasion,
    Food,
    Tech,
    Fitness,
    Fashion,
    Travel,
    RealEstate,
    Automotive,
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

        // === EDUCATION TEMPLATES ===
        templates.push(VideoTemplate {
            id: "edu-explainer".into(),
            name: "Educational Explainer".into(),
            description: "Clean explainer video background".into(),
            prompt_template: "Animated {concept} visualization, educational style, clean background, infographic elements, {style} graphics".into(),
            negative_prompt: Some("complex, busy, cluttered".into()),
            aspect_ratio: AspectRatio::Landscape16x9,
            resolution: VideoResolution::FullHD,
            duration: 4.0,
            params: GenerationParams::default(),
            style: Some(VideoStyle::Animation),
            camera_motion: None,
            examples: HashMap::from([
                ("concept".into(), "solar system planets".into()),
                ("style".into(), "modern flat design".into())
            ]),
            category: TemplateCategory::Education,
        });

        templates.push(VideoTemplate {
            id: "edu-diagram".into(),
            name: "Scientific Diagram".into(),
            description: "Animated scientific diagram".into(),
            prompt_template: "{diagram_type} animation showing {process}, scientific visualization, labeled parts, educational documentary style".into(),
            negative_prompt: None,
            aspect_ratio: AspectRatio::Landscape16x9,
            resolution: VideoResolution::FullHD,
            duration: 6.0,
            params: GenerationParams::default(),
            style: Some(VideoStyle::Documentary),
            camera_motion: Some(CameraMotion::DollyIn),
            examples: HashMap::from([
                ("diagram_type".into(), "Cross-section".into()),
                ("process".into(), "water cycle from ocean to clouds".into())
            ]),
            category: TemplateCategory::Education,
        });

        // === ABSTRACT TEMPLATES ===
        templates.push(VideoTemplate {
            id: "abstract-motion".into(),
            name: "Abstract Motion".into(),
            description: "Abstract artistic motion graphics".into(),
            prompt_template: "Abstract {shape} morphing and flowing, {color_scheme} color palette, mesmerizing motion graphics, hypnotic".into(),
            negative_prompt: Some("realistic, photographic".into()),
            aspect_ratio: AspectRatio::Square,
            resolution: VideoResolution::FullHD,
            duration: 4.0,
            params: GenerationParams { guidance_scale: 7.0, ..Default::default() },
            style: Some(VideoStyle::Animation),
            camera_motion: Some(CameraMotion::OrbitRight),
            examples: HashMap::from([
                ("shape".into(), "geometric shapes".into()),
                ("color_scheme".into(), "vibrant neon".into())
            ]),
            category: TemplateCategory::Abstract,
        });

        templates.push(VideoTemplate {
            id: "abstract-particles".into(),
            name: "Particle Flow".into(),
            description: "Flowing particle animation".into(),
            prompt_template: "{particle_type} particles flowing in {direction}, ethereal glow, dreamlike atmosphere, 4K quality".into(),
            negative_prompt: None,
            aspect_ratio: AspectRatio::Landscape16x9,
            resolution: VideoResolution::FullHD,
            duration: 5.0,
            params: GenerationParams::default(),
            style: Some(VideoStyle::Artistic),
            camera_motion: Some(CameraMotion::ZoomIn),
            examples: HashMap::from([
                ("particle_type".into(), "golden dust".into()),
                ("direction".into(), "spiral pattern".into())
            ]),
            category: TemplateCategory::Abstract,
        });

        // === TRANSITIONS TEMPLATES ===
        templates.push(VideoTemplate {
            id: "transition-morph".into(),
            name: "Morph Transition".into(),
            description: "Smooth morphing transition".into(),
            prompt_template: "{from_object} morphing into {to_object}, smooth transformation, seamless transition, creative morph".into(),
            negative_prompt: Some("jagged, glitchy, abrupt".into()),
            aspect_ratio: AspectRatio::Landscape16x9,
            resolution: VideoResolution::FullHD,
            duration: 3.0,
            params: GenerationParams { guidance_scale: 7.5, ..Default::default() },
            style: Some(VideoStyle::Animation),
            camera_motion: None,
            examples: HashMap::from([
                ("from_object".into(), "book".into()),
                ("to_object".into(), "butterfly".into())
            ]),
            category: TemplateCategory::Transition,
        });

        templates.push(VideoTemplate {
            id: "transition-reveal".into(),
            name: "Logo Reveal".into(),
            description: "Dramatic logo reveal transition".into(),
            prompt_template: "{effect} revealing {text}, dramatic reveal, particle effects, premium animation".into(),
            negative_prompt: Some("simple, plain, boring".into()),
            aspect_ratio: AspectRatio::Landscape16x9,
            resolution: VideoResolution::FullHD,
            duration: 3.0,
            params: GenerationParams { guidance_scale: 8.0, ..Default::default() },
            style: Some(VideoStyle::Commercial),
            camera_motion: Some(CameraMotion::ZoomIn),
            examples: HashMap::from([
                ("effect".into(), "golden sparkles".into()),
                ("text".into(), "COMPANY LOGO".into())
            ]),
            category: TemplateCategory::Transition,
        });

        // === BACKGROUNDS TEMPLATES ===
        templates.push(VideoTemplate {
            id: "bg-gradient".into(),
            name: "Gradient Background".into(),
            description: "Animated gradient background".into(),
            prompt_template: "Smooth animated gradient, {color1} to {color2}, gentle wave motion, modern aesthetic".into(),
            negative_prompt: Some("harsh, jarring colors".into()),
            aspect_ratio: AspectRatio::Landscape16x9,
            resolution: VideoResolution::FullHD,
            duration: 6.0,
            params: GenerationParams::default(),
            style: Some(VideoStyle::Animation),
            camera_motion: None,
            examples: HashMap::from([
                ("color1".into(), "deep purple".into()),
                ("color2".into(), "bright pink".into())
            ]),
            category: TemplateCategory::Background,
        });

        templates.push(VideoTemplate {
            id: "bg-pattern".into(),
            name: "Pattern Background".into(),
            description: "Animated pattern background".into(),
            prompt_template: "{pattern_type} pattern animation, {mood} atmosphere, subtle motion, loopable".into(),
            negative_prompt: None,
            aspect_ratio: AspectRatio::Landscape16x9,
            resolution: VideoResolution::FullHD,
            duration: 4.0,
            params: GenerationParams::default(),
            style: Some(VideoStyle::Animation),
            camera_motion: None,
            examples: HashMap::from([
                ("pattern_type".into(), "geometric hexagon".into()),
                ("mood".into(), "calming".into())
            ]),
            category: TemplateCategory::Background,
        });

        templates.push(VideoTemplate {
            id: "bg-nature".into(),
            name: "Nature Background".into(),
            description: "Relaxing nature background".into(),
            prompt_template: "{nature_scene}, gentle movement, peaceful atmosphere, ambient background, no people".into(),
            negative_prompt: Some("people, animals, text, logos".into()),
            aspect_ratio: AspectRatio::Landscape16x9,
            resolution: VideoResolution::FullHD,
            duration: 8.0,
            params: GenerationParams::default(),
            style: Some(VideoStyle::Documentary),
            camera_motion: Some(CameraMotion::PanRight),
            examples: HashMap::from([
                ("nature_scene".into(), "gentle ocean waves on a sandy beach".into())
            ]),
            category: TemplateCategory::Background,
        });

        // === ANIMATION TEMPLATES ===
        templates.push(VideoTemplate {
            id: "anim-character".into(),
            name: "Character Animation".into(),
            description: "Animated character action".into(),
            prompt_template: "{character} {action}, {style} animation style, expressive, dynamic".into(),
            negative_prompt: Some("static, stiff, robotic".into()),
            aspect_ratio: AspectRatio::Landscape16x9,
            resolution: VideoResolution::FullHD,
            duration: 3.0,
            params: GenerationParams { guidance_scale: 7.5, ..Default::default() },
            style: Some(VideoStyle::Animation),
            camera_motion: None,
            examples: HashMap::from([
                ("character".into(), "cute robot mascot".into()),
                ("action".into(), "waving hello".into()),
                ("style".into(), "3D Pixar-style".into())
            ]),
            category: TemplateCategory::Animation,
        });

        templates.push(VideoTemplate {
            id: "anim-text".into(),
            name: "Text Animation".into(),
            description: "Animated text effect".into(),
            prompt_template: "\"{text}\" text animation, {effect} effect, {style} style, dynamic typography".into(),
            negative_prompt: Some("static text, plain".into()),
            aspect_ratio: AspectRatio::Landscape16x9,
            resolution: VideoResolution::FullHD,
            duration: 3.0,
            params: GenerationParams { guidance_scale: 7.0, ..Default::default() },
            style: Some(VideoStyle::Animation),
            camera_motion: None,
            examples: HashMap::from([
                ("text".into(), "HELLO WORLD".into()),
                ("effect".into(), "neon glow".into()),
                ("style".into(), "modern minimalist".into())
            ]),
            category: TemplateCategory::Animation,
        });

        templates.push(VideoTemplate {
            id: "anim-infographic".into(),
            name: "Animated Infographic".into(),
            description: "Animated infographic element".into(),
            prompt_template: "Animated infographic showing {data}, {style} design, data visualization, professional".into(),
            negative_prompt: Some("messy, confusing".into()),
            aspect_ratio: AspectRatio::Landscape16x9,
            resolution: VideoResolution::FullHD,
            duration: 4.0,
            params: GenerationParams::default(),
            style: Some(VideoStyle::Animation),
            camera_motion: None,
            examples: HashMap::from([
                ("data".into(), "percentage growth chart".into()),
                ("style".into(), "clean corporate".into())
            ]),
            category: TemplateCategory::Animation,
        });

        // === SPECIAL OCCASIONS ===
        templates.push(VideoTemplate {
            id: "occasion-birthday".into(),
            name: "Birthday Celebration".into(),
            description: "Birthday celebration animation".into(),
            prompt_template: "{name} birthday celebration, {elements}, festive atmosphere, confetti, celebration".into(),
            negative_prompt: Some("sad, dark".into()),
            aspect_ratio: AspectRatio::Landscape16x9,
            resolution: VideoResolution::FullHD,
            duration: 5.0,
            params: GenerationParams::default(),
            style: Some(VideoStyle::Animation),
            camera_motion: None,
            examples: HashMap::from([
                ("name".into(), "Sarah".into()),
                ("elements".into(), "balloons and cake".into())
            ]),
            category: TemplateCategory::Occasion,
        });

        templates.push(VideoTemplate {
            id: "occasion-congrats".into(),
            name: "Congratulations".into(),
            description: "Congratulations animation".into(),
            prompt_template: "Congratulations message, {theme}, celebratory, {effect}, achievement".into(),
            negative_prompt: None,
            aspect_ratio: AspectRatio::Landscape16x9,
            resolution: VideoResolution::FullHD,
            duration: 4.0,
            params: GenerationParams::default(),
            style: Some(VideoStyle::Animation),
            camera_motion: Some(CameraMotion::ZoomIn),
            examples: HashMap::from([
                ("theme".into(), "golden trophy".into()),
                ("effect".into(), "sparkles and fireworks".into())
            ]),
            category: TemplateCategory::Occasion,
        });

        // === FOOD & LIFESTYLE ===
        templates.push(VideoTemplate {
            id: "food-recipe".into(),
            name: "Food Recipe".into(),
            description: "Recipe video style".into(),
            prompt_template: "{food_item} being prepared, {action}, cooking video style, overhead shot, {mood}".into(),
            negative_prompt: Some("messy, unappetizing".into()),
            aspect_ratio: AspectRatio::Square,
            resolution: VideoResolution::FullHD,
            duration: 5.0,
            params: GenerationParams::default(),
            style: Some(VideoStyle::Documentary),
            camera_motion: Some(CameraMotion::Static),
            examples: HashMap::from([
                ("food_item".into(), "fresh pasta".into()),
                ("action".into(), "being plated elegantly".into()),
                ("mood".into(), "warm cozy kitchen".into())
            ]),
            category: TemplateCategory::Food,
        });

        templates.push(VideoTemplate {
            id: "food-product".into(),
            name: "Food Product".into(),
            description: "Food product showcase".into(),
            prompt_template: "{food} product shot, {setting}, professional food photography, appetizing".into(),
            negative_prompt: Some("blurry, artificial".into()),
            aspect_ratio: AspectRatio::Landscape16x9,
            resolution: VideoResolution::FullHD,
            duration: 4.0,
            params: GenerationParams { guidance_scale: 8.0, ..Default::default() },
            style: Some(VideoStyle::Commercial),
            camera_motion: Some(CameraMotion::OrbitLeft),
            examples: HashMap::from([
                ("food".into(), "craft beer bottle".into()),
                ("setting".into(), "rustic wooden table with ambient lighting".into())
            ]),
            category: TemplateCategory::Food,
        });

        // === TECH & SCI-FI ===
        templates.push(VideoTemplate {
            id: "tech-interface".into(),
            name: "Tech Interface".into(),
            description: "Futuristic tech interface".into(),
            prompt_template: "Futuristic {interface_type}, holographic display, {color_theme} color scheme, sci-fi, advanced technology".into(),
            negative_prompt: Some("old, dated, primitive".into()),
            aspect_ratio: AspectRatio::Landscape16x9,
            resolution: VideoResolution::FullHD,
            duration: 4.0,
            params: GenerationParams { guidance_scale: 7.5, ..Default::default() },
            style: Some(VideoStyle::SciFi),
            camera_motion: Some(CameraMotion::PanLeft),
            examples: HashMap::from([
                ("interface_type".into(), "data dashboard".into()),
                ("color_theme".into(), "cyan and blue".into())
            ]),
            category: TemplateCategory::Tech,
        });

        templates.push(VideoTemplate {
            id: "tech-cyber".into(),
            name: "Cyberpunk City".into(),
            description: "Cyberpunk cityscape".into(),
            prompt_template: "{location} cyberpunk scene, neon lights, {weather}, futuristic city, blade runner aesthetic".into(),
            negative_prompt: Some("bright, sunny, rural".into()),
            aspect_ratio: AspectRatio::Cinematic2_39_1,
            resolution: VideoResolution::FullHD,
            duration: 5.0,
            params: GenerationParams { guidance_scale: 8.0, ..Default::default() },
            style: Some(VideoStyle::SciFi),
            camera_motion: Some(CameraMotion::DollyIn),
            examples: HashMap::from([
                ("location".into(), "rain-soaked street at night".into()),
                ("weather".into(), "heavy rain with reflections".into())
            ]),
            category: TemplateCategory::Tech,
        });

        // === FITNESS & SPORTS ===
        templates.push(VideoTemplate {
            id: "fitness-workout".into(),
            name: "Fitness Workout".into(),
            description: "Workout demonstration".into(),
            prompt_template: "{exercise} demonstration, {style}, motivational, athletic, fitness video".into(),
            negative_prompt: Some("sedentary, lazy".into()),
            aspect_ratio: AspectRatio::Landscape16x9,
            resolution: VideoResolution::FullHD,
            duration: 4.0,
            params: GenerationParams::default(),
            style: Some(VideoStyle::Documentary),
            camera_motion: None,
            examples: HashMap::from([
                ("exercise".into(), "proper squat form".into()),
                ("style".into(), "modern gym setting".into())
            ]),
            category: TemplateCategory::Fitness,
        });

        templates.push(VideoTemplate {
            id: "fitness-motivation".into(),
            name: "Fitness Motivation".into(),
            description: "Motivational fitness video".into(),
            prompt_template: "{activity} in {setting}, energetic, motivational, sports cinematography".into(),
            negative_prompt: None,
            aspect_ratio: AspectRatio::Landscape16x9,
            resolution: VideoResolution::FullHD,
            duration: 5.0,
            params: GenerationParams { guidance_scale: 7.0, ..Default::default() },
            style: Some(VideoStyle::Commercial),
            camera_motion: Some(CameraMotion::DollyIn),
            examples: HashMap::from([
                ("activity".into(), "person running".into()),
                ("setting".into(), "sunrise mountain trail".into())
            ]),
            category: TemplateCategory::Fitness,
        });

        // === FASHION & BEAUTY ===
        templates.push(VideoTemplate {
            id: "fashion-showcase".into(),
            name: "Fashion Showcase".into(),
            description: "Fashion product video".into(),
            prompt_template: "{item} fashion showcase, {style}, editorial photography, {mood} atmosphere".into(),
            negative_prompt: Some("amateur, poor lighting".into()),
            aspect_ratio: AspectRatio::Portrait9x16,
            resolution: VideoResolution::FullHD,
            duration: 4.0,
            params: GenerationParams { guidance_scale: 8.0, ..Default::default() },
            style: Some(VideoStyle::Commercial),
            camera_motion: Some(CameraMotion::OrbitLeft),
            examples: HashMap::from([
                ("item".into(), "designer handbag".into()),
                ("style".into(), "high-end luxury".into()),
                ("mood".into(), "elegant and sophisticated".into())
            ]),
            category: TemplateCategory::Fashion,
        });

        templates.push(VideoTemplate {
            id: "beauty-product".into(),
            name: "Beauty Product".into(),
            description: "Beauty product showcase".into(),
            prompt_template: "{product} beauty shot, {background}, cosmetics photography, clean aesthetic".into(),
            negative_prompt: Some("messy, cluttered".into()),
            aspect_ratio: AspectRatio::Square,
            resolution: VideoResolution::FullHD,
            duration: 4.0,
            params: GenerationParams { guidance_scale: 7.5, ..Default::default() },
            style: Some(VideoStyle::Commercial),
            camera_motion: None,
            examples: HashMap::from([
                ("product".into(), "luxury perfume bottle".into()),
                ("background".into(), "soft pastel gradient".into())
            ]),
            category: TemplateCategory::Fashion,
        });

        // === TRAVEL ===
        templates.push(VideoTemplate {
            id: "travel-destination".into(),
            name: "Travel Destination".into(),
            description: "Travel destination showcase".into(),
            prompt_template: "{destination}, travel video, {time}, {weather}, wanderlust, adventure".into(),
            negative_prompt: Some("tourist traps, crowded".into()),
            aspect_ratio: AspectRatio::Landscape16x9,
            resolution: VideoResolution::FullHD,
            duration: 5.0,
            params: GenerationParams::default(),
            style: Some(VideoStyle::Documentary),
            camera_motion: Some(CameraMotion::PanRight),
            examples: HashMap::from([
                ("destination".into(), "ancient temple in Kyoto".into()),
                ("time".into(), "golden hour".into()),
                ("weather".into(), "clear sky".into())
            ]),
            category: TemplateCategory::Travel,
        });

        templates.push(VideoTemplate {
            id: "travel-aerial".into(),
            name: "Aerial View".into(),
            description: "Drone aerial footage style".into(),
            prompt_template: "Aerial drone shot of {location}, {time}, sweeping cinematic views, {movement}".into(),
            negative_prompt: Some("ground level, close-up".into()),
            aspect_ratio: AspectRatio::Landscape16x9,
            resolution: VideoResolution::FullHD,
            duration: 5.0,
            params: GenerationParams::default(),
            style: Some(VideoStyle::Cinematic),
            camera_motion: Some(CameraMotion::CraneUp),
            examples: HashMap::from([
                ("location".into(), "tropical island coastline".into()),
                ("time".into(), "sunset".into()),
                ("movement".into(), "smooth descending".into())
            ]),
            category: TemplateCategory::Travel,
        });

        // === MUSIC & ENTERTAINMENT ===
        templates.push(VideoTemplate {
            id: "music-visualizer".into(),
            name: "Music Visualizer".into(),
            description: "Music visualization background".into(),
            prompt_template: "Music visualizer, {style}, {color}, rhythm-reactive visuals, audio reactive".into(),
            negative_prompt: Some("static, boring".into()),
            aspect_ratio: AspectRatio::Landscape16x9,
            resolution: VideoResolution::FullHD,
            duration: 10.0,
            params: GenerationParams { guidance_scale: 7.0, ..Default::default() },
            style: Some(VideoStyle::MusicVideo),
            camera_motion: None,
            examples: HashMap::from([
                ("style".into(), "psychedelic fractal".into()),
                ("color".into(), "rainbow spectrum".into())
            ]),
            category: TemplateCategory::Entertainment,
        });

        templates.push(VideoTemplate {
            id: "music-lyrics".into(),
            name: "Lyric Video".into(),
            description: "Lyric video style".into(),
            prompt_template: "Lyric video background, {mood} atmosphere, {genre} music style, dynamic typography".into(),
            negative_prompt: Some("busy, distracting".into()),
            aspect_ratio: AspectRatio::Landscape16x9,
            resolution: VideoResolution::FullHD,
            duration: 8.0,
            params: GenerationParams::default(),
            style: Some(VideoStyle::MusicVideo),
            camera_motion: None,
            examples: HashMap::from([
                ("mood".into(), "dreamy".into()),
                ("genre".into(), "indie pop".into())
            ]),
            category: TemplateCategory::Entertainment,
        });

        // === REAL ESTATE ===
        templates.push(VideoTemplate {
            id: "realestate-exterior".into(),
            name: "Property Exterior".into(),
            description: "Real estate exterior showcase".into(),
            prompt_template: "{property_type} exterior, {style}, {time}, professional real estate photography".into(),
            negative_prompt: Some("cluttered, messy".into()),
            aspect_ratio: AspectRatio::Landscape16x9,
            resolution: VideoResolution::FullHD,
            duration: 5.0,
            params: GenerationParams::default(),
            style: Some(VideoStyle::Commercial),
            camera_motion: Some(CameraMotion::PanRight),
            examples: HashMap::from([
                ("property_type".into(), "modern luxury home".into()),
                ("style".into(), "minimalist design".into()),
                ("time".into(), "sunset golden hour".into())
            ]),
            category: TemplateCategory::RealEstate,
        });

        templates.push(VideoTemplate {
            id: "realestate-interior".into(),
            name: "Property Interior".into(),
            description: "Real estate interior tour".into(),
            prompt_template: "{room} interior, {style}, professional staging, {lighting}, real estate video".into(),
            negative_prompt: Some("empty, bare".into()),
            aspect_ratio: AspectRatio::Landscape16x9,
            resolution: VideoResolution::FullHD,
            duration: 4.0,
            params: GenerationParams::default(),
            style: Some(VideoStyle::Documentary),
            camera_motion: Some(CameraMotion::DollyIn),
            examples: HashMap::from([
                ("room".into(), "modern living room".into()),
                ("style".into(), "Scandinavian design".into()),
                ("lighting".into(), "natural light through large windows".into())
            ]),
            category: TemplateCategory::RealEstate,
        });

        // === AUTOMOTIVE ===
        templates.push(VideoTemplate {
            id: "auto-showcase".into(),
            name: "Car Showcase".into(),
            description: "Automotive showcase video".into(),
            prompt_template: "{car} showcase, {setting}, automotive photography, {mood}, professional car video".into(),
            negative_prompt: Some("amateur, poor quality".into()),
            aspect_ratio: AspectRatio::Landscape16x9,
            resolution: VideoResolution::FullHD,
            duration: 5.0,
            params: GenerationParams { guidance_scale: 8.0, ..Default::default() },
            style: Some(VideoStyle::Commercial),
            camera_motion: Some(CameraMotion::OrbitLeft),
            examples: HashMap::from([
                ("car".into(), "luxury sports car".into()),
                ("setting".into(), "city skyline at night".into()),
                ("mood".into(), "dramatic and powerful".into())
            ]),
            category: TemplateCategory::Automotive,
        });

        templates.push(VideoTemplate {
            id: "auto-driving".into(),
            name: "Car in Motion".into(),
            description: "Car driving video".into(),
            prompt_template: "{car} driving on {road}, {time}, cinematic car chase style, dynamic".into(),
            negative_prompt: Some("stationary, parked".into()),
            aspect_ratio: AspectRatio::Landscape16x9,
            resolution: VideoResolution::FullHD,
            duration: 4.0,
            params: GenerationParams::default(),
            style: Some(VideoStyle::Cinematic),
            camera_motion: Some(CameraMotion::DollyIn),
            examples: HashMap::from([
                ("car".into(), "vintage muscle car".into()),
                ("road".into(), "winding coastal highway".into()),
                ("time".into(), "sunset".into())
            ]),
            category: TemplateCategory::Automotive,
        });

        // === ANIMALS ===
        templates.push(VideoTemplate {
            id: "animal-portrait".into(),
            name: "Animal Portrait".into(),
            description: "Animal portrait video".into(),
            prompt_template: "{animal}, {setting}, wildlife photography style, {behavior}, nature documentary".into(),
            negative_prompt: Some("domestic, pet store".into()),
            aspect_ratio: AspectRatio::Landscape16x9,
            resolution: VideoResolution::FullHD,
            duration: 4.0,
            params: GenerationParams::default(),
            style: Some(VideoStyle::Documentary),
            camera_motion: None,
            examples: HashMap::from([
                ("animal".into(), "majestic lion".into()),
                ("setting".into(), "African savanna".into()),
                ("behavior".into(), "roaring".into())
            ]),
            category: TemplateCategory::Nature,
        });

        templates.push(VideoTemplate {
            id: "animal-cute".into(),
            name: "Cute Animal".into(),
            description: "Cute animal video".into(),
            prompt_template: "Cute {animal} {action}, adorable, {setting}, heartwarming".into(),
            negative_prompt: Some("scary, aggressive".into()),
            aspect_ratio: AspectRatio::Square,
            resolution: VideoResolution::FullHD,
            duration: 4.0,
            params: GenerationParams::default(),
            style: Some(VideoStyle::Animation),
            camera_motion: None,
            examples: HashMap::from([
                ("animal".into(), "puppy".into()),
                ("action".into(), "playing with toy".into()),
                ("setting".into(), "sunny living room".into())
            ]),
            category: TemplateCategory::Nature,
        });

        // === WEATHER ===
        templates.push(VideoTemplate {
            id: "weather-storm".into(),
            name: "Dramatic Weather".into(),
            description: "Dramatic weather footage".into(),
            prompt_template: "{weather_type}, dramatic, {time}, powerful nature, {location}".into(),
            negative_prompt: Some("calm, peaceful".into()),
            aspect_ratio: AspectRatio::Landscape16x9,
            resolution: VideoResolution::FullHD,
            duration: 5.0,
            params: GenerationParams::default(),
            style: Some(VideoStyle::Documentary),
            camera_motion: None,
            examples: HashMap::from([
                ("weather_type".into(), "massive thunderstorm".into()),
                ("time".into(), "evening".into()),
                ("location".into(), "open plains".into())
            ]),
            category: TemplateCategory::Nature,
        });

        templates.push(VideoTemplate {
            id: "weather-peaceful".into(),
            name: "Peaceful Weather".into(),
            description: "Peaceful weather scene".into(),
            prompt_template: "{weather_type} gently falling, {setting}, peaceful atmosphere, ASMR".into(),
            negative_prompt: Some("storm, harsh".into()),
            aspect_ratio: AspectRatio::Landscape16x9,
            resolution: VideoResolution::FullHD,
            duration: 8.0,
            params: GenerationParams::default(),
            style: Some(VideoStyle::Documentary),
            camera_motion: None,
            examples: HashMap::from([
                ("weather_type".into(), "light rain".into()),
                ("setting".into(), "cozy cabin window view".into())
            ]),
            category: TemplateCategory::Nature,
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
