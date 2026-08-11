#!/usr/bin/env bash
# Dev runtime harness for Ward.
#
# `sage run` is blocked in this environment: the 2.2.0 CLI generates a hearth
# project requiring `sage-runtime = "^2.2.0"`, which is not published on
# crates.io (latest published is 2.1.0). This script regenerates the hearth Rust
# via `sage run` (which fails at cargo resolution AFTER codegen), then patches
# the generated Cargo.toml to use the local sage source from the homebrew tap,
# and builds/runs with cargo directly.
#
# Usage: echo "quit" | ./run-ward.sh
set -uo pipefail
cd "$(dirname "$0")"

# Load LLM credentials from a gitignored .env if present, so the key never has
# to be passed on the command line. Expected vars: SAGE_API_KEY, SAGE_LLM_URL,
# SAGE_MODEL. See .env.example.
if [ -f .env ]; then
  set -a; . ./.env; set +a
fi

SAGE_TAP="/opt/homebrew/Library/Taps/cargopete/homebrew-sage"
SAGE_SRC="$SAGE_TAP/crates"
HEARTH="hearth/sage_program"

# Prefer the locally-patched compiler (cross-module supervisor fix, Bug #6) if
# present, else fall back to the installed `sage`.
SAGE="$SAGE_TAP/target/debug/sage"
[ -x "$SAGE" ] || SAGE="sage"

# 1. Regenerate hearth Rust from .sg sources (ignore the expected cargo failure).
"$SAGE" run . >/dev/null 2>&1

# 2. Patch the freshly-generated Cargo.toml to the local runtime crates.
if ! grep -q 'patch.crates-io' "$HEARTH/Cargo.toml"; then
  cat >> "$HEARTH/Cargo.toml" <<PATCH

[patch.crates-io]
sage-runtime = { path = "$SAGE_SRC/sage-runtime" }
sage-persistence = { path = "$SAGE_SRC/sage-persistence" }
sage-mcp = { path = "$SAGE_SRC/sage-mcp" }
PATCH
fi

# 3. Build & run.
( cd "$HEARTH" && cargo run --quiet 2>&1 )
