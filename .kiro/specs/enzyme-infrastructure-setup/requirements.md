# 要件定義書

## プロジェクト概要（入力）
enzyme-infrastructure-setup "Setup Enzyme AD infrastructure for Layer 3 (pricer_kernel) with nightly Rust toolchain, LLVM bindings, and gradient verification for automatic differentiation at LLVM level"

## イントロダクション

本仕様は、4層アーキテクチャのLayer 3 (pricer_kernel) に対するEnzyme自動微分インフラストラクチャのセットアップを定義します。Enzyme ADエンジンはLLVMレベルで動作し、Layer 1 (pricer_core) およびLayer 2 (pricer_models) から完全に分離されたnightly Rustツールチェイン環境で実行されます。

**目的**:
- pricer_kernelクレートの初期構造を構築し、Enzymeバインディングを統合する
- nightly Rustツールチェイン（Enzyme互換）とstable Rust（L1/L2/L4）の分離を実現する
- 単純な関数（f(x) = x²）を用いてEnzyme勾配計算の検証を行う
- Layer 3の実験的コード（Enzyme）をワークスペースの75%（stable Rust）から隔離する

**対象範囲外**:
- Layer 1/2の機能統合（Phase 4で対応）
- モンテカルロカーネルの実装（Phase 4で対応）
- パフォーマンスベンチマーク（Phase 6で対応）
- プロダクション環境での実行（Phase 6で対応）

## 要件

### 要件1: Nightly Rustツールチェインの構成

**目的**: インフラストラクチャ担当者として、pricer_kernelクレートに対してnightly Rustツールチェインを分離構成したい。これにより、Layer 3のEnzyme依存をワークスペース全体に影響させることなく、L1/L2/L4でstable Rustを維持できる。

#### 受入基準

1. The pricer_kernel crate shall `rust-toolchain.toml`ファイルを`crates/pricer_kernel/`ディレクトリに持ち、`channel = "nightly-2025-01-15"`を指定すること
2. The workspace root shall デフォルトの`rust-toolchain.toml`で`channel = "stable"`を維持すること
3. When `cargo build -p pricer_kernel`を実行する, the build system shall 自動的にnightly-2025-01-15ツールチェインを使用すること
4. When `cargo build --workspace --exclude pricer_kernel`を実行する, the build system shall stable toolchainのみを使用すること
5. The pricer_kernel shall Layer 1およびLayer 2のクレート（pricer_core, pricer_models）に依存してはならないこと（Phase 3.0では分離を維持）

### 要件2: Cargo構成とワークスペース統合

**目的**: ビルドエンジニアとして、pricer_kernelクレートのCargo.tomlをワークスペースに統合し、Enzyme依存関係を正確に定義したい。これにより、依存関係の最小化とビルドの再現性が保証される。

#### 受入基準

1. The pricer_kernel shall `Cargo.toml`ファイルを`crates/pricer_kernel/`に持ち、ワークスペースメンバーとして登録されること
2. The pricer_kernel Cargo.toml shall Rust Edition 2021を指定すること
3. The pricer_kernel shall 依存関係として以下を含むこと:
   - `llvm-sys = "180"` (LLVM 18バインディング)
   - オプション: `enzyme` (Enzymeバインディング、利用可能な場合)
4. The pricer_kernel shall dev-dependenciesとして`approx = "0.5"`を含むこと（勾配検証用）
5. The pricer_kernel shall Phase 3.0ではpricer_core、pricer_models、pricer_xvaへの依存を持たないこと（完全分離）
6. When `cargo tree -p pricer_kernel`を実行する, the output shall 他のpricer_*クレートへの依存を示さないこと

### 要件3: モジュール構造と基盤コード

**目的**: 開発者として、pricer_kernelクレートの初期モジュール構造を定義し、将来のEnzyme統合のための基盤を構築したい。これにより、Phase 4以降の実装が明確な設計パターンに従える。

#### 受入基準

1. The pricer_kernel shall `src/lib.rs`ファイルを持ち、クレートレベルのドキュメントコメント（`//!`）を含むこと
2. The lib.rs documentation shall 以下を説明すること:
   - Layer 3の役割（AD Engine、Monte Carloカーネル）
   - nightly Rust要件とEnzyme分離の理由
   - 依存関係ゼロの原則（Phase 3.0では他のpricer_*クレートに依存しない）
