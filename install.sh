#!/bin/bash
# zippy bootstrap — installs zippy, which installs Zeta
# Usage: curl -sSf https://raw.githubusercontent.com/murphsicles/zippy/main/install.sh | sh
set -euo pipefail

VERSION="v0.1.0"
REPO="murphsicles/zippy"
INSTALL_DIR="${HOME}/.zippy/bin"
BINARY="zippy"

# Colors
AMBER='\033[38;2;234;179;8m'
GREEN='\033[38;2;34;197;94m'
RED='\033[38;2;239;68;68m'
MUTED='\033[38;2;173;181;189m'
TEXT='\033[38;2;216;222;233m'
BOLD='\033[1m'
NC='\033[0m'

echo ""
echo -e "${AMBER}╔════════════════════════════════════╗${NC}"
echo -e "${AMBER}║${NC}                                    ${AMBER}║${NC}"
echo -e "${AMBER}║${NC}  ${BOLD}███████╗███████╗████████╗${BOLD_R}   ${AMBER}║${NC}"
echo -e "${AMBER}║${NC}  ${BOLD}╚══███╔╝╚══███╔╝╚══███╔╝${BOLD_R}   ${AMBER}║${NC}"
echo -e "${AMBER}║${NC}  ${BOLD}  ███╔╝   ███╔╝   ███╔╝${BOLD_R}    ${AMBER}║${NC}"
echo -e "${AMBER}║${NC}  ${BOLD} ███╔╝   ███╔╝   ███╔╝${BOLD_R}     ${AMBER}║${NC}"
echo -e "${AMBER}║${NC}  ${BOLD}███████╗███████╗███████╗${BOLD_R}   ${AMBER}║${NC}"
echo -e "${AMBER}║${NC}  ${BOLD}╚══════╝╚══════╝╚══════╝${BOLD_R}   ${AMBER}║${NC}"
echo -e "${AMBER}║${NC}                                    ${AMBER}║${NC}"
echo -e "${AMBER}║${NC}  ${MUTED}Zeta Installer ${VERSION}${NC}          ${AMBER}║${NC}"
echo -e "${AMBER}╚════════════════════════════════════╝${NC}"
echo ""

# Detect platform
OS="$(uname -s)"
ARCH="$(uname -m)"

case "${OS}-${ARCH}" in
    Linux-x86_64|Linux-amd64)  ASSET="zippy-linux-x64" ;;
    Linux-aarch64)             ASSET="zippy-linux-arm64" ;;
    Darwin-x86_64)             ASSET="zippy-macos-x64" ;;
    Darwin-arm64|Darwin-aarch64) ASSET="zippy-macos-arm64" ;;
    *)
        echo -e "${RED}Unsupported: ${OS} ${ARCH}${NC}"
        echo -e "${MUTED}Build from source: https://github.com/${REPO}${NC}"
        exit 1
        ;;
esac

DOWNLOAD_URL="https://github.com/${REPO}/releases/download/${VERSION}/${ASSET}"

# Install
echo -e "  ${BOLD}${AMBER}▶${NC} ${TEXT}Downloading zippy...${NC}"
mkdir -p "${INSTALL_DIR}"
TMPFILE=$(mktemp)
trap 'rm -f "${TMPFILE}"' EXIT

if command -v curl &>/dev/null; then
    curl -sSfL "${DOWNLOAD_URL}" -o "${TMPFILE}"
elif command -v wget &>/dev/null; then
    wget -q "${DOWNLOAD_URL}" -O "${TMPFILE}"
else
    echo -e "${RED}Need curl or wget.${NC}"
    exit 1
fi

chmod +x "${TMPFILE}"
mv "${TMPFILE}" "${INSTALL_DIR}/${BINARY}"

# Add to PATH
if ! echo "$PATH" | grep -q "${INSTALL_DIR}"; then
    RC_FILE=""
    case "${SHELL:-}" in
        *zsh*) RC_FILE="${HOME}/.zshrc" ;;
        *bash*) RC_FILE="${HOME}/.bashrc" ;;
    esac
    if [ -n "${RC_FILE}" ]; then
        echo "" >> "${RC_FILE}"
        echo "# Added by zippy" >> "${RC_FILE}"
        echo "export PATH=\"${INSTALL_DIR}:\$PATH\"" >> "${RC_FILE}"
        echo -e "  ${MUTED}└─${NC} ${GREEN}✓${NC} ${MUTED}Added to ${RC_FILE}${NC}"
    fi
fi

echo ""
echo -e "${GREEN}${BOLD}✓ zippy installed!${NC}"
echo ""
echo -e "  ${MUTED}Next:${NC}"
echo -e "  ${AMBER}▶${NC} ${TEXT}zippy install${NC}"
echo -e "  ${AMBER}▶${NC} ${TEXT}zetac --zorb search nour${NC}"
echo ""
echo -e "  ${MUTED}z-lang.org  ·  zorbs.io/docs${NC}"
echo ""
