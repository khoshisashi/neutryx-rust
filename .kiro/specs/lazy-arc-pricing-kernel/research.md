# Research & Design Decisions

## Summary
- **Feature**: `lazy-arc-pricing-kernel`
- **Discovery Scope**: Extension（既存A-I-P-Sアーキテクチャへの機能追加）
- **Key Findings**:
  - `Currency` enum が `pricer_core` に既存（`Hash`, `Eq` 実装済み）
  - `CurveEnum<T: Float>` パターンが `pricer_core` に既存（静的ディスパッチ）
  - `StochasticModelEnum<T: Float>` パターンが `pricer_models` に既存
  - `RwLock`/`Arc` パターンが `pricer_pricing/mc/thread_local.rs` に既存

## Research Log

### 既存型システムの調査
- **Context**: 要件1.7で定義された`Currency`型の既存実装を確認
- **Sources Consulted**: `crates/pricer_core/src/types/currency.rs`
- **Findings**:
  - `Currency` enum（USD, EUR, GBP, JPY, CHF）が既に実装済み
  - `Copy, Clone, Debug, PartialEq, Eq, Hash` traits を derive
  - `serde` feature でシリアライズ対応
- **Implications**: 新規 `Currency` 定義は不要。既存型を再利用可能

### CurveEnum パターンの調査
- **Context**: 静的ディスパッチによるカーブ実装パターンを確認
- **Sources Consulted**: `crates/pricer_core/src/market_data/curves/curve_enum.rs`
- **Findings**:
  - `CurveEnum<T: Float>` として Flat/Interpolated バリアントをラップ
  - `YieldCurve<T>` trait を実装し、`discount_factor`, `zero_rate`, `forward_rate` を提供
  - 完全な静的ディスパッチ（match式による分岐）
- **Implications**: このデモでは `f64` 固定の簡易版を使用。本番統合時は既存 `CurveEnum<T>` を活用

### RwLock/Arc キャッシュパターンの調査
- **Context**: スレッドセーフキャッシュの既存実装を確認
- **Sources Consulted**: `crates/pricer_pricing/src/mc/thread_local.rs`
- **Findings**:
  - `ThreadLocalWorkspacePool` で類似のキャッシュパターンを使用
  - `Arc` によるゼロコピー共有が実践済み
- **Implications**: `MarketProvider` は同様のパターンを適用

## Architecture Pattern Evaluation

| Option | Description | Strengths | Risks / Limitations | Notes |
|--------|-------------|-----------|---------------------|-------|
| Pull-then-Push | 準備フェーズで遅延取得、実行フェーズでコンテキストをカーネルに渡す | 関心の分離、キャッシュ効率 | 2フェーズ管理の複雑さ | 採用：要件に合致 |
| Full Eager | 全マーケットデータを事前構築 | シンプル | メモリ無駄、不要な計算 | 不採用 |
| Full Lazy | カーネル内でオンデマンド取得 | 柔軟 | ホットループにHashMap検索が入る | 不採用：要件3.6違反 |

## Design Decisions

### Decision: 簡易型 vs ジェネリック型
- **Context**: 既存の `CurveEnum<T: Float>` は AD 互換だが、このデモは概念実証が目的
- **Alternatives Considered**:
  1. 既存 `CurveEnum<T>` を直接使用
  2. `f64` 固定の簡易版を新規作成
- **Selected Approach**: Option 2（簡易版）
- **Rationale**:
  - デモ目的のため最小限の複雑さを優先
  - 既存型との統合は Phase 2 で実施
- **Trade-offs**: AD 非互換だが、アーキテクチャパターンの検証には十分
- **Follow-up**: 本番統合時に既存 `CurveEnum<T>` への移行を検討

### Decision: MarketProvider の配置
- **Context**: 遅延評価キャッシュ機構の配置先
- **Alternatives Considered**:
  1. `pricer_optimiser`（L2.5）
  2. `pricer_risk`（L4）
  3. 新規 `pricer_cache` crate
- **Selected Approach**: Option 1（`pricer_optimiser`）
- **Rationale**:
  - Bootstrapping/Calibration と論理的に同居
  - 依存性規則（L2.5 は L2 のみに依存）を維持
- **Trade-offs**: 既存 `pricer_optimiser` の責務が若干拡大
- **Follow-up**: 責務が肥大化した場合、分割を検討

### Decision: PricingContext のライフタイム管理
- **Context**: 参照ベースコンテキストのライフタイム設計
- **Alternatives Considered**:
  1. `'a` ライフタイムパラメータで借用
  2. `Arc` で所有権共有
- **Selected Approach**: Option 1（ライフタイムパラメータ）
- **Rationale**:
  - 要件3.1「`&CurveEnum` reference（not `Arc` or `HashMap`）」に合致
  - ゼロコスト抽象化
- **Trade-offs**: ライフタイム管理の複雑さ
- **Follow-up**: Rayon 並列化との相性を実装時に検証

## Risks & Mitigations
- **Risk 1**: 並列実行時のデッドロック — `RwLock` の取得順序を統一（curve → vol の順）
- **Risk 2**: ライフタイム制約によるコンパイルエラー — `Arc` からの参照取得パターンを標準化
- **Risk 3**: ログ出力の並列実行時の順序不定 — デモ目的では許容。本番では structured logging を使用

## References
- [Rust std::sync::RwLock](https://doc.rust-lang.org/std/sync/struct.RwLock.html)
- [Rust std::sync::Arc](https://doc.rust-lang.org/std/sync/struct.Arc.html)
- [Rayon Data Parallelism](https://docs.rs/rayon/latest/rayon/)
