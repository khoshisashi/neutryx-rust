# Gap Analysis: frictional-bank-demo

## Overview

本ドキュメントは、FrictionalBankデモシステムの要件と既存コードベースとのギャップを分析します。

**分析日**: 2026-01-09
**フェーズ**: Requirements Generated (未承認)

---

## 1. 現状調査 (Current State Investigation)

### 1.1 既存アセット一覧

#### A: Adapter Layer (完全実装済み)

| クレート | 状態 | 主要機能 |
|----------|------|----------|
| `adapter_feeds` | 完了 | MarketQuote型、マーケットデータ受信 |
| `adapter_fpml` | 完了 | FpML XMLパーサー |
| `adapter_loader` | 完了 | CSV/CSAローダー |

#### I: Infra Layer (完全実装済み)

| クレート | 状態 | 主要機能 |
|----------|------|----------|
| `infra_config` | 完了 | TOML/YAML設定読込 |
| `infra_master` | 完了 | カレンダー、通貨マスタ |
| `infra_store` | 完了 | PostgreSQL永続化 |

#### P: Pricer Layer (完全実装済み)

| クレート | 状態 | 主要機能 |
|----------|------|----------|
| `pricer_core` | 完了 | 数学関数、市場データ抽象化 |
| `pricer_models` | 完了 | 商品モデル (Equity, Rates, FX, Credit) |
| `pricer_optimiser` | 完了 | キャリブレーション、ブートストラップ |
| `pricer_pricing` | 完了 | Monte Carlo、Enzyme AD |
| `pricer_risk` | 完了 | XVA、エクスポージャー計算 |

#### S: Service Layer (部分実装)

| クレート | 状態 | 主要機能 | 備考 |
|----------|------|----------|------|
| `service_cli` | 完了 | calibrate, check, price, report | 実装済み |
| `service_gateway` | 部分 | REST API | WebSocket未実装、TODO多数 |
| `service_python` | 部分 | PyO3バインディング | 基本型のみ、pricer_*連携未完 |

#### Demo クレート (部分実装)

| クレート | 状態 | 備考 |
|----------|------|------|
| `upstream_systems` | **完了** | MarketDataProvider, TradeSource, FileSource実装済み |
| `downstream_systems` | **完了** | Regulatory, Settlement, RiskDashboard, ReportSink実装済み |
| `frictional_bank` | **未着手** | オーケストレーター未作成 |
| `demo_gui` | **未着手** | TUI/Web GUI未作成 |

### 1.2 ディレクトリ構造

```text
demo/
├── upstream_systems/     ✅ 完全実装
│   ├── Cargo.toml
│   └── src/
│       ├── market_data_provider/ (bloomberg_sim, reuters_sim, synthetic)
│       ├── trade_source/ (front_office, fpml_generator)
│       └── file_source/ (csv_generator)
├── downstream_systems/   ✅ 完全実装
│   ├── Cargo.toml
│   └── src/
│       ├── regulatory/ (regulator_api, audit_store)
│       ├── settlement/ (swift_receiver, netting_engine)
│       ├── risk_dashboard/ (websocket_sink, metrics_store)
│       └── report_sink/ (file_writer, email_sender)
├── frictional_bank/      ❌ 未作成
├── gui/                  ❌ 未作成
├── data/                 ❌ 未作成
└── notebooks/            ❌ 未作成
```

### 1.3 ワークスペース統合状況

**root Cargo.toml** にデモクレートは**未登録**:
```toml
[workspace]
members = [
    # adapter_*, infra_*, pricer_*, service_* のみ
    # demo/* は含まれていない
]
```

---

## 2. 要件実現可能性分析 (Requirements Feasibility)

### 2.1 要件-アセットマッピング

| 要件 | 既存アセット | ギャップ |
|------|------------|---------|
| **Req 1: Upstream Systems** | upstream_systems クレート | ✅ 完了 |
| **Req 2: Downstream Systems** | downstream_systems クレート | ✅ 完了 |
| **Req 3: Orchestrator** | なし | ❌ **Missing**: frictional_bank クレート全体 |
| **Req 4: GUI** | なし | ❌ **Missing**: demo_gui クレート全体 |
| **Req 5: Sample Data** | なし | ❌ **Missing**: demo/data/ ディレクトリ全体 |
| **Req 6: Jupyter Notebooks** | service_python (部分) | ⚠️ **Unknown**: ノートブック未作成、Python連携検証要 |
| **Req 7: A-I-P-S Integration** | 各クレート実装済み | ⚠️ **Constraint**: 統合テスト未実施 |
| **Req 8: Workspace Integration** | なし | ❌ **Missing**: Cargo.toml登録 |
| **Req 9: Demo Execution** | なし | ❌ **Missing**: CLIシナリオ実装 |

### 2.2 技術的ニーズ

#### 必須実装
1. **frictional_bank オーケストレーター**
   - EOD/Intraday/Stress シナリオ制御
   - 全クレート統合ワイヤリング
   - clap CLIインターフェース

2. **demo_gui TUIダッシュボード**
   - ratatui ベースのUI
   - service_gateway REST/WebSocket クライアント
   - リアルタイム更新表示

3. **サンプルデータ**
   - 取引CSV/XML
   - マーケットデータ
   - 設定ファイル

4. **ワークスペース統合**
   - root Cargo.toml への追加

#### 拡張実装 (service層)
1. **service_gateway WebSocket**
   - 現在REST APIのみ
   - リアルタイム配信にはWebSocket必要

