# Research & Design Decisions: frictional-bank-demo

---

## Summary

- **Feature**: frictional-bank-demo
- **Discovery Scope**: Extension（既存システム拡張）
- **Key Findings**:
  - upstream_systems / downstream_systems クレートは完全実装済み、再利用可能
  - ratatui v0.30 はダッシュボードに最適、Chart/Gauge/Tabs ウィジェットが豊富
  - axum 0.8 (ws feature) で WebSocket 統合が容易、split() パターンで送受信並行処理可能
  - service_gateway の既存 REST API は拡張ポイントとして適切

---

## Research Log

### TUI フレームワーク選定

- **Context**: Requirement 4（GUI）の TUI ダッシュボード実装に最適なライブラリを調査
- **Sources Consulted**:
  - [Ratatui 公式サイト](https://ratatui.rs/)
  - [Ratatui GitHub](https://github.com/ratatui/ratatui)
  - [docs.rs/ratatui](https://docs.rs/ratatui/latest/ratatui/)
- **Findings**:
  - ratatui v0.30.0 が最新安定版（11.9M ダウンロード）
  - sub-millisecond レンダリング、60+ FPS 維持可能
  - 組み込みウィジェット: Chart, Gauge, Tabs, Table, Paragraph, Sparkline
  - `ratatui::run()`, `ratatui::init()`, `ratatui::restore()` の便利関数あり
  - Layout による柔軟なパネル分割
- **Implications**:
  - PortfolioView, RiskView, TradeBlotter, Charts の要件に対応可能
  - Canvas ウィジェットで価格/リスク推移グラフ描画可能

### WebSocket 統合（axum）

- **Context**: Requirement 2.4, 4.6 のリアルタイム更新に WebSocket が必要
- **Sources Consulted**:
  - [axum::extract::ws](https://docs.rs/axum/latest/axum/extract/ws/index.html)
  - [tokio-rs/axum GitHub](https://github.com/tokio-rs/axum)
  - [Rust WebSocket Guide](https://www.ruststepbystep.com/creating-websocket-servers-in-axum-a-hands-on-rust-guide/)
- **Findings**:
  - axum 0.8.0 (2025-01-01 リリース) で `ws` feature 使用
  - `WebSocketUpgrade` extractor でハンドシェイク処理
  - `socket.split()` で送受信を別タスクに分離可能
  - Message タイプ: Text, Binary, Close
- **Implications**:
  - service_gateway に `/ws/risk` エンドポイント追加
  - downstream_systems の WebSocketSink と統合可能

### 既存 Demo クレート分析

- **Context**: gap-analysis で判明した既存アセットの再利用可能性確認
- **Sources Consulted**: upstream_systems/, downstream_systems/ ソースコード
- **Findings**:
  - `MarketDataProvider` trait: start(), stop(), snapshot() 完全実装
  - `BloombergSim`: yield curve, vol surface, equity quotes 生成済み
  - `FpmlGenerator`: IRS, CDS, FX Forward, Equity Option の FpML 生成済み
  - `WebSocketSink`: broadcast::Sender<WebSocketMessage> で配信準備済み
  - `RegulatorApi`, `SwiftReceiver`, `NettingEngine` 全て実装済み
- **Implications**:
  - Requirement 1, 2 は実装完了、統合テストのみ必要
  - frictional_bank オーケストレーターはこれらを呼び出すだけ

### pricer_pricing Enzyme 依存

- **Context**: Requirement 8.5 の nightly toolchain 要件の影響範囲確認
- **Sources Consulted**: tech.md, pricer_pricing/Cargo.toml
- **Findings**:
  - pricer_pricing のみ nightly-2025-01-15 + Enzyme 必要
  - 他の全クレートは stable Rust でビルド可能
  - `cargo build --workspace --exclude pricer_pricing` でデモ動作可能
- **Implications**:
  - デモは stable-only モードと full Enzyme モードの両方サポート
  - feature flag で pricer_pricing 依存を optional に

---

## Architecture Pattern Evaluation

| Option | Description | Strengths | Risks / Limitations | Notes |
|--------|-------------|-----------|---------------------|-------|
| **Orchestrator Pattern** (採用) | frictional_bank が全ワークフローを制御 | 明確な責務分離、シナリオ切替容易 | 単一障害点になる可能性 | A-I-P-S アーキテクチャに適合 |
| Event-Driven | メッセージキューで非同期連携 | 疎結合、スケーラブル | オーバーエンジニアリング、デモには過剰 | 本番向け |
| Direct Integration | 各コンポーネントが直接通信 | シンプル | 密結合、テスト困難 | 不採用 |

---

## Design Decisions

### Decision: TUI フレームワーク選定

- **Context**: Requirement 4 の TUI 実装に最適なライブラリ選定
- **Alternatives Considered**:
  1. ratatui — 最新の tui-rs 後継、アクティブ開発、豊富なウィジェット
  2. tui-rs — 元祖、メンテナンス終了
  3. cursive — ウィジェットライブラリ型、複雑な状態管理
- **Selected Approach**: ratatui v0.30
- **Rationale**: アクティブ開発、Chart ウィジェット、Rust エコシステムでの採用実績
- **Trade-offs**: 学習コストあり、しかしテンプレートとドキュメント充実
- **Follow-up**: cargo-generate でテンプレート生成、基本レイアウト検証

### Decision: WebSocket 実装方式

- **Context**: リアルタイムリスク更新配信
- **Alternatives Considered**:
  1. axum 組み込み ws — 標準的、既存 REST と統合容易
  2. tokio-tungstenite — 低レベル、柔軟だが追加依存
  3. warp — 別フレームワーク、既存 axum との混在避けたい
- **Selected Approach**: axum `ws` feature
- **Rationale**: 既存 service_gateway が axum 採用済み、追加依存なし
- **Trade-offs**: axum バージョンに依存
- **Follow-up**: /ws/risk エンドポイント追加、broadcast channel 統合

### Decision: デモシナリオ制御方式

- **Context**: EOD/Intraday/Stress の 3 シナリオを CLI から選択
- **Alternatives Considered**:
  1. clap サブコマンド — 標準的、既存 service_cli パターン
  2. TOML 設定ファイル — 柔軟だが起動時に選択できない
  3. Interactive menu — TUI 統合だがヘッドレス実行困難
- **Selected Approach**: clap サブコマンド (`cargo run -p frictional_bank -- eod`)
- **Rationale**: CI/CD 統合、スクリプト実行に適切、既存パターンに従う
- **Trade-offs**: コマンドライン引数が必要
- **Follow-up**: --config オプションで詳細設定オーバーライド

---

## Risks & Mitigations

- **service_gateway TODO 未実装** — 現在 `handlers.rs` に TODO コメント多数。設計フェーズで基本実装を追加し、デモ動作を保証する
- **Enzyme 環境依存** — stable-only モードを default とし、pricer_pricing は feature flag で optional に
- **ratatui 学習コスト** — 公式テンプレートを活用、最小限の UI から開始
- **WebSocket メッセージ形式** — downstream_systems の `WebSocketMessage` enum を再利用、新規定義不要

---

## References

- [Ratatui 公式](https://ratatui.rs/) — TUI フレームワーク
- [axum WebSocket docs](https://docs.rs/axum/latest/axum/extract/ws/index.html) — WebSocket 統合
- [tokio-rs/axum](https://github.com/tokio-rs/axum) — axum GitHub
- [Neutryx tech.md](.kiro/steering/tech.md) — 技術スタック定義
- [Neutryx structure.md](.kiro/steering/structure.md) — A-I-P-S アーキテクチャ
