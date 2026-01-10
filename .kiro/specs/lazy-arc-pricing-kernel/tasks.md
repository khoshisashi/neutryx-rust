# Implementation Plan

## Tasks

- [x] 1. デモ用型定義モジュールの実装（pricer_models L2）
- [x] 1.1 (P) モデル enum と状態遷移メソッドを実装する
  - BlackScholes（vol パラメータ）と HullWhite（mean_rev, vol パラメータ）の構造体を定義
  - ModelEnum で両モデルをラップし、静的ディスパッチを実現
  - evolve メソッドで状態遷移ロジックを実装（簡略化版）
  - _Requirements: 1.1_

- [x] 1.2 (P) 商品 enum とボラティリティ依存判定を実装する
  - VanillaSwap（fixed_rate）と CmsSwap（fixed_rate）の構造体を定義
  - InstrumentEnum で両商品をラップ
  - requires_vol() メソッドで VanillaSwap は false、CmsSwap は true を返す
  - _Requirements: 1.2, 1.3, 1.4_

- [x] 1.3 (P) マーケットオブジェクト enum を実装する
  - FlatCurve（rate）構造体と CurveEnum を定義
  - get_df(t) で割引ファクター exp(-rate * t) を計算
  - SabrVolSurface（alpha）構造体と VolSurfaceEnum を定義
  - _Requirements: 1.5, 1.6_

- [x] 1.4 モジュール公開と依存性設定を行う
  - demo モジュールを pricer_models/src/demo.rs に作成
  - lib.rs で pub mod demo を追加
  - pricer_core からの Currency 再エクスポートを確認
  - Cargo.toml で pricer_core 依存のみを確認（他の pricer crate への依存なし）
  - _Requirements: 1.7, 6.1_

- [x] 2. 遅延評価キャッシュ機構の実装（pricer_optimiser L2.5）
- [x] 2.1 (P) MarketProvider 構造体とキャッシュを実装する
  - RwLock<HashMap<Currency, Arc<CurveEnum>>> で curve_cache を保持
  - RwLock<HashMap<Currency, Arc<VolSurfaceEnum>>> で vol_cache を保持
  - new() でキャッシュを空で初期化
  - _Requirements: 2.1, 2.2_

- [x] 2.2 カーブの遅延取得メソッドを実装する
  - get_curve(ccy) でまず読み取りロックでキャッシュを確認
  - キャッシュヒット時は即座に Arc をクローンして返却
  - キャッシュミス時は書き込みロックを取得し、ダブルチェック後に構築
  - 構築時に "[Optimiser] Bootstrapping Yield Curve for {currency}..." をログ出力
  - _Requirements: 2.3, 2.4, 2.7, 5.1_

- [x] 2.3 ボラティリティサーフェスの遅延取得メソッドを実装する
  - get_vol(ccy) でカーブと同様の遅延評価パターンを実装
  - キャッシュヒット時はログなしで Arc を返却
  - キャッシュミス時に "[Optimiser] Calibrating SABR Surface for {currency}..." をログ出力
  - _Requirements: 2.5, 2.6, 5.2_

- [x] 2.4 モジュール公開と依存性設定を行う
  - provider.rs を pricer_optimiser/src/ に作成
  - lib.rs で pub mod provider を追加
  - Cargo.toml で pricer_models と pricer_core のみに依存を確認
  - _Requirements: 6.2_

- [x] 3. プライシングコンテキストとカーネルの実装（pricer_pricing L3）
- [x] 3.1 (P) 参照ベースの PricingContext を実装する
  - ライフタイム 'a を持つ PricingContext 構造体を定義
  - discount_curve: &'a CurveEnum で割引カーブ参照を保持
  - adjustment_vol: Option<&'a VolSurfaceEnum> でオプショナルな Vol 参照を保持
  - _Requirements: 3.1, 3.2_

- [x] 3.2 プライシングカーネル関数を実装する
  - price_single_trade(&ModelEnum, &InstrumentEnum, &PricingContext) -> f64 を定義
  - モデルの evolve で状態を更新
  - VanillaSwap の場合は state - fixed_rate でペイオフ計算
  - CmsSwap の場合は adjustment_vol からコンベクシティ調整を適用
  - 最後に discount_curve.get_df(1.0) で割引
  - HashMap 検索や動的アロケーションを一切使用しない
  - _Requirements: 3.3, 3.4, 3.5, 3.6_

