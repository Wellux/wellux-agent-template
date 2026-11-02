# Contributing

Keep the core the only place policy lives.

- `make test` before you open a pull request. CI runs the same command plus `cargo fmt --check`.
- If you add a file, add one line to `docs/MAP.md`.
- If you claim a feature in the README, the file has to exist. Recipes go in `LIVEUI.md` or `docs/NEXT.md`, not in the feature table as done.
- A new checkout is named with `make project NAME=...` once. Do not rename `agent-core`.
- Skills are `skills/<kebab-name>/SKILL.md`. Start from `make skill NAME=...`.
- Do not vendor a frontend that reimplements guardrails.

Licensed under MIT OR Apache-2.0, the same as the rest of the tree.
