//! Core numeric and time types.
//!
//! This module provides:
//! - `dual`: Dual number type integration with num-dual for automatic differentiation (when `num-dual-mode` feature is enabled)
//! - `time`: Time types and Day Count Conventions for financial calculations

#[cfg(feature = "num-dual-mode")]
pub mod dual;
pub mod time;
