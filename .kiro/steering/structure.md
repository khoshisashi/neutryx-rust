# Project Structure

## Organization Philosophy

**Layer-Based Architecture with Dependency Flow**:
Strict bottom-up dependencies (`L1 ← L2 ← L3 → L4`), isolating experimental technology (Enzyme) to Layer 3 while keeping foundation and application stable.

```
L1 (pricer_core)    → No dependencies, pure Rust traits/types
L2 (pricer_models)  → Depends on L1 only
L3 (pricer_kernel)  → Depends on L1+L2, Enzyme isolated here
L4 (pricer_xva)     → Depends on L1+L2+L3, stable Rust
```

## Directory Patterns

### Cargo Workspace Root

**Location**: `/`
**Purpose**: Workspace configuration and top-level metadata
**Key Files**:
- `Cargo.toml` - Workspace members, shared dependencies, release profile
- `rust-toolchain.toml` - Default stable toolchain (nightly pinned for L3)
- `README.md` - User-facing documentation

### Layer 1: Foundation (pricer_core)

**Location**: `crates/pricer_core/src/`
**Purpose**: Math types, traits, smoothing functions, market data abstractions (stable Rust)
**Structure**:
```
math/
├── smoothing.rs    → Smooth approximations (smooth_max, smooth_indicator)
├── interpolators/  → Interpolation methods (linear, bilinear, cubic_spline, monotonic, smooth_interp)
└── solvers/        → Root-finding algorithms (Newton-Raphson, Brent)

traits/     → Priceable, Differentiable, core abstractions
types/
├── dual.rs      → Dual numbers (num-dual) for AD
├── time.rs      → Date, DayCountConvention for financial calculations
├── currency.rs  → ISO 4217 currency codes with metadata
└── error.rs     → Structured error types (PricingError, DateError, etc.)

market_data/
├── curves/     → Yield curve abstractions (YieldCurve trait, FlatCurve, InterpolatedCurve)
├── surfaces/   → Volatility surface abstractions (VolatilitySurface trait, FlatVol, InterpolatedVolSurface)
└── error.rs    → MarketDataError for curve/surface validation
```

**Key Principles**:

- Zero dependencies on other pricer_* crates, pure foundation
- All market data structures generic over `T: Float` for AD compatibility

### Layer 2: Business Logic (pricer_models)

**Location**: `crates/pricer_models/src/`
**Purpose**: Financial instruments and pricing models (stable Rust)
**Structure**:
```
instruments/  → Enum-based instrument definitions (VanillaOption, BarrierOption)
models/       → Stochastic models (GBM, Heston, local vol)
analytical/   → Closed-form solutions (Black-Scholes, barrier formulas)
```

**Key Principle**: Static dispatch via `enum Instrument` for Enzyme compatibility.

### Layer 3: AD Engine (pricer_kernel)

**Location**: `crates/pricer_kernel/src/`
**Purpose**: Monte Carlo + Enzyme AD (nightly Rust, LLVM plugin)
**Structure**:
```
enzyme/       → Enzyme bindings, autodiff macros
mc/           → Monte Carlo kernel, path generation
rng/          → Random number generation (PRNG, QMC sequences)
checkpoint/   → Memory management for path-dependent options
verify/       → Enzyme vs num-dual verification tests
```

**Key Principle**: **Only crate requiring nightly Rust and Enzyme**. Isolated from L1/L2/L4.

**RNG Design**: Zero-allocation batch operations, static dispatch only, Enzyme-compatible. Supports reproducible seeding for deterministic simulations.

### Layer 4: Application (pricer_xva)

**Location**: `crates/pricer_xva/src/`
**Purpose**: Portfolio analytics and XVA calculations (stable Rust)
**Structure**:
```
portfolio/  → Trade structures, netting sets
xva/        → CVA, DVA, FVA calculators
soa/        → Structure of Arrays for vectorization
parallel/   → Rayon-based parallelization
```

**Key Principle**: Consumer of L1+L2+L3, orchestrates portfolio-level computations.

### Infrastructure

**Docker**: `docker/`
- `Dockerfile.stable` - L1/L2/L4 builds (no Enzyme)
- `Dockerfile.nightly` - L3 with Enzyme LLVM plugin

**Scripts**: `scripts/`
- `install_enzyme.sh` - Enzyme installation helper
- `verify_enzyme.sh` - Enzyme verification

**CI/CD**: `.github/workflows/ci.yml`
- Separate jobs for stable (L1/L2/L4) and nightly (L3)

## Naming Conventions

- **Crates**: `pricer_*` prefix, snake_case (`pricer_core`, `pricer_kernel`)
- **Modules**: snake_case (`monte_carlo`, `smoothing`)
- **Traits**: PascalCase (`Priceable`, `Differentiable`)
- **Types**: PascalCase (`DualNumber`, `VanillaOption`)
- **Functions**: snake_case (`smooth_max`, `price_european`)

## Import Organization

**Absolute imports** for cross-crate dependencies:
```rust
use pricer_core::traits::Priceable;
use pricer_models::instruments::Instrument;
```

**Relative imports** within same crate:
```rust
use crate::math::smoothing::smooth_max;
use super::types::DualNumber;
```

**No path aliases** - workspace imports are explicit.

## Code Organization Principles

1. **Bottom-Up Dependencies**: No circular dependencies, L4 never imported by L1/L2/L3
2. **Feature Flag Isolation**: L1 supports both `num-dual-mode` (default) and `enzyme-mode`
3. **Static Dispatch**: Prefer `enum` over `Box<dyn Trait>` for Enzyme optimization
4. **Smooth by Default**: All discontinuous functions have smooth approximations
5. **Test Co-Location**: Unit tests in same file as implementation (`#[cfg(test)]`)

## Phase-Based Development

Current roadmap (see README.md):
- Phase 0: Workspace scaffolding ✅
- Phase 1: L1 foundation (types, traits, smoothing)
- Phase 2: L2 business logic (instruments, models)
- Phase 3: L3 Enzyme integration (AD, verification)
- Phase 4: Advanced MC (checkpointing, path-dependent)
- Phase 5: L4 XVA (CVA, parallelization)
- Phase 6: Production hardening (docs, benchmarks)

---
_Created: 2025-12-29_
_Updated: 2025-12-30_ — Added L1 market_data/ module (yield curves, volatility surfaces), expanded interpolators (bilinear, cubic_spline, monotonic, smooth_interp), expanded solvers (Brent)
_Document patterns, not file trees. New files following patterns shouldn't require updates_
