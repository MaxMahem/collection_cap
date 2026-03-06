#![allow(unused_imports)]
use crate::common::consts::*;
use super::*;

use fluent_result::into::IntoResult;
use tap::Pipe;

check_eq!(min_cap: StaticMinCap::<{ base::CAP }>.min_cap() => StaticMinCap::<{ base::CAP }>);
check_eq!(max_cap: StaticMinCap::<{ base::CAP }>.max_cap() => UnboundedCap);

mod check_compat {
    use super::*;

    check_eq!(compatible: StaticMinCap::<{ base::CAP }>.check_compatibility(&iter::COMPAT_ITER) => Ok(()));
    check_eq!(underflow: StaticMinCap::<{ base::CAP }>.check_compatibility(&iter::UNDER_ITER)
        => base::UNDER_CAP.pipe(MaxUnderflow::<StaticMinCap<{ base::CAP }>>::new).into_err());
    panics!(bad_iter: StaticMinCap::<{ base::CAP }>.check_compatibility(&iter::INVALID_ITER)
        => "Invalid size hint");
}

mod check_fit {
    use super::*;

    check_eq!(compatible: StaticMinCap::<{ base::CAP }>.check_fit(&iter::COMPAT_ITER) => Ok(()));
    check_eq!(underflow: StaticMinCap::<{ base::CAP }>.check_fit(&iter::UNDER_ITER)
        => base::UNDER_CAP.pipe(MinUnderflow::<StaticMinCap<{ base::CAP }>>::new).into_err());
    panics!(bad_iter: StaticMinCap::<{ base::CAP }>.check_fit(&iter::INVALID_ITER)
        => "Invalid size hint");
}

mod range_bounds {
    use super::*;

    check_eq!(start_bound: StaticMinCap::<{ base::CAP }>.start_bound() => Included(&base::CAP));
    check_eq!(end_bound: StaticMinCap::<{ base::CAP }>.end_bound() => Unbounded);
}

check_eq!(range_const: StaticMinCap::<{ base::CAP }>::RANGE => base::CAP..);
check_eq!(from_range_from: RangeFrom::<usize>::from(StaticMinCap::<{ base::CAP }>) 
    => StaticMinCap::<{ base::CAP }>::RANGE);
