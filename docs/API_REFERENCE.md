# 📖 SENTIENT API Reference

Complete REST API documentation for SENTIENT AI OS.

## 🔑 Authentication

All API requests require authentication via Bearer token:

```bash
curl -H "Authorization: Bearer YOUR_TOKEN" https://api.sentient.ai/v1/chat
```

### Getting a Token

```bash
POST /auth/login
{
  "email": "user@example.com",
  "password": "password"
}

Response:
{
  "token": "eyJhbGciOiJIUzI1NiIs...",
  "expires_at": "2024-01-02T00:00:00Z"
}
```

---

## 📍 Endpoints

### Chat

#### POST /v1/chat

Send a message to the AI agent.

**Request:**
```json
{
  "message": "Hello, how can you help me?",
  "session_id": "optional-session-id",
  "model": "gpt-4o",
  "temperature": 0.7,
  "max_tokens": 2000
}
```

**Response:**
```json
{
  "id": "msg_abc123",
  "response": "Hello! I'm here to help you with...",
  "session_id": "sess_xyz789",
  "model": "gpt-4o",
  "usage": {
    "prompt_tokens": 15,
    "completion_tokens": 42,
    "total_tokens": 57
  },
  "created_at": "2024-01-01T12:00:00Z"
}
```

#### POST /v1/chat/stream

Stream responses via Server-Sent Events.

**Request:**
```json
{
  "message": "Write a long story",
  "session_id": "sess_xyz789"
}
```

**Response (SSE):**
```
data: {"token": "Once", "index": 0}
data: {"token": " upon", "index": 1}
data: {"token": " a", "index": 2}
data: {"token": " time...", "index": 3}
data: [DONE]
```

### Sessions

#### GET /v1/sessions

List all sessions.

**Response:**
```json
{
  "sessions": [
    {
      "id": "sess_xyz789",
      "created_at": "2024-01-01T12:00:00Z",
      "message_count": 5,
      "last_active": "2024-01-01T12:30:00Z"
    }
  ],
  "total": 1
}
```

#### GET /v1/sessions/{session_id}

Get session details and history.

**Response:**
```json
{
  "id": "sess_xyz789",
  "messages": [
    {
      "role": "user",
      "content": "Hello!",
      "timestamp": "2024-01-01T12:00:00Z"
    },
    {
      "role": "assistant",
      "content": "Hi there!",
      "timestamp": "2024-01-01T12:00:05Z"
    }
  ],
  "metadata": {
    "model": "gpt-4o",
    "total_tokens": 57
  }
}
```

#### DELETE /v1/sessions/{session_id}

Delete a session.

**Response:** `204 No Content`

### Agents

#### POST /v1/agents

Create a new agent.

**Request:**
```json
{
  "name": "My Agent",
  "model": "gpt-4o",
  "system_prompt": "You are a helpful assistant.",
  "skills": ["web-search", "calculator"],
  "channels": ["telegram", "discord"]
}
```

**Response:**
```json
{
  "id": "agent_abc123",
  "name": "My Agent",
  "status": "active",
  "created_at": "2024-01-01T12:00:00Z"
}
```

#### GET /v1/agents

List all agents.

#### GET /v1/agents/{agent_id}

Get agent details.

#### PUT /v1/agents/{agent_id}

Update agent configuration.

#### DELETE /v1/agents/{agent_id}

Delete an agent.

### Skills

#### GET /v1/skills

List available skills.

**Response:**
```json
{
  "skills": [
    {
      "id": "web-search",
      "name": "Web Search",
      "description": "Search the web for information",
      "version": "1.0.0"
    },
    {
      "id": "calculator",
      "name": "Calculator",
      "description": "Perform calculations",
      "version": "1.0.0"
    }
  ]
}
```

#### POST /v1/skills/install

Install a skill.

**Request:**
```json
{
  "skill_id": "web-search",
  "agent_id": "agent_abc123"
}
```

### Channels

#### GET /v1/channels

List configured channels.

**Response:**
```json
{
  "channels": [
    {
      "id": "telegram_main",
      "type": "telegram",
      "status": "active",
      "connected_at": "2024-01-01T12:00:00Z"
    },
    {
      "id": "discord_main",
      "type": "discord",
      "status": "inactive",
      "error": "Invalid token"
    }
  ]
}
```

#### POST /v1/channels

Configure a new channel.

**Request:**
```json
{
  "type": "telegram",
  "config": {
    "token": "YOUR_BOT_TOKEN"
  }
}
```

#### POST /v1/channels/{channel_id}/start

Start a channel.

