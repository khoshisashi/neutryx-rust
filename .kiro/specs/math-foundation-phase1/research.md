# Research & Design Decisions

---
**Purpose**: Phase 1数学的基盤の発見調査と設計判断の根拠を記録

**Usage**:
- 発見フェーズでの調査活動と成果を記録
- `design.md`に記載するには詳細すぎる設計判断のトレードオフを文書化
- 将来の監査や再利用のための参照と証拠を提供
---

## Summary
- **Feature**: `math-foundation-phase1`
- **Discovery Scope**: 新規機能 (Greenfield - Layer 1基盤の初期実装)
- **Key Findings**:
  - num-dual 0.13.1は安定版Rustでの自動微分を提供し、Enzymeの検証バックエンドとして最適
  - LogSumExp/Boltzmann演算子による微分可能なスムージング関数の数学的基盤を確立
  - num-traitsのFloat traitは514.9M以上のダウンロード実績があり、ジェネリック数値計算の業界標準
  - Rust量子金融ライブラリ(finql, RustQuant)がDay Count Conventionの実装パターンを提供

## Research Log

### Topic: 微分可能なスムージング関数の数学的定式化

- **Context**: Requirement 1で要求されるsmooth_max, smooth_min, smooth_indicator, smooth_absの数学的基盤を調査
- **Sources Consulted**:
  - [Smooth maximum - HandWiki](https://handwiki.org/wiki/Smooth_maximum)
  - [LogSumExp - Wikipedia](https://en.wikipedia.org/wiki/LogSumExp)
  - [Sigmoid functions for smooth approximation](https://www.preprints.org/manuscript/201903.0140/v1/download)
- **Findings**:
  - **LogSumExp (LSE)**: `LSE_α(x₁,…,xₙ) = (1/α) log Σᵢ₌₁ⁿ exp(αxᵢ)` - 滑らかな最大値近似の標準的手法
  - **Boltzmann演算子**: `S_α(x₁,…,xₙ) = Σᵢ₌₁ⁿ xᵢe^(αxᵢ) / Σᵢ₌₁ⁿ e^(αxᵢ)` - 重み付き平均による近似
  - **収束性**: α → ∞で真の最大値に収束、α = 0で算術平均、α → -∞で最小値に収束
  - **Sigmoid関数**: Heaviside関数の近似に使用 `σ(x/ε) = 1/(1 + exp(-x/ε))`
  - **Softplus**: 絶対値関数の近似に使用 `softplus(x) = log(1 + exp(x))`
- **Implications**:
  - epsilon (ε) パラメータは α = 1/ε と対応し、ユーザーが直感的に調整可能
  - 2引数版smooth_maxは `smooth_max(a, b, ε) = ε * log(exp(a/ε) + exp(b/ε))` として実装
  - 全てのスムージング関数は`#[inline]`属性によりLLVM最適化が可能
  - Enzymeは滑らかな関数に対して自動微分を実行し、勾配伝播が保証される

### Topic: num-dualクレートの機能と互換性

- **Context**: Requirement 2のDual数型統合とnum-dualバックエンドの技術的妥当性を検証
- **Sources Consulted**:
  - [num-dual - Rust docs.rs](https://docs.rs/num-dual)
  - [GitHub - itt-ustutt/num-dual](https://github.com/itt-ustutt/num-dual)
- **Findings**:
  - **最新バージョン**: 0.13.1 (2025年時点で活発にメンテナンス中)
  - **サポートされるDual数型**:
    - スカラー: `Dual` (1次微分), `Dual2` (2次微分), `Dual3` (3次微分)
    - ベクトル: `DualVec` (勾配/ヤコビアン), `Dual2Vec` (ヘッセ行列)
    - ハイパーDual: `HyperDual` (2次偏微分), `HyperHyperDual` (3次偏微分)
  - **コアトレイト**:
    - `DualNum<F>`: 一般化されたDual数演算
    - `DualNumFloat`: 基礎となる浮動小数点型 (f32/f64)
  - **num-traits互換性**: `num-traits ^0.2`に依存し、Floatトレイトとシームレスに統合
  - **ライセンス**: MIT OR Apache-2.0のデュアルライセンス
- **Implications**:
  - Phase 1では`Dual`型 (1次微分) のみを使用し、将来のフェーズで高次微分を拡張可能
  - `num_dual::Dual64`をpricer_coreの`DualNumber`型エイリアスとして公開
  - 全てのスムージング関数はジェネリック型 `T: num_traits::Float` で定義し、Dual数でも動作
  - Enzymeとnum-dualの両方でテストすることで、正しさを二重検証

### Topic: num-traitsを使用したジェネリック数値計算

- **Context**: Requirement 1,2,3で要求されるジェネリック型パラメータ`T: num_traits::Float`の実装ベストプラクティス
- **Sources Consulted**:
  - [num-traits Float trait - docs.rs](https://docs.rs/num-traits/latest/num_traits/float/trait.Float.html)
  - [Num traits Rust Guide 2025](https://generalistprogrammer.com/tutorials/num-traits-rust-crate-guide)
  - [Generic Numeric Computations in Rust](https://michaelmauderer.com/blog/generic-numeric-computations/)
- **Findings**:
  - **ダウンロード実績**: 514.9M総ダウンロード、66.2M最近のダウンロード - Rustエコシステムで広く使用
  - **Float trait**: 算術演算、比較演算子、数学関数 (sin, cos, exp, powf等)、定数を提供
  - **FloatCore trait**: `no_std`環境で使用可能、Floatトレイトのサブセット
  - **Copy要件**: 数値型はCopyトレイトを実装する必要があり、f32/f64はどちらもCopy
  - **dyn非互換**: Floatトレイトはdyn互換ではない (古いバージョンでは"object safety"と呼ばれていた)
  - **MSRV**: rustc 1.60以上でテスト済み、最新バージョンは0.2.19
- **Implications**:
  - `T: num_traits::Float`を使用することで、手動でトレイト実装する必要がなくなる
  - f32とf64の両方で動作し、将来的に他の浮動小数点型 (例: f128) にも拡張可能
  - 静的ディスパッチ (enum) を使用し、動的ディスパッチ (Box<dyn Trait>) を避ける
  - no_std環境への将来的な対応も可能 (Phase 1ではstd前提)

### Topic: 金融計算のためのDay Count Convention

- **Context**: Requirement 4の時間型と日付計算、特にDay Count Conventionの実装パターンを調査
- **Sources Consulted**:
  - [finql - Rust docs.rs](https://docs.rs/finql/latest/finql/)
  - [Day count convention - Wikipedia](https://en.wikipedia.org/wiki/Day_count_convention)
  - [GitHub - xemwebe/finql](https://github.com/xemwebe/finql)
- **Findings**:
  - **finqlライブラリ**: "典型的なDay Count Convention手法による年率換算の計算"をサポート
  - **標準的な規約**:
    - **Act/365 Fixed**: 実際の日数 / 365 (英国債、多くのデリバティブで標準)
    - **Act/360**: 実際の日数 / 360 (マネーマーケット商品で一般的)
    - **30/360**: 各月を30日、1年を360日として計算 (米国社債で一般的)
    - **Act/Act ISDA**: 閏年と平年を区別して計算 (スワップ、多くの債券で使用)
  - **chrono統合**: finqlはchrono::NaiveDateを使用して日付計算を実装
- **Implications**:
  - Phase 1では基本的な3つの規約 (Act/365, Act/360, 30/360) を実装
  - `DayCountConvention` enum型を定義し、将来的に追加の規約を拡張可能
  - `time_to_maturity(start: NaiveDate, end: NaiveDate) -> f64`関数でデフォルトをAct/365に設定
  - chronoクレート (バージョン 0.4) を依存関係として追加

### Topic: Rustプロジェクトのテスト戦略

- **Context**: Requirement 6のプロパティテストとユニットテストの実装パターンを調査
- **Sources Consulted**:
  - [proptest crate](https://crates.io/crates/proptest)
  - [approx crate](https://crates.io/crates/approx)
  - [Rust testing best practices](https://doc.rust-lang.org/book/ch11-00-testing.html)
- **Findings**:
  - **proptest**: ランダム入力生成による不変条件の検証、数学的性質のテストに最適
  - **approx**: 浮動小数点数の近似比較マクロ (`assert_relative_eq!`, `assert_abs_diff_eq!`)
  - **criterion**: パフォーマンスベンチマーク、回帰検出
  - **テスト配置**: `#[cfg(test)]`モジュールを実装と同じファイルに配置 (Rustの慣例)
- **Implications**:
  - スムージング関数の収束性をproptest でテスト (例: `smooth_max(a, b, ε) >= max(a, b) - tolerance`)
  - Dual数の勾配計算をapproxで数値的に検証 (解析的微分との一致)
  - criterionをdev-dependencyとして追加し、将来のパフォーマンステストに備える

## Architecture Pattern Evaluation

| Option | Description | Strengths | Risks / Limitations | Notes |
|--------|-------------|-----------|---------------------|-------|
| Pure Library (選択) | Layer 1を純粋な数学ライブラリとして実装、他のpricer_*クレートへの依存なし | 明確な境界、再利用性が高い、テストが容易、安定版Rustのみ使用 | なし (Layer 1の責任範囲として最適) | ステアリング文書の4層アーキテクチャ原則に準拠 |
| Monolithic Crate | 全ての機能を単一のクレートに統合 | シンプルな依存関係ツリー | Layer 3のEnzyme依存が全体に波及し、安定版/ナイトリー分離が不可能 | 却下: 4層分離の原則に違反 |
| Feature Flags | num-dual-mode とenzyme-modeをフィーチャーフラグで切り替え | 柔軟性が高い、条件付きコンパイルが可能 | 複雑性が増す、テストマトリックスが拡大 | 検討中: Layer 1でのデュアルモードサポートに使用可能 |

## Design Decisions

### Decision: スムージング関数の数学的定式化にLogSumExpを採用

- **Context**: smooth_maxとsmooth_minの実装手法を選定
- **Alternatives Considered**:
  1. **Softmax/LogSumExp** - 指数関数ベースの加重平均、収束性が証明されている
  2. **Polynomial approximation** - 多項式による近似、計算コストは低いが収束性が弱い
  3. **Sigmoid-based blending** - シグモイド関数による線形補間、直感的だが数学的基盤が弱い
- **Selected Approach**: LogSumExp (Boltzmann演算子)
  - `smooth_max(a, b, ε) = ε * log(exp(a/ε) + exp(b/ε))`
  - `smooth_min(a, b, ε) = -smooth_max(-a, -b, ε)`
- **Rationale**:
  - 数学的に厳密な収束性 (ε → 0で真の最大値/最小値に収束)
  - 機械学習分野で広く使用され、実装パターンが確立されている
  - Enzyme ADとの互換性が高い (指数関数と対数関数の微分は標準サポート)
- **Trade-offs**:
  - **Benefits**: 数学的厳密性、将来の拡張性 (n引数版への拡張が容易)
  - **Compromises**: 指数関数のオーバーフロー/アンダーフローのリスク (数値安定性の対策が必要)
- **Follow-up**: 実装時に数値安定性のためのmax値のシフト (`log-sum-exp trick`) を検討

### Decision: num-dual 0.9を採用し、将来0.13へのアップグレードパスを確保

- **Context**: Requirement 2でnum-dual依存を追加する際のバージョン選定
- **Alternatives Considered**:
  1. **num-dual 0.13.1** (最新版) - 最新機能を利用可能、将来的に安定
  2. **num-dual 0.9** - より保守的なバージョン、広く使用されている
  3. **dual_num** - 代替クレート、最小メンテナンスモード
- **Selected Approach**: num-dual 0.9をCargo.tomlで指定、要件文書では`num-dual = "0.9"`と記載
- **Rationale**:
  - 要件文書 (requirements.md) で既に0.9が指定されている
  - 0.9と0.13の間に破壊的変更がある可能性があるため、実装フェーズで検証が必要
  - num-dualの活発なメンテナンスにより、将来的なアップグレードパスが確保されている
- **Trade-offs**:
  - **Benefits**: 安定性重視、既存の要件との整合性
  - **Compromises**: 最新機能 (Dual3, HyperHyperDual等) は0.13にアップグレード後に利用可能
- **Follow-up**: 実装時に0.9のAPIドキュメントを確認し、Phase 2以降で0.13へのアップグレードを検討

### Decision: Day Count Conventionの初期実装は3種類に限定

- **Context**: Requirement 4で金融計算に必要なDay Count Convention規約を実装
- **Alternatives Considered**:
  1. **最小セット (Act/365のみ)** - シンプルだが柔軟性が低い
  2. **標準セット (Act/365, Act/360, 30/360)** - 主要な用途をカバー、拡張可能
  3. **完全セット (Act/Act ISDA, Act/365L等も含む)** - 全ての金融商品に対応、複雑性が高い
- **Selected Approach**: 標準セット (Act/365, Act/360, 30/360)
- **Rationale**:
  - Act/365: デリバティブ、英国債で標準
  - Act/360: マネーマーケット商品で一般的
  - 30/360: 米国社債で一般的
  - この3つで大多数の用途をカバー、enumへの追加は将来容易
- **Trade-offs**:
  - **Benefits**: シンプルさと実用性のバランス、テストケースが管理可能
  - **Compromises**: Act/Act ISDAなどの高度な規約は将来のフェーズで追加
- **Follow-up**: DayCountConvention enumに`#[non_exhaustive]`属性を適用し、将来の拡張時に破壊的変更を回避

### Decision: 静的ディスパッチ (enum) を優先し、動的ディスパッチ (trait object) を禁止

- **Context**: Requirement 3のトレイト定義とステアリング文書の型安全性原則
- **Alternatives Considered**:
  1. **静的ディスパッチ (enum)** - コンパイル時に型が確定、Enzyme最適化が可能
  2. **動的ディスパッチ (Box<dyn Trait>)** - 実行時の柔軟性が高いが、Enzymeと非互換
- **Selected Approach**: 静的ディスパッチのみを推奨、`Box<dyn Trait>`をドキュメントで明示的に禁止
- **Rationale**:
  - Enzymeは具体的な型に対してLLVMレベルで最適化を実行するため、trait objectでは機能しない
  - ステアリング文書 (tech.md) で静的ディスパッチが推奨されている
  - num-traitsのFloatトレイトもdyn非互換であり、設計全体と整合性がある
- **Trade-offs**:
  - **Benefits**: 最大限のパフォーマンス、Enzymeとの完全な互換性、ゼロコスト抽象化
  - **Compromises**: Layer 2で`enum Instrument`を使用する必要があるが、これはステアリング文書で既に計画済み
- **Follow-up**: トレイトのドキュメントに静的ディスパッチの要件と例を明記

## Risks & Mitigations

- **Risk 1: スムージング関数の数値安定性** - 指数関数のオーバーフロー/アンダーフロー
  - **Mitigation**: LogSumExpの数値安定版実装 (log-sum-exp trick) を使用、max値をシフトして計算
- **Risk 2: num-dual 0.9と0.13の互換性** - バージョン間の破壊的変更
  - **Mitigation**: 実装フェーズで0.9のAPIドキュメントを確認、将来のアップグレードパスを文書化
- **Risk 3: chronoクレートのMSRV (最小サポートRustバージョン)** - 安定版Rustとの互換性
  - **Mitigation**: chrono 0.4の最新パッチバージョンを使用、Cargo.tomlでMSRVを検証
- **Risk 4: epsilonパラメータの選定** - ユーザーが不適切なepsilon値を使用
  - **Mitigation**: ドキュメントに推奨範囲 (1e-8 ~ 1e-3) と収束性のトレードオフを記載
- **Risk 5: プロパティテストの失敗率** - ランダム入力による偽陽性/偽陰性
  - **Mitigation**: proptestの試行回数 (cases) を適切に設定、再現可能なseedを使用

## References

### Mathematical Foundations
- [Smooth maximum - HandWiki](https://handwiki.org/wiki/Smooth_maximum) - LogSumExp/Boltzmann演算子の数学的定義
- [LogSumExp - Wikipedia](https://en.wikipedia.org/wiki/LogSumExp) - 数値安定性とlog-sum-exp trick
- [Sigmoid functions for smooth approximation](https://www.preprints.org/manuscript/201903.0140/v1/download) - 絶対値関数のsigmoid近似

### Rust Crates and Documentation
- [num-dual - docs.rs](https://docs.rs/num-dual) - Dual数型の公式ドキュメント
- [num-traits Float trait](https://docs.rs/num-traits/latest/num_traits/float/trait.Float.html) - Floatトレイトのリファレンス
- [chrono - GitHub](https://github.com/chronotope/chrono) - 日付時刻ライブラリ
- [proptest - crates.io](https://crates.io/crates/proptest) - プロパティベーステスト
- [approx - crates.io](https://crates.io/crates/approx) - 浮動小数点近似比較

### Financial Calculations
- [finql - docs.rs](https://docs.rs/finql/latest/finql/) - Rust量子金融ライブラリ
- [Day count convention - Wikipedia](https://en.wikipedia.org/wiki/Day_count_convention) - 金融規約の標準的定義

### Best Practices
- [Num traits Rust Guide 2025](https://generalistprogrammer.com/tutorials/num-traits-rust-crate-guide) - num-traitsの使用ガイド
- [Generic Numeric Computations in Rust](https://michaelmauderer.com/blog/generic-numeric-computations/) - ジェネリック数値計算のベストプラクティス
