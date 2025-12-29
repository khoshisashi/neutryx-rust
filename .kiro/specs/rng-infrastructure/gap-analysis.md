# Implementation Gap Analysis: RNG Infrastructure

## 分析概要

本ギャップ分析は、Phase 3.1aにおける乱数生成インフラ(`crates/pricer_kernel/src/rng/`)の実装について、要件と既存コードベースの現状を比較し、実装戦略の選択肢を提示する。

### スコープ
- **対象**: `crates/pricer_kernel/src/rng/` モジュール(新規作成)
- **要件数**: 8要件、42受入基準
- **依存関係**: `rand`, `rand_distr`のみ(pricer_core依存なし)

### 主な発見事項
- ✅ `rng/`ディレクトリは未作成(グリーンフィールド実装)
- ✅ `rand`, `rand_distr`は既にworkspace依存関係として定義済み
- ✅ `pricer_kernel`は現在Phase 3.0(完全独立状態)で、pricer_coreへの依存なし
- ✅ 既存の`verify/`モジュールがドキュメント・テスト・British Englishパターンの参考実装として利用可能
- ⚠️ Enzymeとの将来的な統合を考慮した設計が必要

---

## 1. 現状調査 (Current State Investigation)

### 1.1 既存アセット

#### pricer_kernel の現状
**構造**: `crates/pricer_kernel/src/`
```
lib.rs          → モジュール宣言、ドキュメント(Phase 3.0 完全独立)
verify/mod.rs   → Enzyme検証ユーティリティ(プレースホルダー実装)
enzyme/mod.rs   → Enzymeバインディング(プレースホルダー)
mc/mod.rs       → Monte Carloカーネル(プレースホルダー)
checkpoint/mod.rs → メモリ管理(プレースホルダー)
```

**重要な発見**:
- Phase 3.0は`pricer_core`への依存を**完全に排除**している
- 現在アクティブなのは`verify`モジュールのみ(`lib.rs`で公開)
- `mc`, `enzyme`, `checkpoint`はコメントアウト(Phase 4で有効化予定)

#### 依存関係の現状
**Cargo.toml** (`pricer_kernel`):
```toml
[dependencies]
llvm-sys = "180"          # Enzyme用LLVM 18バインディング
num-traits.workspace = true # 基本的な数値トレイト

[dev-dependencies]
approx.workspace = true    # テスト用近似比較
```

**Workspace依存関係** (ルート`Cargo.toml`):
```toml
rand = "0.8"          # ✅ 既に定義済み
rand_distr = "0.4"    # ✅ 既に定義済み
proptest = "1.6"      # ✅ プロパティベーステスト用
criterion = "0.5"     # ベンチマーク用(オプション)
```

### 1.2 既存パターンと規約

#### ドキュメントパターン (`verify/mod.rs`からの抽出)
```rust
//! Module-level documentation with purpose, usage examples
//!
//! ## Purpose
//! - Bullet points explaining module role
//!
//! ## Usage
//! ```rust
//! // Example code
//! ```

/// Function-level rustdoc
///
/// # Mathematical Definition
/// ```text
/// Mathematical notation
/// ```
///
/// # Arguments
/// * `param` - Description
///
/// # Returns
/// Return value description
///
/// # Examples
/// ```rust
/// // Example code
/// ```
#[inline]
pub fn example() { }
```

#### テストパターン
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_basic_functionality() {
        // Test implementation
    }
}
```

**Property-based testing** (参考: `pricer_core/src/math/smoothing.rs`):
```rust
#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1000))]

        #[test]
        fn test_statistical_property(value in -100.0..100.0_f64) {
            // Property verification
        }
    }
}
```

#### British English規約
- **既存の用例確認**: `verify/mod.rs`、`lib.rs`は米国英語を使用
- **要件**: 本実装ではBritish English統一が必須
  - `initialize` → `initialise`
  - `randomize` → `randomise`
  - `behavior` → `behaviour`

### 1.3 統合ポイント