2. **service_python 拡張**
   - pricer_* との実連携
   - DataFrame返却機能

### 2.3 複雑性シグナル

| コンポーネント | 複雑性 | 理由 |
|---------------|--------|------|
| frictional_bank | 中 | 既存クレート統合、非同期ワークフロー |
| demo_gui TUI | 中 | ratatui学習コスト、レイアウト設計 |
| Sample Data | 低 | 静的ファイル作成 |
| WebSocket統合 | 中 | 既存REST → WebSocket拡張 |
| Jupyter Notebooks | 低 | Python/PyO3既存基盤 |

---

## 3. 実装アプローチオプション

### Option A: 既存コンポーネント拡張 (Extend)

**適用対象**: service_gateway, service_python

#### service_gateway WebSocket拡張
- **対象ファイル**: `crates/service_gateway/src/`
- **変更内容**: axumにWebSocket routeを追加
- **互換性**: 既存REST APIに影響なし

#### service_python 拡張
- **対象ファイル**: `crates/service_python/src/bindings.rs`
- **変更内容**: pricer_* からの実データ取得連携
- **互換性**: 既存バインディングに影響なし

**トレードオフ**:
- ✅ 既存パターン活用
- ✅ 新規ファイル最小限
- ❌ service_* クレートの肥大化リスク

### Option B: 新規コンポーネント作成 (Create)

**適用対象**: frictional_bank, demo_gui, demo/data, demo/notebooks

#### frictional_bank クレート (新規)
- **責務**: デモシナリオオーケストレーション
- **統合点**: upstream_systems, downstream_systems, 全Neutryxクレート
- **インターフェース**: main.rs (CLI), lib.rs (ライブラリ)

#### demo_gui クレート (新規)
- **責務**: 可視化UI
- **統合点**: service_gateway REST/WebSocket API
- **インターフェース**: TUI (ratatui), Web (オプション)

**トレードオフ**:
- ✅ 責務の明確な分離
- ✅ 独立テスト可能
- ❌ 新規ファイル増加
- ❌ インターフェース設計が必要

### Option C: ハイブリッドアプローチ (Recommended)

**推奨**: Option B (新規) をメインに、Option A (拡張) を補完的に使用

#### フェーズ1: 基盤構築
1. root Cargo.toml にデモクレート登録
2. frictional_bank スケルトン作成
3. demo/data サンプルデータ作成

#### フェーズ2: 統合実装
1. frictional_bank シナリオ実装
2. service_gateway WebSocket拡張
3. upstream ↔ Adapter ↔ Pricer ↔ Service ↔ downstream 統合

#### フェーズ3: UI・ドキュメント
1. demo_gui TUI実装
2. Jupyter Notebooks作成
3. 統合テスト

**トレードオフ**:
- ✅ 段階的リスク低減
- ✅ 早期フィードバック可能
- ❌ 計画の複雑化

---

## 4. 工数・リスク評価

### 工数見積もり

| コンポーネント | 工数 | 根拠 |
|---------------|------|------|
| Workspace統合 (Cargo.toml) | **S** (1日) | 設定変更のみ |
| demo/data サンプルデータ | **S** (1-2日) | 静的ファイル作成 |
| frictional_bank オーケストレーター | **M** (3-5日) | 既存パターン活用、非同期統合 |
| service_gateway WebSocket | **M** (2-3日) | axum WebSocket追加 |
| demo_gui TUI | **M** (4-5日) | ratatui学習、レイアウト設計 |
| Jupyter Notebooks | **S** (2日) | 既存service_python活用 |
| 統合テスト | **M** (2-3日) | E2Eシナリオ検証 |

**合計**: L (1-2週間)

### リスク評価

| リスク | レベル | 緩和策 |
|--------|--------|--------|
| service_gateway ↔ pricer_* 統合 | 中 | 現在TODOコメント多数、実装要確認 |
| Enzyme環境依存 | 中 | pricer_pricing 除外オプション検討 |
| ratatui学習コスト | 低 | シンプルなUIから開始 |
| WebSocket実装 | 低 | axum-ws 標準的なパターン |

---

## 5. Research Needed (設計フェーズで調査)

1. **service_gateway TODOの実態調査**
   - `handlers.rs` のTODOコメント（pricer_pricing連携）
   - 実装優先度の決定

2. **pricer_pricing Enzyme依存の回避策**
   - stable Rustでのフォールバック
   - pricer_pricing 除外時のデモ制限

3. **TUIフレームワーク選定**
   - ratatui vs tui-rs
   - チャートライブラリ選定

4. **WebSocket メッセージ形式**
   - downstream_systems WebSocketSinkとの統合
   - プロトコル設計

---

## 6. 設計フェーズへの推奨事項

### 推奨アプローチ
**Option C (ハイブリッド)** を採用し、以下の順序で実装:

1. **Phase 1**: Workspace統合 + サンプルデータ (S)
2. **Phase 2**: frictional_bank オーケストレーター (M)
3. **Phase 3**: service_gateway WebSocket拡張 (M)
4. **Phase 4**: demo_gui TUI (M)
5. **Phase 5**: Jupyter Notebooks (S)
6. **Phase 6**: 統合テスト (M)

### キー決定事項
1. pricer_pricing (Enzyme) を含めるか、stable-only で進めるか
2. demo_gui の Web モードを Phase 1 に含めるか、後回しにするか
3. サンプルデータの規模（最小限 vs 本格的）

### Research Items to Carry Forward
- service_gateway TODO 実装詳細
- TUI チャートライブラリ選定
- WebSocket プロトコル設計
