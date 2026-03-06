#![allow(unused_imports)]
use crate::common::consts::*;
use super::*;

use fluent_result::into::IntoResult;
use tap::Pipe;

check_eq!(min_cap: StaticMaxCap::<{ base::CAP }>.min_cap() => UnboundedCap);
check_eq!(max_cap: StaticMaxCap::<{ base::CAP }>.max_cap() => StaticMaxCap::<{ base::CAP }>);

mod check_compat {
    use super::*;

    check_eq!(compatible: StaticMaxCap::<{ base::CAP }>.check_compatibility(&iter::COMPAT_ITER) => Ok(()));
    check_eq!(overflow: StaticMaxCap::<{ base::CAP }>.check_compatibility(&iter::OVER_ITER)
            => base::OVER_CAP.pipe(MinOverflow::<StaticMaxCap<{ base::CAP }>>::new).into_err());

    panics!(bad_iter: StaticMaxCap::<{ base::CAP }>.check_compatibility(&iter::INVALID_ITER)
        => "Invalid size hint");
}

mod check_fit {
    use super::*;

    check_eq!(compatible: StaticMaxCap::<{ base::CAP }>.check_fit(&iter::COMPAT_ITER) => Ok(()));
    check_eq!(overflow: StaticMaxCap::<{ base::CAP }>.check_fit(&iter::OVER_ITER)
            => base::OVER_CAP.pipe(MaxOverflow::<StaticMaxCap<{ base::CAP }>>::fixed).into_err());
    check_eq!(overflow_unbounded: StaticMaxCap::<{ base::CAP }>.check_fit(&iter::OVER_ITER_UNBOUNDED)
            => Err(MaxOverflow::UNBOUNDED));

    panics!(bad_iter: StaticMaxCap::<{ base::CAP }>.check_fit(&iter::INVALID_ITER)
        => "Invalid size hint");
}

mod range_bounds {
    use super::*;

    check_eq!(start_bound: StaticMaxCap::<{ base::CAP }>.start_bound() => Unbounded);
    check_eq!(end_bound: StaticMaxCap::<{ base::CAP }>.end_bound() => Included(&base::CAP));
}

check_eq!(range_const: StaticMaxCap::<{ base::CAP }>::RANGE => ..=base::CAP);
check_eq!(from_range_to: RangeToInclusive::<usize>::from(StaticMaxCap::<{ base::CAP }>) 
    => StaticMaxCap::<{ base::CAP }>::RANGE);
