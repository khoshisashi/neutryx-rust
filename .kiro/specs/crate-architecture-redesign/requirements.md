# Requirements Document

## Introduction

本仕様書は、neutryx-rustライブラリのクレート構成を全デリバティブ評価に対応できるよう再設計するための要件を定義する。現行の5層アーキテクチャを基盤としつつ、株式デリバティブに加えて金利・クレジット・為替・コモディティ・エキゾチックデリバティブをカバーする拡張性を確保する。

**クレート命名規則（確定）**:

- `pricer_core` (L1) → 基盤（数学・型・市場データ）
- `pricer_models` (L2) → 商品定義・確率モデル
- `pricer_optimiser` (L2.5) → キャリブレーション・ブートストラップ・ソルバー 【NEW】
- `pricer_pricing` (L3) → MC・AD・評価エンジン
- `pricer_risk` (L4) → リスク計算・XVA・エクスポージャー

**命名規則の設計原則**:
アルファベット順（C < M < O < P < R）と依存順（L1 → L2 → L2.5 → L3 → L4）が一致するよう設計。
`ls`やIDEでの表示順が自然に依存階層を反映する。

**現状の実装状況**:

✅ = 実装済み | ⏳ = 一部実装 | ❌ = 未実装

- ✅ 基本的な4層+L2.5アーキテクチャ構造
- ✅ アセットクラス別フォルダ構成（equity, rates, credit, fx, commodity, exotic）
- ✅ マルチカーブ対応（CurveSet, CreditCurve）
- ✅ 確率モデル（GBM, Heston, SABR, Hull-White, CIR）
- ✅ 金利デリバティブ（IRS, Swaption, Cap/Floor）
- ✅ クレジットデリバティブ（CDS with simulation）
- ✅ 為替デリバティブ（FxOption, FxForward, Garman-Kohlhagen）
- ⏳ エキゾチックデリバティブ（モジュール存在、実装はスケルトン）
- ⏳ リスクファクター管理（構造は存在、シナリオエンジン未実装）
- ❌ 規制計算（SA-CCR, FRTB, SIMM）

**目標**:

- アセットクラス非依存の商品階層設計を完成
- pricer_optimiserの役割明確化
- 残りのエキゾチック商品の実装
- リスクファクター・シナリオ管理の整備

## Requirements

### Requirement 1: 5層レイヤー構成（確定）

**Objective:** As a ライブラリメンテナー, I want 5層アーキテクチャの責務を明確化しフォルダ構造を整理, so that 将来のアセットクラス追加が容易になる

#### Acceptance Criteria 1

1. The workspace shall 以下の命名規則でクレートを構成する: pricer_core (L1), pricer_models (L2), pricer_optimiser (L2.5), pricer_pricing (L3), pricer_risk (L4)
2. The pricer_optimiser crate shall キャリブレーション・ブートストラップ・数値ソルバーを提供し、L2とL3の中間層として機能する
3. The dependency graph shall 常にL1→L2→L2.5→L3→L4の方向のみを許可し、循環依存を禁止する
4. The pricer_optimiser crate shall pricer_core (L1) と pricer_models (L2) に依存し、オプションでpricer_pricing (L3) からの勾配計算を利用できる
5. When 新規クレートを追加する場合, the workspace shall 将来的に`pricer_rates`, `pricer_credit`等のアセットクラス別クレート分割をサポートする構造を維持する

### Requirement 2: アセットクラス別商品階層（実装済み確認）

**Objective:** As a クオンツ開発者, I want アセットクラスごとに商品を分類・整理できる階層構造, so that 新規商品の追加が既存コードに影響を与えず拡張可能になる

#### Acceptance Criteria 2

1. The pricer_models crate shall 商品を「equity」「rates」「credit」「fx」「commodity」「exotic」のアセットクラス別サブモジュールに分類する [✅実装済み]
2. When 新しいアセットクラスの商品を追加する場合, the Instrument enum shall 静的ディスパッチを維持しつつアセットクラス別のサブenumで拡張可能である [✅実装済み]
3. The pricer_models crate shall 共通の`InstrumentTrait`を提供し、全商品がpayoff(), expiry(), currency(), notional()メソッドを実装する [✅実装済み]
4. Where 金利デリバティブが含まれる場合, the pricer_models/schedules module shall Schedule構造体（支払日・計算期間・日数計算規約）を提供する [✅実装済み]
5. The pricer_models crate shall feature flagによりアセットクラス別の条件付きコンパイルをサポートする（例: `features = ["rates", "credit", "fx", "exotic"]`） [✅実装済み]

