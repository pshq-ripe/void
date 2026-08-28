#!/bin/bash
# Void IRC Client — Install/Upgrade script
set -e

BINARY="target/release/void"
INSTALL_DIR="${HOME}/.local/bin"
REPO_DIR="$(cd "$(dirname "$0")" && pwd)"

echo "=== Void IRC Client Installer ==="
echo ""

# Check if git repo
if [ -d "$REPO_DIR/.git" ]; then
    echo "[1/4] Pulling latest changes..."
    cd "$REPO_DIR"
    git pull origin main 2>/dev/null || echo "  (git pull skipped — not a git repo or no remote)"
else
    echo "[1/4] Not a git repo — building from source..."
fi

# Build
echo "[2/4] Building release..."
cd "$REPO_DIR"
cargo build --release 2>&1 | tail -3

# Install
echo "[3/4] Installing to $INSTALL_DIR/void..."
mkdir -p "$INSTALL_DIR"
cp "$BINARY" "$INSTALL_DIR/void"
chmod +x "$INSTALL_DIR/void"

# Verify
echo "[4/4] Verifying..."
if command -v void &>/dev/null; then
    VERSION=$(void --version 2>/dev/null || echo "unknown")
    echo ""
    echo "=== Success ==="
    echo "  Binary:  $INSTALL_DIR/void"
    echo "  Version: $VERSION"
    echo ""
    echo "  Run: void -c irc.example.com -n mynick"
else
    echo ""
    echo "=== Warning ==="
    echo "  $INSTALL_DIR is not in PATH"
    echo "  Add to ~/.bashrc: export PATH=\"\$HOME/.local/bin:\$PATH\""
fi
