# 🏗️ SENTIENT OS ARCHITECTURE

> **A1-A12 Native Integration Blueprint**

---

## 📊 System Overview

SENTIENT OS is built on **GraphBit Core** - a Rust-based framework providing:

- Zero-copy data flow
- Lock-free concurrency
- Minimal memory footprint
- Native Python FFI via PyO3

---

## 🔗 A1-A12 Integration Map

### Core Layer (A1-A4)

| Module | Status | Description |
|--------|--------|-------------|
| **A1: GraphBit Core** | ✅ Active | Rust-based orchestration engine |
| **A2: PyO3 Bridge** | ✅ Active | Python FFI for assimilation |
| **A3: Memory Cube** | ✅ Active | SQLite persistent memory |
| **A4: V-GATE Proxy** | ✅ Active | Zero API key architecture |

### Agent Layer (A5-A8)

| Module | Status | Description |
|--------|--------|-------------|
| **A5: Orchestrator** | ✅ Active | Multi-agent coordination |
| **A6: Session Manager** | ✅ Active | Session persistence |
| **A7: Mode Engine** | ✅ Active | Behavioral modes |
| **A8: Persona System** | ✅ Active | Identity management |

### Tool Layer (A9-A12)

| Module | Status | Description |
|--------|--------|-------------|
| **A9: Oasis Hands** | ✅ Active | 43+ native tools |
| **A10: Oasis Browser** | ✅ Active | Lightpanda FFI browser |
| **A11: Oasis Manus** | ✅ Active | Docker sandbox execution |
| **A12: Guardrails** | ✅ Active | Security & safety layer |

---

## 🏛️ Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                           SENTIENT OS AI OS                                  │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌──────────────────────────────────────────────────────────────────────┐   │
│  │                        PRESENTATION LAYER                            │   │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐ │   │
│  │  │ SENTIENT Shell │  │  Dashboard  │  │    CLI      │  │   Web API   │ │   │
│  │  │   (Native)  │  │   (Native)  │  │  (Native)   │  │  (Axum)     │ │   │
│  │  └─────────────┘  └─────────────┘  └─────────────┘  └─────────────┘ │   │
│  └──────────────────────────────────────────────────────────────────────┘   │
│                                      │                                       │
│  ┌──────────────────────────────────────────────────────────────────────┐   │
│  │                         SECURITY LAYER (A12)                         │   │
│  │  ┌─────────────────────┐  ┌─────────────────────────────────────┐   │   │
│  │  │     Guardrails      │  │           V-GATE Proxy              │   │   │
│  │  │  • Prompt Injection │  │  • Zero API Keys                    │   │   │
│  │  │  • Content Filter   │  │  • Request Routing                  │   │   │
│  │  │  • Rate Limiting    │  │  • Load Balancing                   │   │   │
│  │  └─────────────────────┘  └─────────────────────────────────────┘   │   │
│  └──────────────────────────────────────────────────────────────────────┘   │
│                                      │                                       │
│  ┌──────────────────────────────────────────────────────────────────────┐   │
│  │                      ORCHESTRATION LAYER (A5-A8)                     │   │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐ │   │
│  │  │ Orchestrator│  │  Sessions   │  │    Modes    │  │   Personas  │ │   │
│  │  │    (A5)     │  │    (A6)     │  │    (A7)     │  │    (A8)     │ │   │
│  │  └─────────────┘  └─────────────┘  └─────────────┘  └─────────────┘ │   │
│  └──────────────────────────────────────────────────────────────────────┘   │
│                                      │                                       │
│  ┌──────────────────────────────────────────────────────────────────────┐   │
│  │                          CORE LAYER (A1-A4)                          │   │
│  │  ┌─────────────────────────────────────────────────────────────┐    │   │
│  │  │                    GraphBit Core (A1)                       │    │   │
│  │  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐         │    │   │
│  │  │  │  Scheduler  │  │  Executor   │  │   Monitor   │         │    │   │
│  │  │  └─────────────┘  └─────────────┘  └─────────────┘         │    │   │
│  │  └─────────────────────────────────────────────────────────────┘    │   │
│  │  ┌─────────────────────┐  ┌─────────────────────────────────────┐   │   │
│  │  │   PyO3 Bridge (A2)  │  │        Memory Cube (A3)            │   │   │
│  │  │  • Python FFI       │  │  • SQLite Storage                  │   │   │
│  │  │  • Zero-Copy        │  │  • Knowledge Graph                 │   │   │
│  │  │  • Async Support    │  │  • Session Isolation               │   │   │
│  │  └─────────────────────┘  └─────────────────────────────────────┘   │   │
│  └──────────────────────────────────────────────────────────────────────┘   │
│                                      │                                       │
│  ┌──────────────────────────────────────────────────────────────────────┐   │
│  │                         TOOL LAYER (A9-A11)                          │   │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐ │   │
│  │  │ Oasis Hands │  │Oasis Browser│  │ Oasis Manus │  │   Scout     │ │   │
│  │  │  43+ Tools  │  │  Lightpanda │  │   Docker    │  │   Search    │ │   │
│  │  │    (A9)     │  │    (A10)    │  │    (A11)    │  │   (A9+)     │ │   │
│  │  └─────────────┘  └─────────────┘  └─────────────┘  └─────────────┘ │   │
│  └──────────────────────────────────────────────────────────────────────┘   │
│                                                                              │
│  ┌──────────────────────────────────────────────────────────────────────┐   │
│  │                        SKILL LAYER (5587+ Skills)                    │   │
│  │  ┌───────────────────────────────────────────────────────────────┐  │   │
│  │  │  Dev (2965+) │ OSINT (1050+) │ Social (238+) │ Media (246+)  │  │   │
│  │  │  Auto (306+) │ Prod (214+)   │ Security (52) │ Mobile (233+) │  │   │
│  │  └───────────────────────────────────────────────────────────────┘  │   │
│  └──────────────────────────────────────────────────────────────────────┘   │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 📦 Crate Structure

