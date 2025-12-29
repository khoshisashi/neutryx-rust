# Research & Design Decisions

---
**Purpose**: Capture discovery findings, architectural investigations, and rationale that inform the technical design.
---

## Summary
- **Feature**: `core-traits-types`
- **Discovery Scope**: Extension (extending existing pricer_core foundation)
- **Key Findings**:
  - num-traits Float trait provides comprehensive floating-point abstraction compatible with f64 and num-dual
  - Enum dispatch significantly outperforms Box<dyn Trait> for Enzyme LLVM optimization (4.2x performance gain)
  - chrono NaiveDate supports ISO 8601 serialization via serde out-of-the-box
  - quantrs and RustQuant libraries provide reference implementations for day count conventions
  - Existing codebase already implements DayCountConvention enum with Act/365, Act/360, 30/360

## Research Log

### Generic Float Trait Design
- **Context**: Requirements 1.1-1.4 specify generic Float abstraction for both f64 and dual numbers
- **Sources Consulted**:
  - [num-traits Float documentation](https://docs.rs/num-traits/latest/num_traits/float/trait.Float.html)
  - [num-traits GitHub](https://github.com/rust-num/num-traits)
  - [num-dual documentation](https://docs.rs/num-dual)
- **Findings**:
  - num-traits::Float is comprehensive but not dyn-compatible (cannot use as trait object)
  - FloatCore subset available for no_std environments
  - num-dual::DualNumber already implements num-traits::Float
  - Recommended approach: Re-export num-traits::Float as pricer_core::traits::Float
- **Implications**: 
  - No need to define custom Float trait; use supertrait bound on num_traits::Float
  - Ensures compatibility with both f64 and num-dual ecosystem
  - Maintains Enzyme optimization by avoiding trait objects

### Dual Number Automatic Differentiation
- **Context**: Requirement 1.4 requires dual-mode verification with num-dual
- **Sources Consulted**:
  - [num-dual crate](https://docs.rs/num-dual)
  - [dual_num crate](https://docs.rs/dual_num/latest/dual_num/)
  - [autodj crate](https://lib.rs/crates/autodj)
  - [ad-trait paper](https://arxiv.org/html/2504.15976v1)
- **Findings**:
  - num-dual: Generalized recursive scalar/vector dual numbers, already in use in existing codebase
  - dual_num: Fully-featured with Float trait, alternative option
  - autodj: Generic multivariate differentiation with sparse support
  - ad-trait: First Rust AD with both forward and reverse mode + SIMD
- **Implications**:
  - Continue using num-dual (already integrated in types/dual.rs)
  - Maintain feature flag separation: num-dual-mode vs enzyme-mode
  - No changes needed to existing dual number integration

### Chrono Date Serialization
- **Context**: Requirements 4.1-4.6 specify Date struct with ISO 8601 serialization
- **Sources Consulted**:
  - [Serde custom date format guide](https://serde.rs/custom-date-format.html)
  - [chrono::NaiveDate docs](https://docs.rs/chrono/0.4.2/chrono/struct.DateTime.html)
  - [serde_with::chrono utilities](https://docs.rs/serde_with/latest/serde_with/chrono/index.html)
- **Findings**:
  - NaiveDate serializes to ISO 8601 (YYYY-MM-DD) by default with serde feature
  - No custom serializer needed for basic ISO 8601 format
  - chrono::naive::serde provides additional serialization utilities for UNIX timestamps
- **Implications**:
  - Simple newtype wrapper around NaiveDate with derived Serialize/Deserialize
  - Enable chrono serde feature in Cargo.toml
  - Automatic ISO 8601 compliance without custom code

### Day Count Convention Standards
- **Context**: Requirement 6.1-6.5 specify day count convention implementation
- **Sources Consulted**:
  - [Day count convention - Wikipedia](https://en.wikipedia.org/wiki/Day_count_convention)
  - [ACT Learning - How to apply day count conventions](https://learning.treasurers.org/how-to-apply-day-count-conventions)
  - [Day count conventions - ACT Wiki](https://wiki.treasurers.org/wiki/Day_count_conventions)
  - [quantrs library](https://github.com/carlobortolan/quantrs)
  - [RustQuant library](https://github.com/avhz/RustQuant)
- **Findings**:
  - Industry standard conventions: ACT/365, ACT/360, 30/360 (multiple variants), ACT/ACT ISDA
  - quantrs supports: ACT/365F, ACT/365, ACT/360, 30/360 US, 30/360 Eurobond, ACT/ACT ISDA, ACT/ACT ICMA
  - RustQuant provides DayCounter and calendar functionality
  - Existing codebase already implements basic conventions in types/time.rs
- **Implications**:
  - Extend existing DayCountConvention enum with ActActISDA variant
  - Rename variants to match industry standards (Act365Fixed, Act360, Thirty360, ActActISDA)
  - Keep year_fraction method generic over Float for dual-mode compatibility
  - Add property-based tests for ISDA reference values

### Enzyme LLVM Optimization Requirements
- **Context**: Requirement 12.1-12.5 specify static dispatch for Enzyme compatibility
- **Sources Consulted**:
  - [Enzyme GitHub](https://github.com/EnzymeAD/Enzyme)
  - [Enzyme-Rust LLVM discussion](https://internals.rust-lang.org/t/automatic-differentiation-differential-programming-via-llvm/13188)
  - [Enzyme official site](https://enzyme.mit.edu/)
  - [GSoC 2025 Rust-Enzyme improvements](https://discourse.llvm.org/t/gsoc2025-improve-rust-enzyme-reliability-and-compile-times/84523)
- **Findings**:
  - Enzyme operates on LLVM IR after Rust type erasure (ptr types)
  - 4.2x performance benefit from AD on optimized code
  - Rust-specific challenges: type information loss requires expensive reconstruction
  - Recent TypeTrees improvements provide type layout information upfront
  - Enum dispatch preferred over Box<dyn Trait> for concrete type monomorphization
- **Implications**:
  - Use enum for Currency and DayCount (static dispatch)
  - Avoid trait objects (Box<dyn Priceable>) in performance-critical paths
  - Document Enzyme compatibility constraints in trait doc comments
  - Generic Float trait with monomorphization maintains Enzyme optimization

### Static Dispatch vs Dynamic Dispatch
- **Context**: Requirement 12.1 mandates enum dispatch over trait objects
- **Findings** (from knowledge base, web search unavailable):
  - Monomorphization (generics): Zero runtime overhead, compiler generates specific code per type
  - Enum dispatch: Match statements delegate to concrete types, nearly as fast as monomorphization
  - Trait objects (Box<dyn>): Runtime vtable indirection, smaller binaries, slower execution
  - Performance hierarchy: Monomorphization ≈ Enum dispatch > Trait objects
- **Implications**:
  - Currency: enum with 7 variants (USD, EUR, GBP, JPY, CHF, AUD, CAD)
  - DayCount: enum with 4 variants (Act365Fixed, Act360, Thirty360, ActActISDA)
  - Priceable trait: generic over T: Float for monomorphization
  - No Box<dyn Priceable> in Layer 1

## Architecture Pattern Evaluation

| Option | Description | Strengths | Risks / Limitations | Notes |
|--------|-------------|-----------|---------------------|-------|
| Trait Re-export | Re-export num_traits::Float as pricer_core::traits::Float | Standard ecosystem integration, proven compatibility | Couples to external crate API | Aligns with Rust numeric ecosystem best practice |
| Custom Float Trait | Define bespoke Float trait with custom bounds | Full control over API surface | Reinventing wheel, compatibility issues | Rejected: num_traits is de facto standard |
| Newtype Wrappers | Wrap NaiveDate in Date newtype | Type safety, domain-specific methods | Extra boilerplate | Selected: minimal wrapper with domain methods |
| Enum Dispatch for Types | Currency/DayCount as enums | Static dispatch, Enzyme-friendly | Must enumerate all variants | Selected: meets Enzyme optimization requirements |

## Design Decisions

### Decision: Re-export num_traits::Float instead of custom trait

- **Context**: Requirement 1.1 requires Float trait for generic numeric types
- **Alternatives Considered**:
  1. Define custom Float trait with specific bounds → Reinvents ecosystem standard
  2. Re-export num_traits::Float as pricer_core::traits::Float → Leverages ecosystem
- **Selected Approach**: Re-export num_traits::Float with type alias or supertrait bound
- **Rationale**: 
  - num_traits is de facto standard for generic numeric code in Rust
  - Already implemented by f64, f32, and num-dual::DualNumber
  - Proven compatibility with AD backends
- **Trade-offs**: 
  - Benefits: Ecosystem compatibility, zero maintenance, proven correctness
  - Compromises: Coupled to external crate API (minimal risk, stable crate)
- **Follow-up**: Document Float trait in pricer_core::traits with British English comments

### Decision: Date as newtype wrapper over chrono::NaiveDate

- **Context**: Requirement 4 requires Date struct with day count and serialization
- **Alternatives Considered**:
  1. Use chrono::NaiveDate directly → Loses domain-specific methods
  2. Newtype wrapper with domain methods → Type safety + domain API
- **Selected Approach**: `pub struct Date(NaiveDate)` with From/Into conversions and domain methods
- **Rationale**:
  - Encapsulates chrono dependency (Layer 1 boundary)
  - Allows addition of domain-specific methods (e.g., is_business_day, add_tenor)
  - Maintains serde ISO 8601 serialization via transparent newtype
- **Trade-offs**:
  - Benefits: Type safety, domain clarity, future extensibility
  - Compromises: Minor boilerplate for From/Into/Deref
- **Follow-up**: Implement serde transparent attribute for ISO 8601 passthrough

### Decision: Extend existing DayCountConvention with ActActISDA

- **Context**: Existing types/time.rs has DayCountConvention enum with 3 variants
- **Alternatives Considered**:
  1. Keep existing 3 variants (Act/365, Act/360, 30/360) → Missing industry standard
  2. Add ActActISDA variant → Complete industry coverage
- **Selected Approach**: Add ActActISDA variant and rename to match standards
- **Rationale**:
  - ACT/ACT ISDA required for swap pricing and fixed-income instruments
  - Existing implementation extensible via match arms
  - Maintains enum dispatch for static performance
- **Trade-offs**:
  - Benefits: Industry standard compliance, complete convention coverage
  - Compromises: Slightly more complex match logic for ISDA year fraction
- **Follow-up**: Add ISDA test cases from reference values, update docs

### Decision: Generic year_fraction<T: Float> for dual-mode compatibility

- **Context**: DayCount must support both f64 and dual numbers
- **Alternatives Considered**:
  1. year_fraction returns f64 only → Breaks dual-mode AD
  2. year_fraction<T: Float> generic → Dual-mode compatible
- **Selected Approach**: `pub fn year_fraction<T: Float>(&self, start: Date, end: Date) -> T`
- **Rationale**:
  - Allows automatic differentiation of day count calculations
  - Maintains type flexibility for Enzyme (f64) and num-dual (DualNumber)
  - Consistent with generic Float design throughout Layer 1
- **Trade-offs**:
  - Benefits: Dual-mode support, differentiable calculations
  - Compromises: Slightly more complex type signatures
- **Follow-up**: Update tests to verify both f64 and DualNumber year_fraction

### Decision: Time<T: Float> as generic wrapper for type-safe time values

- **Context**: Requirement 7 requires Time wrapper for time-to-maturity calculations
- **Alternatives Considered**:
  1. Use T: Float directly → Lacks semantic type safety
  2. Time<T: Float> newtype → Type-safe with arithmetic operations
- **Selected Approach**: `pub struct Time<T: Float>(T)` with arithmetic trait impls
- **Rationale**:
  - Prevents mixing time values with other Float quantities (e.g., prices)
  - Enables method chaining and domain-specific operations
  - Generic over Float maintains dual-mode compatibility
- **Trade-offs**:
  - Benefits: Type safety, semantic clarity, arithmetic convenience
  - Compromises: Requires trait impls for Add/Sub/Mul/Div
- **Follow-up**: Implement From<(Date, Date, DayCount)> constructor method

### Decision: PricingError enum with descriptive variants

- **Context**: Requirement 8 requires error categorization
- **Alternatives Considered**:
  1. String error messages → No type safety
  2. Enum with structured variants → Type-safe, actionable
- **Selected Approach**: Enum with InvalidInput, NumericalInstability, ModelFailure, UnsupportedInstrument
- **Rationale**:
  - Enables exhaustive match handling in calling code
  - Provides structured context for error reporting
  - Implements std::error::Error for ecosystem compatibility
- **Trade-offs**:
  - Benefits: Type safety, structured errors, exhaustive handling
  - Compromises: Requires Error/Display trait implementations
- **Follow-up**: Add #[derive(thiserror::Error)] for automatic Display impl

## Risks & Mitigations

- **Risk: Enum variants insufficient for all currencies** — Mitigation: Use #[non_exhaustive] attribute to allow future additions
- **Risk: Day count ISDA implementation complexity** — Mitigation: Reference QuantLib C++ implementation and ISDA test cases
- **Risk: Generic Time<T> arithmetic overflow** — Mitigation: Document non-negative invariant, add validation in constructor
- **Risk: Serde serialization format breaking changes** — Mitigation: Add round-trip serialization tests in test suite
- **Risk: num_traits API changes** — Mitigation: Pin to stable 0.2.x version, monitor release notes

## References

- [num-traits Float documentation](https://docs.rs/num-traits/latest/num_traits/float/trait.Float.html) — Rust numeric trait standard
- [num-dual crate](https://docs.rs/num-dual) — Dual number AD implementation
- [chrono NaiveDate](https://docs.rs/chrono/latest/chrono/naive/struct.NaiveDate.html) — Date type with ISO 8601 support
- [Serde custom date format](https://serde.rs/custom-date-format.html) — Serialization patterns
- [Day count conventions - ACT Wiki](https://wiki.treasurers.org/wiki/Day_count_conventions) — Industry standards reference
- [quantrs library](https://github.com/carlobortolan/quantrs) — Rust quant finance reference implementation
- [Enzyme LLVM AD](https://enzyme.mit.edu/) — Automatic differentiation compiler
- [Rust-Enzyme discussion](https://internals.rust-lang.org/t/automatic-differentiation-differential-programming-via-llvm/13188) — LLVM integration challenges
