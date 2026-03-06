use core::ops::Bound::*;
use core::ops::RangeBounds;

use collection_cap::cap::UnboundedCap;
use collection_cap::{Capacity, VariableCap};

use crate::common::check_eq;
use crate::common::consts::*;

check_eq!(min_cap: UnboundedCap.min_cap() => UnboundedCap);
check_eq!(max_cap: UnboundedCap.max_cap() => UnboundedCap);

check_eq!(capacity: UnboundedCap.capacity() => UnboundedCap);

check_eq!(compatible: UnboundedCap.check_compatibility(&iter::COMPAT_ITER) => Ok(()));
check_eq!(fit: UnboundedCap.check_fit(&iter::COMPAT_ITER) => Ok(()));

mod range_bounds {
    use super::*;

    check_eq!(start_bound: UnboundedCap.start_bound() => Unbounded);
    check_eq!(end_bound: UnboundedCap.end_bound() => Unbounded);
}
