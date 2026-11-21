#!/usr/bin/env bash
set -euo pipefail

GLOBAL_PLUGINS_DIR="${HOME}/.gemini/config/plugins"
TARGET="$GLOBAL_PLUGINS_DIR/supercharger"

if [ -L "$TARGET" ] || [ -e "$TARGET" ]; then
  rm -rf "$TARGET"
  echo "Removed global Supercharger plugin from $TARGET"
else
  echo "Supercharger is not installed globally in $TARGET"
fi

if command -v agy >/dev/null 2>&1; then
  echo ""
  echo "=== Current Antigravity Plugin Status ==="
  agy plugin list || true
fi

echo "Antigravity Supercharger has been uninstalled."
