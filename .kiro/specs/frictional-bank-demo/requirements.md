# Requirements Document

## Introduction

本ドキュメントは、FrictionalBankデモシステムの要件を定義します。FrictionalBankは、Neutryxライブラリの全機能（A-I-P-Sアーキテクチャ）を活用した仮想銀行システムのデモンストレーションであり、外部システムとの連携を含めた統合的なデモを提供します。

**参照仕様書**: `docs/demo/FRICTIONAL_BANK_SPEC.md`

## Requirements

### Requirement 1: Upstream Systems（上流システム）

**Objective:** デモ開発者として、マーケットデータ・取引データ・ファイルデータを生成するモックシステムを実装したい。これにより、Adapterレイヤーへの入力をシミュレートできる。

#### Acceptance Criteria 1.1-1.6

1. When upstream_systems クレートが初期化された場合, the MarketDataProvider shall Reuters風リアルタイムティッカー配信機能を提供する
2. When upstream_systems クレートが初期化された場合, the MarketDataProvider shall Bloomberg風スナップショットデータ配信機能を提供する
3. When upstream_systems クレートが初期化された場合, the MarketDataProvider shall 合成マーケットデータ生成機能を提供する
4. When フロントオフィスが取引をブッキングした場合, the TradeSource shall 取引データをFpML形式で生成する
5. When バッチ処理が開始された場合, the FileSource shall CSV/Parquetファイルを生成する
6. The upstream_systems crate shall pricer_core および pricer_models のみに依存する（A-I-P-Sルール準拠）

---

### Requirement 2: Downstream Systems（下流システム）

**Objective:** デモ開発者として、Serviceレイヤーからの出力を受信するモックシステムを実装したい。これにより、規制報告・決済・リスクダッシュボードへの配信をシミュレートできる。

#### Acceptance Criteria 2.1-2.6

1. When service_gateway から規制報告が送信された場合, the RegulatorApi shall 報告データを受信し監査証跡を保存する
2. When service_gateway から決済指図が送信された場合, the SwiftReceiver shall SWIFT風メッセージを受信する
3. When service_gateway から決済指図が送信された場合, the NettingEngine shall ネッティング処理を実行する
4. When service_gateway からリスクメトリクスが配信された場合, the WebSocketSink shall WebSocket経由でデータを受信する
5. When service_cli からレポートが出力された場合, the ReportSink shall ファイル出力およびメール送信（モック）を実行する
6. The downstream_systems crate shall pricer_core および pricer_risk のみに依存する（A-I-P-Sルール準拠）

---

### Requirement 3: FrictionalBank Orchestrator（オーケストレーター）

**Objective:** デモ運用者として、全Neutryxクレートを統合したワークフローを制御したい。これにより、EODバッチ・イントラデイ・ストレステストの各シナリオを実行できる。

#### Acceptance Criteria 3.1-3.6

1. When EODバッチシナリオが実行された場合, the Orchestrator shall upstream_systems → Adapter → Infra → Pricer → Service → downstream_systems の順でデータフローを制御する
2. When イントラデイシナリオが実行された場合, the Orchestrator shall リアルタイムティッカー受信からリスク更新・WebSocket配信までを制御する
3. When ストレステストシナリオが実行された場合, the Orchestrator shall 複数のストレスシナリオを並列実行する
4. The frictional_bank crate shall 全Neutryxクレート（adapter_*, infra_*, pricer_*, service_*）に依存する
5. The frictional_bank crate shall demo_config.tomlから設定を読み込む
6. When デモが起動された場合, the main.rs shall コマンドライン引数でシナリオを選択可能にする

---

### Requirement 4: GUI（ユーザーインターフェース）

**Objective:** デモユーザーとして、ターミナルまたはWebブラウザでリスクメトリクス・ポートフォリオ・取引を可視化したい。これにより、Neutryxの計算結果をリアルタイムで確認できる。

#### Acceptance Criteria 4.1-4.7

1. When TUIモードで起動した場合, the DemoGui shall rataturiベースのターミナルダッシュボードを表示する
2. When TUIダッシュボードが表示された場合, the PortfolioView shall ポートフォリオの構成と評価額を表示する
3. When TUIダッシュボードが表示された場合, the RiskView shall XVAメトリクス（CVA, DVA, FVA）とエクスポージャー（EE, EPE, PFE）を表示する
4. When TUIダッシュボードが表示された場合, the TradeBlotter shall 取引一覧と状態を表示する
5. When TUIダッシュボードが表示された場合, the Charts shall 価格推移・リスク推移のチャートを描画する
6. The demo_gui crate shall service_gateway のREST/WebSocket APIのみを利用する（直接Pricerに依存しない）
7. Where Webモードが有効な場合, the DemoGui shall ブラウザベースのダッシュボードを提供する（オプション）

