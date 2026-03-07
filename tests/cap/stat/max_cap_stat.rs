use std::ops::{Bound, RangeToInclusive};

use collection_cap::cap::{StaticMaxCap, UnboundedCap};
use collection_cap::err::{MaxOverflow, MinOverflow};

use crate::common::check_eq;
use crate::common::consts::*;
use crate::{caps, check_compat, check_fit, contains_size, range_bounds};

caps!(StaticMaxCap::<CAP> => { min: UnboundedCap, max: StaticMaxCap::<CAP> });

contains_size!(StaticMaxCap::<CAP> => { cap: true, under: true, over: false });

const MIN_OVERFLOW: MinOverflow<StaticMaxCap<CAP>> = MinOverflow::<StaticMaxCap<CAP>>::new(OVER_CAP);
check_compat!(StaticMaxCap::<CAP> => { overflow: Err(MIN_OVERFLOW), underflow: Ok(()) });

const MAX_OVERFLOW: MaxOverflow<StaticMaxCap<CAP>> = MaxOverflow::<StaticMaxCap<CAP>>::fixed(OVER_CAP);
check_fit!(StaticMaxCap::<CAP> => {
    underflow: Ok(()),
    overflow: Err(MAX_OVERFLOW),
    unbounded: Err(MaxOverflow::UNBOUNDED),
    both: Err(MAX_OVERFLOW)
});

range_bounds!(StaticMaxCap::<CAP> => { start: Bound::Unbounded, end: Bound::Included(&CAP) });

check_eq!(range_const: StaticMaxCap::<CAP>::RANGE => ..=CAP);
check_eq!(from_range_to: RangeToInclusive::<usize>::from(StaticMaxCap::<CAP>) 
    => StaticMaxCap::<CAP>::RANGE);
