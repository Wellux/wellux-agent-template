<div align="center">

<img src="./assets/logo.png" alt="Wellux" width="120"/>

# Wellux

### The AI Operating System Starter Kit

**Build private & business agent-native applications that own their intelligence.**

[![Rust](https://img.shields.io/badge/Rust-1.80+-orange?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)
[![Tauri](https://img.shields.io/badge/Tauri-2.0-blue?style=for-the-badge&logo=tauri)](https://tauri.app/)
[![ADK-Rust](https://img.shields.io/badge/ADK--Rust-2.3-green?style=for-the-badge)](https://adk-rust.com)
[![License](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue?style=for-the-badge)](LICENSE)

[Documentation](#quick-start) · [Architecture](#architecture) · [Features](#features) · [Wellux myOS](https://well-ux.com)

<img src="./assets/hero.png" alt="Wellux myOS Interface" width="100%"/>

</div>

---

## Why Wellux?

Most agent frameworks lock you into someone else’s cloud.  
**Wellux gives you the stack to build the AI OS you actually own.**

- Local-first by default  
- Shared Rust core that scales from desktop → mobile → IoT  
- Production-grade guardrails, telemetry and sandboxes  
- Second brain + self-improving skills out of the box  

Whether you’re building a personal AI companion or an enterprise agent platform — start here.

---

## Features

| Capability | Implementation | Status |
|------------|----------------|--------|
| **Shared Rust Core** | Domain + DB + Sync reusable across platforms | ✅ |
| **Multi-Provider** | OpenAI, Anthropic, Gemini, xAI, **OrcaRouter**, Ollama | ✅ |
| **Second Brain** | Engram (SQLite + FTS5 via MCP) | ✅ |
| **Skills Loader** | Hermes-style progressive disclosure (`SKILL.md`) | ✅ |
| **Computer Use** | Screenshot + Mouse + Keyboard (enigo) | ✅ |
| **Guardrails** | Tool approval + PII redaction + ContentFilter | ✅ |
| **OpenTelemetry** | GenAI semantic conventions via `adk-telemetry` | ✅ |
| **Native VMs** | microsandbox MicroVMs + Quicksand Bridge | ✅ |
| **Dual Frontend** | Svelte 5 (fast HMR) or Leptos (pure Rust) | ✅ |
| **LiveUI** | shadcn-style components for both paths | ✅ |

---

## Architecture

```
┌──────────────────────────────────────────────────────────────┐
│                        Wellux myOS                          │
│  ┌─────────────┐  ┌──────────────┐  ┌──────────────────┐  │
│  │  Svelte 5   │  │   Leptos     │  │   Tauri Shell     │  │
│  │  (HMR)      │  │  (pure Rust) │  │  Desktop + Mobile │  │
│  └──────┬──────┘  └──────┬───────┘  └─────────┬─────────┘  │
│         └────────────────┼─────────────────────┘            │
│                          ▼                                 │
│              ┌────────────────────────┐                     │
│              │   Shared Rust Core    │                     │
│              │  domain · db · sync   │                     │
│              └───────────┬───────────┘                     │
│                          ▼                                 │
│              ┌──────────────────────────┐                     │
│              │     agent-core        │                     │
│              │ ADK · Providers       │                     │
│              │ Guardrails · OTel     │                     │
│              │ Engram · Skills       │                     │
│              │ Sandbox · Computer-Use│                     │
│              └──────────────────────────┘                     │
└────────────────────────────────────────────────────────────────────────┘
```

---

## Quick Start

### Prerequisites

- Rust 1.80+
- Bun or Node.js
- (Optional) API keys, Engram binary, Hermes

```bash
# Clone
git clone https://github.com/Wellux/wellux-agent-template.git
cd wellux-agent-template
cp .env.example .env

# Install Engram (recommended)
brew install gentleman-programming/tap/engram

# Run
cd crates/tauri-app
bun install
bun run tauri dev
```

### Enable Features

```bash
# Full agent stack
cargo build -p agent-core --features mcp,computer-use,memory-engram,sandbox-microvm
```

---

## Second Brain & Skills

Wellux ships with two complementary memory systems:

- **Engram** — persistent, searchable second brain (MCP)
- **Skills Loader** — Hermes-inspired progressive disclosure for procedural knowledge

Drop `SKILL.md` files into `skills/` and the agent can list and load them on demand.

See [`SECOND_BRAIN.md`](SECOND_BRAIN.md) for full details.

---

## Sandboxes

Agents can spin up isolated environments:

| Runtime | Use Case | Feature Flag |
|---------|----------|--------------|
| **microsandbox** | Fast MicroVMs (recommended) | `sandbox-microvm` |
| **Quicksand** | Full QEMU Linux + Desktop | `sandbox-quicksand` |
| **WASM** | Lightweight sandboxes | `sandbox-wasm` |

---

## Production Checklist

- [x] Shared Core with adapter pattern
- [x] Multi-provider (incl. OrcaRouter)
- [x] Engram Second Brain
- [x] Hermes-style Skills Loader
- [x] Tool Approval + Guardrails
- [x] OpenTelemetry
- [x] Native VMs (microsandbox + Quicksand Bridge)
- [x] Computer-Use tools
- [x] Dual frontend paths
- [x] Feature flags & CI/CD

---

## License

MIT OR Apache-2.0

---

<div align="center">

**Built for the Intelligence Age.**

[well-ux.com](https://well-ux.com) · [GitHub](https://github.com/Wellux)

<img src="./assets/second-brain.png" alt="Second Brain" width="80%"/>

</div>
