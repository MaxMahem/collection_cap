use collection_cap::VariableCap;

use crate::common::consts::*;
use crate::common::{check_eq, panics};

mod range_to {
    use super::*;
    use std::ops::RangeTo;

    const CAP_RANGE: RangeTo<usize> = ..CAP + 1;
    const EMPTY_RANGE: RangeTo<usize> = ..0;

    check_eq!(capacity: CAP_RANGE.capacity() => MAX_CAP_VAL);

    panics!(empty: EMPTY_RANGE.capacity() => "Range must not be empty");
}

mod range_to_inclusive {
    use super::*;
    use std::ops::RangeToInclusive;

    const CAP_RANGE: RangeToInclusive<usize> = ..=CAP;

    check_eq!(capacity: CAP_RANGE.capacity() => MAX_CAP_VAL);
}

mod range_from {
    use super::*;
    use std::ops::RangeFrom;

    const CAP_RANGE: RangeFrom<usize> = CAP..;

    check_eq!(capacity: CAP_RANGE.capacity() => MIN_CAP_VAL);
}

mod range_open {
    use super::*;
    use std::ops::Range;

    const CAP_RANGE: Range<usize> = CAP..CAP + 1;
    const EMPTY_RANGE: Range<usize> = CAP..CAP;
    const INVALID_RANGE: Range<usize> = Range { start: CAP, end: CAP - 1 };

    check_eq!(capacity: CAP_RANGE.capacity() => MIN_MAX_CAP_VAL);

    panics!(empty: EMPTY_RANGE.capacity() => "Range must not be empty");

    panics!(invalid: INVALID_RANGE.capacity() => "Invalid range (start > end)");
}

mod range_inclusive {
    use super::*;
    use std::ops::RangeInclusive;

    const CAP_RANGE: RangeInclusive<usize> = CAP..=CAP;
    const INVALID_RANGE: RangeInclusive<usize> = RangeInclusive::new(CAP, CAP - 1);

    check_eq!(capacity: CAP_RANGE.capacity() => MIN_MAX_CAP_VAL);

    panics!(invalid: INVALID_RANGE.capacity() => "Invalid range (start > end)");
}
