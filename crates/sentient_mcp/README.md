# sentient_mcp

**Model Context Protocol (MCP)** implementation for SENTIENT OS.

[![Crates.io](https://img.shields.io/crates/v/sentient_mcp.svg)](https://crates.io/crates/sentient_mcp)
[![Documentation](https://docs.rs/sentient_mcp/badge.svg)](https://docs.rs/sentient_mcp)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE)

## Overview

This crate provides a native Rust implementation of Anthropic's **Model Context Protocol (MCP)**, enabling seamless integration with:

- **Claude Desktop**
- **Cursor IDE**
- **Windsurf**
- **Continue.dev**
- Any MCP-compatible client

## Features

- ✅ **Multiple Transports**: stdio, TCP, WebSocket, SSE
- ✅ **Server Implementation**: Expose tools, resources, prompts
- ✅ **Client Implementation**: Connect to MCP servers
- ✅ **Tool System**: Register and execute custom tools
- ✅ **Resource System**: Expose files, databases, APIs
- ✅ **Prompt System**: Reusable prompt templates
- ✅ **JSON-RPC 2.0**: Full protocol compliance

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
sentient_mcp = { path = "crates/sentient_mcp" }
```

### Feature Flags

```toml
[dependencies.sentient_mcp]
features = ["full"]  # stdio, tcp, websocket, sse
```

## Quick Start

### Server Example

```rust
use sentient_mcp::{Server, ServerConfig, Tool, ToolExecutor, ToolResult, ToolCall};
use async_trait::async_trait;

// Define a custom tool
struct Calculator;

#[async_trait]
impl ToolExecutor for Calculator {
    async fn execute(&self, call: ToolCall) -> sentient_mcp::Result<ToolResult> {
        let a = call.arguments.get("a").and_then(|v| v.as_i64()).unwrap_or(0);
        let b = call.arguments.get("b").and_then(|v| v.as_i64()).unwrap_or(0);
        let op = call.arguments.get("op").and_then(|v| v.as_str()).unwrap_or("add");
        
        let result = match op {
            "add" => a + b,
            "sub" => a - b,
            "mul" => a * b,
            _ => return Ok(ToolResult::error("Unknown operation")),
        };
        
        Ok(ToolResult::text(format!("Result: {}", result)))
    }
    
    fn definition(&self) -> Tool {
        Tool::new("calculator", "Simple calculator", serde_json::json!({
            "type": "object",
            "properties": {
                "a": { "type": "integer" },
                "b": { "type": "integer" },
                "op": { "type": "string", "enum": ["add", "sub", "mul"] }
            },
            "required": ["a", "b", "op"]
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = Server::new(ServerConfig::default());
    
    // Register tools
    server.register_tool(Calculator);
    
    // Run server
    server.run().await?;
    Ok(())
}
```

### Client Example

```rust
use sentient_mcp::{Client, ClientConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = Client::new(ClientConfig::tcp("localhost", 8080));
    client.connect().await?;
    
    // List tools
    let tools = client.list_tools().await?;
    println!("Available tools: {:?}", tools);
    
    // Call a tool
    let result = client.call_tool("calculator", serde_json::json!({
        "a": 10,
        "b": 5,
        "op": "add"
    }).as_object().unwrap().clone()).await?;
    
    println!("Result: {:?}", result);
    
    client.close().await?;
    Ok(())
}
```

## Built-in Tools

The server comes with built-in tools:

| Tool | Description |
|------|-------------|
| `echo` | Echo text back |
| `current_time` | Get current date/time |

## Built-in Prompts

| Prompt | Description |
|--------|-------------|
| `code_review` | Review code for quality |
| `explain_code` | Explain code functionality |
| `debug` | Help debug issues |

## Transport Options

### stdio (default)
For process communication (Claude Desktop style):

```rust
let config = ClientConfig::stdio("mcp-server-command");
```

### TCP
For network communication:

```rust
let config = ClientConfig::tcp("localhost", 8080);
```

### WebSocket
For browser/real-time communication:

```rust
let config = ClientConfig::websocket("ws://localhost:8080/mcp");
```

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    SENTIENT MCP                              │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐         │
│  │   Client    │  │   Server    │  │  Transport  │         │
│  └─────────────┘  └─────────────┘  └─────────────┘         │
│         │               │                │                  │
│         ▼               ▼                ▼                  │
│  ┌──────────────────────────────────────────────┐          │
│  │              Protocol (JSON-RPC 2.0)          │          │
│  └──────────────────────────────────────────────┘          │
│         │               │                │                  │
│         ▼               ▼                ▼                  │
│  ┌───────────┐   ┌───────────┐   ┌───────────┐            │
│  │   Tools   │   │ Resources │   │  Prompts  │            │
│  └───────────┘   └───────────┘   └───────────┘            │
└─────────────────────────────────────────────────────────────┘
```

## MCP Protocol Support

| Feature | Status |
|---------|--------|
| Initialize | ✅ |
| Tools (list/call) | ✅ |
| Resources (list/read) | ✅ |
| Prompts (list/get) | ✅ |
| Logging | ⚠️ Partial |
| Completion | ⚠️ Partial |
| Sampling | 🔄 Planned |
| Roots | 🔄 Planned |

## Integration with SENTIENT OS

```rust
// In sentient_gateway or sentient_core
use sentient_mcp::{Server, ServerConfig};

// Create MCP server for SENTIENT tools
let mut mcp_server = Server::new(ServerConfig {
    name: "sentient-os".to_string(),
    version: env!("CARGO_PKG_VERSION").to_string(),
    ..Default::default()
});

// Register SENTIENT tools
mcp_server.register_tool(FileSearchTool);
mcp_server.register_tool(CodeAnalysisTool);
mcp_server.register_tool(MemoryQueryTool);

// Run alongside main system
tokio::spawn(async move {
    mcp_server.run().await
});
```

## License

Apache License 2.0

## References

- [MCP Specification](https://modelcontextprotocol.io)
- [Anthropic MCP](https://github.com/anthropics/mcp)
- [MCP TypeScript SDK](https://github.com/modelcontextprotocol/typescript-sdk)

---

*SENTIENT OS - The Operating System That Thinks*
