//! Numerical optimisation solvers.
//!
//! This module implements optimisation algorithms including:
//! - Levenberg-Marquardt for nonlinear least squares
//! - BFGS for general unconstrained optimisation

mod levenberg_marquardt;
mod bfgs;

pub use levenberg_marquardt::{LevenbergMarquardt, LevenbergMarquardtConfig};
pub use bfgs::{Bfgs, BfgsConfig};

/// Optimisation result.
#[derive(Debug, Clone)]
pub struct OptimisationResult {
    /// Optimal parameters
    pub parameters: Vec<f64>,
    /// Final objective value
    pub objective: f64,
    /// Number of iterations
    pub iterations: usize,
    /// Number of function evaluations
    pub function_evaluations: usize,
    /// Convergence status
    pub converged: bool,
}
