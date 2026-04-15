//! Digest composer - assembles sections into final digest

use crate::{Digest, DigestConfig, DigestError, DigestResult, DigestSection};

/// Composes digest sections into final output
pub struct Composer {
    templates: Vec<DigestTemplate>,
}

/// Template for digest composition
pub struct DigestTemplate {
    pub name: String,
    pub language: String,
    pub sections_order: Vec<String>,
}

impl Composer {
    pub fn new() -> Self {
        Self {
            templates: Vec::new(),
        }
    }

    /// Compose digest to text
    pub fn compose_text(&self, digest: &Digest) -> String {
        let mut output = String::new();

        // Header
        output.push_str(&format!("{}\n", digest.title));
        output.push_str(&format!("{}\n\n", "═".repeat(40)));

        // Sections
        for section in &digest.sections {
            output.push_str(&self.format_section(section));
            output.push_str("\n\n");
        }

        // Footer
        output.push_str(&format!(
            "─ {} ─\n",
            digest.created_at.format("%d.%m.%Y %H:%M")
        ));

        output
    }

    /// Compose digest to HTML
    pub fn compose_html(&self, digest: &Digest) -> String {
        let mut html = String::new();

        html.push_str("<!DOCTYPE html>\n<html>\n<head>\n");
        html.push_str("<meta charset=\"UTF-8\">\n");
        html.push_str(&format!("<title>{}</title>\n", digest.title));
        html.push_str("<style>\n");
        html.push_str("body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; max-width: 600px; margin: 0 auto; padding: 20px; }\n");
        html.push_str("h1 { color: #333; border-bottom: 2px solid #007bff; padding-bottom: 10px; }\n");
        html.push_str("h2 { color: #555; margin-top: 20px; }\n");
        html.push_str(".section { background: #f8f9fa; padding: 15px; border-radius: 8px; margin: 10px 0; }\n");
        html.push_str(".item { padding: 10px; border-left: 3px solid #007bff; margin: 10px 0; background: white; }\n");
        html.push_str(".important { border-left-color: #dc3545; }\n");
        html.push_str(".footer { color: #666; font-size: 0.9em; margin-top: 20px; text-align: center; }\n");
        html.push_str("</style>\n</head>\n<body>\n");

        // Title
        html.push_str(&format!("<h1>{}</h1>\n", digest.title));

        // Sections
        for section in &digest.sections {
            html.push_str(&self.format_section_html(section));
        }

        // Footer
        html.push_str(&format!(
            "<div class=\"footer\">{} by {}</div>\n",
            digest.created_at.format("%d.%m.%Y %H:%M"),
            digest.metadata.assistant_name
        ));

        html.push_str("</body>\n</html>");

        html
    }

    fn format_section(&self, section: &DigestSection) -> String {
        let mut output = String::new();

        // Title with icon
        if let Some(ref icon) = section.icon {
            output.push_str(&format!("{} ", icon));
        }
        output.push_str(&format!("{}\n", section.title));

        // Content
        if !section.content.is_empty() {
            output.push_str(&format!("{}\n", section.content));
        }

        // Items
        for item in &section.items {
            let marker = if item.is_important { "⚡" } else { "•" };
            output.push_str(&format!("  {} {}\n", marker, item.title));
            if !item.content.is_empty() {
                output.push_str(&format!("    {}\n", item.content));
            }
        }

        output
    }

    fn format_section_html(&self, section: &DigestSection) -> String {
        let mut html = String::new();

        html.push_str("<div class=\"section\">\n");

        // Title
        if let Some(ref icon) = section.icon {
            html.push_str(&format!("<h2>{} {}</h2>\n", icon, section.title));
        } else {
            html.push_str(&format!("<h2>{}</h2>\n", section.title));
        }

        // Content
        if !section.content.is_empty() {
            html.push_str(&format!("<p>{}</p>\n", section.content));
        }

        // Items
        for item in &section.items {
            let class = if item.is_important { "item important" } else { "item" };
            html.push_str(&format!("<div class=\"{}\">\n", class));
            html.push_str(&format!("<strong>{}</strong>\n", item.title));
            if !item.content.is_empty() {
                html.push_str(&format!("<p>{}</p>\n", item.content));
            }
            html.push_str("</div>\n");
        }

        html.push_str("</div>\n");

        html
    }
}

impl Default for Composer {
    fn default() -> Self {
        Self::new()
    }
}

/// Digest composer trait
pub trait DigestComposer {
    fn compose(&self, digest: &Digest) -> DigestResult<String>;
}

impl DigestComposer for Composer {
    fn compose(&self, digest: &Digest) -> DigestResult<String> {
        Ok(self.compose_text(digest))
    }
}
