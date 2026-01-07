# Gap Analysis: crate-architecture-redesign

## 概要

本ドキュメントは、要件と既存コードベースの間のギャップを分析し、実装戦略を策定するための情報を提供する。

**分析対象:**

- 11件の要件（アセットクラス拡張、マルチカーブ、金利/クレジット/FX/エキゾチック対応）
- 既存4クレート（pricer_core, pricer_models, pricer_kernel, pricer_xva）

## 1. 現状調査

### 1.1 既存アセット一覧

| クレート | モジュール | 主要コンポーネント |
|----------|-----------|-------------------|
| pricer_core | market_data/curves | `YieldCurve` trait, `FlatCurve`, `InterpolatedCurve` |
| pricer_core | market_data/surfaces | `VolatilitySurface` trait, `FlatVol`, `InterpolatedVolSurface` |
| pricer_core | types | `Currency` (5通貨), `Date`, `DayCountConvention` (3種) |
| pricer_core | math/solvers | `NewtonRaphson`, `Brent` |
| pricer_core | math/interpolators | Linear, CubicSpline, Monotonic, Bilinear |
| pricer_models | instruments | `Instrument<T>` enum (Vanilla, Forward, Swap) |
| pricer_models | models | `StochasticModel` trait, `GBM`, `SingleState`, `TwoFactorState` |
| pricer_models | analytical | Black-Scholes distributions |
| pricer_kernel | mc | `MonteCarloPricer`, `Workspace`, paths |
| pricer_kernel | path_dependent | Asian, Barrier, Lookback |
| pricer_kernel | greeks | `GreeksConfig`, `GreeksMode`, `GreeksResult<T>` |
| pricer_kernel | checkpoint | Checkpointing strategy |
| pricer_xva | portfolio | `Portfolio`, `Counterparty`, `NettingSet`, `Trade` |
| pricer_xva | xva | `XvaCalculator`, CVA/DVA/FVA |
| pricer_xva | soa | `ExposureSoA`, `TradeSoA` |

### 1.2 アーキテクチャパターン

**確立済みパターン:**

- **Enum Dispatch**: `Instrument<T>`, `StochasticModelEnum` - trait objectsを避け静的ディスパッチ
- **Generic Float**: 全型が `T: Float` でジェネリック（AD互換性）
- **Builder Pattern**: `PortfolioBuilder`, `GreeksConfig::builder()`
- **SoA Layout**: `ExposureSoA`, `TradeSoA` でベクトル化最適化
- **Workspace Buffers**: MC計算で再利用可能なバッファ
- **Error Types**: クレートごとの専用エラー型

**依存関係:**

```text
pricer_core ← pricer_models ← pricer_kernel ← pricer_xva
     L1            L2              L3            L4
```

### 1.3 統合ポイント

- **YieldCurve trait**: 新カーブ型の追加ポイント
- **StochasticModel trait**: 新モデル追加のインターフェース
- **Instrument enum**: 新商品追加（variant追加）
- **Portfolio**: 取引登録のエントリポイント

## 2. 要件実現可能性分析

### Requirement 1: アセットクラス別商品階層

| 技術要素 | 現状 | ギャップ |
|----------|------|---------|
| Instrument enum | 3 variants (Vanilla, Forward, Swap) | アセットクラス別サブenumに再構成が必要 |
| Instrument trait | 未実装 | price(), greeks(), cashflows()の共通トレイト新規作成 |
| equity/ module | instruments/直下にflat | サブモジュール化が必要 |
| rates/ module | Missing | 新規作成 |
| credit/ module | Missing | 新規作成 |
| fx/ module | Missing | 新規作成 |
| exotic/ module | Missing | 新規作成 |
| Schedule | Missing | 金利商品用に新規作成 |

**複雑度:** M (3-7日) - 既存enumの再構成 + 新モジュール追加
**リスク:** Medium - 後方互換性の維持が必要

### Requirement 2: マルチカーブ市場データ基盤

| 技術要素 | 現状 | ギャップ |
|----------|------|---------|
| YieldCurve trait | 実装済み | 拡張可能 |
| CurveSet | Missing | 名前付きカーブ集合の新規作成 |
| CreditCurve trait | Missing | ハザードレート計算の新規トレイト |
| HazardRateCurve | Missing | CreditCurve実装 |
| FxVolatilitySurface | Missing | デルタ・満期グリッドのサーフェス |
| MarketDataError | 実装済み | 拡張のみ |

**複雑度:** M (3-7日) - 新トレイト + 複数実装
**リスク:** Medium - 既存カーブとの整合性

### Requirement 3: 確率モデル拡張フレームワーク

| 技術要素 | 現状 | ギャップ |
|----------|------|---------|
| StochasticModel trait | 実装済み | `num_factors()` 追加のみ |
| SingleState/TwoFactorState | 実装済み | 十分 |
| GBM | 実装済み | 十分 |
| Hull-White | Missing | 新規実装 |
| CIR | Missing | 新規実装 |
| Heston | Missing | 新規実装 |
| LMM | Missing | 新規実装（複雑） |
| Correlated models | Missing | Cholesky分解の実装 |
| Calibrator trait | Missing | 新規トレイト定義 |

