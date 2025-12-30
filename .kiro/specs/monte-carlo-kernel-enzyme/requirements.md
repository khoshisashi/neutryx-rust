# Requirements: Monte Carlo Kernel - Enzyme Integration

## Overview

This specification defines the Monte Carlo pricing kernel within `pricer_kernel` (Layer 3), integrating Enzyme LLVM-level automatic differentiation for high-performance Greeks computation. The kernel provides path generation, payoff evaluation, and gradient computation with explicit activity analysis for AD compatibility.

## Functional Requirements

### 1. Path Generation

#### FR-1.1: Geometric Brownian Motion Path Simulator
**When** the kernel is configured with GBM parameters (spot, drift, volatility, time steps),
**Then** the system shall generate price paths using the Euler-Maruyama discretisation scheme,
**And** all operations shall be differentiable through Enzyme AD.

**Acceptance Criteria**:
- Path generation produces `n_paths × n_steps` matrix of prices
- Drift and volatility are explicitly marked with `Activity::Dual` for forward mode
- Spot price parameter supports gradient computation
- Time step discretisation error scales as O(sqrt(dt))

#### FR-1.2: Workspace Buffer Management
**When** generating Monte Carlo paths,
**Then** the system shall pre-allocate workspace buffers outside the simulation loop,
**And** Vec allocations shall be hoisted to enable Enzyme optimisation.

**Acceptance Criteria**:
- `PathWorkspace` struct holds pre-allocated `random_normals`, `price_paths`, `payoff_values`
- No dynamic allocation within the inner simulation loop
- Buffer reuse across multiple pricing calls via workspace reset
- Memory layout supports SIMD vectorisation (contiguous f64 arrays)

#### FR-1.3: Seeded Reproducibility
**When** a seed value is provided to the Monte Carlo engine,
**Then** the system shall produce identical paths across executions,
**And** gradient computation shall be deterministically reproducible.

**Acceptance Criteria**:
- `MonteCarloPricer::with_seed(u64)` produces reproducible results
- Parallel path generation maintains determinism via seed derivation
- Test cases verify exact numeric reproducibility

### 2. Payoff Evaluation

#### FR-2.1: European Option Payoff
**When** evaluating a European call or put option,
**Then** the system shall compute the smooth payoff using the terminal path value,
**And** the payoff function shall be differentiable for Enzyme AD.

**Acceptance Criteria**:
- `PayoffKernel::european_call(terminal_price, strike, smoothing)` returns smooth payoff
- Smooth approximation: `soft_plus(x, epsilon)` instead of `max(x, 0)`
- Configurable `smoothing_epsilon` parameter (default: 1e-4)
- Gradient of payoff with respect to spot computes correctly

#### FR-2.2: Path-Dependent Payoff Interface
**When** evaluating Asian or barrier options,
**Then** the system shall accept the full price path for payoff computation,
**And** the interface shall support future checkpoint integration.

**Acceptance Criteria**:
- `PayoffKernel::path_dependent(paths: &[f64], params: &PayoffParams)` interface
- Asian average computed via running sum (AD-compatible)
- Barrier detection uses smooth indicator functions
- Memory checkpoint hooks defined for adjoint mode

### 3. Enzyme AD Integration

#### FR-3.1: Activity Annotation Types
**When** defining autodiff-compatible functions,
**Then** the system shall provide explicit activity annotations for all parameters,
**And** the annotations shall follow Enzyme's forward/reverse mode conventions.

**Acceptance Criteria**:
- `Activity` enum with `Const`, `Dual`, `Active`, `Duplicated`, `DuplicatedOnly`
- Each kernel function documents activity requirements in doc comments
- Type-safe wrappers prevent incorrect activity combinations
- Forward mode uses `Dual` for inputs requiring tangent propagation

#### FR-3.2: Safe Autodiff Wrappers
**When** calling Enzyme autodiff intrinsics,
**Then** the system shall encapsulate unsafe operations in safe wrapper functions,
**And** the wrappers shall validate inputs before invoking Enzyme.

**Acceptance Criteria**:
- `gradient_forward<F>(f, x, dx)` returns primal and tangent values
- `gradient_reverse<F>(f, x)` returns primal and adjoint gradient
- Unsafe Enzyme calls isolated to single module (`enzyme/intrinsics.rs`)
- Safe API exposed via `enzyme/mod.rs` public functions

#### FR-3.3: Gradient Computation for Monte Carlo
**When** computing Greeks via Monte Carlo with AD,
**Then** the system shall support forward mode for Delta/Gamma,
**And** reverse mode for Vega/Rho with appropriate checkpointing.

**Acceptance Criteria**:
- `mc_delta(pricer, instrument, market)` computes spot sensitivity
- `mc_vega(pricer, instrument, market)` computes volatility sensitivity
- Forward mode used for single-input sensitivities (Delta)
- Reverse mode used for multi-input sensitivities (Vega across surface)
- Workspace buffers support adjoint accumulation

### 4. Monte Carlo Pricing Engine

#### FR-4.1: Pricer Configuration
**When** creating a Monte Carlo pricer,
**Then** the system shall accept configuration for paths, steps, and AD mode,
**And** the configuration shall be validated at construction time.

