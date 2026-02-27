use size_hinter::SizeHint;

use crate::ValConstraint;
use crate::err::{FitError, Overflows, Underflows};

const INVALID_SIZE_HINT_MSG: &str = "Invalid size hint";

/// A runtime constraint specifying a maximum capacity.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct MaxCapVal(pub usize);

impl ValConstraint for MaxCapVal {
    type Error = Overflows;

    fn check_if_can_fit<I>(&self, iter: &I) -> Result<(), Self::Error>
    where
        I: Iterator + ?Sized,
    {
        let hint: SizeHint = iter.size_hint().try_into().expect(INVALID_SIZE_HINT_MSG);
        let min_size = hint.lower();
        if min_size > self.0 { Err(Overflows::new(min_size, self.0)) } else { Ok(()) }
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
        let hint: SizeHint = iter.size_hint().try_into().expect(INVALID_SIZE_HINT_MSG);
        hint.upper()
            .filter(|&max_size| max_size < self.0)
            .map(|max_size| Underflows::new(max_size, self.0))
            .map_or(Ok(()), Err)
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
    /// Creates a new [`MinMaxCapVal`].
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
        MinCapVal(self.min).check_if_can_fit(iter).map_err(FitError::Underflows)?;
        MaxCapVal(self.max).check_if_can_fit(iter).map_err(FitError::Overflows)
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
        MinMaxCapVal::new(self.0, self.0).check_if_can_fit(iter)
    }
}