#### Layer 3アーキテクチャとの整合性
- **依存関係制約**: `pricer_core`依存**禁止**(Phase 3.0原則)
- **ツールチェーン**: nightly Rust(`nightly-2025-01-15`)必須
- **モジュール公開**: `lib.rs`で`pub mod rng;`として公開必要

#### 将来的な統合(Phase 4以降)
- **Monte Carloカーネル**: `mc/`モジュールがRNGを消費する主要顧客
- **Enzyme互換性**: 静的ディスパッチ、固定サイズループ、非微分可能境界の明示

---

## 2. 要件実現可能性分析 (Requirements Feasibility Analysis)

### 2.1 技術的ニーズ抽出

| 要件 | 技術的ニーズ | 既存アセット | ギャップ |
|------|------------|------------|--------|
| Req 1: PRNGラッパー | `rand::Rng`ラッパー構造体、シード管理、バッチ生成 | `rand`依存済み | **Missing**: 構造体定義、シード管理ロジック |
| Req 2: 正規分布生成 | Box-Muller/Ziggurat実装、性能最適化 | `rand_distr`依存済み | **Missing**: アルゴリズム選択、最適化実装 |
| Req 3: Sobolプレースホルダー | トレイト定義、スタブ実装 | なし | **Missing**: QMCトレイト設計 |
| Req 4: ベクトル効率性 | `&mut [f64]`スライス操作、in-place生成 | なし | **Missing**: スライスベースAPI |
| Req 5: モジュール独立性 | `pricer_core`非依存、`rand`のみ使用 | ✅ 現状Phase 3.0で達成済み | **No Gap**: 既存方針に完全準拠 |
| Req 6: テスト容易性 | プロパティテスト、統計検証 | `proptest`依存済み、`approx`利用可能 | **Missing**: 統計テストユーティリティ |
| Req 7: British English | コメント・ドキュメント規約 | 既存は米国英語 | **Constraint**: 新規実装で規約統一 |
| Req 8: Enzyme互換性 | 静的ディスパッチ、固定ループ | `verify/`の設計パターン参考可能 | **Missing**: Enzyme設計ガイドライン文書化 |

### 2.2 ギャップと制約の識別

#### Missing Capabilities
1. **RNGラッパー構造体**: `rand::StdRng`や`rand::rngs::SmallRng`をラップする独自構造体
2. **正規分布アルゴリズム**: `rand_distr::Normal`を使用するか、Box-Muller/Ziggurat独自実装
3. **Sobol QMCトレイト**: 将来的な実装のためのインターフェース設計
4. **統計テストユーティリティ**: 平均・分散検証、chi-squared検定

#### Research Needed
1. **Box-Muller vs Ziggurat性能比較**: `rand_distr`の内部実装がZiggurat使用か確認必要
   - → **Action**: `rand_distr::Normal`のソースコード調査、ベンチマーク比較
2. **Sobol次元数**: QMCシーケンスの最大次元数設計(32次元? 64次元?)
   - → **Action**: Monte Carlo用途(GBM, Hestonモデル)の次元要件調査
3. **キャッシュ最適化バッファサイズ**: 推奨バッファサイズの決定
   - → **Action**: L1/L2/L3キャッシュサイズを考慮した実験的決定

#### Constraints
1. **British English統一**: 既存コードは米国英語だが、本モジュールはBritish English必須
2. **Enzyme静的最適化**: 動的ディスパッチ(`Box<dyn Rng>`)禁止、具象型のみ
3. **Phase 3.0独立性**: `pricer_core`への依存禁止

### 2.3 複雑度シグナル

- **アルゴリズム複雑度**: Low (既存ライブラリ活用中心)
- **統合複雑度**: Low (`pricer_kernel`内の新規モジュール、外部依存最小)
- **テスト複雑度**: Medium (統計検証、プロパティテスト必要)
- **ドキュメント複雑度**: Low (既存パターン踏襲)

---

## 3. 実装アプローチオプション (Implementation Approach Options)

### Option A: rand_distrを活用した最小実装

