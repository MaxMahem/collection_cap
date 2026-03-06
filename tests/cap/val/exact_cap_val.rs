#![allow(unused_imports)]
use crate::common::consts::*;
use super::*;

check_eq!(capacity: EXACT_CAP.capacity() => EXACT_CAP);
check_eq!(zero: ExactCapVal::ZERO => ExactCapVal(0));
check_eq!(from_static: ExactCapVal::from(StaticExactCap::<{ base::CAP }>) => EXACT_CAP);
check_eq!(min_cap: EXACT_CAP.min_cap() => MIN_CAP);
check_eq!(max_cap: EXACT_CAP.max_cap() => MAX_CAP);

check_eq!(eq: EXACT_CAP == MIN_MAX_CAP => true);
check_eq!(ne: EXACT_CAP != MIN_MAX_CAP => false);

mod range_bounds {
    use super::*;

    check_eq!(start_bound: EXACT_CAP.start_bound() => Included(&base::CAP));
    check_eq!(end_bound: EXACT_CAP.end_bound() => Included(&base::CAP));
}

mod check_compat {
    use super::*;

    check_eq!(compatible: EXACT_CAP.check_compatibility(&iter::COMPAT_ITER) => Ok(()));
    check_eq!(overflow: EXACT_CAP.check_compatibility(&iter::OVER_ITER) => Err(err_val_compat::OVERFLOW));
    check_eq!(underflow: EXACT_CAP.check_compatibility(&iter::UNDER_ITER) => Err(err_val_compat::UNDERFLOW));

    panics!(bad_iter: EXACT_CAP.check_compatibility(&iter::INVALID_ITER) => "Invalid size hint");
}

mod check_fit {
    use super::*;

    check_eq!(compatible: EXACT_CAP.check_fit(&iter::COMPAT_ITER) => Ok(()));
    check_eq!(underflow: EXACT_CAP.check_fit(&iter::UNDER_ITER) => Err(err_val_fit::UNDERFLOW));
    check_eq!(overflow: EXACT_CAP.check_fit(&iter::OVER_ITER) => Err(err_val_fit::OVERFLOW));
    check_eq!(overflow_unbounded: EXACT_CAP.check_fit(&iter::OVER_ITER_UNBOUNDED) => Err(FitError::Overflow(MaxOverflow::unbounded(MAX_CAP))));
    check_eq!(both: EXACT_CAP.check_fit(&iter::BOTH_ITER) => Err(err_val_fit::BOTH));

    panics!(bad_iter: EXACT_CAP.check_fit(&iter::INVALID_ITER) => "Invalid size hint");
}
