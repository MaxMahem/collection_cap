#![allow(unused_imports)]
use crate::common::consts::*;
use super::*;

use fluent_result::into::IntoResult;
use tap::Pipe;

check_eq!(min_cap: StaticMinMaxCap::<{ base::CAP }, { base::CAP }>.min_cap() => StaticMinCap::<{ base::CAP }>);
check_eq!(max_cap: StaticMinMaxCap::<{ base::CAP }, { base::CAP }>.max_cap() => StaticMaxCap::<{ base::CAP }>);

mod check_compat {
    use super::*;

    check_eq!(compatible: StaticMinMaxCap::<{ base::CAP }, { base::CAP }>.check_compatibility(&iter::COMPAT_ITER) => Ok(()));
    check_eq!(overflow: StaticMinMaxCap::<{ base::CAP }, { base::CAP }>.check_compatibility(&iter::OVER_ITER)
        => base::OVER_CAP.pipe(MinOverflow::<StaticMaxCap<{ base::CAP }>>::new)
            .pipe(CompatError::Overflow)
            .into_err()
    );

    check_eq!(underflow: StaticMinMaxCap::<{ base::CAP }, { base::CAP }>.check_compatibility(&iter::UNDER_ITER)
        => base::UNDER_CAP.pipe(MaxUnderflow::<StaticMinCap<{ base::CAP }>>::new)
            .pipe(CompatError::Underflow)
            .into_err()
    );

    panics!(bad_iter: StaticMinMaxCap::<{ base::CAP }, { base::CAP }>.check_compatibility(&iter::INVALID_ITER)
        => "Invalid size hint");
}

mod check_fit {
    use super::*;

    check_eq!(compatible: StaticMinMaxCap::<{ base::CAP }, { base::CAP }>.check_fit(&iter::COMPAT_ITER) => Ok(()));

    check_eq!(underflow: StaticMinMaxCap::<{ base::CAP }, { base::CAP }>.check_fit(&iter::UNDER_ITER)
        => base::UNDER_CAP.pipe(MinUnderflow::<StaticMinCap<{ base::CAP }>>::new)
            .pipe(FitError::Underflow)
            .into_err()
    );

    check_eq!(overflow: StaticMinMaxCap::<{ base::CAP }, { base::CAP }>.check_fit(&iter::OVER_ITER)
        => base::OVER_CAP.pipe(MaxOverflow::<StaticMaxCap<{ base::CAP }>>::fixed)
            .pipe(FitError::Overflow)
            .into_err()
    );

    check_eq!(overflow_unbounded: StaticMinMaxCap::<{ base::CAP }, { base::CAP }>.check_fit(&iter::OVER_ITER_UNBOUNDED)
        => MaxOverflow::UNBOUNDED.pipe(FitError::Overflow).into_err());

    check_eq!(both: StaticMinMaxCap::<{ base::CAP }, { base::CAP }>.check_fit(&iter::BOTH_ITER)
    => FitErrorSpan::new(
        MaxOverflow::<StaticMaxCap<{ base::CAP }>>::fixed(base::OVER_CAP),
        MinUnderflow::<StaticMinCap<{ base::CAP }>>::new(base::UNDER_CAP)
    ).pipe(FitError::Both).into_err());

    panics!(bad_iter: StaticMinMaxCap::<{ base::CAP }, { base::CAP }>.check_fit(&iter::INVALID_ITER)
        => "Invalid size hint");
}

mod range_bounds {
    use super::*;

    check_eq!(start_bound: StaticMinMaxCap::<{ base::CAP }, { base::CAP }>.start_bound() => Included(&base::CAP));
    check_eq!(end_bound: StaticMinMaxCap::<{ base::CAP }, { base::CAP }>.end_bound() => Included(&base::CAP));
}

check_eq!(range_const: StaticMinMaxCap::<{ base::CAP }, { base::CAP }>::RANGE => base::CAP..=base::CAP);
check_eq!(from_range_inclusive: RangeInclusive::<usize>::from(StaticMinMaxCap::<{ base::CAP }, { base::CAP }>) 
    => StaticMinMaxCap::<{ base::CAP }, { base::CAP }>::RANGE);
