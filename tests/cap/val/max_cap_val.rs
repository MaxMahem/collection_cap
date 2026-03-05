use super::*;

check_eq!(capacity: MAX_CAP.capacity() => MAX_CAP);
check_eq!(from_static: MaxCapVal::from(StaticMaxCap::<CAP>) => MAX_CAP);
check_eq!(min_cap: MAX_CAP.min_cap() => UnboundedCap);
check_eq!(max_cap: MAX_CAP.max_cap() => MAX_CAP);
check_eq!(zero: MaxCapVal::ZERO => MaxCapVal(0));

check_eq!(from_range_to_inclusive: MaxCapVal::from(..=CAP) => MAX_CAP);

mod try_from_range_to {
    use super::*;

    check_eq!(valid: MaxCapVal::try_from(..CAP + 1) => Ok(MAX_CAP));
    check_eq!(empty: MaxCapVal::try_from(..0) => Err(EmptyRange));
}

mod range_bounds {
    use super::*;

    check_eq!(start_bound: MAX_CAP.start_bound() => Unbounded);
    check_eq!(end_bound: MAX_CAP.end_bound() => Included(&CAP));
}

mod check_compat {
    use super::*;

    check_eq!(compatible: MAX_CAP.check_compatibility(&COMPAT_ITER) => Ok(()));
    check_eq!(overflow: MAX_CAP.check_compatibility(&OVER_ITER) => Err(MIN_OVERFLOWS));

    panics!(bad_iter: MAX_CAP.check_compatibility(&INVALID_ITER) => "Invalid size hint");
}

mod check_fit {
    use super::*;

    check_eq!(compatible: MAX_CAP.check_fit(&COMPAT_ITER) => Ok(()));
    check_eq!(overflow: MAX_CAP.check_fit(&OVER_ITER) => Err(MAX_OVERFLOWS));
    check_eq!(overflow_unbounded: MAX_CAP.check_fit(&OVER_ITER_UNBOUNDED) 
        => Err(MaxOverflow::unbounded(MAX_CAP)));

    panics!(bad_iter: MAX_CAP.check_fit(&INVALID_ITER) => "Invalid size hint");
}