### Requirement 3: マルチカーブ市場データ基盤（実装済み確認）

**Objective:** As a リスク管理者, I want 複数のイールドカーブとクレジットカーブを統一的に管理できる基盤, so that 金利デリバティブとXVA計算で適切なディスカウント・フォワードレートを使用できる

#### Acceptance Criteria 3

1. The pricer_core/market_data module shall 複数のイールドカーブ（OIS, SOFR, TONAR等）を名前付きで登録・取得できるCurveSet構造体を提供する [✅実装済み]
2. When デリバティブを評価する場合, the pricing engine shall ディスカウントカーブとフォワードカーブを分離して指定可能である [✅実装済み]
3. The pricer_core crate shall CreditCurveトレイトを提供し、ハザードレート・生存確率・デフォルト確率の計算を抽象化する [✅実装済み]
4. If カーブ構築に必要なマーケットデータが不足している場合, the MarketDataError shall 欠損データの詳細を含むエラーメッセージを返す [✅実装済み]
5. The market_data module shall すべてのカーブ・サーフェスを`T: Float`でジェネリックに保ち、AD互換性を維持する [✅実装済み]

### Requirement 4: 確率モデル拡張フレームワーク（実装済み確認）

**Objective:** As a クオンツ研究者, I want 金利モデル（Hull-White, CIR）やジャンプ拡散モデルを追加できるフレームワーク, so that 様々なアセットクラスに適したモデルで評価できる

#### Acceptance Criteria 4

1. The StochasticModel trait shall `evolve_step()`, `initial_state()`, `brownian_dim()`メソッドを提供し、1ファクター/2ファクター/マルチファクターモデルを統一的に扱う [✅実装済み]
2. When Hull-Whiteモデルを使用する場合, the pricer_models/models/rates module shall mean-reversion速度とボラティリティパラメータを受け取り、短期金利パスを生成する [✅実装済み]
3. The StochasticModelEnum shall 静的ディスパッチを維持しつつ、新規モデル追加時にenum variantの追加のみで拡張可能である [✅実装済み]
4. Where 相関を持つ複数ファクターモデルが必要な場合, the pricer_models/models/hybrid module shall 相関行列を受け取りコレスキー分解で相関ブラウン運動を生成する [✅実装済み]
5. The pricer_models crate shall Heston, SABRモデルを提供する [✅実装済み]

### Requirement 5: キャリブレーション基盤（pricer_optimiser）

**Objective:** As a クオンツ, I want モデルパラメータを市場データにキャリブレートする基盤, so that モデルが市場整合的な価格を出力する

#### Acceptance Criteria 5

1. The pricer_optimiser crate shall bootstrapping, calibration, solversの3モジュールを提供する [✅実装済み]
2. The pricer_optimiser/bootstrapping module shall イールドカーブのブートストラップ（OIS/Swap rates からの構築）を提供する [✅実装済み]
3. The pricer_optimiser/calibration module shall 確率モデルのキャリブレーション（Hull-White α/σ等）を提供する [✅実装済み]
4. The pricer_optimiser/solvers module shall Levenberg-Marquardt, BFGSアルゴリズムを提供する [✅実装済み]
5. If キャリブレーションが収束しない場合, the OptimiserError shall 残差、イテレーション数、収束判定基準を含む詳細情報を返す [✅実装済み]
6. The calibration framework shall Enzyme ADを活用した勾配計算によるキャリブレーション高速化をサポートする（オプション依存） [⏳一部実装]

### Requirement 6: 金利デリバティブ対応（実装済み確認）

**Objective:** As a 金利トレーダー, I want IRS、Swaption、Cap/Floorの評価機能, so that 金利デリバティブのポートフォリオをXVA計算に含められる

#### Acceptance Criteria 6

1. The pricer_models/instruments/rates module shall InterestRateSwap構造体（固定レグ・変動レグ・ノーショナル・日数計算規約）を提供する [✅実装済み]
2. When IRSを評価する場合, the pricing engine shall 変動レグのフォワードレート計算とディスカウントを適切なカーブで実行する [✅実装済み]
3. The pricer_models/instruments/rates module shall Swaption構造体（underlying swap, expiry, strike, option type）を提供する [✅実装済み]
4. Where Black or Bachelierモデルが選択された場合, the pricer_models/analytical module shall Swaption価格の解析解を提供する [✅実装済み]
5. The pricer_models/schedules module shall IMM日付、Modified Following、Act/360等の標準的な日付規約をサポートする [✅実装済み]
6. The pricer_models/instruments/rates module shall Cap/Floor構造体を提供する [✅実装済み]

