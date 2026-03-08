pub use core::ops::Bound;

use crate::common::check_eq;
use crate::common::consts::*;
use crate::{caps, check_intersects, check_overlaps, contains_size, range_bounds};

use collection_cap::cap::{ConstExactCap, ExactCapVal, MaxCapVal, MinCapVal, MinMaxCapVal};
use collection_cap::err::{IntersectError, OverlapError, OverlapErrorSpan, MaxOverflow, MaxUnderflow, MinOverflow, MinUnderflow};

pub const EXACT_CAP_VAL: ExactCapVal = ExactCapVal(CAP);
const MAX_CAP_VAL: MaxCapVal = MaxCapVal(CAP);
const MIN_CAP_VAL: MinCapVal = MinCapVal(CAP);
const MIN_MAX_CAP_VAL: MinMaxCapVal = MinMaxCapVal::new(CAP, CAP);

check_eq!(capacity: EXACT_CAP_VAL.capacity() => EXACT_CAP_VAL);
check_eq!(zero: ExactCapVal::ZERO => ExactCapVal(0));
check_eq!(from_static: ExactCapVal::from(ConstExactCap::<CAP>) => EXACT_CAP_VAL);

check_eq!(eq: EXACT_CAP_VAL == MIN_MAX_CAP_VAL => true);
check_eq!(ne: EXACT_CAP_VAL != MIN_MAX_CAP_VAL => false);

caps!(EXACT_CAP_VAL => { min: MIN_CAP_VAL, max: MAX_CAP_VAL });

contains_size!(EXACT_CAP_VAL => { cap: true, under: false, over: false });

const MAX_UNDERFLOW: MaxUnderflow<MinCapVal> = MaxUnderflow::<MinCapVal>::new(UNDER_CAP, MIN_CAP_VAL);
const MIN_OVERFLOW: MinOverflow<MaxCapVal> = MinOverflow::<MaxCapVal>::new(OVER_CAP, MAX_CAP_VAL);
check_intersects!(EXACT_CAP_VAL => {
    overflow: Err(IntersectError::Overflow(MIN_OVERFLOW)),
    underflow: Err(IntersectError::Underflow(MAX_UNDERFLOW))
});

const MIN_UNDERFLOW: MinUnderflow<MinCapVal> = MinUnderflow::<MinCapVal>::new(UNDER_CAP, MIN_CAP_VAL);
const MAX_OVERFLOW: MaxOverflow<MaxCapVal> = MaxOverflow::<MaxCapVal>::fixed(OVER_CAP, MAX_CAP_VAL);
const MAX_OVERFLOW_UNBOUNDED: MaxOverflow<MaxCapVal> = MaxOverflow::<MaxCapVal>::unbounded(MAX_CAP_VAL);
const OVERLAP_ERROR_SPAN: OverlapErrorSpan<MinCapVal, MaxCapVal> = OverlapErrorSpan::new(MAX_OVERFLOW, MIN_UNDERFLOW);
check_overlaps!(EXACT_CAP_VAL => {
    underflow: Err(OverlapError::Underflow(MIN_UNDERFLOW)),
    overflow: Err(OverlapError::Overflow(MAX_OVERFLOW)),
    unbounded: Err(OverlapError::Overflow(MAX_OVERFLOW_UNBOUNDED)),
    both: Err(OverlapError::Both(OVERLAP_ERROR_SPAN))
});

range_bounds!(EXACT_CAP_VAL => { start: Bound::Included(&CAP), end: Bound::Included(&CAP) });
