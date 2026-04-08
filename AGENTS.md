# ═══════════════════════════════════════════════════════════════════════════════
#  SENTIENT OS - PROJECT MISSION AND ARCHITECTURE
# ═══════════════════════════════════════════════════════════════════════════════

## MISSION AND IDENTITY

**Project Name:** SENTIENT OS  
**Developer:** Pi  
**Purpose:** Build an autonomous, secure, and high-performance AI Operating System with a Rust core.

This project is designed as an **integration platform** that enables different open-source projects to work harmoniously within the SENTIENT ecosystem. All architecture is based on modularity and extensibility principles.

---

## CORE ARCHITECTURE COMPONENTS

### 1. CENTRAL NERVOUS SYSTEM (Core)

The system's main orchestration is built on a Rust-based event-graph framework:

- Low memory consumption
- Lock-free concurrency
- High-performance event processing

**Location:** `crates/sentient_core/`, `crates/sentient_graph/`

### 2. PyO3 Bridge (Integration Layer)

Python-based tools are integrated into the Rust core as **native modules** using PyO3 library:

- Zero-copy data flow
- Type-safe bridges
- Automatic error conversion

**Location:** `crates/sentient_python/`

### 3. COGNITIVE MEMORY (Hippocampus)

All agent history, short/long-term memory, and knowledge graphs are managed in a memory system running on local SQLite:

- Episodic memory (experiences)
- Semantic memory (information)
- Procedural memory (methods)

**Location:** `crates/sentient_memory/`

### 4. WEB INTERACTION MODULE

Internet interactions via browser-use and Lightpanda integrations:

- Headless browser control
- Web scraping
- Form filling and automation

**Location:** `crates/oasis_browser/`

### 5. SANDBOX EXECUTION MODULE

Isolated code execution in Docker:

- Secure code execution
- Resource isolation
- Timeout management

**Location:** `crates/oasis_manus/`, `crates/sentient_sandbox/`

### 6. SECURITY SYSTEM (Guardrails)

Security filters for all input/output layers:

- Prompt injection detection
- Data leak prevention
- SQL injection protection

**Location:** `crates/sentient_guardrails/`

### 7. V-GATE ARCHITECTURE (API Security)

**API keys are NEVER in client code.**

```
┌─────────────┐      ┌─────────────┐      ┌─────────────┐
│   SENTIENT  │      │   V-GATE    │      │   LLM API   │
│   Client    │─────▶│   Proxy     │─────▶│   Provider  │
└─────────────┘      └─────────────┘      └─────────────┘
                           │
                     API Key (Secure)
                     Stored on server
```

---

## ERROR MANAGEMENT AND IDENTITY

Raw errors from external systems (TypeError, Traceback, etc.) are not shown directly to the user. All outputs are converted to SENTIENT's professional format.

**Example:**
```
❌ Python: TypeError: 'NoneType' object is not subscriptable
✅ SENTIENT: DATA_MODULE: Requested data not found. Please check parameters.
```

---

## DEVELOPMENT METHODOLOGY

### Coding Standards

1. **Language:** Rust (core), Python (integrations)
2. **Testing:** Unit tests mandatory for each module
3. **Documentation:** Explanatory comments in code

### Models Used

- `qwen/qwen3.6-plus:free` - General tasks
- `qwen/qwen3-coder:free` - Code writing

### Workflow

1. Develop modules hierarchically
2. Compile and test at each step (`cargo build`, `cargo test`)
3. Analyze and fix errors
4. Verify stability

---

## INTEGRATION TABLE

| Category | Projects | License |
|----------|----------|---------|
| Agent Framework | CrewAI, AutoGen, OpenHands | MIT |
| Browser | Browser-Use, Lightpanda | MIT |
| Memory | Mem0, MemGPT, ChromaDB | MIT/Apache-2 |
| Security | NeMo-Guardrails | Apache-2 |
| Sandbox | OpenManus, Open Interpreter | MIT/AGPL-3 |

**Detailed license information:** `THIRD_PARTY_NOTICES.md`

---

## PROJECT STATUS

| Metric | Value |
|--------|-------|
| Rust Crate | 37 |
| Rust Files | 553 |
| Tests | 547 ✅ |
| Integrated Projects | 71 |

---

*SENTIENT OS - The Operating System That Thinks*
