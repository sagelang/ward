#!/usr/bin/env bash
# Build the Ward binary from the Sage sources in src/.
#
# Prints the path to the built binary on stdout; everything else goes to stderr,
# so callers can do `BIN=$(scripts/build.sh --release)`.
#
# Two things make this more than `sage build`:
#
#   1. `sage run`/`build` generates a hearth depending on `sage-runtime ^2.2.0`,
#      which is not published on crates.io yet (latest published is 2.1.0), so
#      cargo resolution fails after codegen. We generate the Rust with
#      `--emit-rust`, point the generated Cargo.toml at a local Sage checkout,
#      and drive cargo ourselves. Once 2.2.0 publishes, set SAGE_SRC="" and this
#      step drops out.
#   2. Ward needs the cross-module supervisor fix (docs/sage-notes.md, bug 7),
#      so a locally built compiler is preferred over the installed `sage`.
#
# Environment:
#   SAGE       path to the sage compiler        (default: local tap build, else `sage`)
#   SAGE_SRC   path to the sage `crates/` dir   (default: the Homebrew tap, if present)
#              set to "" to build against published crates.io versions
set -euo pipefail

REPO="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
HEARTH="$REPO/hearth/sage_program"

# A plain string rather than an array: macOS ships bash 3.2, where expanding an
# empty array under `set -u` is an error.
PROFILE_FLAG=""
PROFILE_DIR="debug"
if [ "${1:-}" = "--release" ]; then
    PROFILE_FLAG="--release"
    PROFILE_DIR="release"
fi

DEFAULT_TAP="/opt/homebrew/Library/Taps/cargopete/homebrew-sage"
: "${SAGE_SRC=$([ -d "$DEFAULT_TAP/crates" ] && echo "$DEFAULT_TAP/crates" || echo "")}"

if [ -z "${SAGE:-}" ]; then
    if [ -x "$DEFAULT_TAP/target/debug/sage" ]; then
        SAGE="$DEFAULT_TAP/target/debug/sage"
    else
        SAGE="sage"
    fi
fi
command -v "$SAGE" >/dev/null 2>&1 || [ -x "$SAGE" ] || {
    echo "ward: no sage compiler found (set SAGE=/path/to/sage)" >&2
    exit 1
}

# The trailing `.` matters: given a file (or nothing, which defaults to the
# grove.toml entry) the compiler builds in single-file mode and cannot resolve
# `use core::Ward` across modules. A directory puts it in project mode.
echo "ward: generating Rust with $SAGE" >&2
( cd "$REPO" && "$SAGE" build . --emit-rust >/dev/null )

# Point the generated manifest at a local Sage checkout, if we have one. The
# generated Cargo.toml is rewritten by every codegen run, so this reapplies.
if [ -n "$SAGE_SRC" ] && ! grep -q 'patch.crates-io' "$HEARTH/Cargo.toml"; then
    echo "ward: patching hearth to build against $SAGE_SRC" >&2
    cat >> "$HEARTH/Cargo.toml" <<PATCH

[patch.crates-io]
sage-runtime = { path = "$SAGE_SRC/sage-runtime" }
sage-persistence = { path = "$SAGE_SRC/sage-persistence" }
sage-mcp = { path = "$SAGE_SRC/sage-mcp" }
PATCH
fi

echo "ward: compiling ($PROFILE_DIR)" >&2
( cd "$HEARTH" && cargo build --quiet $PROFILE_FLAG >&2 )

echo "$HEARTH/target/$PROFILE_DIR/sage_program"
