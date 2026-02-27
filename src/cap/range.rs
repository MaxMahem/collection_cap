use core::convert::Infallible;
use core::ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};

use crate::ValConstraint;
use crate::err::{FitError, Overflows, Underflows};

impl ValConstraint for RangeTo<usize> {
    type Error = Overflows;

    /// Checks if the count of elements that `iter` can produce fits within
    /// the range.
    ///
    /// # Panics
    ///
    /// Panics if `self.end = 0` - range is empty
    fn check_if_can_fit<I>(&self, iter: &I) -> Result<(), Self::Error>
    where
        I: Iterator + ?Sized,
    {
        match self.end {
            0 => panic!("capacity constraint range must not be empty"),
            end => Overflows::ensure_can_fit(iter, end.saturating_sub(1)),
        }
    }
}

impl ValConstraint for RangeToInclusive<usize> {
    type Error = Overflows;

    fn check_if_can_fit<I>(&self, iter: &I) -> Result<(), Self::Error>
    where
        I: Iterator + ?Sized,
    {
        Overflows::ensure_can_fit(iter, self.end)
    }
}

impl ValConstraint for RangeFrom<usize> {
    type Error = Underflows;

    fn check_if_can_fit<I>(&self, iter: &I) -> Result<(), Self::Error>
    where
        I: Iterator + ?Sized,
    {
        Underflows::ensure_can_fit(iter, self.start)
    }
}

impl ValConstraint for Range<usize> {
    type Error = FitError;

    /// Checks if the count of elements that `iter` can produce fits within
    /// the range.
    ///
    /// # Panics
    ///
    /// Panics if:
    /// - `self.start() == self.end()` - range is empty
    /// - `self.start() > self.end()` - range is invalid
    fn check_if_can_fit<I>(&self, iter: &I) -> Result<(), Self::Error>
    where
        I: Iterator + ?Sized,
    {
        match (self.start, self.end) {
            (start, end) if start == end => panic!("range must not be empty"),
            (start, end) if start > end => panic!("invalid range (start > end)"),
            (start, end) => FitError::ensure_can_fit(iter, start, end.saturating_sub(1)),
        }
    }
}

impl ValConstraint for RangeInclusive<usize> {
    type Error = FitError;

    /// Checks if the iterator can fit within the inclusive range.
    ///
    /// # Panics
    ///
    /// Panics if `self.start() > self.end()`.
    fn check_if_can_fit<I>(&self, iter: &I) -> Result<(), Self::Error>
    where
        I: Iterator + ?Sized,
    {
        match (self.start(), self.end()) {
            (start, end) if start > end => panic!("invalid range (start > end)"),
            (start, end) => FitError::ensure_can_fit(iter, *start, *end),
        }
    }
}

impl ValConstraint for RangeFull {
    type Error = Infallible;

    /// Always returns `Ok(())` because [`RangeFull`] declares an open-ended
    /// capacity constraint, with no min or max capacity.
    fn check_if_can_fit<I>(&self, _iter: &I) -> Result<(), Self::Error>
    where
        I: Iterator + ?Sized,
    {
        Ok(())
    }
}
