use core::ops::Bound;

use collection_cap::cap::UnboundedCap;
use collection_cap::{Capacity, VariableCap};

use crate::common::check_eq;
use crate::common::consts::iter::*;
use crate::{caps, contains_size, range_bounds};

caps!(UnboundedCap => { min: UnboundedCap, max: UnboundedCap });

contains_size!(UnboundedCap => { cap: true, under: true, over: true });

check_eq!(capacity: UnboundedCap.capacity() => UnboundedCap);

mod check_compatibility {
    use super::*;

    check_eq!(intersecting: UnboundedCap.check_intersects(&INTERSECT_ITER) => Ok(()));
    check_eq!(overflow: UnboundedCap.check_intersects(&OVER_ITER) => Ok(()));
    check_eq!(underflow: UnboundedCap.check_intersects(&UNDER_ITER) => Ok(()));
}

mod check_fit {
    use super::*;

    check_eq!(overlap: UnboundedCap.check_overlaps(&INTERSECT_ITER) => Ok(()));
    check_eq!(overlap_underflow: UnboundedCap.check_overlaps(&UNDER_ITER) => Ok(()));
    check_eq!(overlap_overflow: UnboundedCap.check_overlaps(&OVER_ITER) => Ok(()));
    check_eq!(overlap_both: UnboundedCap.check_overlaps(&BOTH_ITER) => Ok(()));
}

range_bounds!(UnboundedCap => { start: Bound::Unbounded, end: Bound::Unbounded });
