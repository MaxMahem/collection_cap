use core::ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};

use crate::cap::{MaxCapVal, MinCapVal, MinMaxCapVal, UnboundedCapVal};
use crate::{EMPTY_RANGE_MSG, INVALID_RANGE_MSG, VariableCap};

impl VariableCap for RangeTo<usize> {
    type Cap = MaxCapVal;

    /// Returns the capacity for this range.
    ///
    /// # Panics
    ///
    /// Panics if `self.end == 0` (range is empty).
    fn capacity(&self) -> MaxCapVal {
        #[expect(clippy::option_if_let_else)]
        match usize::checked_sub(self.end, 1) {
            None => panic!("{EMPTY_RANGE_MSG}"),
            Some(end) => MaxCapVal(end),
        }
    }
}

impl VariableCap for RangeToInclusive<usize> {
    type Cap = MaxCapVal;

    fn capacity(&self) -> MaxCapVal {
        MaxCapVal(self.end)
    }
}

impl VariableCap for RangeFrom<usize> {
    type Cap = MinCapVal;

    fn capacity(&self) -> MinCapVal {
        MinCapVal(self.start)
    }
}

impl VariableCap for Range<usize> {
    type Cap = MinMaxCapVal;

    /// Returns the capacity for this range.
    ///
    /// # Panics
    ///
    /// Panics if:
    /// - `self.start == self.end` — range is empty
    /// - `self.start > self.end` — range is invalid
    fn capacity(&self) -> MinMaxCapVal {
        match (self.start, self.end) {
            (start, end) if start == end => panic!("{EMPTY_RANGE_MSG}"),
            (start, end) if start > end => panic!("{INVALID_RANGE_MSG}"),
            (start, end) => MinMaxCapVal::new(start, end.saturating_sub(1)),
        }
    }
}

impl VariableCap for RangeInclusive<usize> {
    type Cap = MinMaxCapVal;

    /// Returns the capacity for this range.
    ///
    /// # Panics
    ///
    /// Panics if `self.start() > self.end()`.
    fn capacity(&self) -> MinMaxCapVal {
        match (self.start(), self.end()) {
            (start, end) if start > end => panic!("{INVALID_RANGE_MSG}"),
            (start, end) => MinMaxCapVal::new(*start, *end),
        }
    }
}

impl VariableCap for RangeFull {
    type Cap = UnboundedCapVal;

    /// Returns the capacity for this range.
    ///
    /// This is always [`UnboundedCapVal`].
    fn capacity(&self) -> UnboundedCapVal {
        UnboundedCapVal
    }
}
