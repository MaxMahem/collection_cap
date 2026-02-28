use core::convert::Infallible;
use core::ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};

use crate::cap::val::{MaxCapVal, MinCapVal, MinMaxCapVal};
use crate::err::{Overflows, Underflows, VarCapError};
use crate::{EMPTY_RANGE_MSG, INVALID_RANGE_MSG, VariableCap};

impl VariableCap for RangeTo<usize> {
    type Error = Overflows;

    /// Checks if the given iterator is compatible with the range.
    ///
    /// # Panics
    ///
    /// Panics if:
    /// - `self.end = 0` - range is empty
    /// - `iter`'s [size hint](Iterator::size_hint) is invalid.
    fn check_compatability<I>(&self, iter: &I) -> Result<(), Self::Error>
    where
        I: Iterator + ?Sized,
    {
        #[expect(clippy::option_if_let_else)]
        match usize::checked_sub(self.end, 1) {
            None => panic!("{EMPTY_RANGE_MSG}"),
            Some(end) => MaxCapVal(end).check_compatability(iter),
        }
    }
}

impl VariableCap for RangeToInclusive<usize> {
    type Error = Overflows;

    fn check_compatability<I>(&self, iter: &I) -> Result<(), Self::Error>
    where
        I: Iterator + ?Sized,
    {
        MaxCapVal(self.end).check_compatability(iter)
    }
}

impl VariableCap for RangeFrom<usize> {
    type Error = Underflows;

    fn check_compatability<I>(&self, iter: &I) -> Result<(), Self::Error>
    where
        I: Iterator + ?Sized,
    {
        MinCapVal(self.start).check_compatability(iter)
    }
}

impl VariableCap for Range<usize> {
    type Error = VarCapError;

    /// Checks if the given iterator is compatible with the range.
    ///
    /// # Panics
    ///
    /// Panics if:
    /// - `self.start() == self.end()` - range is empty
    /// - `self.start() > self.end()` - range is invalid
    fn check_compatability<I>(&self, iter: &I) -> Result<(), Self::Error>
    where
        I: Iterator + ?Sized,
    {
        match (self.start, self.end) {
            (start, end) if start == end => panic!("{EMPTY_RANGE_MSG}"),
            (start, end) if start > end => panic!("{INVALID_RANGE_MSG}"),
            (start, end) => MinMaxCapVal::new(start, end.saturating_sub(1)).check_compatability(iter),
        }
    }
}

impl VariableCap for RangeInclusive<usize> {
    type Error = VarCapError;

    /// Checks if the iterator is compatible with the inclusive range.
    ///
    /// # Panics
    ///
    /// Panics if `self.start() > self.end()`.
    fn check_compatability<I>(&self, iter: &I) -> Result<(), Self::Error>
    where
        I: Iterator + ?Sized,
    {
        match (self.start(), self.end()) {
            (start, end) if start > end => panic!("{INVALID_RANGE_MSG}"),
            (start, end) => MinMaxCapVal::new(*start, *end).check_compatability(iter),
        }
    }
}

impl VariableCap for RangeFull {
    type Error = Infallible;

    /// Always returns `Ok(())` because [`RangeFull`] declares an open-ended
    /// capacity constraint, with no min or max capacity.
    fn check_compatability<I>(&self, _iter: &I) -> Result<(), Self::Error>
    where
        I: Iterator + ?Sized,
    {
        Ok(())
    }
}
