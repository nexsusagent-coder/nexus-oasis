//! Integration Setup - Entegrasyon kurulumları

pub struct TelegramSetup;
pub struct DiscordSetup;
pub struct SlackSetup;
pub struct EmailSetup;
pub struct GitHubSetup;

impl TelegramSetup {
    pub fn guide() {
        println!();
        println!("╔════════════════════════════════════════════════════════════════╗");
        println!("║          📱 TELEGRAM BOT KURULUM REHBERİ                        ║");
        println!("╚════════════════════════════════════════════════════════════════╝");
        println!();
        println!("1. Telegram uygulamasını açın");
        println!("2. Arama çubuğuna @BotFather yazın");
        println!("3. /newbot komutunu gönderin");
        println!("4. Bot için bir isim girin (ör: MySENTIENTBot)");
        println!("5. Bot için bir username girin (ör: my_sentient_bot)");
        println!("6. BotFather size bir token verecek");
        println!("   Örnek: 1234567890:ABCdefGHIjklMNOpqrsTUVwxyz");
        println!();
        println!("7. Chat ID almak için:");
        println!("   - Botu bir gruba ekleyin veya direkt mesaj atın");
        println!("   - https://api.telegram.org/bot<TOKEN>/getUpdates");
        println!("   - URL'den chat_id değerini alın");
        println!();
    }
}

impl DiscordSetup {
    pub fn guide() {
        println!();
        println!("╔════════════════════════════════════════════════════════════════╗");
        println!("║          🎮 DISCORD BOT KURULUM REHBERİ                         ║");
        println!("╚════════════════════════════════════════════════════════════════╝");
        println!();
        println!("1. https://discord.com/developers/applications");
        println!("2. 'New Application' tıklayın");
        println!("3. Bot ismi girin (ör: SENTIENT)");
        println!("4. Sol menüden 'Bot' seçin");
        println!("5. 'Add Bot' tıklayın");
        println!("6. 'Token' kopyalayın");
        println!();
        println!("7. Botu sunucuya ekleme:");
        println!("   - OAuth2 → URL Generator");
        println!("   - Scopes: bot");
        println!("   - Permissions: Send Messages, Read Messages");
        println!("   - Oluşan URL'yi açın ve sunucuya ekleyin");
        println!();
    }
}

impl SlackSetup {
    pub fn guide() {
        println!();
        println!("╔════════════════════════════════════════════════════════════════╗");
        println!("║          💼 SLACK WEBHOOK KURULUM REHBERİ                       ║");
        println!("╚════════════════════════════════════════════════════════════════╝");
        println!();
        println!("1. https://api.slack.com/apps");
        println!("2. 'Create New App' tıklayın");
        println!("3. 'From scratch' seçin");
        println!("4. App ismi ve workspace seçin");
        println!("5. 'Incoming Webhooks' aktif edin");
        println!("6. 'Add New Webhook to Workspace'");
        println!("7. Webhook URL'yi kopyalayın");
        println!("   Örnek: https://hooks.slack.com/services/T00/B00/XXX");
        println!();
    }
}

impl EmailSetup {
    pub fn guide() {
        println!();
        println!("╔════════════════════════════════════════════════════════════════╗");
        println!("║          📧 EMAIL SMTP KURULUM REHBERİ                         ║");
        println!("╚════════════════════════════════════════════════════════════════╝");
        println!();
        println!("Gmail için:");
        println!("1. Google Hesabınızda 2FA aktif olmalı");
        println!("2. https://myaccount.google.com/apppasswords");
        println!("3. 'App passwords' oluşturun");
        println!("4. Şifre olarak bu app password'u kullanın");
        println!();
        println!("SMTP Ayarları:");
        println!("   Host: smtp.gmail.com");
        println!("   Port: 587 (TLS) veya 465 (SSL)");
        println!("   User: your@gmail.com");
        println!("   Pass: app-password");
        println!();
    }
}

impl GitHubSetup {
    pub fn guide() {
        println!();
        println!("╔════════════════════════════════════════════════════════════════╗");
        println!("║          🐙 GITHUB TOKEN KURULUM REHBERİ                        ║");
        println!("╚════════════════════════════════════════════════════════════════╝");
        println!();
        println!("1. https://github.com/settings/tokens");
        println!("2. 'Generate new token (classic)'");
        println!("3. Token ismi: SENTIENT NEXUS OS");
        println!("4. Expiration: 90 days veya No expiration");
        println!("5. Scopes seçin:");
        println!("   ☑ repo (Full control of private repositories)");
        println!("   ☑ workflow (Update GitHub Action workflows)");
        println!("   ☑ admin:org (Read and write org data)");
        println!("6. 'Generate token' tıklayın");
        println!("7. Token'ı kopyalayın (sadece bir kez gösterilir!)");
        println!();
    }
}
