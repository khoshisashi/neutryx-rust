# Implementation Plan

## Tasks

- [x] 1. クレートディレクトリのリネーム
  - `git mv`を使用して`runtime_cli`を`service_cli`にリネーム
  - `git mv`を使用して`runtime_python`を`service_python`にリネーム
  - `git mv`を使用して`runtime_server`を`service_gateway`にリネーム
  - リネーム後のディレクトリ存在確認（`ls crates/service_*`）
  - アルファベット順序の維持を確認（cli → gateway → python）
  - _Requirements: 1.1, 1.2, 1.3, 1.4_

- [x] 2. Cargo.toml設定の更新
- [x] 2.1 ワークスペースCargo.tomlの更新
  - `members`配列の`runtime_cli`を`service_cli`に変更
  - `members`配列の`runtime_python`を`service_python`に変更
  - `members`配列の`runtime_server`を`service_gateway`に変更
  - `cargo check --workspace`でパス解決を検証
  - _Requirements: 2.1_

- [x] 2.2 各クレートのCargo.toml更新
  - `service_cli/Cargo.toml`の`[package].name`を`service_cli`に変更
  - `service_python/Cargo.toml`の`[package].name`を`service_python`に変更
  - `service_gateway/Cargo.toml`の`[package].name`を`service_gateway`に変更
  - Cargo.toml内のコメントで「A-I-P-R」を「A-I-P-S」に更新
  - バイナリ名（`neutryx`, `neutryx-server`）およびライブラリ名（`neutryx`）は維持
  - _Requirements: 2.2, 2.3, 8.1_

- [x] 2.3 ビルド検証
  - `cargo clean`でキャッシュをクリア
  - `cargo build --workspace`で全クレートのビルド成功を確認
  - 依存関係グラフの検証（`cargo tree`）
  - _Requirements: 2.4_

- [x] 3. ソースコードの更新
- [x] 3.1 インポート文およびモジュール参照の更新
  - `use runtime_cli::`を`use service_cli::`に変更（該当箇所がある場合）
  - `use runtime_python::`を`use service_python::`に変更（該当箇所がある場合）
  - `use runtime_server::`を`use service_gateway::`に変更（該当箇所がある場合）
  - `crate::`および`super::`パスの正常動作を確認
  - _Requirements: 3.1, 3.2, 3.3, 3.4_

- [x] 3.2 テスト実行による検証
  - `cargo test --workspace`で全テストパスを確認
  - `cargo clippy --workspace`で警告なしを確認
  - _Requirements: 3.5_

- [x] 4. ドキュメントの更新
- [x] 4.1 (P) Steeringファイルの更新
  - `.kiro/steering/product.md`で「A-I-P-R」を「A-I-P-S」に変更
  - `.kiro/steering/product.md`で「Runtime」を「Service」に変更
  - `.kiro/steering/structure.md`のRuntime Layer説明をService Layerに変更
  - `.kiro/steering/structure.md`内の`runtime_*`クレート名を`service_*`に変更
  - `.kiro/steering/tech.md`の技術スタック説明を更新
  - 依存関係ルールの説明で「**S**ervice」を「**R**untime」に置換
  - _Requirements: 4.1, 4.2, 4.3, 4.8_

- [x] 4.2 (P) ルートドキュメントの更新
  - `README.md`内のアーキテクチャ図を更新（A-I-P-R → A-I-P-S）
  - `README.md`内の`runtime_*`クレート名参照を`service_*`に変更
  - `CLAUDE.md`内のA-I-P-R参照をA-I-P-Sに変更
  - `CONTRIBUTING.md`内のレイヤー説明およびService層クレート参照を更新
  - _Requirements: 4.4, 4.5, 4.6_

- [x] 4.3 (P) GitHubテンプレートの更新
  - `.github/PULL_REQUEST_TEMPLATE.md`のAffected Crate(s)セクションにService層クレートを追加
  - Runtime層の参照を削除またはService層に置換
  - _Requirements: 4.7_

