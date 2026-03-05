use super::*;
use fluent_result::into::IntoResult;
use tap::Pipe;

check_eq!(min_cap: StaticExactCap::<CAP>.min_cap() => StaticMinCap::<CAP>);
check_eq!(max_cap: StaticExactCap::<CAP>.max_cap() => StaticMaxCap::<CAP>);

mod check_compat {
    use super::*;

    check_eq!(compatible: StaticExactCap::<CAP>.check_compatibility(&COMPAT_ITER) => Ok(()));
    check_eq!(underflow: StaticExactCap::<CAP>.check_compatibility(&UNDER_ITER)
        => UNDER_CAP.pipe(MaxUnderflow::<StaticMinCap<CAP>>::new)
            .pipe(CompatError::Underflow)
            .into_err()
    );

    check_eq!(overflow: StaticExactCap::<CAP>.check_compatibility(&OVER_ITER)
        => OVER_CAP.pipe(MinOverflow::<StaticMaxCap<CAP>>::new)
            .pipe(CompatError::Overflow)
            .into_err()
    );

    panics!(bad_iter: StaticExactCap::<CAP>.check_compatibility(&INVALID_ITER)
        => "Invalid size hint");
}

mod check_fit {
    use super::*;

    check_eq!(compatible: StaticExactCap::<CAP>.check_fit(&COMPAT_ITER) => Ok(()));
    check_eq!(underflow: StaticExactCap::<CAP>.check_fit(&UNDER_ITER)
        => UNDER_CAP.pipe(MinUnderflow::<StaticMinCap<CAP>>::new)
            .pipe(FitError::Underflow)
            .into_err()
    );

    check_eq!(overflow: StaticExactCap::<CAP>.check_fit(&OVER_ITER)
        => OVER_CAP.pipe(MaxOverflow::<StaticMaxCap<CAP>>::fixed)
            .pipe(FitError::Overflow)
            .into_err()
    );

    check_eq!(overflow_unbounded: StaticExactCap::<CAP>.check_fit(&OVER_ITER_UNBOUNDED)
        => MaxOverflow::UNBOUNDED.pipe(FitError::Overflow).into_err());

    check_eq!(both: StaticExactCap::<CAP>.check_fit(&BOTH_ITER)
        => FitErrorSpan::new(
            MaxOverflow::<StaticMaxCap<CAP>>::fixed(OVER_CAP),
            MinUnderflow::<StaticMinCap<CAP>>::new(UNDER_CAP),
        ).pipe(FitError::Both).into_err()
    );

    panics!(bad_iter: StaticExactCap::<CAP>.check_fit(&INVALID_ITER)
        => "Invalid size hint");
}

mod range_bounds {
    use super::*;

    check_eq!(start_bound: StaticExactCap::<CAP>.start_bound() => Included(&CAP));
    check_eq!(end_bound: StaticExactCap::<CAP>.end_bound() => Included(&CAP));
}

check_eq!(range_const: StaticExactCap::<CAP>::RANGE => CAP..=CAP);
check_eq!(from_range_inclusive: RangeInclusive::<usize>::from(StaticExactCap::<CAP>)
    => StaticExactCap::<CAP>::RANGE);
