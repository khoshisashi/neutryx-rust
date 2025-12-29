# Requirements Document

## Project Description (Input)
Phase 3.1a: Random Number Generators - Implement RNG infrastructure in crates/pricer_kernel/src/rng/ with wrapper for rand::Rng, Normal distribution (Box-Muller/Ziggurat), and optional Sobol sequence placeholders for QMC. Must produce f64 vectors efficiently without pricer_core trait dependencies. British English comments throughout.

## Introduction

本仕様は、Layer 3 (pricer_kernel) における乱数生成インフラストラクチャの実装を定義する。Monte Carloシミュレーションに必要な疑似乱数生成器(PRNG)と準モンテカルロ(QMC)シーケンスの基盤を提供し、効率的なf64ベクトル生成を実現する。`pricer_core`への依存を持たず、将来的なEnzyme統合に備えた設計とする。

## Requirements

### Requirement 1: 疑似乱数生成器(PRNG)ラッパー

**Objective:** モンテカルロシミュレーションの開発者として、`rand::Rng`を効率的にラップした乱数生成インターフェースを使用し、パス生成における一貫性と拡張性を確保したい。

#### Acceptance Criteria

1. The RNG module shall provide a wrapper structure around `rand::Rng` trait implementations
2. The RNG module shall support seeding for reproducible simulations
3. When an RNG instance is created with a seed, the RNG module shall produce deterministic sequences across multiple runs
4. The RNG module shall generate `f64` uniform random variates in the range [0, 1)
5. The RNG module shall support efficient batch generation of uniform `f64` values into pre-allocated vectors
6. The RNG module shall use British English spelling in all comments and documentation

### Requirement 2: 正規分布生成 (Box-Muller / Ziggurat法)

**Objective:** モンテカルロシミュレーションの開発者として、高性能な正規分布乱数生成を利用し、Geometric Brownian MotionやHestonモデルのパス生成を効率的に実行したい。

#### Acceptance Criteria

1. The RNG module shall implement normal (Gaussian) distribution sampling using either Box-Muller or Ziggurat algorithm
2. When generating normal variates, the RNG module shall produce values with mean 0.0 and standard deviation 1.0
3. The RNG module shall support batch generation of normal `f64` values into pre-allocated vectors
4. The RNG module shall achieve generation performance suitable for Monte Carlo path simulations (minimum 1M samples/second on standard hardware)
5. If Box-Muller algorithm is used, the RNG module shall handle both generated values from each transformation to minimize waste
6. If Ziggurat algorithm is used, the RNG module shall implement lookup-table-based fast path for majority of samples

### Requirement 3: Sobolシーケンス(QMC)プレースホルダー

**Objective:** 準モンテカルロ手法の将来的な実装に向けて、Sobolシーケンスのインターフェースを事前に定義し、段階的な移行を可能にしたい。

#### Acceptance Criteria

1. The RNG module shall define a placeholder trait or structure for Sobol sequence generation
2. Where Sobol sequence functionality is included, the RNG module shall document intended interface for multi-dimensional low-discrepancy sequences
3. The RNG module shall provide stub implementations or compile-time errors with clear messages indicating future implementation status
4. The RNG module shall separate Sobol placeholder types from active PRNG implementations to prevent accidental usage

### Requirement 4: ベクトル効率性とメモリ管理

**Objective:** 大規模モンテカルロシミュレーションの開発者として、メモリ効率的かつキャッシュフレンドリーな乱数生成を実現し、数百万パスのシミュレーションをスムーズに実行したい。

#### Acceptance Criteria

1. The RNG module shall support in-place generation into mutable `&mut [f64]` slices to avoid allocations
2. When generating vectors of random numbers, the RNG module shall minimize heap allocations by reusing pre-allocated buffers
3. The RNG module shall provide generation methods that accept vector length as a parameter and fill provided slices
4. The RNG module shall document recommended buffer sizes for optimal cache performance in Monte Carlo contexts
5. While generating large batches (>10,000 samples), the RNG module shall maintain consistent performance without degradation

### Requirement 5: 独立性とモジュール境界

**Objective:** Layer 3アーキテクチャの保守担当者として、`pricer_core`への依存を排除し、RNGモジュールの独立性を維持したい。

#### Acceptance Criteria

1. The RNG module shall not import or depend on any `pricer_core` traits or types
2. The RNG module shall depend only on `rand`, `rand_distr`, and standard library crates
3. If future integration with `pricer_core` traits is needed, the RNG module shall define local trait implementations without direct dependency
4. The RNG module shall be testable in isolation without requiring other `pricer_kernel` submodules

### Requirement 6: テスト容易性と検証性

**Objective:** 数値計算ライブラリの品質保証担当者として、乱数生成器の統計的性質と再現性を検証し、本番環境での信頼性を確保したい。

#### Acceptance Criteria

1. The RNG module shall provide test utilities for verifying statistical properties of generated sequences
2. When running unit tests, the RNG module shall verify that seeded generators produce identical sequences
3. The RNG module shall include property-based tests (using `proptest`) for distribution moment verification (mean, variance)
4. The RNG module shall provide example test cases demonstrating correct usage patterns for uniform and normal generation
5. If statistical tests fail (e.g., chi-squared test for uniformity), then the RNG module shall report clear diagnostic information

### Requirement 7: ドキュメントとコメント規約

**Objective:** 国際的な開発チームのメンバーとして、一貫したBritish Englishドキュメントを活用し、コードの可読性とプロフェッショナリズムを維持したい。

#### Acceptance Criteria

1. The RNG module shall use British English spelling in all code comments (e.g., "initialise", "randomise", "behaviour")
2. The RNG module shall include rustdoc documentation for all public functions, structures, and traits
3. When documenting algorithms, the RNG module shall reference academic sources or standard implementations (e.g., "Box-Muller transform as described in Knuth TAOCP Vol. 2")
4. The RNG module shall provide module-level documentation explaining design rationale and usage examples
5. The RNG module shall document performance characteristics and recommended use cases for each generator type

### Requirement 8: Enzyme互換性への準備

**Objective:** Enzyme自動微分の将来的な統合担当者として、RNGモジュールがEnzyme最適化と共存できる設計を事前に確保したい。

#### Acceptance Criteria

1. The RNG module shall avoid dynamic dispatch (`Box<dyn Trait>`) in hot paths to enable Enzyme static analysis
2. The RNG module shall use fixed-size loops where possible instead of while-loops with dynamic conditions
3. Where randomness is consumed in differentiable contexts (future integration), the RNG module shall document separation between seed/generation (non-differentiable) and consumption (potentially differentiable)
4. The RNG module shall document any known Enzyme incompatibilities or limitations in algorithmic choices
