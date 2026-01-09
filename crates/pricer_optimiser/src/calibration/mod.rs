//! Model calibration module.
//!
//! This module implements calibration of stochastic models by minimising
//! the error between theoretical prices and market prices.

mod engine;

pub use engine::{CalibrationConfig, CalibrationEngine, CalibrationResult};

/// Market data for calibration.
#[derive(Debug, Clone)]
pub struct CalibrationMarketData {
    /// Market prices to match
    pub market_prices: Vec<f64>,
    /// Weights for each price (optional, defaults to equal weighting)
    pub weights: Option<Vec<f64>>,
}

impl CalibrationMarketData {
    /// Create new calibration market data.
    pub fn new(market_prices: Vec<f64>) -> Self {
        Self {
            market_prices,
            weights: None,
        }
    }

    /// Set custom weights.
    pub fn with_weights(mut self, weights: Vec<f64>) -> Self {
        self.weights = Some(weights);
        self
    }
}
