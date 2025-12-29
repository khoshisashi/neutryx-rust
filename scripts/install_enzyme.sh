#!/usr/bin/env bash
#
# Install Enzyme LLVM plugin for automatic differentiation
#
# This script:
# 1. Clones the Enzyme repository
# 2. Builds the LLVM plugin
# 3. Installs to /usr/local/lib
# 4. Verifies the installation

set -euo pipefail

# Configuration
ENZYME_VERSION="${ENZYME_VERSION:-v0.0.100}"
LLVM_VERSION="${LLVM_VERSION:-18}"
INSTALL_DIR="${INSTALL_DIR:-/usr/local/lib}"
ENZYME_REPO="https://github.com/EnzymeAD/Enzyme.git"

echo "===================================="
echo "Enzyme Installation Script"
echo "===================================="
echo "Enzyme version: $ENZYME_VERSION"
echo "LLVM version: $LLVM_VERSION"
echo "Install directory: $INSTALL_DIR"
echo ""

# Check for required dependencies
echo "[1/5] Checking dependencies..."
for cmd in git cmake ninja clang-$LLVM_VERSION llvm-config-$LLVM_VERSION; do
    if ! command -v $cmd &> /dev/null; then
        echo "Error: $cmd not found. Please install LLVM-$LLVM_VERSION development tools."
        exit 1
    fi
done
echo "✓ All dependencies found"

# Clone Enzyme
echo "[2/5] Cloning Enzyme repository..."
ENZYME_DIR=$(mktemp -d)
git clone --depth 1 --branch $ENZYME_VERSION $ENZYME_REPO $ENZYME_DIR
echo "✓ Cloned to $ENZYME_DIR"

# Build Enzyme
echo "[3/5] Building Enzyme (this may take 15-30 minutes)..."
cd $ENZYME_DIR/enzyme
mkdir build && cd build

cmake .. \
    -G Ninja \
    -DLLVM_DIR=/usr/lib/llvm-$LLVM_VERSION/lib/cmake/llvm \
    -DCMAKE_BUILD_TYPE=Release

ninja
echo "✓ Build complete"

# Install
echo "[4/5] Installing Enzyme to $INSTALL_DIR..."
sudo ninja install

# Verify installation
echo "[5/5] Verifying installation..."
ENZYME_LIB="$INSTALL_DIR/LLVMEnzyme-$LLVM_VERSION.so"
if [ -f "$ENZYME_LIB" ]; then
    echo "✓ Enzyme installed successfully: $ENZYME_LIB"
else
    echo "✗ Installation failed: $ENZYME_LIB not found"
    exit 1
fi

# Cleanup
cd /
rm -rf $ENZYME_DIR
echo "✓ Cleanup complete"

echo ""
echo "===================================="
echo "Installation Complete!"
echo "===================================="
echo ""
echo "To use Enzyme with Rust, set:"
echo "  export RUSTFLAGS=\"-C llvm-args=-load=$ENZYME_LIB\""
echo ""
echo "Or add to .cargo/config.toml:"
echo "  [build]"
echo "  rustflags = [\"-C\", \"llvm-args=-load=$ENZYME_LIB\"]"
echo ""
