# LiveUI Integration

## Leptos Path (recommended for pure Rust)

Use **detaxine-ui** or **leptos-shadcn-ui** for production components:

```bash
# Add to leptos-frontend/Cargo.toml
detaxine-ui = "0.8"
# or
leptos-shadcn-ui = "0.7"
```

Example component structure:
```
crates/leptos-frontend/src/
├── components/
│   ├── ui/          # copied from registry
│   ├── chat.rs      # live chat with streaming
│   └── approval.rs  # tool approval dialogs
└── app.rs
```

## Svelte Path (fastest HMR)

Use **shadcn-svelte**:

```bash
cd crates/tauri-app
bunx shadcn-svelte@latest init
bunx shadcn-svelte@latest add button dialog toast
```

## Live Rendering Features

- Streaming token-by-token via ADK events
- Tool approval dialogs (human-in-the-loop)
- Session list with real-time updates
- Computer-use screenshot preview
- OpenTelemetry traces in debug panel
- Engram memory graph visualization
- Skills browser with progressive disclosure
