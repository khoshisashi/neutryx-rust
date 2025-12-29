# Requirements Document

## Introduction

本仕様書は、`pricer_core` クレートにおける基盤型とトレイトの拡張を定義する。既存の `Priceable<T>` トレイトと `DayCountConvention` 列挙型を基盤として、Date 構造体ラッパー、Currency 列挙型、およびシリアライゼーション対応を追加する。

**対象パス**: `crates/pricer_core/src/traits/` および `crates/pricer_core/src/types/`

**設計制約**:
- ジェネリック: `trait Priceable<T: Float>` パターン維持（f64 ハードコード禁止）
- 列挙型ディスパッチ: Currency と DayCount は `Box<dyn>` 不使用
- 日付: `chrono` ラッパーの `Date` 構造体、標準シリアライゼーション形式
- ドキュメント: British English

## Requirements

### Requirement 1: Date 構造体ラッパー

**Objective:** As a 量的開発者, I want chrono をラップした型安全な Date 構造体, so that 日付演算と標準シリアライゼーションを統一できる

#### Acceptance Criteria

1. The pricer_core crate shall provide a `Date` struct that wraps `chrono::NaiveDate` internally
2. When Date is serialised, the pricer_core crate shall output ISO 8601 format (`YYYY-MM-DD`)
3. When Date is deserialised from a string, the pricer_core crate shall parse ISO 8601 format
4. The Date struct shall implement `Copy`, `Clone`, `Debug`, `PartialEq`, `Eq`, `PartialOrd`, `Ord`, `Hash`
5. When two Date values are subtracted, the pricer_core crate shall return the number of days as `i64`
6. The Date struct shall provide `today()`, `from_ymd(year, month, day)`, and `from_str(&str)` constructors
7. If an invalid date is constructed, the pricer_core crate shall return `Result<Date, DateError>`

### Requirement 2: Currency 列挙型

**Objective:** As a 量的開発者, I want 静的ディスパッチ可能な Currency 列挙型, so that Enzyme 最適化を維持しつつ通貨情報を扱える

#### Acceptance Criteria

1. The pricer_core crate shall provide a `Currency` enum with variants: `USD`, `EUR`, `GBP`, `JPY`, `CHF`
2. The Currency enum shall implement `Copy`, `Clone`, `Debug`, `PartialEq`, `Eq`, `Hash`
3. When Currency is serialised, the pricer_core crate shall output ISO 4217 currency code (e.g., "USD", "EUR")
4. When Currency is deserialised, the pricer_core crate shall parse ISO 4217 currency codes
5. The Currency enum shall provide a `code(&self) -> &'static str` method returning ISO 4217 code
6. The Currency enum shall provide a `decimal_places(&self) -> u8` method returning standard decimal precision
7. The Currency enum shall be marked `#[non_exhaustive]` to allow future extension

### Requirement 3: DayCountConvention シリアライゼーション拡張

**Objective:** As a 量的開発者, I want 既存の DayCountConvention にシリアライゼーション対応, so that 設定ファイルや API で日数計算規約を指定できる

#### Acceptance Criteria

1. The DayCountConvention enum shall implement `serde::Serialize` and `serde::Deserialize`
2. When DayCountConvention is serialised, the pricer_core crate shall output standard convention names (e.g., "ACT/365", "ACT/360", "30/360")
3. When DayCountConvention is deserialised, the pricer_core crate shall parse standard convention names case-insensitively
4. The DayCountConvention enum shall provide a `name(&self) -> &'static str` method returning the convention name
5. If an unknown convention name is deserialised, the pricer_core crate shall return a descriptive error

### Requirement 4: 汎用 Year Fraction 計算

**Objective:** As a 量的開発者, I want Date 構造体と統合された year_fraction 計算, so that NaiveDate への依存を隠蔽できる

#### Acceptance Criteria

1. The DayCountConvention enum shall provide `year_fraction_dates(&self, start: Date, end: Date) -> f64` method
2. When start > end, the year_fraction_dates method shall return a negative year fraction (not panic)
3. The pricer_core crate shall provide a `time_to_maturity_dates(start: Date, end: Date) -> f64` convenience function
4. While Date is used in year fraction calculations, the result shall be identical to using NaiveDate directly

### Requirement 5: Float トレイト境界の維持

**Objective:** As a 量的開発者, I want 既存のジェネリック Float 境界を維持, so that f64 と DualNumber の両方で動作することを保証できる

#### Acceptance Criteria

1. The `Float` trait alias in `traits/mod.rs` shall remain unchanged
2. The `Priceable<T: Float>` trait shall remain unchanged
3. The `Differentiable` marker trait shall remain unchanged
4. While implementing new types, the pricer_core crate shall not introduce f64-specific dependencies in traits

### Requirement 6: シリアライゼーション Feature Flag

**Objective:** As a 量的開発者, I want serde 依存をオプショナルに, so that シリアライゼーション不要時にコンパイル時間を短縮できる

#### Acceptance Criteria

1. The pricer_core crate shall provide a `serde` feature flag (enabled by default)
2. When `serde` feature is enabled, the Date, Currency, and DayCountConvention types shall implement Serialize/Deserialize
3. When `serde` feature is disabled, the pricer_core crate shall compile without serde dependency
4. The Cargo.toml shall declare serde as an optional dependency with `features = ["derive"]`

### Requirement 7: エラー型の拡張

**Objective:** As a 量的開発者, I want Date および Currency 関連のエラー型, so that 型安全なエラーハンドリングができる

#### Acceptance Criteria

1. The pricer_core crate shall provide `DateError` enum with variants: `InvalidDate`, `ParseError`
2. The pricer_core crate shall provide `CurrencyError` enum with variants: `UnknownCurrency`, `ParseError`
3. The DateError and CurrencyError enums shall implement `std::error::Error` and `std::fmt::Display`
4. If Date construction fails, the pricer_core crate shall return `Err(DateError::InvalidDate)`
5. If Currency parsing fails, the pricer_core crate shall return `Err(CurrencyError::UnknownCurrency)`

### Requirement 8: ドキュメンテーション標準

**Objective:** As a 量的開発者, I want British English での一貫したドキュメント, so that コードベース全体で統一された表記を維持できる

#### Acceptance Criteria

1. The pricer_core crate shall use British English spelling in all doc comments (e.g., "serialisation", "behaviour", "colour")
2. The Date struct shall include module-level documentation with usage examples
3. The Currency enum shall include variant-level documentation explaining each currency
4. While adding new public API, the pricer_core crate shall include `# Examples` section in doc comments
