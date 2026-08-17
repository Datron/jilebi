#!/usr/bin/env bash
set -euo pipefail

# Jilebi Installation Script
# Supports: Linux (x86_64), macOS (Intel & Apple Silicon)

JILEBI_BASE_URL="https://mcp.jilebi.ai/api/download/bin"
INSTALL_DIR="${JILEBI_HOME:-$HOME/.jilebi}"
BIN_DIR="$INSTALL_DIR/bin"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

error() {
    echo -e "${RED}[ERROR]${NC} $1"
    exit 1
}

# Detect OS and Architecture
detect_platform() {
    local os arch target

    os="$(uname -s)"
    arch="$(uname -m)"

    case "$os" in
        Linux)
            case "$arch" in
                x86_64)
                    target="jilebi-x86_64-unknown-linux-gnu"
                    ;;
                *)
                    error "Unsupported Linux architecture: $arch. Only x86_64 is supported."
                    ;;
            esac
            ;;
        Darwin)
            case "$arch" in
                x86_64)
                    target="jilebi-x86_64-apple-darwin"
                    ;;
                arm64)
                    target="jilebi-aarch64-apple-darwin"
                    ;;
                *)
                    error "Unsupported macOS architecture: $arch"
                    ;;
            esac
            ;;
        *)
            error "Unsupported operating system: $os"
            ;;
    esac

    echo "$target"
}

# Download and extract
download_and_install() {
    local target="$1"
    local download_url="$JILEBI_BASE_URL/$target.zip"
    local tmp_dir

    tmp_dir="$(mktemp -d)"
    trap 'rm -rf "$tmp_dir"' EXIT

    info "Downloading Jilebi from $download_url..."

    if command -v curl &> /dev/null; then
        curl -fsSL "$download_url" -o "$tmp_dir/jilebi.zip"
    elif command -v wget &> /dev/null; then
        wget -q "$download_url" -O "$tmp_dir/jilebi.zip"
    else
        error "Neither curl nor wget found. Please install one of them."
    fi

    info "Extracting..."

    if command -v unzip &> /dev/null; then
        unzip -q "$tmp_dir/jilebi.zip" -d "$tmp_dir"
    else
        error "unzip command not found. Please install unzip."
    fi

    # Create installation directory
    mkdir -p "$BIN_DIR"

    # Find and move the binary
    local binary_path
    binary_path="$(find "$tmp_dir" -name "jilebi" -type f | head -n 1)"

    if [[ -z "$binary_path" ]]; then
        error "Could not find jilebi binary in the downloaded archive."
    fi

    mv "$binary_path" "$BIN_DIR/jilebi"
    chmod +x "$BIN_DIR/jilebi"

    success "Jilebi installed to $BIN_DIR/jilebi"
}

# Setup PATH
setup_path() {
    local shell_profile=""
    local path_export="export PATH=\"\$PATH:$BIN_DIR\""

    # Determine shell profile
    case "${SHELL:-}" in
        */zsh)
            shell_profile="$HOME/.zshrc"
            ;;
        */bash)
            if [[ -f "$HOME/.bash_profile" ]]; then
                shell_profile="$HOME/.bash_profile"
            else
                shell_profile="$HOME/.bashrc"
            fi
            ;;
        */fish)
            shell_profile="$HOME/.config/fish/config.fish"
            path_export="set -gx PATH \$PATH $BIN_DIR"
            ;;
        *)
            shell_profile="$HOME/.profile"
            ;;
    esac

    # Check if already in PATH
    if [[ ":$PATH:" == *":$BIN_DIR:"* ]]; then
        info "PATH already contains $BIN_DIR"
        return
    fi

    # Check if already added to profile
    if [[ -f "$shell_profile" ]] && grep -q "$BIN_DIR" "$shell_profile" 2>/dev/null; then
        info "PATH export already in $shell_profile"
    else
        info "Adding $BIN_DIR to PATH in $shell_profile..."
        echo "" >> "$shell_profile"
        echo "# Jilebi" >> "$shell_profile"
        echo "$path_export" >> "$shell_profile"
        success "Added to $shell_profile"
    fi

    warn "Restart your shell or run: source $shell_profile"
}

# macOS specific setup
macos_setup() {
    if [[ "$(uname -s)" == "Darwin" ]]; then
        info "Removing quarantine attribute (macOS)..."
        xattr -d com.apple.quarantine "$BIN_DIR/jilebi" 2>/dev/null || true

        # Check for xz dependency
        if ! command -v xz &> /dev/null; then
            warn "xz is not installed. Some features may require it."
            if command -v brew &> /dev/null; then
                info "Install with: brew install xz"
            fi
        fi
    fi
}

# Install recommended plugins
install_plugins() {
    info "Installing recommended plugins..."
    "$BIN_DIR/jilebi" plugins add memory || warn "Failed to install memory plugin"
    "$BIN_DIR/jilebi" plugins add sequential-thinking || warn "Failed to install sequential-thinking plugin"
    success "Recommended plugins installed!"
}

# Main
main() {
    echo ""
    echo "       ██╗██╗██╗     ███████╗██████╗ ██╗"
    echo "       ██║██║██║     ██╔════╝██╔══██╗██║"
    echo "       ██║██║██║     █████╗  ██████╔╝██║"
    echo "  ██   ██║██║██║     ██╔══╝  ██╔══██╗██║"
    echo "  ╚█████╔╝██║███████╗███████╗██████╔╝██║"
    echo "   ╚════╝ ╚═╝╚══════╝╚══════╝╚═════╝ ╚═╝"
    echo "        Installation Script"
    echo ""

    local target
    target="$(detect_platform)"
    info "Detected platform: $target"

    download_and_install "$target"

    if [[ "$(uname -s)" == "Darwin" ]]; then
        macos_setup
    fi

    setup_path

    echo ""
    read -p "Install recommended plugins (memory, sequential-thinking)? [y/N] " -n 1 -r
    echo ""
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        # Add to PATH for current session
        export PATH="$PATH:$BIN_DIR"
        install_plugins
    fi

    echo ""
    success "Jilebi installation complete!"
    echo ""
    info "To get started, run: jilebi --help"
    info "To start the MCP server: jilebi stdio"
    echo ""
}

main "$@"
