#!/usr/bin/env bash
# Dev harness: rebuild Ward from src/ and run it against the current project.
#
# Unlike `install.sh`, this rebuilds every time and runs the debug binary, so it
# is what you want while working on Ward itself. The build details (why we do not
# simply `sage run`) live in scripts/build.sh.
#
# Ward runs in the directory you invoke this from, not in the Ward repo:
#
#   cd ~/some/project && ~/Projects/ward/run-ward.sh
#
# Usage: ./run-ward.sh            interactive
#        echo "quit" | ./run-ward.sh   smoke test
set -uo pipefail

REPO="$(cd "$(dirname "$0")" && pwd)"
HERE="$PWD"

# Load LLM credentials from a gitignored .env if present, so the key never has
# to be passed on the command line. Prefer the current project's .env, then
# Ward's own. Expected vars: SAGE_API_KEY, SAGE_LLM_URL, SAGE_MODEL. See
# .env.example.
for env_file in "$HERE/.env" "$REPO/.env"; do
    if [ -f "$env_file" ]; then
        set -a; . "$env_file"; set +a
        break
    fi
done

BIN="$("$REPO/scripts/build.sh")" || exit 1

# Run from the invoking directory: Ward's tools operate on its own cwd.
exec "$BIN"
