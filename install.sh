#!/usr/bin/env bash

set -e

echo "🚀 Installing Asmodeus Compiler (asmod)..."

# release version
echo "📦 Building optimized binary..."
cargo build --release

# binary location
BINARY_PATH="target/release/asmodeus"
INSTALL_DIR="$HOME/.local/bin"

# install directory
mkdir -p "$INSTALL_DIR"

# copy binary
echo "📥 Installing to $INSTALL_DIR/asmod..."
cp "$BINARY_PATH" "$INSTALL_DIR/asmod"

# executable
chmod +x "$INSTALL_DIR/asmod"

echo "✅ Asmodeus Compiler installed successfully!"
echo ""
echo "🎯 Usage:"
echo "  asmod run program.asmod           # Run assembly program"
echo "  asmod run --debug program.asmod   # Run with debug output"
echo "  asmod --help                      # Show all options"
echo ""
echo "📝 Make sure $INSTALL_DIR is in your PATH"
echo "   Add this to your ~/.bashrc or ~/.zshrc:"
echo "   export PATH=\"\$HOME/.local/bin:\$PATH\""
