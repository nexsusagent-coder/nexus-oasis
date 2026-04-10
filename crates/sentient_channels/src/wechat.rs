//! WeChat Work / Enterprise WeChat Channel

use async_trait::async_trait;
use crate::{Channel, ChannelMessage, ChannelType, MessageContent, ChannelError};

pub struct WeChatChannel {
    corp_id: String,
    agent_id: String,
    secret: String,
    access_token: Option<String>,
    connected: bool,
}

impl WeChatChannel {
    pub fn new(corp_id: String, agent_id: String, secret: String) -> Self {
        Self { corp_id, agent_id, secret, access_token: None, connected: false }
    }
}

#[async_trait]
impl Channel for WeChatChannel {
    fn channel_type(&self) -> ChannelType { ChannelType::WeChat }

    async fn init(&mut self) -> Result<(), ChannelError> {
        let url = format!(
            "https://qyapi.weixin.qq.com/cgi-bin/gettoken?corpid={}&corpsecret={}",
            self.corp_id, self.secret
        );

        let client = reqwest::Client::new();
        let response = client
            .get(&url)
            .send()
            .await
            .map_err(|e| ChannelError::Network(e.to_string()))?;

        if response.status().is_success() {
            let json: serde_json::Value = response.json().await
                .map_err(|e| ChannelError::Parse(e.to_string()))?;
            self.access_token = json["access_token"].as_str().map(|s| s.to_string());
            self.connected = true;
            Ok(())
        } else {
            Err(ChannelError::AuthFailed("Failed to get access token".into()))
        }
    }

    async fn send(&self, message: ChannelMessage) -> Result<String, ChannelError> {
        let token = self.access_token.as_ref()
            .ok_or_else(|| ChannelError::AuthFailed("No access token".into()))?;

        let url = format!("https://qyapi.weixin.qq.com/cgi-bin/message/send?access_token={}", token);

        let body = match &message.content {
            MessageContent::Text(text) => serde_json::json!({
                "touser": message.chat_id,
                "msgtype": "text",
                "agentid": self.agent_id,
                "text": { "content": text }
            }),
            MessageContent::Markdown(text) => serde_json::json!({
                "touser": message.chat_id,
                "msgtype": "markdown",
                "agentid": self.agent_id,
                "markdown": { "content": text }
            }),
            _ => return Err(ChannelError::InvalidMessage("Unsupported content type".into())),
        };

        let client = reqwest::Client::new();
        let response = client
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| ChannelError::Network(e.to_string()))?;

        if response.status().is_success() {
            Ok("sent".to_string())
        } else {
            Err(ChannelError::ApiError(response.status().to_string()))
        }
    }

    async fn receive(&self) -> Result<Vec<ChannelMessage>, ChannelError> { Ok(Vec::new()) }
    fn is_connected(&self) -> bool { self.connected }
}
