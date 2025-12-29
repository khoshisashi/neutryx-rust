//! Pricing error types for structured error handling.

use std::fmt;

/// Categorised pricing errors.
///
/// Provides structured error handling for pricing operations with
/// descriptive context for each failure mode.
///
/// # Variants
/// - `InvalidInput`: Invalid market data or parameters
/// - `NumericalInstability`: Computation failed to converge
/// - `ModelFailure`: Model assumptions violated
/// - `UnsupportedInstrument`: Instrument type not supported by model
///
/// # Examples
/// ```
/// use pricer_core::types::PricingError;
///
/// let err = PricingError::InvalidInput("Negative spot price".to_string());
/// assert_eq!(format!("{}", err), "Invalid input: Negative spot price");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PricingError {
    /// Invalid input data or parameters
    InvalidInput(String),

    /// Numerical instability during computation
    NumericalInstability(String),

    /// Model failed to produce valid result
    ModelFailure(String),

    /// Instrument type not supported
    UnsupportedInstrument(String),
}

impl fmt::Display for PricingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PricingError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            PricingError::NumericalInstability(msg) => {
                write!(f, "Numerical instability: {}", msg)
            }
            PricingError::ModelFailure(msg) => write!(f, "Model failure: {}", msg),
            PricingError::UnsupportedInstrument(msg) => {
                write!(f, "Unsupported instrument: {}", msg)
            }
        }
    }
}

impl std::error::Error for PricingError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_input_display() {
        let err = PricingError::InvalidInput("Test error".to_string());
        assert_eq!(format!("{}", err), "Invalid input: Test error");
    }

    #[test]
    fn test_numerical_instability_display() {
        let err = PricingError::NumericalInstability("Failed to converge".to_string());
        assert_eq!(format!("{}", err), "Numerical instability: Failed to converge");
    }

    #[test]
    fn test_model_failure_display() {
        let err = PricingError::ModelFailure("Volatility out of range".to_string());
        assert_eq!(format!("{}", err), "Model failure: Volatility out of range");
    }

    #[test]
    fn test_unsupported_instrument_display() {
        let err = PricingError::UnsupportedInstrument("Asian option".to_string());
        assert_eq!(format!("{}", err), "Unsupported instrument: Asian option");
    }

    #[test]
    fn test_error_trait_implementation() {
        let err = PricingError::InvalidInput("Test".to_string());
        let _: &dyn std::error::Error = &err; // Verify Error trait is implemented
    }

    #[test]
    fn test_clone_and_equality() {
        let err1 = PricingError::InvalidInput("Test".to_string());
        let err2 = err1.clone();
        assert_eq!(err1, err2);
    }
}
