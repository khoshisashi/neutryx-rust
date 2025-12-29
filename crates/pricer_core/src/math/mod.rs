//! Mathematical utilities and smooth approximations.
//!
//! This module provides differentiable smoothing functions that replace
//! discontinuous operations (max, min, abs, indicator) with smooth approximations
//! compatible with Enzyme automatic differentiation.
//!
//! ## Submodules
//! - `smoothing`: Smooth approximations using LogSumExp and sigmoid functions

pub mod smoothing;
