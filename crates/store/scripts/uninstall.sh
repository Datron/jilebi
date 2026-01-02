#!/usr/bin/env bash
set -euo pipefail

# Jilebi Uninstallation Script
# Supports: Linux and macOS

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

# Get data directory based on OS
get_data_dir() {
    local os
    os="$(uname -s)"

    case "$os" in
        Linux)
            echo "$HOME/.local/share/jilebi-server"
            ;;
        Darwin)
            echo "$HOME/Library/Application Support/ai.jilebi.jilebi-server"
            ;;
        *)
            echo "$HOME/.jilebi-server"
            ;;
    esac
}

# Get config directory based on OS
get_config_dir() {
    local os
    os="$(uname -s)"

    case "$os" in
        Linux)
            echo "$HOME/.config/jilebi"
            ;;
        Darwin)
            echo "$HOME/Library/Preferences/ai.jilebi.jilebi-server"
            ;;
        *)
            echo "$HOME/.jilebi"
            ;;
    esac
}

# Remove from PATH in shell profile
remove_from_path() {
    local shell_profile=""

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
            ;;
        *)
            shell_profile="$HOME/.profile"
            ;;
    esac

    if [[ -f "$shell_profile" ]]; then
        # Create a backup
        cp "$shell_profile" "$shell_profile.jilebi-backup"

        # Remove Jilebi-related lines
        if grep -q "$BIN_DIR" "$shell_profile" 2>/dev/null; then
            # Use sed to remove the Jilebi PATH export and comment
            if [[ "$(uname -s)" == "Darwin" ]]; then
                # macOS sed requires different syntax
                sed -i '' "/# Jilebi/d" "$shell_profile" 2>/dev/null || true
                sed -i '' "/$BIN_DIR/d" "$shell_profile" 2>/dev/null || true
            else
                sed -i "/# Jilebi/d" "$shell_profile" 2>/dev/null || true
                sed -i "/$BIN_DIR/d" "$shell_profile" 2>/dev/null || true
            fi
            success "Removed Jilebi from PATH in $shell_profile"
            info "Backup saved to $shell_profile.jilebi-backup"
        else
            info "Jilebi PATH entry not found in $shell_profile"
        fi
    fi
}

# Remove installation directory
remove_install_dir() {
    if [[ -d "$INSTALL_DIR" ]]; then
        info "Removing installation directory: $INSTALL_DIR"
        rm -rf "$INSTALL_DIR"
        success "Removed $INSTALL_DIR"
    else
        info "Installation directory not found: $INSTALL_DIR"
    fi
}

# Remove data directory
remove_data_dir() {
    local data_dir
    data_dir="$(get_data_dir)"

    if [[ -d "$data_dir" ]]; then
        info "Removing data directory: $data_dir"
        rm -rf "$data_dir"
        success "Removed $data_dir"
    else
        info "Data directory not found: $data_dir"
    fi
}

# Remove config directory
remove_config_dir() {
    local config_dir
    config_dir="$(get_config_dir)"

    if [[ -d "$config_dir" ]]; then
        info "Removing config directory: $config_dir"
        rm -rf "$config_dir"
        success "Removed $config_dir"
    else
        info "Config directory not found: $config_dir"
    fi
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
    echo "        Uninstallation Script"
    echo ""

    # Check if jilebi is installed
    if [[ ! -d "$INSTALL_DIR" ]] && [[ ! -f "$BIN_DIR/jilebi" ]]; then
        warn "Jilebi does not appear to be installed at $INSTALL_DIR"
        echo ""
        read -p "Continue anyway? [y/N] " -n 1 -r
        echo ""
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            info "Uninstallation cancelled."
            exit 0
        fi
    fi

    echo ""
    warn "This will remove Jilebi and all its data from your system."
    echo ""
    echo "The following will be removed:"
    echo "  - Installation directory: $INSTALL_DIR"
    echo "  - Data directory: $(get_data_dir)"
    echo "  - Config directory: $(get_config_dir)"
    echo "  - PATH entry from your shell profile"
    echo ""

    read -p "Are you sure you want to uninstall Jilebi? [y/N] " -n 1 -r
    echo ""

    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        info "Uninstallation cancelled."
        exit 0
    fi

    echo ""

    # Ask about keeping data
    read -p "Keep plugins and data? [y/N] " -n 1 -r
    echo ""
    local keep_data=false
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        keep_data=true
        info "Plugins and data will be preserved."
    fi

    echo ""
    info "Uninstalling Jilebi..."
    echo ""

    # Remove from PATH
    remove_from_path

    # Remove installation directory
    remove_install_dir

    # Remove data and config if not keeping
    if [[ "$keep_data" == false ]]; then
        remove_data_dir
        remove_config_dir
    else
        info "Skipping data and config removal (--keep-data)"
    fi

    echo ""
    success "Jilebi has been uninstalled!"
    echo ""
    info "Please restart your shell or run: source ~/.$(basename "$SHELL")rc"
    echo ""
}

main "$@"