- [x] 3.3 モジュール公開と依存性設定を行う
  - context.rs を pricer_pricing/src/ に作成
  - lib.rs で pub mod context を追加
  - Cargo.toml で pricer_models と pricer_core のみに依存を確認
  - _Requirements: 6.3_

- [x] 4. ポートフォリオオーケストレーションの実装（pricer_risk L4）
- [x] 4.1 (P) Trade 構造体を実装する
  - id: String, ccy: Currency, model: ModelEnum, instrument: InstrumentEnum フィールドを定義
  - 必要に応じて Debug, Clone を derive
  - _Requirements: 4.1_

- [x] 4.2 Pull-then-Push 並列処理関数を実装する
  - run_portfolio_pricing(&[Trade], &MarketProvider) を定義
  - rayon::par_iter() でトレードを並列処理
  - 各トレードで market.get_curve(trade.ccy) を呼び出し（Pull フェーズ）
  - trade.instrument.requires_vol() が true の場合のみ market.get_vol(trade.ccy) を呼び出し
  - Arc から参照を借用して PricingContext を構築
  - price_single_trade を呼び出してPVを計算（Push フェーズ）
  - _Requirements: 4.2, 4.3, 4.4, 4.5, 4.6, 4.7_

- [x] 4.3 モジュール公開と依存性設定を行う
  - demo.rs を pricer_risk/src/ に作成
  - lib.rs で pub mod demo を追加
  - Cargo.toml で pricer_models, pricer_optimiser, pricer_pricing, rayon に依存を設定
  - _Requirements: 6.4_

- [x] 5. 統合とエンドツーエンド検証
- [x] 5.1 CLI デモエントリポイントを実装する
  - service_cli に demo サブコマンドまたは main 関数を追加
  - MarketProvider を初期化
  - 4つのトレード（USD Vanilla x2, USD CMS, JPY Vanilla）を作成
  - run_portfolio_pricing を呼び出して実行
  - Cargo.toml で pricer_models, pricer_optimiser, pricer_risk に依存を設定
  - _Requirements: 6.5_

- [x] 5.2 ログ出力パターンでアーキテクチャを検証する
  - USD カーブのブートストラップログが1回のみ出力されることを確認（Arc キャッシュ検証）
  - VanillaSwap のみ処理時に Vol キャリブレーションログが出ないことを確認（遅延評価検証）
  - CmsSwap 処理時に USD Vol キャリブレーションログが1回のみ出力されることを確認
  - JPY カーブのブートストラップログが出力されることを確認
  - _Requirements: 5.3, 5.4, 5.5_

- [x]* 5.3 ユニットテストを追加する
  - ModelEnum::evolve の状態遷移テスト
  - InstrumentEnum::requires_vol() の戻り値テスト
  - CurveEnum::get_df() の計算精度テスト
  - MarketProvider のキャッシュヒット/ミス動作テスト
  - _Requirements: 1.1, 1.2, 1.3, 1.4, 1.5, 2.3, 2.4, 2.5, 2.6_

## Requirements Coverage

| Requirement | Tasks |
|-------------|-------|
| 1.1 | 1.1 |
| 1.2 | 1.2 |
| 1.3 | 1.2 |
| 1.4 | 1.2 |
| 1.5 | 1.3 |
| 1.6 | 1.3 |
| 1.7 | 1.4 |
| 2.1 | 2.1 |
| 2.2 | 2.1 |
| 2.3 | 2.2 |
| 2.4 | 2.2 |
| 2.5 | 2.3 |
| 2.6 | 2.3 |
| 2.7 | 2.2 |
| 3.1 | 3.1 |
| 3.2 | 3.1 |
| 3.3 | 3.2 |
| 3.4 | 3.2 |
| 3.5 | 3.2 |
| 3.6 | 3.2 |
| 4.1 | 4.1 |
| 4.2 | 4.2 |
| 4.3 | 4.2 |
| 4.4 | 4.2 |
| 4.5 | 4.2 |
| 4.6 | 4.2 |
| 4.7 | 4.2 |
| 5.1 | 2.2 |
| 5.2 | 2.3 |
| 5.3 | 5.2 |
| 5.4 | 5.2 |
| 5.5 | 5.2 |
| 6.1 | 1.4 |
| 6.2 | 2.4 |
| 6.3 | 3.3 |
| 6.4 | 4.3 |
| 6.5 | 5.1 |
