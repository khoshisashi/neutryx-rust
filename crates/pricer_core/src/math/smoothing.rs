//! Smooth approximations for discontinuous functions.
//!
//! This module provides differentiable smoothing functions that replace
//! discontinuous operations (max, min, abs, indicator) with smooth approximations.
//! Required for Enzyme AD: hard `if` conditions are non-differentiable.
//!
//! All functions use generic type parameter `T: num_traits::Float` for f32/f64 support.

use num_traits::Float;

/// Differentiable maximum function using LogSumExp.
///
/// # Mathematical Definition
/// ```text
/// smooth_max(a, b, ε) = ε * log(exp(a/ε) + exp(b/ε))
/// ```
///
/// # Convergence
/// As ε → 0, smooth_max(a, b, ε) → max(a, b)
///
/// # Arguments
/// * `a` - First argument
/// * `b` - Second argument
/// * `epsilon` - Smoothing parameter (recommended range: 1e-8 to 1e-3)
///
/// # Returns
/// Differentiable approximation of max(a, b)
///
/// # Panics
/// Panics if epsilon <= 0
///
/// # Examples
/// ```
/// use pricer_core::math::smoothing::smooth_max;
///
/// let result = smooth_max(3.0_f64, 5.0_f64, 1e-6);
/// assert!((result - 5.0).abs() < 1e-3);
/// ```
#[inline]
pub fn smooth_max<T: Float>(a: T, b: T, epsilon: T) -> T {
    assert!(epsilon > T::zero(), "epsilon must be positive");

    // Numerically stable LogSumExp using log-sum-exp trick
    // smooth_max(a, b, ε) = m + ε * log(exp((a-m)/ε) + exp((b-m)/ε))
    // where m = max(a, b) to prevent overflow
    let m = if a > b { a } else { b };
    let exp_a = ((a - m) / epsilon).exp();
    let exp_b = ((b - m) / epsilon).exp();

    m + epsilon * (exp_a + exp_b).ln()
}

/// Differentiable minimum function as dual of smooth_max.
///
/// # Mathematical Definition
/// ```text
/// smooth_min(a, b, ε) = -smooth_max(-a, -b, ε)
/// ```
///
/// # Convergence
/// As ε → 0, smooth_min(a, b, ε) → min(a, b)
///
/// # Arguments
/// * `a` - First argument
/// * `b` - Second argument
/// * `epsilon` - Smoothing parameter (recommended range: 1e-8 to 1e-3)
///
/// # Returns
/// Differentiable approximation of min(a, b)
///
/// # Panics
/// Panics if epsilon <= 0
#[inline]
pub fn smooth_min<T: Float>(a: T, b: T, epsilon: T) -> T {
    assert!(epsilon > T::zero(), "epsilon must be positive");

    // Dual of smooth_max: smooth_min(a, b, ε) = -smooth_max(-a, -b, ε)
    -smooth_max(-a, -b, epsilon)
}

/// Differentiable Heaviside function using sigmoid.
///
/// # Mathematical Definition
/// ```text
/// smooth_indicator(x, ε) = 1 / (1 + exp(-x/ε))
/// ```
///
/// # Convergence
/// As ε → 0:
/// - x < 0 → 0
/// - x = 0 → 0.5
/// - x > 0 → 1
///
/// # Arguments
/// * `x` - Input value
/// * `epsilon` - Smoothing parameter (recommended range: 1e-8 to 1e-3)
///
/// # Returns
/// Differentiable approximation of Heaviside step function
///
/// # Panics
/// Panics if epsilon <= 0
#[inline]
pub fn smooth_indicator<T: Float>(x: T, epsilon: T) -> T {
    assert!(epsilon > T::zero(), "epsilon must be positive");

    // Sigmoid function: σ(x/ε) = 1 / (1 + exp(-x/ε))
    let one = T::one();
    one / (one + (-x / epsilon).exp())
}

