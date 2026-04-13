//! ═════════════════════════════════════════════════════════════════
//!  SSE STREAMING MODULE - Server-Sent Events Desteği
//! ═════════════════════════════════════════════════════════════════
//!
//! LLM yanıtlarının stream olarak gönderilmesi.
//! Uzun yanıtlarda kullanıcı bekleme süresini azaltır.

use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};

/// SSE olayı
#[derive(Debug, Clone, serde::Serialize)]
pub struct SseEvent {
    pub event: Option<String>,
    pub data: String,
    pub id: Option<String>,
    pub retry: Option<u64>,
}

impl SseEvent {
    pub fn data(data: impl Into<String>) -> Self {
        Self { event: None, data: data.into(), id: None, retry: None }
    }

    pub fn with_event(mut self, event: &str) -> Self {
        self.event = Some(event.into());
        self
    }

    pub fn with_id(mut self, id: &str) -> Self {
        self.id = Some(id.into());
        self
    }

    /// SSE formatına çevir
    pub fn to_sse_string(&self) -> String {
        let mut output = String::new();
        if let Some(ref event) = self.event {
            output.push_str(&format!("event: {}\n", event));
        }
        for line in self.data.lines() {
            output.push_str(&format!("data: {}\n", line));
        }
        if let Some(ref id) = self.id {
            output.push_str(&format!("id: {}\n", id));
        }
        if let Some(retry) = self.retry {
            output.push_str(&format!("retry: {}\n", retry));
        }
        output.push('\n');
        output
    }
}

/// SSE akış yöneticisi
pub struct SseStream {
    sender: mpsc::UnboundedSender<SseEvent>,
    active: Arc<Mutex<bool>>,
}

impl SseStream {
    /// Yeni SSE akışı oluştur
    pub fn new() -> (Self, mpsc::UnboundedReceiver<SseEvent>) {
        let (sender, receiver) = mpsc::unbounded_channel();
        let stream = Self {
            sender,
            active: Arc::new(Mutex::new(true)),
        };
        (stream, receiver)
    }

    /// Olay gönder
    pub fn send(&self, event: SseEvent) -> Result<(), String> {
        self.sender.send(event).map_err(|e| format!("SSE gönderme hatası: {}", e.0.data))
    }

    /// Token akışı gönder (LLM streaming)
    pub fn send_token(&self, token: &str, model: &str) -> Result<(), String> {
        let event = SseEvent::data(serde_json::json!({
            "type": "token",
            "content": token,
            "model": model
        }).to_string()).with_event("token");
        self.send(event)
    }

    /// Akış tamamlandı
    pub fn send_done(&self, model: &str, total_tokens: u32) -> Result<(), String> {
        let event = SseEvent::data(serde_json::json!({
            "type": "done",
            "model": model,
            "total_tokens": total_tokens
        }).to_string()).with_event("done");
        self.send(event)
    }

    /// Hata gönder
    pub fn send_error(&self, error: &str) -> Result<(), String> {
        let event = SseEvent::data(serde_json::json!({
            "type": "error",
            "message": error
        }).to_string()).with_event("error");
        self.send(event)
    }

    /// Akış aktif mi?
    pub async fn is_active(&self) -> bool {
        *self.active.lock().await
    }

    /// Akışı kapat
    pub async fn close(&self) {
        *self.active.lock().await = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sse_event_format() {
        let event = SseEvent::data("hello world").with_event("message").with_id("1");
        let sse = event.to_sse_string();
        assert!(sse.contains("event: message"));
        assert!(sse.contains("data: hello world"));
        assert!(sse.contains("id: 1"));
    }

    #[test]
    fn test_sse_stream() {
        let (stream, mut receiver) = SseStream::new();
        stream.send_token("Hello", "gpt-4").unwrap();
        stream.send_token(" World", "gpt-4").unwrap();
        stream.send_done("gpt-4", 10).unwrap();

        let event1 = receiver.try_recv().unwrap();
        assert_eq!(event1.event, Some("token".into()));
    }

    #[test]
    fn test_sse_multiline_data() {
        let event = SseEvent::data("line1\nline2\nline3");
        let sse = event.to_sse_string();
        assert!(sse.contains("data: line1"));
        assert!(sse.contains("data: line2"));
        assert!(sse.contains("data: line3"));
    }
}