3. The pricer_kernel shall `src/verify.rs`モジュールを持つこと（Enzyme検証用）
4. The lib.rs shall `pub mod verify;`宣言を含むこと
5. The pricer_kernel shall `#![warn(missing_docs)]`属性を持ち、すべてのpublic項目にドキュメントを強制すること
6. The verify module shall `//!`形式のモジュールドキュメントを持ち、「Enzyme gradient verification utilities」と説明すること

### 要件4: Enzyme勾配検証の実装

**目的**: 量子金融開発者として、単純な関数（f(x) = x²）を用いてEnzyme勾配計算の正しさを検証したい。これにより、Enzyme ADエンジンが正確に動作することを確認し、将来の複雑な金融計算の基盤とする。

#### 受入基準

1. The verify module shall `square`関数を定義し、`pub fn square(x: f64) -> f64`シグネチャを持つこと
2. The square function shall `x * x`を返すこと
3. The verify module shall `square_gradient`関数を定義し、Enzymeを用いて`square`関数の勾配を計算すること
4. When `square_gradient(x)`を呼び出す, the function shall `2.0 * x`を返すこと（解析的微分: d(x²)/dx = 2x）
5. The verify module shall `#[cfg(test)]`モジュールを含み、以下のテストケースを実装すること:
   - `test_square_value`: square(3.0) == 9.0を検証
   - `test_square_gradient`: square_gradient(3.0) ≈ 6.0を検証（approx使用、ε = 1e-10）
   - `test_square_gradient_at_zero`: square_gradient(0.0) == 0.0を検証
   - `test_square_gradient_negative`: square_gradient(-2.5) ≈ -5.0を検証
6. When `cargo test -p pricer_kernel`を実行する, all tests shall 成功すること

### 要件5: Enzymeバインディングの統合（条件付き）

**目的**: インフラストラクチャ担当者として、Enzymeバインディングを統合し、LLVMレベルの自動微分を有効化したい。これにより、将来のモンテカルロカーネル実装で高性能なGreeks計算が可能になる。

#### 受入基準

1. Where Enzymeバインディングが利用可能な場合, the pricer_kernel shall `Cargo.toml`に`enzyme`依存を含むこと
2. Where Enzymeバインディングが利用不可能な場合, the verify module shall プレースホルダー実装を提供し、`square_gradient`が解析的に計算した`2.0 * x`を返すこと
3. The pricer_kernel documentation shall Enzyme統合の現状（実装済み/プレースホルダー）を明記すること
4. If Enzymeバインディングの統合に失敗する, then the build process shall 明確なエラーメッセージを表示し、LLVMのバージョン要件（LLVM 18）を示すこと
5. The pricer_kernel shall ビルド成功時にEnzymeバインディングの状態（enabled/disabled）をログ出力すること

### 要件6: ビルド検証とCI統合準備

**目的**: DevOpsエンジニアとして、pricer_kernelクレートのビルドが再現可能であり、ワークスペース全体のCI/CDパイプラインに統合可能であることを確認したい。

#### 受入基準

1. When `cargo build -p pricer_kernel`を実行する, the build shall エラーなしで成功すること
2. When `cargo test -p pricer_kernel`を実行する, all tests shall 成功すること（Enzyme統合の有無に関わらず）
3. When `cargo clippy -p pricer_kernel -- -D warnings`を実行する, the output shall 警告を含まないこと
4. When `cargo fmt -p pricer_kernel -- --check`を実行する, the output shall フォーマット違反を示さないこと
5. The pricer_kernel shall `cargo doc --no-deps -p pricer_kernel`でドキュメントが生成され、リンク切れがないこと
6. When `cargo build --workspace`を実行する, the workspace shall pricer_kernelを含む全クレートをビルドすること（stable/nightly混在環境で）

### 要件7: ドキュメントとコメントの品質

**目的**: 開発者として、pricer_kernelクレートのコードとドキュメントがBritish Englishで書かれ、明確な使用例を含むことを確認したい。これにより、チーム全体がEnzyme統合の目的と使用方法を理解できる。

#### 受入基準

