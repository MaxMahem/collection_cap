use std::ops::{Bound, RangeInclusive};

use collection_cap::cap::{StaticExactCap, StaticMaxCap, StaticMinCap};
use collection_cap::err::{CompatError, FitError, FitErrorSpan, MaxOverflow, MaxUnderflow, MinOverflow, MinUnderflow};

use crate::common::check_eq;
use crate::common::consts::*;
use crate::{caps, check_compat, check_fit, contains_size, range_bounds};

caps!(StaticExactCap::<CAP> => { min: StaticMinCap::<CAP>, max: StaticMaxCap::<CAP> });

contains_size!(StaticExactCap::<CAP> => { cap: true, under: false, over: false });

const MIN_OVERFLOW: MinOverflow<StaticMaxCap<CAP>> = MinOverflow::<StaticMaxCap<CAP>>::new(OVER_CAP);
const MAX_UNDERFLOW: MaxUnderflow<StaticMinCap<CAP>> = MaxUnderflow::<StaticMinCap<CAP>>::new(UNDER_CAP);
const COMPAT_ERR_OVERFLOW: CompatError<StaticMinCap<CAP>, StaticMaxCap<CAP>> = CompatError::Overflow(MIN_OVERFLOW);
const COMPAT_ERR_UNDERFLOW: CompatError<StaticMinCap<CAP>, StaticMaxCap<CAP>> = CompatError::Underflow(MAX_UNDERFLOW);

check_compat!(StaticExactCap::<CAP> => {
    overflow: Err(COMPAT_ERR_OVERFLOW),
    underflow: Err(COMPAT_ERR_UNDERFLOW)
});

const MIN_UNDERFLOW: MinUnderflow<StaticMinCap<CAP>> = MinUnderflow::<StaticMinCap<CAP>>::new(UNDER_CAP);
const MAX_OVERFLOW: MaxOverflow<StaticMaxCap<CAP>> = MaxOverflow::<StaticMaxCap<CAP>>::fixed(OVER_CAP);
const MAX_OVERFLOW_UNBOUNDED: MaxOverflow<StaticMaxCap<CAP>> = MaxOverflow::<StaticMaxCap<CAP>>::UNBOUNDED;
const FIT_ERROR_SPAN: FitErrorSpan<StaticMinCap<CAP>, StaticMaxCap<CAP>> =
    FitErrorSpan::new(MAX_OVERFLOW, MIN_UNDERFLOW);

check_fit!(StaticExactCap::<CAP> => {
    underflow: Err(FitError::Underflow(MIN_UNDERFLOW)),
    overflow: Err(FitError::Overflow(MAX_OVERFLOW)),
    unbounded: Err(FitError::Overflow(MAX_OVERFLOW_UNBOUNDED)),
    both: Err(FitError::Both(FIT_ERROR_SPAN))
});

range_bounds!(StaticExactCap::<CAP> => { start: Bound::Included(&CAP), end: Bound::Included(&CAP) });

check_eq!(range_const: StaticExactCap::<CAP>::RANGE => CAP_RANGE);
check_eq!(from_range_inclusive: RangeInclusive::<usize>::from(StaticExactCap::<CAP>)
    => StaticExactCap::<CAP>::RANGE);
