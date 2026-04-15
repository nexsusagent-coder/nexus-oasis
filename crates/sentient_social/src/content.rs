//! ─── Content Generation ───

use crate::models::*;
use crate::{SocialResult, SocialError};

/// Content type for generation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContentType {
    Text,
    Image,
    Video,
    Carousel,
    Story,
    Reel,
    Thread,
}

/// Content generator
pub struct ContentGenerator {
    style: ContentStyle,
}

#[derive(Debug, Clone)]
pub struct ContentStyle {
    pub tone: Tone,
    pub emoji_level: EmojiLevel,
    pub hashtag_count: u8,
    pub max_length: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum Tone {
    Professional,
    Casual,
    Friendly,
    Excited,
    Informative,
    Humorous,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EmojiLevel {
    None,
    Minimal,
    Moderate,
    Heavy,
}

impl Default for ContentStyle {
    fn default() -> Self {
        Self {
            tone: Tone::Casual,
            emoji_level: EmojiLevel::Moderate,
            hashtag_count: 3,
            max_length: 280,
        }
    }
}

impl ContentGenerator {
    pub fn new() -> Self {
        Self {
            style: ContentStyle::default(),
        }
    }
    
    pub fn with_style(mut self, style: ContentStyle) -> Self {
        self.style = style;
        self
    }
    
    /// Generate caption for image
    pub async fn generate_caption(&self, context: &str, content_type: ContentType) -> SocialResult<String> {
        // TODO: Integrate with sentient_llm for AI generation
        let caption = match content_type {
            ContentType::Image => format!("{} 📸 #ai #technology", context),
            ContentType::Video => format!("{} 🎬 Watch this!", context),
            _ => context.to_string(),
        };
        
        Ok(self.apply_style(&caption))
    }
    
    /// Generate Reddit comment
    pub async fn generate_reddit_comment(&self, post_title: &str, post_content: &str) -> SocialResult<String> {
        // TODO: Use LLM for context-aware comment generation
        let comment = format!(
            "Thanks for sharing this! This is really interesting. {}",
            self.get_tone_marker()
        );
        
        Ok(comment)
    }
    
    /// Generate hashtags
    pub async fn generate_hashtags(&self, topic: &str) -> SocialResult<Vec<String>> {
        let base_hashtags = vec![
            format!("#{}", topic.to_lowercase().replace(' ', "")),
            "#ai".into(),
            "#technology".into(),
            "#innovation".into(),
        ];
        
        Ok(base_hashtags.into_iter().take(self.style.hashtag_count as usize).collect())
    }
    
    /// Generate thread
    pub async fn generate_thread(&self, topic: &str, count: u8) -> SocialResult<Vec<String>> {
        let mut thread = Vec::new();
        
        thread.push(format!("🧵 Thread: {}", topic));
        
        for i in 1..count {
            thread.push(format!("{}/{} More thoughts on {}...", i, count, topic));
        }
        
        thread.push("That's all! Thanks for reading 🙏".into());
        
        Ok(thread)
    }
    
    /// Generate story content
    pub async fn generate_story(&self, topic: &str) -> SocialResult<String> {
        Ok(format!(
            "✨ {} ✨\n\nSwipe to learn more!\n\n{}",
            topic,
            self.get_call_to_action()
        ))
    }
    
    fn apply_style(&self, text: &str) -> String {
        let mut result = text.to_string();
        
        // Apply emoji
        if self.style.emoji_level != EmojiLevel::None {
            result = self.add_emojis(&result);
        }
        
        // Truncate if needed
        if result.len() > self.style.max_length {
            result = format!("{}...", &result[..self.style.max_length - 3]);
        }
        
        result
    }
    
    fn add_emojis(&self, text: &str) -> String {
        let emojis = match self.style.emoji_level {
            EmojiLevel::None => return text.to_string(),
            EmojiLevel::Minimal => vec!["✨"],
            EmojiLevel::Moderate => vec!["✨", "🚀", "💡"],
            EmojiLevel::Heavy => vec!["✨", "🚀", "💡", "🔥", "💪", "🎯"],
        };
        
        // Add random emoji
        use rand::Rng;
        let emoji = emojis[rand::thread_rng().gen_range(0..emojis.len())];
        format!("{} {}", text, emoji)
    }
    
    fn get_tone_marker(&self) -> &'static str {
        match self.style.tone {
            Tone::Professional => "Looking forward to hearing more perspectives.",
            Tone::Casual => "Would love to hear your thoughts!",
            Tone::Friendly => "Thanks for sharing! 🙌",
            Tone::Excited => "This is amazing! 🎉",
            Tone::Informative => "Here's some additional context...",
            Tone::Humorous => "And here I was thinking I knew it all 😄",
        }
    }
    
    fn get_call_to_action(&self) -> String {
        match self.style.tone {
            Tone::Professional => "Visit our website for more information.",
            Tone::Casual => "Let me know what you think!",
            Tone::Friendly => "Share with your friends! 🙌",
            Tone::Excited => "Don't miss out! 🔥",
            Tone::Informative => "Learn more in the link in bio.",
            Tone::Humorous => "If you know, you know 😎",
        }.to_string()
    }
}

impl Default for ContentGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_caption_generation() {
        let gen = ContentGenerator::new();
        
        let caption = tokio_test::block_on(
            gen.generate_caption("Test content", ContentType::Image)
        ).unwrap();
        
        assert!(caption.contains("Test content"));
    }
}
