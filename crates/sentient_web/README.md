# sentient_web

**Web server for SENTIENT OS** - REST API, WebSocket, Dashboard.

[![Crates.io](https://img.shields.io/crates/v/sentient_web.svg)](https://crates.io/crates/sentient_web)
[![Documentation](https://docs.rs/sentient_web/badge.svg)](https://docs.rs/sentient_web)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE)

## Overview

This crate provides a web server for SENTIENT OS:

- 🌐 **REST API**: Full REST API with OpenAPI-style endpoints
- 🔌 **WebSocket**: Real-time bidirectional communication
- 📊 **Dashboard**: Built-in web dashboard
- 🔐 **Authentication**: JWT-based authentication
- ⚡ **Rate Limiting**: Request rate limiting
- 🔄 **CORS**: Cross-origin resource sharing support
- 📦 **Compression**: Gzip compression

## Features

| Feature | Description | Default |
|---------|-------------|---------|
| `dashboard` | Built-in dashboard | ❌ |
| `full` | All features enabled | ❌ |

## Installation

```toml
[dependencies]
sentient_web = { path = "crates/sentient_web" }
```

## Quick Start

### Basic Server

```rust
use sentient_web::{WebServer, ServerConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ServerConfig::new(8080);

    let server = WebServer::new(config);
    server.run().await?;

    Ok(())
}
```

### With Authentication

```rust
let config = ServerConfig::new(8080)
    .with_auth("your-jwt-secret-key")
    .with_cors(vec!["http://localhost:3000".to_string()]);

let server = WebServer::new(config);
server.run().await?;
```

### With Rate Limiting

```rust
let config = ServerConfig::new(8080)
    .with_rate_limit(100)  // 100 requests per minute
    .with_auth("secret");

let server = WebServer::new(config);
server.run().await?;
```

## API Endpoints

### Health

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/health` | Health check |
| GET | `/api/v1/status` | Server status |

### Authentication

| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/api/v1/auth/login` | Login |
| POST | `/api/v1/auth/logout` | Logout |
| POST | `/api/v1/auth/refresh` | Refresh token |

### Users

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/v1/users` | List users |
| GET | `/api/v1/users/:id` | Get user |
| PUT | `/api/v1/users/:id` | Update user |
| DELETE | `/api/v1/users/:id` | Delete user |

### Agents

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/v1/agents` | List agents |
| POST | `/api/v1/agents` | Create agent |
| GET | `/api/v1/agents/:id` | Get agent |
| POST | `/api/v1/agents/:id/chat` | Chat with agent |
| POST | `/api/v1/agents/:id/stream` | Stream response |

### Skills

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/v1/skills` | List skills |
| GET | `/api/v1/skills/:id` | Get skill |

### WebSocket

| Endpoint | Description |
|----------|-------------|
| `/ws` | WebSocket connection |

## Authentication

### JWT Tokens

```rust
use sentient_web::{AuthService, JwtConfig, User};

let config = JwtConfig::new("secret-key")
    .with_expiration(3600);  // 1 hour

let auth = AuthService::new(config);

// Generate token
let user = User::new("username");
let token = auth.generate_token(&user)?;

// Validate token
let claims = auth.validate_token(&token)?;
println!("User: {}", claims.username);
```

### API Keys

```rust
use sentient_web::ApiKey;

let key = ApiKey::new(user_id, "my-api-key");
let key_string = ApiKey::generate_key();

// Returns: sk-xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx
```

## WebSocket

### Connect

```javascript
const ws = new WebSocket('ws://localhost:8080/ws');

ws.onopen = () => {
    ws.send(JSON.stringify({
        type: 'chat',
        content: 'Hello!'
    }));
};

ws.onmessage = (event) => {
    const message = JSON.parse(event.data);
    console.log('Received:', message);
};
```

### Message Types

```typescript
// Chat message
{ "type": "chat", "content": "Hello", "conversation_id": null }

// Stream chunk
{ "type": "stream_chunk", "content": "...", "done": false }

// Status
{ "type": "status", "status": "processing", "data": {} }

// Error
{ "type": "error", "message": "Error message" }
```

## Configuration

```rust
let config = ServerConfig {
    host: "0.0.0.0".to_string(),
    port: 8080,
    cors: true,
    cors_origins: vec!["*".to_string()],
    auth_enabled: true,
    jwt_secret: "secret".to_string(),
    jwt_expiration: 3600,
    rate_limit: true,
    rate_limit_per_minute: 60,
    compression: true,
    dashboard_path: None,
};
```

## Response Format

All API responses follow this format:

```json
{
  "success": true,
  "data": { ... },
  "meta": {
    "timestamp": "2024-01-01T00:00:00Z",
    "request_id": "uuid"
  }
}
```

Error responses:

```json
{
  "success": false,
  "error": "Error message",
  "meta": {
    "timestamp": "2024-01-01T00:00:00Z"
  }
}
```

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     sentient_web                            │
├─────────────────────────────────────────────────────────────┤
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │   Server     │  │    Auth      │  │ Middleware   │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
│         │                                                   │
│         ▼                                                   │
│  ┌──────────────────────────────────────────────────┐      │
│  │                    Router                         │      │
│  │  ┌────────┐ ┌────────┐ ┌────────┐ ┌────────┐     │      │
│  │  │ Health │ │  Auth  │ │ Users  │ │ Agents │     │      │
│  │  └────────┘ └────────┘ └────────┘ └────────┘     │      │
│  │  ┌────────┐ ┌────────┐ ┌────────┐                │      │
│  │  │ Skills │ │   WS   │ │  ...   │                │      │
│  │  └────────┘ └────────┘ └────────┘                │      │
│  └──────────────────────────────────────────────────┘      │
└─────────────────────────────────────────────────────────────┘
```

## License

Apache License 2.0

---

*SENTIENT OS - The Operating System That Thinks*