#### POST /v1/channels/{channel_id}/stop

Stop a channel.

### Voice

#### POST /v1/voice/transcribe

Transcribe audio to text.

**Request:** `multipart/form-data`
- `audio`: Audio file (wav, mp3, ogg)

**Response:**
```json
{
  "text": "Hello, this is transcribed text",
  "duration_seconds": 5.2,
  "language": "en"
}
```

#### POST /v1/voice/synthesize

Synthesize text to speech.

**Request:**
```json
{
  "text": "Hello, this is a test",
  "voice": "alloy",
  "format": "mp3"
}
```

**Response:** Audio file (binary)

### Memory

#### GET /v1/memory

Get memory statistics.

**Response:**
```json
{
  "total_entries": 1500,
  "total_size_bytes": 524288,
  "oldest_entry": "2024-01-01T00:00:00Z",
  "newest_entry": "2024-01-15T12:00:00Z"
}
```

#### POST /v1/memory/search

Search memory.

**Request:**
```json
{
  "query": "previous conversations about AI",
  "limit": 10
}
```

**Response:**
```json
{
  "results": [
    {
      "id": "mem_abc123",
      "content": "We discussed AI agents...",
      "relevance_score": 0.95,
      "timestamp": "2024-01-10T14:00:00Z"
    }
  ]
}
```

### Enterprise

#### GET /v1/enterprise/users

List users (RBAC required).

#### POST /v1/enterprise/users

Create a user.

**Request:**
```json
{
  "email": "user@example.com",
  "name": "John Doe",
  "role": "developer"
}
```

#### GET /v1/enterprise/audit

Get audit logs.

**Query Parameters:**
- `start_date`: ISO 8601 date
- `end_date`: ISO 8601 date
- `user_id`: Filter by user
- `action`: Filter by action type

---

## 📊 Rate Limits

| Plan | Requests/min | Tokens/min |
|------|--------------|------------|
| Free | 60 | 10,000 |
| Pro | 600 | 100,000 |
| Enterprise | Unlimited | Unlimited |

Rate limit headers:
```
X-RateLimit-Limit: 60
X-RateLimit-Remaining: 45
X-RateLimit-Reset: 1704110400
```

---

## ❌ Error Codes

| Code | Description |
|------|-------------|
| 400 | Bad Request - Invalid parameters |
| 401 | Unauthorized - Invalid or missing token |
| 403 | Forbidden - Insufficient permissions |
| 404 | Not Found - Resource doesn't exist |
| 429 | Too Many Requests - Rate limit exceeded |
| 500 | Internal Server Error |
| 503 | Service Unavailable |

**Error Response Format:**
```json
{
  "error": {
    "code": "invalid_request",
    "message": "The 'message' field is required",
    "details": {
      "field": "message"
    }
  }
}
```

---

## 🔄 Webhooks

Configure webhooks for events:

```json
POST /v1/webhooks
{
  "url": "https://your-server.com/webhook",
  "events": ["message.received", "message.sent", "agent.created"],
  "secret": "webhook-secret"
}
```

**Webhook Payload:**
```json
{
  "event": "message.received",
  "timestamp": "2024-01-01T12:00:00Z",
  "data": {
    "message_id": "msg_abc123",
    "channel": "telegram",
    "content": "Hello!"
  },
  "signature": "sha256=..."
}
```

---

## 📚 SDKs

### Rust

```rust
use sentient_client::{Client, Message};

let client = Client::new("YOUR_API_KEY");
let response = client.chat("Hello!").await?;
println!("{}", response.content);
```

### Python

```python
from sentient import Client

client = Client(api_key="YOUR_API_KEY")
response = client.chat("Hello!")
print(response.content)
```

### JavaScript/TypeScript

```typescript
import { SentientClient } from '@sentient/ai';

const client = new SentientClient({ apiKey: 'YOUR_API_KEY' });
const response = await client.chat('Hello!');
console.log(response.content);
```

---

## 🔗 OpenAPI Specification

Full OpenAPI spec available at: `https://api.sentient.ai/openapi.json`

```yaml
openapi: 3.0.0
info:
  title: SENTIENT API
  version: 1.0.0
servers:
  - url: https://api.sentient.ai/v1
paths:
  /chat:
    post:
      summary: Send a chat message
      requestBody:
        content:
          application/json:
            schema:
              $ref: '#/components/schemas/ChatRequest'
      responses:
        '200':
          description: Successful response
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/ChatResponse'
```

---

*API Version: 1.0.0*
*Base URL: https://api.sentient.ai/v1*