1. The pricer_kernel shall すべてのpublic関数にRustdocコメント（`///`）を持つこと
2. The documentation comments shall British Englishで記述されること（例: "optimise" not "optimize", "colour" not "color"）
3. The lib.rs shall 使用例を含み、`square`関数と`square_gradient`関数の呼び出し例を示すこと
4. The verify module documentation shall Enzymeの役割（LLVM-level AD）と検証の目的を説明すること
5. When `cargo test --doc -p pricer_kernel`を実行する, all documentation examples shall コンパイルおよび実行に成功すること
6. The pricer_kernel shall READMEファイルまたはlib.rsドキュメントで以下を説明すること:
   - Enzymeのインストール方法（Docker推奨）
   - `square_gradient`検証テストの実行方法
   - 既知の制約（nightly Rust要件、LLVM 18依存）

### 要件8: エラーハンドリングと診断

**目的**: インフラストラクチャ担当者として、Enzymeセットアップの失敗時に明確なエラーメッセージと診断情報を取得したい。これにより、問題の迅速な特定と解決が可能になる。

#### 受入基準

1. If LLVM 18がインストールされていない, then the build process shall "LLVM 18 required for Enzyme support"というエラーメッセージを表示すること
2. If nightly toolchainが利用不可能, then the build process shall 推奨されるツールチェインバージョン（nightly-2025-01-15）を示すエラーメッセージを表示すること
3. The pricer_kernel shall ビルドスクリプト（`build.rs`、オプション）でLLVM環境変数（`LLVM_SYS_180_PREFIX`など）の検証を行うこと
4. When Enzymeバインディングが見つからない, the build process shall 警告を表示し、プレースホルダーモードで続行すること
5. The verify module tests shall 失敗時にEnzymeの計算結果と期待値の両方を出力すること（例: "Expected 6.0, got 5.99"）

### 要件9: 将来の拡張性とPhase 4準備

**目的**: アーキテクトとして、pricer_kernelの初期構造が将来のPhase 4実装（Monte Carloカーネル、Layer 1/2統合）をサポートできることを確認したい。

#### 受入基準

1. The pricer_kernel module structure shall 将来の拡張のための予約モジュールを含むこと:
   - `enzyme/` (Enzymeバインディングと自動微分マクロ、Phase 4)
   - `mc/` (Monte Carloカーネル、経路生成、Phase 4)
2. The lib.rs documentation shall Phase 4での計画機能を説明すること（実装は含まない）:
   - Layer 1/2との統合（pricer_coreのスムージング関数使用）
   - モンテカルロ経路生成とEnzyme AD
3. The pricer_kernel shall `#[non_exhaustive]`や`pub(crate)`を使用し、将来のAPI変更に対応できること
4. The verify module shall `square_gradient`以外の検証関数を追加するための拡張可能な設計を持つこと
5. The pricer_kernel shall Phase 3.0完了時にステアリング文書（`.kiro/steering/`）へのコンプライアンスを文書化すること:
   - 4層分離の遵守（Layer 3のみがnightly Rust）
   - 静的ディスパッチの原則（将来の実装で適用）
   - 依存関係ゼロの原則（Phase 3.0では他のpricer_*クレートに依存しない）

## 要件カバレッジマップ

| 要件ID | 要件領域 | 優先度 | 検証方法 |
|--------|----------|--------|----------|
| 1 | Nightly Rustツールチェイン構成 | P0 | `cargo build -p pricer_kernel`成功 |
| 2 | Cargo構成とワークスペース統合 | P0 | `cargo tree -p pricer_kernel`検証 |
| 3 | モジュール構造と基盤コード | P0 | コード構造確認、ドキュメント生成 |
| 4 | Enzyme勾配検証の実装 | P0 | `cargo test -p pricer_kernel`全テスト成功 |
| 5 | Enzymeバインディングの統合 | P1 | ビルド成功、プレースホルダーフォールバック |
| 6 | ビルド検証とCI統合準備 | P0 | CI/CDパイプライン実行 |
| 7 | ドキュメントとコメント品質 | P0 | `cargo doc`、`cargo test --doc`成功 |
| 8 | エラーハンドリングと診断 | P1 | エラーシナリオテスト |
| 9 | 将来の拡張性とPhase 4準備 | P1 | ステアリング文書コンプライアンス確認 |

---
_作成日: 2025-12-29_
_すべての要件はEARSフォーマットに準拠し、テスト可能であること_
