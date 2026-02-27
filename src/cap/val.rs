use size_hinter::SizeHint;

use crate::VariableCap;
use crate::err::{Overflows, Underflows, VarCapError};

const INVALID_SIZE_HINT_MSG: &str = "Invalid size hint";

/// A runtime constraint specifying a maximum capacity.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct MaxCapVal(pub usize);

impl VariableCap for MaxCapVal {
    type Error = Overflows;

    fn check_compatability<I>(&self, iter: &I) -> Result<(), Self::Error>
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

impl VariableCap for MinCapVal {
    type Error = Underflows;

    fn check_compatability<I>(&self, iter: &I) -> Result<(), Self::Error>
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

impl VariableCap for MinMaxCapVal {
    type Error = VarCapError;

    fn check_compatability<I>(&self, iter: &I) -> Result<(), Self::Error>
    where
        I: Iterator + ?Sized,
    {
        MinCapVal(self.min).check_compatability(iter).map_err(VarCapError::Underflows)?;
        MaxCapVal(self.max).check_compatability(iter).map_err(VarCapError::Overflows)
    }
}

/// A runtime constraint specifying an exact capacity.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct ExactCapVal(pub usize);

impl VariableCap for ExactCapVal {
    type Error = VarCapError;

    fn check_compatability<I>(&self, iter: &I) -> Result<(), Self::Error>
    where
        I: Iterator + ?Sized,
    {
        MinMaxCapVal::new(self.0, self.0).check_compatability(iter)
    }
}
