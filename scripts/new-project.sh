#!/usr/bin/env bash
# Turn this template into one named project. Safe to refuse, unsafe to repeat.
set -euo pipefail

name="${1:-}"
if [[ -z "$name" ]]; then
  echo "usage: scripts/new-project.sh <kebab-name>" >&2
  exit 1
fi
if [[ ! "$name" =~ ^[a-z0-9]+(-[a-z0-9]+)*$ ]]; then
  echo "name must be kebab-case: $name" >&2
  exit 1
fi

root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$root"
manifest="project.toml"
current="$(sed -n 's/^name = "\([^"]*\)"/\1/p' "$manifest" | head -1)"

if [[ "$current" != "wellux-agent-template" && "${FORCE:-}" != "1" ]]; then
  echo "this checkout is already '$current'." >&2
  echo "refusing to rename it. Set FORCE=1 only if you mean it." >&2
  exit 1
fi

cat > "$manifest" <<EOF
# This checkout's name. Set once by \`make project\`.
name = "$name"
profile = "personal"
EOF

if [[ ! -f .env ]]; then
  cp .env.example .env
fi
if grep -q '^WELLUX_AGENT_NAME=' .env; then
  # portable in-place edit
  tmp="$(mktemp)"
  sed "s/^WELLUX_AGENT_NAME=.*/WELLUX_AGENT_NAME=$name/" .env > "$tmp"
  mv "$tmp" .env
else
  printf '\nWELLUX_AGENT_NAME=%s\n' "$name" >> .env
fi

if [[ ! -e "skills/$name" ]]; then
  "$root/scripts/new-skill.sh" "$name"
fi

cat <<EOF

Project '$name' is ready.

  1. make doctor
  2. make personal     # or make business / make desktop
  3. make test
  4. make shell
  5. Fill the keys in .env for the doors you want open.

Your name lives in project.toml. Skills go in skills/. The look is ui/.
Policy stays in crates/agent-core. Do not copy it into the UI.
EOF