### Requirement 7: クレジットデリバティブ対応（実装済み確認）

**Objective:** As a クレジットアナリスト, I want CDSの評価機能とハザードレート計算, so that クレジットエクスポージャーとCVA/DVA計算の精度を向上できる

#### Acceptance Criteria 7

1. The pricer_models/instruments/credit module shall CreditDefaultSwap構造体（参照エンティティ、ノーショナル、スプレッド、満期）を提供する [✅実装済み]
2. When CDSを評価する場合, the pricing engine shall ハザードレートカーブから生存確率を計算し、プロテクションレグとプレミアムレグのPVを算出する [✅実装済み]
3. The pricer_core/market_data module shall HazardRateCurve構造体（ハザードレートの期間構造）を提供する [✅実装済み]
4. If デフォルトイベントが発生した場合のシミュレーションにおいて, the pricer_models/instruments/credit/simulation module shall デフォルト時刻を生存確率の逆関数でサンプリングする [✅実装済み]
5. The pricer_risk crate shall Wrong-Way Risk（WWR）を考慮したCVA計算オプションを提供する [❌未実装]

### Requirement 8: 為替デリバティブ対応（実装済み確認）

**Objective:** As a FXトレーダー, I want FXオプションとFXフォワードの評価機能, so that マルチカレンシーポートフォリオのリスク管理ができる

#### Acceptance Criteria 8

1. The pricer_models/instruments/fx module shall FxOption構造体（通貨ペア、ストライク、満期、オプションタイプ）を提供する [✅実装済み]
2. When FXオプションを評価する場合, the pricing engine shall 国内・外国の金利カーブを使用したGarman-Kohlhagenモデルを適用する [✅実装済み]
3. The pricer_core/types module shall CurrencyPair構造体を提供し、ベース通貨・クォート通貨・スポットレートを管理する [✅実装済み]
4. Where マルチカレンシーXVA計算が必要な場合, the pricer_risk crate shall 各取引の決済通貨と評価通貨の変換を自動的に処理する [⏳一部実装]
5. The pricer_core/market_data module shall FxVolatilitySurface（デルタ・満期グリッドでのボラティリティ）を提供する [✅実装済み]

### Requirement 9: エキゾチックデリバティブ対応（実装中）

**Objective:** As a ストラクチャラー, I want Variance Swap、Autocallable、Cliquetなどのエキゾチック商品の評価機能, so that 仕組商品のプライシングとリスク管理ができる

#### Acceptance Criteria 9

1. The pricer_models/instruments/exotic module shall VarianceSwap構造体（実現ボラティリティ vs ストライク、バリアンス・ノーショナル）を提供する [❌スケルトンのみ]
2. When Variance Swapを評価する場合, the pricing engine shall ログリターンの二乗和から実現バリアンスを計算し、レプリケーションまたはMCで公正バリアンスストライクを算出する [❌未実装]
3. The pricer_models/instruments/exotic module shall Cliquet構造体（リセット日、ローカルキャップ/フロア、グローバルキャップ/フロア）を提供する [❌スケルトンのみ]
4. The pricer_models/instruments/exotic module shall Autocallable構造体（観測日、早期償還バリア、クーポン条件、ノックインプット）を提供する [❌スケルトンのみ]
5. Where 複数原資産オプション（Rainbow）が必要な場合, the pricer_models/instruments/exotic module shall BestOf/WorstOf構造体と相関パラメータを提供する [❌スケルトンのみ]
6. The pricer_models/instruments/exotic module shall QuantoOption構造体（原資産通貨、決済通貨、quanto調整）を提供する [❌スケルトンのみ]
7. When Bermudan Swaptionを評価する場合, the pricer_pricing crate shall Longstaff-Schwartz法による早期行使境界の推定を提供する [❌未実装]
8. The pricer_models/instruments/exotic module shall VolatilitySwap構造体（実現ボラティリティのペイオフ）を提供し、バリアンススワップとの違いを明確化する [❌スケルトンのみ]

### Requirement 10: リスクファクター管理（拡張予定）

**Objective:** As a リスクマネージャー, I want 複数のリスクファクター（金利、クレジット、FX）を統一的に管理, so that ポートフォリオ全体の感応度分析とストレステストができる

