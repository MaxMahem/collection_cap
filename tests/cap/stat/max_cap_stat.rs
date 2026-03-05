use super::*;

use fluent_result::into::IntoResult;
use tap::Pipe;

check_eq!(min_cap: StaticMaxCap::<CAP>.min_cap() => UnboundedCap);
check_eq!(max_cap: StaticMaxCap::<CAP>.max_cap() => StaticMaxCap::<CAP>);

mod check_compat {
    use super::*;

    check_eq!(compatible: StaticMaxCap::<CAP>.check_compatibility(&COMPAT_ITER) => Ok(()));
    check_eq!(overflow: StaticMaxCap::<CAP>.check_compatibility(&OVER_ITER)
            => OVER_CAP.pipe(MinOverflow::<StaticMaxCap<CAP>>::new).into_err());

    panics!(bad_iter: StaticMaxCap::<CAP>.check_compatibility(&INVALID_ITER)
        => "Invalid size hint");
}

mod check_fit {
    use super::*;

    check_eq!(compatible: StaticMaxCap::<CAP>.check_fit(&COMPAT_ITER) => Ok(()));
    check_eq!(overflow: StaticMaxCap::<CAP>.check_fit(&OVER_ITER)
            => OVER_CAP.pipe(MaxOverflow::<StaticMaxCap<CAP>>::fixed).into_err());
    check_eq!(overflow_unbounded: StaticMaxCap::<CAP>.check_fit(&OVER_ITER_UNBOUNDED)
            => Err(MaxOverflow::UNBOUNDED));

    panics!(bad_iter: StaticMaxCap::<CAP>.check_fit(&INVALID_ITER)
        => "Invalid size hint");
}

mod range_bounds {
    use super::*;

    check_eq!(start_bound: StaticMaxCap::<CAP>.start_bound() => Unbounded);
    check_eq!(end_bound: StaticMaxCap::<CAP>.end_bound() => Included(&CAP));
}

check_eq!(range_const: StaticMaxCap::<CAP>::RANGE => ..=CAP);
check_eq!(from_range_to: RangeToInclusive::<usize>::from(StaticMaxCap::<CAP>) 
    => StaticMaxCap::<CAP>::RANGE);
