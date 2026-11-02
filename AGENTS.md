# Agent instructions

Someone gave you this repository so you would create a project from it. Do the steps below. Do not redesign the template, and do not invent a second policy next to `crates/agent-core`.

## If the message contains a GitHub link

You have enough to start when you can see one or both of these:

- the template: `https://github.com/Wellux/wellux-agent-template`
- their empty repository: `https://github.com/<owner>/<name>`

If you are not already inside a checkout of the template, clone it:

```bash
git clone https://github.com/Wellux/wellux-agent-template.git
cd wellux-agent-template
```

Then run exactly one of these.

They named the project and gave a destination repo:

```bash
make project NAME=<kebab-name> REPO=https://github.com/<owner>/<name>
```

They gave only the destination repo. The name is taken from the repo:

```bash
make project REPO=https://github.com/<owner>/<name>
```

They gave only this template and a name, and no destination:

```bash
make project NAME=<kebab-name>
```

If they gave neither a name nor a destination repo, ask once. Do not invent a product name.

## Then prove it

```bash
make test
make doctor
```

If `make test` fails, stop. Report the first error. Do not keep editing until it is green.

`make doctor` prints what is missing. Engram and Quicksand are optional. Do not install them unless the person asked for that profile.

## Hard rules

- Never commit `.env`. Never paste its values into a reply, a skill, or `ui/`.
- Never put `SUPABASE_SERVICE_ROLE_KEY`, `PAYPAL_CLIENT_SECRET`, or `OPENAI_API_KEY` in the browser, in `ui/`, or in git.
- Do not push. The script sets `origin` to their repo and keeps this template as `upstream`. Push only after they explicitly say so.
- Do not edit `crates/agent-core` to make a door "work". Add a skill under `skills/`, change `ui/`, or add a door in `crates/shell/src/surface.rs`.
- Do not claim PayPal, Supabase, or the live API are connected. A door opens only when its keys are in `.env`.
- The destination URL must be `https://github.com/owner/name`. Reject anything else. The template URL is not a destination.

## Done

Say the project name, the destination repo if there is one, and that tests passed. The next command they can run is `make personal`, `make business`, `make desktop`, or `make shell`.
