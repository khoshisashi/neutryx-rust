# Requirements Document

## Project Description (Input)
Phase 1: Math Foundation - Smoothing functions & Dual compatibility

## Introduction

Phase 1では、XVA価格計算ライブラリの数学的基盤を構築します。具体的には、Layer 1 (pricer_core)に自動微分対応のスムージング関数、Dual数型、および基本的なtraitを実装します。これにより、Layer 2以降で金融商品の微分可能な価格計算が可能になり、Enzymeとnum-dualの双方で動作する検証可能なシステムの基礎を確立します。

## Requirements

### Requirement 1: スムージング関数の実装

**Objective:** As a quantitative developer, I want 微分可能なスムージング関数群を使用する, so that 不連続な関数(max, if条件など)をEnzyme ADで微分可能な近似関数に置き換えることができる

#### Acceptance Criteria

1. The pricer_core shall スムージング関数モジュール(`crates/pricer_core/src/math/smoothing.rs`)を提供する
2. When epsilon値が正の浮動小数点数として与えられた場合, the pricer_core shall `smooth_max(a, b, epsilon)`関数を提供し、max(a, b)の微分可能な近似を返す
3. When epsilon値が正の浮動小数点数として与えられた場合, the pricer_core shall `smooth_min(a, b, epsilon)`関数を提供し、min(a, b)の微分可能な近似を返す
4. When 条件値xとepsilon値が与えられた場合, the pricer_core shall `smooth_indicator(x, epsilon)`関数を提供し、Heaviside関数のsigmoid近似を返す
5. When 条件値xとepsilon値が与えられた場合, the pricer_core shall `smooth_abs(x, epsilon)`関数を提供し、絶対値関数の微分可能な近似を返す
6. The pricer_core shall 全てのスムージング関数に対してジェネリック型パラメータ`T: num_traits::Float`を使用し、f32とf64の両方で動作する
7. When epsilon → 0の極限を取った場合, the pricer_core shall スムージング関数が元の不連続関数に収束することをドキュメントで保証する
8. The pricer_core shall 各スムージング関数のドキュメントコメントに数学的定義、収束性、および使用例を含める

### Requirement 2: Dual数型とnum-dual統合

**Objective:** As a quantitative developer, I want Dual数型による自動微分機能を使用する, so that Enzymeを使わずに勾配計算の正しさを検証できる

#### Acceptance Criteria

1. The pricer_core shall `num-dual`クレートを依存関係として追加する(`num-dual = "0.9"`)
2. The pricer_core shall Dual数型モジュール(`crates/pricer_core/src/types/dual.rs`)を提供する
3. When num-dualモードが有効な場合, the pricer_core shall `DualNumber`型エイリアスを`num_dual::Dual64`にマッピングする
4. The pricer_core shall 全てのスムージング関数が`DualNumber`型で動作することをテストで検証する
5. When スムージング関数にDual数が渡された場合, the pricer_core shall 勾配情報を正しく伝播し、解析的微分と一致することを保証する
6. The pricer_core shall Dual数の基本演算(加算、減算、乗算、除算、指数関数、対数関数)がnum_traitsトレイトを通じて動作することを検証する
7. If num-dualクレートが利用不可能な場合, then the pricer_core shall コンパイルエラーを発生させ、依存関係の不足を明示する

### Requirement 3: 基本的なトレイト定義

**Objective:** As a system architect, I want Layer 1の基本的な抽象化トレイトを定義する, so that Layer 2以降の価格計算ロジックが型安全に実装できる

#### Acceptance Criteria

1. The pricer_core shall トレイトモジュール(`crates/pricer_core/src/traits/mod.rs`)を提供する
2. The pricer_core shall `Priceable`トレイトを定義し、`price(&self) -> T`メソッドを要求する
3. The pricer_core shall `Differentiable`トレイトを定義し、`gradient(&self) -> T`メソッドを要求する
4. The pricer_core shall 全てのトレイトにジェネリック型パラメータ`T: num_traits::Float`を使用する
5. When トレイトが実装される場合, the pricer_core shall 静的ディスパッチ(`enum`ベース)とEnzyme最適化の互換性を保証するドキュメントを提供する
6. The pricer_core shall トレイトのドキュメントコメントに使用例、実装要件、およびLayer 2での使用パターンを含める
7. The pricer_core shall `Box<dyn Trait>`の動的ディスパッチを使用せず、静的ディスパッチのみを推奨することをドキュメントで明記する

### Requirement 4: 時間型と日付計算

**Objective:** As a quantitative developer, I want 金融計算に必要な時間型を使用する, so that 満期までの時間や年率換算を正確に計算できる

#### Acceptance Criteria

1. The pricer_core shall 時間型モジュール(`crates/pricer_core/src/types/time.rs`)を提供する
2. The pricer_core shall `chrono`クレートを依存関係として追加する(`chrono = "0.4"`)
3. When 開始日と終了日が与えられた場合, the pricer_core shall 年単位の時間差を計算する`time_to_maturity(start: NaiveDate, end: NaiveDate) -> f64`関数を提供する
4. The pricer_core shall 年率換算の標準規約(Act/365、Act/360、30/360)をサポートする列挙型`DayCountConvention`を定義する
5. When DayCountConventionが指定された場合, the pricer_core shall 対応する年率換算ロジックを適用する
6. The pricer_core shall 時間型のドキュメントに金融市場での使用例とデフォルト規約(Act/365)を記載する

### Requirement 5: モジュール構造とエクスポート

**Objective:** As a system architect, I want Layer 1のモジュール構造を明確に定義する, so that Layer 2以降が必要な機能を正しくインポートできる

#### Acceptance Criteria

