# Requirements Document

## Introduction

本仕様は、A-I-P-S構造、3段ロケットパターン、遅延評価（Lazy Evaluation）、およびArcキャッシュを統合した最小構成のポートフォリオプライシング実装を定義する。標準スワップ（Vol不要）とCMSスワップ（Vol必要）が混在するポートフォリオを評価し、各アーキテクチャ要素の効果をログおよびロジックで明確に確認可能とすることを目的とする。

## Requirements

### Requirement 1: モデル・商品定義（pricer_models L2）

**Objective:** As a 開発者, I want 純粋なデータ構造とEnum定義を持つモデル・商品・マーケットオブジェクトの型, so that 3段ロケットの第1段として静的ディスパッチによる高速な型分岐が実現できる

#### Acceptance Criteria 1

1. The pricer_models shall provide `ModelEnum` enumeration containing `BlackScholes` and `HullWhite` model variants
2. The pricer_models shall provide `InstrumentEnum` enumeration containing `VanillaSwap` and `CmsSwap` instrument variants
3. When `InstrumentEnum::requires_vol()` is called on `VanillaSwap`, the pricer_models shall return `false`
4. When `InstrumentEnum::requires_vol()` is called on `CmsSwap`, the pricer_models shall return `true`
5. The pricer_models shall provide `CurveEnum` enumeration with `get_df(t: f64)` method returning discount factor
6. The pricer_models shall provide `VolSurfaceEnum` enumeration for volatility surface representation
7. The pricer_models shall provide `Currency` enumeration with `USD`, `JPY`, `EUR` variants implementing `Hash` and `Eq` traits

### Requirement 2: 遅延評価とArcキャッシュ（pricer_optimiser L2.5）

**Objective:** As a 開発者, I want スレッドセーフな遅延評価キャッシュ機構, so that 必要なカーブ/Volのみがオンデマンドで構築され、同一オブジェクトが複数スレッド間で共有される

#### Acceptance Criteria 2

1. The MarketProvider shall maintain `RwLock<HashMap<Currency, Arc<CurveEnum>>>` for curve caching
2. The MarketProvider shall maintain `RwLock<HashMap<Currency, Arc<VolSurfaceEnum>>>` for volatility surface caching
3. When `get_curve(ccy)` is called and cache contains the currency, the MarketProvider shall return `Arc<CurveEnum>` without bootstrapping
4. When `get_curve(ccy)` is called and cache does not contain the currency, the MarketProvider shall bootstrap the curve, log the operation, cache the result, and return `Arc<CurveEnum>`
5. When `get_vol(ccy)` is called and cache contains the currency, the MarketProvider shall return `Arc<VolSurfaceEnum>` without calibration
6. When `get_vol(ccy)` is called and cache does not contain the currency, the MarketProvider shall calibrate the surface, log the operation, cache the result, and return `Arc<VolSurfaceEnum>`
7. The MarketProvider shall implement double-check locking pattern to prevent duplicate construction under concurrent access

### Requirement 3: プライシングコンテキストとカーネル（pricer_pricing L3）

**Objective:** As a 開発者, I want 参照ベースの軽量コンテキストとHashMap検索のないカーネル, so that ホットループ内で純粋なポインタ参照のみによる高速計算が実現できる

#### Acceptance Criteria 3

1. The PricingContext shall hold `&CurveEnum` reference for discount curve (not `Arc` or `HashMap`)
2. The PricingContext shall hold `Option<&VolSurfaceEnum>` for optional volatility adjustment
3. The price_single_trade function shall accept `&ModelEnum`, `&InstrumentEnum`, and `&PricingContext` as parameters
4. When pricing `VanillaSwap`, the pricing kernel shall compute payoff without accessing volatility surface
5. When pricing `CmsSwap`, the pricing kernel shall apply convexity adjustment using `ctx.adjustment_vol`
6. The pricing kernel shall not perform any `HashMap` lookups or dynamic allocation during price calculation

### Requirement 4: ポートフォリオオーケストレーション（pricer_risk L4）

**Objective:** As a 開発者, I want Pull-then-Push実行パターンによるポートフォリオ並列処理, so that 準備フェーズ（遅延取得）と実行フェーズ（カーネル呼び出し）が明確に分離される

#### Acceptance Criteria 4

1. The Trade struct shall contain `id: String`, `ccy: Currency`, `model: ModelEnum`, and `instrument: InstrumentEnum` fields
2. The run_portfolio_pricing function shall accept `&[Trade]` and `&MarketProvider` as parameters
3. When processing trades, the pricer_risk shall use `rayon::par_iter()` for parallel execution
4. While processing each trade, the pricer_risk shall first resolve curve dependency via `market.get_curve(trade.ccy)`
5. If `trade.instrument.requires_vol()` returns `true`, the pricer_risk shall resolve volatility dependency via `market.get_vol(trade.ccy)`
6. If `trade.instrument.requires_vol()` returns `false`, the pricer_risk shall not call `market.get_vol()`
7. The pricer_risk shall construct `PricingContext` with resolved references and invoke `price_single_trade`

### Requirement 5: 可観測性とアーキテクチャ検証

**Objective:** As a 開発者, I want ログ出力による遅延評価とキャッシュ動作の可視化, so that 設計目標の達成をログ出力パターンで証明できる

#### Acceptance Criteria 5

1. When bootstrapping a yield curve, the MarketProvider shall log `"[Optimiser] Bootstrapping Yield Curve for {currency}..."`
2. When calibrating a volatility surface, the MarketProvider shall log `"[Optimiser] Calibrating SABR Surface for {currency}..."`
3. When two trades with the same currency are processed, the bootstrap log shall appear only once (Arc cache verification)
4. When `VanillaSwap` trades are processed, the volatility calibration log shall not appear (lazy evaluation verification)
5. When `CmsSwap` trade is processed, the volatility calibration log shall appear for the first occurrence only

### Requirement 6: A-I-P-S依存性規則の遵守

**Objective:** As a アーキテクト, I want 一方向データフローの強制, so that レイヤー間の依存関係が明確でありテスト可能である

#### Acceptance Criteria 6

1. The pricer_models (L2) shall not depend on pricer_optimiser, pricer_pricing, pricer_risk, or any Service/Adapter crate
2. The pricer_optimiser (L2.5) shall depend only on pricer_models and pricer_core
3. The pricer_pricing (L3) shall depend only on pricer_models and pricer_core
4. The pricer_risk (L4) shall depend on pricer_models, pricer_optimiser, and pricer_pricing
5. The service_cli shall depend on pricer_models, pricer_optimiser, and pricer_risk for demonstration execution
