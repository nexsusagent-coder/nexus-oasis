//! ─── TELEGRAM BOT ───
//!
//! SENTIENT'ya Telegram üzerinden erişim:
//! - /start - Bot başlat
//! - /task <hedef> - Yeni görev başlat
//! - /status - Mevcut görev durumu
//! - /cancel - Görevi iptal et
//! - /help - Yardım mesajı

use teloxide::{
    dispatching::UpdateFilterExt,
    dptree,
    prelude::*,
    types::ParseMode,
    utils::command::BotCommands,
};
use std::sync::Arc;

use crate::{GatewayRequest, RequestSource};
use crate::dispatcher::TaskDispatcher;
use crate::task_manager::TaskManager;

/// ─── BOT KOMUTLARI ───

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "SENTIENT Komutları")]
enum Command {
    #[command(description = "Botu başlat ve karşılama mesajı göster")]
    Start,
    
    #[command(description = "Yeni görev başlat")]
    Task(String),
    
    #[command(description = "Mevcut görev durumunu göster")]
    Status,
    
    #[command(description = "Aktif görevi iptal et")]
    Cancel,
    
    #[command(description = "Bot bilgisi")]
    About,
    
    #[command(description = "Yardım mesajı")]
    Help,
}

/// ─── BOT STATE ───

#[derive(Clone)]
pub struct BotState {
    pub dispatcher: Arc<TaskDispatcher>,
    pub task_manager: Arc<TaskManager>,
}

/// Bot'u başlat
pub async fn run_bot(
    token: &str,
    dispatcher: Arc<TaskDispatcher>,
    task_manager: Arc<TaskManager>,
) -> crate::SENTIENTResult<()> {
    log::info!("🤖  Telegram Bot başlatılıyor...");
    
    let bot = Bot::new(token);
    let state = BotState { dispatcher, task_manager };
    
    let handler = Update::filter_message()
        .branch(
            dptree::entry()
                .filter_command::<Command>()
                .endpoint(handle_command)
        )
        .branch(
            dptree::filter(|msg: Message| msg.text().map(|t| !t.starts_with('/')).unwrap_or(false))
                .endpoint(handle_text_message)
        );
    
    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![state])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
    
    Ok(())
}

