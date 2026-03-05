#!/usr/bin/env bash
set -euo pipefail

VERSION="15.2.rel1"
INSTALL_DIR=""
FORCE=0

usage() {
    cat <<EOF
Usage: $0 [options]

Download and install Arm GNU Toolchain into a chosen directory.

Options:
  -v, --version <version>      Toolchain version (default: ${VERSION})
  -d, --install-dir <dir>      Install directory (default: ~/.local/arm-gnu-toolchain/<version>)
  -f, --force                  Remove existing install directory before installing
  -h, --help                   Show this help

Examples:
  $0
  $0 --version 14.2.rel1
  $0 --version 15.2.rel1 --install-dir "$HOME/.local/gcc-arm/15.2.rel1"

After installation, export GCC_HOME to the install directory.
EOF
}

while [[ $# -gt 0 ]]; do
    case "$1" in
        -v|--version)
            VERSION="$2"
            shift 2
            ;;
        -d|--install-dir)
            INSTALL_DIR="$2"
            shift 2
            ;;
        -f|--force)
            FORCE=1
            shift
            ;;
        -h|--help)
            usage
            exit 0
            ;;
        *)
            echo "Unknown argument: $1" >&2
            usage
            exit 1
            ;;
    esac
done

if [[ -z "${INSTALL_DIR}" ]]; then
    INSTALL_DIR="${HOME}/.local/arm-gnu-toolchain/${VERSION}"
fi

ARCHIVE="arm-gnu-toolchain-${VERSION}-x86_64-arm-none-eabi.tar.xz"
URL="https://developer.arm.com/-/media/Files/downloads/gnu/${VERSION}/binrel/${ARCHIVE}"

if [[ -d "${INSTALL_DIR}" ]]; then
    if [[ "${FORCE}" -eq 1 ]]; then
        rm -rf "${INSTALL_DIR}"
    else
        echo "Install directory already exists: ${INSTALL_DIR}" >&2
        echo "Use --force to overwrite." >&2
        exit 1
    fi
fi

mkdir -p "${INSTALL_DIR}"

TMP_ARCHIVE="$(mktemp /tmp/arm-gcc-${VERSION}.XXXXXX.tar.xz)"
trap 'rm -f "${TMP_ARCHIVE}"' EXIT

echo "Downloading ${URL}"
if command -v wget >/dev/null 2>&1; then
    wget -q "${URL}" -O "${TMP_ARCHIVE}"
elif command -v curl >/dev/null 2>&1; then
    curl -fsSL "${URL}" -o "${TMP_ARCHIVE}"
else
    echo "Neither wget nor curl is available. Install one of them and retry." >&2
    exit 1
fi

echo "Extracting to ${INSTALL_DIR}"
tar -xf "${TMP_ARCHIVE}" --strip-components=1 -C "${INSTALL_DIR}"

if [[ ! -x "${INSTALL_DIR}/bin/arm-none-eabi-gcc" ]]; then
    echo "Installation failed: arm-none-eabi-gcc not found in ${INSTALL_DIR}/bin" >&2
    exit 1
fi

echo "Installed Arm GNU Toolchain ${VERSION} to ${INSTALL_DIR}"
echo
echo "Add this to your shell before running tests:"
echo "  export GCC_HOME=${INSTALL_DIR}"
