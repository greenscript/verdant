#!/bin/bash

echo "🌿 Installing Verdant..."
echo

# Build the optimized binary
echo "📦 Building optimized binary..."
cargo build --release

# Check if build was successful
if [ $? -eq 0 ]; then
    echo "✅ Build successful!"
else
    echo "❌ Build failed!"
    exit 1
fi

# Create a symlink in a directory that's in PATH
INSTALL_DIR="$HOME/.local/bin"
mkdir -p "$INSTALL_DIR"

# Copy binary to install directory
cp target/release/verdant "$INSTALL_DIR/verdant"

# Make it executable
chmod +x "$INSTALL_DIR/verdant"

echo "✅ Verdant installed to $INSTALL_DIR/verdant"
echo

# Check if ~/.local/bin is in PATH
if [[ ":$PATH:" == *":$HOME/.local/bin:"* ]]; then
    echo "🎉 Ready to use! Try: verdant --help"
else
    echo "⚠️  Add this to your ~/.zshrc or ~/.bashrc:"
    echo "export PATH=\"\$HOME/.local/bin:\$PATH\""
    echo
    echo "Then restart your terminal and try: verdant --help"
fi