```
crates/
├── sentient_core/           # A1: GraphBit Core
│   ├── src/
│   │   ├── lib.rs        # Public API
│   │   ├── scheduler.rs  # Task scheduling
│   │   ├── executor.rs   # Task execution
│   │   └── monitor.rs    # Health monitoring
│   └── Cargo.toml
│
├── sentient_python/         # A2: PyO3 Bridge
│   ├── src/
│   │   ├── lib.rs        # Python module
│   │   ├── bridge.rs     # FFI bridge
│   │   └── converter.rs  # Type conversion
│   └── Cargo.toml
│
├── sentient_memory/         # A3: Memory Cube
│   ├── src/
│   │   ├── lib.rs        # Memory API
│   │   ├── storage.rs    # SQLite backend
│   │   ├── graph.rs      # Knowledge graph
│   │   └── cache.rs      # LRU cache
│   └── Cargo.toml
│
├── sentient_vgate/          # A4: V-GATE Proxy
│   ├── src/
│   │   ├── lib.rs        # Proxy API
│   │   ├── router.rs     # Request routing
│   │   ├── pool.rs       # Connection pool
│   │   └── keychain.rs   # Key management
│   └── Cargo.toml
│
├── sentient_orchestrator/   # A5: Multi-Agent Orchestration
│   ├── src/
│   │   ├── lib.rs        # Orchestrator API
│   │   ├── agent.rs      # Agent lifecycle
│   │   ├── task.rs       # Task management
│   │   └── healing.rs    # Self-healing
│   └── Cargo.toml
│
├── sentient_session/        # A6: Session Management
│   ├── src/
│   │   ├── lib.rs        # Session API
│   │   ├── manager.rs    # Session lifecycle
│   │   └── persistence.rs # State persistence
│   └── Cargo.toml
│
├── sentient_modes/          # A7: Behavioral Modes
│   ├── src/
│   │   ├── lib.rs        # Mode API
│   │   ├── engine.rs     # Mode engine
│   │   └── transitions.rs # Mode transitions
│   └── Cargo.toml
│
├── sentient_persona/        # A8: Identity Management
│   ├── src/
│   │   ├── lib.rs        # Persona API
│   │   ├── identity.rs   # Identity core
│   │   └── traits.rs     # Personality traits
│   └── Cargo.toml
│
├── oasis_hands/          # A9: Tool Library (43+)
│   ├── src/
│   │   ├── lib.rs        # Tool API
│   │   ├── registry.rs   # Tool registry
│   │   └── tools/        # Tool implementations
│   └── Cargo.toml
│
├── oasis_browser/        # A10: Browser Automation
│   ├── src/
│   │   ├── lib.rs        # Browser API
│   │   ├── lightpanda.rs # FFI bindings
│   │   └── automation.rs # Automation layer
│   └── Cargo.toml
│
├── oasis_manus/          # A11: Docker Sandbox
│   ├── src/
│   │   ├── lib.rs        # Manus API
│   │   ├── sandbox.rs    # Sandbox management
│   │   └── executor.rs   # Code execution
│   └── Cargo.toml
│
├── sentient_guardrails/     # A12: Security Layer
│   ├── src/
│   │   ├── lib.rs        # Guardrails API
│   │   ├── filter.rs     # Content filtering
│   │   ├── validator.rs  # Input validation
│   │   └── rate_limit.rs # Rate limiting
│   └── Cargo.toml
│
├── sentient_cli/            # Command Line Interface
├── sentient_gateway/        # API Gateway
├── sentient_ingestor/       # Skill Ingestion
├── sentient_selfcoder/      # Self-Improvement
├── sentient_graph/          # Graph Processing
├── sentient_storage/        # Storage Abstraction
├── sentient_scout/          # Search & Discovery
├── sentient_forge/          # Code Generation
├── sentient_sandbox/        # Isolated Execution
├── sentient_checkpoint/     # Checkpoint System
├── sentient_reporting/      # Reporting Engine
└── sentient_common/         # Shared Utilities
```

