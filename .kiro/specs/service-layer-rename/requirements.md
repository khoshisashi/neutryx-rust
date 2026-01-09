# Requirements Document

## Introduction
本仕様は、Neutryx XVAプライシングライブラリのアーキテクチャにおける「Runtime」レイヤーから「Service」レイヤーへのリネームを定義する。現行のA-I-P-R（Adapter-Infra-Pricer-Runtime）構造をA-I-P-S（Adapter-Infra-Pricer-Service）に変更し、マイクロサービス文脈により適した命名規則を採用する。

### 変更対象
| 現行名 | 新名称 | 理由 |
|--------|--------|------|
| `runtime_cli` | `service_cli` | レイヤープレフィックス統一 |
| `runtime_python` | `service_python` | レイヤープレフィックス統一 |
| `runtime_server` | `service_gateway` | マイクロサービスゲートウェイとしての役割を明確化 |

## Requirements

### Requirement 1: クレートディレクトリのリネーム
**Objective:** As a 開発者, I want Runtimeレイヤーのクレートディレクトリ名をServiceプレフィックスに変更したい, so that アーキテクチャ命名がA-I-P-Sの一貫性を保つ

#### Acceptance Criteria
1. When リネーム処理を実行する, the Build System shall `crates/runtime_cli/`を`crates/service_cli/`にリネームする
2. When リネーム処理を実行する, the Build System shall `crates/runtime_python/`を`crates/service_python/`にリネームする
3. When リネーム処理を実行する, the Build System shall `crates/runtime_server/`を`crates/service_gateway/`にリネームする
4. The Directory Structure shall アルファベット順で`service_cli`, `service_gateway`, `service_python`の順序を維持する

### Requirement 2: Cargo.tomlワークスペース設定の更新
**Objective:** As a 開発者, I want ワークスペースのCargo.tomlを更新したい, so that 新しいクレート名でビルドが成功する

#### Acceptance Criteria
1. When ワークスペースCargo.tomlを更新する, the Cargo Workspace shall `members`配列の`runtime_*`エントリを`service_*`に変更する
2. When クレートCargo.tomlを更新する, the Cargo Package shall 各クレートの`[package].name`を新名称に変更する
3. When 依存関係を更新する, the Cargo Dependencies shall `[dependencies]`および`[dev-dependencies]`内の`runtime_*`参照を`service_*`に変更する
4. The Cargo Workspace shall `cargo build --workspace`が成功することを保証する

### Requirement 3: ソースコード内インポートの更新
**Objective:** As a 開発者, I want ソースコード内のインポート文を更新したい, so that コンパイルエラーが発生しない

#### Acceptance Criteria
1. When インポートを更新する, the Rust Compiler shall `use runtime_cli::`を`use service_cli::`に変更する
2. When インポートを更新する, the Rust Compiler shall `use runtime_python::`を`use service_python::`に変更する
3. When インポートを更新する, the Rust Compiler shall `use runtime_server::`を`use service_gateway::`に変更する
4. When モジュールパスを更新する, the Rust Compiler shall `crate::`パスおよび`super::`パスが正しく解決されることを保証する
5. The Rust Compiler shall `cargo test --workspace`が全テストパスすることを保証する

### Requirement 4: ドキュメント・Steeringファイルの更新
**Objective:** As a 開発者, I want ドキュメントおよびSteeringファイルを更新したい, so that アーキテクチャ説明がA-I-P-S構造を反映する

#### Acceptance Criteria
1. When Steeringを更新する, the Documentation shall `.kiro/steering/product.md`の「A-I-P-R」参照を「A-I-P-S」に変更する
2. When Steeringを更新する, the Documentation shall `.kiro/steering/structure.md`のRuntime Layer説明をService Layerに変更する
3. When Steeringを更新する, the Documentation shall `.kiro/steering/tech.md`の技術スタック説明を更新する
4. When READMEを更新する, the Documentation shall `README.md`内のアーキテクチャ図およびクレート名参照を更新する
5. When CLAUDE.mdを更新する, the Documentation shall `CLAUDE.md`内のA-I-P-R参照をA-I-P-Sに変更する
6. When CONTRIBUTING.mdを更新する, the Documentation shall `CONTRIBUTING.md`内のレイヤー説明（L1-L4）およびService層クレート参照を更新する
7. When PRテンプレートを更新する, the Documentation shall `.github/PULL_REQUEST_TEMPLATE.md`のAffected Crate(s)セクションにService層クレートを追加する
8. The Documentation shall 依存関係ルールの説明で「**S**ervice」を「**R**untime」の代わりに使用する

**Note:** 以下の履歴文書は変更対象外とする（過去の仕様履歴として保持）:
- `.kiro/specs/`内の完了済み仕様ドキュメント
- `crates/pricer_core/IMPLEMENTATION_SUMMARY.md`
- `crates/pricer_pricing/PHASE_3_0_SUMMARY.md`

### Requirement 5: CI/CD設定の更新
**Objective:** As a 開発者, I want CI/CDパイプライン設定を更新したい, so that 自動ビルドおよびテストが新クレート名で動作する

#### Acceptance Criteria
1. When GitHub Actionsを更新する, the CI Pipeline shall `.github/workflows/ci.yml`内の`runtime_*`参照を`service_*`に変更する
2. If リリースワークフローが存在する, then the CI Pipeline shall `.github/workflows/release.yml`内のクレート名参照を更新する
3. The CI Pipeline shall 全ワークフローがパスすることを保証する

### Requirement 6: Docker設定の更新
**Objective:** As a 開発者, I want Docker関連ファイルを更新したい, so that コンテナビルドが新クレート名で動作する

#### Acceptance Criteria
1. When Dockerfileを更新する, the Docker Build shall `docker/Dockerfile.stable`内のクレート名参照を更新する
2. When Dockerfileを更新する, the Docker Build shall `docker/Dockerfile.nightly`内のクレート名参照を更新する
3. Where docker-compose.ymlが存在する, the Docker Build shall サービス名およびビルドコンテキストを更新する
4. The Docker Build shall `docker build`コマンドが成功することを保証する

### Requirement 7: 依存関係ルールの整合性維持
**Objective:** As a アーキテクト, I want A-I-P-Sの依存関係ルールが維持されることを確認したい, so that アーキテクチャの一貫性が保たれる

#### Acceptance Criteria
1. The Architecture shall **S**erviceクレートが**P**ricer, **I**nfra, **A**dapterクレートに依存可能であることを維持する
2. The Architecture shall **P**ricerクレートが**S**erviceまたは**A**dapterクレートに依存しないことを維持する
3. The Architecture shall **I**nfraクレートが**P**ricerまたは**S**erviceクレートに依存しないことを維持する
4. The Architecture shall **A**dapterクレートが**S**erviceクレートに依存しないことを維持する
5. The Architecture shall 依存関係違反時にビルドエラーが発生することを保証する

### Requirement 8: 後方互換性の考慮
**Objective:** As a 利用者, I want 既存のCLIコマンドおよびAPI互換性を確認したい, so that 移行時の影響を把握できる

#### Acceptance Criteria
1. The CLI shall `neutryx`コマンド名を維持する（バイナリ名は変更しない）
2. The Python Bindings shall PyO3モジュール名の変更が必要な場合、マイグレーションガイドを提供する
3. Where gRPC/RESTエンドポイントが存在する, the API Gateway shall エンドポイントURLの変更有無を文書化する
4. The Migration Guide shall 利用者向けの移行手順を提供する
