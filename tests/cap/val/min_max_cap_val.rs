use super::*;

check_eq!(capacity: MIN_MAX_CAP.capacity() => MIN_MAX_CAP);
check_eq!(new: MinMaxCapVal::new(CAP, CAP) => MIN_MAX_CAP);
check_eq!(zero: MinMaxCapVal::ZERO => MinMaxCapVal::new(0, 0));
check_eq!(min_val: MIN_MAX_CAP.min() => MinCapVal(CAP));
check_eq!(max_val: MIN_MAX_CAP.max() => MaxCapVal(CAP));
check_eq!(min_cap: MIN_MAX_CAP.min_cap() => MIN_CAP);
check_eq!(max_cap: MIN_MAX_CAP.max_cap() => MAX_CAP);

mod from {
    use super::*;

    check_eq!(exact: MinMaxCapVal::from(EXACT_CAP) => MIN_MAX_CAP);
    check_eq!(static_cap: MinMaxCapVal::from(StaticMinMaxCap::<CAP, CAP>) => MIN_MAX_CAP);
    check_eq!(static_cap_exact: MinMaxCapVal::from(StaticExactCap::<CAP>) => MIN_MAX_CAP);
}

mod try_from_range {
    use super::*;

    check_eq!(valid: MinMaxCapVal::try_from(CAP..CAP + 1) => Ok(MIN_MAX_CAP));
    check_eq!(empty: MinMaxCapVal::try_from(CAP..CAP) => Err(RangeError::Empty(EmptyRange)));
    check_eq!(invalid: MinMaxCapVal::try_from(core::ops::Range { start: CAP, end: CAP - 1 }) => Err(RangeError::InvalidRange(InvalidRange)));
    check_eq!(inclusive_valid: MinMaxCapVal::try_from(CAP..=CAP) => Ok(MIN_MAX_CAP));
    check_eq!(inclusive_invalid: MinMaxCapVal::try_from(core::ops::RangeInclusive::new(CAP, CAP - 1)) => Err(InvalidRange));
}

check_eq!(eq: MIN_MAX_CAP == EXACT_CAP => true);
check_eq!(ne: MIN_MAX_CAP != EXACT_CAP => false);

mod range_bounds {
    use super::*;

    check_eq!(start_bound: MIN_MAX_CAP.start_bound() => Included(&CAP));
    check_eq!(end_bound: MIN_MAX_CAP.end_bound() => Included(&CAP));
}

mod check_compat {
    use super::*;

    check_eq!(compatible: MIN_MAX_CAP.check_compatibility(&COMPAT_ITER) => Ok(()));
    check_eq!(overflow: MIN_MAX_CAP.check_compatibility(&OVER_ITER) => Err(CAP_ERROR_OVERFLOW));
    check_eq!(underflow: MIN_MAX_CAP.check_compatibility(&UNDER_ITER) => Err(CAP_ERROR_UNDERFLOW));

    panics!(bad_iter: MIN_MAX_CAP.check_compatibility(&INVALID_ITER) => "Invalid size hint");
    panics!(invalid_range: MinMaxCapVal::new(CAP, CAP - 1) => "Invalid range (start > end)");
}

mod check_fit {
    use super::*;

    check_eq!(compatible: MIN_MAX_CAP.check_fit(&COMPAT_ITER) => Ok(()));
    check_eq!(underflow: MIN_MAX_CAP.check_fit(&UNDER_ITER) => Err(FIT_ERROR_UNDERFLOW));
    check_eq!(overflow: MIN_MAX_CAP.check_fit(&OVER_ITER) => Err(FIT_ERROR_OVERFLOW));
    check_eq!(overflow_unbounded: MIN_MAX_CAP.check_fit(&OVER_ITER_UNBOUNDED) => Err(FitError::Overflow(MaxOverflow::unbounded(MAX_CAP))));
    check_eq!(both: MIN_MAX_CAP.check_fit(&BOTH_ITER) => Err(FIT_ERROR_BOTH));

    panics!(bad_iter: MIN_MAX_CAP.check_fit(&INVALID_ITER) => "Invalid size hint");
}
