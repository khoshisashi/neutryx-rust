//! Time types and Day Count Conventions for financial calculations.
//!
//! This module provides date arithmetic and year fraction calculations
//! using industry-standard Day Count Conventions.

use chrono::NaiveDate;

/// Day Count Convention (year fraction convention).
///
/// # Variants
/// - `ActualActual365`: Actual days / 365 (standard for derivatives and UK bonds)
/// - `ActualActual360`: Actual days / 360 (common in money market instruments)
/// - `Thirty360`: Each month treated as 30 days, year as 360 days (US corporate bonds)
///
/// # Usage
///
/// ```
/// use pricer_core::types::time::DayCountConvention;
/// use chrono::NaiveDate;
///
/// let start = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
/// let end = NaiveDate::from_ymd_opt(2024, 7, 1).unwrap();
///
/// let act_365 = DayCountConvention::ActualActual365;
/// let year_fraction = act_365.year_fraction(start, end);
/// // 182 days / 365.0 ≈ 0.4986
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DayCountConvention {
    /// Actual/365 Fixed: actual_days / 365.0
    ///
    /// Used in:
    /// - Most derivatives markets
    /// - UK gilts
    /// - Japanese government bonds
    ActualActual365,

    /// Actual/360: actual_days / 360.0
    ///
    /// Used in:
    /// - Money market instruments
    /// - US Treasury bills
    /// - LIBOR-based instruments
    ActualActual360,

    /// 30/360 US Bond Basis
    ///
    /// Used in:
    /// - US corporate bonds
    /// - US agency bonds
    /// - Some municipal bonds
    ///
    /// Each month is treated as having 30 days, and the year as 360 days.
    Thirty360,
}

impl DayCountConvention {
    /// Calculate year fraction between two dates.
    ///
    /// # Arguments
    /// * `start` - Start date
    /// * `end` - End date
    ///
    /// # Returns
    /// Year fraction as f64 (e.g., 0.5 for 6 months, 1.0 for 1 year)
    ///
    /// # Panics
    /// Panics if `start > end`
    ///
    /// # Examples
    ///
    /// ```
    /// use pricer_core::types::time::DayCountConvention;
    /// use chrono::NaiveDate;
    ///
    /// let start = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
    /// let end = NaiveDate::from_ymd_opt(2024, 7, 1).unwrap();
    ///
    /// // Act/365
    /// let act_365 = DayCountConvention::ActualActual365;
    /// let yf_365 = act_365.year_fraction(start, end);
    /// assert!((yf_365 - 0.4986).abs() < 0.001);
    ///
    /// // Act/360
    /// let act_360 = DayCountConvention::ActualActual360;
    /// let yf_360 = act_360.year_fraction(start, end);
    /// assert!((yf_360 - 0.5056).abs() < 0.001);
    /// ```
    pub fn year_fraction(&self, start: NaiveDate, end: NaiveDate) -> f64 {
        assert!(
            start <= end,
            "start date must be less than or equal to end date"
        );

        match self {
            DayCountConvention::ActualActual365 => {
                let days = (end - start).num_days();
                days as f64 / 365.0
            }
            DayCountConvention::ActualActual360 => {
                let days = (end - start).num_days();
                days as f64 / 360.0
            }
            DayCountConvention::Thirty360 => {
                // 30/360 US Bond Basis implementation
                let y1 = start.year();
                let m1 = start.month();
                let d1 = start.day();

                let y2 = end.year();
                let m2 = end.month();
                let d2 = end.day();

                // 30/360 US adjustments
                let d1_adj = if d1 == 31 { 30 } else { d1 };
                let d2_adj = if d2 == 31 && d1_adj == 30 { 30 } else { d2 };

                let days = 360 * (y2 - y1) + 30 * (m2 as i32 - m1 as i32) + (d2_adj - d1_adj);
                days as f64 / 360.0
            }
        }
    }
}

