# Start a project

This repository is a template. You do not develop inside it as "Wellux". You copy it once and give the copy your name.

## 1. Take a copy

On GitHub, open the repository and choose **Use this template**, then **Create a new repository**. That gives you your own history, not a fork you have to keep in sync.

If you cannot use that button:

```bash
git clone https://github.com/Wellux/wellux-agent-template.git my-agent
cd my-agent
rm -rf .git
git init
```

## 2. Name it

```bash
cp .env.example .env
make project NAME=my-agent
```

`make project` does four things and then refuses to run again:

- writes your name into `project.toml`
- copies `.env.example` to `.env` if you skipped that
- sets `WELLUX_AGENT_NAME`
- stamps `skills/my-agent/SKILL.md` from `skills/_template`

The name is kebab-case: lowercase letters, digits, hyphens. `My Agent` is rejected. `my-agent` is accepted.

## 3. Prove it, then pick a profile

```bash
make doctor
make test
make personal      # companion, local memory
# make business    # traces, MCP, a MicroVM for untrusted code
# make desktop     # computer use, a person must approve
```

`make test` needs no keys and no network after the first Cargo fetch. A red test means you do not have a starting point yet. Do not continue past it.

`make doctor` tells you what is missing. Engram and Quicksand are optional. The core compiles without them.

## 4. Open the shell

```bash
make shell
```

The page draws one card per door. A door stays locked until the matching keys in `.env` are filled. Nothing is billed and nobody is signed in until you fill those keys and press the button.

| Door | You fill | Then |
|------|----------|------|
| Supabase | `SUPABASE_URL`, `SUPABASE_PUBLISHABLE_KEY` | Email login, or GitHub / Google |
| PayPal | `PAYPAL_CLIENT_ID`, `PAYPAL_CLIENT_SECRET` | Sandbox checkout. Leave `PAYPAL_ENV=sandbox` |
| OpenAI | nothing, for the first ChatGPT login | Live API needs `OPENAI_API_KEY` or a granted plan scope |

The service-role key, the PayPal secret and `OPENAI_API_KEY` stay in `.env`. They are gitignored. Do not paste them into `ui/`.

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

A new door: add it in `crates/shell/src/surface.rs`. Reload the shell. The page picks it up because it renders `/api/surface`, not a card you drew by hand.

A different look: replace the `ui/` folder. Keep calling the same routes.

## 6. When you are stuck

Open an issue with the starter template. Say which profile, the command you ran, and the first error. Do not paste `.env`.
