use crate::ValConstraint;
use crate::err::{FitError, Overflows, Underflows};

/// A runtime constraint specifying a maximum capacity.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct MaxCapVal(pub usize);

impl ValConstraint for MaxCapVal {
    type Error = Overflows;

    fn check_if_can_fit<I>(&self, iter: &I) -> Result<(), Self::Error>
    where
        I: Iterator + ?Sized,
    {
        Overflows::ensure_can_fit(iter, self.0)
    }
}

/// A runtime constraint specifying a minimum capacity.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct MinCapVal(pub usize);

impl ValConstraint for MinCapVal {
    type Error = Underflows;

    fn check_if_can_fit<I>(&self, iter: &I) -> Result<(), Self::Error>
    where
        I: Iterator + ?Sized,
    {
        Underflows::ensure_can_fit(iter, self.0)
    }
}

/// A runtime constraint specifying both a minimum and maximum capacity.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct MinMaxCapVal {
    /// The minimum capacity required.
    min: usize,
    /// The maximum capacity allowed.
    max: usize,
}

impl MinMaxCapVal {
    /// Creates a new [`MinMaxConstraint`].
    ///
    /// # Panics
    ///
    /// Panics if `min > max`.
    #[must_use]
    pub const fn new(min: usize, max: usize) -> Self {
        match (min, max) {
            (min, max) if min > max => panic!("min cap must be <= max cap"),
            (min, max) => Self { min, max },
        }
    }

    /// Returns the minimum capacity required.
    #[must_use]
    pub const fn min(&self) -> usize {
        self.min
    }

    /// Returns the maximum capacity allowed.
    #[must_use]
    pub const fn max(&self) -> usize {
        self.max
    }
}

impl ValConstraint for MinMaxCapVal {
    type Error = FitError;

    fn check_if_can_fit<I>(&self, iter: &I) -> Result<(), Self::Error>
    where
        I: Iterator + ?Sized,
    {
        FitError::ensure_can_fit(iter, self.min, self.max)
    }
}

/// A runtime constraint specifying an exact capacity.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct ExactCapVal(pub usize);

impl ValConstraint for ExactCapVal {
    type Error = FitError;

    fn check_if_can_fit<I>(&self, iter: &I) -> Result<(), Self::Error>
    where
        I: Iterator + ?Sized,
    {
        FitError::ensure_can_fit(iter, self.0, self.0)
    }
}