#### 概要
`rand`と`rand_distr`を最大限活用し、薄いラッパーとして実装。正規分布は`rand_distr::Normal`を直接使用。

#### 詳細設計
**ファイル構成**:
```
crates/pricer_kernel/src/rng/
  mod.rs        → モジュールドキュメント、公開API
  prng.rs       → PRNG ラッパー実装
  normal.rs     → 正規分布生成(rand_distr活用)
  qmc.rs        → Sobolプレースホルダー
  tests.rs      → 統計テストユーティリティ(proptest使用)
```

**主要構造体**:
```rust
// prng.rs
pub struct PricerRng {
    inner: rand::rngs::StdRng,  // または SmallRng
}

impl PricerRng {
    pub fn from_seed(seed: u64) -> Self { }
    pub fn fill_uniform(&mut self, buffer: &mut [f64]) { }
    pub fn fill_normal(&mut self, buffer: &mut [f64]) { }
}
```

#### 統合ポイント
- `lib.rs`に`pub mod rng;`追加
- `Cargo.toml`に`rand.workspace = true`, `rand_distr.workspace = true`追加

#### トレードオフ
- ✅ **最速実装**: 既存ライブラリ活用で開発時間最小
- ✅ **保守性高**: `rand`コミュニティの品質保証に依存
- ✅ **性能優秀**: `rand_distr::Normal`はZiggurat法使用(推定)
- ❌ **カスタマイズ制約**: アルゴリズム内部制御は困難
- ❌ **学習機会損失**: Box-Muller/Ziggurat実装経験を得られない

---

### Option B: Box-Muller独自実装 + rand基盤

#### 概要
PRNG部分は`rand`を活用しつつ、正規分布生成はBox-Muller法を独自実装。教育的価値と完全制御を重視。

#### 詳細設計
**ファイル構成**:
```
crates/pricer_kernel/src/rng/
  mod.rs         → モジュールドキュメント
  prng.rs        → PRNG ラッパー
  normal.rs      → Box-Muller独自実装
  algorithms/    → (オプション)アルゴリズム詳細
    box_muller.rs → Box-Muller実装
  qmc.rs         → Sobolプレースホルダー
  tests.rs       → 統計検証
```

**Box-Muller実装**:
```rust
// normal.rs
pub struct BoxMullerGenerator {
    cached_value: Option<f64>,  // 2値生成のキャッシュ
}

impl BoxMullerGenerator {
    pub fn fill_normal(&mut self, rng: &mut impl Rng, buffer: &mut [f64]) {
        // Box-Muller transform with cached value optimization
    }
}
```

#### 統合ポイント
- 数学的正確性の検証が重要(プロパティテストで分布検証)
- `approx`を用いた統計モーメント検証

#### トレードオフ
- ✅ **完全制御**: アルゴリズム詳細を完全把握・カスタマイズ可能
- ✅ **教育的**: Box-Muller実装経験、数値安定性の学習
- ✅ **ドキュメント充実**: TAOCP引用、数学的背景説明可能
- ❌ **開発時間増**: 実装・テスト・検証に時間必要
- ❌ **性能リスク**: `rand_distr`より遅い可能性(要ベンチマーク)
- ❌ **保守負担**: 自前実装のバグ修正・最適化負担

---

### Option C: ハイブリッド実装(推奨)

#### 概要
**Phase 3.1a**: `rand_distr`活用で迅速実装(Option A)
**Phase 3.1b(将来)**: Box-Muller独自実装を追加し、選択可能に

#### 段階的実装戦略

**Phase 3.1a(本仕様対象)**:
```rust
pub struct PricerRng {
    inner: rand::rngs::StdRng,
}

impl PricerRng {
    // rand_distrを使用した高速実装
    pub fn fill_normal(&mut self, buffer: &mut [f64]) {
        use rand_distr::{Distribution, Normal};
        let normal = Normal::new(0.0, 1.0).unwrap();
        for value in buffer.iter_mut() {
            *value = normal.sample(&mut self.inner);
        }
    }
}
```

