# Ten next moves

These are not in the tree. They are the shortest path from this starter to a product, in the order that keeps the core honest.

## 1. Tauri shell as a crate

Add `crates/tauri-app` with one window: a transcript, a streaming reply, and an approval dialog bound to `GuardrailOutcome.approval_required`. Do not reimplement redaction in TypeScript.

## 2. One UiEvent for both frontends

Svelte and Leptos should emit the same events (`Token`, `ToolRequested`, `ToolResolved`, `MemoryHit`). If the event type lives in `agent-core`, the shells cannot drift.

## 3. Doctor that talks to Engram

`scripts/doctor.sh` currently checks that `engram` is on `PATH`. Next: `engram --version` and a one-shot MCP initialize, so a broken binary fails the doctor instead of failing the first chat.

## 4. Approval queue

`apply_guardrails_for` already flags `screenshot`, `mouse`, `keyboard`, `shell` and `sandbox.exec`. The missing piece is a blocking queue the shell renders. Default stays closed.

## 5. OTel without a collector

`init_otel` only reports whether `OTEL_EXPORTER_OTLP_ENDPOINT` is set. A ring buffer of the last 20 spans lets a desktop build show traces before anyone runs a collector.

## 6. Bundle identity for a desktop shell

`make project NAME=` now writes `project.toml`, the agent name and the first skill, and it will not run twice. What it does not do yet is rename a Tauri bundle id or a window title, because that crate is not in the tree. When `crates/tauri-app` exists, the same command should refuse to leave the bundle id as `com.wellux.template`.

## 7. sandbox.toml

Feature flags select the profile. Image name and memory should not hide in code. `SandboxManager::spec` is the function that should read the file.

## 8. Signed skills

`SkillsLoader::index` trusts every folder. A `skills.lock` of hashes, checked before `load`, keeps a dropped-in `SKILL.md` from becoming procedure.

## 9. Ten golden tasks

`evals/` with fixture prompts and expected redaction or skill selection. No API key. CI already runs offline tests; this extends that promise past unit tests.

## 10. A public companion

A second repository, `wellux-companion`, that depends on this template and does nothing else. Readers then see a product instead of guessing which recipe to build first.
