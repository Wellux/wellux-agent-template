<picture>
  <source media="(prefers-color-scheme: dark)" srcset=".github/assets/header-dunkel.svg">
  <img alt="Wellux Agent Native Project Template: Shared Rust core, Engram second brain, Hermes skills, guardrails, OpenTelemetry, native VMs and OrcaRouter." src=".github/assets/header-hell.svg" width="100%">
</picture>

<p align="center">
  <img alt="Rust" src="https://img.shields.io/badge/Rust-1.80+-1B8DBE?style=flat-square&logo=rust">
  <img alt="Tauri" src="https://img.shields.io/badge/Tauri-2.0-1B8DBE?style=flat-square&logo=tauri">
  <img alt="ADK-Rust" src="https://img.shields.io/badge/ADK--Rust-2.3-1B8DBE?style=flat-square">
  <img alt="License" src="https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-95969A?style=flat-square">
</p>

<h3 align="center">Wellux Agent Native Project Template</h3>

<p align="center">
  Build private and business agent-native applications that own their intelligence.<br>
  Shared Rust core · Engram second brain · Hermes-style skills · Guardrails · OpenTelemetry · Native VMs
</p>

<p align="center">
  <a href="https://well-ux.com"><b>well-ux.com</b></a> &nbsp;·&nbsp;
  <a href="#quick-start">Quick Start</a> &nbsp;·&nbsp;
  <a href="#architecture">Architecture</a> &nbsp;·&nbsp;
  <a href="#features">Features</a>
</p>

---

## Why Wellux?

Most agent stacks lock you into someone else’s cloud and constraints.  
**Wellux gives you a production-grade foundation you control.**

- Local-first by default  
- Shared Rust core reusable across platforms  
- Guardrails, OpenTelemetry and isolated sandboxes built in  
- Second brain + progressive skills out of the box  

Whether you are building a personal companion or an enterprise agent platform — start here.

---

## Features

| Capability | Implementation | Status |
|------------|----------------|--------|
| **Shared Rust Core** | Domain, database and sync reusable across platforms | ✅ |
| **Multi-Provider** | OpenAI, Anthropic, Gemini, xAI, **OrcaRouter**, Ollama | ✅ |
| **Second Brain** | Engram (SQLite + FTS5 via MCP) | ✅ |
| **Skills Loader** | Hermes-style progressive disclosure (`SKILL.md`) | ✅ |
| **Computer Use** | Screenshot, mouse and keyboard control | ✅ |
| **Guardrails** | Tool approval + PII redaction + content filters | ✅ |
| **OpenTelemetry** | GenAI semantic conventions via `adk-telemetry` | ✅ |
| **Native VMs** | microsandbox MicroVMs + Quicksand Bridge | ✅ |
| **Dual Frontend** | Svelte 5 (fast HMR) or Leptos (pure Rust) | ✅ |
| **LiveUI** | shadcn-style components for both paths | ✅ |

---

## Architecture

<picture>
  <source media="(prefers-color-scheme: dark)" srcset=".github/assets/arch-dunkel.svg">
  <img alt="Architecture: Svelte/Leptos and Tauri feed into Shared Rust Core, then agent-core and providers." src=".github/assets/arch-hell.svg" width="100%">
</picture>

Shared Rust core sits in the middle. Frontends and Tauri shell on the left, agent runtime and providers on the right.

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

Drop `SKILL.md` files into `skills/` and the agent lists and loads them on demand.

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

<p align="center"><sub>Built by <a href="https://github.com/Wellux">Marco Mengele · Wellux</a> · One block at a time.</sub></p>
