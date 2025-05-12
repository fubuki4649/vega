#!/bin/sh

set -eu

have() { command -v "$1" >/dev/null 2>&1; }

append() {
    [ -n "${out-}" ] && out="$out, $1" || out="$1"
}

out=""

# ---------- Homebrew ----------
if have brew; then
    f=$(brew list --formula 2>/dev/null | wc -l | tr -d '[:space:]')
    c=$(brew list --cask    2>/dev/null | wc -l | tr -d '[:space:]')
    total=$((f + c))
    [ "$total" -gt 0 ] && append "$total (brew)"
fi

# ---------- MacPorts ----------
if have port; then
    p=$(port -qv installed 2>/dev/null | awk '/Active/ {print $2}' | wc -l | tr -d '[:space:]')
    [ "$p" -gt 0 ] && append "$p (port)"
fi

# ---------- pkgin ----------
if have pkgin; then
    pi=$(pkgin list 2>/dev/null | wc -l | tr -d '[:space:]')
    [ "$pi" -gt 0 ] && append "$pi (pkgin)"
fi

# ---------- dpkg ----------
if have dpkg-query; then
    dp=$(dpkg-query -f . -W 2>/dev/null | wc -c | tr -d '[:space:]')
    [ "$dp" -gt 0 ] && append "$dp (dpkg)"
fi

# ---------- Nix ----------
if have nix-store; then
    [ -e /run/current-system/sw ] && {
        ns=$(nix-store -qR /run/current-system/sw 2>/dev/null | wc -l | tr -d '[:space:]')
        [ "$ns" -gt 0 ] && append "$ns (nix-system)"
    }
    [ -e "$HOME/.nix-profile" ] && {
        nu=$(nix-store -qR "$HOME/.nix-profile" 2>/dev/null | wc -l | tr -d '[:space:]')
        [ "$nu" -gt 0 ] && append "$nu (nix-user)"
    }
fi

# ---------- Output ----------
[ -n "$out" ] && printf '%s\n' "$out" || echo "None"