#### Acceptance Criteria 10

1. The pricer_risk crate shall RiskFactorトレイト（factor_type(), bump(), scenario()）を提供する [❌未実装]
2. When バンプシナリオを生成する場合, the risk framework shall 各リスクファクターを独立または同時にシフトできる [❌未実装]
3. The pricer_risk crate shall ポートフォリオレベルのDelta、Gamma、Vegaを計算するGreeksAggregator構造体を提供する [❌未実装]
4. Where ストレステストシナリオが定義された場合, the scenario engine shall 複数リスクファクターを同時にシフトしたPnLを計算する [❌未実装]
5. The pricer_risk/risk_factors module shall 金利カーブバンプ（パラレル、ツイスト、バタフライ）のプリセットシナリオを提供する [❌未実装]

### Requirement 11: パフォーマンスとメモリ効率（実装済み確認）

**Objective:** As a プロダクションエンジニア, I want 大規模ポートフォリオでも高速に評価できる性能, so that リアルタイムリスク計算とバッチ処理の両方に対応できる

#### Acceptance Criteria 11

1. The architecture shall Structure of Arrays (SoA)レイアウトをL4（pricer_risk）で維持し、ベクトル化最適化を可能にする [✅実装済み]
2. When 10,000件以上の取引を評価する場合, the parallel module shall Rayonによる自動並列化でCPUコアを効率的に使用する [✅実装済み]
3. The pricer_pricing crate shall メモリアロケーションを最小化するためのワークスペースバッファパターンを全評価パスで適用する [✅実装済み]
4. If メモリ制約がある環境で実行する場合, the checkpointing module shall メモリ使用量と再計算のトレードオフを設定可能にする [✅実装済み]
5. The benchmark suite shall 各アセットクラスの代表的な商品で`criterion`ベンチマークを提供し、パフォーマンス回帰を検出する [⏳一部実装]

### Requirement 12: 規制計算対応（将来拡張）

**Objective:** As a 規制報告担当者, I want SA-CCR、FRTB、SIMMの計算機能, so that 規制資本要件を満たすレポートを生成できる

#### Acceptance Criteria 12

1. The pricer_risk/regulatory module shall SA-CCR（Standardised Approach for Counterparty Credit Risk）計算を提供する [❌未実装]
2. The pricer_risk/regulatory module shall FRTB-SA（Fundamental Review of the Trading Book - Standardised Approach）計算を提供する [❌未実装]
3. The pricer_risk/regulatory module shall SIMM（Standard Initial Margin Model）計算を提供する [❌未実装]
4. Where 規制報告が必要な場合, the regulatory module shall XML/JSON形式でのレポート出力をサポートする [❌未実装]

## Appendix: 現行フォルダ構造

### A.1 pricer_core (L1: 基盤レイヤー)

```text
pricer_core/src/
├── lib.rs
├── market_data/
│   ├── mod.rs
│   ├── error.rs           → MarketDataError
│   ├── curves/
│   │   ├── mod.rs
│   │   ├── traits.rs      → YieldCurve trait
│   │   ├── flat.rs        → FlatCurve
│   │   ├── interpolated.rs → InterpolatedCurve
│   │   ├── curve_set.rs   → CurveSet (OIS, SOFR, TONAR等)
│   │   ├── curve_enum.rs  → CurveEnum
│   │   └── credit.rs      → CreditCurve trait, HazardRateCurve
│   └── surfaces/
│       ├── mod.rs
│       ├── traits.rs      → VolatilitySurface trait
│       ├── flat.rs        → FlatVolSurface
│       ├── interpolated.rs → InterpolatedVolSurface
│       └── fx.rs          → FxVolatilitySurface
├── math/
│   ├── mod.rs
│   ├── smoothing.rs       → smooth_max, smooth_indicator
│   ├── interpolators/
│   │   ├── mod.rs
│   │   ├── traits.rs
│   │   ├── linear.rs
│   │   ├── bilinear.rs
│   │   ├── cubic_spline.rs
│   │   ├── monotonic.rs
│   │   └── smooth_interp.rs
│   └── solvers/
│       ├── mod.rs
│       ├── config.rs
│       ├── newton_raphson.rs
│       ├── brent.rs
│       └── levenberg_marquardt.rs
├── traits/
│   ├── mod.rs
│   ├── priceable.rs       → Priceable trait
│   └── calibration.rs     → Calibration traits
├── types/
│   ├── mod.rs
│   ├── dual.rs            → Dual numbers (num-dual)
│   ├── time.rs            → Date, DayCountConvention
│   ├── currency.rs        → Currency enum
│   ├── currency_pair.rs   → CurrencyPair
│   └── error.rs           → 共通エラー型
```

