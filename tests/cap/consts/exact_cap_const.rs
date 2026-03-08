use std::ops::{Bound, RangeInclusive};

use collection_cap::cap::{ConstExactCap, ConstMaxCap, ConstMinCap};
use collection_cap::err::{IntersectError, OverlapError, OverlapErrorSpan, MaxOverflow, MaxUnderflow, MinOverflow, MinUnderflow};

use crate::common::check_eq;
use crate::common::consts::*;
use crate::{caps, check_intersects, check_overlaps, contains_size, range_bounds};

caps!(ConstExactCap::<CAP> => { min: ConstMinCap::<CAP>, max: ConstMaxCap::<CAP> });

contains_size!(ConstExactCap::<CAP> => { cap: true, under: false, over: false });

const MIN_OVERFLOW: MinOverflow<ConstMaxCap<CAP>> = MinOverflow::<ConstMaxCap<CAP>>::new(OVER_CAP);
const MAX_UNDERFLOW: MaxUnderflow<ConstMinCap<CAP>> = MaxUnderflow::<ConstMinCap<CAP>>::new(UNDER_CAP);
const INTERSECT_ERR_OVERFLOW: IntersectError<ConstMinCap<CAP>, ConstMaxCap<CAP>> = IntersectError::Overflow(MIN_OVERFLOW);
const INTERSECT_ERR_UNDERFLOW: IntersectError<ConstMinCap<CAP>, ConstMaxCap<CAP>> = IntersectError::Underflow(MAX_UNDERFLOW);

check_intersects!(ConstExactCap::<CAP> => {
    overflow: Err(INTERSECT_ERR_OVERFLOW),
    underflow: Err(INTERSECT_ERR_UNDERFLOW)
});

const MIN_UNDERFLOW: MinUnderflow<ConstMinCap<CAP>> = MinUnderflow::<ConstMinCap<CAP>>::new(UNDER_CAP);
const MAX_OVERFLOW: MaxOverflow<ConstMaxCap<CAP>> = MaxOverflow::<ConstMaxCap<CAP>>::fixed(OVER_CAP);
const MAX_OVERFLOW_UNBOUNDED: MaxOverflow<ConstMaxCap<CAP>> = MaxOverflow::<ConstMaxCap<CAP>>::UNBOUNDED;
const OVERLAP_ERROR_SPAN: OverlapErrorSpan<ConstMinCap<CAP>, ConstMaxCap<CAP>> = OverlapErrorSpan::new(MAX_OVERFLOW, MIN_UNDERFLOW);

check_overlaps!(ConstExactCap::<CAP> => {
    underflow: Err(OverlapError::Underflow(MIN_UNDERFLOW)),
    overflow: Err(OverlapError::Overflow(MAX_OVERFLOW)),
    unbounded: Err(OverlapError::Overflow(MAX_OVERFLOW_UNBOUNDED)),
    both: Err(OverlapError::Both(OVERLAP_ERROR_SPAN))
});

range_bounds!(ConstExactCap::<CAP> => { start: Bound::Included(&CAP), end: Bound::Included(&CAP) });

check_eq!(range_const: ConstExactCap::<CAP>::RANGE => CAP_RANGE);
check_eq!(from_range_inclusive: RangeInclusive::<usize>::from(ConstExactCap::<CAP>)
    => ConstExactCap::<CAP>::RANGE);