**複雑度:** L (1-2週) - 複数モデル実装 + キャリブレーション
**リスク:** High - LMMの実装複雑性、Enzyme互換性確認

### Requirement 4: 金利デリバティブ対応

| 技術要素 | 現状 | ギャップ |
|----------|------|---------|
| Swap struct | 基本実装あり | レグ構造の拡張が必要 |
| InterestRateSwap | Missing | 固定/変動レグ、日数計算 |
| Swaption | Missing | 新規実装 |
| Cap/Floor | Missing | 新規実装 |
| Schedule | Missing | 支払日生成ロジック |
| Black76 | Missing | Swaption解析解 |
| Bachelier | Missing | Normal model |

**複雑度:** L (1-2週) - 金利商品の基盤構築
**リスク:** High - スケジュール生成の複雑性、カーブ選択ロジック

### Requirement 5: クレジットデリバティブ対応

| 技術要素 | 現状 | ギャップ |
|----------|------|---------|
| CreditParams | pricer_xvaに存在 | pricer_coreに移動検討 |
| CDS struct | Missing | 新規実装 |
| HazardRateCurve | Missing | Req 2と共通 |
| Default simulation | Missing | MC拡張 |
| WWR | Missing | CVA計算拡張 |

**複雑度:** M (3-7日) - CDS実装 + ハザードレートカーブ
**リスク:** Medium - XVAとの統合

### Requirement 6: 為替デリバティブ対応

| 技術要素 | 現状 | ギャップ |
|----------|------|---------|
| Currency enum | 5通貨実装済み | 拡張可能 |
| CurrencyPair | Missing | 新規構造体 |
| FxOption | Missing | 新規実装 |
| FxForward | Missing | 新規実装 |
| Garman-Kohlhagen | Missing | FXオプション解析解 |
| FxVolatilitySurface | Missing | Req 2と共通 |

**複雑度:** M (3-7日) - FX商品 + GK model
**リスク:** Low - 明確な実装パス

### Requirement 7: レイヤー構成とフォルダ構造

| 技術要素 | 現状 | ギャップ |
|----------|------|---------|
| pricer_kernel → pricer_engine | リネーム必要 | Cargo.toml + 参照更新 |
| pricer_xva → pricer_risk | リネーム必要 | Cargo.toml + 参照更新 |
| instruments/ sub-modules | flat構造 | equity/, rates/, credit/, fx/, exotic/ |
| models/ sub-modules | flat構造 | equity/, rates/, hybrid/ |
| feature flags | 未実装 | Cargo.toml features追加 |

**複雑度:** M (3-7日) - 大規模リファクタリング
**リスク:** High - 全クレートに影響、CI/CDテスト必須

### Requirement 8: キャリブレーション基盤

| 技術要素 | 現状 | ギャップ |
|----------|------|---------|
| Newton-Raphson | 実装済み | 十分 |
| Brent | 実装済み | 十分 |
| Levenberg-Marquardt | Missing | 非線形最小二乗法の新規実装 |
| Calibrator trait | Missing | 新規トレイト定義 |
| CalibrationError | Missing | 新規エラー型 |

**複雑度:** M (3-7日) - L-Mソルバー + トレイト
**リスク:** Medium - 数値安定性、収束性

### Requirement 9: リスクファクター管理

| 技術要素 | 現状 | ギャップ |
|----------|------|---------|
| GreeksConfig/Result | 実装済み | 拡張可能 |
| RiskFactor trait | Missing | 新規トレイト |
| GreeksAggregator | Missing | ポートフォリオレベル集計 |
| Scenario engine | Missing | 新規実装 |
| Preset scenarios | Missing | パラレル/ツイスト/バタフライ |

**複雑度:** M (3-7日) - リスク基盤構築
**リスク:** Medium - 既存Greeks統合

### Requirement 10: パフォーマンスとメモリ効率

| 技術要素 | 現状 | ギャップ |
|----------|------|---------|
| SoA layout | 実装済み (pricer_xva) | 十分 |
| Rayon parallelization | 実装済み | 十分 |
| Workspace buffers | 実装済み (pricer_kernel) | 十分 |
| Checkpointing | 実装済み | 十分 |
| criterion benchmarks | 実装済み | アセットクラス別追加 |

**複雑度:** S (1-3日) - ベンチマーク追加のみ
**リスク:** Low - 既存パターン適用

### Requirement 11: エキゾチックデリバティブ対応

| 技術要素 | 現状 | ギャップ |
|----------|------|---------|
| Asian/Barrier/Lookback | pricer_kernelに実装済み | pricer_modelsに移動検討 |
| VarianceSwap | Missing | 新規実装 |
| VolatilitySwap | Missing | 新規実装 |
| Cliquet | Missing | 新規実装 |
| Autocallable | Missing | 新規実装 |
| Rainbow | Missing | マルチアセット対応 |
| QuantoOption | Missing | Quanto調整 |
| Bermudan | Missing | Longstaff-Schwartz |