**Phase 3.1b(オプション拡張)**:
```rust
pub enum NormalAlgorithm {
    RandDistr,      // デフォルト(Ziggurat)
    BoxMuller,      // 独自実装
    // 将来: Ziggurat(独自実装)
}

pub struct PricerRngBuilder {
    seed: u64,
    normal_algorithm: NormalAlgorithm,
}
```

#### 移行戦略
1. **Phase 3.1a**: `rand_distr`のみで全要件充足
2. **統計検証**: プロパティテストで分布正確性確認
3. **性能測定**: Criterionベンチマークで性能ベースライン確立
4. **Phase 3.1b**: Box-Muller実装追加、ベンチマーク比較、選択可能に

#### リスク軽減
- **機能フラグ**: `features = ["custom-normal"]`で独自実装を有効化
- **ロールバック容易**: `rand_distr`実装は常に維持
- **段階的検証**: 各フェーズで統計検証・ベンチマーク実施

#### トレードオフ
- ✅ **迅速な初期実装**: Phase 3.1aで全要件充足
- ✅ **将来拡張性**: アルゴリズム選択可能性を保持
- ✅ **リスク分散**: 複数実装で検証・比較可能
- ❌ **複雑度増**: 複数実装の保守負担
- ❌ **計画コスト**: フェーズ分割の計画・管理コスト

---

## 4. 実装複雑度とリスク評価

### 実装規模見積もり: **Small (S)** - 1-3日

**根拠**:
- グリーンフィールド実装(既存コード変更なし)
- 既存ライブラリ(`rand`, `rand_distr`)活用中心
- 既存パターン(`verify/mod.rs`)の踏襲
- ファイル数: 4-5ファイル程度

**内訳**:
- PRNG ラッパー実装: 0.5日
- 正規分布生成(Option A採用時): 0.5日
- Sobolプレースホルダー: 0.25日
- テスト・プロパティテスト: 1日
- ドキュメント・British English校正: 0.5日
- 統合・検証: 0.25日

### リスク評価: **Low**

**根拠**:
- **技術熟知度**: `rand`は成熟したエコシステム、ドキュメント充実
- **統合リスク**: 新規モジュール追加のみ、既存コード影響なし
- **性能リスク**: `rand_distr`のZiggurat法は高性能実績あり
- **セキュリティリスク**: 暗号論的安全性は非要件(Monte Carlo用途)

**潜在的リスクと軽減策**:

| リスク | 確率 | 影響 | 軽減策 |
|-------|------|------|--------|
| 統計検証失敗 | Medium | Medium | Property-based testingで早期検出、`rand_distr`の品質に依存 |
| 性能未達(<1M samples/s) | Low | Low | Criterionベンチマークで早期測定、`rand_distr`は高速実績あり |
| Enzyme非互換 | Low | Medium | 静的ディスパッチ徹底、Phase 4で検証 |
| British English漏れ | Medium | Low | レビューチェックリスト、CI lint追加検討 |

---

## 5. 設計フェーズへの推奨事項

### 5.1 推奨アプローチ

**Option C(ハイブリッド実装)のPhase 3.1aを推奨**

**理由**:
1. **迅速性**: 既存ライブラリ活用で1-3日実装完了
2. **品質**: `rand_distr`の成熟した実装に依存、統計検証済み
3. **将来拡張性**: Phase 3.1bでBox-Muller追加可能な設計
4. **リスク最小**: Low複雑度、Low risk、グリーンフィールド実装

### 5.2 主要設計決定事項

#### 必須決定事項(設計フェーズで確定)
1. **PRNG選択**: `rand::rngs::StdRng` vs `rand::rngs::SmallRng`
   - **推奨**: `StdRng`(ChaCha20ベース、高品質、再現性)

2. **正規分布実装**: `rand_distr::Normal`使用(Phase 3.1a)
   - **確認事項**: `rand_distr`がZiggurat法使用か調査

3. **Sobolプレースホルダー設計**: トレイトか構造体か
   - **推奨**: トレイト定義 + `unimplemented!()`スタブ