---

### Requirement 5: Sample Data（サンプルデータ）

**Objective:** デモ開発者として、デモ実行に必要なサンプルデータを整備したい。これにより、即座にデモを実行できる環境を提供できる。

#### Acceptance Criteria 5.1-5.5

1. The demo/data/input/trades/ directory shall Equity/Rates/FX/CDS取引データ（CSV/XML）を含む
2. The demo/data/input/market_data/ directory shall イールドカーブ・ボラティリティ・クレジットスプレッド・スポットレートのサンプルデータを含む
3. The demo/data/input/counterparties/ directory shall カウンターパーティ・ネッティングセット・CSA契約のサンプルデータを含む
4. The demo/data/config/ directory shall neutryx.toml設定ファイル・休日カレンダー・通貨マスタを含む
5. The demo/data/output/ directory shall レポート・決済・規制報告の出力先ディレクトリ構造を提供する

---

### Requirement 6: Jupyter Notebooks（Python連携デモ）

**Objective:** クオンツアナリストとして、Jupyterノートブックからservice_pythonを通じてNeutryxの機能を利用したい。これにより、研究・分析ワークフローを実演できる。

#### Acceptance Criteria 6.1-6.6

1. The 01_pricing_demo.ipynb notebook shall 各資産クラス（Equity, Rates, FX, Credit）のプライシングを実演する
2. The 02_calibration_demo.ipynb notebook shall pricer_optimiserを使用したモデルキャリブレーションを実演する
3. The 03_risk_analysis.ipynb notebook shall ポートフォリオリスク分析とエクスポージャー計算を実演する
4. The 04_xva_calculation.ipynb notebook shall CVA/DVA/FVA計算を実演する
5. The 05_performance_bench.ipynb notebook shall Enzyme ADとnum-dualのパフォーマンス比較を実演する
6. When ノートブックが実行された場合, the service_python bindings shall 計算結果をPandas DataFrameとして返却する

---

### Requirement 7: A-I-P-S Data Flow Integration

**Objective:** システムアーキテクトとして、A-I-P-Sアーキテクチャの正しいデータフローをデモで実証したい。これにより、Neutryxのアーキテクチャ設計を理解できる。

#### Acceptance Criteria 7.1-7.6

1. The demo system shall Adapter → Infra → Pricer → Service の一方向データフローを厳守する
2. When 外部データが受信された場合, the Adapter layer shall データを正規化してInfraレイヤーに渡す
3. When 設定・マスタデータが必要な場合, the Infra layer shall 横断的にデータを提供する
4. When 価格計算が必要な場合, the Pricer layer shall pricer_core → pricer_models → pricer_optimiser → pricer_pricing → pricer_risk の順で処理する
5. When 結果が外部に出力される場合, the Service layer shall 唯一の出口として機能する
6. If A-I-P-S依存関係ルールに違反した場合, the cargo build shall コンパイルエラーを発生させる

---

### Requirement 8: Workspace Integration

**Objective:** デモ開発者として、デモクレートをNeutryxワークスペースに統合したい。これにより、既存のビルドシステムと一貫した開発体験を提供できる。

#### Acceptance Criteria 8.1-8.5

1. The root Cargo.toml shall demo/upstream_systems, demo/downstream_systems, demo/frictional_bank, demo/gui をワークスペースメンバーとして含む
2. When cargo build --workspace が実行された場合, the build system shall 全デモクレートを正常にビルドする
3. When cargo test --workspace が実行された場合, the test system shall 全デモクレートのテストを実行する
4. The demo crates shall workspace依存関係（tokio, serde, tracing等）を活用する
5. If pricer_pricingが含まれる場合, the build system shall nightly toolchainとEnzyme環境を要求する

---

### Requirement 9: Demo Scenarios Execution

**Objective:** デモ運用者として、コマンドラインからデモシナリオを簡単に実行したい。これにより、Neutryxの機能を効果的にデモンストレーションできる。

#### Acceptance Criteria 9.1-9.5

1. When `cargo run -p frictional_bank -- eod` が実行された場合, the Demo shall EODバッチ処理シナリオを実行する
2. When `cargo run -p frictional_bank -- intraday` が実行された場合, the Demo shall リアルタイムトレーディングシナリオを実行する
3. When `cargo run -p frictional_bank -- stress` が実行された場合, the Demo shall ストレステストシナリオを実行する
4. When デモシナリオが完了した場合, the Demo shall 実行結果サマリーを標準出力に表示する
5. While デモが実行中の場合, the Demo shall tracingによるログ出力を提供する