**Acceptance Criteria**:
- `MonteCarloConfig { n_paths, n_steps, ad_mode, seed }` structure
- `ad_mode` enum: `NoAD`, `ForwardMode`, `ReverseMode`
- Validation: `n_paths > 0`, `n_steps > 0`, `n_paths <= 10_000_000`
- Builder pattern: `MonteCarloConfig::builder().paths(10000).steps(100).build()`

#### FR-4.2: Pricing with Greeks
**When** pricing an instrument with Greeks enabled,
**Then** the system shall return both price and requested sensitivities,
**And** the computation shall use Enzyme AD for gradient calculation.

**Acceptance Criteria**:
- `PricingResult { price, delta, gamma, vega, theta, rho }` structure
- Optional fields: `Option<f64>` for each Greek
- `pricer.price_with_greeks(instrument, market, greeks: &[Greek])` API
- Greeks enum: `Greek::Delta`, `Greek::Gamma`, `Greek::Vega`, etc.

#### FR-4.3: Convergence Monitoring
**When** running Monte Carlo simulation,
**Then** the system shall track standard error and convergence metrics,
**And** provide confidence intervals for the price estimate.

**Acceptance Criteria**:
- `MonteCarloStats { mean, std_error, n_paths, elapsed_time }` structure
- Standard error: `std_dev / sqrt(n_paths)`
- 95% confidence interval computed from standard error
- Optional early termination when `std_error < target_error`

## Non-Functional Requirements

### NFR-1: Performance

#### NFR-1.1: Path Generation Throughput
**Given** the kernel operates on modern CPU hardware,
**When** generating Monte Carlo paths,
**Then** the throughput shall exceed 1 million path-steps per second per core.

**Acceptance Criteria**:
- Benchmark: 10,000 paths × 252 steps < 2.5 seconds single-threaded
- Memory bandwidth: path data fits in L3 cache for typical simulations
- SIMD utilisation via contiguous memory layout

#### NFR-1.2: AD Overhead
**Given** Enzyme AD is enabled,
**When** computing Greeks alongside price,
**Then** the overhead shall not exceed 3× the primal computation cost.

**Acceptance Criteria**:
- Forward mode Delta: < 2× primal cost
- Reverse mode Vega: < 3× primal cost
- Benchmark comparison: AD vs finite difference runtime

### NFR-2: Memory Efficiency

#### NFR-2.1: Workspace Reuse
**Given** multiple pricing calls are made,
**When** using the same pricer instance,
**Then** workspace buffers shall be reused without reallocation.

**Acceptance Criteria**:
- `PathWorkspace::reset()` clears state without deallocation
- Capacity growth amortised over multiple calls
- Peak memory: O(n_paths × n_steps × sizeof(f64))

#### NFR-2.2: Allocation-Free Inner Loop
**Given** the simulation inner loop executes,
**When** generating paths and computing payoffs,
**Then** no heap allocations shall occur within the loop.

**Acceptance Criteria**:
- `#[cfg(debug_assertions)]` allocation counter validates zero allocations
- Vec operations use `set_len` not `push` for pre-allocated buffers
- All closures capture by reference, not by value

### NFR-3: Correctness

#### NFR-3.1: Numerical Stability
**Given** extreme market conditions (high vol, long tenor),
**When** generating paths,
**Then** numerical stability shall be maintained.

**Acceptance Criteria**:
- Log-space computation for GBM: `S(t+dt) = S(t) * exp(...)`
- No overflow/underflow for volatility < 500%, tenor < 30 years
- NaN/Inf detection with meaningful error messages

#### NFR-3.2: Gradient Accuracy
**Given** Enzyme computes gradients,
**When** compared to finite difference approximations,
**Then** relative error shall be < 1e-4 for typical parameters.

**Acceptance Criteria**:
- Test suite compares AD gradients vs central differences (h=1e-6)
- Relative error metric: `|ad - fd| / max(|ad|, |fd|, 1e-8)`
- Regression tests for known analytical gradient cases

## Constraints

### C-1: Layer 3 Isolation
The Monte Carlo kernel shall have zero dependencies on `pricer_core` or `pricer_models` in Phase 3.2. Integration with L1/L2 types deferred to Phase 4.

### C-2: Nightly Rust Features
The implementation shall use `nightly-2025-01-15` toolchain features:
- `#![feature(autodiff)]` when Enzyme is enabled
- Conditional compilation via `#[cfg(feature = "enzyme-ad")]`

### C-3: Static Dispatch
All kernel functions shall use concrete types (no `dyn Trait`). Enum-based dispatch permitted for payoff types.

### C-4: British English
All documentation, comments, and user-facing messages shall use British English spelling conventions.

## Dependencies

| Dependency | Type | Purpose |
|------------|------|---------|
| RNG Infrastructure (Phase 3.1a) | Internal | Random number generation for paths |
| Enzyme Module (Phase 3.0) | Internal | Autodiff bindings |
| llvm-sys | External | LLVM 18 bindings for Enzyme |
| approx | Dev | Floating-point comparison in tests |

## Glossary

| Term | Definition |
|------|------------|
| Activity | Enzyme parameter annotation specifying differentiation mode |
| Adjoint | Reverse-mode AD gradient accumulation |
| Checkpointing | Memory optimisation for reverse-mode AD |
| GBM | Geometric Brownian Motion stochastic process |
| Primal | Original (non-differentiated) computation |
| Tangent | Forward-mode AD derivative propagation |
| Workspace | Pre-allocated buffer for simulation data |