4. **British English検証**: 手動レビューかlint自動化か
   - **推奨**: レビューチェックリスト作成

#### オプション決定事項
1. **ベンチマーク追加**: Criterionベンチマーク含めるか
   - **推奨**: 性能要件(1M samples/s)検証のため含める

2. **機能フラグ導入**: Phase 3.1bへの準備
   - **推奨**: Phase 3.1aではスキップ、必要時に追加

### 5.3 設計フェーズで実施すべき調査

#### 必須調査
1. **`rand_distr::Normal`実装確認**
   - ソースコード調査でZiggurat使用確認
   - ベンチマーク測定で性能要件(1M samples/s)達成確認

2. **統計テスト設計**
   - Kolmogorov-Smirnov検定か Anderson-Darling検定か
   - Property-based testingでの平均・分散検証方法

#### オプション調査
1. **Sobol次元数要件**
   - Monte Carloシミュレーションの典型的次元数調査
   - Phase 4以降の要件確認

2. **キャッシュ最適化**
   - バッファサイズとキャッシュライン整合性
   - SIMD最適化の可能性

### 5.4 実装タスク候補(設計フェーズでの詳細化)

**Phase 3.1a実装タスク**(詳細は`tasks.md`で生成):
1. ディレクトリ・モジュール構造作成
2. PRNG ラッパー実装(`PricerRng`構造体)
3. 一様分布生成(`fill_uniform`)
4. 正規分布生成(`fill_normal`, `rand_distr`使用)
5. Sobolプレースホルダートレイト定義
6. 統計検証テスト(property-based tests)
7. 再現性テスト(シード固定)
8. ドキュメント作成(rustdoc, British English)
9. Criterionベンチマーク(性能検証)
10. `lib.rs`統合、CI確認

---

## 6. 要件-アセットマッピング

| 要件ID | 要件概要 | 既存アセット | ギャップ | 実装方針 |
|--------|---------|------------|--------|---------|
| Req 1 | PRNGラッパー | `rand` workspace依存 | **Missing**: 構造体実装 | `PricerRng`新規作成 |
| Req 2 | 正規分布生成 | `rand_distr` workspace依存 | **Missing**: API実装 | `rand_distr::Normal`活用 |
| Req 3 | Sobolプレースホルダー | なし | **Missing**: トレイト設計 | トレイト定義 + stub |
| Req 4 | ベクトル効率性 | なし | **Missing**: スライスAPI | `fill_*(&mut [f64])` |
| Req 5 | モジュール独立性 | ✅ Phase 3.0原則 | **No Gap** | 現状維持 |
| Req 6 | テスト容易性 | `proptest`, `approx`依存済み | **Missing**: テスト実装 | Property tests作成 |
| Req 7 | British English | `verify/`は米国英語 | **Constraint**: 新規統一 | レビューチェックリスト |
| Req 8 | Enzyme互換性 | `verify/`設計パターン | **Missing**: 文書化 | 設計ガイドライン追加 |

---

## 7. まとめと次ステップ

### 分析結果サマリ
- **実装タイプ**: グリーンフィールド(新規モジュール作成)
- **推奨アプローチ**: Option C Phase 3.1a(`rand_distr`活用)
- **複雑度**: Small (1-3日)
- **リスク**: Low

### 設計フェーズへの移行準備完了
本ギャップ分析により、以下が明確化された:
1. ✅ 既存アセットの活用可能性(`rand`, `rand_distr`, `proptest`)
2. ✅ 実装ギャップの特定(構造体、API、テスト実装)
3. ✅ 3つの実装オプションの評価(A: 最小、B: 独自、C: ハイブリッド)
4. ✅ 複雑度・リスク評価(Small/Low)

### 次ステップ: 技術設計生成
```bash
/kiro:spec-design rng-infrastructure
```

設計フェーズでは、以下を詳細化:
- API設計(構造体、メソッドシグネチャ)
- ファイル構成の確定
- テスト戦略の詳細化
- ベンチマーク設計
- British English チェックリスト作成