**複雑度:** XL (2週以上) - 多数のエキゾチック実装
**リスク:** High - 複雑なペイオフ、MC精度

## 3. 実装アプローチオプション

### Option A: 既存コンポーネント拡張

**対象要件:** Req 2, 3, 8, 9, 10

**戦略:**

- YieldCurve traitを維持し、CurveSet/CreditCurve を追加
- StochasticModel traitを維持し、新モデルをenum variantとして追加
- math/solversにLevenberg-Marquardtを追加
- pricer_xvaのGreeksをpricer_coreに移動してリスクファクター基盤化

**トレードオフ:**

- ✅ 既存パターン活用、学習コスト低
- ✅ 後方互換性維持が容易
- ❌ ファイル肥大化リスク
- ❌ 責務境界が曖昧になる可能性

### Option B: 新規コンポーネント作成

**対象要件:** Req 1, 4, 5, 6, 7, 11

**戦略:**

- pricer_models/instruments/ 配下にアセットクラス別モジュール新規作成
- pricer_models/schedules/ 新規モジュール
- pricer_models/instruments/exotic/ に全エキゾチック商品
- クレート名リネーム（kernel→engine, xva→risk）

**トレードオフ:**

- ✅ 明確な責務分離
- ✅ 独立したテスト可能性
- ❌ ファイル数増加
- ❌ インターフェース設計の複雑性

### Option C: ハイブリッドアプローチ（推奨）

**戦略:**

1. **Phase 1**: クレート名リネームとフォルダ再構成（Req 7）
2. **Phase 2**: 市場データ基盤拡張（Req 2）
3. **Phase 3**: アセットクラス別商品追加（Req 1, 4, 5, 6）
4. **Phase 4**: モデル拡張とキャリブレーション（Req 3, 8）
5. **Phase 5**: リスク管理とエキゾチック（Req 9, 11）
6. **Phase 6**: パフォーマンス検証（Req 10）

**段階的移行:**

- 既存API維持しながら新構造を並行構築
- Feature flagで段階的有効化
- 各フェーズ完了後にテスト・ベンチマーク

**トレードオフ:**

- ✅ リスク分散
- ✅ 段階的検証可能
- ✅ ロールバック容易
- ❌ 移行期間中の複雑性
- ❌ 重複コード一時的に発生

## 4. 複雑度・リスク評価サマリ

| 要件 | 複雑度 | リスク | 理由 |
|------|--------|--------|------|
| Req 1 | M | Medium | enum再構成、後方互換性 |
| Req 2 | M | Medium | 新トレイト、既存カーブ統合 |
| Req 3 | L | High | LMM複雑性、Enzyme互換性 |
| Req 4 | L | High | スケジュール生成、カーブ選択 |
| Req 5 | M | Medium | XVA統合 |
| Req 6 | M | Low | 明確な実装パス |
| Req 7 | M | High | 全クレート影響 |
| Req 8 | M | Medium | 数値安定性 |
| Req 9 | M | Medium | 既存Greeks統合 |
| Req 10 | S | Low | 既存パターン適用 |
| Req 11 | XL | High | 複雑なペイオフ、MC精度 |

**総合評価:** **L〜XL** (2-4週) - 段階的実装推奨

## 5. 設計フェーズへの推奨事項

### 優先実装順序

1. **Req 7** (クレート名・構造変更) - 他の全要件の基盤
2. **Req 2** (マルチカーブ) - 金利/クレジット商品の前提条件
3. **Req 1** (商品階層) - 新商品追加の基盤
4. **Req 4, 5, 6** (アセットクラス別商品) - 並行実装可能
5. **Req 3, 8** (モデル・キャリブレーション) - 商品実装後
6. **Req 9** (リスクファクター) - 商品・モデル完成後
7. **Req 11** (エキゾチック) - 最後に追加
8. **Req 10** (パフォーマンス) - 全体通して継続

### 調査必要事項

| 項目 | 調査内容 | 優先度 |
|------|----------|--------|
| LMM実装 | BGM vs LMM、Enzyme互換性 | High |
| Longstaff-Schwartz | 回帰基底関数の選択 | High |
| スケジュール生成 | IMM日付、カレンダー統合 | Medium |
| Wrong-Way Risk | CVA計算への統合方法 | Medium |
| Variance Swap | レプリケーション vs MC | Low |

### 決定事項

設計フェーズで以下を決定する必要あり:

1. **Instrument trait vs enum dispatch のみ**: トレイト追加の必要性
2. **LMM実装スコープ**: 1ファクター簡略版 vs フルLMM
3. **カレンダー外部依存**: chrono拡張 vs 専用ライブラリ
4. **Feature flag粒度**: アセットクラス単位 vs 商品単位
