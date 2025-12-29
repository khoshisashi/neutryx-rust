//! Integration tests for pricer_core module exports and public API.
//!
//! Task 8.1: Module export integration tests

use chrono::NaiveDate;
use pricer_core::math::smoothing::{smooth_abs, smooth_indicator, smooth_max, smooth_min};
use pricer_core::traits::priceable::{Differentiable, Priceable};
use pricer_core::types::time::{time_to_maturity, DayCountConvention};

#[cfg(feature = "num-dual-mode")]
use pricer_core::types::dual::DualNumber;

// Test absolute path imports from lib.rs

#[test]
fn test_smoothing_module_exports() {
    // Verify that smoothing functions are accessible via absolute paths
    let result = smooth_max(3.0, 5.0, 1e-6);
    assert!((result - 5.0).abs() < 1e-3);

    let result = smooth_min(3.0, 5.0, 1e-6);
    assert!((result - 3.0).abs() < 1e-3);

    let result = smooth_indicator(0.0, 1e-3);
    assert!((result - 0.5).abs() < 0.01);

    let result = smooth_abs(3.5, 1e-6);
    assert!((result - 3.5).abs() < 1e-3);
}

#[test]
fn test_traits_module_exports() {
    // Verify that traits are accessible and can be implemented

    enum TestInstrument {
        FixedValue(f64),
    }

    impl Priceable<f64> for TestInstrument {
        fn price(&self) -> f64 {
            match self {
                TestInstrument::FixedValue(val) => *val,
            }
        }
    }

    struct TestGradient {
        slope: f64,
    }

    impl Differentiable<f64> for TestGradient {
        fn gradient(&self) -> f64 {
            self.slope
        }
    }

    let instrument = TestInstrument::FixedValue(100.0);
    assert_eq!(instrument.price(), 100.0);

    let gradient = TestGradient { slope: 2.5 };
    assert_eq!(gradient.gradient(), 2.5);
}

#[test]
fn test_time_module_exports() {
    // Verify chrono integration and time_to_maturity function

    let start = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
    let end = NaiveDate::from_ymd_opt(2024, 7, 1).unwrap();

    let ttm = time_to_maturity(start, end);
    assert!((ttm - 182.0 / 365.0).abs() < 1e-10);

    let act_365 = DayCountConvention::ActualActual365;
    let yf = act_365.year_fraction(start, end);
    assert_eq!(yf, ttm);
}

#[cfg(feature = "num-dual-mode")]
#[test]
fn test_dual_number_integration() {
    // Verify that DualNumber works with smoothing functions

    let a = DualNumber::from(3.0).derivative();
    let b = DualNumber::from(5.0);

    let result = smooth_max(a, b, 1e-6);
    assert!((result.re() - 5.0).abs() < 1e-3);
    assert!(result.eps.is_finite());
}

#[test]
fn test_cross_module_integration() {
    // Test interaction between different modules

    // Use smoothing functions with time calculations
    let start = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
    let end1 = NaiveDate::from_ymd_opt(2024, 7, 1).unwrap();
    let end2 = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();

    let ttm1 = time_to_maturity(start, end1);
    let ttm2 = time_to_maturity(start, end2);

    // Use smooth_max to get the longer maturity
    let max_ttm = smooth_max(ttm1, ttm2, 1e-6);
    assert!((max_ttm - ttm2).abs() < 1e-3);

    // Implement a simple pricing model using traits
    struct TimeDependent {
        ttm: f64,
    }

    impl Priceable<f64> for TimeDependent {
        fn price(&self) -> f64 {
            // Simple exponential decay model
            (-0.05 * self.ttm).exp() * 100.0
        }
    }

    let model = TimeDependent { ttm: ttm1 };
    let price = model.price();
    assert!(price > 0.0 && price <= 100.0);
}

#[test]
fn test_all_public_modules_accessible() {
    // Verify that all three public modules are accessible

    // math module
    let _ = smooth_max(1.0, 2.0, 1e-6);

    // traits module (already tested above via Priceable)

    // types module
    let date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
    let convention = DayCountConvention::ActualActual365;
    let _ = convention.year_fraction(date, date);

    #[cfg(feature = "num-dual-mode")]
    {
        let _ = DualNumber::from(1.0);
    }
}
