#!/bin/bash
# zippy bootstrap — installs zippy, which installs Zeta
# Usage: curl -sSf https://raw.githubusercontent.com/murphsicles/zippy/main/install.sh | sh
#   Windows (Git Bash / WSL): same command
#   Windows (PowerShell): iex (iwr -useb https://raw.githubusercontent.com/murphsicles/zippy/main/install.ps1)
set -euo pipefail

VERSION="v0.1.1"
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
echo -e "${AMBER}║${NC}  ${BOLD}███████╗███████╗████████╗${NC}   ${AMBER}║${NC}"
echo -e "${AMBER}║${NC}  ${BOLD}╚══███╔╝╚══███╔╝╚══███╔╝${NC}   ${AMBER}║${NC}"
echo -e "${AMBER}║${NC}  ${BOLD}  ███╔╝   ███╔╝   ███╔╝${NC}    ${AMBER}║${NC}"
echo -e "${AMBER}║${NC}  ${BOLD} ███╔╝   ███╔╝   ███╔╝${NC}     ${AMBER}║${NC}"
echo -e "${AMBER}║${NC}  ${BOLD}███████╗███████╗███████╗${NC}   ${AMBER}║${NC}"
echo -e "${AMBER}║${NC}  ${BOLD}╚══════╝╚══════╝╚══════╝${NC}   ${AMBER}║${NC}"
echo -e "${AMBER}║${NC}                                    ${AMBER}║${NC}"
echo -e "${AMBER}║${NC}  ${MUTED}Zeta Installer ${VERSION}${NC}          ${AMBER}║${NC}"
echo -e "${AMBER}╚════════════════════════════════════╝${NC}"
echo ""

# Detect platform
OS="$(uname -s)"
ARCH="$(uname -m)"

# Windows detection (Git Bash / MSYS2 / Cygwin report "MINGW*" or "MSYS*" or "CYGWIN*")
case "$(uname -o 2>/dev/null || echo '')" in
    Msys|Cygwin|MSYS|CYGWIN|MINGW*|MSYS*) IS_WINDOWS=1 ;;
    *) IS_WINDOWS=0 ;;
esac

case "${OS}-${ARCH}" in
    Linux-x86_64|Linux-amd64)  ASSET="zippy-linux-x64";       EXE="" ;;
    Linux-aarch64)             ASSET="zippy-linux-arm64";     EXE="" ;;
    Darwin-x86_64)             ASSET="zippy-macos-x64";       EXE="" ;;
    Darwin-arm64|Darwin-aarch64) ASSET="zippy-macos-arm64";   EXE="" ;;
    *)
        if [ "${IS_WINDOWS}" = "1" ]; then
            case "${ARCH}" in
                x86_64|amd64) ASSET="zippy-windows-x64.exe"; EXE=".exe" ;;
                aarch64)      ASSET="zippy-windows-arm64.exe"; EXE=".exe" ;;
                *)
                    echo -e "${RED}Unsupported Windows architecture: ${ARCH}${NC}"
                    echo -e "${MUTED}Build from source: https://github.com/${REPO}${NC}"
                    exit 1
                    ;;
            esac
        else
            echo -e "${RED}Unsupported: ${OS} ${ARCH}${NC}"
            echo -e "${MUTED}Build from source: https://github.com/${REPO}${NC}"
            echo -e "${MUTED}Windows users: run PowerShell as Administrator and paste:${NC}"
            echo -e "${TEXT}  iex (iwr -useb https://raw.githubusercontent.com/${REPO}/main/install.ps1)${NC}"
            exit 1
        fi
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
mv "${TMPFILE}" "${INSTALL_DIR}/${BINARY}${EXE}"

# Add to PATH
PATH_ENTRY="${INSTALL_DIR}"
if ! echo "$PATH" | grep -q "${PATH_ENTRY}"; then
    RC_FILE=""
    case "${SHELL:-}" in
        *zsh*)  RC_FILE="${HOME}/.zshrc" ;;
        *bash*) RC_FILE="${HOME}/.bashrc" ;;
    esac
    # If running under Windows (Git Bash), use .bash_profile or .bashrc
    if [ "${IS_WINDOWS}" = "1" ] && [ -z "${RC_FILE}" ]; then
        RC_FILE="${HOME}/.bashrc"
    fi
    if [ -n "${RC_FILE}" ]; then
        echo "" >> "${RC_FILE}"
        echo "# Added by zippy" >> "${RC_FILE}"
        echo "export PATH=\"${PATH_ENTRY}:\$PATH\"" >> "${RC_FILE}"
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
