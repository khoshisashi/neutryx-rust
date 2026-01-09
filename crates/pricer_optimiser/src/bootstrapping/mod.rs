//! Yield curve bootstrapping from OIS/Swap rates.
//!
//! This module implements multi-curve stripping logic to construct
//! yield curves from market-observed swap rates.

mod curve_builder;

pub use curve_builder::{BootstrapConfig, CurveBootstrapper};

/// Result of curve bootstrapping.
#[derive(Debug, Clone)]
pub struct BootstrapResult {
    /// Discount factors at each pillar
    pub discount_factors: Vec<f64>,
    /// Pillar dates (in years from today)
    pub pillars: Vec<f64>,
    /// Residual error
    pub residual: f64,
}
