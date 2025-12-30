# Research & Design Decisions

## Summary
- **Feature**: `market-data-structures`
- **Discovery Scope**: Extension (adding to existing pricer_core crate)
- **Key Findings**:
  - 既存の `Interpolator<T: Float>` trait パターンを YieldCurve / VolatilitySurface に適用可能
  - `LinearInterpolator`, `BilinearInterpolator` が既に実装済み - 再利用可能
  - `InterpolationError` enum が存在 - MarketDataError として拡張または統合

## Research Log

### Existing Interpolation Infrastructure
- **Context**: 要件3, 6で既存補間器との統合が必要
- **Sources Consulted**: `pricer_core::math::interpolators` module
- **Findings**:
  - `Interpolator<T: Float>` trait: `interpolate(&self, x: T) -> Result<T, InterpolationError>`, `domain(&self) -> (T, T)`
  - `LinearInterpolator<T>`: 1D線形補間、ソート済みデータ点
  - `BilinearInterpolator<T>`: 2Dグリッド補間、Vol Surface に最適
  - Binary search による O(log n) セグメント検索
- **Implications**: YieldCurve は Interpolator を内部で利用、VolSurface は BilinearInterpolator をラップ

### Error Handling Patterns
- **Context**: 要件7で統一エラーハンドリングが必要
- **Sources Consulted**: `pricer_core::types::error` module
- **Findings**:
  - `InterpolationError`: OutOfBounds, InsufficientData, NonMonotonicData, InvalidInput
  - `PricingError`: InvalidInput, NumericalInstability, ModelFailure, UnsupportedInstrument
  - `thiserror` を使用した derive macro パターン
- **Implications**: `MarketDataError` を新規追加し、`InterpolationError` からの変換 trait を実装

### Generic Type System
- **Context**: 要件8でDual64互換性が必要
- **Sources Consulted**: `pricer_core::traits`, `num_traits::Float`
- **Findings**:
  - `pub use num_traits::Float;` で Float trait を re-export
  - 全 interpolator が `T: Float` で generic
  - `smooth_max`, `smooth_abs` が AD 互換の分岐回避を提供
- **Implications**: 全 market data struct を `T: Float` でパラメータ化、分岐は smoothing 関数で回避

## Architecture Pattern Evaluation

| Option | Description | Strengths | Risks / Limitations | Notes |
|--------|-------------|-----------|---------------------|-------|
| Trait + Enum Dispatch | trait定義 + concrete実装 | Enzyme互換、静的dispatch | 新実装追加時にenum拡張必要 | Box<dyn>禁止のため採用 |
| Wrapper over Interpolator | 既存Interpolatorをラップ | コード再利用最大化 | 抽象化レイヤー追加 | InterpolatedCurve, InterpolatedVolSurfaceで採用 |

## Design Decisions

### Decision: YieldCurve Trait API Design
- **Context**: 割引因子、ゼロレート、フォワードレート計算の統一インターフェース
- **Alternatives Considered**:
  1. 全メソッドを trait に定義
  2. discount_factor のみ必須、他はデフォルト実装
- **Selected Approach**: Option 2 - `discount_factor` を必須、`zero_rate` と `forward_rate` はデフォルト実装
- **Rationale**: discount_factor から他の量は導出可能、実装負荷軽減
- **Trade-offs**: デフォルト実装が効率的でない場合あり（override 可能）
- **Follow-up**: 実装時に数値安定性を検証

### Decision: Error Type Integration
- **Context**: MarketDataError を既存エラー体系と統合
- **Alternatives Considered**:
  1. 新規 MarketDataError enum を追加
  2. PricingError を拡張
  3. InterpolationError を直接使用
- **Selected Approach**: Option 1 - 新規 MarketDataError を追加、From trait で InterpolationError から変換
- **Rationale**: 市場データ固有のエラー（InvalidMaturity, InvalidStrike）を明確に区別
- **Trade-offs**: エラー型が増加
- **Follow-up**: PricingError との統合ポイントを文書化

### Decision: InterpolatedCurve Interpolation Strategy
- **Context**: 要件3.4, 3.5で複数補間方式をサポート
- **Alternatives Considered**:
  1. 補間器を型パラメータ化 `InterpolatedCurve<T, I: Interpolator<T>>`
  2. Enum で補間方式を選択
  3. 内部で LinearInterpolator を固定使用
- **Selected Approach**: Option 2 - `InterpolationMethod` enum (Linear, LogLinear, CubicSpline)
- **Rationale**: 実行時に補間方式を切り替え可能、API シンプル
- **Trade-offs**: 静的型安全性がやや低下
- **Follow-up**: CubicSpline は既存実装を確認後に統合

## Risks & Mitigations
- **Risk 1**: 補間器の数値安定性（特に log-linear） — 小さい discount factor に対する ln() のオーバーフローチェック追加
- **Risk 2**: Vol Surface の extrapolation boundary — flat extrapolation をデフォルトとし、設定可能に
- **Risk 3**: Dual64 での AD tape 一貫性 — smoothing 関数を活用、分岐を回避

## References
- [num-traits Float trait](https://docs.rs/num-traits/latest/num_traits/float/trait.Float.html) — Float trait 定義
- [QuantLib YieldTermStructure](https://www.quantlib.org/reference/class_quant_lib_1_1_yield_term_structure.html) — 業界標準インターフェース参考
