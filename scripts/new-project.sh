#!/usr/bin/env bash
# Name this checkout once, and optionally point it at the GitHub repo it should live in.
# Does not push. Does not invent keys.
set -euo pipefail

name="${1:-}"
repo_raw="${2:-}"

usage() {
  echo "usage: scripts/new-project.sh <kebab-name> [https://github.com/owner/name]" >&2
  echo "       or pass only the repo URL; the name is taken from it" >&2
  exit 1
}

# Accept only a GitHub repo. Prints the canonical https URL.
canon_repo() {
  local raw="$1"
  raw="${raw%.git}"
  raw="${raw%/}"
  local owner="" repo=""
  if [[ "$raw" =~ ^https://github\.com/([A-Za-z0-9_.-]+)/([A-Za-z0-9_.-]+)$ ]]; then
    owner="${BASH_REMATCH[1]}"
    repo="${BASH_REMATCH[2]}"
  elif [[ "$raw" =~ ^git@github\.com:([A-Za-z0-9_.-]+)/([A-Za-z0-9_.-]+)$ ]]; then
    owner="${BASH_REMATCH[1]}"
    repo="${BASH_REMATCH[2]}"
  else
    echo "repo must be https://github.com/owner/name — got: $raw" >&2
    exit 1
  fi
  if [[ "$owner" == .* || "$repo" == .* || "$owner" == *".."* || "$repo" == *".."* ]]; then
    echo "repo path is not a project name" >&2
    exit 1
  fi
  printf 'https://github.com/%s/%s\n' "$owner" "$repo"
}

slug_from() {
  local raw="$1"
  raw="$(printf '%s' "$raw" | tr '[:upper:]' '[:lower:]' | tr '._' '--')"
  raw="$(printf '%s' "$raw" | sed -E 's/-+/-/g; s/^-+//; s/-+$//')"
  printf '%s' "$raw"
}

canon=""
if [[ -n "$repo_raw" ]]; then
  canon="$(canon_repo "$repo_raw")"
  if [[ "$canon" == "https://github.com/Wellux/wellux-agent-template" ]]; then
    echo "that URL is the template. Pass the empty repository you want the project in." >&2
    exit 1
  fi
fi

if [[ -z "$name" && -n "$canon" ]]; then
  name="$(slug_from "${canon##*/}")"
fi
if [[ -z "$name" ]]; then
  usage
fi
if [[ ! "$name" =~ ^[a-z0-9]+(-[a-z0-9]+)*$ ]]; then
  echo "name must be kebab-case: $name" >&2
  echo "pass NAME= explicitly if the GitHub name cannot be slugified." >&2
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

{
  echo "# This checkout's name. Set once by \`make project\`."
  echo "name = \"$name\""
  echo "profile = \"personal\""
  if [[ -n "$canon" ]]; then
    echo "repo = \"$canon\""
  fi
} > "$manifest"

if [[ ! -f .env ]]; then
  cp .env.example .env
fi
tmp="$(mktemp)"
if grep -q '^WELLUX_AGENT_NAME=' .env; then
  sed "s/^WELLUX_AGENT_NAME=.*/WELLUX_AGENT_NAME=$name/" .env > "$tmp"
else
  cat .env > "$tmp"
  printf '\nWELLUX_AGENT_NAME=%s\n' "$name" >> "$tmp"
fi
mv "$tmp" .env

if [[ ! -e "skills/$name" ]]; then
  "$root/scripts/new-skill.sh" "$name"
fi

remote_note="no git repository, so origin was not changed"
if git rev-parse --is-inside-work-tree >/dev/null 2>&1 && [[ -n "$canon" ]]; then
  current_url="$(git remote get-url origin 2>/dev/null || true)"
  case "$current_url" in
    *Wellux/wellux-agent-template*)
      git remote rename origin upstream
      git remote add origin "$canon"
      remote_note="origin is $canon — template kept as upstream. Not pushed."
      ;;
    "")
      git remote add origin "$canon"
      remote_note="origin is $canon. Not pushed."
      ;;
    "$canon"|"$canon.git")
      remote_note="origin already points at the destination. Not pushed."
      ;;
    *)
      if [[ "${FORCE:-}" == "1" ]]; then
        git remote set-url origin "$canon"
        remote_note="origin replaced with $canon. Not pushed."
      else
        remote_note="origin stays $current_url. Not replaced. Set FORCE=1 to override."
      fi
      ;;
  esac
fi

cat <<EOF

Project '$name' is ready.
${canon:+Repo:    $canon}
Remote:  $remote_note

  1. make test
  2. make doctor
  3. make personal     # or make business / make desktop
  4. make shell

Do not push unless the person asked. Do not commit .env.
Skills go in skills/. The look is ui/. Policy stays in crates/agent-core.
EOF
