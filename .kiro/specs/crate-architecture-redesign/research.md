# Research & Design Decisions: crate-architecture-redesign

## Summary

- **Feature**: `crate-architecture-redesign`
- **Discovery Scope**: Complex Integration（既存システムの大規模再構成）
- **Key Findings**:
  - Enum Dispatchパターンは静的ディスパッチで10倍のパフォーマンス向上、Enzyme AD互換性に必須
  - SOFR/OISマルチカーブフレームワークが現代の金利デリバティブ評価の標準
  - Hull-White 1Fモデルはxva計算の業界標準、mean-reversionパラメータの適切な選択が精度に重要
  - Longstaff-Schwartz法は3-5個のLaguerre多項式基底関数で実用的な精度を達成

## Research Log

### Enum Dispatch vs Trait Objects

- **Context**: Enzyme ADとの互換性を維持しながら多様な金融商品・モデルを扱う多態性の実現方法
- **Sources Consulted**:
  - [enum_dispatch crate](https://docs.rs/enum_dispatch/latest/enum_dispatch/)
  - [Rust Dispatch Explained](https://www.somethingsblog.com/2025/04/20/rust-dispatch-explained-when-enums-beat-dyn-trait/)
  - [Rust Polymorphism Guide](https://www.possiblerust.com/guide/enum-or-trait-object)
- **Findings**:
  - Enum dispatchはtrait objectsと比較して最大10倍のパフォーマンス向上
  - コンパイラが最適化（インライン化）を適用可能、vtableルックアップ不要
  - 「Closed World」前提: 全variant型がコンパイル時に既知である必要
  - コード膨張のリスクあり（モノモーフィゼーション）
- **Implications**:
  - 現行の`Instrument<T>` enum、`StochasticModelEnum`パターンを継続
  - アセットクラス別にサブenumを定義し、トップレベルenumでディスパッチ
  - Enzyme互換性のため、全商品・モデルでenum dispatch維持必須

### マルチカーブフレームワーク（SOFR/OIS）

- **Context**: 金利デリバティブ評価に必要なマルチカーブ基盤の設計
- **Sources Consulted**:
  - [CME SOFR Derivatives Pricing](https://www.cmegroup.com/articles/2025/price-and-hedging-usd-sofr-interest-swaps-with-sofr-futures.html)
  - [SOFR Discount - ScienceDirect](https://www.sciencedirect.com/science/article/pii/S0304405X24002125)
  - [Quantifi Curve Construction](https://www.quantifisolutions.com/tackling-interest-rate-curve-construction-complexity/)
- **Findings**:
  - LIBOR廃止後、SOFR OISカーブがUSDデリバティブのディスカウント標準
  - デュアルカーブディスカウント: フォワードレート予測用とディスカウント用で別カーブ
  - SOFRカーブ構築の複雑性: 日次平均、遡及的支払い、幾何的複利
  - 短期はデポジットレート、長期はスワップレートでブートストラップ
- **Implications**:
  - `CurveSet`構造体で名前付きカーブ管理（"OIS", "SOFR", "TONAR"等）
  - 各商品でディスカウントカーブとフォワードカーブを分離指定可能に
  - ブートストラップアルゴリズムの実装（将来拡張）

### Hull-White 1Fモデルとキャリブレーション

- **Context**: 金利モデルの実装とxva計算への適用
- **Sources Consulted**:
  - [Hull-White Wikipedia](https://en.wikipedia.org/wiki/Hull%E2%80%93White_model)
  - [S&P Global Hull-White for xVA](https://www.spglobal.com/marketintelligence/en/mi/research-analysis/xva-modeling-squeezing-accuracy-from-the-industry-standard-hul.html)
  - [KTH Calibration Methods](https://people.kth.se/~aaurell/Teaching/SF2975_HT17/calibration-hull-white.pdf)
- **Findings**:
  - Hull-White 1FはxVA計算の業界標準モデル
  - パラメータ: mean-reversion (α)、short rate volatility (σ)、θ(初期イールドカーブから計算)
  - σはATM co-terminal swaptionにキャリブレーション
  - mean-reversionパラメータはswaption volatility surfaceの形状に大きく影響
  - xVAエクスポージャー計算にはChevron形状のswaption選択が有効
- **Implications**:
  - Hull-White 1Fを最初の金利モデルとして実装
  - `Calibrator` traitでswaption volatility surfaceへのキャリブレーション
  - mean-reversionは設定可能パラメータ、σは時間依存piece-wise constant

### Longstaff-Schwartz法（Bermudan/American Options）

- **Context**: Bermudan Swaptionの早期行使境界推定
- **Sources Consulted**:
  - [Original Paper](https://people.math.ethz.ch/~hjfurrer/teaching/LongstaffSchwartzAmericanOptionsLeastSquareMonteCarlo.pdf)
  - [Oxford Advanced MC](http://www2.maths.ox.ac.uk/~gilesm/mc/module_6/american.pdf)
  - [CRAN LSMRealOptions](https://cran.r-project.org/web/packages/LSMRealOptions/vignettes/LSMRealOptions.html)
- **Findings**:
  - 最小二乗法で継続価値の条件付き期待値を推定
  - In-the-moneyパスのみを回帰に使用（効率向上）
  - 基底関数: Laguerre多項式、3-5個で実用的精度
  - バイアス考慮: 決定用パスと評価用パスを分離推奨
  - 50,000パス程度で収束
- **Implications**:
  - `pricer_engine/american/lsm.rs`でLongstaff-Schwartz実装
  - 基底関数は`BasisFunction` enumで選択可能（Polynomial, Laguerre, Hermite）
  - 2セットパス方式でバイアス低減オプション

## Architecture Pattern Evaluation

| Option | Description | Strengths | Risks / Limitations | Notes |
|--------|-------------|-----------|---------------------|-------|
| Enum Dispatch継続 | 既存パターン維持、アセットクラス別サブenum | 10x性能、Enzyme互換、コンパイル時検証 | variant数増加でコンパイル時間増 | **採用** - 既存パターン、AD必須要件 |
| Trait Objects | `Box<dyn Instrument>`で拡張性 | オープン拡張、コード簡潔 | 10x性能低下、Enzyme非互換 | 却下 - AD互換性不可 |
| Hybrid (enum + trait) | 基本はenum、拡張点でtrait | 柔軟性と性能のバランス | 複雑性増加、境界設計難 | 将来検討 - 現時点では不要 |

## Design Decisions

### Decision: クレート名変更（kernel→engine, xva→risk）

- **Context**: 役割ベースの一貫した命名規則の確立
- **Alternatives Considered**:
  1. 現状維持（kernel, xva）— 変更コストゼロだが、xvaだけが製品名
  2. 役割ベース（engine, risk）— 一貫性あり
  3. 機能ベース（compute, analytics）— 抽象的すぎる
- **Selected Approach**: Option 2 - `pricer_kernel` → `pricer_engine`、`pricer_xva` → `pricer_risk`
- **Rationale**: core/models/engine/riskで責務が明確、xvaはriskの一部機能
- **Trade-offs**: 全参照更新必要、既存ユーザーへの影響
- **Follow-up**: Cargo.toml更新、`pub use`でエイリアス提供（deprecation警告付き）

### Decision: アセットクラス別サブモジュール構成

- **Context**: 商品とモデルの整理方法
- **Alternatives Considered**:
  1. Flat構造（現状）— シンプルだが探しにくい
  2. アセットクラス別（equity/, rates/, credit/等）— 明確な分類
  3. 商品タイプ別（options/, swaps/, forwards/）— アセット横断だが混在
- **Selected Approach**: Option 2 - アセットクラス別サブモジュール
- **Rationale**: 金融業界の標準分類、チーム分担に適合
- **Trade-offs**: ファイル数増加、一部商品の分類が曖昧（Quantoは exotic? fx?）
- **Follow-up**: Quantoはexotic配下、FX関連はfx配下でクロスリファレンス

### Decision: Instrument trait追加

- **Context**: 共通インターフェースの必要性（Req 1.3）
- **Alternatives Considered**:
  1. Enum methodsのみ（現状）— シンプルだがポリモーフィズム限定的
  2. Instrument trait追加 — 共通契約定義、将来の拡張性
  3. 複数trait（Priceable, Hedgeable等）— 細粒度だが複雑
- **Selected Approach**: Option 2 - 単一`Instrument` trait
- **Rationale**: price(), greeks(), cashflows()の共通契約、enumでの実装
- **Trade-offs**: trait定義の追加作業
- **Follow-up**: trait定義はpricer_models/instruments/traits.rsに配置

### Decision: CurveSetの設計

- **Context**: マルチカーブ管理の実装方法（Req 2）
- **Alternatives Considered**:
  1. HashMap<String, Box<dyn YieldCurve>>— 動的だがAD非互換
  2. CurveSet struct with named fields— 静的だが固定
  3. CurveSet<T> with HashMap<CurveName, CurveEnum<T>>— 名前付き + enum dispatch
- **Selected Approach**: Option 3 - `CurveSet<T: Float>`構造体 + `CurveName` enum + `CurveEnum<T>`
- **Rationale**: AD互換性維持、名前付き管理、静的ディスパッチ
- **Trade-offs**: 新カーブ追加時にCurveEnum更新必要
- **Follow-up**: CurveName enumは"OIS", "SOFR", "Forward", "Discount"等を定義

### Decision: Feature Flag粒度

- **Context**: 条件付きコンパイルの粒度（Req 7.4）
- **Alternatives Considered**:
  1. クレート単位— 粗すぎる
  2. アセットクラス単位（"rates", "credit", "fx"）— 適切な粒度
  3. 商品単位（"irs", "cds", "fxoption"）— 細かすぎる
- **Selected Approach**: Option 2 - アセットクラス単位feature flag
- **Rationale**: 依存関係管理が容易、コンパイル時間の有意な削減可能
- **Trade-offs**: 特定商品のみ除外は不可
- **Follow-up**: default = ["equity"], optional = ["rates", "credit", "fx", "commodity", "exotic"]

## Risks & Mitigations

| リスク | 影響度 | 発生確率 | 緩和策 |
|--------|--------|----------|--------|
| クレート名変更による既存ユーザー影響 | High | Medium | `pub use`エイリアス + deprecation警告で移行期間提供 |
| LMM実装の複雑性 | High | High | Phase 1ではHull-White 1Fのみ、LMMは将来拡張 |
| Enzyme互換性の確認不足 | High | Medium | 各モデル・商品追加時にenzyme-modeでテスト |
| スケジュール生成のエッジケース | Medium | Medium | chrono依存、IMM日付のみ初期実装、カレンダーは将来拡張 |
| enum variant数増加によるコンパイル時間増 | Medium | High | feature flagでアセットクラス別分離、必要なもののみ有効化 |
| 後方互換性の破壊 | High | Medium | 主要APIは維持、内部構造のみ変更、セマンティックバージョニング |

## References

- [enum_dispatch - Rust](https://docs.rs/enum_dispatch/latest/enum_dispatch/) — Enum dispatch性能ベンチマーク
- [CME SOFR Derivatives Pricing](https://www.cmegroup.com/articles/2025/price-and-hedging-usd-sofr-interest-swaps-with-sofr-futures.html) — SOFRスワップ評価
- [S&P Global Hull-White for xVA](https://www.spglobal.com/marketintelligence/en/mi/research-analysis/xva-modeling-squeezing-accuracy-from-the-industry-standard-hul.html) — Hull-White xVA適用
- [Longstaff-Schwartz Original Paper](https://people.math.ethz.ch/~hjfurrer/teaching/LongstaffSchwartzAmericanOptionsLeastSquareMonteCarlo.pdf) — LSM法の原論文
- [Oxford Advanced MC Methods](http://www2.maths.ox.ac.uk/~gilesm/mc/module_6/american.pdf) — American option MC
