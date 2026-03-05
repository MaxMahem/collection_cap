use super::*;

use fluent_result::into::IntoResult;
use tap::Pipe;

check_eq!(min_cap: StaticMinCap::<CAP>.min_cap() => StaticMinCap::<CAP>);
check_eq!(max_cap: StaticMinCap::<CAP>.max_cap() => UnboundedCap);

mod check_compat {
    use super::*;

    check_eq!(compatible: StaticMinCap::<CAP>.check_compatibility(&COMPAT_ITER) => Ok(()));
    check_eq!(underflow: StaticMinCap::<CAP>.check_compatibility(&UNDER_ITER)
        => UNDER_CAP.pipe(MaxUnderflow::<StaticMinCap<CAP>>::new).into_err());
    panics!(bad_iter: StaticMinCap::<CAP>.check_compatibility(&INVALID_ITER)
        => "Invalid size hint");
}

mod check_fit {
    use super::*;

    check_eq!(compatible: StaticMinCap::<CAP>.check_fit(&COMPAT_ITER) => Ok(()));
    check_eq!(underflow: StaticMinCap::<CAP>.check_fit(&UNDER_ITER)
        => UNDER_CAP.pipe(MinUnderflow::<StaticMinCap<CAP>>::new).into_err());
    panics!(bad_iter: StaticMinCap::<CAP>.check_fit(&INVALID_ITER)
        => "Invalid size hint");
}

mod range_bounds {
    use super::*;

    check_eq!(start_bound: StaticMinCap::<CAP>.start_bound() => Included(&CAP));
    check_eq!(end_bound: StaticMinCap::<CAP>.end_bound() => Unbounded);
}

check_eq!(range_const: StaticMinCap::<CAP>::RANGE => CAP..);
check_eq!(from_range_from: RangeFrom::<usize>::from(StaticMinCap::<CAP>) 
    => StaticMinCap::<CAP>::RANGE);
