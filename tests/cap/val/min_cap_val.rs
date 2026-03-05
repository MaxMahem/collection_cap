use super::*;

check_eq!(capacity: MIN_CAP.capacity() => MIN_CAP);
check_eq!(from_static: MinCapVal::from(StaticMinCap::<CAP>) => MIN_CAP);
check_eq!(min_cap: MIN_CAP.min_cap() => MIN_CAP);
check_eq!(max_cap: MIN_CAP.max_cap() => UnboundedCap);
check_eq!(from_range_from: MinCapVal::from(CAP..) => MIN_CAP);

mod range_bounds {
    use super::*;

    check_eq!(start_bound: MIN_CAP.start_bound() => Included(&CAP));
    check_eq!(end_bound: MIN_CAP.end_bound() => Unbounded);
}

mod check_compat {
    use super::*;

    check_eq!(compatible: MIN_CAP.check_compatibility(&COMPAT_ITER) => Ok(()));
    check_eq!(underflow: MIN_CAP.check_compatibility(&UNDER_ITER) => Err(MAX_UNDERFLOWS));

    panics!(bad_iter: MIN_CAP.check_compatibility(&INVALID_ITER) => "Invalid size hint");
}

mod check_fit {
    use super::*;

    check_eq!(compatible: MIN_CAP.check_fit(&COMPAT_ITER) => Ok(()));
    check_eq!(underflow: MIN_CAP.check_fit(&UNDER_ITER) => Err(MIN_UNDERFLOWS));

    panics!(bad_iter: MIN_CAP.check_fit(&INVALID_ITER) => "Invalid size hint");
}
