# Map

One page. If a promise is not in this list, it is not in the repo.

| Path | Owns |
|------|------|
| `crates/agent-core/src/lib.rs` | Public surface. Frontends import from here only. |
| `access.rs` | Which credentials may enter the browser. Tested. |
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
| `docs/START.md` | The only quick start. |
| `docs/NEXT.md` | Work that is intentionally absent. |
| `SECOND_BRAIN.md` | Engram and Hermes, in German. |
| `LIVEUI.md` | How a Svelte or Leptos shell should look. |
| `Makefile` | `check`, `test`, `personal`, `business`, `desktop`, `skill`, `doctor`. |

Do not add a frontend crate until it speaks the core types. A second policy implementation is how this template stops being one block.
