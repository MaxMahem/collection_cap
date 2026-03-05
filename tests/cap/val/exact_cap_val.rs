use super::*;

check_eq!(capacity: EXACT_CAP.capacity() => EXACT_CAP);
check_eq!(zero: ExactCapVal::ZERO => ExactCapVal(0));
check_eq!(from_static: ExactCapVal::from(StaticExactCap::<CAP>) => EXACT_CAP);
check_eq!(min_cap: EXACT_CAP.min_cap() => MIN_CAP);
check_eq!(max_cap: EXACT_CAP.max_cap() => MAX_CAP);

check_eq!(eq: EXACT_CAP == MIN_MAX_CAP => true);
check_eq!(ne: EXACT_CAP != MIN_MAX_CAP => false);

mod range_bounds {
    use super::*;

    check_eq!(start_bound: EXACT_CAP.start_bound() => Included(&CAP));
    check_eq!(end_bound: EXACT_CAP.end_bound() => Included(&CAP));
}

mod check_compat {
    use super::*;

    check_eq!(compatible: EXACT_CAP.check_compatibility(&COMPAT_ITER) => Ok(()));
    check_eq!(overflow: EXACT_CAP.check_compatibility(&OVER_ITER) => Err(CAP_ERROR_OVERFLOW));
    check_eq!(underflow: EXACT_CAP.check_compatibility(&UNDER_ITER) => Err(CAP_ERROR_UNDERFLOW));

    panics!(bad_iter: EXACT_CAP.check_compatibility(&INVALID_ITER) => "Invalid size hint");
}

mod check_fit {
    use super::*;

    check_eq!(compatible: EXACT_CAP.check_fit(&COMPAT_ITER) => Ok(()));
    check_eq!(underflow: EXACT_CAP.check_fit(&UNDER_ITER) => Err(FIT_ERROR_UNDERFLOW));
    check_eq!(overflow: EXACT_CAP.check_fit(&OVER_ITER) => Err(FIT_ERROR_OVERFLOW));
    check_eq!(overflow_unbounded: EXACT_CAP.check_fit(&OVER_ITER_UNBOUNDED) => Err(FitError::Overflow(MaxOverflow::unbounded(MAX_CAP))));
    check_eq!(both: EXACT_CAP.check_fit(&BOTH_ITER) => Err(FIT_ERROR_BOTH));

    panics!(bad_iter: EXACT_CAP.check_fit(&INVALID_ITER) => "Invalid size hint");
}
