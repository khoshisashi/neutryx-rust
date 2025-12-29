# Verification Checklist: pricer_core Implementation

## Pre-requisites

Before running verification commands, ensure you're in the workspace root:
```bash
cd /workspaces/neutryx-rust
```

## Step 1: Code Formatting

```bash
cargo fmt --all -- --check
```

**Expected:** No output (all files already formatted)

**If needed to format:**
```bash
cargo fmt --all
```

## Step 2: Linting with Clippy

```bash
cargo clippy -p pricer_core --all-targets --all-features -- -D warnings
```

**Expected:** No warnings or errors

**Common issues:**
- Unused imports: Remove them
- Needless borrows: Simplify code
- Complex type signatures: Already optimized

## Step 3: Build Verification (Stable Rust)

```bash
cargo build -p pricer_core
```

**Expected:** Successful compilation with 0 errors

**Verify features:**
```bash
# Default features (num-dual-mode enabled)
cargo build -p pricer_core

# No features
cargo build -p pricer_core --no-default-features

# Enzyme mode
cargo build -p pricer_core --no-default-features --features enzyme-mode
```

## Step 4: Test Execution

### Unit Tests
```bash
cargo test -p pricer_core --lib
```

**Expected:**
- All smoothing function tests pass
- All Dual number tests pass (with num-dual-mode)
- All Day Count Convention tests pass
- All trait tests pass

### Integration Tests
```bash
cargo test -p pricer_core --test integration_test
```

**Expected:**
- Module export tests pass
- Cross-module integration tests pass
- Dual number integration tests pass (with num-dual-mode)

### Property Tests
```bash
cargo test -p pricer_core -- --include-ignored
```

**Expected:**
- 16 property tests pass with 1000 cases each
- No test failures or panics

### All Tests Combined
```bash
cargo test -p pricer_core
```

**Expected:** All 60+ tests pass

## Step 5: Documentation Tests

```bash
cargo test --doc -p pricer_core
```

**Expected:** All documentation examples compile and run successfully

## Step 6: Documentation Generation

```bash
cargo doc --no-deps -p pricer_core --open
```

**Expected:**
- Documentation generates without warnings
- All public items documented
- No broken links
- Examples render correctly

**Manual checks:**
- Verify lib.rs crate-level docs
- Check all module docs (`//!`)
- Verify all function docs (`///`)
- Ensure British English spelling

## Step 7: Dependency Tree Verification

```bash
cargo tree -p pricer_core
```

**Expected dependencies:**
```
pricer_core v0.1.0
├── chrono v0.4.*
├── num-dual v0.9.* (with num-dual-mode feature)
├── num-traits v0.2.*
├── serde v1.0.*
└── thiserror v2.0.*
```

**Verify:**
- ✅ No dependencies on other pricer_* crates
- ✅ Only specified external dependencies
- ✅ No unexpected transitive dependencies

```bash
# Verify no pricer_* dependencies
cargo tree -p pricer_core | grep -i pricer
```

**Expected:** Only "pricer_core" appears (no other pricer_* crates)

## Step 8: Feature Flag Verification

### Test with num-dual-mode (default)
```bash
cargo test -p pricer_core --features num-dual-mode
```

**Expected:** All tests pass, DualNumber tests included

### Test without num-dual-mode
```bash
cargo test -p pricer_core --no-default-features
```

**Expected:**
- Smoothing tests pass
- Time tests pass
- Trait tests pass
- DualNumber tests skipped (feature-gated)

### Test with enzyme-mode
```bash
cargo build -p pricer_core --no-default-features --features enzyme-mode
```

**Expected:** Builds successfully (no Dual number module)

## Step 9: Performance Check (Optional)

### Build with optimizations
```bash
cargo build -p pricer_core --release
```

**Expected:**
- Successful compilation
- LTO enabled (from workspace profile)
- codegen-units = 1 applied

### Quick benchmark (manual)
```rust
use pricer_core::math::smoothing::smooth_max;
use std::time::Instant;

let start = Instant::now();
for _ in 0..1_000_000 {
    let _ = smooth_max(3.0, 5.0, 1e-6);
}
let duration = start.elapsed();
println!("1M smooth_max calls: {:?}", duration);
```

**Expected:** < 50ms for 1M calls (release mode, typical hardware)

## Step 10: Coverage Analysis (Optional)

Using `cargo-tarpaulin` (if installed):
```bash
cargo tarpaulin -p pricer_core --out Html
```

**Expected:**
- Line coverage > 90%
- All public functions covered
- Edge cases tested

## Final Verification Matrix

| Check | Command | Status |
|-------|---------|--------|
| Format | `cargo fmt --all -- --check` | ⬜ |
| Clippy | `cargo clippy -p pricer_core -- -D warnings` | ⬜ |
| Build (default) | `cargo build -p pricer_core` | ⬜ |
| Build (no features) | `cargo build -p pricer_core --no-default-features` | ⬜ |
| Build (enzyme) | `cargo build -p pricer_core --features enzyme-mode --no-default-features` | ⬜ |
| Unit tests | `cargo test -p pricer_core --lib` | ⬜ |
| Integration tests | `cargo test -p pricer_core --test integration_test` | ⬜ |
| Property tests | `cargo test -p pricer_core` | ⬜ |
| Doc tests | `cargo test --doc -p pricer_core` | ⬜ |
| Documentation | `cargo doc --no-deps -p pricer_core` | ⬜ |
| Dependency tree | `cargo tree -p pricer_core` | ⬜ |
| Release build | `cargo build -p pricer_core --release` | ⬜ |

## Success Criteria

All checks must pass:
- ✅ Zero compiler errors
- ✅ Zero clippy warnings
- ✅ All tests pass (60+ tests)
- ✅ Documentation generates cleanly
- ✅ No dependencies on other pricer_* crates
- ✅ Feature flags work correctly
- ✅ British English in all comments

## Troubleshooting

### Issue: Clippy warnings about unused code
**Solution:** Remove unused imports/variables or add `#[allow(dead_code)]` if intentional

### Issue: Doc tests fail
**Solution:** Check that all `///` examples are wrapped in proper fences and feature gates

### Issue: Property tests timeout
**Solution:** Reduce proptest cases temporarily (default 1000 is recommended for production)

### Issue: Dependency resolution errors
**Solution:** Run `cargo update` and check Cargo.lock

### Issue: Feature gate conflicts
**Solution:** Verify `#[cfg(feature = "...")]` attributes match Cargo.toml features

## Ready for Next Phase

Once all checks pass:
- ✅ Commit changes with descriptive message
- ✅ Update specification status (`/kiro:spec-status math-foundation-phase1`)
- ✅ Proceed to Phase 2 (Layer 2: pricer_models)
