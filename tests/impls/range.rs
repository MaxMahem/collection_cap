use collection_cap::VariableCap;

use collection_cap::cap::{ExactCapVal, MaxCapVal, MinCapVal, MinMaxCapVal};

use crate::common::consts::*;
use crate::common::{check_eq, panics};

const EXACT_CAP_VAL: ExactCapVal = ExactCapVal(CAP);
const MAX_CAP_VAL: MaxCapVal = MaxCapVal(CAP);
const MIN_CAP_VAL: MinCapVal = MinCapVal(CAP);
const MIN_MAX_CAP_VAL: MinMaxCapVal = MinMaxCapVal::new(CAP, CAP);

mod range_to_inclusive {
    use super::*;
    use std::ops::RangeToInclusive;

    const CAP_RANGE: RangeToInclusive<usize> = ..=CAP;

    check_eq!(capacity: CAP_RANGE.capacity() => MAX_CAP_VAL);
    check_eq!(from_max_cap_val: RangeToInclusive::<usize>::from(MAX_CAP_VAL) => CAP_RANGE);
}

mod range_from {
    use super::*;
    use std::ops::RangeFrom;

    const CAP_RANGE: RangeFrom<usize> = CAP..;

    check_eq!(capacity: CAP_RANGE.capacity() => MIN_CAP_VAL);
    check_eq!(from_min_cap_val: RangeFrom::<usize>::from(MIN_CAP_VAL) => CAP_RANGE);
}

mod range_inclusive {
    use super::*;
    use crate::common::consts::{CAP, CAP_RANGE};
    use std::ops::RangeInclusive;

    const INVALID_RANGE: RangeInclusive<usize> = RangeInclusive::new(CAP, CAP - 1);

    check_eq!(capacity: CAP_RANGE.capacity() => MIN_MAX_CAP_VAL);

    panics!(invalid: INVALID_RANGE.capacity() => "Invalid range (start > end)");
    check_eq!(from_min_max: RangeInclusive::<usize>::from(MIN_MAX_CAP_VAL) => CAP_RANGE);
    check_eq!(from_exclusive: RangeInclusive::<usize>::from(EXACT_CAP_VAL) => CAP_RANGE);
}

mod range_full {
    use collection_cap::cap::UnboundedCap;

    use super::*;
    use std::ops::RangeFull;

    const CAP_RANGE: RangeFull = ..;

    check_eq!(capacity: CAP_RANGE.capacity() => UnboundedCap);
    check_eq!(from_unbounded: RangeFull::from(UnboundedCap) => CAP_RANGE);
}
