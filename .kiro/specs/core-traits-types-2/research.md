# Research & Design Decisions

## Summary
- **Feature**: `core-traits-types-2`
- **Discovery Scope**: Extension（既存システムの拡張）
- **Key Findings**:
  - serde 依存は既に Cargo.toml に存在（feature flag 追加のみ必要）
  - `DayCountConvention` は `chrono::NaiveDate` を直接使用中（Date ラッパー統合が必要）
  - エラーパターンは `PricingError` で確立済み（同様のパターンで DateError/CurrencyError を追加）

## Research Log

### 既存コードベース分析

- **Context**: 拡張ポイントと既存パターンの特定
- **Sources Consulted**:
  - `crates/pricer_core/src/types/time.rs`
  - `crates/pricer_core/src/types/error.rs`
  - `crates/pricer_core/src/traits/mod.rs`
  - `crates/pricer_core/Cargo.toml`
- **Findings**:
  - `DayCountConvention` は `chrono::NaiveDate` を引数として直接受け取る `year_fraction` メソッドを持つ
  - `PricingError` は `thiserror` を使用せず手動で `Display` と `Error` を実装
  - `serde` は workspace 依存として既に追加されているが、feature flag 制御なし
  - `Float` トレイトは `num_traits::Float` の re-export
- **Implications**:
  - 新しい `Date` 型は `NaiveDate` をラップし、既存 API との後方互換性を維持
  - エラー型は既存パターンに従い手動実装（thiserror 使用せず）
  - serde feature flag を追加し、条件付きコンパイルを導入

### chrono 日付シリアライゼーション

- **Context**: ISO 8601 形式での Date シリアライゼーション方法
- **Sources Consulted**: chrono crate documentation
- **Findings**:
  - `chrono::NaiveDate` は `serde` feature で `Serialize`/`Deserialize` をサポート
  - デフォルト形式は ISO 8601 (`YYYY-MM-DD`)
  - カスタム形式には `chrono::serde` モジュールを使用可能
- **Implications**:
  - `Date` struct の serde 実装は内部 `NaiveDate` に委譲可能
  - `#[serde(transparent)]` を使用して薄いラッパーとして実装

### Currency ISO 4217 標準

- **Context**: 通貨コードと小数点以下桁数の標準値
- **Sources Consulted**: ISO 4217 standard
- **Findings**:
  - USD, EUR, GBP, CHF: 2 decimal places
  - JPY: 0 decimal places（円は小数なし）
  - コードは 3 文字のアルファベット大文字
- **Implications**:
  - `Currency::decimal_places()` は通貨ごとに異なる値を返す
  - シリアライゼーションは 3 文字コードを使用

## Architecture Pattern Evaluation

| Option | Description | Strengths | Risks / Limitations | Notes |
|--------|-------------|-----------|---------------------|-------|
| Newtype Pattern | `Date(NaiveDate)` ラッパー | 型安全、ゼロコスト抽象化 | Deref 必要時の追加コード | Rust イディオム、採用 |
| Type Alias | `type Date = NaiveDate` | 実装コストゼロ | 型安全性なし、シリアライゼーションカスタマイズ不可 | 却下 |

## Design Decisions

### Decision: Newtype Pattern for Date

- **Context**: chrono::NaiveDate のラッパー実装方法
- **Alternatives Considered**:
  1. Type alias — シンプルだが型安全性なし
  2. Newtype with Deref — 便利だが抽象化が漏れる
  3. Newtype without Deref — 明示的だが API 追加が必要
- **Selected Approach**: Newtype without Deref、必要なメソッドを明示的に実装
- **Rationale**: 型安全性を最大化し、API サーフェスを制御
- **Trade-offs**: 追加のメソッド実装が必要だが、将来の変更に対する柔軟性を確保
- **Follow-up**: `into_inner()` メソッドで chrono API へのアクセスを提供

### Decision: serde Feature Flag

- **Context**: シリアライゼーション依存のオプション化
- **Alternatives Considered**:
  1. Always-on serde — シンプルだがコンパイル時間増
  2. Feature flag (default on) — 柔軟性と利便性のバランス
  3. Feature flag (default off) — 明示的だが不便
- **Selected Approach**: `serde` feature flag、default で有効
- **Rationale**: 既存ユーザーへの後方互換性を維持しつつオプション化
- **Trade-offs**: feature flag 管理の複雑さが増加
- **Follow-up**: `#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]` パターンを使用

### Decision: DayCountConvention カスタムシリアライゼーション

- **Context**: 業界標準名称（"ACT/365"）でのシリアライゼーション
- **Alternatives Considered**:
  1. derive(Serialize) — variant 名がそのまま出力される（"ActualActual365"）
  2. `#[serde(rename)]` — 各 variant に rename 指定
  3. カスタム impl — 完全な制御、case-insensitive パース
- **Selected Approach**: カスタム Serialize/Deserialize 実装
- **Rationale**: 業界標準名称と case-insensitive パースの両方を実現
- **Trade-offs**: 実装コードが増えるが、ユーザビリティ向上
- **Follow-up**: "ACT/365", "Actual/365", "Act365" などのエイリアスをサポート

## Risks & Mitigations

- **Risk 1**: Date と NaiveDate の混在による API 混乱
  - **Mitigation**: 既存 `year_fraction(NaiveDate)` は維持し、新 API `year_fraction_dates(Date)` を追加
- **Risk 2**: serde feature flag 無効時のコンパイルエラー
  - **Mitigation**: CI で両方の feature 設定をテスト
- **Risk 3**: Currency enum の将来拡張
  - **Mitigation**: `#[non_exhaustive]` を使用し、将来の variant 追加に備える

## References

- [chrono crate documentation](https://docs.rs/chrono/) — 日付型と serde サポート
- [ISO 4217](https://www.iso.org/iso-4217-currency-codes.html) — 通貨コード標準
- [Rust Newtype Pattern](https://doc.rust-lang.org/rust-by-example/generics/new_types.html) — 型安全なラッパーパターン
