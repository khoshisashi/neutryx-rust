# Requirements Document

## Project Description (Input)
Phase 1: Core Traits & Types - Date, Currency, and Pricing interfaces with generic Float support, enum dispatch for Currency and DayCount, chrono-based Date struct with standard serialisation, and comprehensive trait definitions for priceable instruments

## イントロダクション

本仕様は、Layer 1 (pricer_core) の基盤となる型システムとトレイト定義を確立する。ジェネリックな浮動小数点型対応、enum dispatchによる静的ディスパッチ、chrono基盤の日付型、標準的なシリアライゼーション対応を含む、価格計算可能な金融商品のための包括的なインターフェースを提供する。

## Requirements

### Requirement 1: ジェネリック浮動小数点トレイト

**Objective:** As a quantitative developer, I want generic Float abstraction, so that pricing models can support both f64 (production) and dual numbers (AD mode) without code duplication.

#### Acceptance Criteria
1. The pricer_core module shall define a `Float` trait that is generic over numeric types supporting arithmetic operations, comparisons, and mathematical functions
2. When `Float` trait is implemented, the pricer_core module shall require `num_traits::Float` as a supertrait bound
3. The pricer_core module shall ensure `Float` trait supports `Copy`, `Clone`, `Debug`, and `PartialOrd` for all implementing types
4. Where dual-mode verification is enabled, the pricer_core module shall allow `Float` to be satisfied by both `f64` and `num_dual::DualNumber` types

### Requirement 2: 価格計算可能トレイト (Priceable)

**Objective:** As a pricing engine developer, I want a generic `Priceable` trait, so that all instruments can be priced uniformly with type-parameterised floating-point precision.

#### Acceptance Criteria
1. The pricer_core module shall define a `Priceable<T: Float>` trait with a `price` method returning `T`
2. When `Priceable::price` is called, the implementing type shall accept market data and model parameters as input
3. The pricer_core module shall document `Priceable` trait with British English doc comments (e.g., "behaviour", "parameterise")
4. If instrument pricing fails due to invalid inputs, then the implementing type shall return `Result<T, PricingError>`
5. The pricer_core module shall ensure `Priceable` trait is object-safe for potential future trait object usage (no associated types with generic parameters outside the trait)

### Requirement 3: 微分可能トレイト (Differentiable)

**Objective:** As an AD developer, I want a `Differentiable` trait marker, so that Enzyme and num-dual backends can identify smooth, differentiable computations.

#### Acceptance Criteria
1. The pricer_core module shall define a `Differentiable` marker trait for types that guarantee smoothness
2. When a type implements `Differentiable`, the pricer_core module shall require all operations to use smooth approximations (no `if` branches on Float values)
3. The pricer_core module shall document differentiability constraints in British English doc comments
4. The pricer_core module shall provide a `smoothing_epsilon: T` parameter for configurable approximation precision

### Requirement 4: 日付型 (Date)

**Objective:** As a fixed-income developer, I want a `Date` struct wrapping `chrono::NaiveDate`, so that day count calculations and standard serialisation are supported.

#### Acceptance Criteria
1. The pricer_core module shall define a `Date` struct that wraps `chrono::NaiveDate`
2. When `Date` is serialised, the pricer_core module shall use ISO 8601 format (YYYY-MM-DD)
3. The pricer_core module shall implement `From<chrono::NaiveDate>` and `Into<chrono::NaiveDate>` for `Date`
4. When day count calculations are performed, the pricer_core module shall provide methods for year fractions (ACT/365, ACT/360, 30/360)
5. The pricer_core module shall implement `Copy`, `Clone`, `Debug`, `PartialOrd`, `Ord`, `PartialEq`, `Eq` for `Date`
6. The pricer_core module shall support `serde::Serialize` and `serde::Deserialize` for `Date` with standard ISO 8601 format

### Requirement 5: 通貨型 (Currency) - Enum Dispatch

**Objective:** As a multi-currency pricing developer, I want a `Currency` enum, so that static dispatch is used instead of `Box<dyn>` for Enzyme optimisation.

#### Acceptance Criteria
1. The pricer_core module shall define a `Currency` enum with variants for major currencies (USD, EUR, GBP, JPY, CHF, AUD, CAD)
2. When currency conversion is required, the pricer_core module shall provide methods accepting exchange rates as generic `Float` parameters
3. The pricer_core module shall implement `Copy`, `Clone`, `Debug`, `PartialEq`, `Eq`, `Hash` for `Currency`
4. The pricer_core module shall support `serde::Serialize` and `serde::Deserialize` for `Currency`
5. The pricer_core module shall document `Currency` with British English doc comments

### Requirement 6: Day Count Convention (DayCount) - Enum Dispatch

