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
echo "Antigravity Supercharger built successfully!"
echo "Binary located at: $DIR/bin/supercharger"
echo ""
echo "To use this plugin with Antigravity CLI:"
echo "  1. Project-level: Link or copy this directory into .agents/plugins/supercharger"
echo "  2. Global-level:  Link or copy this directory into ~/.gemini/config/plugins/supercharger"
