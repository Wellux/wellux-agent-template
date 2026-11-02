//! Shared Wellux agent core.
//!
//! Frontends (Tauri, Svelte, Leptos) stay thin. Domain, policy and provider
//! routing live here so every shell reuses the same behavior.
//!
//! Feature flags (`mcp`, `computer-use`, `memory-engram`, `sandbox-*`, `otel`)
//! are the switchboard documented in the README. This crate compiles with or
//! without them: flags select a profile, they do not pull hidden services.

pub mod agent;
pub mod computer_use;
pub mod guardrails;
pub mod providers;
pub mod quicksand_bridge;
pub mod sandbox;
pub mod session_service;
pub mod sessions;
pub mod skills;
pub mod telemetry;
pub mod tools;

pub use agent::{create_agent, AgentSession};
pub use computer_use::computer_use_tools;
pub use guardrails::{apply_guardrails, apply_guardrails_for, GuardrailOutcome};
pub use providers::{create_provider, ProviderEndpoint, ProviderKind};
pub use quicksand_bridge::QuicksandBridge;
pub use sandbox::{SandboxManager, SandboxRuntime, SandboxSpec};
pub use session_service::{create_session_service, LibsqlSessionService, SessionRecord};
pub use sessions::AgentSessionStore;
pub use skills::SkillsLoader;
pub use telemetry::init_otel;
pub use tools::{load_engram_memory, load_hermes_mcp, ToolSpec};
