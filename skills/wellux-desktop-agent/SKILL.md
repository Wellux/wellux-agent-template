---
name: wellux-desktop-agent
description: Core patterns for building secure desktop agents with Wellux.
version: 1.0.0
metadata:
  hermes:
    tags: [desktop, agent, tauri, guardrails]
---

# Wellux Desktop Agent

## When to Use
When building or extending agent-native desktop applications on the Wellux template.

## Procedure
1. Use the shared Rust core for all domain logic.
2. Route model calls through OrcaRouter or direct providers.
3. Enable Engram for persistent second-brain memory.
4. Require tool approval for computer-use and destructive actions.
5. Spin up microsandbox VMs for untrusted code.

## Pitfalls
- Never bypass guardrails in production.
- Keep secrets out of the sandbox.

## Verification
Confirm OpenTelemetry traces appear and tool approvals are enforced.
