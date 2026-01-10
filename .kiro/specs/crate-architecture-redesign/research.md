# Research Log: crate-architecture-redesign

## Summary

**Discovery Scope**: Extension（既存システムの拡張）

本リサーチは、neutryx-rustの5層アーキテクチャ（L1→L2→L2.5→L3→L4）を基盤とした拡張設計のための調査を記録する。主に以下の3領域を対象とした：

1. **pricer_optimiser (L2.5)の役割明確化**: キャリブレーション・ブートストラップ・ソルバーの責務分離
2. **エキゾチックデリバティブ実装**: VarianceSwap, Cliquet, Autocallable等の構造体設計
3. **リスクファクター管理**: RiskFactor trait, ScenarioEngine, GreeksAggregatorの設計

**Key Findings**:

- 現行コードベースは既に5層構造を採用しており、大部分の基盤は実装済み
- エキゾチック商品はモジュールのみ存在し、構造体実装が必要
- リスクファクター管理は新規モジュールとして追加が必要

---

## Research Log

### Topic 1: pricer_optimiser (L2.5)の位置づけ

**Investigation Date**: 2026-01-10

**Sources**:

- 既存コード: `crates/pricer_optimiser/src/`
- Steering: `.kiro/steering/tech.md`

**Findings**:

pricer_optimiserは以下の3モジュールで構成：

```text
pricer_optimiser/src/
├── bootstrapping/   → CurveBuilder (イールドカーブ構築)
├── calibration/     → CalibrationEngine (モデルパラメータ最適化)
└── solvers/         → LM, BFGS (数値ソルバー)
```

**Implications**:

- L2（モデル定義）とL3（評価エンジン）の間に位置し、両方に依存可能
- キャリブレーションはL2のモデル定義を使用し、オプションでL3の勾配計算を活用
- この構造により、キャリブレーションロジックが評価ロジックから分離され、テスト容易性が向上

**Decision**: 現行構造を維持。L2.5としての役割を設計書に明記。

---

### Topic 2: エキゾチックデリバティブ構造体設計

**Investigation Date**: 2026-01-10

**Sources**:

- 既存コード: `crates/pricer_models/src/instruments/exotic/mod.rs`
- 業界標準: FpML 5.12 exotic product definitions
- 学術文献: Gatheral (2006) "The Volatility Surface"

**Findings**:

現行の`exotic/mod.rs`はスケルトンのみ。必要な構造体：

| 商品 | 主要フィールド | 評価手法 |
|------|---------------|---------|
| VarianceSwap | strike, notional, observation_dates | MC + Log-return計算 |
| VolatilitySwap | strike, notional | VarianceSwapの平方根近似 |
| Cliquet | reset_dates, local/global cap/floor | MC + Forward-starting |
| Autocallable | barriers, coupon_rate, knockin | MC + 早期終了判定 |
| Rainbow | underlyings, weights, rainbow_type | MC + 相関モデル |
| QuantoOption | fx_rate, underlying_option | GK + Quanto調整 |

**Implications**:

- 全構造体は`T: Float`でジェネリックにしEnzyme AD互換性を維持
- `#[cfg(feature = "exotic")]`で条件付きコンパイル
- InstrumentEnumに`Exotic(ExoticInstrument<T>)`variantを追加

**Decision**: 上記6構造体を優先実装。Bermudan SwaptionはLSM実装後に追加。

---

### Topic 3: Longstaff-Schwartz (LSM)法の実装戦略

**Investigation Date**: 2026-01-10

**Sources**:

- Longstaff & Schwartz (2001) "Valuing American Options by Simulation"
- 既存コード: `crates/pricer_pricing/src/mc/`

**Findings**:

LSM法の主要コンポーネント：

1. **Backward Induction**: 満期から逆方向に継続価値を計算
2. **Regression**: 基底関数（多項式、Laguerre、Hermite）による条件付き期待値推定
3. **Exercise Decision**: 即時行使価値 vs 継続価値の比較

実装上の考慮点：

- **Bias**: 同一パスで回帰と評価を行うとバイアス発生 → Two-pass法で軽減
- **基底関数選択**: Laguerre多項式が一般的だが、多項式でも十分な精度
- **パス数**: 50,000以上で安定した結果

**Implications**:

- pricer_pricing/mc/に`lsm.rs`モジュールを追加
- MCConfigにLSM用パラメータ（基底関数タイプ、数）を追加
- Bermudan Swaptionで検証（Hull-Whiteモデル）

**Decision**: Polynomial基底をデフォルトとし、Laguerreをオプションで提供。

---

### Topic 4: リスクファクター管理アーキテクチャ

**Investigation Date**: 2026-01-10

**Sources**:

- 既存コード: `crates/pricer_risk/src/`
- 業界標準: FRTB-SA risk factor taxonomy

**Findings**:

現行pricer_riskの構造：

```text
pricer_risk/src/
├── portfolio/    → Trade, Counterparty, NettingSet
├── exposure/     → EE, EPE, PFE
├── xva/          → CVA, DVA, FVA
├── soa/          → Structure of Arrays
└── parallel/     → Rayon utilities
```

不足している機能：

1. **RiskFactor trait**: 複数リスクタイプの統一インターフェース
2. **ScenarioEngine**: ストレステスト・感応度分析
3. **GreeksAggregator**: ポートフォリオレベルGreeks集約