**Objective:** As a fixed-income developer, I want a `DayCount` enum, so that year fraction calculations use static dispatch for performance.

#### Acceptance Criteria
1. The pricer_core module shall define a `DayCount` enum with variants: `Act365Fixed`, `Act360`, `Thirty360`, `ActActISDA`
2. When year fraction is calculated, the pricer_core module shall provide a `year_fraction<T: Float>(start: Date, end: Date) -> T` method
3. The pricer_core module shall implement `Copy`, `Clone`, `Debug`, `PartialEq`, `Eq` for `DayCount`
4. The pricer_core module shall ensure year fraction calculations are deterministic and match industry-standard conventions
5. The pricer_core module shall document `DayCount` with British English doc comments

### Requirement 7: Time型 (Time wrapper)

**Objective:** As a pricing model developer, I want a `Time<T: Float>` struct, so that time-to-maturity calculations are type-safe and generic.

#### Acceptance Criteria
1. The pricer_core module shall define a `Time<T: Float>` struct wrapping a generic floating-point value representing years
2. When `Time` is constructed, the pricer_core module shall enforce non-negative values via validation or type-level guarantees
3. The pricer_core module shall implement arithmetic operations (`Add`, `Sub`, `Mul`, `Div`) for `Time<T>`
4. The pricer_core module shall provide conversion methods from `Date` pairs using `DayCount` conventions
5. The pricer_core module shall implement `Copy`, `Clone`, `Debug`, `PartialOrd` for `Time<T>` where `T: Float`

### Requirement 8: 価格計算エラー型 (PricingError)

**Objective:** As a library user, I want a `PricingError` enum, so that pricing failures are categorised and actionable.

#### Acceptance Criteria
1. The pricer_core module shall define a `PricingError` enum with variants: `InvalidInput`, `NumericalInstability`, `ModelFailure`, `UnsupportedInstrument`
2. When a pricing error occurs, the pricer_core module shall provide descriptive error messages with context
3. The pricer_core module shall implement `std::error::Error`, `Debug`, `Display` for `PricingError`
4. The pricer_core module shall document error variants with British English doc comments

### Requirement 9: モジュール構造とエクスポート

**Objective:** As a library maintainer, I want a clear module structure in `pricer_core/src/`, so that types and traits are logically organised.

#### Acceptance Criteria
1. The pricer_core module shall organise code into `types/` and `traits/` subdirectories
2. When users import from pricer_core, the module shall re-export commonly used types and traits from `lib.rs`
3. The pricer_core module shall place `Date`, `Currency`, `DayCount`, `Time` in `types/` module
4. The pricer_core module shall place `Priceable`, `Differentiable`, `Float` in `traits/` module
5. The pricer_core module shall provide a `prelude` module for convenient wildcard imports

### Requirement 10: テストとドキュメント

**Objective:** As a library user, I want comprehensive tests and British English documentation, so that I can trust the foundation layer.

#### Acceptance Criteria
1. The pricer_core module shall provide unit tests for all public types and traits
2. When day count calculations are tested, the pricer_core module shall validate against known reference values (e.g., ISDA test cases)
3. The pricer_core module shall include property-based tests using `proptest` for mathematical invariants
4. The pricer_core module shall document all public items with British English doc comments including examples
5. When `cargo test --package pricer_core` is run, all tests shall pass without warnings

### Requirement 11: Serdeシリアライゼーション対応

**Objective:** As a data pipeline developer, I want serde support for all core types, so that pricing configurations can be serialised to JSON/TOML.

#### Acceptance Criteria
1. The pricer_core module shall enable `serde` feature flag by default
2. When `Date` is serialised to JSON, the output shall be ISO 8601 string format (e.g., "2025-12-29")
3. When `Currency` and `DayCount` are serialised, the pricer_core module shall use string representations (e.g., "USD", "Act365Fixed")
4. The pricer_core module shall provide round-trip serialisation tests for all types
5. The pricer_core module shall document serialisation format in British English doc comments

### Requirement 12: 型安全性とEnzyme互換性

**Objective:** As an Enzyme AD developer, I want static dispatch and no trait objects, so that LLVM optimisation is maximised.

#### Acceptance Criteria
1. The pricer_core module shall use `enum` dispatch for `Currency` and `DayCount` instead of `Box<dyn Trait>`
2. When generic `Float` trait is used, the pricer_core module shall ensure concrete type monomorphisation for Enzyme
3. The pricer_core module shall avoid dynamic dispatch in performance-critical paths
4. The pricer_core module shall document Enzyme compatibility constraints in British English doc comments
5. Where trait objects are unavoidable, the pricer_core module shall document the performance trade-off
