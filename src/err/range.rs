use thiserror::Error;

/// Error returned when a capacity range is empty.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
#[error("Range must not be empty")]
pub struct EmptyRange;

/// Error returned when a capacity range is strictly invalid (e.g., start > end).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
#[error("Invalid range (start > end)")]
pub struct InvalidRange;

/// Error returned when attempting to convert a standard range into a capacity constraint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum RangeError {
    /// The input range was empty.
    #[error(transparent)]
    Empty(#[from] EmptyRange),
    /// The input range was strictly invalid (e.g. `start > end`).
    #[error(transparent)]
    InvalidRange(#[from] InvalidRange),
}
