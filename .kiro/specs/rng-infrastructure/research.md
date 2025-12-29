# Research & Design Decisions: RNG Infrastructure

---
**Purpose**: 調査結果、アーキテクチャ分析、技術設計の根拠を記録する。

**Usage**:
- ディスカバリフェーズ中の調査活動と結果を記録
- `design.md`には詳細すぎる設計決定のトレードオフを文書化
- 将来の監査や再利用のための参照とエビデンスを提供
---

## Summary
- **Feature**: `rng-infrastructure`
- **Discovery Scope**: New Feature (グリーンフィールド実装)
- **Key Findings**:
  - `rand_distr::Normal`はZIGNOR Zigguratアルゴリズムを使用(高性能)
  - Monte Carlo用途では`SmallRng`が`StdRng`より5倍高速(7GB/s vs 1.5GB/s)
  - Sobolシーケンス用に`sobol`クレートが利用可能(21,201次元まで対応)
  - バッファサイズは1M要素程度がMonte Carloに最適

## Research Log

### PRNG選択: StdRng vs SmallRng

- **Context**: Monte Carloシミュレーションに最適なPRNGの選択
- **Sources Consulted**:
  - [The Rust Rand Book - Our RNGs](https://rust-random.github.io/book/guide-rngs.html)
  - [GitHub - rust-random/rand](https://github.com/rust-random/rand)
  - [StdRng in rand::rngs](https://docs.rs/rand/latest/rand/rngs/struct.StdRng.html)
- **Findings**:
  - **StdRng**: ChaCha12アルゴリズム、CSPRNG、1.5GB/s、136バイト状態
  - **SmallRng**: Xoshiro256PlusPlus(64bit)、7GB/s、16バイト状態
  - SmallRngはStdRngより約5倍高速、メモリ使用量1/8
  - Monte Carloでは暗号論的安全性不要、SmallRngで十分
  - ただしSmallRngは「portable」ではない(アルゴリズムが将来変更される可能性)
- **Implications**:
  - **推奨**: 性能重視のMonte CarloにはSmallRng使用
  - **代替**: 再現性が長期間必要な場合はStdRng(アルゴリズム安定)
  - **設計**: 両方をサポートする抽象化層を提供

### 正規分布生成アルゴリズム

- **Context**: 正規分布サンプリングのアルゴリズム選択
- **Sources Consulted**:
  - [rand_distr::Normal](https://docs.rs/rand_distr/0.4.3/rand_distr/struct.Normal.html)
  - [rand_distr::StandardNormal](https://docs.rs/rand_distr/latest/rand_distr/struct.StandardNormal.html)
- **Findings**:
  - `rand_distr::Normal`は**ZIGNOR variant of Ziggurat method**を使用
  - 参照: Jurgen A. Doornik (2005), "An Improved Ziggurat Method to Generate Normal Random Samples"
  - `StandardNormal`は平均0、標準偏差1に最適化された実装
  - `from_zscore()`メソッドで相関サンプル生成が可能
  - `num-traits`に依存、`Copy`/`Clone`実装済み
- **Implications**:
  - Box-Muller独自実装は不要(Zigguratの方が高速)
  - `rand_distr::StandardNormal`を直接使用推奨
  - 要件2.1「Box-Muller or Ziggurat」はZigguratで充足

### Sobolシーケンス(QMC)

- **Context**: 準モンテカルロ用低食い違いシーケンスの調査
- **Sources Consulted**:
  - [sobol - crates.io](https://crates.io/crates/sobol)
  - [Quasi-Monte Carlo method - Wikipedia](https://en.wikipedia.org/wiki/Quasi-Monte_Carlo_method)
- **Findings**:
  - `sobol`クレートが利用可能(Antonov-Saleev再帰グレイコード最適化)
  - Joe-Kuo D6パラメータで最大**21,201次元**をサポート
  - 収束速度: QMC O(1/N) vs MC O(N^-0.5)
  - Sobolは6次元以上で優位、Haltonは低次元向き
  - f32およびその他のRust数値プリミティブをサポート
- **Implications**:
  - Phase 3.1aではプレースホルダーのみ実装
  - 将来的に`sobol`クレート統合を検討
  - トレイト設計で将来のQMC拡張に備える

### キャッシュ最適化とバッファサイズ

- **Context**: Monte Carlo用バッファサイズの最適化
- **Sources Consulted**:
  - [MATLAB Monte Carlo Chunking](https://www.mathworks.com/help/finance/improving-performance-of-monte-carlo-simulation-with-parallel-computing.html)
  - [GPU Monte Carlo in Rust](https://medium.com/@joseph.frost_91327/gpu-monte-carlo-simulations-in-python-and-rust-c9b345525bcf)
- **Findings**:
  - MATLABでは1M要素のチャンクが経験的に最適
  - チャンクなしは約20%低速、メモリ制限に到達
  - f32 vs f64の型一貫性が重要
  - L3キャッシュ(典型的に8-32MB)を考慮
  - f64は8バイト、1Mサンプル = 8MB(L3キャッシュに収まる)
- **Implications**:
  - **推奨バッファサイズ**: 64KB-1MB(L1-L3キャッシュ考慮)
  - **最小バッチ**: 8KB(L1キャッシュ32KB以内)
  - **最大バッチ**: 8MB(大規模シミュレーション用)
  - ドキュメントに推奨サイズを記載

### Enzyme互換性

- **Context**: 将来的なEnzyme AD統合への準備
- **Sources Consulted**:
  - 既存`verify/mod.rs`のパターン
  - プロジェクトsteering docs (tech.md)
- **Findings**:
  - 静的ディスパッチ必須(`Box<dyn Rng>`禁止)
  - 固定サイズforループ推奨
  - RNG生成は非微分可能(seed/generation)
  - 消費側(パス計算)が微分対象
- **Implications**:
  - 具象型`PricerRng<R: Rng>`でジェネリクス使用
  - `fill_*`メソッドは固定サイズイテレーション
  - 微分可能/非微分可能境界を文書化

## Architecture Pattern Evaluation

| Option | Description | Strengths | Risks / Limitations | Notes |
|--------|-------------|-----------|---------------------|-------|
| **A: rand_distr活用** | rand_distrを直接使用、薄いラッパー | 最速実装、品質保証済み | カスタマイズ制約 | **推奨** |
| B: Box-Muller独自 | 正規分布を独自実装 | 完全制御、教育的 | 性能リスク、保守負担 | 将来オプション |
| C: ハイブリッド | A + 将来的にB追加 | 拡張性 | 複雑度増 | Phase 3.1bで検討 |

## Design Decisions

### Decision: PRNG実装 - SmallRng + StdRngサポート

- **Context**: Monte Carlo用PRNGの選択と再現性要件のバランス
- **Alternatives Considered**:
  1. SmallRngのみ — 最高性能、ポータビリティなし
  2. StdRngのみ — 安定性、低性能
  3. 両方サポート — 柔軟性、実装コスト増
- **Selected Approach**: **デフォルトStdRng、SmallRngオプション**
  - `PricerRng`は`StdRng`を内部で使用(デフォルト)
  - `PricerRng::fast(seed)`で`SmallRng`使用バージョン提供
- **Rationale**:
  - Phase 3.0の再現性重視原則に整合
  - 性能が必要な場合はSmallRngを選択可能
  - アルゴリズム安定性でデバッグ容易性確保
- **Trade-offs**:
  - (+) 再現性とデバッグ容易性を優先
  - (+) 性能オプションも提供
  - (-) デフォルトは最速ではない
- **Follow-up**: ベンチマークで性能差を測定、必要に応じてデフォルト変更検討

### Decision: 正規分布生成 - rand_distr::StandardNormal使用

- **Context**: 正規分布サンプリングアルゴリズムの選択
- **Alternatives Considered**:
  1. Box-Muller独自実装 — 完全制御、開発コスト高
  2. rand_distr::Normal — 成熟実装、Ziggurat使用
  3. カスタムZiggurat — 最適化可能、複雑
- **Selected Approach**: **rand_distr::StandardNormal**
  - バッチ生成時は`StandardNormal`を直接使用
  - 平均・標準偏差変換は後処理で実施
- **Rationale**:
  - ZIGNOR Zigguratは業界標準、十分高速
  - Doornik (2005)の学術的裏付け
  - 独自実装のバグリスクを回避
- **Trade-offs**:
  - (+) 成熟した高品質実装
  - (+) 保守負担なし
  - (-) アルゴリズム詳細の制御不可
- **Follow-up**: ベンチマークで1M samples/sec要件達成確認

### Decision: Sobolプレースホルダー - トレイトベース設計

- **Context**: QMC将来実装のためのインターフェース設計
- **Alternatives Considered**:
  1. 構造体スタブ — シンプル、拡張困難
  2. トレイト定義 — 柔軟、複雑
  3. 完全スキップ — 最小、将来の統合困難
- **Selected Approach**: **トレイト定義 + マーカー構造体**
  ```rust
  pub trait LowDiscrepancySequence {
      fn dimension(&self) -> usize;
      fn next_point(&mut self) -> &[f64];
  }

  pub struct SobolPlaceholder;  // unimplemented!()
  ```
- **Rationale**:
  - 将来の`sobol`クレート統合が容易
  - インターフェースを事前定義で設計安定
  - コンパイル時エラーで誤用防止
- **Trade-offs**:
  - (+) 将来の拡張性確保
  - (+) API設計の早期固定
  - (-) 現時点では使用不可
- **Follow-up**: Phase 4でSobol実装検討

### Decision: バッチ生成API - スライスベース設計

- **Context**: ベクトル効率性要件の実現方法
- **Alternatives Considered**:
  1. Iterator API — 遅延評価、柔軟
  2. Vec返却 — シンプル、アロケーション発生
  3. スライス書込み — ゼロアロケーション、呼び出し側バッファ管理
- **Selected Approach**: **スライスベース `fill_*(&mut [f64])`**
  ```rust
  fn fill_uniform(&mut self, buffer: &mut [f64]);
  fn fill_normal(&mut self, buffer: &mut [f64]);
  ```
- **Rationale**:
  - Monte Carlo用途でバッファ再利用が一般的
  - アロケーション最小化で性能向上
  - 呼び出し側がバッファサイズ制御
- **Trade-offs**:
  - (+) ゼロアロケーション
  - (+) キャッシュ効率
  - (-) 呼び出し側のバッファ管理負担
- **Follow-up**: 便利メソッド`gen_uniform_vec(n)`も追加検討

## Risks & Mitigations

- **統計検証失敗** — Property-based testingで早期検出、`rand_distr`品質に依存
- **性能未達(<1M/s)** — Criterionベンチマーク、`rand_distr`は高速実績あり
- **Enzyme非互換** — 静的ディスパッチ徹底、Phase 4で検証
- **British English漏れ** — レビューチェックリスト作成

## References

- [The Rust Rand Book](https://rust-random.github.io/book/guide-rngs.html) — PRNG比較、推奨事項
- [rand_distr::Normal Documentation](https://docs.rs/rand_distr/0.4.3/rand_distr/struct.Normal.html) — Ziggurat実装詳細
- [sobol crate](https://crates.io/crates/sobol) — Rust用Sobolシーケンス
- [Doornik (2005)](https://www.doornik.com/research/ziggurat.pdf) — ZIGNOR Zigguratアルゴリズム論文
- [GitHub rust-random/rand](https://github.com/rust-random/rand) — rand crateソースコード
