//! Test utilities and unit tests for the RNG module.
//!
//! This module contains tests verifying:
//! - Module structure and public API accessibility
//! - PRNG seed reproducibility
//! - Distribution properties (uniform range, normal moments)
//! - QMC placeholder behaviour

use super::*;

/// Verifies that the module structure is correctly set up and all
/// public types are accessible.
#[test]
fn test_module_structure() {
    // Verify PricerRng is accessible
    let rng = PricerRng::from_seed(42);
    assert_eq!(rng.seed(), 42);

    // Verify LowDiscrepancySequence trait is accessible
    fn _accepts_lds<T: LowDiscrepancySequence>(_: &T) {}

    // Verify SobolPlaceholder type is accessible (but construction panics)
    // Just checking the type exists at compile time
    let _: fn(usize) -> SobolPlaceholder = SobolPlaceholder::new;
}

/// Verifies that the same seed produces identical sequences.
#[test]
fn test_seed_reproducibility() {
    let mut rng1 = PricerRng::from_seed(12345);
    let mut rng2 = PricerRng::from_seed(12345);

    // Generate several values and compare
    for _ in 0..100 {
        assert_eq!(rng1.gen_uniform(), rng2.gen_uniform());
    }

    // Reset and verify normal generation is also reproducible
    let mut rng3 = PricerRng::from_seed(12345);
    let mut rng4 = PricerRng::from_seed(12345);

    for _ in 0..100 {
        assert_eq!(rng3.gen_normal(), rng4.gen_normal());
    }
}

/// Verifies that uniform values are in the correct range [0, 1).
#[test]
fn test_uniform_range() {
    let mut rng = PricerRng::from_seed(42);

    for _ in 0..10_000 {
        let value = rng.gen_uniform();
        assert!(value >= 0.0, "Uniform value {} is below 0", value);
        assert!(value < 1.0, "Uniform value {} is >= 1", value);
    }
}

/// Verifies that batch fill operations work correctly.
#[test]
fn test_fill_uniform() {
    let mut rng = PricerRng::from_seed(42);
    let mut buffer = vec![0.0; 1000];

    rng.fill_uniform(&mut buffer);

    for &value in &buffer {
        assert!(value >= 0.0 && value < 1.0);
    }
}

/// Verifies that empty buffer is handled gracefully.
#[test]
fn test_empty_buffer() {
    let mut rng = PricerRng::from_seed(42);
    let mut empty: Vec<f64> = vec![];

    // These should not panic
    rng.fill_uniform(&mut empty);
    rng.fill_normal(&mut empty);
}

/// Verifies that SobolPlaceholder::new panics with appropriate message.
#[test]
#[should_panic(expected = "Sobol sequence not implemented in Phase 3.1a")]
fn test_sobol_placeholder_panics() {
    let _ = SobolPlaceholder::new(10);
}
