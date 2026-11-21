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

SETTINGS_FILE="${HOME}/.gemini/antigravity-cli/settings.json"
if [ -f "$SETTINGS_FILE" ]; then
  python3 -c "
import json, os
path = os.path.expanduser('$SETTINGS_FILE')
try:
    with open(path, 'r') as f:
        data = json.load(f)
    if 'statusLine' in data:
        del data['statusLine']
        with open(path, 'w') as f:
            json.dump(data, f, indent=2)
        print('Reverted statusLine in ~/.gemini/antigravity-cli/settings.json')
except Exception:
    pass
"
fi

if command -v agy >/dev/null 2>&1; then
  echo ""
  echo "=== Current Antigravity Plugin Status ==="
  agy plugin list || true
fi

echo "Antigravity Supercharger has been uninstalled."
