# Start

Rust 1.80 or newer. A network connection the first time Cargo fetches crates. Nothing else is required to prove the core.

```bash
git clone https://github.com/Wellux/wellux-agent-template.git
cd wellux-agent-template
cp .env.example .env
make project NAME=my-agent REPO=https://github.com/OWNER/my-agent
make doctor
make test
```

## Profiles

| Goal | Command | What it turns on |
|------|---------|------------------|
| Personal companion, local memory | `make personal` | `memory-engram` |
| Business agent, traces, isolated code | `make business` | `otel`, `mcp`, `memory-engram`, `sandbox-microvm` |
| Desktop agent, human approval | `make desktop` | `computer-use`, both sandboxes, `memory-engram` |

The feature flags compile today as a switchboard. They do not download Engram or boot a VM. Wire those when `make doctor` says they are missing.

### Personal

```bash
brew install gentleman-programming/tap/engram
# or: go install github.com/Gentleman-Programming/engram/v2@latest
make personal
```

Put the provider key you actually use into `.env`. The recommended gateway is OrcaRouter (`ORCAROUTER_API_KEY`, base `https://api.orcarouter.ai/v1`). Ollama needs no key.

### Business

Set `OTEL_EXPORTER_OTLP_ENDPOINT` when a collector is running. Until then `init_otel` reports the exporter as off and the process still starts.

### Desktop

Computer-use tools (`screenshot`, `mouse`, `keyboard`) are marked approval-required by `apply_guardrails_for`. Quicksand is optional:

```bash
pip install 'quick-sandbox[qemu,ubuntu]'
```

## A skill

```bash
make skill NAME=release-notes
```

Edit `skills/release-notes/SKILL.md`. The loader indexes `name` and `description` only, and reads the body when something calls `SkillsLoader::load`.

## What this will not do

`bun run tauri dev` is not a step yet. The Tauri, Svelte and Leptos shells are specified in `LIVEUI.md` and listed as the first next move in `docs/NEXT.md`. The previous quick start pointed at `crates/tauri-app`, which was not in the tree.
