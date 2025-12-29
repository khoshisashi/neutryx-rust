# 実装タスク: core-traits-types

## タスク概要

本実装計画は、Layer 1 (pricer_core) の基盤型システムとトレイト定義を段階的に構築する。全12要件を10のメジャータスクと28のサブタスクに分割し、ジェネリックFloat抽象化、enum dispatch、chrono基盤のDate型、標準シリアライゼーション、型安全なエラーハンドリングを実現する。

**実装フォーカス**:
- ジェネリックトレイト（Float, Priceable<T>, Differentiable）
- Enum dispatch型（Currency, DayCount）
- Newtype wrapper（Date, Time<T>）
- 構造化エラー（PricingError）
- モジュール構造とprelude

**テスト戦略**:
- ユニットテスト（各型・トレイト）
- Property-basedテスト（数学的不変条件）
- シリアライゼーションround-trip
- ISDA参照値検証

---

## 実装タスク

### Phase 1: 基盤トレイトとエラー型

- [ ] 1. Cargo.toml依存関係とfeature flag設定
- [x] 1.1 (P) Cargo.toml依存関係追加
  - num-traits 0.2.x をpricer_core dependencies追加
  - chrono 0.4.x のserde feature有効化 ✓
  - serde 1.0.x とserde_deriveのデフォルトfeature設定
  - proptest 1.x をdev-dependencies追加
  - approx 0.5.x既存設定確認
  - _Requirements: 1, 4, 11_
  - **実装済**: /workspaces/neutryx-rust/Cargo.tomlでchrono serde feature有効化

- [ ] 2. トレイト定義（traits/モジュール）
- [x] 2.1 (P) Float traitのre-export
  - traits/mod.rsにFloat traitを定義（num_traits::Floatのre-exportまたはsupertrait bound） ✓
  - pub use num_traits::Floatまたはpub trait Float: num_traits::Float + Copy + Clone + Debug + PartialOrd {} ✓
  - f64とnum_dual::DualNumberでコンパイルテスト（#[cfg(test)]モジュール） ✓
  - British Englishドキュメントコメント作成 ✓
  - _Requirements: 1_
  - **実装済**: /workspaces/neutryx-rust/crates/pricer_core/src/traits/mod.rs (pub use num_traits::Float + 4 tests)

- [x] 2.2 (P) Priceable<T: Float> trait定義
  - traits/priceable.rsでPriceable<T: Float> traitを定義 ✓
  - priceメソッドのシグネチャ（Result<T, PricingError>を返す） ✓
  - British Englishドキュメント（"behaviour", "parameterise"使用） ✓
  - トレイトオブジェクト安全性確認（object-safe制約） ✓
  - _Requirements: 2_
  - **実装済**: /workspaces/neutryx-rust/crates/pricer_core/src/traits/priceable.rs (Result<T, PricingError> signature + 5 tests)

- [x] 2.3 (P) Differentiable marker trait定義
  - traits/differentiable.rsでDifferentiableマーカートレイト定義 ✓
  - smoothing_epsilon制約をドキュメントで明記 ✓
  - Float値に対するif分岐禁止をドキュメント化 ✓
  - math::smoothing関数使用例をdoctestで提供 ✓
  - _Requirements: 3_
  - **実装済**: /workspaces/neutryx-rust/crates/pricer_core/src/traits/priceable.rs (marker trait with comprehensive docs)

- [ ] 3. PricingErrorエラー型
- [x] 3.1 (P) PricingError enum実装
  - types/error.rs新規作成 ✓
  - InvalidInput, NumericalInstability, ModelFailure, UnsupportedInstrument variantsをString contextで定義 ✓
  - std::error::ErrorとDebug, Displayトレイト実装 ✓
  - British Englishエラーメッセージ（Display実装） ✓
  - _Requirements: 8_
  - **実装済**: /workspaces/neutryx-rust/crates/pricer_core/src/types/error.rs (4 variants + Error/Display impl)

- [x] 3.2 (P) PricingErrorユニットテスト
  - 各variantのDisplay出力テスト ✓
  - std::error::Error traitメソッド確認 ✓
  - エラーメッセージの内容検証 ✓
  - _Requirements: 8_
  - **実装済**: /workspaces/neutryx-rust/crates/pricer_core/src/types/error.rs (6 unit tests)

### Phase 2: Date型とDay Count Convention

- [ ] 4. Date型実装
- [ ] 4.1 Date newtype定義
  - types/date.rs新規作成
  - Date(NaiveDate) newtypeを#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]で定義
  - from_ymd(year, month, day) -> Option<Self>コンストラクタ
  - inner() -> NaiveDateアクセサ
  - From<NaiveDate>とFrom<Date> for NaiveDate実装
  - _Requirements: 4_

