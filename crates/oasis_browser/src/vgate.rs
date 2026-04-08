//! ═══════════════════════════════════════════════════════════════════════════════
//!  V-GATE BRIDGE - Tarayıcı <-> LLM Entegrasyonu
//! ═══════════════════════════════════════════════════════════════════════════════

use crate::error::{BrowserError, BrowserResult};
use crate::observation::Observation;
use crate::actions::BrowserAction;
use sentient_vgate::{VGateEngine, LlmRequest, Message};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Browser için V-GATE köprüsü
#[derive(Clone)]
pub struct BrowserVGate {
    /// V-GATE engine
    engine: Arc<Mutex<VGateEngine>>,
    /// Model adı
    model: String,
    /// System prompt
    system_prompt: String,
}

impl BrowserVGate {
    pub fn new(vgate_url: &str) -> Self {
        let config = sentient_vgate::VGateConfig {
            proxy_url: vgate_url.into(),
            ..Default::default()
        };
        
        Self {
            engine: Arc::new(Mutex::new(VGateEngine::new(config))),
            model: "qwen/qwen3-1.7b:free".into(),
            system_prompt: BROWSER_SYSTEM_PROMPT.into(),
        }
    }
    
    /// Aksiyon kararı al
    pub async fn get_next_action(&self, observation: &Observation, task: &str) -> BrowserResult<BrowserAction> {
        log::info!("V-GATE: Aksiyon kararı isteniyor...");
        
        let user_message = format!(
            "GOREV: {}\n\n{}\n\nBir sonraki aksiyon nedir?",
            task,
            self.format_observation(observation)
        );
        
        let request = LlmRequest {
            model: self.model.clone(),
            messages: vec![
                Message { role: "system".into(), content: self.system_prompt.clone() },
                Message { role: "user".into(), content: user_message },
            ],
            max_tokens: Some(500),
            temperature: Some(0.1),
            stream: Some(false),
        };
        
        let engine = self.engine.lock().await;
        let event = engine.send_request(request).await.map_err(|e| {
            BrowserError::VGateConnectionFailed(e.to_string())
        })?;
        
        // Event'ten aksiyon çıkar
        let content = event.payload.get("content")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        
        self.parse_action(content)
    }
    
    /// Observation formatla
    fn format_observation(&self, obs: &Observation) -> String {
        let elements_str = obs.interactive_elements.iter()
            .enumerate()
            .map(|(i, e)| format!("[{}] {:?}: {} ({})", i+1, e.element_type, e.description, e.selector))
            .collect::<Vec<_>>()
            .join("\n");
        
        let content_preview: String = obs.text_content.chars().take(1000).collect();
        
        format!(
            "=== SAYFA ===\nURL: {}\nBaslik: {}\nDurum: {:?}\n\n=== ETKILESIM ELEMENTLERI ===\n{}\n\n=== IÇERIK ===\n{}",
            obs.url,
            obs.title,
            obs.state,
            elements_str,
            content_preview
        )
    }
    
    /// LLM yanıtından aksiyon parse et
    fn parse_action(&self, response: &str) -> BrowserResult<BrowserAction> {
        // JSON formatı ara
        if let Some(json_start) = response.find("{\"type\"") {
            if let Some(json_end) = response[json_start..].rfind('}') {
                let json_str = &response[json_start..=json_start + json_end];
                if let Ok(action) = serde_json::from_str::<BrowserAction>(json_str) {
                    return Ok(action);
                }
            }
        }
        
        // Basit metin komutu parse et
        let lower = response.to_lowercase();
        
        if lower.contains("tikla") || lower.contains("click") {
            if let Some(selector) = self.extract_selector(response) {
                return Ok(BrowserAction::Click { selector, xpath: None, index: None });
            }
        }
        
        if lower.contains("yaz") || lower.contains("type") {
            if let Some((selector, text)) = self.extract_type_params(response) {
                return Ok(BrowserAction::Type {
                    selector,
                    text,
                    press_enter: false,
                    clear_first: true,
                });
            }
        }
        
        if lower.contains("git") || lower.contains("navigate") {
            if let Some(url) = self.extract_url(response) {
                return Ok(BrowserAction::Navigate { url });
            }
        }
        
        if lower.contains("tamamlandi") || lower.contains("done") {
            return Ok(BrowserAction::Done { result: "Görev tamamlandi".into() });
        }
        
        // Varsayılan
        Ok(BrowserAction::Done { result: response.into() })
    }
    
    fn extract_selector(&self, text: &str) -> Option<String> {
        let patterns = ["#", ".", "[", "button", "a ", "input"];
        for pattern in patterns {
            if let Some(start) = text.find(pattern) {
                let end = text[start..].find(|c: char| c.is_whitespace() || c == ',' || c == ')')
                    .unwrap_or(text[start..].len());
                return Some(text[start..start+end].to_string());
            }
        }
        None
    }
    
    fn extract_type_params(&self, _text: &str) -> Option<(String, String)> {
        None
    }
    
    fn extract_url(&self, text: &str) -> Option<String> {
        if let Some(start) = text.find("http") {
            let end = text[start..].find(|c: char| c.is_whitespace())
                .unwrap_or(text[start..].len());
            return Some(text[start..start+end].to_string());
        }
        None
    }
}

/// Browser Agent System Prompt
static BROWSER_SYSTEM_PROMPT: &str = r###"
Sen SENTIENT'nin web tarayici ajanisin. Gorevin kullanici isteklerini web'de gerceklestirmek.

KURALLAR:
1. Sadece DIS web sitelerine erisebilirsin (file:// yasak)
2. Elementleri dikkatli sec, selector kullan
3. Her adimda bir aksiyon al
4. Gorevin tamamlandiginda Done dondur

AKSIYON FORMATLARI:
- Navigate: {"type": "Navigate", "url": "https://..."}
- Click: {"type": "Click", "selector": "#button-id"}
- Type: {"type": "Type", "selector": "#input", "text": "...", "press_enter": false}
- Scroll: {"type": "Scroll", "direction": "Down", "amount": 500}
- Done: {"type": "Done", "result": "Sonuc aciklamasi"}

ORNEK:
Kullanici: GitHub'da Rust repoları ara
1. Navigate -> https://github.com
2. Type -> arama kutusuna rust
3. Click -> ara butonuna
4. Done -> 10 Rust repo listelendi
"###;
