#!/usr/bin/env bash
# Build Ward and install it as `ward` on your PATH.
#
#   ./install.sh                     -> ~/.local/bin/ward
#   PREFIX=/usr/local ./install.sh   -> /usr/local/bin/ward
#
# Once `sage-runtime` 2.2.0 reaches crates.io this can be replaced by a Homebrew
# formula alongside `sage`; until then Ward has to be built against a local Sage
# checkout. See scripts/build.sh.
set -euo pipefail

REPO="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PREFIX="${PREFIX:-$HOME/.local}"
DEST="$PREFIX/bin"

BIN="$("$REPO/scripts/build.sh" --release)"

mkdir -p "$DEST"
install -m 755 "$BIN" "$DEST/ward"

echo
echo "  installed: $DEST/ward"

case ":$PATH:" in
    *":$DEST:"*)
        echo "  run it from any project:  cd ~/some/project && ward"
        ;;
    *)
        echo
        echo "  $DEST is not on your PATH. Add it:"
        echo "    echo 'export PATH=\"$DEST:\$PATH\"' >> ~/.zshrc"
        ;;
esac
echo