- [ ] 4.2 Date serdeシリアライゼーション
  - #[derive(Serialize, Deserialize)]追加
  - #[serde(transparent)]属性でISO 8601パススルー
  - British Englishドキュメント（"serialisation"使用）
  - _Requirements: 4, 11_

- [ ] 4.3 Dateユニットテストとround-trip
  - from_ymd正常・異常系テスト
  - From/Into変換テスト
  - JSON/TOMLシリアライゼーションround-tripテスト
  - ISO 8601形式検証（"2025-12-29"）
  - _Requirements: 4, 11_

- [ ] 5. DayCount enum実装
- [ ] 5.1 DayCount enum定義とyear_fraction
  - 既存types/time.rsのDayCountConventionをDayCountにリネーム
  - Act365Fixed, Act360, Thirty360, ActActISDA variantsを#[non_exhaustive]で定義
  - year_fraction<T: Float>(&self, start: Date, end: Date) -> Tメソッド実装
  - Act365Fixed, Act360, Thirty360実装（既存ロジック移植、f64 → T: Float）
  - ActActISDA実装（QuantLib C++参照、年ごとの日数計算）
  - _Requirements: 6_

- [ ] 5.2 DayCount serdeシリアライゼーション
  - #[derive(Serialize, Deserialize)]追加
  - 文字列表現確認（"Act365Fixed", "Act360", etc.）
  - British Englishドキュメント
  - _Requirements: 6, 11_

- [ ] 5.3 DayCountユニットテストとISDA検証
  - 既存Act365, Act360, Thirty360テスト維持（型シグネチャ更新）
  - ActActISDA参照値テスト（ISDA test cases）
  - 同一日付で0.0を返すテスト
  - start > endでpanicテスト
  - _Requirements: 6, 10_

- [ ] 5.4* DayCount property-basedテスト
  - proptestで非負性テスト（year_fraction >= 0）
  - 単調性テスト（start <= mid <= end → yf(start, mid) + yf(mid, end) ≈ yf(start, end)）
  - Act365/Act360比率テスト（ratio ≈ 360/365）
  - _Requirements: 6, 10_

### Phase 3: Currency型とTime型

- [ ] 6. Currency enum実装
- [ ] 6.1 (P) Currency enum定義
  - types/currency.rs新規作成
  - USD, EUR, GBP, JPY, CHF, AUD, CAD variantsを#[non_exhaustive]で定義
  - #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
  - British Englishドキュメント（各variant説明）
  - _Requirements: 5_

- [ ] 6.2 (P) Currencyユニットテストとserde
  - 各variantのシリアライゼーションround-tripテスト
  - 文字列表現確認（"USD" == serde_json::to_string(&Currency::USD)）
  - _Requirements: 5, 11_

- [ ] 7. Time<T: Float>型実装
- [ ] 7.1 Time<T> newtype定義
  - types/time.rsにTime<T: Float>(T)を#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]で定義
  - new(value: T) -> Selfコンストラクタ（assert! value >= T::zero()）
  - as_years(&self) -> Tアクセサ
  - _Requirements: 7_

- [ ] 7.2 Time<T>算術演算実装
  - impl<T: Float> Add for Time<T>（Time + Time → Time）
  - impl<T: Float> Sub for Time<T>（Time - Time → Time）
  - impl<T: Float> Mul<T> for Time<T>（Time * T → Time）
  - impl<T: Float> Div<T> for Time<T>（Time / T → Time）
  - _Requirements: 7_

- [ ] 7.3 Time<T> From<(Date, Date, DayCount)>実装
  - impl<T: Float> From<(Date, Date, DayCount)> for Time<T>
  - DayCount::year_fraction呼び出しでTime(T)構築
  - _Requirements: 7_

- [ ] 7.4 Time<T>ユニットテスト
  - newコンストラクタ正常・panic系テスト
  - 算術演算テスト（加算、減算、乗算、除算）
  - From<(Date, Date, DayCount)>テスト
  - f64とDualNumberでのコンパイルテスト
  - _Requirements: 7_

- [ ] 7.5* Time<T> property-basedテスト
  - 交換律テスト（t1 + t2 == t2 + t1）
  - 結合律テスト（(t1 + t2) + t3 == t1 + (t2 + t3)）
  - _Requirements: 10_

### Phase 4: モジュール構造とprelude

- [ ] 8. モジュール構造整理とprelude
- [ ] 8.1 types/mod.rsとtraits/mod.rs更新
  - types/mod.rsにpub mod date, currency, error, timeを追加
  - traits/mod.rsにpub mod priceable, differentiableを追加、pub use Float
  - 既存dual.rsとtime.rsの統合確認
  - _Requirements: 9_

