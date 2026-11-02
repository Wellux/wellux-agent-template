# Second Brain & Agent Memory

## Empfohlene Lösung: Engram (Gentleman-Programming/engram)

**Beste lokale Second-Brain-Lösung für Agenten.**

- Single Go-Binary, Zero Dependencies
- SQLite + FTS5 (eine Datei: `~/.engram/engram.db`)
- MCP-stdio mit ~15–20 Tools (`mem_save`, `mem_search`, `mem_context`, `mem_session_summary`, `mem_timeline` …)
- Agent entscheidet selbst, was gespeichert wird (kein automatisches Rauschen)
- Git-Sync für portable Chunks möglich
- Funktioniert mit jedem MCP-Client (Claude Code, Cursor, ADK, Hermes …)

### Integration in dieses Template

```bash
# Installieren
brew install gentleman-programming/tap/engram
# oder
go install github.com/Gentleman-Programming/engram/v2@latest

# MCP starten (stdio)
engram mcp
```

In `agent.rs` oder via Feature `memory-engram` wird es automatisch als MCP-Toolset geladen.

### Alternativen

| System              | Stärke                          | Empfehlung                  |
|---------------------|---------------------------------|-----------------------------|
| **Engram (Gentleman)** | Lokal, FTS5, MCP, zero-dep     | **Primär**                  |
| Engram (engram.fyi) | Semantisch + Knowledge Graph   | Gut für Cloud/Team          |
| Mem0                | Gute Benchmarks, Graph optional| Managed oder self-hosted    |
| Hermes Memory       | Teil des Hermes-Agenten        | Siehe Hermes-Integration    |

## Hermes Agent (Nous Research)

Hermes ist ein **selbstverbessernder Agent** mit Learning Loop, Skills-System und starker Memory-Architektur. Er ist Python-basiert und eignet sich hervorragend als:

1. **Externer Agent** über MCP (`hermes mcp-serve`)
2. **Quelle für Patterns** (Skills, User Modeling, Session Search)
3. **Modell-Provider** über Nous Portal / OpenRouter

### Empfohlene Integration

```yaml
# In deiner MCP-Config oder via tools::load_mcp_tools
command: hermes
args: ["mcp-serve", "--tools"]
```

Damit bekommt dein ADK-Agent Zugriff auf Hermes’ Skills, Web-Search, Browser, Vision etc., während der Rust-Core die Desktop-Shell und Guardrails steuert.

### Skills-Pattern (inspiriert von Hermes)

Skills leben als Markdown-Dateien (`SKILL.md`) und werden nur bei Bedarf geladen (Progressive Disclosure). Das Template unterstützt das über MCP-Tools oder einen eigenen `skills/`-Ordner.

## Quicksand (Microsoft)

Rootless QEMU-VMs (Ubuntu/Alpine, optional Desktop) – ideal wenn du volle Linux-Umgebungen mit GUI, Snapshots und Datei-Mounts brauchst. Wird als optionaler Sandbox-Backend unterstützt (Feature `sandbox-quicksand`).

**Empfehlung:** `microsandbox` für schnelle MicroVMs, Quicksand wenn du ein volles Desktop-Linux brauchst.
