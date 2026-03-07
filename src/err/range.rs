use thiserror::Error;

/// Error returned when a capacity range is empty.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
#[error("Range must not be empty")]
pub struct EmptyRange;

/// Error returned when a capacity range is strictly invalid (e.g., start > end).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
#[error("Invalid range: start ({start}) > end ({end})")]
pub struct InvalidRange {
    /// The inclusive start bound.
    start: usize,
    /// The inclusive end bound.
    end: usize,
}

impl InvalidRange {
    /// Internal unchecked constructor.
    #[must_use]
    pub(crate) const fn new_unchecked(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    /// Creates a new [`InvalidRange`] error.
    ///
    /// # Arguments
    ///
    /// - `start`: The inclusive start bound.
    /// - `end`: The inclusive end bound.
    ///
    /// # Panics
    ///
    /// Panics if `start <= end`.
    #[must_use]
    pub const fn new(start: usize, end: usize) -> Self {
        match (start, end) {
            (start, end) if start <= end => panic!("Range is valid"),
            (start, end) => Self::new_unchecked(start, end),
        }
    }

    /// Returns the inclusive start bound.
    #[must_use]
    pub const fn start(&self) -> usize {
        self.start
    }
    /// Returns the inclusive end bound.
    #[must_use]
    pub const fn end(&self) -> usize {
        self.end
    }
}

/// Error returned when attempting to convert a standard range into a capacity constraint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum FromRangeError {
    /// The input range was empty.
    #[error(transparent)]
    Empty(#[from] EmptyRange),
    /// The input range was strictly invalid (e.g. `start > end`).
    #[error(transparent)]
    InvalidRange(#[from] InvalidRange),
}

impl FromRangeError {
    /// An [`EmptyRange`] error.
    pub const EMPTY: Self = Self::Empty(EmptyRange);
}
