#!/bin/bash
# Void IRC Client — Install/Upgrade script
set -e

BINARY="target/release/void"
INSTALL_DIR="${HOME}/.local/bin"
VOID_DIR="${HOME}/.void"
REPO_DIR="$(cd "$(dirname "$0")" && pwd)"

echo "=== Void IRC Client Installer ==="
echo ""

# Check if git repo
if [ -d "$REPO_DIR/.git" ]; then
    echo "[1/5] Pulling latest changes..."
    cd "$REPO_DIR"
    git pull origin main 2>/dev/null || echo "  (git pull skipped — not a git repo or no remote)"
else
    echo "[1/5] Not a git repo — building from source..."
fi

# Build
echo "[2/5] Building release..."
cd "$REPO_DIR"
cargo build --release 2>&1 | tail -3

# Install binary
echo "[3/5] Installing binary to $INSTALL_DIR/void..."
mkdir -p "$INSTALL_DIR"
cp "$BINARY" "$INSTALL_DIR/void"
chmod +x "$INSTALL_DIR/void"

# Install modules and config
echo "[4/5] Installing modules to $VOID_DIR/..."
mkdir -p "$VOID_DIR/modules"
if [ -d "$REPO_DIR/modules/lice" ]; then
    cp -r "$REPO_DIR/modules/lice" "$VOID_DIR/modules/"
    echo "  Modules: $VOID_DIR/modules/lice/"
fi
if [ -f "$REPO_DIR/config.lua" ] && [ ! -f "$VOID_DIR/config.lua" ]; then
    cp "$REPO_DIR/config.lua" "$VOID_DIR/config.lua"
    echo "  Config:  $VOID_DIR/config.lua"
fi

# Verify
echo "[5/5] Verifying..."
if command -v void &>/dev/null; then
    echo ""
    echo "=== Success ==="
    echo "  Binary:  $INSTALL_DIR/void"
    echo "  Modules: $VOID_DIR/modules/lice/"
    echo "  Config:  $VOID_DIR/config.lua"
    echo ""
    echo "  Run: void -c irc.example.com -n mynick"
else
    echo ""
    echo "=== Warning ==="
    echo "  $INSTALL_DIR is not in PATH"
    echo "  Add to ~/.bashrc: export PATH=\"\$HOME/.local/bin:\$PATH\""
fi
