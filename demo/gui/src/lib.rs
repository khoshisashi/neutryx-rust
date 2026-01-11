//! # Demo GUI
//!
//! Terminal-based dashboard for the FrictionalBank demo.
//! Uses ratatui for rendering and crossterm for terminal handling.
//!
//! ## Screens
//!
//! - **Dashboard**: Portfolio summary and risk metrics overview
//! - **Portfolio**: Trade list with PV and Greeks
//! - **Risk**: CVA, DVA, FVA, Exposure display
//! - **TradeBlotter**: Selected trade details

pub mod api_client;
pub mod app;
pub mod screens;

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::api_client::ApiClient;
    pub use crate::app::{Screen, TuiApp};
}
