#!/usr/bin/env bash
set -euo pipefail

# Install locales only if they aren't already present
if ! dpkg -s locales >/dev/null 2>&1; then
  apt-get update
  apt-get install -y --no-install-recommends locales
fi

# Enable and generate en_US.UTF-8 (idempotent)
if ! grep -q '^en_US.UTF-8 UTF-8' /etc/locale.gen; then
  echo 'en_US.UTF-8 UTF-8' >> /etc/locale.gen
fi
locale-gen en_US.UTF-8
update-locale LANG=en_US.UTF-8

# Export for the current shell (helps the script itself)
export LANG=en_US.UTF-8
export LC_ALL=en_US.UTF-8

# Set USER if not already set (common in containers)
export USER=${USER:-root}
export HOME=${HOME:-/root}

# Install Rust via rustup (avoid Nix glibc conflicts with Swift)
if ! command -v cargo &> /dev/null; then
    echo "Installing Rust via rustup..."
    apt-get update
    apt-get install -y --no-install-recommends curl build-essential
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable
    export PATH="$HOME/.cargo/bin:$PATH"
    echo "✓ Rust installed via rustup"
else
    echo "✓ Rust already installed: $(rustc --version)"
fi

# Get the directory where this script lives
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORKSPACE_DIR="$(dirname "$SCRIPT_DIR")"

# Add Nix channels
nix-channel --add https://github.com/nix-community/home-manager/archive/master.tar.gz home-manager
nix-channel --update

# Install home-manager
nix-shell '<home-manager>' -A install

# Apply the configuration from this repo
home-manager switch -b backup -f "${SCRIPT_DIR}/home.nix"

# Install Swift from Swift.org (Nix Swift build fails on Linux)
SWIFT_VERSION="6.2.3"
SWIFT_PLATFORM="debian12"
if ! command -v swift &> /dev/null; then
    echo "Installing Swift ${SWIFT_VERSION} for Debian 12..."
    apt-get update
    apt-get install -y --no-install-recommends \
        binutils \
        git \
        gnupg2 \
        libc6-dev \
        libcurl4-openssl-dev \
        libedit2 \
        libgcc-12-dev \
        libncurses6 \
        libpython3-dev \
        libsqlite3-0 \
        libstdc++-12-dev \
        libxml2-dev \
        libz3-dev \
        pkg-config \
        tzdata \
        unzip \
        zlib1g-dev \
        wget

    cd /tmp
    # Detect architecture and download appropriate Swift package
    ARCH=$(uname -m)
    if [ "$ARCH" = "x86_64" ]; then
        SWIFT_URL="https://download.swift.org/swift-${SWIFT_VERSION}-release/${SWIFT_PLATFORM}/swift-${SWIFT_VERSION}-RELEASE/swift-${SWIFT_VERSION}-RELEASE-${SWIFT_PLATFORM}.tar.gz"
        SWIFT_DIR="swift-${SWIFT_VERSION}-RELEASE-${SWIFT_PLATFORM}"
    elif [ "$ARCH" = "aarch64" ]; then
        SWIFT_URL="https://download.swift.org/swift-${SWIFT_VERSION}-release/${SWIFT_PLATFORM}-aarch64/swift-${SWIFT_VERSION}-RELEASE/swift-${SWIFT_VERSION}-RELEASE-${SWIFT_PLATFORM}-aarch64.tar.gz"
        SWIFT_DIR="swift-${SWIFT_VERSION}-RELEASE-${SWIFT_PLATFORM}-aarch64"
    else
        echo "⚠️  Unsupported architecture: $ARCH"
        exit 1
    fi

    echo "Downloading Swift for $ARCH..."
    wget -q "$SWIFT_URL"
    tar xzf "${SWIFT_DIR}.tar.gz"
    mv "$SWIFT_DIR" /usr/share/swift
    ln -s /usr/share/swift/usr/bin/swift /usr/local/bin/swift
    ln -s /usr/share/swift/usr/bin/swiftc /usr/local/bin/swiftc
    rm "${SWIFT_DIR}.tar.gz"
    echo "✓ Swift ${SWIFT_VERSION} installed"
else
    echo "✓ Swift already installed: $(swift --version | head -n1)"
fi

# Allow direnv for this template repo (if it has .envrc)
if [ -f "${WORKSPACE_DIR}/.envrc" ]; then
    cd "${WORKSPACE_DIR}"
    direnv allow
fi