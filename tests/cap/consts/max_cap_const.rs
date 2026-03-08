use std::ops::{Bound, RangeToInclusive};

use collection_cap::cap::{ConstMaxCap, UnboundedCap};
use collection_cap::err::{MaxOverflow, MinOverflow};

use crate::common::check_eq;
use crate::common::consts::*;
use crate::{caps, check_intersects, check_overlaps, contains_size, range_bounds};

caps!(ConstMaxCap::<CAP> => { min: UnboundedCap, max: ConstMaxCap::<CAP> });

contains_size!(ConstMaxCap::<CAP> => { cap: true, under: true, over: false });

const MIN_OVERFLOW: MinOverflow<ConstMaxCap<CAP>> = MinOverflow::<ConstMaxCap<CAP>>::new(OVER_CAP);
check_intersects!(ConstMaxCap::<CAP> => { overflow: Err(MIN_OVERFLOW), underflow: Ok(()) });

const MAX_OVERFLOW: MaxOverflow<ConstMaxCap<CAP>> = MaxOverflow::<ConstMaxCap<CAP>>::fixed(OVER_CAP);
check_overlaps!(ConstMaxCap::<CAP> => {
    underflow: Ok(()),
    overflow: Err(MAX_OVERFLOW),
    unbounded: Err(MaxOverflow::UNBOUNDED),
    both: Err(MAX_OVERFLOW)
});

range_bounds!(ConstMaxCap::<CAP> => { start: Bound::Unbounded, end: Bound::Included(&CAP) });

check_eq!(range_const: ConstMaxCap::<CAP>::RANGE => ..=CAP);
check_eq!(from_range_to: RangeToInclusive::<usize>::from(ConstMaxCap::<CAP>) 
    => ConstMaxCap::<CAP>::RANGE);
