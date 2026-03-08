pub use core::ops::Bound;

use crate::common::consts::*;
use crate::common::{check_eq, panics};
use crate::{caps, check_intersects, check_overlaps, contains_size, range_bounds};

use collection_cap::cap::{ExactCapVal, MaxCapVal, MinCapVal, MinMaxCapVal, ConstExactCap, ConstMinMaxCap};
use collection_cap::err::{IntersectError, OverlapError, OverlapErrorSpan, FromRangeError, InvalidRange, MaxOverflow, MaxUnderflow, MinOverflow, MinUnderflow};

pub const MIN_MAX_CAP_VAL: MinMaxCapVal = MinMaxCapVal::new(CAP, CAP);

const EXACT_CAP_VAL: ExactCapVal = ExactCapVal(CAP);
const MAX_CAP_VAL: MaxCapVal = MaxCapVal(CAP);
const MIN_CAP_VAL: MinCapVal = MinCapVal(CAP);

const FROM_RANGE_ERR_INVALID: FromRangeError = FromRangeError::InvalidRange(InvalidRange::new(CAP, CAP - 1));

use core::ops::{Range, RangeInclusive};

check_eq!(new: MinMaxCapVal::new(CAP, CAP) => MIN_MAX_CAP_VAL);
panics!(new_invalid_range: MinMaxCapVal::new(CAP, CAP - 1) => "Invalid range (start > end)");
check_eq!(zero: MinMaxCapVal::ZERO => MinMaxCapVal::new(0, 0));

check_eq!(capacity: MIN_MAX_CAP_VAL.capacity() => MIN_MAX_CAP_VAL);
check_eq!(min_val: MIN_MAX_CAP_VAL.min() => MinCapVal(CAP));
check_eq!(max_val: MIN_MAX_CAP_VAL.max() => MaxCapVal(CAP));

mod from {
    use super::*;

    check_eq!(exact: MinMaxCapVal::from(EXACT_CAP_VAL) => MIN_MAX_CAP_VAL);
    check_eq!(static_cap: MinMaxCapVal::from(ConstMinMaxCap::<CAP, CAP>) => MIN_MAX_CAP_VAL);
    check_eq!(static_cap_exact: MinMaxCapVal::from(ConstExactCap::<CAP>) => MIN_MAX_CAP_VAL);
}

mod try_from_range {
    use super::*;

    check_eq!(valid: MinMaxCapVal::try_from(CAP..CAP + 1) => Ok(MIN_MAX_CAP_VAL));
    check_eq!(empty: MinMaxCapVal::try_from(CAP..CAP) => Err(FromRangeError::EMPTY));

    const INVALID_RANGE: Range<usize> = Range { start: CAP, end: CAP - 1 };
    check_eq!(invalid: MinMaxCapVal::try_from(INVALID_RANGE) => Err(FROM_RANGE_ERR_INVALID));
    check_eq!(inclusive_valid: MinMaxCapVal::try_from(CAP_RANGE) => Ok(MIN_MAX_CAP_VAL));

    const INVALID_INCLUSIVE_RANGE: RangeInclusive<usize> = RangeInclusive::new(CAP, CAP - 1);
    check_eq!(inclusive_invalid: MinMaxCapVal::try_from(INVALID_INCLUSIVE_RANGE) 
        => Err(InvalidRange::new(CAP, CAP - 1)));
}

check_eq!(eq: MIN_MAX_CAP_VAL == EXACT_CAP_VAL => true);
check_eq!(ne: MIN_MAX_CAP_VAL != EXACT_CAP_VAL => false);

caps!(MIN_MAX_CAP_VAL => { min: MIN_CAP_VAL, max: MAX_CAP_VAL });

contains_size!(MIN_MAX_CAP_VAL => { cap: true, under: false, over: false });

const MAX_UNDERFLOW: MaxUnderflow<MinCapVal> = MaxUnderflow::<MinCapVal>::new(UNDER_CAP, MIN_CAP_VAL);
const MIN_OVERFLOW: MinOverflow<MaxCapVal> = MinOverflow::<MaxCapVal>::new(OVER_CAP, MAX_CAP_VAL);
check_intersects!(MIN_MAX_CAP_VAL => { 
    overflow: Err(IntersectError::Overflow(MIN_OVERFLOW)), 
    underflow: Err(IntersectError::Underflow(MAX_UNDERFLOW)) 
});

const MIN_UNDERFLOW: MinUnderflow<MinCapVal> = MinUnderflow::<MinCapVal>::new(UNDER_CAP, MIN_CAP_VAL);
const MAX_OVERFLOW: MaxOverflow<MaxCapVal> = MaxOverflow::<MaxCapVal>::fixed(OVER_CAP, MAX_CAP_VAL);
const MAX_OVERFLOW_UNBOUNDED: MaxOverflow<MaxCapVal> = MaxOverflow::<MaxCapVal>::unbounded(MAX_CAP_VAL);
const OVERLAP_ERROR_SPAN: OverlapErrorSpan<MinCapVal, MaxCapVal> = OverlapErrorSpan::new(MAX_OVERFLOW, MIN_UNDERFLOW);
check_overlaps!(MIN_MAX_CAP_VAL => {
    underflow: Err(OverlapError::Underflow(MIN_UNDERFLOW)),
    overflow: Err(OverlapError::Overflow(MAX_OVERFLOW)),
    unbounded: Err(OverlapError::Overflow(MAX_OVERFLOW_UNBOUNDED)),
    both: Err(OverlapError::Both(OVERLAP_ERROR_SPAN))
});

range_bounds!(MIN_MAX_CAP_VAL => { start: Bound::Included(&CAP), end: Bound::Included(&CAP) });