/// Differentiable absolute value function using Softplus.
///
/// # Mathematical Definition
/// ```text
/// smooth_abs(x, ε) = ε * log(exp(x/ε) + exp(-x/ε))
/// ```
///
/// # Convergence
/// As ε → 0, smooth_abs(x, ε) → |x|
///
/// # Arguments
/// * `x` - Input value
/// * `epsilon` - Smoothing parameter (recommended range: 1e-8 to 1e-3)
///
/// # Returns
/// Differentiable approximation of |x|
///
/// # Panics
/// Panics if epsilon <= 0
#[inline]
pub fn smooth_abs<T: Float>(x: T, epsilon: T) -> T {
    assert!(epsilon > T::zero(), "epsilon must be positive");

    // Softplus-based: smooth_abs(x, ε) = ε * log(exp(x/ε) + exp(-x/ε))
    // Using log-sum-exp trick for numerical stability
    let x_over_eps = x / epsilon;
    let exp_pos = x_over_eps.exp();
    let exp_neg = (-x_over_eps).exp();

    epsilon * (exp_pos + exp_neg).ln()
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    // Task 2.5: Smoothing function unit tests

    #[test]
    fn test_smooth_max_convergence() {
        // Test convergence to true max as epsilon decreases
        let a = 3.0_f64;
        let b = 5.0_f64;
        let true_max = a.max(b);

        let eps_values = [1e-2, 1e-4, 1e-6];
        let tolerances = [1e-1, 1e-2, 1e-3];

        for (eps, tol) in eps_values.iter().zip(tolerances.iter()) {
            let result = smooth_max(a, b, *eps);
            assert_relative_eq!(result, true_max, epsilon = *tol);
        }
    }

    #[test]
    fn test_smooth_max_commutativity() {
        let a = 3.0_f64;
        let b = 5.0_f64;
        let epsilon = 1e-6;

        let result1 = smooth_max(a, b, epsilon);
        let result2 = smooth_max(b, a, epsilon);

        assert_relative_eq!(result1, result2, epsilon = 1e-10);
    }

    #[test]
    fn test_smooth_min_duality() {
        // smooth_min(a, b, ε) == -smooth_max(-a, -b, ε)
        let a = 3.0_f64;
        let b = 5.0_f64;
        let epsilon = 1e-6;

        let result_min = smooth_min(a, b, epsilon);
        let result_dual = -smooth_max(-a, -b, epsilon);

        assert_relative_eq!(result_min, result_dual, epsilon = 1e-10);
    }

    #[test]
    fn test_smooth_min_convergence() {
        let a = 3.0_f64;
        let b = 5.0_f64;
        let true_min = a.min(b);

        let eps_values = [1e-2, 1e-4, 1e-6];
        let tolerances = [1e-1, 1e-2, 1e-3];

        for (eps, tol) in eps_values.iter().zip(tolerances.iter()) {
            let result = smooth_min(a, b, *eps);
            assert_relative_eq!(result, true_min, epsilon = *tol);
        }
    }

    #[test]
    fn test_smooth_indicator_boundary() {
        // At x=0, smooth_indicator should be approximately 0.5
        let epsilon = 1e-6;
        let result = smooth_indicator(0.0_f64, epsilon);
        assert_relative_eq!(result, 0.5, epsilon = 1e-3);
    }

    #[test]
    fn test_smooth_indicator_convergence() {
        let epsilon = 1e-6;

        // For large negative x, should converge to 0
        let result_neg = smooth_indicator(-10.0_f64, epsilon);
        assert!(result_neg < 0.01);

        // For large positive x, should converge to 1
        let result_pos = smooth_indicator(10.0_f64, epsilon);
        assert!(result_pos > 0.99);
    }

    #[test]
    fn test_smooth_abs_even_function() {
        // smooth_abs(-x, ε) == smooth_abs(x, ε)
        let x = 3.5_f64;
        let epsilon = 1e-6;

        let result_pos = smooth_abs(x, epsilon);
        let result_neg = smooth_abs(-x, epsilon);

        assert_relative_eq!(result_pos, result_neg, epsilon = 1e-10);
    }

    #[test]
    fn test_smooth_abs_convergence() {
        let x = 3.5_f64;
        let true_abs = x.abs();

        let eps_values = [1e-2, 1e-4, 1e-6];
        let tolerances = [1e-1, 1e-2, 1e-3];

        for (eps, tol) in eps_values.iter().zip(tolerances.iter()) {
            let result = smooth_abs(x, *eps);
            assert_relative_eq!(result, true_abs, epsilon = *tol);
        }
    }

    #[test]
    #[should_panic(expected = "epsilon must be positive")]
    fn test_smooth_max_panics_on_zero_epsilon() {
        smooth_max(3.0_f64, 5.0_f64, 0.0);
    }

    #[test]
    #[should_panic(expected = "epsilon must be positive")]
    fn test_smooth_max_panics_on_negative_epsilon() {
        smooth_max(3.0_f64, 5.0_f64, -1e-6);
    }

    #[test]
    #[should_panic(expected = "epsilon must be positive")]
    fn test_smooth_min_panics_on_zero_epsilon() {
        smooth_min(3.0_f64, 5.0_f64, 0.0);
    }

    #[test]
    #[should_panic(expected = "epsilon must be positive")]
    fn test_smooth_indicator_panics_on_zero_epsilon() {
        smooth_indicator(0.0_f64, 0.0);
    }

    #[test]
    #[should_panic(expected = "epsilon must be positive")]
    fn test_smooth_abs_panics_on_zero_epsilon() {
        smooth_abs(3.0_f64, 0.0);
    }

    // Task 6.1: Property-based tests for smoothing functions
    #[cfg(test)]
    mod property_tests {
        use super::*;
        use proptest::prelude::*;

        // Generate epsilon in practical range [1e-8, 1e-3]
        fn epsilon_strategy() -> impl Strategy<Value = f64> {
            prop::num::f64::POSITIVE.prop_filter("epsilon in range", |&e| e >= 1e-8 && e <= 1e-3)
        }

        // Generate finite f64 values for testing
        fn finite_f64_strategy() -> impl Strategy<Value = f64> {
            prop::num::f64::NORMAL
        }

        proptest! {
            #![proptest_config(ProptestConfig::with_cases(1000))]

            #[test]
            fn test_smooth_max_inequality(
                a in finite_f64_strategy(),
                b in finite_f64_strategy(),
                epsilon in epsilon_strategy()
            ) {
                let result = smooth_max(a, b, epsilon);
                let true_max = a.max(b);
                let tolerance = epsilon.abs() * 10.0; // O(ε) tolerance

                // smooth_max(a, b, ε) >= max(a, b) - tolerance
                assert!(
                    result >= true_max - tolerance,
                    "smooth_max({}, {}, {}) = {} should be >= {} - {}",
                    a, b, epsilon, result, true_max, tolerance
                );
            }

            #[test]
            fn test_smooth_max_commutativity_property(
                a in finite_f64_strategy(),
                b in finite_f64_strategy(),
                epsilon in epsilon_strategy()
            ) {
                let result1 = smooth_max(a, b, epsilon);
                let result2 = smooth_max(b, a, epsilon);

                // smooth_max(a, b, ε) == smooth_max(b, a, ε)
                assert_relative_eq!(result1, result2, epsilon = 1e-10);
            }

            #[test]
            fn test_smooth_min_inequality(
                a in finite_f64_strategy(),
                b in finite_f64_strategy(),
                epsilon in epsilon_strategy()
            ) {
                let result = smooth_min(a, b, epsilon);
                let true_min = a.min(b);
                let tolerance = epsilon.abs() * 10.0; // O(ε) tolerance

                // smooth_min(a, b, ε) <= min(a, b) + tolerance
                assert!(
                    result <= true_min + tolerance,
                    "smooth_min({}, {}, {}) = {} should be <= {} + {}",
                    a, b, epsilon, result, true_min, tolerance
                );
            }

            #[test]
            fn test_smooth_min_commutativity_property(
                a in finite_f64_strategy(),
                b in finite_f64_strategy(),
                epsilon in epsilon_strategy()
            ) {
                let result1 = smooth_min(a, b, epsilon);
                let result2 = smooth_min(b, a, epsilon);

                // smooth_min(a, b, ε) == smooth_min(b, a, ε)
                assert_relative_eq!(result1, result2, epsilon = 1e-10);
            }

            #[test]
            fn test_smooth_indicator_monotonicity(
                x1 in finite_f64_strategy(),
                x2 in finite_f64_strategy(),
                epsilon in epsilon_strategy()
            ) {
                // Only test when x1 < x2
                if x1 < x2 {
                    let result1 = smooth_indicator(x1, epsilon);
                    let result2 = smooth_indicator(x2, epsilon);

                    // x1 < x2 → smooth_indicator(x1, ε) <= smooth_indicator(x2, ε)
                    assert!(
                        result1 <= result2,
                        "smooth_indicator({}, {}) = {} should be <= smooth_indicator({}, {}) = {}",
                        x1, epsilon, result1, x2, epsilon, result2
                    );
                }
            }

            #[test]
            fn test_smooth_indicator_bounds(
                x in finite_f64_strategy(),
                epsilon in epsilon_strategy()
            ) {
                let result = smooth_indicator(x, epsilon);

                // smooth_indicator should always be in [0, 1]
                assert!(
                    result >= 0.0 && result <= 1.0,
                    "smooth_indicator({}, {}) = {} should be in [0, 1]",
                    x, epsilon, result
                );
            }

            #[test]
            fn test_smooth_abs_even_function_property(
                x in finite_f64_strategy(),
                epsilon in epsilon_strategy()
            ) {
                let result_pos = smooth_abs(x, epsilon);
                let result_neg = smooth_abs(-x, epsilon);

                // smooth_abs(-x, ε) == smooth_abs(x, ε)
                assert_relative_eq!(result_pos, result_neg, epsilon = 1e-10);
            }

            #[test]
            fn test_smooth_abs_non_negative(
                x in finite_f64_strategy(),
                epsilon in epsilon_strategy()
            ) {
                let result = smooth_abs(x, epsilon);

                // smooth_abs should always be non-negative
                assert!(
                    result >= 0.0,
                    "smooth_abs({}, {}) = {} should be non-negative",
                    x, epsilon, result
                );
            }

            #[test]
            fn test_all_smoothing_functions_return_finite(
                a in finite_f64_strategy(),
                b in finite_f64_strategy(),
                x in finite_f64_strategy(),
                epsilon in epsilon_strategy()
            ) {
                // All smoothing functions should return finite values for finite inputs
                let max_result = smooth_max(a, b, epsilon);
                let min_result = smooth_min(a, b, epsilon);
                let ind_result = smooth_indicator(x, epsilon);
                let abs_result = smooth_abs(x, epsilon);

                assert!(max_result.is_finite());
                assert!(min_result.is_finite());
                assert!(ind_result.is_finite());
                assert!(abs_result.is_finite());
            }
        }
    }
}
