# Start a project

You can hand the GitHub link to an agent. The agent reads [AGENTS.md](../AGENTS.md) and runs the commands. You can also run them yourself.

## Give this to an agent

Paste this. Replace the name and the destination. The destination is an empty repository you own, not this template.

```text
Create a project from this template. Follow AGENTS.md.

Template: https://github.com/Wellux/wellux-agent-template
Name: my-agent
Destination: https://github.com/OWNER/my-agent

Do not push. Do not commit .env. Stop if make test fails.
```

German, same contract:

```text
Lege ein Projekt aus dieser Vorlage an. Folge AGENTS.md.

Vorlage: https://github.com/Wellux/wellux-agent-template
Name: mein-agent
Ziel-Repo: https://github.com/OWNER/mein-agent

Nicht pushen. .env nicht committen. Stoppe, wenn make test rot ist.
```

If you only have the destination link, omit the name. `make project` takes it from the repo: `https://github.com/OWNER/mein-agent` becomes `mein-agent`.

The script refuses the template URL as a destination, so an agent cannot point `origin` back at this repository and push over it.

## 1. Take a copy

On GitHub, choose **Use this template**, then **Create a new repository**.

Or tell the agent the two links above and let it clone.

## 2. Name it and point it

```bash
make project NAME=my-agent REPO=https://github.com/OWNER/my-agent
```

This writes `project.toml`, sets `WELLUX_AGENT_NAME`, stamps `skills/my-agent/SKILL.md`, and — when the checkout is a git repo whose `origin` is still this template — renames that remote to `upstream` and sets `origin` to your repo.

It does not push. It refuses to run a second time.

`https://github.com/owner/name` and `git@github.com:owner/name.git` are accepted. Anything else is rejected.

## 3. Prove it, then pick a profile

```bash
make test
make doctor
make personal      # companion, local memory
# make business    # traces, MCP, a MicroVM for untrusted code
# make desktop     # computer use, a person must approve
```

`make test` needs no keys. A red test means stop.

## 4. Open the shell

```bash
make shell
```

The page draws one card per door and shows the destination repo when `project.toml` has one. A door stays locked until its keys in `.env` are filled.

| Door | You fill | Then |
|------|----------|------|
| Supabase | `SUPABASE_URL`, `SUPABASE_PUBLISHABLE_KEY` | Email login, or GitHub / Google |
| PayPal | `PAYPAL_CLIENT_ID`, `PAYPAL_CLIENT_SECRET` | Sandbox checkout. Leave `PAYPAL_ENV=sandbox` |
| OpenAI | nothing, for the first ChatGPT login | Live API needs `OPENAI_API_KEY` or a granted plan scope |

## 5. What you change, and what you leave

| You own | Leave alone |
|---------|-------------|
| `project.toml` | `crates/agent-core` policy |
| `skills/<name>/SKILL.md` | Guardrail tests |
| `ui/theme.css`, `ui/app.js` | The door list in `surface.rs`, unless you are adding a door |
| `.env` | `.env.example` as the list of names |

A new skill, after the first one:

```bash
make skill NAME=release-notes
```

A different look: replace the `ui/` folder. Keep calling the same routes.

## 6. When you are stuck

Open an issue with the starter template. Say which profile, the command you ran, and the first error. Do not paste `.env`.
