#!/usr/bin/env bash
#
# DEPRECATED: Manual Enzyme installation is no longer required
#
# As of late 2024, Rust nightly has built-in Enzyme support via the
# #![feature(autodiff)] feature. You do not need to build or install
# Enzyme separately.
#
# To use autodiff:
# 1. Use Rust nightly toolchain
# 2. Add #![feature(autodiff)] to your crate
# 3. Build with: RUSTFLAGS="-Z autodiff=Enable" cargo +nightly build
#
# See: https://github.com/rust-lang/rust/issues/124509
#

echo "=================================================="
echo "NOTE: Manual Enzyme installation is no longer needed"
echo "=================================================="
echo ""
echo "Rust nightly now includes built-in Enzyme support."
echo ""
echo "To use autodiff in your Rust project:"
echo ""
echo "1. Use Rust nightly:"
echo "   rustup default nightly"
echo ""
echo "2. Add to your lib.rs or main.rs:"
echo "   #![feature(autodiff)]"
echo "   use std::autodiff::autodiff;"
echo ""
echo "3. Build with autodiff enabled:"
echo "   export RUSTFLAGS=\"-Z autodiff=Enable\""
echo "   cargo build"
echo ""
echo "Run ./scripts/verify_enzyme.sh to test autodiff support."
echo ""
