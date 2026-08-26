#!/bin/bash
# Void IRC Client — Install script
set -e

BINARY="target/release/void"
INSTALL_DIR="${HOME}/.local/bin"

echo "=== Void IRC Client Installer ==="

# Build release
echo "[1/3] Building release..."
cargo build --release 2>&1 | tail -3

# Create install dir
mkdir -p "$INSTALL_DIR"

# Copy binary
echo "[2/3] Installing to $INSTALL_DIR/void..."
cp "$BINARY" "$INSTALL_DIR/void"
chmod +x "$INSTALL_DIR/void"

# Create config dir
mkdir -p ~/.void

# Verify
echo "[3/3] Verifying..."
if command -v void &>/dev/null; then
    echo "✓ void installed at $(which void)"
    echo "✓ Run: void -c irc.example.com -n mynick"
else
    echo "⚠ $INSTALL_DIR is not in PATH"
    echo "  Add to ~/.bashrc: export PATH=\"\$HOME/.local/bin:\$PATH\""
fi

echo ""
echo "=== Quick Start ==="
echo "  void -c irc.spadhausen.com -n mynick -j '#mychannel'"
echo "  void --help"
echo ""
echo "=== Done ==="
