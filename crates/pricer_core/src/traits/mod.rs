//! Core traits for priceable instruments and differentiation.
//!
//! This module defines fundamental abstractions for:
//! - Price calculation (`Priceable` trait)
//! - Gradient computation (`Differentiable` trait)
//!
//! All traits are designed for static dispatch (enum-based) to ensure
//! compatibility with Enzyme AD optimization at LLVM level.
//!
//! ## Important
//! Do NOT use `Box<dyn Trait>` dynamic dispatch with these traits,
//! as it is incompatible with Enzyme's LLVM-level optimization.

pub mod priceable;
