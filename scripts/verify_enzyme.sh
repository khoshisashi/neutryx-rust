#!/usr/bin/env bash
#
# Verify Rust autodiff (Enzyme) integration
#
# This script:
# 1. Checks Rust nightly toolchain
# 2. Creates a simple test Rust function
# 3. Compiles with autodiff enabled (-Z autodiff=Enable)
# 4. Runs basic differentiation test
#
# Note: Modern Rust nightly has built-in Enzyme support via #![feature(autodiff)]
# No external Enzyme library is required.

set -euo pipefail

echo "===================================="
echo "Rust Autodiff Verification Script"
echo "===================================="
echo ""

# Check Rust nightly
echo "[1/3] Checking Rust toolchain..."
if ! rustc +nightly --version &> /dev/null; then
    echo "✗ Nightly Rust not found"
    echo "Install with: rustup toolchain install nightly"
    exit 1
fi
echo "✓ Nightly Rust: $(rustc +nightly --version)"

# Check if autodiff feature is available
echo "[2/3] Checking autodiff support..."
if rustc +nightly -Z help 2>&1 | grep -q "autodiff"; then
    echo "✓ autodiff feature available"
else
    echo "⚠ autodiff may not be available in this nightly version"
    echo "  This is expected - autodiff is still experimental"
fi

# Create test project
echo "[3/3] Creating and testing autodiff project..."
TEST_DIR=$(mktemp -d)
cd $TEST_DIR

cat > Cargo.toml <<'EOF'
[package]
name = "autodiff_test"
version = "0.1.0"
edition = "2024"

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
    fn test_autodiff_derivative() {
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

# Build and test with autodiff enabled
# Note: -Z autodiff=Enable activates Rust's built-in Enzyme support
export RUSTFLAGS="-Z autodiff=Enable"

if cargo +nightly test 2>&1 | tee build.log; then
    echo ""
    echo "✓ Autodiff verification PASSED"
    echo ""
    echo "===================================="
    echo "Verification Complete!"
    echo "===================================="
    echo ""
    echo "Rust autodiff (Enzyme) is working correctly."
    echo ""
    # Cleanup
    cd /
    rm -rf $TEST_DIR
else
    echo ""
    echo "✗ Autodiff verification FAILED"
    echo ""
    echo "Build log saved to: $TEST_DIR/build.log"
    echo "Common issues:"
    echo "  - autodiff feature not available in this nightly"
    echo "  - Missing #![feature(autodiff)] in source"
    echo "  - Rust nightly version too old (autodiff merged ~2024)"
    echo ""
    echo "Note: autodiff is still experimental and may not be"
    echo "available in all nightly builds."
    echo ""
    exit 1
fi