---

## 🔄 Data Flow

```
User Request
     │
     ▼
┌─────────────┐
│ SENTIENT Shell │
│   /skill    │
└──────┬──────┘
       │
       ▼
┌─────────────┐     ┌─────────────┐
│  Guardrails │────►│  Validator  │
│   (A12)     │     │   Check     │
└──────┬──────┘     └─────────────┘
       │
       ▼
┌─────────────┐     ┌─────────────┐
│ Orchestrator│────►│   Session   │
│    (A5)     │     │    (A6)     │
└──────┬──────┘     └─────────────┘
       │
       ▼
┌─────────────┐     ┌─────────────┐
│ GraphBit    │────►│   Memory    │
│ Core (A1)   │     │  Cube (A3)  │
└──────┬──────┘     └─────────────┘
       │
       ▼
┌─────────────┐     ┌─────────────┐
│  V-GATE     │────►│    LLM      │
│ Proxy (A4)  │     │   Provider  │
└─────────────┘     └─────────────┘
```

---

## 🔐 Security Model

### V-GATE Architecture (A4)

```
┌─────────────────────────────────────────────────┐
│                   V-GATE PROXY                  │
├─────────────────────────────────────────────────┤
│                                                 │
│  ┌─────────────┐    ┌─────────────────────┐    │
│  │   Client    │    │   Encrypted Keys    │    │
│  │  Request    │    │   (Server-Side)     │    │
│  └──────┬──────┘    └──────────┬──────────┘    │
│         │                      │                │
│         ▼                      ▼                │
│  ┌─────────────────────────────────────────┐   │
│  │            Request Router               │   │
│  │  • Load Balancing                       │   │
│  │  • Rate Limiting                        │   │
│  │  • Key Injection                        │   │
│  └──────────────────────┬──────────────────┘   │
│                         │                       │
│         ┌───────────────┼───────────────┐      │
│         ▼               ▼               ▼      │
│  ┌───────────┐  ┌───────────┐  ┌───────────┐  │
│  │ OpenRouter│  │  OpenAI   │  │ Anthropic │  │
│  └───────────┘  └───────────┘  └───────────┘  │
│                                                │
└────────────────────────────────────────────────┘

NO API KEYS IN CLIENT CODE!
```

### Guardrails (A12)

| Layer | Protection |
|-------|------------|
| Input | Prompt injection detection |
| Process | Content filtering |
| Output | Data leak prevention |
| Rate | Request throttling |

---

## 📊 Performance Characteristics

| Metric | Value |
|--------|-------|
| Startup Time | < 100ms |
| Memory Footprint | ~50MB base |
| Skill Load | < 1ms per skill |
| LLM Latency | Provider-dependent |
| Concurrent Agents | Unlimited (async) |

---

## 🔧 Configuration

### Environment Variables

```bash
# V-GATE Configuration
VGATE_HOST=localhost
VGATE_PORT=8080

# Memory Cube
MEMORY_DB_PATH=./data/memory.db

# Logging
RUST_LOG=info

# No API keys needed!
```

### Cargo.toml Workspace

```toml
[workspace]
members = [
    "crates/sentient_core",
    "crates/sentient_memory",
    "crates/sentient_vgate",
    # ... 25 total crates
]
resolver = "2"
```

---

## 🚀 Deployment

### Binary Distribution

```bash
# Build all binaries
cargo build --release

# Resulting binaries
target/release/
├── sentient-shell       # Hybrid terminal
├── sentient-dashboard   # Web dashboard
├── sentient-ingest      # Skill ingestion
├── sentient-selfcoder   # Self-improvement
└── sentient-gateway     # API gateway
```

### Docker

```dockerfile
FROM rust:1.75 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/* /usr/local/bin/
CMD ["sentient-shell"]
```

---

## 📚 Integration Points

| Integration | Method | Status |
|-------------|--------|--------|
| OpenClaw Skills | YAML Ingestion | ✅ Active |
| MemOS | SQLite Bridge | ✅ Active |
| browser-use | FFI (A10) | ✅ Active |
| OpenManus | Docker (A11) | ✅ Active |
| MoltGuard | Guardrails (A12) | ✅ Active |

---

## 🔮 Future Roadmap

| Phase | Target | Status |
|-------|--------|--------|
| Phase 1 | Core + Memory | ✅ Complete |
| Phase 2 | V-GATE + Guardrails | ✅ Complete |
| Phase 3 | Tools + Browser | ✅ Complete |
| Phase 4 | Self-Improvement | ✅ Complete |
| Phase 5 | Community Skills | ✅ 5587+ |
| Phase 6 | Public Launch | 🚀 Now |

---

*Architecture Version: 1.0.0*  
*Last Updated: 2026-04-06*
