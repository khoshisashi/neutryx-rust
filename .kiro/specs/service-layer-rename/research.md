# Research & Design Decisions

## Summary

- **Feature**: `service-layer-rename`
- **Discovery Scope**: Extension（既存システムのリファクタリング）
- **Key Findings**:
  - Runtime層クレートは3つ（`runtime_cli`, `runtime_python`, `runtime_server`）で、いずれもA-I-P-R依存関係ルールに従っている
  - バイナリ名（`neutryx`, `neutryx-server`）およびPythonモジュール名（`neutryx`）は維持可能
  - CI/CDワークフローに多数のruntime_*参照があり、一括置換が必要

## Research Log

### クレート構造の分析

- **Context**: Service層へのリネームに伴う影響範囲の特定
- **Sources Consulted**: `crates/runtime_*/Cargo.toml`, `Cargo.toml`（ワークスペース）
- **Findings**:
  - `runtime_cli`: バイナリ名`neutryx`、依存先はP/I/A全層
  - `runtime_python`: ライブラリ名`neutryx`（cdylib）、PyO3バインディング
  - `runtime_server`: バイナリ名`neutryx-server`、gRPC/REST両対応
- **Implications**: バイナリ名・ライブラリ名は変更不要、パッケージ名のみ変更

### CI/CDワークフローの分析

- **Context**: GitHub Actionsでのruntime参照箇所の特定
- **Sources Consulted**: `.github/workflows/ci.yml`
- **Findings**:
  - `--exclude runtime_*`パターンが複数箇所（stable/nightly分離のため）
  - 個別クレートビルド：`cargo build -p runtime_cli`等
  - runtime_pythonはPythonヘッダー依存のため別ステップ
- **Implications**: 置換は単純な文字列置換で対応可能、ただし`service_gateway`への変更は個別対応

### ドキュメント参照箇所の分析

- **Context**: A-I-P-R参照を含むドキュメントの特定
- **Sources Consulted**: `README.md`, `CONTRIBUTING.md`, `.kiro/steering/*.md`, `CLAUDE.md`
- **Findings**:
  - 依存関係ルールの説明で「**R**untime」を使用
  - アーキテクチャ図にruntime_*クレート名を記載
  - CONTRIBUTING.mdのToolchain GuideはPricer層のみ記載（Runtime層なし）
- **Implications**: 全ドキュメントで「A-I-P-R」→「A-I-P-S」、「Runtime」→「Service」の置換

## Architecture Pattern Evaluation

| Option | Description | Strengths | Risks / Limitations | Notes |
|--------|-------------|-----------|---------------------|-------|
| 一括リネーム | git mvで3ディレクトリをリネーム | シンプル、git履歴維持 | なし | 推奨 |
| 段階的移行 | 新名称でシンボリックリンク作成後に移行 | 後方互換性 | 複雑性増加、不要 | 不採用 |

## Design Decisions

### Decision: ディレクトリリネーム方式

- **Context**: 3つのruntime_*クレートを一括でリネームする必要がある
- **Alternatives Considered**:
  1. `git mv`による直接リネーム — シンプルで履歴維持
  2. 新ディレクトリ作成→コピー→旧削除 — 履歴分断
- **Selected Approach**: `git mv`による直接リネーム
- **Rationale**: Git履歴を維持しながら最小限の変更で完了
- **Trade-offs**: なし
- **Follow-up**: リネーム後に`cargo build --workspace`で検証

### Decision: runtime_server → service_gateway

- **Context**: `runtime_server`のみ`service_server`ではなく`service_gateway`にリネーム
- **Alternatives Considered**:
  1. `service_server` — 単純な置換
  2. `service_gateway` — マイクロサービス文脈で適切な命名
  3. `service_api` — API提供の役割を明示
- **Selected Approach**: `service_gateway`
- **Rationale**: マイクロサービスアーキテクチャにおける「ゲートウェイ」の役割を明確化
- **Trade-offs**: 他のクレートと命名パターンが若干異なる
- **Follow-up**: バイナリ名`neutryx-server`は維持

### Decision: バイナリ名・ライブラリ名の維持

- **Context**: エンドユーザー向けの後方互換性
- **Alternatives Considered**:
  1. バイナリ名も変更 — 一貫性は高いが破壊的
  2. バイナリ名維持 — 後方互換性維持
- **Selected Approach**: バイナリ名維持（`neutryx`, `neutryx-server`, Pythonモジュール`neutryx`）
- **Rationale**: 利用者への影響を最小化
- **Trade-offs**: パッケージ名とバイナリ名の不一致
- **Follow-up**: マイグレーションガイドで明記

## Risks & Mitigations

- **Risk 1**: IDEキャッシュによるビルド失敗 — `cargo clean`後に再ビルド
- **Risk 2**: git履歴の参照困難 — `git log --follow`で追跡可能であることを確認
- **Risk 3**: 外部依存（crates.io未公開のため該当なし） — 内部依存のみのため影響なし

## References

- [Cargo Package Renaming Best Practices](https://doc.rust-lang.org/cargo/reference/manifest.html#the-name-field) — パッケージ名変更の公式ガイダンス
- [Git mv Documentation](https://git-scm.com/docs/git-mv) — ファイル移動時の履歴維持
