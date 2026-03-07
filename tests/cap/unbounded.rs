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

    check_eq!(compatible: UnboundedCap.check_compatibility(&COMPAT_ITER) => Ok(()));
    check_eq!(overflow: UnboundedCap.check_compatibility(&OVER_ITER) => Ok(()));
    check_eq!(underflow: UnboundedCap.check_compatibility(&UNDER_ITER) => Ok(()));
}

mod check_fit {
    use super::*;

    check_eq!(fit: UnboundedCap.check_fit(&COMPAT_ITER) => Ok(()));
    check_eq!(underflow: UnboundedCap.check_fit(&UNDER_ITER) => Ok(()));
    check_eq!(overflow: UnboundedCap.check_fit(&OVER_ITER) => Ok(()));
    check_eq!(overflow_unbounded: UnboundedCap.check_fit(&OVER_ITER_UNBOUNDED) => Ok(()));
}

range_bounds!(UnboundedCap => { start: Bound::Unbounded, end: Bound::Unbounded });
