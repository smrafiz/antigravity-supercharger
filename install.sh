#!/usr/bin/env bash
set -euo pipefail

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$DIR"

echo "=== Building Antigravity Supercharger (Rust) ==="
RUSTC_WRAPPER="" cargo build --release --offline || RUSTC_WRAPPER="" cargo build --release

mkdir -p bin
cp target/release/antigravity-supercharger bin/supercharger
chmod +x bin/supercharger

echo "=== Testing Supercharger Binary ==="
./bin/supercharger version

echo ""
echo "=== Activating Globally in Antigravity CLI ==="
GLOBAL_PLUGINS_DIR="${HOME}/.gemini/config/plugins"
mkdir -p "$GLOBAL_PLUGINS_DIR"

ln -sfn "$DIR" "$GLOBAL_PLUGINS_DIR/supercharger"

echo "Supercharger linked to $GLOBAL_PLUGINS_DIR/supercharger"

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
