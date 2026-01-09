//! Master data errors.

use thiserror::Error;

/// Errors that can occur when accessing master data.
#[derive(Error, Debug)]
pub enum MasterDataError {
    /// Calendar not found
    #[error("Calendar not found: {0}")]
    CalendarNotFound(String),

    /// Invalid date
    #[error("Invalid date: {0}")]
    InvalidDate(String),

    /// Invalid ISIN
    #[error("Invalid ISIN: {0}")]
    InvalidIsin(String),
}
