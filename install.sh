#!/bin/sh
# Install llm-cli from GitHub releases to ~/.local/bin/
# Usage: curl -fsSL https://raw.githubusercontent.com/Glaicer/llm-cli/main/install.sh | sh

set -e

BINARY_NAME="llm"
INSTALL_DIR="${HOME}/.local/bin"
GITHUB_USER="Glaicer"
GITHUB_REPO="llm-cli"
API_URL="https://api.github.com/repos/${GITHUB_USER}/${GITHUB_REPO}/releases/latest"

# Helper: fetch a URL to stdout, falling back from curl to wget
download() {
    if command -v curl >/dev/null 2>&1; then
        curl -fsSL "$1"
    elif command -v wget >/dev/null 2>&1; then
        wget -qO- "$1"
    else
        echo "error: curl or wget is required but neither was found." >&2
        exit 1
    fi
}

# Helper: download a URL to a file, falling back from curl to wget
download_to() {
    url="$1"
    dest="$2"
    if command -v curl >/dev/null 2>&1; then
        curl -fsSL "${url}" -o "${dest}"
    elif command -v wget >/dev/null 2>&1; then
        wget -q "${url}" -O "${dest}"
    else
        echo "error: curl or wget is required but neither was found." >&2
        exit 1
    fi
}

echo "==> Detecting latest release..."

# Fetch latest release info
RELEASE_JSON=$(download "${API_URL}")
TAG_NAME=$(echo "${RELEASE_JSON}" | grep '"tag_name":' | head -n 1 | sed 's/.*"tag_name": "\([^"]*\)".*/\1/')

if [ -z "${TAG_NAME}" ]; then
    echo "error: failed to detect latest release tag." >&2
    exit 1
fi

echo "==> Found latest release: ${TAG_NAME}"

# Determine download URL
# The release asset is just named 'llm' (x86_64-linux)
DOWNLOAD_URL="https://github.com/${GITHUB_USER}/${GITHUB_REPO}/releases/download/${TAG_NAME}/${BINARY_NAME}"

# Create install directory if it doesn't exist
if [ ! -d "${INSTALL_DIR}" ]; then
    echo "==> Creating ${INSTALL_DIR}..."
    mkdir -p "${INSTALL_DIR}"
fi

# Determine if we need sudo (only if installing outside home, but here it's ~/.local/bin)
TARGET="${INSTALL_DIR}/${BINARY_NAME}"

echo "==> Downloading ${BINARY_NAME} (${TAG_NAME})..."
download_to "${DOWNLOAD_URL}" "${TARGET}"

echo "==> Making ${BINARY_NAME} executable..."
chmod +x "${TARGET}"

echo ""
echo "==> ${BINARY_NAME} installed to ${TARGET}"
echo ""

# Check if ~/.local/bin is in PATH
if ! echo "${PATH}" | grep -q "${INSTALL_DIR}"; then
    echo "warning: ${INSTALL_DIR} is not in your PATH."
    echo "         Add it by adding this line to your ~/.bashrc or ~/.zshrc:"
    echo ""
    echo '         export PATH="'"${INSTALL_DIR}"':$PATH"'
    echo ""
    echo "         Then restart your terminal or run:"
    echo "         source ~/.bashrc   # (or source ~/.zshrc)"
    echo ""
fi

echo "Run '${BINARY_NAME} --help' to get started."
