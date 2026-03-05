use core::ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};

use crate::cap::{MaxCapVal, MinCapVal, MinMaxCapVal, UnboundedCap};
use crate::{StaticCap, VariableCap};

const EMPTY_RANGE_MSG: &str = "Range must not be empty";
const INVALID_RANGE_MSG: &str = "Invalid range (start > end)";

impl VariableCap for RangeTo<usize> {
    type Cap = MaxCapVal;

    /// Returns the capacity for this range.
    ///
    /// # Panics
    ///
    /// Panics if `self.end == 0` (range is empty).
    fn capacity(&self) -> MaxCapVal {
        usize::checked_sub(self.end, 1).map_or_else(|| panic!("{EMPTY_RANGE_MSG}"), MaxCapVal)
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
            (start, end) => MinMaxCapVal::new(start, end - 1),
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
    type Cap = UnboundedCap;

    /// Returns [`UnboundedCap`].
    fn capacity(&self) -> UnboundedCap {
        UnboundedCap
    }
}

impl StaticCap for RangeFull {
    type Cap = UnboundedCap;
    const CAP: UnboundedCap = UnboundedCap;
}
