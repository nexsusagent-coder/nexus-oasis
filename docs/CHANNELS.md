# SENTIENT AI - Channels Guide

Complete guide to multi-channel communication in SENTIENT AI.

---

## Overview

SENTIENT supports 15+ communication channels:

| Channel | Type | Features |
|---------|------|----------|
| **Telegram** | Messaging | Bot API, Commands, Keyboards |
| **Discord** | Messaging | Slash Commands, Embeds, Voice |
| **WhatsApp** | Messaging | Business API, Templates, Media |
| **Slack** | Messaging | Blocks, Modals, Reactions |
| **Signal** | Messaging | E2EE, Groups, Reactions |
| **Matrix** | Messaging | E2EE, Rooms, Federation |
| **IRC** | Messaging | SASL, IRCv3, Multi-network |
| **Email** | Messaging | SMTP, IMAP, Templates |

---

## Quick Start

### Add Channel

```bash
sentient channel add telegram --token "YOUR_TOKEN"
sentient channel start telegram
```

### List Channels

```bash
sentient channel list
```

### Send Message

```bash
sentient channel send telegram @chat_id "Hello!"
```

---

## Telegram

### Setup

1. Create bot with [@BotFather](https://t.me/botfather)
2. Get bot token
3. Configure SENTIENT:

```bash
sentient channel add telegram --token "123456:ABC-DEF..."
```

### Commands

```bash
# Register commands
sentient channel telegram register-commands

# Set webhook
sentient channel telegram webhook set https://your-domain.com/telegram/webhook

# Get updates
sentient channel telegram get-me
```

### Custom Commands

```rust
use sentient_channels::{TelegramBot, Command};

let bot = TelegramBot::new(token).await?;

bot.command("/hello", |ctx| async move {
    ctx.reply("Hello!").await
}).await?;

bot.command("/weather", |ctx| async move {
    let city = ctx.args().join(" ");
    let weather = get_weather(&city).await?;
    ctx.reply(format!("Weather in {}: {}", city, weather)).await
}).await?;
```

### Keyboards

```rust
use sentient_channels::telegram::{Keyboard, Button};

// Reply keyboard
let keyboard = Keyboard::reply(vec![
    vec![Button::text("Option 1"), Button::text("Option 2")],
    vec![Button::text("Cancel")],
]);

bot.send_keyboard(chat_id, "Choose:", keyboard).await?;

// Inline keyboard
let inline = Keyboard::inline(vec![
    vec![
        Button::callback("Yes", "confirm_yes"),
        Button::callback("No", "confirm_no"),
    ],
]);

bot.send_keyboard(chat_id, "Confirm?", inline).await?;
```

---

## Discord

### Setup

1. Create app at [Discord Developer Portal](https://discord.com/developers/applications)
2. Create bot and get token
3. Configure SENTIENT:

```bash
sentient channel add discord --token "Bot YOUR_TOKEN"
```

### Slash Commands

```bash
# Register commands globally
sentient channel discord register

# Register for specific guild
sentient channel discord register --guild GUILD_ID
```

### Custom Commands

```rust
use sentient_channels::{DiscordBot, SlashCommand};

let bot = DiscordBot::new(token).await?;

bot.command("hello", "Say hello", |ctx| async move {
    ctx.reply("Hello there!").await
}).await?;

bot.command("weather", "Get weather")
    .option("city", "City name", true)
    .handler(|ctx| async move {
        let city = ctx.get_string("city")?;
        let weather = get_weather(&city).await?;
        ctx.reply(format!("Weather: {}", weather)).await
    })
    .register().await?;
```

### Embeds

```rust
bot.send_embed(channel_id, |e| e
    .title("Weather Report")
    .description("Current conditions")
    .field("Temperature", "25°C", true)
    .field("Humidity", "60%", true)
    .color(0x00FF00)
    .footer("Sentient Weather")
).await?;
```

### Voice

```rust
// Join voice channel
bot.join_voice(channel_id).await?;

// Play audio
bot.play_audio(file_path).await?;

// Leave voice
bot.leave_voice().await?;
```

---

## WhatsApp Business

### Setup

1. Create [WhatsApp Business Account](https://business.facebook.com)
2. Get Phone Number ID and Access Token
3. Configure webhook

```bash
sentient channel add whatsapp \
  --phone-id "PHONE_NUMBER_ID" \
  --token "ACCESS_TOKEN"
```

### Messages

```bash
# Send text
sentient channel send whatsapp +1234567890 "Hello!"

# Send image
sentient channel whatsapp send-image +1234567890 https://example.com/image.jpg

# Send document
sentient channel whatsapp send-document +1234567890 https://example.com/doc.pdf
```

### Templates

```rust
use sentient_channels::whatsapp::{WhatsAppClient, Template};

let client = WhatsAppClient::new(phone_id, token).await?;

// Send template
client.send_template(
    "+1234567890",
    "welcome_message",
    Template::new()
        .component("header", "Welcome!")
        .component("body", "John", "SENTIENT")
).await?;
```

### Webhooks

```bash
# Set webhook URL
sentient channel whatsapp webhook set https://your-domain.com/whatsapp/webhook

# Verify webhook
sentient channel whatsapp webhook verify VERIFY_TOKEN
```

---

## Slack

### Setup

1. Create [Slack App](https://api.slack.com/apps)
2. Get Bot User OAuth Token (`xoxb-...`)
3. Configure:

```bash
sentient channel add slack --token "xoxb-..."
```

### Messages

```rust
use sentient_channels::slack::{SlackBot, Block, Button};

let bot = SlackBot::new(token).await?;

// Simple message
bot.send_message("#general", "Hello Slack!").await?;

// Rich message with blocks
bot.send_blocks("#general", vec![
    Block::section("Hello *Slack*!"),
    Block::divider(),
    Block::actions(vec![
        Button::new("approve", "Approve").primary(),
        Button::new("deny", "Deny").danger(),
    ]),
]).await?;
```

### Modals

```rust
bot.open_modal(trigger_id, Modal::new()
    .title("Create Task")
    .input("Task title", "title", true)
    .textarea("Description", "description", false)
    .select("Priority", "priority", vec!["Low", "Medium", "High"])
).await?;
```

### Reactions

```rust
// Add reaction
bot.add_reaction(channel, ts, "thumbsup").await?;

// Remove reaction
bot.remove_reaction(channel, ts, "thumbsup").await?;
```

---

## Signal

### Setup

Requires [signal-cli-rest-api](https://github.com/bbernhard/signal-cli-rest-api).

```bash
# Start signal-cli REST API
docker run -p 8080:8080 bbernhard/signal-cli-rest-api

# Configure SENTIENT
sentient channel add signal \
  --api-url "http://localhost:8080" \
  --phone-number "+1234567890"
```

### Messages

```rust
use sentient_channels::signal::{SignalClient};

let client = SignalClient::new(api_url, phone_number).await?;

// Send message
client.send_message("+0987654321", "Hello Signal!").await?;

// Send to group
client.send_to_group(group_id, "Hello group!").await?;
```

---

## Matrix

### Setup

```bash
sentient channel add matrix \
  --homeserver "https://matrix.org" \
  --username "@sentient:matrix.org" \
  --password "PASSWORD"
```

### Messages

```rust
use sentient_channels::matrix::{MatrixClient};

let client = MatrixClient::new(homeserver, username, password).await?;

// Join room
client.join_room("#sentient:matrix.org").await?;

// Send message
client.send_message(room_id, "Hello Matrix!").await?;

// Encrypted message (E2EE)
client.send_encrypted(room_id, "Secret message").await?;
```

---

## IRC

### Setup

```bash
sentient channel add irc \
  --server "irc.libera.chat" \
  --nickname "SentientBot" \
  --channels "#sentient,#help"
```

### Messages

```rust
use sentient_channels::irc::{IrcClient};

let client = IrcClient::new(config).await?;

// Join channel
client.join("#sentient").await?;

// Send message
client.send("#sentient", "Hello IRC!").await?;

// Private message
client.send_private("user", "Hello!").await?;
```

---

## Multi-Channel Broadcast

Send to all channels at once:

```bash
sentient channel broadcast "Important announcement!"
```

```rust
use sentient_channels::{ChannelManager, Message};

let manager = ChannelManager::new().await?;

// Broadcast to all
manager.broadcast(Message::text("Hello everyone!")).await?;

// Broadcast to specific channels
manager.broadcast_to(
    vec!["telegram", "discord", "slack"],
    Message::text("Hello team!")
).await?;
```

---

## Webhook Security

### Verify Signatures

```rust
use sentient_channels::webhook::{verify_signature, SignatureAlgorithm};

// WhatsApp
let is_valid = verify_signature(
    &body,
    &signature,
    app_secret,
    SignatureAlgorithm::HmacSha256
)?;

// Slack
let is_valid = verify_signature(
    &body,
    &signature,
    signing_secret,
    SignatureAlgorithm::HmacSha256
)?;
```

---

## Rate Limits

| Channel | Limit |
|---------|-------|
| Telegram | 30 msg/sec |
| Discord | 50 msg/sec |
| WhatsApp | 80 msg/sec |
| Slack | 1 msg/sec/channel |
| Matrix | 10 msg/sec |

---

## Troubleshooting

### Connection Issues

```bash
# Test connection
sentient channel test telegram

# Check status
sentient channel status
```

### Debug Mode

```bash
SENTIENT_DEBUG=1 sentient channel start telegram
```

---

**Connect everywhere with SENTIENT! 🔗**