- [x] 4.4 ドキュメント検証
  - `grep -r "A-I-P-R"`で残存参照を確認
  - `grep -r "runtime_cli\|runtime_python\|runtime_server"`で残存クレート名を確認
  - 履歴文書（`.kiro/specs/`内）は更新対象外であることを確認
  - _Requirements: 4.8_

- [x] 5. CI/CD設定の更新
- [x] 5.1 (P) GitHub Actionsワークフローの更新
  - `.github/workflows/ci.yml`内の`runtime_cli`を`service_cli`に変更
  - `.github/workflows/ci.yml`内の`runtime_python`を`service_python`に変更
  - `.github/workflows/ci.yml`内の`runtime_server`を`service_gateway`に変更
  - `--exclude`オプションのクレート名を更新
  - _Requirements: 5.1_

- [x] 5.2 (P) リリースワークフローの確認と更新
  - `.github/workflows/release.yml`が存在する場合、クレート名参照を更新
  - 存在しない場合はスキップ
  - _Requirements: 5.2_

- [x] 6. Docker設定の更新
- [x] 6.1 (P) Dockerfileの確認と更新
  - `docker/Dockerfile.stable`内にruntime参照があれば更新
  - `docker/Dockerfile.nightly`内にruntime参照があれば更新
  - 現時点でruntime参照がない場合は変更不要
  - _Requirements: 6.1, 6.2_

- [x] 6.2 (P) docker-compose.ymlの確認
  - `docker-compose.yml`が存在する場合、サービス名およびビルドコンテキストを確認
  - 存在しない場合はスキップ
  - _Requirements: 6.3_

- [x] 7. 依存関係ルールおよび後方互換性の検証
- [x] 7.1 A-I-P-S依存関係ルールの検証
  - Service層クレートがP/I/A層に依存可能であることを確認
  - P層クレートがService層に依存していないことを確認（`cargo tree`で検証）
  - I層クレートがP/S層に依存していないことを確認
  - A層クレートがS層に依存していないことを確認
  - _Requirements: 7.1, 7.2, 7.3, 7.4, 7.5_

- [x] 7.2 後方互換性の検証
  - `neutryx --help`でCLIが正常動作することを確認
  - Pythonモジュール名`neutryx`が維持されていることを確認
  - `neutryx-server`バイナリが正常にビルドされることを確認
  - _Requirements: 8.1, 8.2, 8.3_

- [x] 8. 最終検証および完了
- [x] 8.1 全体ビルドおよびテスト
  - `cargo build --workspace`の成功を確認
  - `cargo test --workspace`の全テストパスを確認
  - `cargo clippy --workspace -- -D warnings`で警告なしを確認
  - _Requirements: 2.4, 3.5, 5.3_

- [x] 8.2 マイグレーションガイドの作成
  - 利用者向けの移行手順を文書化
  - バイナリ名・APIエンドポイントに変更がないことを明記
  - パッケージ名の変更が内部のみであることを説明
  - _Requirements: 8.4_

## Requirements Coverage

| Requirement | Tasks |
|-------------|-------|
| 1.1, 1.2, 1.3, 1.4 | 1 |
| 2.1 | 2.1 |
| 2.2, 2.3 | 2.2 |
| 2.4 | 2.3, 8.1 |
| 3.1, 3.2, 3.3, 3.4 | 3.1 |
| 3.5 | 3.2, 8.1 |
| 4.1, 4.2, 4.3, 4.8 | 4.1 |
| 4.4, 4.5, 4.6 | 4.2 |
| 4.7 | 4.3 |
| 5.1 | 5.1 |
| 5.2 | 5.2 |
| 5.3 | 8.1 |
| 6.1, 6.2 | 6.1 |
| 6.3 | 6.2 |
| 6.4 | 8.1 |
| 7.1, 7.2, 7.3, 7.4, 7.5 | 7.1 |
| 8.1, 8.2, 8.3 | 7.2 |
| 8.4 | 8.2 |

## Parallel Execution Notes

タスク4.1〜4.3、5.1〜5.2、6.1〜6.2は `(P)` マーカーで並列実行可能です。これらのタスクは：

- 異なるファイル群を編集するため、競合なし
- データ依存関係なし（タスク3完了後に実行可能）
- 独立してテスト可能
