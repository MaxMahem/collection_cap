pub mod range_to_inclusive {
    use crate::VariableCap;
    use crate::cap::MaxCapVal;
    use core::ops::RangeToInclusive;

    impl VariableCap for RangeToInclusive<usize> {
        type Cap = MaxCapVal;

        fn capacity(&self) -> MaxCapVal {
            MaxCapVal(self.end)
        }
    }

    impl From<MaxCapVal> for RangeToInclusive<usize> {
        fn from(value: MaxCapVal) -> Self {
            ..=value.0
        }
    }
}

pub mod range_from {
    use crate::VariableCap;
    use crate::cap::MinCapVal;
    use core::ops::RangeFrom;

    impl VariableCap for RangeFrom<usize> {
        type Cap = MinCapVal;

        fn capacity(&self) -> MinCapVal {
            MinCapVal(self.start)
        }
    }

    impl From<MinCapVal> for RangeFrom<usize> {
        fn from(value: MinCapVal) -> Self {
            value.0..
        }
    }
}

pub mod range_inclusive {
    use crate::VariableCap;
    use crate::cap::{ExactCapVal, MinMaxCapVal};
    use core::ops::RangeInclusive;

    const INVALID_RANGE_MSG: &str = "Invalid range (start > end)";

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

    impl From<MinMaxCapVal> for RangeInclusive<usize> {
        fn from(value: MinMaxCapVal) -> Self {
            value.min().0..=value.max().0
        }
    }

    impl From<ExactCapVal> for RangeInclusive<usize> {
        fn from(value: ExactCapVal) -> Self {
            value.0..=value.0
        }
    }
}

pub mod range_full {
    use crate::cap::UnboundedCap;
    use crate::{ConstCap, VariableCap};
    use core::ops::RangeFull;

    impl VariableCap for RangeFull {
        type Cap = UnboundedCap;

        /// Returns [`UnboundedCap`].
        fn capacity(&self) -> UnboundedCap {
            UnboundedCap
        }
    }

    impl ConstCap for RangeFull {
        type Cap = UnboundedCap;
        const CAP: UnboundedCap = UnboundedCap;
    }

    impl From<UnboundedCap> for RangeFull {
        fn from(_: UnboundedCap) -> Self {
            ..
        }
    }
}
