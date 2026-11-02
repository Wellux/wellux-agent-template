#!/usr/bin/env bash
set -euo pipefail

name="${1:-}"
if [[ -z "$name" ]]; then
  echo "usage: scripts/new-skill.sh <kebab-name>" >&2
  exit 1
fi
if [[ ! "$name" =~ ^[a-z0-9]+(-[a-z0-9]+)*$ ]]; then
  echo "name must be kebab-case: $name" >&2
  exit 1
fi

root="$(cd "$(dirname "$0")/.." && pwd)"
dest="$root/skills/$name"
if [[ -e "$dest" ]]; then
  echo "already exists: skills/$name" >&2
  exit 1
fi

mkdir -p "$dest"
sed "s/skill-template/$name/g" "$root/skills/_template/SKILL.md" > "$dest/SKILL.md"
echo "created skills/$name/SKILL.md"