**Implications**:

- RiskFactor traitはpricer_core/traits/に配置（L1）
- ScenarioEngine, GreeksAggregatorはpricer_risk/に配置（L4）
- プリセットシナリオ（Parallel +100bp, Twist, Butterfly等）を提供

**Decision**: 新規モジュール`risk_factors/`と`scenarios/`をpricer_riskに追加。

---

### Topic 5: Enum Dispatch vs Trait Objects

**Investigation Date**: 2026-01-07（初回調査）

**Sources**:

- enum_dispatch crate documentation
- Rust Polymorphism patterns

**Findings**:

- Enum dispatchはtrait objectsと比較して最大10倍のパフォーマンス向上
- コンパイラが最適化（インライン化）を適用可能、vtableルックアップ不要
- 「Closed World」前提: 全variant型がコンパイル時に既知である必要
- コード膨張のリスクあり（モノモーフィゼーション）

**Implications**:

- 現行の`InstrumentEnum<T>`, `StochasticModelEnum`パターンを継続
- アセットクラス別にサブenumを定義し、トップレベルenumでディスパッチ
- Enzyme互換性のため、全商品・モデルでenum dispatch維持必須

**Decision**: 既存パターン継続。

---

## Architecture Decisions

### ADR-1: 5層アーキテクチャの維持

**Context**: pricer_optimiser (L2.5)が追加された現行構造

**Decision**: L1→L2→L2.5→L3→L4の5層構造を維持

**Rationale**:

- アルファベット順（C < M < O < P < R）と依存順が一致
- キャリブレーションがモデル定義と評価エンジンの間に自然に位置
- Enzyme AD (L3)の隔離が継続

**Consequences**:

- 新規開発者の学習コストは4層より若干増加
- ただし、責務の明確化により長期的なメンテナンス性は向上

### ADR-2: Enum Dispatchの継続

**Context**: エキゾチック商品追加時のディスパッチ方式

**Decision**: `ExoticInstrument<T>` enumを使用し、trait objectsを避ける

**Rationale**:

- Enzyme ADは静的ディスパッチで最適化
- コンパイル時型安全性
- パターンマッチによる網羅性検査

**Consequences**:

- 新商品追加時はenum variantの追加が必要
- ただし、これは設計上意図的な制約（明示的な拡張）

### ADR-3: RiskFactor traitの配置

**Context**: RiskFactor抽象化の配置先

**Decision**: pricer_core/traits/ (L1)に配置

**Rationale**:

- リスクファクターは市場データの一種として基盤層に属する
- L4 (pricer_risk)からL1 (pricer_core)への依存は許可されている
- 将来的にL2/L3でもリスクファクターを参照可能

**Consequences**:

- pricer_coreのAPIサーフェスが増加
- ただし、トレイト定義のみでありコンパイル時間への影響は軽微

### ADR-4: Feature Flag粒度

**Context**: 条件付きコンパイルの粒度

**Decision**: アセットクラス単位feature flag

**Rationale**:

- 依存関係管理が容易
- コンパイル時間の有意な削減可能
- 金融業界の標準分類に合致

**Consequences**:

- default = ["equity"]
- optional = ["rates", "credit", "fx", "commodity", "exotic"]

---

## Risks and Mitigations

### Risk 1: LSM実装の数値安定性

**Likelihood**: Medium
**Impact**: High

**Description**: 基底関数の選択やパス数不足により、Bermudan評価が不安定になる可能性

**Mitigation**:

- 単純なAmerican putで既知解との比較テスト
- 50,000パス以上を推奨設定として文書化
- Two-pass法によるバイアス軽減をデフォルト有効化

### Risk 2: エキゾチック商品のEnzyme AD互換性

**Likelihood**: Low
**Impact**: Medium

**Description**: 複雑なペイオフ構造がEnzyme ADで微分不可能になる可能性

**Mitigation**:

- smooth_max, smooth_indicatorの徹底使用
- 各構造体でsmoothing_epsilonパラメータを保持
- num-dualモードでの検証テストを必須化

### Risk 3: ScenarioEngineのパフォーマンス

**Likelihood**: Medium
**Impact**: Medium

**Description**: 大規模ポートフォリオ×多数シナリオでの評価時間増大

**Mitigation**:

- Rayon並列化（シナリオ単位、取引単位の2レベル）
- ベースケース評価結果のキャッシュ
- 差分評価（変更リスクファクターのみ再計算）の将来実装

---

## Open Questions

1. **WWR (Wrong-Way Risk)の実装優先度**: CVA計算でのWWR対応は本フェーズに含めるか、次フェーズに延期するか
2. **Bermudan SwaptionのLMM依存**: Hull-Whiteでの実装を先行し、LMMは将来フェーズとするか
3. **規制計算モジュールの配置**: `pricer_risk/regulatory/`に配置するか、別クレート`pricer_regulatory`とするか

---

## References

- Longstaff & Schwartz (2001) "Valuing American Options by Simulation"
- Gatheral (2006) "The Volatility Surface"
- enum_dispatch crate: https://docs.rs/enum_dispatch/
- FRTB-SA risk factor taxonomy (Basel Committee)

---

_Last Updated: 2026-01-10_
