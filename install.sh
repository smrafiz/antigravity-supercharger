#!/usr/bin/env bash
set -euo pipefail

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$DIR"

echo "=== Building Antigravity Supercharger (Rust) ==="
RUSTC_WRAPPER="" cargo build --release --offline || RUSTC_WRAPPER="" cargo build --release

mkdir -p bin
cp target/release/antigravity-supercharger bin/supercharger.tmp
mv -f bin/supercharger.tmp bin/supercharger
chmod +x bin/supercharger

echo "=== Testing Supercharger Binary ==="
./bin/supercharger version

echo ""
echo "=== Activating Globally in Antigravity CLI ==="
GLOBAL_PLUGINS_DIR="${HOME}/.gemini/config/plugins"
mkdir -p "$GLOBAL_PLUGINS_DIR"

ln -sfn "$DIR" "$GLOBAL_PLUGINS_DIR/supercharger"

echo "Supercharger linked to $GLOBAL_PLUGINS_DIR/supercharger"

# Configure Antigravity CLI statusline in settings.json
SETTINGS_FILE="${HOME}/.gemini/antigravity-cli/settings.json"
if [ -f "$SETTINGS_FILE" ]; then
  python3 -c "
import json, os
path = os.path.expanduser('$SETTINGS_FILE')
try:
    with open(path, 'r') as f:
        data = json.load(f)
    data['statusLine'] = {
        'command': os.path.expanduser('$GLOBAL_PLUGINS_DIR/supercharger/bin/supercharger statusline')
    }
    with open(path, 'w') as f:
        json.dump(data, f, indent=2)
    print('Configured Supercharger statusline in ~/.gemini/antigravity-cli/settings.json')
except Exception as e:
    print('Notice: Could not auto-configure statusline in settings.json:', e)
"
fi

if command -v agy >/dev/null 2>&1; then
  echo ""
  echo "=== Antigravity Plugin Status ==="
  agy plugin list || true
fi

echo ""
echo "Antigravity Supercharger is now AUTOMATICALLY ACTIVATED in all projects!"
echo "To disable or uninstall:"
echo "  agy plugin disable supercharger   # disable without removing"
echo "  bash uninstall.sh                 # remove global activation"
