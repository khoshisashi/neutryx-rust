//! Root-finding solvers for numerical computation.
//!
//! This module provides a collection of root-finding algorithms designed for
//! financial applications such as implied volatility calculation and curve
//! calibration, with support for automatic differentiation.
//!
//! ## Available Solvers
//!
//! - [`NewtonRaphsonSolver`]: Fast quadratic convergence using derivatives
//! - [`BrentSolver`]: Robust bracketing method without derivative requirement
//!
//! ## Configuration
//!
//! All solvers use [`SolverConfig`] for configuring:
//! - `tolerance`: Convergence tolerance (default: 1e-10)
//! - `max_iterations`: Maximum iteration count (default: 100)
//!
//! ## AD Compatibility
//!
//! The Newton-Raphson solver provides an AD-powered `find_root_ad` method
//! that automatically computes derivatives using `Dual64`, eliminating the
//! need to provide explicit derivative functions.
//!
//! ## Example
//!
//! ```
//! use pricer_core::math::solvers::{NewtonRaphsonSolver, SolverConfig};
//!
//! // Solve x² - 2 = 0 (find √2)
//! let config = SolverConfig::default();
//! let solver = NewtonRaphsonSolver::new(config);
//!
//! let f = |x: f64| x * x - 2.0;
//! let f_prime = |x: f64| 2.0 * x;
//!
//! let root = solver.find_root(f, f_prime, 1.0).unwrap();
//! assert!((root - std::f64::consts::SQRT_2).abs() < 1e-10);
//! ```

mod brent;
mod config;
mod newton_raphson;

// Re-export public types at module level
pub use brent::BrentSolver;
pub use config::SolverConfig;
pub use newton_raphson::NewtonRaphsonSolver;
