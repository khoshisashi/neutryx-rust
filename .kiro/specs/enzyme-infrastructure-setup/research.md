# Research & Design Decisions: Enzyme Infrastructure Setup

---
**Purpose**: Document discovery findings for Enzyme AD infrastructure in pricer_kernel.
**Discovery Scope**: Complex Integration (nightly Rust + LLVM plugin)
---

## Summary

- **Feature**: `enzyme-infrastructure-setup`
- **Discovery Scope**: Complex Integration
- **Key Findings**:
  - Rust nightly provides `#[autodiff_forward]` and `#[autodiff_reverse]` macros via Enzyme
  - LLVM 18 is required; llvm-sys 180 provides Rust bindings
  - Existing pricer_kernel crate has placeholder implementation ready for upgrade

## Research Log

### Enzyme AD Integration Status in Rust

- **Context**: Determining current state of Enzyme support in Rust nightly toolchain
- **Sources Consulted**:
  - [Rust nightly autodiff intrinsic](https://doc.rust-lang.org/nightly/std/intrinsics/fn.autodiff.html)
  - [Enzyme MIT project](https://enzyme.mit.edu/)
  - [Enzyme GitHub](https://github.com/EnzymeAD/Enzyme)
  - [Rust-Enzyme ecosystem history](https://enzyme.mit.edu/rust/ecosystem.html)
- **Findings**:
  - `std::intrinsics::autodiff` available in nightly Rust
  - Attribute macros `#[autodiff_forward]` and `#[autodiff_reverse]` expand to intrinsic calls
  - Function signature: `autodiff<F, G, T, R>(f: F, df: G, args: T) -> R`
  - Requires `#![feature(autodiff)]` or `#![feature(core_intrinsics)]`
- **Implications**: Phase 3.0 can use placeholder; Phase 4 will integrate actual Enzyme macros

### LLVM Version Requirements

- **Context**: Determining required LLVM version for Enzyme compatibility
- **Sources Consulted**:
  - llvm-sys crate documentation
  - Enzyme LLVM plugin requirements
- **Findings**:
  - Enzyme requires LLVM 18 for current nightly compatibility
  - llvm-sys version 180 corresponds to LLVM 18.x
  - RUSTFLAGS must include `-C llvm-args=-load=/path/to/LLVMEnzyme-18.so`
- **Implications**: Build script should validate LLVM 18 presence; Docker recommended for reproducibility

### Existing Crate Structure Analysis

- **Context**: Analysing current pricer_kernel implementation
- **Sources Consulted**: Codebase analysis via Grep/Read
- **Findings**:
  - rust-toolchain.toml already specifies `nightly-2025-01-15`
  - Cargo.toml has `llvm-sys = "180"` dependency
  - verify module contains placeholder `square` and `square_gradient` functions
  - Comprehensive test suite with 7 tests including finite difference validation
  - enzyme/mc/checkpoint modules exist but are commented out
- **Implications**: Infrastructure largely in place; design focuses on formalising contracts and enabling actual Enzyme integration

### Autodiff Macro Usage Pattern

- **Context**: Understanding how to use Enzyme macros in Rust
- **Sources Consulted**: Rust nightly documentation, LLVM Dev Meeting 2024 slides
- **Findings**:
  - Forward mode: `#[autodiff_forward(df, Dual, Const, Dual)]`
  - Reverse mode: `#[autodiff_reverse(df, Duplicated, Const, Active)]`
  - Activity annotations: `Dual`, `Const`, `Active`, `Duplicated`, `DuplicatedOnly`
  - Shadow arguments automatically added for active parameters
- **Implications**: verify module needs to use `#[autodiff_reverse]` for gradient computation

## Architecture Pattern Evaluation

| Option | Description | Strengths | Risks / Limitations | Notes |
|--------|-------------|-----------|---------------------|-------|
| Placeholder + Feature Flag | Current approach: analytical placeholder, Enzyme behind feature flag | Safe, builds without Enzyme | Requires conditional compilation | Aligns with phased development |
| Direct Enzyme Integration | Enable `#![feature(autodiff)]` and use macros directly | Full AD capability | Requires Enzyme plugin installed | Target for Phase 4 |
| FFI to Enzyme | Call Enzyme via C FFI | Decouples from rustc | Complex build, maintenance burden | Rejected: outdated approach |

**Selected**: Placeholder + Feature Flag approach for Phase 3.0, with infrastructure ready for Phase 4 direct integration.

## Design Decisions

### Decision: Placeholder Implementation for Phase 3.0

- **Context**: Need infrastructure validation without requiring full Enzyme setup
- **Alternatives Considered**:
  1. Full Enzyme integration immediately
  2. No implementation, just structure
  3. Placeholder with analytical derivatives
- **Selected Approach**: Placeholder using analytical derivatives (2x for x²)
- **Rationale**: Allows test suite validation, CI/CD setup, and documentation without Enzyme dependency
- **Trade-offs**: Tests pass but don't verify actual AD; clear TODO markers for Phase 4
- **Follow-up**: Phase 4 will replace with `#[autodiff_reverse]` macro

### Decision: Feature Flag Architecture

- **Context**: Support builds with and without Enzyme
- **Alternatives Considered**:
  1. Always require Enzyme
  2. Feature flag `enzyme-ad`
  3. Separate crate for Enzyme bindings
- **Selected Approach**: Feature flag `enzyme-ad` in Cargo.toml
- **Rationale**: Allows `--workspace --exclude pricer_kernel` builds; feature enables actual AD
- **Trade-offs**: Conditional compilation complexity
- **Follow-up**: Document feature flag usage in crate docs

### Decision: Nightly Toolchain Pinning

- **Context**: Enzyme requires nightly Rust features
- **Alternatives Considered**:
  1. Use latest nightly
  2. Pin specific version
  3. Range specification
- **Selected Approach**: Pin to `nightly-2025-01-15`
- **Rationale**: Reproducible builds; known-working version with Enzyme
- **Trade-offs**: May miss newer features/fixes; requires periodic updates
- **Follow-up**: Quarterly review of nightly version compatibility

## Risks & Mitigations

- **Risk 1**: Enzyme plugin not available in CI environment → Mitigation: Docker image with pre-installed Enzyme; `--exclude pricer_kernel` for stable jobs
- **Risk 2**: LLVM version mismatch → Mitigation: build.rs validates LLVM 18; clear error messages with installation guidance
- **Risk 3**: Nightly API instability → Mitigation: Pin specific nightly version; comprehensive test suite catches regressions

## References

- [Enzyme AD Project](https://enzyme.mit.edu/) — Official Enzyme documentation
- [Rust autodiff intrinsic](https://doc.rust-lang.org/nightly/std/intrinsics/fn.autodiff.html) — Nightly intrinsic reference
- [Enzyme GitHub](https://github.com/EnzymeAD/Enzyme) — Source and issues
- [Rust-Enzyme Ecosystem](https://enzyme.mit.edu/rust/ecosystem.html) — History and integration approach
- [LLVM Dev Meeting 2024 Talk](https://llvm.org/devmtg/2024-10/slides/techtalk/Drehwald-AutomaticDifferentiation-in-Rust.pdf) — Technical deep-dive
- [GSoC 2025 Enzyme-Rust](https://blog.karanjanthe.me/posts/enzyme-autodiff-rust-gsoc/) — Recent stability work
