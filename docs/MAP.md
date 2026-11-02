# Map

One page. If a promise is not in this list, it is not in the repo.

| Path | Owns |
|------|------|
| `crates/agent-core/src/lib.rs` | Public surface. Frontends import from here only. |
| `access.rs` | Credential boundary and the door functions. Tested. |
| `agent.rs` | `create_agent`, session id, approval on by default. |
| `crates/shell` | Local UI. Loopback only. Cards from `surface.rs`. |
| `ui/` | The base interface. Replace this folder to change the look. |
| `docs/ACCESS.md` | Supabase, PayPal, ChatGPT sign-in, Realtime. |
| `providers.rs` | Provider table. No HTTP. |
| `guardrails.rs` | Redaction and the approval flag. |
| `skills.rs` | Hermes-style progressive disclosure. |
| `tools.rs` | Engram and Hermes tool names. |
| `computer_use.rs` | Screenshot, mouse, keyboard specs. |
| `sandbox.rs` | Which runtime to pick. |
| `quicksand_bridge.rs` | Process bridge. No VM is started from the tests. |
| `session_service.rs` | In-memory stand-in for libSQL. |
| `sessions.rs` | Id store. |
| `telemetry.rs` | OTel hook. |
| `skills/_template/SKILL.md` | Stamp copied by `make skill`. |
| `skills/wellux-desktop-agent/` | The shipped example skill. |
| `.github/assets/*.svg` | README graphics. Dark and light. Motion degrades to a still layout. |
| `.github/workflows/ci.yml` | Format, test, three profiles. |
| `project.toml` | This checkout's name. Set once by `make project`. |
| `.env.example` | Key names only. `.env` is gitignored. |
| `AGENTS.md` | What an agent runs when someone pastes the GitHub link. |
| `CLAUDE.md` | Pointer. Claude follows `AGENTS.md`. |
| `docs/START.md` | The 60-second path. |
| `docs/PROJECT.md` | How a third party starts their own project. |
| `docs/NEXT.md` | Work that is intentionally absent. |
| `SECOND_BRAIN.md` | Engram and Hermes, in German. |
| `LIVEUI.md` | How a Svelte or Leptos shell should look. |
| `Makefile` | `check`, `test`, `personal`, `business`, `desktop`, `skill`, `doctor`. |

Do not add a frontend crate until it speaks the core types. A second policy implementation is how this template stops being one block.
