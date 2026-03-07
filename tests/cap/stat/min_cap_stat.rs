use std::ops::{Bound, RangeFrom};

use collection_cap::cap::{StaticMinCap, UnboundedCap};

use collection_cap::err::{MaxUnderflow, MinUnderflow};

use crate::common::check_eq;
use crate::common::consts::*;
use crate::{caps, check_compat, check_fit, contains_size, range_bounds};

caps!(StaticMinCap::<CAP> => { min: StaticMinCap::<CAP>, max: UnboundedCap });

contains_size!(StaticMinCap::<CAP> => { cap: true, under: false, over: true });

const MAX_UNDERFLOW: MaxUnderflow<StaticMinCap<CAP>> = MaxUnderflow::<StaticMinCap<CAP>>::new(UNDER_CAP);
check_compat!(StaticMinCap::<CAP> => { overflow: Ok(()), underflow: Err(MAX_UNDERFLOW) });

const MIN_UNDERFLOW: MinUnderflow<StaticMinCap<CAP>> = MinUnderflow::<StaticMinCap<CAP>>::new(UNDER_CAP);
check_fit!(StaticMinCap::<CAP> => {
    underflow: Err(MIN_UNDERFLOW),
    overflow: Ok(()),
    unbounded: Ok(()),
    both: Err(MIN_UNDERFLOW)
});

range_bounds!(StaticMinCap::<CAP> => { start: Bound::Included(&CAP), end: Bound::Unbounded });

check_eq!(range_const: StaticMinCap::<CAP>::RANGE => CAP..);
check_eq!(from_range_from: RangeFrom::<usize>::from(StaticMinCap::<CAP>) 
    => StaticMinCap::<CAP>::RANGE);