### A.2 pricer_models (L2: 商品・モデル定義)

```text
pricer_models/src/
├── lib.rs
├── demo.rs
├── instruments/
│   ├── mod.rs             → Instrument trait, InstrumentEnum
│   ├── traits.rs          → InstrumentTrait, CashflowInstrument
│   ├── error.rs
│   ├── exercise.rs
│   ├── forward.rs
│   ├── params.rs
│   ├── payoff.rs
│   ├── swap.rs
│   ├── vanilla.rs
│   ├── equity/
│   │   └── mod.rs         → VanillaOption, etc.
│   ├── rates/
│   │   ├── mod.rs
│   │   ├── swap.rs        → InterestRateSwap
│   │   ├── swaption.rs    → Swaption
│   │   ├── capfloor.rs    → Cap, Floor
│   │   └── pricing.rs
│   ├── credit/
│   │   ├── mod.rs
│   │   ├── cds.rs         → CreditDefaultSwap
│   │   ├── pricing.rs
│   │   └── simulation.rs
│   ├── fx/
│   │   ├── mod.rs
│   │   ├── option.rs      → FxOption
│   │   └── forward.rs     → FxForward
│   ├── commodity/
│   │   └── mod.rs         → (skeleton)
│   └── exotic/
│       └── mod.rs         → (skeleton)
├── models/
│   ├── mod.rs
│   ├── stochastic.rs      → StochasticModel trait
│   ├── model_enum.rs      → StochasticModelEnum
│   ├── gbm.rs             → GeometricBrownianMotion
│   ├── heston.rs          → Heston
│   ├── sabr.rs            → SABR
│   ├── equity/
│   │   └── mod.rs
│   ├── rates/
│   │   ├── mod.rs
│   │   ├── hull_white.rs  → HullWhite (1F)
│   │   └── cir.rs         → CoxIngersollRoss
│   └── hybrid/
│       ├── mod.rs
│       └── correlated.rs  → CorrelatedModels (Cholesky)
├── schedules/
│   ├── mod.rs
│   ├── schedule.rs        → Schedule, ScheduleBuilder
│   ├── frequency.rs       → Frequency enum
│   ├── period.rs          → Period
│   └── error.rs
├── analytical/
│   ├── mod.rs
│   ├── error.rs
│   ├── distributions.rs
│   ├── black_scholes.rs
│   ├── garman_kohlhagen.rs
│   └── bachelier.rs
└── calibration/
    ├── mod.rs
    ├── model_calibrator.rs
    └── swaption_calibrator.rs
```

### A.3 pricer_optimiser (L2.5: キャリブレーション) 【NEW】

```text
pricer_optimiser/src/
├── lib.rs
├── error.rs               → OptimiserError
├── provider.rs
├── bootstrapping/
│   ├── mod.rs
│   └── curve_builder.rs   → CurveBuilder (OIS/Swap stripping)
├── calibration/
│   ├── mod.rs
│   └── engine.rs          → CalibrationEngine
└── solvers/
    ├── mod.rs
    ├── levenberg_marquardt.rs → LM法
    └── bfgs.rs            → BFGS
```

### A.4 pricer_pricing (L3: 評価エンジン)

```text
pricer_pricing/src/
├── lib.rs
├── context.rs
├── mc/
│   ├── mod.rs             → MonteCarloEngine
│   ├── config.rs          → MCConfig
│   ├── paths.rs           → PathGenerator
│   ├── payoff.rs          → Payoff trait
│   ├── pricer.rs          → MonteCarloPricer
│   ├── pricer_checkpoint.rs
│   ├── workspace.rs       → PathWorkspace
│   ├── workspace_checkpoint.rs
│   ├── thread_local.rs    → ThreadLocalWorkspacePool
│   └── error.rs
├── rng/
│   ├── mod.rs
│   ├── prng.rs            → PRNG implementations
│   ├── qmc.rs             → Sobol sequence
│   └── tests.rs
├── path_dependent/
│   ├── mod.rs
│   ├── observer.rs        → PathObserver
│   ├── payoff.rs          → PathDependentPayoff trait
│   ├── payoff_type.rs     → PathPayoffType enum
│   ├── asian.rs           → Asian averaging
│   ├── barrier.rs         → Barrier monitoring
│   └── lookback.rs        → Lookback tracking
├── analytical/
│   ├── mod.rs
│   ├── asian.rs           → Geometric Asian (Kemna-Vorst)
│   └── barrier.rs         → Barrier formulas
├── greeks/
│   ├── mod.rs
│   ├── config.rs          → GreeksConfig
│   ├── result.rs          → GreeksResult<T>
│   └── tests.rs
├── checkpoint/
│   ├── mod.rs
│   ├── budget.rs          → MemoryBudget
│   ├── strategy.rs        → CheckpointStrategy
│   ├── manager.rs
│   └── state.rs
├── pool/
│   └── mod.rs             → ThreadLocalPool
├── enzyme/
│   └── mod.rs             → Enzyme AD bindings
├── verify/
│   └── mod.rs
├── integration_tests.rs
└── verify_enzyme.rs
```