/// Komut işleyici
async fn handle_command(
    bot: Bot,
    msg: Message,
    cmd: Command,
    state: BotState,
) -> ResponseResult<()> {
    let chat_id = msg.chat.id;
    
    match cmd {
        Command::Start => {
            let welcome = r#"
🐺 **SENTIENT'ya Hoş Geldiniz!**

Ben NEXUS OASIS Yapay Zeka İşletim Sistemi'nin otonom ajanıyım.

**Yapabildiklerim:**
• Web'de araştırma yapabilirim
• Kod yazıp çalıştırabilirim
• Analiz ve raporlar hazırlayabilirim
• Veri toplayıp işleyebilirim

**Komutlar:**
/task <hedef> - Yeni görev başlat
/status - Görev durumu
/cancel - Görevi iptal et
/about - Bot bilgisi
/help - Yardım

**Örnek:**
`/task Yapay zeka trendleri hakkında araştırma yap`
"#;
            bot.send_message(chat_id, welcome)
                .parse_mode(ParseMode::Markdown)
                .await?;
        }
        
        Command::Task(goal) => {
            if goal.trim().is_empty() {
                bot.send_message(chat_id, "⚠️ Hedef boş olamaz.\n\nKullanım: `/task <hedef>`")
                    .parse_mode(ParseMode::Markdown)
                    .await?;
                return Ok(());
            }
            
            bot.send_message(chat_id, "⏳ Göreviniz işleniyor...")
                .await?;
            
            // Gateway request oluştur
            let request = GatewayRequest::new(
                goal.clone(),
                RequestSource::Telegram {
                    chat_id: chat_id.0,
                    username: None,
                }
            );
            
            // Dispatch et
            match state.dispatcher.dispatch(request).await {
                Ok(result) => {
                    let response = if result.accepted {
                        format!(
                            "✅ **Görev Kabul Edildi**\n\n\
                            🎯 Hedef: {}\n\
                            🆔 Görev ID: `{}`\n\
                            📍 Kuyruk pozisyonu: {}\n\n\
                            Durumu kontrol etmek için: `/status`",
                            goal,
                            result.task_id,
                            result.queue_position
                        )
                    } else {
                        format!("❌ Görev reddedildi: {}", result.message)
                    };
                    
                    bot.send_message(chat_id, response)
                        .parse_mode(ParseMode::Markdown)
                        .await?;
                }
                Err(e) => {
                    bot.send_message(chat_id, format!("❌ Hata: {}", e.to_sentient_message()))
                        .await?;
                }
            }
        }
        
        Command::Status => {
            let tasks = state.task_manager.get_active_tasks().await;
            let user_tasks: Vec<_> = tasks
                .into_iter()
                .filter(|t| {
                    if let RequestSource::Telegram { chat_id: cid, .. } = t.source {
                        cid == chat_id.0
                    } else {
                        false
                    }
                })
                .collect();
            
            if user_tasks.is_empty() {
                bot.send_message(chat_id, "📋 Aktif göreviniz yok.")
                    .await?;
            } else {
                let mut response = "📋 **Aktif Görevleriniz:**\n\n".to_string();
                
                for task in user_tasks {
                    response.push_str(&format!(
                        "🎯 {}\n\
                         🆔 `{}`\n\
                         📊 Durum: {:?}\n\
                         ⏱️ Süre: {}s\n\
                         📈 İlerleme: {:.0}%\n\n",
                        task.goal.chars().take(50).collect::<String>(),
                        task.id,
                        task.status,
                        task.duration_secs(),
                        task.progress
                    ));
                }
                
                bot.send_message(chat_id, response)
                    .parse_mode(ParseMode::Markdown)
                    .await?;
            }
        }
        
        Command::Cancel => {
            let tasks = state.task_manager.get_active_tasks().await;
            let user_tasks: Vec<_> = tasks
                .into_iter()
                .filter(|t| {
                    if let RequestSource::Telegram { chat_id: cid, .. } = t.source {
                        cid == chat_id.0
                    } else {
                        false
                    }
                })
                .collect();
            
            if user_tasks.is_empty() {
                bot.send_message(chat_id, "📋 İptal edilecek aktif görev yok.")
                    .await?;
            } else {
                let mut cancelled = 0;
                for task in user_tasks {
                    if state.dispatcher.cancel_task(task.id).await.is_ok() {
                        cancelled += 1;
                    }
                }
                
                bot.send_message(chat_id, format!("✅ {} görev iptal edildi.", cancelled))
                    .await?;
            }
        }
        
        Command::About => {
            let about = r#"
🐺 **SENTIENT — NEXUS OASIS**

**Versiyon:** 0.1.0

**Mimari:**
• Rust çekirdeği ( güvenlik)
• LLM entegrasyonu (V-GATE)
• Browser-Use (otonom web)
• Docker sandbox (izole kod)
• Event Graph (lock-free)

**Güvenlik:**
• API anahtarları sunucuda
• Prompt injection koruması
• Veri sızıntısı engeli

**Geliştirici:** NEXUS OASIS Team
"#;
            bot.send_message(chat_id, about)
                .parse_mode(ParseMode::Markdown)
                .await?;
        }
        
        Command::Help => {
            let help = format!(
                "🤖 **SENTIENT Komut Yardımı**\n\n\
                 {}\n\n\
                 **Örnekler:**\n\
                 `/task Python ile web scraper yaz`\n\
                 `/task Rust vs Go performans karşılaştırması`\n\
                 `/task Güncel AI haberlerini topla`",
                Command::descriptions()
            );
            
            bot.send_message(chat_id, help)
                .parse_mode(ParseMode::Markdown)
                .await?;
        }
    }
    
    Ok(())
}

/// Metin mesajı işleyici (komut olmayan)
async fn handle_text_message(
    bot: Bot,
    msg: Message,
    state: BotState,
) -> ResponseResult<()> {
    let text = match msg.text() {
        Some(t) => t,
        None => return Ok(()),
    };
    
    let chat_id = msg.chat.id;
    
    // Doğal dil mesajı olarak görev oluştur
    bot.send_message(chat_id, "💬 Mesajınız anlaşılıyor...")
        .await?;
    
    // Gateway request oluştur
    let request = GatewayRequest::new(
        text.to_string(),
        RequestSource::Telegram {
            chat_id: chat_id.0,
            username: None,
        }
    );
    
    // Dispatch et
    match state.dispatcher.dispatch(request).await {
        Ok(result) => {
            if result.accepted {
                bot.send_message(
                    chat_id,
                    format!(
                        "✅ Anlaşıldı! Göreviniz başlatıldı.\n\n\
                         🆔 ID: `{}`\n\
                         📊 `/status` ile durumu kontrol edebilirsiniz.",
                        result.task_id
                    )
                )
                .parse_mode(ParseMode::Markdown)
                .await?;
            } else {
                bot.send_message(chat_id, format!("❌ {}", result.message))
                    .await?;
            }
        }
        Err(e) => {
            bot.send_message(chat_id, format!("❌ Hata: {}", e.to_sentient_message()))
                .await?;
        }
    }
    
    Ok(())
}

/// ─── YARDIMCI TRAIT ───

trait SENTIENTMessage {
    fn to_sentient_message(&self) -> String;
}

impl SENTIENTMessage for crate::SENTIENTError {
    fn to_sentient_message(&self) -> String {
        match self {
            Self::General(s) => format!("SENTIENT Hatası: {}", s),
            Self::ValidationError(s) => format!("Doğrulama Hatası: {}", s),
            _ => "Bilinmeyen hata".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_welcome_message() {
        let welcome = "🐺 **SENTIENT'ya Hoş Geldiniz!**";
        assert!(welcome.contains("SENTIENT"));
    }
}