/// Calculate time to maturity using default convention (Act/365).
///
/// # Arguments
/// * `start` - Valuation date
/// * `end` - Maturity date
///
/// # Returns
/// Time to maturity in years (Act/365 convention)
///
/// # Panics
/// Panics if `start > end`
///
/// # Examples
///
/// ```
/// use pricer_core::types::time::time_to_maturity;
/// use chrono::NaiveDate;
///
/// let valuation_date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
/// let maturity_date = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
///
/// let ttm = time_to_maturity(valuation_date, maturity_date);
/// assert!((ttm - 1.0027).abs() < 0.001); // ~1 year (366 days in 2024 leap year)
/// ```
pub fn time_to_maturity(start: NaiveDate, end: NaiveDate) -> f64 {
    DayCountConvention::ActualActual365.year_fraction(start, end)
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    // Task 5.4: Day Count Convention unit tests

    #[test]
    fn test_act_365_known_dates() {
        // 2024-01-01 to 2024-07-01 is 182 days
        let start = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let end = NaiveDate::from_ymd_opt(2024, 7, 1).unwrap();

        let convention = DayCountConvention::ActualActual365;
        let result = convention.year_fraction(start, end);

        let expected = 182.0 / 365.0; // ≈ 0.4986
        assert_relative_eq!(result, expected, epsilon = 1e-10);
    }

    #[test]
    fn test_act_360_known_dates() {
        // 2024-01-01 to 2024-07-01 is 182 days
        let start = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let end = NaiveDate::from_ymd_opt(2024, 7, 1).unwrap();

        let convention = DayCountConvention::ActualActual360;
        let result = convention.year_fraction(start, end);

        let expected = 182.0 / 360.0; // ≈ 0.5056
        assert_relative_eq!(result, expected, epsilon = 1e-10);
    }

    #[test]
    fn test_thirty_360_known_dates() {
        // 2024-01-01 to 2024-07-01
        // Years: 0, Months: 6, Days: 0 (1st to 1st)
        // Total days in 30/360: 0*360 + 6*30 + 0 = 180
        let start = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let end = NaiveDate::from_ymd_opt(2024, 7, 1).unwrap();

        let convention = DayCountConvention::Thirty360;
        let result = convention.year_fraction(start, end);

        let expected = 180.0 / 360.0; // 0.5
        assert_relative_eq!(result, expected, epsilon = 1e-10);
    }

    #[test]
    fn test_thirty_360_with_31st_days() {
        // Test 30/360 adjustments for 31st day of month
        // 2024-01-31 to 2024-03-31
        // d1 = 31 -> 30 (adjusted)
        // d2 = 31, d1_adj = 30 -> 30 (adjusted)
        // Months: 2, Days: 0 (after adjustment)
        // Total: 2*30 + 0 = 60 days
        let start = NaiveDate::from_ymd_opt(2024, 1, 31).unwrap();
        let end = NaiveDate::from_ymd_opt(2024, 3, 31).unwrap();

        let convention = DayCountConvention::Thirty360;
        let result = convention.year_fraction(start, end);

        let expected = 60.0 / 360.0; // ≈ 0.1667
        assert_relative_eq!(result, expected, epsilon = 1e-10);
    }

    #[test]
    fn test_time_to_maturity_matches_act_365() {
        let start = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let end = NaiveDate::from_ymd_opt(2024, 7, 1).unwrap();

        let ttm = time_to_maturity(start, end);
        let act_365 = DayCountConvention::ActualActual365.year_fraction(start, end);

        assert_relative_eq!(ttm, act_365, epsilon = 1e-10);
    }

    #[test]
    fn test_same_date_returns_zero() {
        let date = NaiveDate::from_ymd_opt(2024, 6, 15).unwrap();

        let act_365 = DayCountConvention::ActualActual365;
        assert_eq!(act_365.year_fraction(date, date), 0.0);

        let act_360 = DayCountConvention::ActualActual360;
        assert_eq!(act_360.year_fraction(date, date), 0.0);

        let thirty_360 = DayCountConvention::Thirty360;
        assert_eq!(thirty_360.year_fraction(date, date), 0.0);
    }

    #[test]
    fn test_one_year_period() {
        let start = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let end = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();

        // 2024 is a leap year, so 366 days
        let act_365 = DayCountConvention::ActualActual365;
        let result_365 = act_365.year_fraction(start, end);
        assert_relative_eq!(result_365, 366.0 / 365.0, epsilon = 1e-10);

        let act_360 = DayCountConvention::ActualActual360;
        let result_360 = act_360.year_fraction(start, end);
        assert_relative_eq!(result_360, 366.0 / 360.0, epsilon = 1e-10);

        let thirty_360 = DayCountConvention::Thirty360;
        let result_30_360 = thirty_360.year_fraction(start, end);
        assert_relative_eq!(result_30_360, 1.0, epsilon = 1e-10); // Exactly 1 year in 30/360
    }

    #[test]
    #[should_panic(expected = "start date must be less than or equal to end date")]
    fn test_year_fraction_panics_on_reverse_dates() {
        let start = NaiveDate::from_ymd_opt(2024, 7, 1).unwrap();
        let end = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();

        let convention = DayCountConvention::ActualActual365;
        convention.year_fraction(start, end);
    }

    #[test]
    #[should_panic(expected = "start date must be less than or equal to end date")]
    fn test_time_to_maturity_panics_on_reverse_dates() {
        let start = NaiveDate::from_ymd_opt(2024, 7, 1).unwrap();
        let end = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();

        time_to_maturity(start, end);
    }

    #[test]
    fn test_act_365_vs_act_360_ratio() {
        // Verify that Act/360 results are approximately 360/365 times Act/365
        let start = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let end = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();

        let result_365 = DayCountConvention::ActualActual365.year_fraction(start, end);
        let result_360 = DayCountConvention::ActualActual360.year_fraction(start, end);

        // ratio should be close to 365/360
        let ratio = result_365 / result_360;
        assert_relative_eq!(ratio, 360.0 / 365.0, epsilon = 1e-10);
    }

    // Task 6.2: Property-based tests for Day Count Convention
    #[cfg(test)]
    mod property_tests {
        use super::*;
        use proptest::prelude::*;

        // Generate valid NaiveDate values (avoiding edge cases)
        fn date_strategy() -> impl Strategy<Value = NaiveDate> {
            (2000i32..2100i32, 1u32..13u32, 1u32..29u32).prop_filter_map(
                "valid date",
                |(year, month, day)| NaiveDate::from_ymd_opt(year, month, day),
            )
        }

        proptest! {
            #![proptest_config(ProptestConfig::with_cases(1000))]

            #[test]
            fn test_year_fraction_non_negative(
                start in date_strategy(),
                end in date_strategy(),
            ) {
                // Only test when start <= end
                if start <= end {
                    let conventions = [
                        DayCountConvention::ActualActual365,
                        DayCountConvention::ActualActual360,
                        DayCountConvention::Thirty360,
                    ];

                    for convention in &conventions {
                        let result = convention.year_fraction(start, end);
                        assert!(
                            result >= 0.0,
                            "{:?}.year_fraction({}, {}) = {} should be non-negative",
                            convention, start, end, result
                        );
                    }
                }
            }

            #[test]
            fn test_act_365_vs_act_360_ratio_property(
                start in date_strategy(),
                end in date_strategy(),
            ) {
                // Only test when start < end (avoid division by zero)
                if start < end {
                    let result_365 = DayCountConvention::ActualActual365.year_fraction(start, end);
                    let result_360 = DayCountConvention::ActualActual360.year_fraction(start, end);

                    // Act/360 should be approximately 365/360 times Act/365
                    // result_360 = days / 360, result_365 = days / 365
                    // So result_365 / result_360 = 360 / 365
                    if result_360 > 0.0 {
                        let ratio = result_365 / result_360;
                        assert_relative_eq!(ratio, 360.0 / 365.0, epsilon = 1e-10);
                    }
                }
            }

            #[test]
            fn test_time_to_maturity_matches_act_365_property(
                start in date_strategy(),
                end in date_strategy(),
            ) {
                // Only test when start <= end
                if start <= end {
                    let ttm = time_to_maturity(start, end);
                    let act_365 = DayCountConvention::ActualActual365.year_fraction(start, end);

                    assert_relative_eq!(ttm, act_365, epsilon = 1e-10);
                }
            }

            #[test]
            fn test_year_fraction_is_monotonic(
                start in date_strategy(),
                mid in date_strategy(),
                end in date_strategy(),
            ) {
                // Sort dates to ensure start <= mid <= end
                let mut dates = [start, mid, end];
                dates.sort();
                let [d1, d2, d3] = dates;

                let conventions = [
                    DayCountConvention::ActualActual365,
                    DayCountConvention::ActualActual360,
                    DayCountConvention::Thirty360,
                ];

                for convention in &conventions {
                    let yf_1_2 = convention.year_fraction(d1, d2);
                    let yf_2_3 = convention.year_fraction(d2, d3);
                    let yf_1_3 = convention.year_fraction(d1, d3);

                    // Year fraction should be additive (or close to it)
                    // yf(d1, d3) ≈ yf(d1, d2) + yf(d2, d3)
                    assert_relative_eq!(
                        yf_1_3,
                        yf_1_2 + yf_2_3,
                        epsilon = 0.01, // Small tolerance for 30/360 rounding
                        max_relative = 0.01
                    );
                }
            }

            #[test]
            fn test_same_date_always_returns_zero_property(
                date in date_strategy(),
            ) {
                let conventions = [
                    DayCountConvention::ActualActual365,
                    DayCountConvention::ActualActual360,
                    DayCountConvention::Thirty360,
                ];

                for convention in &conventions {
                    let result = convention.year_fraction(date, date);
                    assert_eq!(
                        result, 0.0,
                        "{:?}.year_fraction({}, {}) should be 0.0",
                        convention, date, date
                    );
                }
            }

            #[test]
            fn test_year_fraction_finite(
                start in date_strategy(),
                end in date_strategy(),
            ) {
                // Only test when start <= end
                if start <= end {
                    let conventions = [
                        DayCountConvention::ActualActual365,
                        DayCountConvention::ActualActual360,
                        DayCountConvention::Thirty360,
                    ];

                    for convention in &conventions {
                        let result = convention.year_fraction(start, end);
                        assert!(
                            result.is_finite(),
                            "{:?}.year_fraction({}, {}) = {} should be finite",
                            convention, start, end, result
                        );
                    }
                }
            }
        }
    }
}
