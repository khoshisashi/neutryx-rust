#!/usr/bin/env bash
#
# Verify Enzyme installation and Rust integration
#
# This script:
# 1. Checks if Enzyme library exists
# 2. Creates a simple test Rust function
# 3. Compiles with Enzyme AD enabled
# 4. Runs basic differentiation test

set -euo pipefail

LLVM_VERSION="${LLVM_VERSION:-18}"
ENZYME_LIB="/usr/local/lib/LLVMEnzyme-$LLVM_VERSION.so"

echo "===================================="
echo "Enzyme Verification Script"
echo "===================================="
echo ""

# Check Enzyme library
echo "[1/4] Checking Enzyme library..."
if [ -f "$ENZYME_LIB" ]; then
    echo "✓ Enzyme found: $ENZYME_LIB"
else
    echo "✗ Enzyme not found: $ENZYME_LIB"
    echo "Run ./scripts/install_enzyme.sh first"
    exit 1
fi

# Check Rust nightly
echo "[2/4] Checking Rust toolchain..."
if ! rustc +nightly --version &> /dev/null; then
    echo "✗ Nightly Rust not found"
    echo "Install with: rustup toolchain install nightly"
    exit 1
fi
echo "✓ Nightly Rust: $(rustc +nightly --version)"

# Create test project
echo "[3/4] Creating test project..."
TEST_DIR=$(mktemp -d)
cd $TEST_DIR

cat > Cargo.toml <<'EOF'
[package]
name = "enzyme_test"
version = "0.1.0"
edition = "2021"

[dependencies]
EOF

mkdir src
cat > src/lib.rs <<'EOF'
#![feature(autodiff)]
use std::autodiff::autodiff;

// Simple test: f(x) = x²
// Expected: f'(x) = 2x
#[autodiff(d_square, Reverse, Duplicated, Active)]
pub fn square(x: f64) -> f64 {
    x * x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enzyme_derivative() {
        let x = 3.0;
        let mut dx = 1.0;

        let result = d_square(x, &mut dx);

        // f(3) = 9
        assert!((result - 9.0).abs() < 1e-10);

        // f'(3) = 6
        assert!((dx - 6.0).abs() < 1e-10);
    }
}
EOF

# Build and test
echo "[4/4] Building and testing with Enzyme..."
export RUSTFLAGS="-C llvm-args=-load=$ENZYME_LIB"

if cargo +nightly test 2>&1 | tee build.log; then
    echo ""
    echo "✓ Enzyme verification PASSED"
    echo ""
    echo "===================================="
    echo "Verification Complete!"
    echo "===================================="
    echo ""
    echo "Enzyme is correctly installed and working with Rust."
    echo ""
else
    echo ""
    echo "✗ Enzyme verification FAILED"
    echo ""
    echo "Build log saved to: $TEST_DIR/build.log"
    echo "Common issues:"
    echo "  - LLVM version mismatch"
    echo "  - Enzyme not built for correct LLVM version"
    echo "  - Missing #![feature(autodiff)] flag"
    echo ""
    exit 1
fi

# Cleanup
cd /
rm -rf $TEST_DIR
