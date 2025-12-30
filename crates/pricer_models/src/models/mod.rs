//! Stochastic process models (GBM, Heston, etc.).
//!
//! This module provides stochastic models for Monte Carlo simulation:
//! - `StochasticModel` trait: Unified interface for all models
//! - `StochasticModelEnum`: Static dispatch enum for Enzyme compatibility
//!
//! ## Design Philosophy
//!
//! All models use:
//! - Static dispatch via enum (not `Box<dyn Trait>`)
//! - Generic `Float` type for AD compatibility
//! - Smooth approximations for differentiability

pub mod stochastic;

// Re-export core types for convenience
pub use stochastic::{SingleState, StochasticModel, StochasticState, TwoFactorState};
