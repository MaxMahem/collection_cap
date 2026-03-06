use collection_cap::VariableCap;

use crate::common::consts::*;
use crate::common::{check_eq, panics};

mod range_to_inclusive {
    use super::*;
    use std::ops::RangeToInclusive;

    const base::CAP_RANGE: RangeToInclusive<usize> = ..=base::CAP;

    check_eq!(capacity: base::CAP_RANGE.capacity() => val::MAX_CAP_VAL);
    check_eq!(from_max_cap_val: RangeToInclusive::<usize>::from(val::MAX_CAP_VAL) => base::CAP_RANGE);
}

mod range_from {
    use super::*;
    use std::ops::RangeFrom;

    const base::CAP_RANGE: RangeFrom<usize> = base::CAP..;

    check_eq!(capacity: base::CAP_RANGE.capacity() => val::MIN_CAP_VAL);
    check_eq!(from_min_cap_val: RangeFrom::<usize>::from(val::MIN_CAP_VAL) => base::CAP_RANGE);
}

mod range_inclusive {
    use super::*;
    use std::ops::RangeInclusive;

    const base::CAP_RANGE: RangeInclusive<usize> = base::CAP..=base::CAP;
    const INVALID_RANGE: RangeInclusive<usize> = RangeInclusive::new(base::CAP, base::CAP - 1);

    check_eq!(capacity: base::CAP_RANGE.capacity() => val::MIN_MAX_CAP_VAL);

    panics!(invalid: INVALID_RANGE.capacity() => "Invalid range (start > end)");
    check_eq!(from_min_max: RangeInclusive::<usize>::from(val::MIN_MAX_CAP_VAL) => base::CAP_RANGE);
    check_eq!(from_exclusive: RangeInclusive::<usize>::from(val::EXACT_CAP_VAL) => base::CAP_RANGE);
}

mod range_full {
    use collection_cap::cap::UnboundedCap;

    use super::*;
    use std::ops::RangeFull;

    const base::CAP_RANGE: RangeFull = ..;

    check_eq!(capacity: base::CAP_RANGE.capacity() => UnboundedCap);
    check_eq!(from_unbounded: RangeFull::from(UnboundedCap) => base::CAP_RANGE);
}
