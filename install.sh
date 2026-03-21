#!/usr/bin/env bash
set -e

REPO="gonardfreeman/lcli"
BINARY="lcli"

OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
	Linux) PLATFORM="Linux" ;;
	Darwin) PLATFORM="macos" ;;
	*) echo "Unsupported OS"; exit 1 ;;
esac

case "$ARCH" in
	x86_64) ARCH="x86_64" ;;
	arm64|aarch64) ARCH="arm64" ;;
	*) echo "Unsupported architecture"; exit 1 ;;
esac

FILENAME="$BINARY-$PLATFORM-$ARCH"

# Get latest version
VERSION=$(curl -s https://api.github.com/repos/$REPO/releases/latest | grep tag_name | cut -d '"' -f 4)

URL="https://github.com/$REPO/releases/download/$VERSION/$FILENAME"

INSTALL_DIR="$HOME/.local/bin"
mkdir -p "$INSTALL_DIR"

echo "Downloading $URL"
curl -L "$URL" -o "$INSTALL_DIR/$BINARY"

chmod +x "$INSTALL_DIR/$BINARY"

echo "Installed to $INSTALL_DIR/$BINARY"

if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
	echo "Add this to your ~/.zshrc:"
	echo "export PATH=\"$HOME/.local/bin/:\$PATH\""
fi