1. The pricer_core shall `lib.rs`でpublicモジュールとして`math`, `traits`, `types`を公開する
2. The pricer_core shall `math`モジュール内に`smoothing`サブモジュールを配置する
3. The pricer_core shall `types`モジュール内に`dual`および`time`サブモジュールを配置する
4. When 他のクレートがpricer_coreをインポートする場合, the pricer_core shall 絶対パス(`use pricer_core::math::smoothing::smooth_max`)でアクセス可能にする
5. The pricer_core shall 各モジュールのドキュメントに目的、責任範囲、および依存関係を明記する
6. The pricer_core shall `lib.rs`にクレートレベルのドキュメントを追加し、Layer 1の役割と依存関係ゼロの原則を説明する

### Requirement 6: テスト戦略とプロパティテスト

**Objective:** As a quality engineer, I want 数学的正しさを検証するテストを実装する, so that スムージング関数とDual数の挙動が理論的予測と一致することを保証できる

#### Acceptance Criteria

1. The pricer_core shall 各スムージング関数に対してユニットテスト(`#[cfg(test)]`)を配置する
2. When epsilon = 1e-6の場合, the pricer_core shall `smooth_max`が真のmax関数から1e-3以内の誤差であることを検証する
3. When epsilon → 0の極限で複数のepsilon値をテストした場合, the pricer_core shall スムージング関数の収束性を検証する
4. The pricer_core shall `proptest`クレートを依存関係として追加する(`proptest = "1.0"`)
5. When ランダムな入力(a, b, epsilon)が与えられた場合, the pricer_core shall `smooth_max(a, b, epsilon) >= max(a, b) - tolerance`の不変条件を検証するプロパティテストを実装する
6. The pricer_core shall Dual数の勾配計算が解析的微分と一致することを数値的に検証するテストを実装する(`approx`クレート使用)
7. When テストが失敗した場合, the pricer_core shall 失敗したケースの入力値と期待値/実際値を明確に報告する

### Requirement 7: ドキュメントとコードスタイル

**Objective:** As a developer, I want 一貫したコードスタイルとドキュメントを維持する, so that コードベースの可読性と保守性を確保できる

#### Acceptance Criteria

1. The pricer_core shall `cargo fmt --all -- --check`で警告なくフォーマットされる
2. The pricer_core shall `cargo clippy --all-targets -- -D warnings`で警告なく検証される
3. When public関数やトレイトが定義される場合, the pricer_core shall Rustdocコメント(///)を含み、引数、戻り値、使用例、およびパニック条件を記載する
4. The pricer_core shall 各モジュールに`//!`形式のモジュールドキュメントを含める
5. The pricer_core shall `unsafe`コードを使用しない(Enzymeバインディングは除く、ただしLayer 1には不要)
6. When コードレビューが実施される場合, the pricer_core shall ステアリング文書で定義されたRustコーディング規約(型安全性、静的ディスパッチ)に準拠する

### Requirement 8: ビルドとCI統合

**Objective:** As a DevOps engineer, I want Layer 1が安定版Rustツールチェインでビルドできる, so that Enzymeへの依存なしに開発とテストが可能になる

#### Acceptance Criteria

1. The pricer_core shall `Cargo.toml`でRust Edition 2021を指定する
2. The pricer_core shall 安定版Rustツールチェイン(1.70以降)でビルドエラーなくコンパイルされる
3. When `cargo build -p pricer_core`が実行された場合, the pricer_core shall ナイトリーツールチェインを要求せずビルドが成功する
4. The pricer_core shall `cargo test -p pricer_core`で全てのテストが成功する
5. When CI/CDパイプラインで実行される場合, the pricer_core shall `.github/workflows/ci.yml`のstableジョブで検証される
6. The pricer_core shall `rust-toolchain.toml`でワークスペースのデフォルトツールチェイン(stable)を使用する
7. If ビルドが失敗した場合, then the pricer_core shall コンパイルエラーメッセージを明確に表示し、依存関係の不足を報告する

### Requirement 9: 依存関係の最小化

**Objective:** As a system architect, I want Layer 1の依存関係をゼロ(標準ライブラリ以外)に保つ, so that Foundation層の安定性と再利用性を最大化できる

#### Acceptance Criteria

1. The pricer_core shall 他のpricer_*クレートへの依存を持たない
2. The pricer_core shall 外部依存を最小限(num-traits, num-dual, chrono, テストクレートのみ)に制限する
3. When 新しい依存が追加される場合, the pricer_core shall その必要性をドキュメントで正当化する
4. The pricer_core shall `cargo tree -p pricer_core`で依存関係ツリーを検証し、不要な推移的依存を排除する
5. The pricer_core shall ステアリング文書で定義された4層アーキテクチャ原則(L1は依存ゼロ)に準拠する

### Requirement 10: パフォーマンスとメモリ効率

**Objective:** As a performance engineer, I want スムージング関数が最適化可能である, so that リリースビルドでゼロコスト抽象化を実現できる

#### Acceptance Criteria

1. The pricer_core shall 全てのスムージング関数に`#[inline]`属性を適用する
2. When リリースモード(`--release`)でビルドされた場合, the pricer_core shall LLVMによる最適化が可能な形式(インライン展開、SIMD化)を保証する
3. The pricer_core shall 動的メモリ割り当て(Box, Vec)をホットパスで使用しない
4. When スムージング関数が呼び出される場合, the pricer_core shall スタック上の演算のみで完結する
5. The pricer_core shall ベンチマーク用のcriterionクレートを追加し、スムージング関数のパフォーマンス基準を確立する準備をする
6. The pricer_core shall `Cargo.toml`のリリースプロファイルでLTO(Link-Time Optimization)とcodegen-units=1を使用する(ワークスペースレベルで設定済み)

