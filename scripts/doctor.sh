#!/usr/bin/env bash
set -euo pipefail

ok() { printf "  ok   %s\n" "$1"; }
miss() { printf "  --   %s\n" "$1"; }

echo "Wellux doctor"
echo

if command -v rustc >/dev/null 2>&1; then
  ok "rustc $(rustc --version | awk '{print $2}')"
else
  miss "rustc 1.80+  (https://rustup.rs)"
fi

if command -v cargo >/dev/null 2>&1; then
  ok "cargo"
else
  miss "cargo"
fi

if command -v engram >/dev/null 2>&1; then
  ok "engram (second brain)"
else
  miss "engram  →  brew install gentleman-programming/tap/engram"
fi

if python3 -c "import quicksand" >/dev/null 2>&1; then
  ok "quicksand python module"
else
  miss "quicksand  →  pip install 'quick-sandbox[qemu,ubuntu]'"
fi

if [[ -f .env ]]; then
  ok ".env"
else
  miss ".env  →  cp .env.example .env"
fi

if git rev-parse --is-inside-work-tree >/dev/null 2>&1 && git ls-files --error-unmatch .env >/dev/null 2>&1; then
  miss ".env is tracked by git — remove it from the index"
else
  ok ".env is not tracked"
fi

echo
echo "Profiles:  make personal | make business | make desktop"
echo "Shell:     make shell     →  http://127.0.0.1:8787"
echo "Access:    docs/ACCESS.md  (Supabase, PayPal, ChatGPT, Realtime)"
echo "New skill: make skill NAME=my-skill"
