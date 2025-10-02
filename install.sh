#!/bin/bash
# gex Installation Script for Linux/macOS
# Usage: curl -fsSL https://raw.githubusercontent.com/yourusername/gex/main/install.sh | bash

set -e

echo "Installing gex - Git Profile Switcher"
echo ""

# Detect OS and architecture
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case "$ARCH" in
    x86_64|amd64)
        ARCH="x86_64"
        ;;
    aarch64|arm64)
        ARCH="aarch64"
        ;;
    *)
        echo "Error: Unsupported architecture: $ARCH"
        exit 1
        ;;
esac

case "$OS" in
    linux)
        TARGET="${ARCH}-unknown-linux-gnu"
        ;;
    darwin)
        TARGET="${ARCH}-apple-darwin"
        ;;
    *)
        echo "Error: Unsupported OS: $OS"
        exit 1
        ;;
esac

# Get latest release info
REPO="FriezaForce/gex"
API_URL="https://api.github.com/repos/$REPO/releases/latest"

echo "Fetching latest release..."

RELEASE_INFO=$(curl -fsSL "$API_URL")
VERSION=$(echo "$RELEASE_INFO" | grep '"tag_name"' | sed -E 's/.*"([^"]+)".*/\1/')

if [ -z "$VERSION" ]; then
    echo "Error: Failed to fetch release information"
    exit 1
fi

echo "Latest version: $VERSION"

# Find the appropriate binary
ASSET_NAME=$(echo "$RELEASE_INFO" | grep '"name"' | grep "$TARGET" | sed -E 's/.*"([^"]+)".*/\1/' | head -n 1)

if [ -z "$ASSET_NAME" ]; then
    # Try alternative naming
    ASSET_NAME=$(echo "$RELEASE_INFO" | grep '"name"' | grep "$OS" | grep -v "windows" | sed -E 's/.*"([^"]+)".*/\1/' | head -n 1)
fi

if [ -z "$ASSET_NAME" ]; then
    echo "Error: No binary found for $OS-$ARCH"
    exit 1
fi

DOWNLOAD_URL="https://github.com/$REPO/releases/download/$VERSION/$ASSET_NAME"

echo "Downloading $ASSET_NAME..."

# Create temp directory
TEMP_DIR=$(mktemp -d)
trap "rm -rf $TEMP_DIR" EXIT

cd "$TEMP_DIR"

# Download the binary
if ! curl -fsSL -o "$ASSET_NAME" "$DOWNLOAD_URL"; then
    echo "Error: Failed to download binary"
    exit 1
fi

echo "Downloaded successfully"

# Extract the archive
echo "Extracting..."
if [[ "$ASSET_NAME" == *.tar.gz ]]; then
    tar -xzf "$ASSET_NAME"
elif [[ "$ASSET_NAME" == *.zip ]]; then
    unzip -q "$ASSET_NAME"
else
    echo "Error: Unknown archive format"
    exit 1
fi

# Find the gex binary
GEX_BIN=$(find . -name "gex" -type f | head -n 1)

if [ -z "$GEX_BIN" ]; then
    echo "Error: gex binary not found in archive"
    exit 1
fi

# Make it executable
chmod +x "$GEX_BIN"

# Determine installation directory
if [ -w "/usr/local/bin" ]; then
    INSTALL_DIR="/usr/local/bin"
    SUDO=""
elif command -v sudo &> /dev/null; then
    INSTALL_DIR="/usr/local/bin"
    SUDO="sudo"
else
    INSTALL_DIR="$HOME/.local/bin"
    SUDO=""
    mkdir -p "$INSTALL_DIR"
fi

# Install the binary
echo "Installing to $INSTALL_DIR..."
$SUDO cp "$GEX_BIN" "$INSTALL_DIR/gex"

# Add to PATH if needed
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]] && [ "$INSTALL_DIR" = "$HOME/.local/bin" ]; then
    echo ""
    echo "Adding $INSTALL_DIR to PATH..."
    
    # Detect shell and add to appropriate rc file
    if [ -n "$BASH_VERSION" ]; then
        RC_FILE="$HOME/.bashrc"
    elif [ -n "$ZSH_VERSION" ]; then
        RC_FILE="$HOME/.zshrc"
    else
        RC_FILE="$HOME/.profile"
    fi
    
    if ! grep -q "$INSTALL_DIR" "$RC_FILE" 2>/dev/null; then
        echo "export PATH=\"\$PATH:$INSTALL_DIR\"" >> "$RC_FILE"
        echo "Added to $RC_FILE (restart your shell or run: source $RC_FILE)"
    fi
fi

echo ""
echo "✓ gex installed successfully!"
echo ""
echo "Installation location: $INSTALL_DIR/gex"
echo ""
echo "Quick start:"
echo "  gex --help                    # Show help"
echo "  gex add <name> ...            # Add a profile"
echo "  gex list                      # List profiles"
echo "  gex switch <name> --global    # Switch profile"
echo ""

# Verify installation
if command -v gex &> /dev/null; then
    echo "Verification: $(gex --version)"
else
    echo "Note: You may need to restart your shell or run: source $RC_FILE"
fi