- [ ] 8.2 lib.rs prelude module実装
  - pub mod preludeを定義
  - traits::{Float, Priceable, Differentiable}をre-export
  - types::{Date, Currency, DayCount, Time, PricingError}をre-export
  - #[cfg(feature = "num-dual-mode")]でDualNumberもre-export
  - British Englishドキュメント
  - _Requirements: 9_

- [ ] 8.3 lib.rs direct re-exports
  - lib.rsにpub use traits::{Float, Priceable, Differentiable}追加
  - pub use types::{Date, Currency, DayCount, Time, PricingError}追加
  - 既存pub mod math, traits, typesは維持
  - _Requirements: 9_

- [ ] 8.4 prelude integrationテスト
  - use pricer_core::prelude::*;でのインポート確認
  - 各型の使用例doctestをprelude moduleに追加
  - _Requirements: 9, 10_

### Phase 5: 統合テストとBritish Englishドキュメント

- [ ] 9. 統合テストとdoctest
- [ ] 9.1 Float + DayCount統合テスト
  - f64でのyear_fraction計算テスト
  - DualNumberでのyear_fraction計算テスト（#[cfg(feature = "num-dual-mode")]）
  - _Requirements: 1, 6_

- [ ] 9.2 Date + DayCount + Time統合テスト
  - Date pairからTime<f64>生成フローテスト
  - Time<T>算術演算後にyear fractionへの逆変換テスト
  - _Requirements: 4, 6, 7_

- [ ] 9.3 Priceable trait stub実装とテスト
  - tests/モジュールでdummy instrument実装（impl<T: Float> Priceable<T>）
  - Result<T, PricingError>返却動作確認
  - Layer 2統合準備
  - _Requirements: 2_

- [ ] 9.4* British Englishドキュメント全体レビュー
  - 全pub itemsのdoc comments確認（"behaviour", "parameterise", "serialisation"等）
  - doctestの実行可能性確認
  - cargo doc --package pricer_coreでドキュメント生成確認
  - _Requirements: 2, 3, 4, 5, 6, 8, 10, 11, 12_

- [ ] 10. cargo test全体実行とカバレッジ
- [ ] 10.1 cargo test --package pricer_core実行
  - 全ユニットテスト通過確認
  - 全proptestテスト通過確認
  - 警告なし確認
  - _Requirements: 10_

- [ ] 10.2 テストカバレッジ確認
  - cargo tarpaulinまたはllvm-covで90%以上カバレッジ確認
  - 未カバレッジ箇所の妥当性確認
  - _Requirements: 10_

---

## タスクサマリー

- **メジャータスク**: 10
- **サブタスク**: 28
- **並列実行可能タスク**: 9 (P)マーク
- **オプションテスト**: 3 (*)マーク
- **要件カバレッジ**: 全12要件（Requirement 1-12）

### 要件マッピング検証

| 要件 | カバーするタスク |
|------|----------------|
| Requirement 1 | 1.1, 2.1, 9.1 |
| Requirement 2 | 2.2, 9.3 |
| Requirement 3 | 2.3 |
| Requirement 4 | 1.1, 4.1, 4.2, 4.3, 9.2 |
| Requirement 5 | 6.1, 6.2 |
| Requirement 6 | 5.1, 5.2, 5.3, 5.4, 9.1, 9.2 |
| Requirement 7 | 7.1, 7.2, 7.3, 7.4, 7.5, 9.2 |
| Requirement 8 | 3.1, 3.2 |
| Requirement 9 | 8.1, 8.2, 8.3, 8.4 |
| Requirement 10 | 5.3, 5.4, 7.5, 8.4, 9.4, 10.1, 10.2 |
| Requirement 11 | 1.1, 4.2, 4.3, 5.2, 6.2 |
| Requirement 12 | 9.4 |

### 依存関係フロー

```
Phase 1 (Parallel可能: 1.1, 2.1, 2.2, 2.3, 3.1, 3.2)
  ↓
Phase 2 (Date実装 → DayCount拡張)
  4.1 → 4.2 → 4.3
  5.1 → 5.2 → 5.3 → 5.4
  ↓
Phase 3 (Parallel可能: 6.1, 6.2 | 7.1 → 7.2 → 7.3 → 7.4 → 7.5)
  ↓
Phase 4 (モジュール統合)
  8.1 → 8.2 → 8.3 → 8.4
  ↓
Phase 5 (統合テストと検証)
  9.1, 9.2, 9.3 (Parallel可能)
  9.4 → 10.1 → 10.2
```

---

**生成日時**: 2025-12-29
**言語**: 日本語（British Englishドキュメントコメント）
**対象**: pricer_core Layer 1基盤型システム