### A.5 pricer_risk (L4: リスク計算)

```text
pricer_risk/src/
├── lib.rs
├── demo.rs
├── portfolio/
│   ├── mod.rs
│   ├── builder.rs         → PortfolioBuilder
│   ├── trade.rs           → Trade, TradeBuilder
│   ├── counterparty.rs    → Counterparty, CreditParams
│   ├── netting_set.rs     → NettingSet, CollateralAgreement
│   ├── ids.rs             → TradeId, CounterpartyId, NettingSetId
│   └── error.rs           → PortfolioError
├── exposure/
│   └── mod.rs             → EE, EPE, PFE, ExposureCalculator
├── xva/
│   ├── mod.rs             → XvaCalculator
│   ├── cva.rs             → CVA calculation
│   ├── dva.rs             → DVA calculation
│   ├── fva.rs             → FVA calculation
│   ├── params.rs          → XvaConfig, FundingParams
│   ├── result.rs          → PortfolioXva, NettingSetXva
│   └── error.rs           → XvaError
├── soa/
│   ├── mod.rs
│   ├── trade_soa.rs       → TradeSoA
│   └── exposure_soa.rs    → ExposureSoA
└── parallel/
    └── mod.rs             → ParallelConfig, Rayon utilities
```

### A.6 依存関係グラフ（5層）

```text
                    ┌─────────────────────────────────────┐
                    │                                     │
                    │   pricer_risk (L4)                  │
                    │   - XVA/CVA/DVA計算                 │
                    │   - ポートフォリオ管理              │
                    │   - SoA/並列処理                    │
                    │                                     │
                    └───────────────┬─────────────────────┘
                                    │
                                    ▼
                    ┌─────────────────────────────────────┐
                    │                                     │
                    │   pricer_pricing (L3)               │
                    │   - Monte Carlo評価                 │
                    │   - Path-dependent options          │
                    │   - Greeks計算                      │
                    │   - Enzyme AD統合                   │
                    │                                     │
                    └───────────────┬─────────────────────┘
                                    │
                                    ▼
                    ┌─────────────────────────────────────┐
                    │                                     │
                    │   pricer_optimiser (L2.5) [NEW]     │
                    │   - キャリブレーション              │
                    │   - ブートストラップ                │
                    │   - 数値ソルバー                    │
                    │                                     │
                    └───────────────┬─────────────────────┘
                                    │
                                    ▼
                    ┌─────────────────────────────────────┐
                    │                                     │
                    │   pricer_models (L2)                │
                    │   - 商品定義                        │
                    │   - 確率モデル                      │
                    │   - スケジュール生成                │
                    │   - 解析解                          │
                    │                                     │
                    └───────────────┬─────────────────────┘
                                    │
                                    ▼
                    ┌─────────────────────────────────────┐
                    │                                     │
                    │   pricer_core (L1)                  │
                    │   - 市場データ (curves, surfaces)   │
                    │   - 基本型 (Currency, Date)         │
                    │   - 数学ユーティリティ              │
                    │   - 共通トレイト                    │
                    │                                     │
                    └─────────────────────────────────────┘
```

## 実装優先度

### 高優先度（残タスク）

1. **Requirement 9**: エキゾチックデリバティブ構造体の実装
2. **Requirement 10**: リスクファクター管理の実装

### 中優先度

1. **Requirement 7.5**: Wrong-Way Risk (WWR) 対応
2. **Requirement 11.5**: ベンチマーク整備

### 低優先度（将来拡張）

1. **Requirement 12**: 規制計算（SA-CCR, FRTB, SIMM）
